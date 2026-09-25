// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The scalo Push listener this service's direct transport is built on.
//!
//! A record pushed at a listener comes back out of its own `recv`, with no
//! broker anywhere. That is the transport half of issue #19: it proves the
//! `grpc` feature wires scalo's listener and client through to this crate, and
//! it needs no container, so it runs by default rather than behind `#[ignore]`.
//!
//! The batch loop runs over this transport too: the end-to-end test below
//! pushes at the transform's listener and reads transformed events off a
//! second one, with no broker in the path.

#![cfg(feature = "grpc")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bytes::Bytes;
use dfe_transform_elastic::config::{Config, SinkConfig, SourceConfig, Transport};
use dfe_transform_elastic::envelope::EnvelopeSetting;
use dfe_transform_elastic::metrics::TransformMetrics;
use dfe_transform_elastic::service;
use scalo::metrics::MetricsManager;
use scalo::transport::grpc::{GrpcConfig, GrpcTransport};
use scalo::transport::{AnyReceiver, AnySender, Record, TransportReceiver, TransportSender};
use tokio_util::sync::CancellationToken;

/// Allocate a free loopback port.
fn random_port() -> u16 {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    let port = listener.local_addr().expect("local addr").port();
    drop(listener);
    port
}

/// Poll until the port accepts a connection, or fail after 15s.
async fn wait_for_port(port: u16) {
    let addr = format!("127.0.0.1:{port}");
    for _ in 0..300 {
        if tokio::net::TcpStream::connect(&addr).await.is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    panic!("nothing listening on 127.0.0.1:{port} within 15s");
}

/// Start a Push listener and return its endpoint plus the transport.
///
/// The port is free when picked and can be taken before the bind lands under a
/// parallel test run, so a failed bind retries on a fresh one.
async fn start_listener() -> (String, GrpcTransport) {
    let mut last_err = String::new();
    for _ in 0..20 {
        let port = random_port();
        match GrpcTransport::new(&GrpcConfig::server(&format!("127.0.0.1:{port}"))).await {
            Ok(transport) => {
                wait_for_port(port).await;
                return (format!("http://127.0.0.1:{port}"), transport);
            }
            Err(e) => {
                last_err = e.to_string();
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        }
    }
    panic!("listener failed to start after 20 attempts: {last_err}");
}

#[tokio::test]
async fn a_record_pushed_at_the_listener_comes_back_out_of_it() {
    let (endpoint, listener) = start_listener().await;
    let pusher = GrpcTransport::new(&GrpcConfig::client(&endpoint))
        .await
        .expect("push client connects to the listener");

    let sent: Vec<String> = (0..3)
        .map(|id| format!(r#"{{"message":"line {id}"}}"#))
        .collect();
    for payload in &sent {
        let result = pusher
            .send("elastic_in", Bytes::from(payload.clone()))
            .await;
        assert!(
            matches!(result, scalo::transport::SendResult::Ok),
            "push failed: {result:?}"
        );
    }

    let mut received: Vec<Vec<u8>> = Vec::new();
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while received.len() < sent.len() {
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        if remaining.is_zero() {
            break;
        }
        match tokio::time::timeout(remaining, listener.recv(10)).await {
            Ok(Ok(batch)) => {
                received.extend(batch.records.into_iter().map(|r| r.payload.to_vec()));
            }
            Ok(Err(_)) => tokio::time::sleep(Duration::from_millis(20)).await,
            Err(_) => break,
        }
    }

    assert_eq!(
        received.len(),
        sent.len(),
        "every pushed record must reach the listener"
    );
    // Byte-for-byte: the transport carries the payload, it does not parse it.
    for (payload, expected) in received.iter().zip(&sent) {
        assert_eq!(
            std::str::from_utf8(payload).expect("payload is utf-8"),
            expected
        );
    }
}

/// A direct deployment: no brokers, no group, no topics.
///
/// `work_state` does not idle on empty topics here, because on this arm the
/// listener is the work.
fn direct_config(listen: &str, endpoint: &str) -> Config {
    Config {
        source: SourceConfig {
            name: "filebeat.okta.default".into(),
            transport: Transport::Direct,
            listen: listen.into(),
            envelope: EnvelopeSetting::Auto,
            topics: Vec::new(),
            batch_size: 16,
            max_batch_bytes: dfe_transform_elastic::config::default_max_batch_bytes(),
            group_id: String::new(),
            brokers: Vec::new(),
        },
        sink: SinkConfig {
            topic: "elastic_out".into(),
            transport: Transport::Direct,
            endpoint: endpoint.into(),
            brokers: None,
            max_message_bytes: dfe_transform_elastic::config::default_max_message_bytes(),
        },
        geoip: scalo::geoip_download::GeoIpConfig::default(),
    }
}

/// `count` okta events in the shape Beats delivers: the vendor payload as a
/// STRING in `message`.
fn okta_events(count: usize) -> Vec<u8> {
    let raw = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/okta/system/test-okta-system-events.log"
    ))
    .expect("okta fixture is committed");
    let lines: Vec<&str> = raw.lines().filter(|l| !l.trim().is_empty()).collect();
    assert!(!lines.is_empty(), "okta fixture is empty");

    let mut out = Vec::new();
    for i in 0..count {
        let beat = serde_json::json!({ "message": lines[i % lines.len()] });
        out.extend_from_slice(&serde_json::to_vec(&beat).expect("event serialises"));
        out.push(b'\n');
    }
    out
}

/// Read records off `listener` until they carry `events` JSON lines, or 30s pass.
async fn drain(listener: &GrpcTransport, events: usize) -> Vec<Record> {
    let mut records: Vec<Record> = Vec::new();
    let mut lines = 0;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    while lines < events {
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        if remaining.is_zero() {
            break;
        }
        let batch = match tokio::time::timeout(remaining, listener.recv(16)).await {
            Ok(Ok(batch)) => batch,
            Ok(Err(_)) => {
                tokio::time::sleep(Duration::from_millis(20)).await;
                continue;
            }
            Err(_) => break,
        };
        for record in batch.records {
            lines += String::from_utf8_lossy(&record.payload)
                .lines()
                .filter(|line| !line.trim().is_empty())
                .count();
            records.push(record);
        }
    }
    records
}

#[tokio::test]
async fn a_batch_pushed_over_grpc_comes_out_the_grpc_sink_transformed() {
    let (loader_endpoint, loader) = start_listener().await;
    let (transform_endpoint, transform_listener) = start_listener().await;
    let pusher = GrpcTransport::new(&GrpcConfig::client(&transform_endpoint))
        .await
        .expect("push client");
    let sink = AnySender::Grpc(
        GrpcTransport::new(&GrpcConfig::client(&loader_endpoint))
            .await
            .expect("sink client"),
    );

    let listen = transform_endpoint
        .strip_prefix("http://")
        .expect("endpoint is http")
        .to_string();
    let config = direct_config(&listen, &loader_endpoint);
    let consumer = AnyReceiver::Grpc(transform_listener);
    let manager = MetricsManager::new("dfe-transform-elastic-test");
    let metrics = TransformMetrics::register(&manager, "0.0.0-test", "test");

    let shutdown = CancellationToken::new();
    let loop_shutdown = shutdown.clone();
    let loop_task = tokio::spawn(async move {
        service::run_loop(&config, &consumer, &sink, &loop_shutdown, &metrics, None).await
    });

    let sent = pusher.send("elastic_in", Bytes::from(okta_events(3))).await;
    assert!(matches!(sent, scalo::transport::SendResult::Ok), "{sent:?}");

    let mut events: Vec<serde_json::Value> = Vec::new();
    for record in drain(&loader, 3).await {
        // `publish` hands `sink.topic` to the sender whichever transport
        // sits underneath, and the listener carries it as the routing key.
        assert_eq!(
            record.key.as_deref(),
            Some("elastic_out"),
            "the sink topic must reach the wire"
        );
        for line in String::from_utf8_lossy(&record.payload).lines() {
            if !line.trim().is_empty() {
                events.push(serde_json::from_str(line).expect("output is JSON"));
            }
        }
    }

    shutdown.cancel();
    let result = tokio::time::timeout(Duration::from_secs(30), loop_task)
        .await
        .expect("service loop stops on cancel")
        .expect("service task did not panic");
    result.expect("service loop returned an error");

    assert_eq!(events.len(), 3, "every pushed event must reach the sink");
    for event in &events {
        // `ecs.version` is stamped on the transform's first line and so cannot
        // tell a transformed event from an untouched one; the `okta.*`
        // namespace exists only once `message` has been unpacked.
        assert!(
            event.get("okta").is_some(),
            "output did not reach the transform: {event}"
        );
    }
}

/// Every counter series by name and labels, as a scrape keys it.
#[derive(Default)]
struct SeriesCapture {
    counters: Mutex<HashMap<metrics::Key, Arc<AtomicU64>>>,
}

impl SeriesCapture {
    /// `name` summed over every label set, as a `sum()` over it reads.
    fn total(&self, name: &str) -> u64 {
        self.counters
            .lock()
            .unwrap()
            .iter()
            .filter(|(key, _)| key.name() == name)
            .map(|(_, cell)| cell.load(Ordering::Acquire))
            .sum()
    }
}

impl metrics::Recorder for SeriesCapture {
    fn describe_counter(
        &self,
        _: metrics::KeyName,
        _: Option<metrics::Unit>,
        _: metrics::SharedString,
    ) {
    }
    fn describe_gauge(
        &self,
        _: metrics::KeyName,
        _: Option<metrics::Unit>,
        _: metrics::SharedString,
    ) {
    }
    fn describe_histogram(
        &self,
        _: metrics::KeyName,
        _: Option<metrics::Unit>,
        _: metrics::SharedString,
    ) {
    }

    fn register_counter(&self, key: &metrics::Key, _: &metrics::Metadata<'_>) -> metrics::Counter {
        let cell = Arc::clone(
            self.counters
                .lock()
                .unwrap()
                .entry(key.clone())
                .or_default(),
        );
        metrics::Counter::from_arc(cell)
    }

    fn register_gauge(&self, _: &metrics::Key, _: &metrics::Metadata<'_>) -> metrics::Gauge {
        metrics::Gauge::noop()
    }

    fn register_histogram(
        &self,
        _: &metrics::Key,
        _: &metrics::Metadata<'_>,
    ) -> metrics::Histogram {
        metrics::Histogram::noop()
    }
}

/// The transform's own view of one pushed batch: each record and each byte
/// counted once, with the scalo transports owning the `transport_*` series.
#[test]
fn a_pushed_batch_is_counted_once_by_the_transform() {
    let capture = SeriesCapture::default();
    // The pusher and the downstream listener stand in for other services, so
    // they run on a runtime of their own, off the thread whose metrics are read.
    let neighbours = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .expect("neighbour runtime");
    let (loader_endpoint, loader) = neighbours.block_on(start_listener());
    let wire = okta_events(3);
    let wire_bytes = wire.len() as u64;

    let service = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("service runtime");
    let delivered_bytes = metrics::with_local_recorder(&capture, || {
        service.block_on(async {
            let (transform_endpoint, transform_listener) = start_listener().await;
            let sink = AnySender::Grpc(
                GrpcTransport::new(&GrpcConfig::client(&loader_endpoint))
                    .await
                    .expect("sink client"),
            );
            let listen = transform_endpoint
                .strip_prefix("http://")
                .expect("endpoint is http")
                .to_string();
            let config = direct_config(&listen, &loader_endpoint);
            let consumer = AnyReceiver::Grpc(transform_listener);
            let manager = MetricsManager::with_config(scalo::metrics::MetricsConfig::offline(""));
            let metrics = TransformMetrics::register(&manager, "0.0.0-test", "test");
            let shutdown = CancellationToken::new();

            let push = neighbours.spawn(async move {
                let pusher = GrpcTransport::new(&GrpcConfig::client(&transform_endpoint))
                    .await
                    .expect("push client");
                pusher.send("elastic_in", Bytes::from(wire)).await
            });
            let delivered = neighbours.spawn(async move {
                drain(&loader, 3)
                    .await
                    .iter()
                    .map(|record| record.payload.len() as u64)
                    .sum::<u64>()
            });

            let run = service::run_loop(&config, &consumer, &sink, &shutdown, &metrics, None);
            let stop = async {
                let sent = push.await.expect("push task");
                assert!(matches!(sent, scalo::transport::SendResult::Ok), "{sent:?}");
                let bytes = delivered.await.expect("drain task");
                shutdown.cancel();
                bytes
            };
            let (result, bytes) = tokio::join!(run, stop);
            result.expect("service loop returned an error");
            bytes
        })
    });

    assert!(delivered_bytes > 0, "the sink delivered nothing");
    // (records received, wire records received, bytes received, bytes sent)
    assert_eq!(
        (
            capture.total("records_received_total"),
            capture.total("transport_received_events_total"),
            capture.total("transport_received_bytes_total"),
            capture.total("transport_sent_bytes_total"),
        ),
        (3, 1, wire_bytes, delivered_bytes),
        "three events in one pushed record, and each byte, read once"
    );
}

#[tokio::test]
async fn a_client_pointed_at_nothing_fails_rather_than_hanging() {
    // A dead endpoint is what a misconfigured `sink.endpoint` produces, and the
    // service must learn about it at startup rather than at the first batch.
    let port = random_port();
    let client = GrpcTransport::new(&GrpcConfig::client(&format!("http://127.0.0.1:{port}"))).await;

    // Refusing at construction and refusing at the send are both correct, so
    // the branch is named rather than skipped.
    let refused_at = match client {
        Ok(transport) => {
            let result = transport
                .send("elastic_in", Bytes::from_static(b"{}"))
                .await;
            assert!(
                !matches!(result, scalo::transport::SendResult::Ok),
                "a send to a dead endpoint reported success"
            );
            "send"
        }
        Err(_) => "construction",
    };
    println!("a dead endpoint was refused at {refused_at}");
}
