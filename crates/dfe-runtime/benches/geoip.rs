// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! What a `GeoIP` cache hit costs, on one thread and on eight.
//!
//! The service runs one transform thread per Kafka partition, so a shared
//! reader-count atomic that is free on one core is not free on eight. The
//! eight-thread group is the one that decides: `grok_cache` measured the same
//! pattern at 29 ns on one thread and 1,522 ns on eight.
//!
//! Sized at the shipped `batch_size` of 20,000, because a win at 100 lookups
//! can be a cache artefact.
//!
//! Run with: `cargo bench -p dfe-runtime --bench geoip`

// A bench that cannot build its own fixture has nothing to measure, so it
// should stop rather than report a number for the wrong thing.
#![allow(clippy::expect_used)]

use std::net::IpAddr;
use std::sync::Arc;

use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use dfe_runtime::enrichment::geoip_cache::{Cache, Database, Fields};
use serde_json::json;

/// Events in one batch, matching the shipped `source.batch_size`.
const BATCH: usize = 20_000;

/// Transform threads in the parallel group -- one per Kafka partition.
const THREADS: usize = 8;

/// Distinct addresses in the hot set. Log data repeats addresses heavily: a
/// firewall talks to the same handful of neighbours for hours.
const HOT_SET: usize = 256;

/// A city result of the width the enricher actually returns: nine fields, one
/// of them a nested location object.
fn city_fields() -> Fields {
    let mut map = std::collections::HashMap::new();
    map.insert("city_name".to_string(), json!("Sydney"));
    map.insert("region_iso_code".to_string(), json!("AU-NSW"));
    map.insert("region_name".to_string(), json!("New South Wales"));
    map.insert("timezone".to_string(), json!("Australia/Sydney"));
    map.insert(
        "location".to_string(),
        json!({ "lat": -33.8688, "lon": 151.2093 }),
    );
    map.insert("country_iso_code".to_string(), json!("AU"));
    map.insert("country_name".to_string(), json!("Australia"));
    map.insert("continent_name".to_string(), json!("Oceania"));
    map.insert("network".to_string(), json!("1.128.0.0/11"));
    Arc::new(map)
}

/// The hot set, as parsed addresses.
fn hot_set() -> Vec<IpAddr> {
    (0..HOT_SET)
        .map(|i| {
            #[allow(clippy::cast_possible_truncation)]
            let addr = format!("81.2.{}.{}", (i / 256) as u8, (i % 256) as u8);
            addr.parse().expect("a generated address parses")
        })
        .collect()
}

/// A cache already holding the hot set, so every lookup is a HIT.
fn warmed() -> (Arc<Cache>, Vec<IpAddr>) {
    let cache = Arc::new(Cache::default());
    let addresses = hot_set();
    for address in &addresses {
        cache.put(Database::City, *address, city_fields());
    }
    (cache, addresses)
}

/// One transform thread, reading its own partition's batch.
fn hits(c: &mut Criterion) {
    let (cache, addresses) = warmed();

    let mut group = c.benchmark_group("geoip_cache_hit");
    group.throughput(Throughput::Elements(BATCH as u64));
    group.bench_function("one_thread/20k", |b| {
        b.iter(|| {
            for i in 0..BATCH {
                let address = addresses[i % HOT_SET];
                black_box(cache.get(black_box(Database::City), black_box(address)));
            }
        });
    });
    group.finish();
}

/// Eight transform threads against the one process-global cache, which is how
/// the service actually runs. A shared reader-count atomic shows up here and
/// nowhere else.
fn hits_parallel(c: &mut Criterion) {
    let (cache, addresses) = warmed();
    let addresses = Arc::new(addresses);
    let per_thread = BATCH / THREADS;

    let mut group = c.benchmark_group("geoip_cache_hit");
    group.throughput(Throughput::Elements(BATCH as u64));
    group.bench_function("eight_threads/20k", |b| {
        b.iter(|| {
            std::thread::scope(|scope| {
                for t in 0..THREADS {
                    let cache = Arc::clone(&cache);
                    let addresses = Arc::clone(&addresses);
                    scope.spawn(move || {
                        for i in 0..per_thread {
                            let address = addresses[(t * per_thread + i) % HOT_SET];
                            black_box(cache.get(black_box(Database::City), black_box(address)));
                        }
                    });
                }
            });
        });
    });
    group.finish();
}

criterion_group!(benches, hits, hits_parallel);
criterion_main!(benches);
