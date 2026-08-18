// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! An LRU cache in front of the MMDB readers.
//!
//! Ported from dfe-loader's `GeoIpEnricher`, which owns IP enrichment for the
//! platform. The two repos keep independent copies while both are moving; the
//! shared parts are extracted once they settle (see docs/SHARED-CRATES.md).
//!
//! Worth having because the miss path walks an MMDB B-tree and log data
//! repeats addresses heavily: a firewall talks to the same handful of
//! neighbours for hours. 14 of the 60 source pipelines carry a geoip
//! processor, several of them four or more, so this sits under a 20k-event
//! batch several times per event.

use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

/// Entries kept before eviction starts.
const DEFAULT_CAPACITY: usize = 100_000;

/// Fraction of the cache dropped when it fills, as a divisor. Evicting a
/// quarter at a time amortises the sort over many inserts.
const EVICT_DIVISOR: usize = 4;

/// Which database a lookup went to. Part of the cache key, because the city
/// and ASN databases answer differently for the same address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Database {
    /// City, country and location.
    City,
    /// Autonomous system number and organisation.
    Asn,
}

/// A cached lookup and when it was stored.
struct Entry {
    fields: HashMap<String, Value>,
    stored_at: u64,
}

/// Hits, misses and current size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stats {
    /// Lookups answered from the cache.
    pub hits: u64,
    /// Lookups that reached a database.
    pub misses: u64,
    /// Entries currently held.
    pub size: usize,
}

/// An LRU cache of lookup results.
pub struct Cache {
    entries: RwLock<HashMap<(Database, String), Entry>>,
    capacity: usize,
    hits: AtomicU64,
    misses: AtomicU64,
}

impl Default for Cache {
    fn default() -> Self {
        Self::with_capacity(DEFAULT_CAPACITY)
    }
}

impl Cache {
    /// A cache holding at most `capacity` entries.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: RwLock::new(HashMap::with_capacity(1024)),
            capacity: capacity.max(1),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }

    /// The cached fields for `ip` in `database`, if any.
    ///
    /// A poisoned lock counts as a miss rather than propagating: a cache that
    /// panics is worse than one that stops caching.
    #[must_use]
    pub fn get(&self, database: Database, ip: &str) -> Option<HashMap<String, Value>> {
        let entries = self.entries.read().ok()?;
        let hit = entries.get(&(database, ip.to_string()));
        if hit.is_some() {
            self.hits.fetch_add(1, Ordering::Relaxed);
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
        }
        hit.map(|entry| entry.fields.clone())
    }

    /// Store `fields` for `ip`, evicting the oldest entries if full.
    ///
    /// An empty result is cached too: a miss in the database is as worth
    /// remembering as a hit, and private ranges are the common case.
    pub fn put(&self, database: Database, ip: &str, fields: HashMap<String, Value>) {
        let Ok(mut entries) = self.entries.write() else {
            return;
        };

        if entries.len() >= self.capacity {
            evict_oldest(&mut entries, self.capacity / EVICT_DIVISOR);
        }

        entries.insert(
            (database, ip.to_string()),
            Entry {
                fields,
                stored_at: now(),
            },
        );
    }

    /// Hits, misses and current size.
    #[must_use]
    pub fn stats(&self) -> Stats {
        let size = self.entries.read().map_or(0, |e| e.len());
        Stats {
            hits: self.hits.load(Ordering::Relaxed),
            misses: self.misses.load(Ordering::Relaxed),
            size,
        }
    }

    /// Drop every entry, keeping the counters.
    pub fn clear(&self) {
        if let Ok(mut entries) = self.entries.write() {
            entries.clear();
        }
    }
}

/// Remove the `count` oldest entries.
fn evict_oldest(entries: &mut HashMap<(Database, String), Entry>, count: usize) {
    if entries.is_empty() || count == 0 {
        return;
    }

    let mut by_age: Vec<((Database, String), u64)> = entries
        .iter()
        .map(|(key, entry)| (key.clone(), entry.stored_at))
        .collect();
    by_age.sort_unstable_by_key(|(_, stored_at)| *stored_at);

    for (key, _) in by_age.into_iter().take(count) {
        entries.remove(&key);
    }
}

/// Nanoseconds since the epoch, or 0 if the clock is before it.
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| u64::try_from(d.as_nanos()).unwrap_or(u64::MAX))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use serde_json::json;

    fn fields(country: &str) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("country_iso_code".to_string(), json!(country));
        map
    }

    #[test]
    fn a_miss_then_a_hit() {
        let cache = Cache::default();

        assert!(cache.get(Database::City, "8.8.8.8").is_none());
        cache.put(Database::City, "8.8.8.8", fields("US"));
        assert_eq!(cache.get(Database::City, "8.8.8.8"), Some(fields("US")));

        let stats = cache.stats();
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.size, 1);
    }

    /// The two databases answer differently for the same address, so the
    /// database has to be part of the key.
    #[test]
    fn city_and_asn_do_not_share_an_entry() {
        let cache = Cache::default();
        cache.put(Database::City, "8.8.8.8", fields("US"));

        assert!(cache.get(Database::Asn, "8.8.8.8").is_none());
        assert!(cache.get(Database::City, "8.8.8.8").is_some());
    }

    /// An address with no data is worth remembering: private ranges and
    /// unallocated space are the common case in log data.
    #[test]
    fn an_empty_result_is_cached() {
        let cache = Cache::default();
        cache.put(Database::City, "203.0.113.1", HashMap::new());

        assert_eq!(
            cache.get(Database::City, "203.0.113.1"),
            Some(HashMap::new())
        );
        assert_eq!(cache.stats().hits, 1);
    }

    #[test]
    fn eviction_keeps_the_cache_within_capacity() {
        let cache = Cache::with_capacity(8);

        for i in 0..40 {
            cache.put(Database::City, &format!("198.51.100.{i}"), fields("AU"));
        }

        assert!(
            cache.stats().size <= 8,
            "size {} exceeded capacity",
            cache.stats().size
        );
    }

    /// Eviction drops the oldest first, so a recently stored entry survives a
    /// round of it.
    #[test]
    fn eviction_takes_the_oldest_first() {
        let cache = Cache::with_capacity(4);

        cache.put(Database::City, "1.1.1.1", fields("AU"));
        cache.put(Database::City, "2.2.2.2", fields("AU"));
        cache.put(Database::City, "3.3.3.3", fields("AU"));
        // Fills it and evicts one, which must be 1.1.1.1.
        cache.put(Database::City, "4.4.4.4", fields("AU"));
        cache.put(Database::City, "5.5.5.5", fields("AU"));

        assert!(cache.get(Database::City, "5.5.5.5").is_some());
        assert!(cache.get(Database::City, "1.1.1.1").is_none());
    }

    #[test]
    fn capacity_is_never_zero() {
        let cache = Cache::with_capacity(0);
        cache.put(Database::City, "8.8.8.8", fields("US"));
        assert!(cache.stats().size <= 1);
    }

    #[test]
    fn clear_drops_entries_but_keeps_counters() {
        let cache = Cache::default();
        cache.put(Database::City, "8.8.8.8", fields("US"));
        let _ = cache.get(Database::City, "8.8.8.8");

        cache.clear();

        assert_eq!(cache.stats().size, 0);
        assert_eq!(cache.stats().hits, 1);
    }
}
