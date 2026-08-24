// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! What the map's HASHER costs, against the lookup it is part of.
//!
//! `benches/event.rs` puts one root-map lookup at ~17 ns and a three-segment
//! path at ~72 ns, which is ~24 ns a segment. A lookup that has to hash a
//! seven-byte key and then compare it should not cost that, so this splits the
//! two apart: the same map, the same keys, the std hasher against a fast one.
//!
//! The map is an `IndexMap` because that is what `serde_json` uses under
//! `preserve_order`, which is not optional here -- Elasticsearch's own maps are
//! insertion-ordered and parity depends on matching that.
//!
//! This is a MEASUREMENT, not a proposal. `serde_json::Map` does not let a
//! caller choose the hasher, so a win here says how much is on the table, not
//! how to collect it.

use std::hash::{BuildHasher, BuildHasherDefault, Hasher};
use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use indexmap::IndexMap;

/// rustc's own `FxHasher`, which is the reference point for "as fast as
/// hashing a short key can reasonably be".
///
/// Not cryptographic and not collision-resistant against chosen input. Field
/// names in an ingest pipeline are attacker-influenced, so this is here to
/// price the alternative, not to recommend it unexamined.
#[derive(Default)]
struct FxHasher {
    hash: u64,
}

impl FxHasher {
    const SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;

    fn add(&mut self, word: u64) {
        self.hash = (self.hash.rotate_left(5) ^ word).wrapping_mul(Self::SEED);
    }
}

impl Hasher for FxHasher {
    fn write(&mut self, bytes: &[u8]) {
        let (words, tail) = bytes.as_chunks::<8>();
        for word in words {
            self.add(u64::from_ne_bytes(*word));
        }
        if !tail.is_empty() {
            let mut buffer = [0u8; 8];
            buffer[..tail.len()].copy_from_slice(tail);
            self.add(u64::from_ne_bytes(buffer));
        }
    }

    fn finish(&self) -> u64 {
        self.hash
    }
}

/// The root keys of a realistic event, in the order a document carries them.
const KEYS: [&str; 12] = [
    "@timestamp",
    "message",
    "event",
    "source",
    "destination",
    "user",
    "host",
    "network",
    "url",
    "tags",
    "ecs",
    "agent",
];

fn build<S: BuildHasher + Default>() -> IndexMap<String, u32, S> {
    let mut map = IndexMap::default();
    for (index, key) in KEYS.iter().enumerate() {
        map.insert((*key).to_string(), u32::try_from(index).unwrap_or_default());
    }
    map
}

/// Hashing alone, with no map around it.
fn hasher_only(c: &mut Criterion) {
    let std_hasher = std::hash::RandomState::new();
    let fx = BuildHasherDefault::<FxHasher>::default();

    let mut group = c.benchmark_group("hash_key");
    group.bench_function("siphash", |b| {
        b.iter(|| black_box(std_hasher.hash_one(black_box("destination"))));
    });
    group.bench_function("fxhash", |b| {
        b.iter(|| black_box(fx.hash_one(black_box("destination"))));
    });
    group.finish();
}

/// The lookup, both ways, hit and miss.
fn map_lookup(c: &mut Criterion) {
    let std_map: IndexMap<String, u32> = build();
    let fx_map: IndexMap<String, u32, BuildHasherDefault<FxHasher>> = build();

    let mut group = c.benchmark_group("indexmap_get");
    group.bench_function("siphash/hit", |b| {
        b.iter(|| black_box(std_map.get(black_box("destination"))));
    });
    group.bench_function("fxhash/hit", |b| {
        b.iter(|| black_box(fx_map.get(black_box("destination"))));
    });
    group.bench_function("siphash/miss", |b| {
        b.iter(|| black_box(std_map.get(black_box("threat"))));
    });
    group.bench_function("fxhash/miss", |b| {
        b.iter(|| black_box(fx_map.get(black_box("threat"))));
    });
    group.finish();
}

/// A three-segment path is three lookups, which is what a transform actually
/// pays. Measuring the walk end to end keeps the per-segment arithmetic
/// honest.
fn three_segments(c: &mut Criterion) {
    let std_map: IndexMap<String, u32> = build();
    let fx_map: IndexMap<String, u32, BuildHasherDefault<FxHasher>> = build();

    let mut group = c.benchmark_group("indexmap_three_gets");
    group.bench_function("siphash", |b| {
        b.iter(|| {
            black_box((
                std_map.get(black_box("source")),
                std_map.get(black_box("event")),
                std_map.get(black_box("url")),
            ))
        });
    });
    group.bench_function("fxhash", |b| {
        b.iter(|| {
            black_box((
                fx_map.get(black_box("source")),
                fx_map.get(black_box("event")),
                fx_map.get(black_box("url")),
            ))
        });
    });
    group.finish();
}

criterion_group!(benches, hasher_only, map_lookup, three_segments);
criterion_main!(benches);
