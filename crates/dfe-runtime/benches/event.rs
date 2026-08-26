// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    reason = "a bench asserts its fixture by panicking rather than carrying error handling into the measured path"
)]

//! The dotted-path accessors, which every processor in every transform calls.
//!
//! `rename` 12,275, `set` 8,419, `append` 8,380, `convert` 4,102 and `remove`
//! 3,618 across the vendored pipelines, against `script` 1,242 and `grok` 640.
//! The hot path is field plumbing, so whatever one path walk costs is the
//! number that multiplies out to a batch: an okta event alone does hundreds.
//!
//! What these separate:
//!
//! - the FLOOR -- one `serde_json::Map` lookup, which is what a resolved path
//!   would cost if the walk itself were free;
//! - the walk, at one, two and three segments, so the per-segment cost is
//!   visible rather than inferred;
//! - a MISS, which is the common case and not the rare one. A generated
//!   transform guards nearly every processor with `if event.has(...)`, and
//!   most of those fields are absent on most events.
//!
//! `serde_json` carries `preserve_order` here, which is not optional -- the
//! engine's own maps are insertion-ordered and parity depends on it -- so the
//! map underneath is an `IndexMap` and every lookup hashes its key.

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use dfe_runtime::Event;
use serde_json::{Value, json};

/// An event shaped like the ones the transforms actually see: a wide root, a
/// few nested objects, and the vendor payload still present.
fn realistic() -> Event {
    Event::new(json!({
        "@timestamp": "2025-03-24T06:57:59.852Z",
        "message": "the raw vendor line, which is usually the biggest field",
        "event": {"action": "user.session.start", "outcome": "success", "kind": "event"},
        "source": {"ip": "10.0.0.7", "port": 5601,
                   "geo": {"city_name": "Sydney", "country_iso_code": "AU"}},
        "destination": {"ip": "67.43.156.13", "port": 443},
        "user": {"name": "someone", "id": "00u1abvz4pYqdM8ms4x6"},
        "host": {"name": "docker-fleet-agent", "os": {"type": "linux"}},
        "network": {"transport": "tcp", "protocol": "https"},
        "url": {"domain": "example.com", "path": "/api/v1/thing"},
        "tags": ["preserve_original_event"],
        "ecs": {"version": "8.11.0"},
    }))
}

/// One lookup in the map that backs the root object.
///
/// The floor: whatever a path walk costs, it cannot be cheaper than this, and
/// the gap between them is the walk's own overhead.
fn map_floor(c: &mut Criterion) {
    let event = realistic();
    let Some(Value::Object(root)) = Some(event.as_value()) else {
        panic!("the fixture is an object");
    };

    let mut group = c.benchmark_group("event_floor");
    group.bench_function("map_get_hit", |b| {
        b.iter(|| black_box(root.get(black_box("message"))));
    });
    group.bench_function("map_get_miss", |b| {
        b.iter(|| black_box(root.get(black_box("nothing_here"))));
    });
    group.finish();
}

/// The walk itself, by depth and by outcome.
fn path_walk(c: &mut Criterion) {
    let event = realistic();

    let mut group = c.benchmark_group("event_get");
    for (name, path) in [
        ("depth1/hit", "message"),
        ("depth2/hit", "event.action"),
        ("depth3/hit", "source.geo.country_iso_code"),
        // A miss at the ROOT is the cheap one -- nothing below it is walked.
        ("depth1/miss", "nothing_here"),
        // A miss at the LEAF has already paid for the walk down.
        ("depth3/miss", "source.geo.region_name"),
        // A miss whose FIRST segment is present but whose second is not: this
        // is the shape `flat_key` exists for, and it scans every key.
        ("depth2/miss", "source.as_number"),
    ] {
        group.bench_function(name, |b| {
            b.iter(|| black_box(event.get(black_box(path))));
        });
    }
    group.finish();
}

/// `has` is what a generated guard calls, so it is on the path of nearly every
/// processor whether the field is there or not.
fn presence(c: &mut Criterion) {
    let event = realistic();

    let mut group = c.benchmark_group("event_has");
    group.bench_function("present", |b| {
        b.iter(|| black_box(event.has(black_box("source.geo.city_name"))));
    });
    group.bench_function("absent", |b| {
        b.iter(|| black_box(event.has(black_box("threat.indicator.ip"))));
    });
    group.finish();
}

/// Writing, which auto-creates the objects on the way down.
fn writes(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_set");

    group.bench_function("existing_leaf", |b| {
        let mut event = realistic();
        b.iter(|| {
            let _ = event.set(black_box("source.geo.city_name"), black_box("Melbourne"));
        });
    });

    group.bench_function("new_leaf_existing_parents", |b| {
        let mut event = realistic();
        b.iter(|| {
            let _ = event.set(black_box("source.geo.region_name"), black_box("NSW"));
        });
    });

    group.bench_function("new_branch", |b| {
        b.iter_batched(
            realistic,
            |mut event| {
                let _ = event.set(
                    black_box("threat.indicator.marking.tlp"),
                    black_box("AMBER"),
                );
                event
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Removing, which the generated transforms do constantly -- every `remove`
/// processor, and the cleanup at the end of nearly every pipeline.
fn removes(c: &mut Criterion) {
    let mut group = c.benchmark_group("event_remove");

    group.bench_function("depth1/present", |b| {
        b.iter_batched(
            realistic,
            |mut event| {
                let taken = event.remove(black_box("message"));
                (event, taken)
            },
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("depth3/present", |b| {
        b.iter_batched(
            realistic,
            |mut event| {
                let taken = event.remove(black_box("source.geo.city_name"));
                (event, taken)
            },
            criterion::BatchSize::SmallInput,
        );
    });

    // The common case: a pipeline's cleanup names fields most events never
    // had, so the miss is what runs.
    group.bench_function("depth2/absent", |b| {
        let mut event = realistic();
        b.iter(|| black_box(event.remove(black_box("threat.indicator"))));
    });

    group.finish();
}

criterion_group!(benches, map_floor, path_walk, presence, writes, removes);
criterion_main!(benches);
