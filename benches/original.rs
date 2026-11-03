// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! What settling `event.original` costs per event, on parsed fortinet output.
//!
//! The service pays it once per emitted event, after the transform returns.
//! The events are the committed fortigate samples already run through their
//! transform, so each one is Removed. The clone that gives each iteration its
//! own event is set-up and is not timed.
//!
//! Run with: `cargo bench --bench original`

// criterion_group! expands to an undocumented function, and a bench asserts its
// setup by panicking the same way a test does.
#![allow(missing_docs, clippy::expect_used)]

use std::hint::black_box;
use std::path::Path;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use dfe_runtime::Event;
use dfe_runtime::transform::{Original, settle_original};
use dfe_transform_elastic::registry;
use serde_json::json;

/// Parsed fortinet events, each still holding its `event.original`.
fn parsed_events() -> Vec<Event> {
    let transform = registry::lookup("filebeat.fortinet.default").expect("registered");
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/unencumbered/fortinet");
    let mut files: Vec<_> = std::fs::read_dir(path)
        .expect("fortinet samples")
        .flatten()
        .map(|entry| entry.path())
        .collect();
    files.sort();

    let mut events = Vec::new();
    for file in files {
        let text = std::fs::read_to_string(file).expect("sample reads");
        for line in text.lines().filter(|line| !line.trim().is_empty()) {
            let mut event = Event::new(json!({ "message": line }));
            if transform.transform(&mut event).is_ok() && event.has("event.original") {
                events.push(event);
            }
        }
    }
    assert!(!events.is_empty(), "no fortinet event parsed");
    events
}

/// The fortinet sample lines, as the service receives them.
fn lines() -> Vec<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/unencumbered/fortinet");
    let mut files: Vec<_> = std::fs::read_dir(path)
        .expect("fortinet samples")
        .flatten()
        .map(|entry| entry.path())
        .collect();
    files.sort();
    let mut out = Vec::new();
    for file in files {
        let text = std::fs::read_to_string(file).expect("sample reads");
        out.extend(
            text.lines()
                .filter(|l| !l.trim().is_empty())
                .map(str::to_owned),
        );
    }
    out
}

fn parts(c: &mut Criterion) {
    let events = parsed_events();
    let lines = lines();
    let transform = registry::lookup("filebeat.fortinet.default").expect("registered");
    let mut i = 0;
    c.bench_function("parts/fortinet_transform", |b| {
        b.iter_batched(
            || {
                i = (i + 1) % lines.len();
                Event::new(json!({ "message": lines[i] }))
            },
            |mut event| {
                black_box(transform.transform(black_box(&mut event)).is_ok());
                event
            },
            BatchSize::SmallInput,
        );
    });
    let mut j = 0;
    c.bench_function("parts/tags_lookup", |b| {
        b.iter_batched(
            || {
                j = (j + 1) % events.len();
                events[j].clone()
            },
            |event| {
                black_box(event.get("tags").is_some());
                event
            },
            BatchSize::SmallInput,
        );
    });
    let mut k = 0;
    c.bench_function("parts/remove_only", |b| {
        b.iter_batched(
            || {
                k = (k + 1) % events.len();
                events[k].clone()
            },
            |mut event| {
                black_box(event.remove("event.original"));
                event
            },
            BatchSize::SmallInput,
        );
    });
}

fn settle(c: &mut Criterion) {
    let events = parsed_events();
    for event in &events {
        assert_eq!(settle_original(&mut event.clone()), Original::Removed);
    }
    let mut next = 0;
    // The same loop with nothing timed inside it, so the settle cost is the
    // difference between the two.
    let mut idle = 0;
    c.bench_function("settle_original/baseline_no_op", |b| {
        b.iter_batched(
            || {
                idle = (idle + 1) % events.len();
                events[idle].clone()
            },
            black_box,
            BatchSize::SmallInput,
        );
    });
    c.bench_function("settle_original/fortinet_removed", |b| {
        b.iter_batched(
            || {
                next = (next + 1) % events.len();
                events[next].clone()
            },
            |mut event| {
                black_box(settle_original(black_box(&mut event)));
                event
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, settle, parts);
criterion_main!(benches);
