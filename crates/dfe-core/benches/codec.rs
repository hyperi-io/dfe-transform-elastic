// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    reason = "a bench asserts its fixture by panicking rather than carrying error handling into the measured path"
)]

//! The JSON codec on the record path: `Event::from_json` decodes each inbound
//! line and `Event::write_json` encodes each outbound event, once per record.
//!
//! Both halves run on the licence-clean okta System Log sample, all 120 events:
//!
//! - `decode` reads each event as it arrives, the vendor payload a STRING in a
//!   Beats `message`;
//! - `encode` writes each vendor document, parsed, as the nested object an ECS
//!   output is.
//!
//! Throughput is per record, so criterion's time is per the 120 and its rate is
//! records a second.
//!
//! Run with: `cargo bench -p dfe-core --bench codec`

use std::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use dfe_core::Event;
use serde_json::Value;

/// The okta System Log sample, one vendor document per line.
fn vendor_lines() -> Vec<String> {
    let raw = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/unencumbered/okta/panther-okta-systemlog.ndjson"
    ))
    .expect("the okta sample is committed");
    raw.lines()
        .filter(|line| !line.trim().is_empty())
        .map(String::from)
        .collect()
}

/// Each vendor line as a Beats event carries it: a string in `message`.
fn beats_lines(vendor: &[String]) -> Vec<String> {
    vendor
        .iter()
        .map(|line| serde_json::json!({ "message": line }).to_string())
        .collect()
}

/// Decode, encode, and both together as one record costs them.
fn codec(c: &mut Criterion) {
    let vendor = vendor_lines();
    let lines = beats_lines(&vendor);
    let documents: Vec<Event> = vendor
        .iter()
        .map(|line| Event::new(serde_json::from_str::<Value>(line).unwrap()))
        .collect();
    let records = u64::try_from(lines.len()).unwrap();

    let mut group = c.benchmark_group("codec");
    group.throughput(Throughput::Elements(records));

    group.bench_function("decode", |b| {
        b.iter(|| {
            for line in &lines {
                black_box(Event::from_json(black_box(line)).unwrap());
            }
        });
    });

    group.bench_function("encode", |b| {
        let mut out = Vec::with_capacity(4096);
        b.iter(|| {
            for document in &documents {
                out.clear();
                document.write_json(&mut out).unwrap();
                black_box(&out);
            }
        });
    });

    group.bench_function("record", |b| {
        let mut out = Vec::with_capacity(4096);
        b.iter(|| {
            for (line, document) in lines.iter().zip(&documents) {
                black_box(Event::from_json(black_box(line)).unwrap());
                out.clear();
                document.write_json(&mut out).unwrap();
                black_box(&out);
            }
        });
    });

    group.finish();
}

criterion_group!(benches, codec);
criterion_main!(benches);
