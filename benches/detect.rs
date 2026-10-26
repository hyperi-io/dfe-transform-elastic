// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! What agentless detection costs per event.
//!
//! Detects the first 20,000 lines of the compat corpus, the shipped
//! `batch_size`, so the figure is the batch loop's per-event addition. The
//! corpus mixes every source, so the table sees the lines it will see in
//! production. Without `testdata/compat` the bench has nothing to run on.
//!
//! Run with: `cargo bench --bench detect`

#![allow(missing_docs, clippy::expect_used)]

#[path = "../tests/detect_corpus/loader.rs"]
mod loader;

use std::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use dfe_runtime::Event;
use dfe_transform_elastic::detect::Detector;
use serde_json::json;

const BATCH: usize = 20_000;

fn bench_detect(c: &mut Criterion) {
    let Some(lines) = loader::load() else {
        eprintln!(
            "no compat corpus at {}, skipping",
            loader::corpus_root().display()
        );
        return;
    };
    // An even stride across the sorted corpus, so a source is not over-weighted
    // by sitting first.
    let stride = (lines.len() / BATCH).max(1);
    let events: Vec<Event> = lines
        .iter()
        .step_by(stride)
        .take(BATCH)
        .map(|l| Event::new(json!({ "message": l.text })))
        .collect();
    let detector = Detector::new(&[]).expect("every source");

    let mut group = c.benchmark_group("detect");
    group.throughput(Throughput::Elements(events.len() as u64));
    group.bench_function("batch_20000", |b| {
        let mut scratch = String::new();
        b.iter(|| {
            for event in &events {
                black_box(detector.detect_event(black_box(event), &mut scratch));
            }
        });
    });
    group.finish();
}

criterion_group!(benches, bench_detect);
criterion_main!(benches);
