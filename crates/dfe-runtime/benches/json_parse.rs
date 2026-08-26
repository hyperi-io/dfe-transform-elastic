// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Reading the vendor payload, which every JSON-carrying source does first.
//!
//! A SIMD parser is the house default for this work, settled by bake-off rather
//! than reputation. The crate is `sonic-rs`, which dfe-loader runs; simd-json
//! was measured against it there and rejected, because parsing in place needs
//! `&mut [u8]` and a pipeline holding payloads as `Arc<[u8]>` pays a memcpy per
//! message for it.
//!
//! This bench is why the tree is still on `serde_json`: the allocation profiler
//! puts `parse_json_str` at 357 allocations an event on okta against 179 for
//! `serde_json::from_str` over the same document. `sonic-rs` has not been
//! measured here yet, and preserving object insertion order decides whether it
//! can be.
//!
//! The payloads are the committed fixtures, so a fresh clone measures the same
//! documents.

use std::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use serde_json::Value;

/// One representative payload per shape: a big nested vendor document, and a
/// small flat one.
fn payloads() -> Vec<(&'static str, String)> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures");
    [
        ("okta", "okta/system/test-okta-system-events.log"),
        ("meraki", "cisco/meraki/logs/test-events.log"),
    ]
    .iter()
    .filter_map(|(name, relative)| {
        let raw = std::fs::read_to_string(root.join(relative)).ok()?;
        let line = raw.lines().find(|l| !l.trim().is_empty())?;
        Some((*name, line.to_string()))
    })
    .collect()
}

/// The scratch-buffer copy plus simd-json, against `serde_json` straight off
/// the string.
fn parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_parse");

    for (name, text) in payloads() {
        group.throughput(Throughput::Bytes(text.len() as u64));

        group.bench_function(format!("{name}/serde_json"), |b| {
            b.iter(|| black_box(serde_json::from_str::<Value>(black_box(&text))));
        });

        // The same shape `parse_json_str` uses: a reused buffer, so the copy
        // does not allocate.
        group.bench_function(format!("{name}/simd_json"), |b| {
            let mut buffer: Vec<u8> = Vec::with_capacity(text.len());
            b.iter(|| {
                buffer.clear();
                buffer.extend_from_slice(black_box(&text).as_bytes());
                black_box(simd_json::serde::from_slice::<Value>(&mut buffer).ok())
            });
        });

        // What the runtime actually calls, so the wrapper's own cost is
        // visible rather than assumed away.
        group.bench_function(format!("{name}/parse_json_str"), |b| {
            b.iter(|| black_box(dfe_runtime::codegen_api::parse_json_str(black_box(&text))));
        });
    }

    group.finish();
}

/// The Kafka ingestion path, which is a different question again.
///
/// `Event::from_bytes` builds a simd-json `OwnedValue` and then converts it to
/// a `serde_json::Value` through `serde_json::to_value` -- two complete trees
/// for one document. Whether that is worth it against parsing the bytes
/// directly is what this asks.
fn ingest(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_ingest");

    for (name, text) in payloads() {
        group.throughput(Throughput::Bytes(text.len() as u64));

        group.bench_function(format!("{name}/serde_json_slice"), |b| {
            b.iter(|| black_box(serde_json::from_slice::<Value>(black_box(text.as_bytes()))));
        });

        group.bench_function(format!("{name}/simd_owned_then_to_value"), |b| {
            let mut buffer: Vec<u8> = Vec::with_capacity(text.len());
            b.iter(|| {
                buffer.clear();
                buffer.extend_from_slice(black_box(&text).as_bytes());
                let owned = simd_json::to_owned_value(&mut buffer).ok();
                black_box(owned.and_then(|o| serde_json::to_value(&o).ok()))
            });
        });
    }

    group.finish();
}

criterion_group!(benches, parse, ingest);
criterion_main!(benches);
