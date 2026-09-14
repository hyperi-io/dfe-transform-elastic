// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! What a grok call site costs, native path against the compiled regex.
//!
//! Lives here rather than in `dfe-transforms` because it needs only
//! `grok_cache` and `dfe-parse`. Benching it there meant an `opt-level=3`,
//! `codegen-units=1` build of a crate that holds one module per data stream,
//! which takes long enough that the measurement stopped being taken.
//!
//! Run with: `cargo bench -p dfe-runtime --bench grok`

// A bench that cannot build its own fixture has nothing to measure, so it
// should stop rather than report a number for the wrong thing.
#![allow(clippy::expect_used)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};

/// A typical syslog body: one line, no newline in it.
const LINE: &str = "Feb 11 13:12:45 fw01 kernel: a fairly typical syslog body with some detail";

/// A value whose first line is a small part of it, which is where the two
/// paths could most easily diverge in cost.
const MULTI: &str = "first line of a stack trace\n  at frame one\n  at frame two\n";

const ADDRESS: &str = "192.168.1.100";
const PAIR: &str = "10.0.0.7:443";

/// Does `dfe-parse` actually beat a COMPILED regex?
///
/// The design rests on "grok/regex is the #1 hot-path bottleneck, replace it
/// with native parsers". That was written when every regex was rebuilt per
/// event. Now that they are compiled once, the premise deserves a measurement
/// rather than an assumption.
fn native_vs_regex(c: &mut Criterion) {
    let mut group = c.benchmark_group("native_vs_regex");

    group.bench_function("ipv4/regex", |b| {
        let compiled = dfe_runtime::grok_cache::grok("^%{IPV4:source.ip}$");
        // The fast engine specifically: both of these shapes compile on it, and
        // measuring a dispatch would not answer the question this bench asks.
        let re = compiled
            .regex
            .fast()
            .expect("an address needs no lookaround");
        b.iter(|| black_box(re.captures(black_box(ADDRESS))));
    });

    group.bench_function("ipv4/native", |b| {
        b.iter(|| black_box(dfe_parse::ip::parse_ipv4(black_box(ADDRESS))));
    });

    group.bench_function("ip_port/regex", |b| {
        let compiled = dfe_runtime::grok_cache::grok("^%{IPV4:_temp.src_ip}:%{PORT:sport}$");
        let re = compiled.regex.fast().expect("a pair needs no lookaround");
        b.iter(|| black_box(re.captures(black_box(PAIR))));
    });

    group.bench_function("ip_port/native", |b| {
        b.iter(|| {
            // The composite the grok expresses: address, literal colon, port.
            let parsed = dfe_parse::ip::parse_ipv4(black_box(PAIR)).and_then(|(rest, ip)| {
                let rest = rest.strip_prefix(':').unwrap_or(rest);
                dfe_parse::numeric::parse_port(rest).map(|(tail, port)| (tail, ip, port))
            });
            black_box(parsed)
        });
    });

    group.finish();
}

/// The widest form in the tree: 194 call sites over 35 patterns spell a lone
/// `%{GREEDYDATA:field}`, which expands to `(?m)^.*$` and captures the first
/// line.
fn first_line(c: &mut Criterion) {
    let mut group = c.benchmark_group("grok_first_line");

    let compiled = dfe_runtime::grok_cache::grok("^%{GREEDYDATA:message}$");
    let re = compiled
        .regex
        .fast()
        .expect("a catch-all needs no lookaround");

    group.bench_function("single_line/regex", |b| {
        b.iter(|| black_box(re.captures(black_box(LINE))));
    });

    group.bench_function("single_line/native", |b| {
        b.iter(|| {
            let input = black_box(LINE);
            black_box(input.split_once('\n').map_or(input, |(head, _)| head))
        });
    });

    group.bench_function("multi_line/regex", |b| {
        b.iter(|| black_box(re.captures(black_box(MULTI))));
    });

    group.bench_function("multi_line/native", |b| {
        b.iter(|| {
            let input = black_box(MULTI);
            black_box(input.split_once('\n').map_or(input, |(head, _)| head))
        });
    });

    // End to end through the dispatch a call site actually takes, so the
    // figure includes the `Event::set` both paths pay.
    group.bench_function("single_line/extract_into", |b| {
        b.iter(|| {
            let mut event = dfe_runtime::Event::new(serde_json::json!({}));
            let _ = compiled.extract_into(black_box(LINE), &mut event);
            black_box(event)
        });
    });

    group.finish();
}

/// What the word boundaries in `%{WORD}` cost.
///
/// `WORD` is spelled at 2,194 call sites, and a line the pattern does NOT
/// match is the common case there -- a grok list tries each alternative in
/// turn, so most attempts fail. Both are measured, because the boundaries are
/// an assertion the engine checks at every candidate start.
fn word_boundaries(c: &mut Criterion) {
    let mut group = c.benchmark_group("grok_word");

    let compiled = dfe_runtime::grok_cache::grok(
        "^%{WORD:action} %{WORD:outcome} for %{WORD:user} on %{WORD:host}$",
    );
    let re = compiled.regex.fast().expect("a word needs no lookaround");

    group.bench_function("matching", |b| {
        b.iter(|| black_box(re.captures(black_box("login success for alice on fw01"))));
    });

    group.bench_function("not_matching", |b| {
        b.iter(|| {
            black_box(re.captures(black_box(
                "Feb 11 13:12:45 fw01 kernel: nothing here looks like that pattern at all",
            )))
        });
    });

    group.finish();
}

criterion_group!(benches, native_vs_regex, first_line, word_boundaries);
criterion_main!(benches);
