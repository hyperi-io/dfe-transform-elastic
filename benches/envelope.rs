// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! What the envelope layer costs, and whether a declared rename list can be
//! afforded on the hot path.
//!
//! Two separate questions, and they have different answers because they run at
//! different rates. Detection runs ONCE PER BATCH -- one partition has one
//! producer -- so `detect/*` is amortised over the whole batch. Unwrapping runs
//! PER EVENT, so `unwrap/*` is what a config-declared spec would have to pay
//! 20,000 times.
//!
//! `renames/*` is the question itself: a hand-written arm per field, against a
//! `&'static [(&str, &str)]` walked in a loop, against a `Vec<(String, String)>`
//! built at startup from a spec. If the third is within noise of the first, a
//! declared list costs nothing and the flexibility is free.
//!
//! Measured 2026-08-20 on the dev box, `--measurement-time 3`:
//!
//! | bench | ns | rate |
//! |---|---|---|
//! | `detect/receiver` | 53 | per BATCH |
//! | `detect/beats_bare` | 80 | per BATCH |
//! | `unwrap/beats` | 6.7 | per event |
//! | `unwrap/receiver_body` | 2,094 | per event |
//! | `unwrap/receiver_line` | 2,740 | per event |
//! | `renames/hand_written` | 1,689 | per event |
//! | `renames/static_list` | 1,688 | per event |
//! | `renames/compiled_spec` | 1,708 | per event |
//! | `renames/collected_vec` | 1,818 | per event |
//!
//! A declared spec costs 1% over hand-written arms, so config-driven unwrapping
//! is affordable. The `Vec` the library used to build per event cost 7.7%, and
//! is gone. What dominates all four is `Event::get` and `Event::set` walking a
//! dotted path -- roughly 210 ns a pair -- so that, not the spec format, is
//! where the next order of magnitude is.
//!
//! Run with: `cargo bench --bench envelope`

// criterion_group! expands to an undocumented function, and a bench asserts its
// setup by panicking the same way a test does.
#![allow(missing_docs, clippy::expect_used)]

use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use dfe_runtime::Event;
use dfe_transform_elastic::envelope::{self, Envelope, EnvelopeSetting};
use dfe_transform_elastic::registry::{self, Framing};
use serde_json::{Value, json};

/// dfe-receiver's syslog output, field for field.
fn receiver_event() -> Event {
    Event::new(json!({
        "message": "%SEC-6-IPACCESSLOGRP: list 177 denied igmp 192.168.100.197",
        "facility": "local4",
        "severity": "notice",
        "timestamp": "2026-03-03T10:30:00+11:00",
        "hostname": "web01",
        "appname": "nginx",
        "procid": "1234",
        "msgid": "ID47",
        "_source": "syslog"
    }))
}

fn beats_event() -> Event {
    Event::new(json!({
        "message": "Feb  8 04:00:48 192.168.100.2 585917: %SEC-6-IPACCESSLOGRP: list 177 denied",
        "tags": ["preserve_original_event"]
    }))
}

/// The batch-rate question: one probe against a whole default-sized batch.
fn bench_detect(c: &mut Criterion) {
    let intake = registry::intake("filebeat.cisco_ios.default").expect("registered");

    let mut group = c.benchmark_group("detect");
    for (name, event) in [
        ("beats_bare", beats_event()),
        ("receiver", receiver_event()),
    ] {
        group.bench_function(name, |b| {
            b.iter(|| black_box(envelope::detect(black_box(&event))));
        });
    }
    group.bench_function("resolve", |b| {
        let event = receiver_event();
        b.iter(|| {
            black_box(envelope::resolve(
                EnvelopeSetting::Auto,
                Some(black_box(&event)),
                intake,
                "cisco_ios.log",
            ))
        });
    });
    group.finish();
}

/// The event-rate question: what each envelope costs to remove, once.
fn bench_unwrap(c: &mut Criterion) {
    let mut group = c.benchmark_group("unwrap");

    group.bench_function("beats", |b| {
        b.iter_batched_ref(
            beats_event,
            |e| Envelope::Beats.unwrap_into_beats(e, None, "bare"),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("receiver_line", |b| {
        b.iter_batched_ref(
            receiver_event,
            |e| Envelope::Receiver.unwrap_into_beats(e, Some(Framing::Line), "syslog"),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("receiver_body", |b| {
        b.iter_batched_ref(
            receiver_event,
            |e| Envelope::Receiver.unwrap_into_beats(e, Some(Framing::Body), "syslog"),
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

/// A hand-written arm per field: the shape a spec would have to beat.
fn renames_hand_written(event: &mut Event) {
    if let Some(v) = event.get("hostname").cloned() {
        let _ = event.set("log.syslog.hostname", v);
    }
    if let Some(v) = event.get("appname").cloned() {
        let _ = event.set("log.syslog.appname", v);
    }
    if let Some(v) = event.get("procid").cloned() {
        let _ = event.set("log.syslog.procid", v);
    }
    if let Some(v) = event.get("msgid").cloned() {
        let _ = event.set("log.syslog.msgid", v);
    }
    if let Some(v) = event.get("facility").cloned() {
        let _ = event.set("log.syslog.facility.name", v);
    }
    if let Some(v) = event.get("severity").cloned() {
        let _ = event.set("log.syslog.severity.name", v);
    }
    if let Some(v) = event.get("hostname").cloned() {
        let _ = event.set("host.hostname", v);
    }
    if let Some(v) = event.get("timestamp").cloned() {
        let _ = event.set("@timestamp", v);
    }
}

/// A list of borrowed literals, walked. What the library does now.
fn renames_static(event: &mut Event, renames: &[(&str, &str)]) {
    for (from, to) in renames {
        if let Some(value) = event.get(from).cloned() {
            let _ = event.set(to, value);
        }
    }
}

/// A list of owned strings, built once at startup from a declared spec.
fn renames_compiled(event: &mut Event, renames: &[(String, String)]) {
    for (from, to) in renames {
        if let Some(value) = event.get(from.as_str()).cloned() {
            let _ = event.set(to.as_str(), value);
        }
    }
}

/// The intermediate `Vec` the library used to build per event, kept so the
/// allocation it cost stays visible rather than being a claim.
fn renames_collected(event: &mut Event, renames: &[(&str, &str)]) {
    let pairs: Vec<(&str, Value)> = renames
        .iter()
        .filter_map(|(from, to)| event.get(from).cloned().map(|v| (*to, v)))
        .collect();
    for (path, value) in pairs {
        let _ = event.set(path, value);
    }
}

fn bench_renames(c: &mut Criterion) {
    const STATIC: &[(&str, &str)] = &[
        ("hostname", "log.syslog.hostname"),
        ("appname", "log.syslog.appname"),
        ("procid", "log.syslog.procid"),
        ("msgid", "log.syslog.msgid"),
        ("facility", "log.syslog.facility.name"),
        ("severity", "log.syslog.severity.name"),
        ("hostname", "host.hostname"),
        ("timestamp", "@timestamp"),
    ];
    let compiled: Vec<(String, String)> = STATIC
        .iter()
        .map(|(f, t)| ((*f).to_string(), (*t).to_string()))
        .collect();

    let mut group = c.benchmark_group("renames");
    group.bench_function("hand_written", |b| {
        b.iter_batched_ref(
            receiver_event,
            |e| renames_hand_written(black_box(e)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("static_list", |b| {
        b.iter_batched_ref(
            receiver_event,
            |e| renames_static(black_box(e), STATIC),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("compiled_spec", |b| {
        b.iter_batched_ref(
            receiver_event,
            |e| renames_compiled(black_box(e), &compiled),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("collected_vec", |b| {
        b.iter_batched_ref(
            receiver_event,
            |e| renames_collected(black_box(e), STATIC),
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

criterion_group!(benches, bench_detect, bench_unwrap, bench_renames);
criterion_main!(benches);
