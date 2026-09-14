// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! What a `date` processor costs per event, and where inside it the time goes.
//!
//! Every generated call site hands `parse_date` a LITERAL format list, so
//! everything the pattern alone determines is re-derived on every event:
//! `expand_optional` builds a `Vec` of readings and `java_to_chrono` builds a
//! translated `String`. This prices both against the whole parse, so a change
//! to either is judged on the number rather than on the argument.
//!
//! The batch group runs at the shipped `batch_size` of 20,000 -- a win at one
//! event that vanishes at 20,000 is a cache artefact.

use std::hint::black_box;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use dfe_core::date_formats::{java_to_chrono, parse_date};

/// The shipped Kafka batch, so the per-event number is measured where it runs.
const BATCH: usize = 20_000;

/// The commonest pattern in the vendored pipelines: one format, no optional
/// section, and a year in the text.
const ISO: [&str; 1] = ["yyyy-MM-dd'T'HH:mm:ss.SSSXXX"];

/// A format list the first entry does NOT match, which is the ordinary case
/// for a pipeline that names several -- the cost is paid once per candidate.
const LADDER: [&str; 3] = [
    "epoch_millis",
    "yyyy-MM-dd HH:mm:ss",
    "yyyy-MM-dd'T'HH:mm:ssXXX",
];

/// A pattern carrying optional sections, which `expand_optional` reads as four
/// candidate patterns rather than one.
const OPTIONAL: [&str; 1] = ["[EEE ]MMM [ ]d[ yyyy] HH:mm:ss"];

fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("date_formats");

    group.bench_function("iso_one_format", |b| {
        b.iter(|| parse_date(black_box("2024-04-03T21:02:19.168+00:00"), &ISO, None));
    });

    group.bench_function("ladder_third_format_matches", |b| {
        b.iter(|| parse_date(black_box("2024-04-03T21:02:19Z"), &LADDER, None));
    });

    group.bench_function("optional_sections", |b| {
        b.iter(|| parse_date(black_box("Mar  6 2022 20:52:12"), &OPTIONAL, None));
    });

    // A value no format reads: the whole ladder is walked and every candidate
    // pattern is built before the call declines.
    group.bench_function("no_format_matches", |b| {
        b.iter(|| parse_date(black_box("not a date at all"), &LADDER, None));
    });

    group.finish();
}

fn bench_translate(c: &mut Criterion) {
    let mut group = c.benchmark_group("date_formats_translate");

    group.bench_function("java_to_chrono", |b| {
        b.iter(|| java_to_chrono(black_box(ISO[0])));
    });

    group.finish();
}

fn bench_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("date_formats_batch");
    group.throughput(Throughput::Elements(BATCH as u64));

    group.bench_function("iso_20k", |b| {
        b.iter(|| {
            for _ in 0..BATCH {
                black_box(parse_date(
                    black_box("2024-04-03T21:02:19.168+00:00"),
                    &ISO,
                    None,
                ));
            }
        });
    });

    group.finish();
}

criterion_group!(benches, bench_parse, bench_translate, bench_batch);
criterion_main!(benches);
