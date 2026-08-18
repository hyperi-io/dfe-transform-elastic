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

// A test asserts by panicking; the workspace lints ban that in library code.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use std::time::Duration;

use dfe_transform_elastic::config::{Config, SinkConfig, SourceConfig};
use dfe_transform_elastic::metrics::TransformMetrics;
use dfe_transform_elastic::service;
use scalo::metrics::MetricsManager;
use scalo::transport::{TransportReceiver, TransportSender};
use tokio_util::sync::CancellationToken;

/// How long to wait for the loop to move a batch end to end. Generous, because
/// a cold container spends most of it electing a controller.
const ROUND_TRIP_TIMEOUT: Duration = Duration::from_mins(1);

fn config(brokers: Vec<String>, source_topic: &str, sink_topic: &str, group: &str) -> Config {
    Config {
        pipeline_name: "broker-test".into(),
        source: SourceConfig {
            name: "filebeat.okta.default".into(),
            topics: vec![source_topic.to_string()],
            // Small, so one produce fills a batch rather than waiting.
            batch_size: 16,
            group_id: group.to_string(),
            brokers: brokers.clone(),
        },
        sink: SinkConfig {
            topic: sink_topic.to_string(),
            brokers: Some(brokers),
        },
    }
}

/// The display name stamped into every seeded event, so the round trip proves
/// non-ASCII survives the WIRE and not merely the in-process transform.
const NON_ASCII_ACTOR: &str = "Björn Ärlig 日本語";

/// `count` okta events from the committed fixture, in the shape the transform
/// accepts: the raw vendor payload as a STRING in `message`, which is how
/// Beats delivers it. A bare vendor object passes through barely touched.
fn okta_events(count: usize) -> Vec<u8> {
    let raw = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/okta/system/test-okta-system-events.log"
    ))
    .expect("okta fixture is committed");

    let mut out = Vec::new();
    for line in raw.lines().filter(|l| !l.trim().is_empty()).take(count) {
        let mut vendor: serde_json::Value =
            serde_json::from_str(line).expect("fixture line is JSON");
        vendor["actor"]["displayName"] = serde_json::Value::String(NON_ASCII_ACTOR.into());

        let beat = serde_json::json!({
            "message": serde_json::to_string(&vendor).expect("vendor payload serialises"),
        });
        out.extend_from_slice(&serde_json::to_vec(&beat).expect("event serialises"));
        out.push(b'\n');
    }
    out
}

/// Read NDJSON records off `topic` until `want` events have arrived or the
/// timeout expires.
async fn drain(
    consumer: &scalo::transport::kafka::KafkaTransport,
    want: usize,
) -> Vec<serde_json::Value> {
    let deadline = tokio::time::Instant::now() + ROUND_TRIP_TIMEOUT;
    let mut events = Vec::new();

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
        for record in &batch.records {
            for line in String::from_utf8_lossy(&record.payload).lines() {
                if line.trim().is_empty() {
                    continue;
                }
                events.push(serde_json::from_str(line).expect("output line is JSON"));
            }
        }
    }

    events
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

    let consumer = broker.consumer("service", &source_topic, &group).await;
    let producer = broker.producer("service").await;
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
    let out = drain(&sink, 5).await;

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
        let consumer = broker.consumer("first", &source_topic, &group).await;
        let producer = broker.producer("first").await;
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
        assert_eq!(drain(&sink, 3).await.len(), 3);

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
