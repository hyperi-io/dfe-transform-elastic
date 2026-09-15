// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! PGO workload driver: fixture-derived events onto a Kafka topic.
//!
//! Produces the committed Beats fixtures as NDJSON records so a running,
//! PGO-instrumented `dfe-transform-elastic` accumulates profile data over its
//! real hot path -- consume, parse the batch, detect the envelope, transform,
//! serialise, produce.
//!
//! The payloads are captured vendor data rather than synthesised text: this
//! service carries one generated module per data stream, so a profile is only
//! representative if the events actually reach the transform the instance
//! resolved. Breadth across sources belongs to `scripts/pgo-workload.sh`, which
//! restarts the service once per source -- one instance applies ONE transform
//! to everything that arrives, so feeding it another source's payloads profiles
//! the error paths instead.
//!
//! Built only with `--features pgo-driver`; the service binary is unaffected.
//!
//! Configuration is environment-only:
//! - `PGO_DRIVER_DURATION_SECS` (default 300) -- how long to produce for
//! - `PGO_DRIVER_BROKERS` (default `127.0.0.1:19092`)
//! - `PGO_DRIVER_TOPIC` (default `pgo_source`) -- the topic the service consumes
//! - `PGO_DRIVER_RPS` (default 5000) -- EVENTS per second, not records
//! - `PGO_DRIVER_FIXTURES` -- comma-separated `.log` paths, required
//!
//! Exit codes: 0 when the workload ran its full duration, 1 on a setup failure
//! (no fixtures, no events in them, or a producer that would not start).

// A load driver reports by printing and stops by exiting; neither is library
// code, and the workspace bans the panic that `expect` would otherwise add.
#![allow(clippy::cast_precision_loss)]

use std::time::{Duration, Instant};

use scalo::transport::kafka::{KafkaConfig, KafkaProducer, ProducerProfile};

/// Events packed into one Kafka record, matching the NDJSON framing Beats
/// delivers and the service's `parse_batch` reads.
const EVENTS_PER_RECORD: usize = 500;

/// Byte ceiling on one record, applied with the event cap so a source with
/// large events cannot build a record librdkafka rejects as oversize.
const MAX_RECORD_BYTES: usize = 256 * 1024;

/// How often the driver prints throughput.
const REPORT_INTERVAL: Duration = Duration::from_secs(15);

/// The pacing window: the rate is applied in slices this long rather than
/// sleeping per event, which would cost a syscall per message.
const PACE_WINDOW_MS: u64 = 10;

/// The pacing window as a [`Duration`].
const PACE_WINDOW: Duration = Duration::from_millis(PACE_WINDOW_MS);

fn main() {
    let config = Config::from_env();
    println!("pgo-driver starting: {config:?}");

    let records = match build_records(&config.fixtures) {
        Ok(records) => records,
        Err(e) => {
            eprintln!("pgo-driver: {e}");
            std::process::exit(1);
        }
    };
    println!(
        "pgo-driver: {} records from {} fixture(s)",
        records.len(),
        config.fixtures.len()
    );

    let producer =
        match KafkaProducer::new(&producer_config(&config), ProducerProfile::HighThroughput) {
            Ok(producer) => producer,
            Err(e) => {
                eprintln!("pgo-driver: producer init failed: {e}");
                std::process::exit(1);
            }
        };

    let stats = run(&producer, &config, &records);

    // Outstanding records still count toward the profile, so the flush is part
    // of the workload rather than tidy-up.
    let in_flight = producer.flush(Duration::from_secs(10));
    if in_flight > 0 {
        eprintln!("pgo-driver: {in_flight} records still in flight after the flush");
    }

    stats.report();
    println!("pgo-driver: complete");
}

/// The environment contract, resolved once.
#[derive(Debug)]
struct Config {
    duration: Duration,
    brokers: String,
    topic: String,
    rps: u64,
    fixtures: Vec<String>,
}

impl Config {
    fn from_env() -> Self {
        Self {
            duration: Duration::from_secs(env_u64("PGO_DRIVER_DURATION_SECS", 300)),
            brokers: env_str("PGO_DRIVER_BROKERS", "127.0.0.1:19092"),
            topic: env_str("PGO_DRIVER_TOPIC", "pgo_source"),
            rps: env_u64("PGO_DRIVER_RPS", 5000).max(1),
            fixtures: env_str("PGO_DRIVER_FIXTURES", "")
                .split(',')
                .map(str::trim)
                .filter(|path| !path.is_empty())
                .map(String::from)
                .collect(),
        }
    }
}

fn env_str(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn env_u64(key: &str, default: u64) -> u64 {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

/// A producer pointed at the workload broker, with scalo owning every knob.
fn producer_config(config: &Config) -> KafkaConfig {
    KafkaConfig {
        brokers: vec![config.brokers.clone()],
        client_id: "dfe-transform-elastic-pgo-driver".to_string(),
        ..Default::default()
    }
}

/// Fixture lines, wrapped and framed into the records a Beats producer sends.
///
/// Each line is the raw vendor payload and becomes one event as
/// `{"message": <line>}`, which is how Beats and Elastic Agent deliver it and
/// what the transforms are written against.
fn build_records(fixtures: &[String]) -> Result<Vec<Vec<u8>>, String> {
    if fixtures.is_empty() {
        return Err("PGO_DRIVER_FIXTURES is empty; name at least one .log fixture".to_string());
    }

    let mut records = Vec::new();
    let mut current: Vec<u8> = Vec::new();
    let mut events_in_current = 0_usize;

    for path in fixtures {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| format!("fixture {path} could not be read: {e}"))?;

        for line in raw.lines().filter(|line| !line.trim().is_empty()) {
            let event = serde_json::json!({ "message": line });
            let mut bytes = serde_json::to_vec(&event)
                .map_err(|e| format!("fixture {path} holds a line that will not serialise: {e}"))?;
            bytes.push(b'\n');

            if !current.is_empty()
                && (events_in_current >= EVENTS_PER_RECORD
                    || current.len() + bytes.len() > MAX_RECORD_BYTES)
            {
                records.push(std::mem::take(&mut current));
                events_in_current = 0;
            }

            current.extend_from_slice(&bytes);
            events_in_current += 1;
        }
    }

    if !current.is_empty() {
        records.push(current);
    }
    if records.is_empty() {
        return Err("the named fixtures hold no events".to_string());
    }
    Ok(records)
}

/// Produce the records in a ring until the duration is spent.
fn run(producer: &KafkaProducer, config: &Config, records: &[Vec<u8>]) -> Stats {
    let mut stats = Stats::new();
    let deadline = stats.start + config.duration;
    let mut next_report = stats.start + REPORT_INTERVAL;
    let mut index = 0_usize;

    // Events per pacing window, rounded up so a rate below one window's worth
    // still produces rather than stalling on an integer-divided zero.
    let events_per_window = (config.rps * PACE_WINDOW_MS).div_ceil(1000).max(1);

    while Instant::now() < deadline {
        let window_start = Instant::now();
        let mut events_this_window = 0_u64;

        while events_this_window < events_per_window {
            let record = &records[index % records.len()];
            index = index.wrapping_add(1);

            let key = (index % 1024).to_string();
            match producer.send(&config.topic, Some(key.as_bytes()), record) {
                Ok(()) => {
                    stats.records_sent += 1;
                    stats.events_sent += count_events(record);
                }
                Err(e) => {
                    stats.send_errors += 1;
                    stats.last_error = Some(e.to_string());
                    // A full queue is backpressure, not a fault; yielding lets
                    // librdkafka drain rather than spinning on the error.
                    std::thread::sleep(Duration::from_millis(1));
                }
            }
            events_this_window += count_events(record);
        }

        if Instant::now() >= next_report {
            stats.report();
            next_report = Instant::now() + REPORT_INTERVAL;
        }

        if let Some(remaining) = PACE_WINDOW.checked_sub(window_start.elapsed()) {
            std::thread::sleep(remaining);
        }
    }

    stats
}

/// Events in one NDJSON record, which is its line count.
fn count_events(record: &[u8]) -> u64 {
    record.iter().filter(|byte| **byte == b'\n').count() as u64
}

/// What the run produced, printed periodically and once at the end.
struct Stats {
    start: Instant,
    records_sent: u64,
    events_sent: u64,
    send_errors: u64,
    last_error: Option<String>,
}

impl Stats {
    fn new() -> Self {
        Self {
            start: Instant::now(),
            records_sent: 0,
            events_sent: 0,
            send_errors: 0,
            last_error: None,
        }
    }

    fn report(&self) {
        let elapsed = self.start.elapsed().as_secs_f64().max(1.0);
        println!(
            "pgo-driver [{:>6.1}s] records={} events={} errors={} rate={:.0} events/s",
            self.start.elapsed().as_secs_f64(),
            self.records_sent,
            self.events_sent,
            self.send_errors,
            self.events_sent as f64 / elapsed,
        );
        if let Some(error) = &self.last_error {
            println!("pgo-driver: most recent send error: {error}");
        }
    }
}
