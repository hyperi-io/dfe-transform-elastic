// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Broker round-trip: real events in one topic, transformed events out another.
//!
//! Every other test in this repo drives the transform directly. These drive the
//! service loop over a real broker, which is the only place the parts that
//! matter in production are exercised: consumer-group subscription, batch
//! assembly, NDJSON framing on the wire, and commit-after-send.
//!
//! Ignored by default. A test that reaches a broker must be asked for:
//! `cargo test --test broker -- --ignored`.

// The whole file drives the Kafka service loop, which the `kafka` feature
// gates. Without it there is no `service` module and no scalo Kafka transport.
#![cfg(feature = "kafka")]
// A test asserts by panicking; the workspace lints ban that in library code.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use std::time::Duration;

use dfe_transform_elastic::config::{Config, SinkConfig, SourceConfig};
use dfe_transform_elastic::envelope::EnvelopeSetting;
use dfe_transform_elastic::metrics::TransformMetrics;
use dfe_transform_elastic::service;
use scalo::metrics::MetricsManager;
use scalo::transport::{AnyReceiver, AnySender, TransportReceiver, TransportSender};
use tokio_util::sync::CancellationToken;

/// How long to wait for the loop to move a batch end to end. Generous, because
/// a cold container spends most of it electing a controller.
const ROUND_TRIP_TIMEOUT: Duration = Duration::from_mins(1);

fn config(brokers: Vec<String>, source_topic: &str, sink_topic: &str, group: &str) -> Config {
    // Small, so one produce fills a batch rather than waiting. The shipped
    // default is exercised by `a_batch_larger_than_one_kafka_record_arrives`.
    config_sized(brokers, source_topic, sink_topic, group, 16)
}

fn config_sized(
    brokers: Vec<String>,
    source_topic: &str,
    sink_topic: &str,
    group: &str,
    batch_size: usize,
) -> Config {
    Config {
        source: SourceConfig {
            name: "filebeat.okta.default".into(),
            transport: dfe_transform_elastic::config::Transport::Bus,
            listen: String::new(),
            envelope: EnvelopeSetting::Auto,
            topics: vec![source_topic.to_string()],
            batch_size,
            max_batch_bytes: dfe_transform_elastic::config::default_max_batch_bytes(),
            group_id: group.to_string(),
            brokers: brokers.clone(),
        },
        sink: SinkConfig {
            topic: sink_topic.to_string(),
            transport: dfe_transform_elastic::config::Transport::Bus,
            endpoint: String::new(),
            brokers: Some(brokers),
            max_message_bytes: dfe_transform_elastic::config::default_max_message_bytes(),
        },
        // Unread here: the round trip drives `run_loop`, and provisioning sits
        // on `run`, so no test reaches a provider.
        geoip: scalo::geoip_download::GeoIpConfig::default(),
    }
}

/// The display name stamped into every seeded event, so the round trip proves
/// non-ASCII survives the WIRE and not merely the in-process transform.
const NON_ASCII_ACTOR: &str = "Björn Ärlig 日本語";

/// `count` okta events from the committed fixture, in the shape the transform
/// accepts: the raw vendor payload as a STRING in `message`, which is how
/// Beats delivers it. A bare vendor object passes through barely touched.
///
/// The fixture holds 24 lines; asking for more cycles through them, so a test
/// can ask for a payload big enough to exceed one Kafka record.
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
        let mut vendor: serde_json::Value =
            serde_json::from_str(lines[i % lines.len()]).expect("fixture line is JSON");
        vendor["actor"]["displayName"] = serde_json::Value::String(NON_ASCII_ACTOR.into());
        // Unique per event, so a duplicate cannot be mistaken for an arrival.
        vendor["uuid"] = serde_json::Value::String(format!("seed-{i}"));

        let beat = serde_json::json!({
            "message": serde_json::to_string(&vendor).expect("vendor payload serialises"),
        });
        out.extend_from_slice(&serde_json::to_vec(&beat).expect("event serialises"));
        out.push(b'\n');
    }
    out
}

/// Split an NDJSON payload into records the producer will actually accept.
///
/// The seed side has the same 1 MB ceiling as the service, so a multi-megabyte
/// seed has to be produced as several records -- on LINE boundaries, or the
/// service reads half an event.
fn as_records(payload: &[u8], max_bytes: usize) -> Vec<Vec<u8>> {
    let mut records = Vec::new();
    let mut current = Vec::new();
    for line in payload.split_inclusive(|b| *b == b'\n') {
        if !current.is_empty() && current.len() + line.len() > max_bytes {
            records.push(std::mem::take(&mut current));
        }
        current.extend_from_slice(line);
    }
    if !current.is_empty() {
        records.push(current);
    }
    records
}

/// Read NDJSON records off `topic` until `want` events have arrived or the
/// timeout expires. Returns the events and how many Kafka RECORDS carried
/// them, which is what tells a split batch from a single one.
async fn drain(
    consumer: &scalo::transport::kafka::KafkaTransport,
    want: usize,
) -> (Vec<serde_json::Value>, usize) {
    let deadline = tokio::time::Instant::now() + ROUND_TRIP_TIMEOUT;
    let mut events = Vec::new();
    let mut records = 0;

    while events.len() < want && tokio::time::Instant::now() < deadline {
        let Ok(batch) = consumer.recv(64).await else {
            tokio::time::sleep(Duration::from_millis(200)).await;
            continue;
        };
        if batch.records.is_empty() {
            tokio::time::sleep(Duration::from_millis(200)).await;
            continue;
        }
        eprintln!("drain: {} record(s)", batch.records.len());
        records += batch.records.len();
        for record in &batch.records {
            for line in String::from_utf8_lossy(&record.payload).lines() {
                if line.trim().is_empty() {
                    continue;
                }
                events.push(serde_json::from_str(line).expect("output line is JSON"));
            }
        }
    }

    (events, records)
}

#[tokio::test(flavor = "multi_thread")]
#[ignore = "reaches a Kafka broker. Run with `cargo test --test broker -- --ignored`."]
async fn a_batch_survives_in_transform_out() {
    let broker = broker_or_skip!("round-trip");
    let source_topic = common::topic("src");
    let sink_topic = common::topic("sink");
    let group = common::topic("cg");

    let seed = broker.producer("seed").await;
    let sent = seed
        .send(&source_topic, bytes::Bytes::from(okta_events(5)))
        .await;
    assert!(
        matches!(sent, scalo::transport::SendResult::Ok),
        "seed produce failed: {sent:?}"
    );

    // Prove the seed is readable before blaming the service loop for not
    // reading it.
    {
        let probe = broker
            .consumer("probe", &source_topic, &common::topic("probe-cg"))
            .await;
        let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
        let mut readable = 0;
        while readable == 0 && tokio::time::Instant::now() < deadline {
            if let Ok(batch) = probe.recv(64).await {
                readable = batch.records.len();
            }
            if readable == 0 {
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        }
        assert!(
            readable > 0,
            "the seed produce is not readable from the source topic"
        );
        eprintln!("probe read {readable} record(s) from the source topic");
    }

    let config = config(broker.list(), &source_topic, &sink_topic, &group);
    let shutdown = CancellationToken::new();

    // `run_loop` takes the factory's enums, so the round trip wraps the real
    // Kafka pair rather than passing it bare.
    let consumer = AnyReceiver::Kafka(broker.consumer("service", &source_topic, &group).await);
    let producer = AnySender::Kafka(broker.producer("service").await);
    let manager = MetricsManager::new("dfe-transform-elastic-test");
    let metrics = TransformMetrics::register(&manager, "0.0.0-test", "test");

    let loop_shutdown = shutdown.clone();
    let service = tokio::spawn(async move {
        service::run_loop(
            &config,
            &consumer,
            &producer,
            &loop_shutdown,
            &metrics,
            None,
        )
        .await
    });

    let sink = broker
        .consumer("verify", &sink_topic, &common::topic("verify-cg"))
        .await;
    let (out, _) = drain(&sink, 5).await;

    shutdown.cancel();
    let result = tokio::time::timeout(Duration::from_secs(30), service)
        .await
        .expect("service loop stops on cancel")
        .expect("service task did not panic");
    result.expect("service loop returned an error");

    assert_eq!(out.len(), 5, "every event must come out the far side");

    for event in &out {
        assert_eq!(
            event.pointer("/ecs/version").and_then(|v| v.as_str()),
            Some("8.11.0"),
            "output did not reach the transform: {event}"
        );
        // `ecs.version` is set unconditionally on the transform's first line,
        // so it cannot distinguish a transformed event from an untouched one.
        // The `okta.*` namespace only exists once `message` has been unpacked.
        assert!(
            event.pointer("/okta/event_type").is_some(),
            "output reached the transform but was not transformed: {event}"
        );
        // Non-ASCII must survive the whole wire round trip, not just the
        // in-process one.
        let actor = event
            .pointer("/okta/actor/display_name")
            .and_then(|v| v.as_str());
        assert_eq!(
            actor,
            Some(NON_ASCII_ACTOR),
            "text changed on the wire: {event}"
        );
    }
}

/// Offsets must move only after a successful send. A second service with the
/// same group must therefore find nothing left to read.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "reaches a Kafka broker. Run with `cargo test --test broker -- --ignored`."]
async fn offsets_commit_after_the_batch_is_sent() {
    let broker = broker_or_skip!("commit-after-send");
    let source_topic = common::topic("commit-src");
    let sink_topic = common::topic("commit-sink");
    let group = common::topic("commit-cg");

    let seed = broker.producer("seed").await;
    seed.send(&source_topic, bytes::Bytes::from(okta_events(3)))
        .await;

    let config = config(broker.list(), &source_topic, &sink_topic, &group);

    // First run: consume, transform, send, commit.
    {
        let shutdown = CancellationToken::new();
        let consumer = AnyReceiver::Kafka(broker.consumer("first", &source_topic, &group).await);
        let producer = AnySender::Kafka(broker.producer("first").await);
        let manager = MetricsManager::new("dfe-transform-elastic-test");
        let metrics = TransformMetrics::register(&manager, "0.0.0-test", "test");

        let loop_shutdown = shutdown.clone();
        let cfg = config.clone();
        let service = tokio::spawn(async move {
            service::run_loop(&cfg, &consumer, &producer, &loop_shutdown, &metrics, None).await
        });

        let sink = broker
            .consumer("verify", &sink_topic, &common::topic("commit-verify-cg"))
            .await;
        assert_eq!(drain(&sink, 3).await.0.len(), 3);

        shutdown.cancel();
        tokio::time::timeout(Duration::from_secs(30), service)
            .await
            .expect("first run stops")
            .expect("first run did not panic")
            .expect("first run returned an error");
    }

    // Second run, same group: the committed offset means nothing is replayed.
    let replay = broker.consumer("second", &source_topic, &group).await;
    let batch = tokio::time::timeout(Duration::from_secs(15), replay.recv(64))
        .await
        .map_or(0, |r| r.map_or(0, |b| b.records.len()));

    assert_eq!(
        batch, 0,
        "the first run committed, so nothing should replay"
    );
}

/// A batch at the SHIPPED `batch_size` does not fit in one Kafka record.
///
/// librdkafka's producer `message.max.bytes` defaults to 1,000,000 and scalo
/// sets no override, so publishing a whole batch as one record fails for every
/// batch of any size. The other tests in this file run a `batch_size` of 16 and
/// never touched the ceiling -- this one uses the default and seeds several MB.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "reaches a Kafka broker. Run with `cargo test --test broker -- --ignored`."]
async fn a_batch_larger_than_one_kafka_record_arrives() {
    /// 2,000 okta events is roughly 4 MB in and more out -- several records at
    /// the 900 KB budget, without the elapsed time of a full 20,000.
    const EVENTS: usize = 2_000;

    let broker = broker_or_skip!("oversize-batch");
    let source_topic = common::topic("big-src");
    let sink_topic = common::topic("big-sink");
    let group = common::topic("big-cg");

    let seed = broker.producer("seed").await;
    let payload = okta_events(EVENTS);
    assert!(
        payload.len() > 1_000_000,
        "the seed must exceed one Kafka record to test anything: {} bytes",
        payload.len()
    );
    for record in as_records(&payload, 500_000) {
        let sent = seed.send(&source_topic, bytes::Bytes::from(record)).await;
        assert!(
            matches!(sent, scalo::transport::SendResult::Ok),
            "seed produce failed: {sent:?}"
        );
    }

    // The shipped default, not a convenient small number.
    let config = config_sized(broker.list(), &source_topic, &sink_topic, &group, 20_000);
    assert_eq!(config.source.batch_size, 20_000);

    let shutdown = CancellationToken::new();
    // `run_loop` takes the factory's enums, so the round trip wraps the real
    // Kafka pair rather than passing it bare.
    let consumer = AnyReceiver::Kafka(broker.consumer("service", &source_topic, &group).await);
    let producer = AnySender::Kafka(broker.producer("service").await);
    let manager = MetricsManager::new("dfe-transform-elastic-test");
    let metrics = TransformMetrics::register(&manager, "0.0.0-test", "test");

    let loop_shutdown = shutdown.clone();
    let service = tokio::spawn(async move {
        service::run_loop(
            &config,
            &consumer,
            &producer,
            &loop_shutdown,
            &metrics,
            None,
        )
        .await
    });

    let sink = broker
        .consumer("verify", &sink_topic, &common::topic("big-verify-cg"))
        .await;
    let (out, records) = drain(&sink, EVENTS).await;

    shutdown.cancel();
    tokio::time::timeout(Duration::from_secs(30), service)
        .await
        .expect("service loop stops on cancel")
        .expect("service task did not panic")
        .expect("service loop returned an error");

    assert_eq!(out.len(), EVENTS, "the split lost events");
    assert!(
        records > 1,
        "the batch was published as one record, so the split never ran"
    );
    for event in &out {
        assert!(
            event.pointer("/okta/event_type").is_some(),
            "output was not transformed: {event}"
        );
    }
}

/// The premise the split rests on: librdkafka refuses a record above its
/// `message.max.bytes` producer default of 1,000,000 bytes, and scalo sets no
/// override. If that ever stops being true, the budget can be raised -- but it
/// should be raised deliberately, not discovered in production.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "reaches a Kafka broker. Run with `cargo test --test broker -- --ignored`."]
async fn the_producer_refuses_a_record_above_one_megabyte() {
    let broker = broker_or_skip!("oversize-record");
    let topic = common::topic("oversize-record");
    let producer = broker.producer("oversize-record").await;

    let sent = producer
        .send(&topic, bytes::Bytes::from(vec![b'x'; 1_100_000]))
        .await;

    assert!(
        !matches!(sent, scalo::transport::SendResult::Ok),
        "a 1.1 MB record was accepted, so the 900 KB budget is no longer needed: {sent:?}"
    );
}

/// The container this suite starts must carry a traceable name and the suite
/// label, so an operator can tell what left it behind.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "reaches a Kafka broker. Run with `cargo test --test broker -- --ignored`."]
async fn container_is_named_and_labelled() {
    let broker = broker_or_skip!("container-hygiene");
    if !broker.is_container() {
        eprintln!("using a live broker; nothing to inspect");
        return;
    }

    let out = std::process::Command::new("docker")
        .args([
            "ps",
            "--format",
            "{{.Names}}",
            "--filter",
            &format!(
                "label={}={}",
                common::TEST_SUITE_LABEL.0,
                common::TEST_SUITE_LABEL.1
            ),
        ])
        .output()
        .expect("docker ps runs");

    let names = String::from_utf8_lossy(&out.stdout);
    assert!(
        names.contains("dfe-transform-elastic-test-broker-container-hygiene"),
        "container is not traceable by name or label: {names}"
    );
}
