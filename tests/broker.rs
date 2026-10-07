// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Broker round-trip: real events in one topic, transformed events out another.
//!
//! Every other test in this repo drives the transform directly. These drive the
//! service loop over a real broker, which is the only place the parts that
//! matter in production are exercised: consumer-group subscription, batch
//! assembly, one record per event on the wire, and the offset commit held
//! until the block is delivered.
//!
//! They run by default, on a live broker named by `KAFKA_BROKERS` or else an
//! ephemeral container. With neither, a test skips locally and FAILS when `CI`
//! is set, so CI cannot report green on a suite that tested nothing.

// The whole file drives the Kafka service loop, which the `kafka` feature
// gates. Without it there is no `service` module and no scalo Kafka transport.
#![cfg(feature = "kafka")]
// A test asserts by panicking; the workspace lints ban that in library code.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use std::collections::BTreeSet;
use std::time::Duration;

use dfe_transform_elastic::config::{Config, SinkConfig, SourceConfig};
use dfe_transform_elastic::envelope::EnvelopeSetting;
use dfe_transform_elastic::metrics::TransformMetrics;
use dfe_transform_elastic::service;
use scalo::metrics::MetricsManager;
use scalo::transport::{AnyReceiver, AnySender, TransportReceiver, TransportSender};
use scalo::worker::BatchEngine;
use tokio::task::JoinHandle;
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
            acknowledgements: scalo::transport::AcknowledgementsConfig::default(),
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

/// Run the service loop over `consumer` and `producer` until the returned
/// token is cancelled.
///
/// `run_loop` takes the factory's enums, so the round trip wraps the real Kafka
/// pair rather than passing it bare.
fn start_loop(
    config: Config,
    consumer: AnyReceiver,
    producer: AnySender,
    metrics: TransformMetrics,
) -> (
    CancellationToken,
    JoinHandle<dfe_transform_elastic::Result<()>>,
) {
    let shutdown = CancellationToken::new();
    let loop_shutdown = shutdown.clone();
    let service = tokio::spawn(async move {
        let engine = service::with_dead_letters(BatchEngine::new(service::engine_config(&config)));
        service::run_loop(
            &config,
            &engine,
            &consumer,
            &producer,
            &loop_shutdown,
            &metrics,
            None,
        )
        .await
    });
    (shutdown, service)
}

/// Cancel the loop and require it to stop cleanly.
async fn stop_loop(
    shutdown: &CancellationToken,
    service: JoinHandle<dfe_transform_elastic::Result<()>>,
) {
    shutdown.cancel();
    tokio::time::timeout(Duration::from_secs(30), service)
        .await
        .expect("service loop stops on cancel")
        .expect("service task did not panic")
        .expect("service loop returned an error");
}

/// The manager a test reads its counters back from: no namespace, as the
/// running service has unless a deployment sets `metrics.namespace`.
fn scraped_manager() -> MetricsManager {
    MetricsManager::with_config(scalo::metrics::MetricsConfig::offline(""))
}

/// `name` summed over every series whose labels contain `label`, as a scrape
/// of `manager` reads it.
fn scraped(manager: &MetricsManager, name: &str, label: &str) -> f64 {
    let text = manager.render();
    assert!(!text.is_empty(), "no metrics recorder is installed");
    text.lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| {
            let (series, value) = line.rsplit_once(' ')?;
            let metric = series.split('{').next()?;
            (metric == name && series.contains(label))
                .then(|| value.parse::<f64>().ok())
                .flatten()
        })
        .sum()
}

/// The display name stamped into every seeded event, so the round trip proves
/// non-ASCII survives the WIRE and not merely the in-process transform.
const NON_ASCII_ACTOR: &str = "Björn Ärlig 日本語";

/// `count` okta events from the committed sample, in the shape the transform
/// accepts: the raw vendor payload as a STRING in `message`, which is how
/// Beats delivers it. A bare vendor object passes through barely touched.
///
/// The 33 of the sample's 120 records shaped as Okta's API writes them -- an
/// `actor` object, which the seed writes into, and a camelCase `eventType`,
/// which the transform reads -- are used. Asking for more cycles through them,
/// so a test can ask for a payload big enough to exceed one Kafka record.
fn okta_events(count: usize) -> Vec<u8> {
    okta_events_named("seed", count)
}

/// The same, with each event's okta `uuid` set to `{prefix}-{i}`, so a lost or
/// duplicated event shows up by value and not only by count.
fn okta_events_named(prefix: &str, count: usize) -> Vec<u8> {
    okta_events_padded(prefix, count, 0)
}

/// The same, with `pad` bytes added to each event's `displayMessage`, which the
/// transform carries into its output.
fn okta_events_padded(prefix: &str, count: usize, pad: usize) -> Vec<u8> {
    let raw = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/unencumbered/okta/panther-okta-systemlog.ndjson"
    ))
    .expect("okta sample is committed");

    let lines: Vec<&str> = raw
        .lines()
        .filter(|l| {
            serde_json::from_str::<serde_json::Value>(l)
                .is_ok_and(|v| v["actor"].is_object() && v["eventType"].is_string())
        })
        .collect();
    assert!(!lines.is_empty(), "okta sample has no API-shaped record");

    let mut out = Vec::new();
    for i in 0..count {
        let mut vendor: serde_json::Value =
            serde_json::from_str(lines[i % lines.len()]).expect("fixture line is JSON");
        vendor["actor"]["displayName"] = serde_json::Value::String(NON_ASCII_ACTOR.into());
        vendor["uuid"] = serde_json::Value::String(format!("{prefix}-{i}"));
        if pad > 0 {
            vendor["displayMessage"] = serde_json::Value::String("x".repeat(pad));
        }

        let beat = serde_json::json!({
            "message": serde_json::to_string(&vendor).expect("vendor payload serialises"),
        });
        out.extend_from_slice(&serde_json::to_vec(&beat).expect("event serialises"));
        out.push(b'\n');
    }
    out
}

/// The okta `uuid` of every transformed event.
fn uuids(events: &[serde_json::Value]) -> BTreeSet<String> {
    events
        .iter()
        .filter_map(|event| event.pointer("/okta/uuid").and_then(|v| v.as_str()))
        .map(String::from)
        .collect()
}

/// `{prefix}-0` to `{prefix}-{count - 1}`, the uuids a seed carries.
fn expected_uuids(prefix: &str, count: usize) -> BTreeSet<String> {
    (0..count).map(|i| format!("{prefix}-{i}")).collect()
}

/// Split an NDJSON payload into records a stock broker will actually accept.
///
/// A stock broker refuses a record over about 1 MB, so a multi-megabyte seed
/// has to be produced as several records -- on LINE boundaries, or the service
/// reads half an event.
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
    drain_for(consumer, want, ROUND_TRIP_TIMEOUT).await
}

/// [`drain`], waiting at most `timeout`.
async fn drain_for(
    consumer: &scalo::transport::kafka::KafkaTransport,
    want: usize,
    timeout: Duration,
) -> (Vec<serde_json::Value>, usize) {
    let deadline = tokio::time::Instant::now() + timeout;
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

/// Read transformed events off `consumer` until every uuid in `want` has
/// arrived or `timeout` passes, returning every uuid seen.
///
/// Other uuids are allowed through: a consumer that rejoins its group after a
/// broker restart reads the topic again from the start.
async fn drain_uuids(
    consumer: &scalo::transport::kafka::KafkaTransport,
    want: &BTreeSet<String>,
    timeout: Duration,
) -> BTreeSet<String> {
    let deadline = tokio::time::Instant::now() + timeout;
    let mut seen = BTreeSet::new();
    while !seen.is_superset(want) {
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        if remaining.is_zero() {
            break;
        }
        seen.extend(uuids(&drain_for(consumer, 1, remaining).await.0));
    }
    seen
}

#[tokio::test(flavor = "multi_thread")]
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
    let consumer = AnyReceiver::Kafka(broker.consumer("service", &source_topic, &group).await);
    let producer = AnySender::Kafka(broker.producer("service").await);
    let manager = MetricsManager::new("dfe-transform-elastic-test");
    let metrics = TransformMetrics::register(&manager, "0.0.0-test", "test");
    let (shutdown, service) = start_loop(config, consumer, producer, metrics);

    let sink = broker
        .consumer("verify", &sink_topic, &common::topic("verify-cg"))
        .await;
    let (out, _) = drain(&sink, 5).await;

    stop_loop(&shutdown, service).await;

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
        let consumer = AnyReceiver::Kafka(broker.consumer("first", &source_topic, &group).await);
        let producer = AnySender::Kafka(broker.producer("first").await);
        let manager = MetricsManager::new("dfe-transform-elastic-test");
        let metrics = TransformMetrics::register(&manager, "0.0.0-test", "test");
        let (shutdown, service) = start_loop(config.clone(), consumer, producer, metrics);

        let sink = broker
            .consumer("verify", &sink_topic, &common::topic("commit-verify-cg"))
            .await;
        assert_eq!(drain(&sink, 3).await.0.len(), 3);

        stop_loop(&shutdown, service).await;
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

/// A batch at the SHIPPED `batch_size` arrives as one record per event.
///
/// Publishing a whole batch as one record fails once it passes the broker's
/// `message.max.bytes` -- and dfe-loader would reject the concatenation anyway.
/// The other tests in this file run a `batch_size` of 16 and never touched the
/// ceiling; this one uses the default and seeds several MB.
#[tokio::test(flavor = "multi_thread")]
async fn a_batch_larger_than_one_kafka_record_arrives() {
    /// 2,000 okta events is roughly 2.6 MB in and more out -- well past one
    /// Kafka record, without the elapsed time of a full 20,000.
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

    let consumer = AnyReceiver::Kafka(broker.consumer("service", &source_topic, &group).await);
    let producer = AnySender::Kafka(broker.producer("service").await);
    let manager = MetricsManager::new("dfe-transform-elastic-test");
    let metrics = TransformMetrics::register(&manager, "0.0.0-test", "test");
    let (shutdown, service) = start_loop(config, consumer, producer, metrics);

    let sink = broker
        .consumer("verify", &sink_topic, &common::topic("big-verify-cg"))
        .await;
    let (out, records) = drain(&sink, EVENTS).await;

    stop_loop(&shutdown, service).await;

    assert_eq!(out.len(), EVENTS, "the batch lost events");
    assert!(
        records >= EVENTS,
        "{records} records carried {EVENTS} events, so something was concatenated"
    );
    for event in &out {
        assert!(
            event.pointer("/okta/event_type").is_some(),
            "output was not transformed: {event}"
        );
    }
}

/// A sink broker down for longer than any fixed retry budget is waited out:
/// the loop never exits, the source offsets are held, and every event arrives
/// once the broker is back.
///
/// The service producer fails a delivery after one second, so each send comes
/// back refused quickly and repeatedly for the whole outage. A loop that gave
/// up after a fixed number of attempts -- eight, over about 25 seconds of
/// backoff -- exits long before the broker returns.
#[tokio::test(flavor = "multi_thread")]
async fn a_sink_outage_longer_than_a_retry_budget_is_held_not_crashed() {
    const EVENTS: usize = 5;
    const OUTAGE: Duration = Duration::from_secs(90);

    let Some(source) = common::Broker::container("outage-source").await else {
        common::require_broker_in_ci();
        eprintln!("SKIP: no container runtime.");
        return;
    };
    let Some(downstream) = common::Broker::container("outage-sink").await else {
        common::require_broker_in_ci();
        eprintln!("SKIP: no container runtime.");
        return;
    };
    let source_topic = common::topic("outage-src");
    let sink_topic = common::topic("outage-sink");
    let group = common::topic("outage-cg");

    let mut config = config(source.list(), &source_topic, &sink_topic, &group);
    config.sink.brokers = Some(downstream.list());

    let seed = source.producer("seed").await;
    let sent = seed
        .send(
            &source_topic,
            bytes::Bytes::from(okta_events_named("before", EVENTS)),
        )
        .await;
    assert!(matches!(sent, scalo::transport::SendResult::Ok), "{sent:?}");

    let consumer = AnyReceiver::Kafka(source.consumer("service", &source_topic, &group).await);
    let producer = AnySender::Kafka(
        downstream
            .producer_with("service", &[("message.timeout.ms", "1000")])
            .await,
    );
    let manager = scraped_manager();
    let metrics = TransformMetrics::register(&manager, "0.0.0-test", "test");
    let (shutdown, service) = start_loop(config, consumer, producer, metrics);

    let sink = downstream
        .consumer("verify", &sink_topic, &common::topic("outage-verify-cg"))
        .await;
    let (before, _) = drain(&sink, EVENTS).await;
    assert_eq!(
        uuids(&before),
        expected_uuids("before", EVENTS),
        "the loop is not running before the outage"
    );

    downstream.stop().await;
    let sent = seed
        .send(
            &source_topic,
            bytes::Bytes::from(okta_events_named("during", EVENTS)),
        )
        .await;
    assert!(matches!(sent, scalo::transport::SendResult::Ok), "{sent:?}");

    tokio::time::sleep(OUTAGE).await;
    assert!(!service.is_finished(), "the loop exited during the outage");
    // More refusals than the eight attempts a fixed budget would have allowed.
    let refused = scraped(&manager, "send_backpressure_total", "");
    assert!(
        refused > 8.0,
        "the sink refused {refused} sends, so the outage never outlasted a fixed budget"
    );

    downstream.start().await;
    let during = expected_uuids("during", EVENTS);
    let after = drain_uuids(&sink, &during, Duration::from_mins(3)).await;

    assert!(!service.is_finished(), "the loop exited after the outage");
    stop_loop(&shutdown, service).await;

    let lost: Vec<&String> = during.difference(&after).collect();
    assert!(
        lost.is_empty(),
        "events produced during the outage were lost: {lost:?}"
    );
}

/// A record the sink would refuse is dropped and counted, never counted
/// delivered, and never stalls the partition behind it.
///
/// The service producer's `message.max.bytes` is set below one padded event,
/// so the pipeline's screen takes that event out before the send while the
/// events beside it go through.
#[tokio::test(flavor = "multi_thread")]
async fn a_record_the_sink_refuses_is_counted_not_delivered() {
    const KEPT: usize = 3;
    /// Below `sink.max_message_bytes`, so the service's own budget lets the
    /// padded event through and the sink transport is what refuses it.
    const SINK_CEILING: &str = "262144";

    let broker = broker_or_skip!("refused-record");
    let source_topic = common::topic("refused-src");
    let sink_topic = common::topic("refused-sink");
    let group = common::topic("refused-cg");

    let seed = broker.producer("seed").await;
    for payload in [
        okta_events_named("kept", KEPT),
        okta_events_padded("refused", 1, 150_000),
    ] {
        let sent = seed.send(&source_topic, bytes::Bytes::from(payload)).await;
        assert!(matches!(sent, scalo::transport::SendResult::Ok), "{sent:?}");
    }

    let config = config(broker.list(), &source_topic, &sink_topic, &group);
    let consumer = AnyReceiver::Kafka(broker.consumer("service", &source_topic, &group).await);
    let producer = AnySender::Kafka(
        broker
            .producer_with("service", &[("message.max.bytes", SINK_CEILING)])
            .await,
    );
    let manager = scraped_manager();
    let metrics = TransformMetrics::register(&manager, "0.0.0-test", "test");
    let (shutdown, service) = start_loop(config.clone(), consumer, producer, metrics);

    let sink = broker
        .consumer("verify", &sink_topic, &common::topic("refused-verify-cg"))
        .await;
    let (mut out, _) = drain(&sink, KEPT).await;
    // Room for the refused event to turn up, which it must not.
    out.extend(drain_for(&sink, 1, Duration::from_secs(10)).await.0);

    stop_loop(&shutdown, service).await;

    assert_eq!(
        uuids(&out),
        expected_uuids("kept", KEPT),
        "the sink received something other than the events it can take"
    );
    let kept = f64::from(u32::try_from(KEPT).unwrap());
    assert!(
        (scraped(&manager, "records_delivered_total", "") - kept).abs() < f64::EPSILON,
        "the refused event was counted delivered"
    );
    assert!(
        (scraped(
            &manager,
            "pipeline_dead_letters_dropped_total",
            "reason=\"too_large\""
        ) - 1.0)
            .abs()
            < f64::EPSILON,
        "the refused event was not counted"
    );

    // The block was released despite the refusal, so nothing replays.
    let replay = broker.consumer("second", &source_topic, &group).await;
    let replayed = tokio::time::timeout(Duration::from_secs(15), replay.recv(64))
        .await
        .map_or(0, |r| r.map_or(0, |b| b.records.len()));
    assert_eq!(replayed, 0, "the refused record held its block unreleased");
}

/// The catalogue name dfe-engine renders into `source.name` for a `cisco-ios`
/// source on this app (`filebeat.{entry}.{transform}` in dfe-infra's
/// `apps.yaml`).
const CISCO_IOS_SOURCE: &str = "filebeat.cisco_ios.default";

/// What dfe-receiver stamps into `_source` when a source rule matches, and what
/// dfe-loader then routes the transformed row to that source's table on.
const CISCO_IOS_VARIANT: &str = "cisco-ios";

/// A `cisco_ios` syslog line unique to `i`, so a lost or duplicated event shows
/// up by value and not only by count. The shape is the napalm-logs sample in
/// `tests/fixtures/unencumbered/cisco_ios/cisco-ios-syslog.log`.
fn cisco_ios_line(i: usize) -> String {
    format!(
        "<189>{}: test-ztp: May 23 13:56:15.055: %SYS-5-CONFIG_I: Configured from console by \
         admin on vty0 (10.31.0.24)",
        30 + i
    )
}

/// The document an Elastic Agent hands the `cisco_ios` integration: the
/// Agent's wrapper keys around the vendor line, which the caller sets. The
/// values are made up.
fn agent_cisco_ios_document() -> serde_json::Value {
    serde_json::json!({
        "@timestamp": "2026-02-19T00:00:00.000Z",
        "agent": {
            "ephemeral_id": "0b5d3f4e-8f61-4b8e-9d0e-3c1f2a7b6c5d",
            "id": "6f1c2d3e-4b5a-4c6d-8e7f-9a0b1c2d3e4f",
            "name": "edge-agent-01",
            "type": "filebeat",
            "version": "8.0.0"
        },
        "data_stream": {
            "dataset": "cisco_ios.log",
            "namespace": "default",
            "type": "logs"
        },
        "elastic_agent": {
            "id": "6f1c2d3e-4b5a-4c6d-8e7f-9a0b1c2d3e4f",
            "snapshot": false,
            "version": "8.0.0"
        },
        "event": { "agent_id_status": "verified" },
        "input": { "type": "tcp" },
        "log": { "source": { "address": "192.0.2.10:46792" } },
        "tags": ["preserve_original_event", "cisco-ios", "forwarded"]
    })
}

/// `count` Elastic Agent `cisco_ios` documents as dfe-receiver writes them to a
/// source's land topic: the agent's own document with the receive stamp and
/// the matched `_source` appended, one Kafka record per event.
fn receiver_cisco_ios_records(count: usize) -> Vec<Vec<u8>> {
    let agent = agent_cisco_ios_document();

    (0..count)
        .map(|i| {
            let mut event = agent.clone();
            event["message"] = serde_json::Value::String(cisco_ios_line(i));
            event["_timestamp_receiver"] = serde_json::json!(1_757_000_000_000_u64);
            event["_source"] = serde_json::Value::String(CISCO_IOS_VARIANT.into());
            serde_json::to_vec(&event).expect("event serialises")
        })
        .collect()
}

/// Read Kafka records off the consumer's topic, unsplit, until they carry
/// `want` events or the timeout expires.
///
/// Returned as they came off the wire, so the assertions judge each record the
/// way dfe-loader does: one record, one JSON document.
async fn drain_records(
    consumer: &scalo::transport::kafka::KafkaTransport,
    want: usize,
) -> Vec<bytes::Bytes> {
    let deadline = tokio::time::Instant::now() + ROUND_TRIP_TIMEOUT;
    let mut records = Vec::new();
    let mut events = 0;

    while events < want && tokio::time::Instant::now() < deadline {
        let Ok(batch) = consumer.recv(64).await else {
            tokio::time::sleep(Duration::from_millis(200)).await;
            continue;
        };
        if batch.records.is_empty() {
            tokio::time::sleep(Duration::from_millis(200)).await;
            continue;
        }
        for record in batch.records {
            events += String::from_utf8_lossy(&record.payload)
                .lines()
                .filter(|line| !line.trim().is_empty())
                .count();
            records.push(record.payload);
        }
    }

    records
}

/// A multi-event batch through the service loop, on the receiver path the rc.14
/// filebeat e2e drives: every event reaches the sink as a record of its own,
/// each record is ONE JSON document, and each carries the `_source` dfe-loader
/// routes on (hyperi-io/dfe-transform-elastic#84).
///
/// A batch framed as one NDJSON record is dead-lettered whole by dfe-loader,
/// and a record without `_source` lands in the catch-all table. Both are silent
/// everywhere upstream of a row count.
#[tokio::test(flavor = "multi_thread")]
async fn a_receiver_batch_reaches_the_sink_as_one_routable_record_per_event() {
    const EVENTS: usize = 6;

    let broker = broker_or_skip!("receiver-batch");
    let source_topic = common::topic("rcv-src");
    let sink_topic = common::topic("rcv-sink");
    let group = common::topic("rcv-cg");

    let seed = broker.producer("seed").await;
    for record in receiver_cisco_ios_records(EVENTS) {
        let sent = seed.send(&source_topic, bytes::Bytes::from(record)).await;
        assert!(
            matches!(sent, scalo::transport::SendResult::Ok),
            "seed produce failed: {sent:?}"
        );
    }

    // Every seed record readable before the service starts, so its first
    // receive takes them as one batch rather than one at a time.
    {
        let probe = broker
            .consumer("probe", &source_topic, &common::topic("rcv-probe-cg"))
            .await;
        let deadline = tokio::time::Instant::now() + Duration::from_secs(30);
        let mut readable = 0;
        while readable < EVENTS && tokio::time::Instant::now() < deadline {
            match probe.recv(64).await {
                Ok(batch) if !batch.records.is_empty() => readable += batch.records.len(),
                _ => tokio::time::sleep(Duration::from_millis(200)).await,
            }
        }
        assert_eq!(
            readable, EVENTS,
            "the seed is not readable from the source topic"
        );
    }

    let mut config = config(broker.list(), &source_topic, &sink_topic, &group);
    config.source.name = CISCO_IOS_SOURCE.into();

    let consumer = AnyReceiver::Kafka(broker.consumer("service", &source_topic, &group).await);
    let producer = AnySender::Kafka(broker.producer("service").await);
    let manager = MetricsManager::new("dfe-transform-elastic-test");
    let metrics = TransformMetrics::register(&manager, "0.0.0-test", "test");
    let (shutdown, service) = start_loop(config, consumer, producer, metrics);

    let sink = broker
        .consumer("verify", &sink_topic, &common::topic("rcv-verify-cg"))
        .await;
    let records = drain_records(&sink, EVENTS).await;

    stop_loop(&shutdown, service).await;

    assert_eq!(
        records.len(),
        EVENTS,
        "{} record(s) carried the {EVENTS} events; the loader needs one per event",
        records.len()
    );

    let mut originals = Vec::with_capacity(EVENTS);
    for payload in &records {
        // The loader's own parse: the whole record as ONE value.
        let event: serde_json::Value = serde_json::from_slice(payload).unwrap_or_else(|e| {
            panic!(
                "a record is not one JSON document ({e}): {}",
                String::from_utf8_lossy(payload)
            )
        });
        assert!(event.is_object(), "a record is not an object: {event}");
        assert_eq!(
            event.get("_source").and_then(|v| v.as_str()),
            Some(CISCO_IOS_VARIANT),
            "the routing field dfe-loader matches on did not survive: {event}"
        );
        assert_eq!(
            event.pointer("/observer/vendor").and_then(|v| v.as_str()),
            Some("Cisco"),
            "the record did not go through the cisco_ios transform: {event}"
        );
        originals.push(
            event
                .pointer("/event/original")
                .and_then(|v| v.as_str())
                .expect("event.original carries the vendor line")
                .to_string(),
        );
    }

    originals.sort();
    let mut expected: Vec<String> = (0..EVENTS).map(cisco_ios_line).collect();
    expected.sort();
    assert_eq!(originals, expected, "an event was lost or duplicated");
}

/// `len` bytes no codec can shrink.
fn incompressible(len: usize) -> Vec<u8> {
    let mut state: u64 = 0x9E37_79B9_7F4A_7C15;
    (0..len)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state.to_le_bytes()[0]
        })
        .collect()
}

/// A stock broker, whose `message.max.bytes` is about 1 MB, refuses a record
/// the 15 MiB budget passes. The sink reports that as a dead letter, so the
/// loop drops and counts the record rather than retrying the partition forever.
///
/// The producer allows 16 MiB and compresses, and the broker judges the
/// compressed batch, so only an incompressible record reaches it at size. A
/// container, never a live broker: a live one may carry the stack's ceiling.
#[tokio::test(flavor = "multi_thread")]
async fn a_stock_broker_refuses_an_oversize_record_as_a_dead_letter() {
    let Some(broker) = common::Broker::container("oversize-record").await else {
        common::require_broker_in_ci();
        eprintln!("SKIP: no container runtime.");
        return;
    };
    let topic = common::topic("oversize-record");
    let producer = broker.producer("oversize-record").await;

    let sent = producer
        .send(&topic, bytes::Bytes::from(incompressible(1_100_000)))
        .await;

    assert!(
        matches!(sent, scalo::transport::SendResult::FilteredDlq),
        "a stock broker's refusal of a 1.1 MB record must come back as a dead letter: {sent:?}"
    );
}

/// On a broker at the stack's 16 MiB ceiling, an event over a stock broker's
/// 1 MB reaches the sink whole instead of being dropped as oversize.
#[tokio::test(flavor = "multi_thread")]
async fn an_event_over_one_megabyte_arrives_on_a_broker_at_the_stack_ceiling() {
    /// Past a stock broker's ceiling, and well inside the shipped budget.
    const PAD: usize = 2_000_000;

    let Some(broker) = common::Broker::container_with_ceiling(
        "large-event",
        dfe_transform_elastic::config::PIPELINE_MESSAGE_MAX_BYTES,
    )
    .await
    else {
        common::require_broker_in_ci();
        eprintln!("SKIP: no container runtime.");
        return;
    };
    let source_topic = common::topic("large-src");
    let sink_topic = common::topic("large-sink");
    let group = common::topic("large-cg");

    let seed = broker.producer("seed").await;
    let sent = seed
        .send(
            &source_topic,
            bytes::Bytes::from(okta_events_padded("large", 1, PAD)),
        )
        .await;
    assert!(matches!(sent, scalo::transport::SendResult::Ok), "{sent:?}");

    let config = config(broker.list(), &source_topic, &sink_topic, &group);
    let consumer = AnyReceiver::Kafka(broker.consumer("service", &source_topic, &group).await);
    let producer = AnySender::Kafka(broker.producer("service").await);
    let manager = scraped_manager();
    let metrics = TransformMetrics::register(&manager, "0.0.0-test", "test");
    let (shutdown, service) = start_loop(config, consumer, producer, metrics);

    let sink = broker
        .consumer("verify", &sink_topic, &common::topic("large-verify-cg"))
        .await;
    let (out, _) = drain(&sink, 1).await;

    stop_loop(&shutdown, service).await;

    assert_eq!(
        uuids(&out),
        expected_uuids("large", 1),
        "the large event did not arrive"
    );
    let carried = out[0]
        .pointer("/okta/display_message")
        .and_then(|v| v.as_str())
        .map_or(0, str::len);
    assert_eq!(carried, PAD, "the event arrived without its padding");
    assert!(
        scraped(&manager, "events_oversize_total", "").abs() < f64::EPSILON,
        "the large event was counted oversize"
    );
}

/// The container this suite starts must carry a traceable name and the suite
/// label, so an operator can tell what left it behind.
#[tokio::test(flavor = "multi_thread")]
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
