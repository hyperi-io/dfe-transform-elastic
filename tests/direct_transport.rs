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

use std::time::Duration;

use bytes::Bytes;
use dfe_transform_elastic::config::{Config, SinkConfig, SourceConfig, Transport};
use dfe_transform_elastic::envelope::EnvelopeSetting;
use dfe_transform_elastic::metrics::TransformMetrics;
use dfe_transform_elastic::service;
use scalo::metrics::MetricsManager;
use scalo::transport::grpc::{GrpcConfig, GrpcTransport};
use scalo::transport::{AnyReceiver, AnySender, TransportReceiver, TransportSender};
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
        assert!(result.is_ok(), "push failed: {result:?}");
    }

    let mut received: Vec<Vec<u8>> = Vec::new();
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while received.len() < sent.len() && tokio::time::Instant::now() < deadline {
        if let Ok(batch) = listener.recv(10).await {
            received.extend(batch.records.into_iter().map(|r| r.payload.to_vec()));
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
            // Carried on the record here too: `publish` hands the topic to the
            // sender whichever transport sits underneath it.
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

#[tokio::test]
async fn a_batch_pushed_over_grpc_comes_out_the_grpc_sink_transformed() {
    // The downstream stage, standing in for the loader.
    let (loader_endpoint, loader) = start_listener().await;
    // The transform's own listener, and a client that pushes into it.
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
    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
    while events.len() < 3 && tokio::time::Instant::now() < deadline {
        if let Ok(batch) = loader.recv(16).await {
            for record in &batch.records {
                for line in String::from_utf8_lossy(&record.payload).lines() {
                    if !line.trim().is_empty() {
                        events.push(serde_json::from_str(line).expect("output is JSON"));
                    }
                }
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
