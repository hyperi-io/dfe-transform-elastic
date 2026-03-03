// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Benchmarks comparing dfe-parse against regex equivalents.

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_parsers(_c: &mut Criterion) {
    // TODO: Add parser benchmarks
}

criterion_group!(benches, bench_parsers);
criterion_main!(benches);
