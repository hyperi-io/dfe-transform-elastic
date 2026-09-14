// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A bounded cache in front of the MMDB readers.
//!
//! Ported from dfe-loader's `GeoIpEnricher`, which owns IP enrichment for the
//! platform. The two repos keep independent copies while both are moving; the
//! shared parts are extracted once they settle (see docs/shared-crates.md).
//!
//! Worth having because the miss path walks an MMDB B-tree and log data
//! repeats addresses heavily: a firewall talks to the same handful of
//! neighbours for hours. 14 of the 60 source pipelines carry a geoip
//! processor, several of them four or more, so this sits under a 20k-event
//! batch several times per event.
//!
//! Eviction is oldest-STORED first, not least-recently-USED. True LRU means
//! taking the write lock on every hit to restamp the entry, which serialises
//! the read-mostly path this cache exists to keep parallel. With 100,000
//! entries against a hot set orders of magnitude smaller, the two evict the
//! same entries anyway.
//!
//! The key is a parsed [`IpAddr`], never the event string it came from: a
//! 17-byte key cannot be grown by whatever a vendor decided to put in
//! `source.ip`.
//!
//! A hit hands back an `Arc`, so it costs a refcount bump rather than a deep
//! clone of the nine-field city result. Cloning the map out was 769 ns a hit on
//! one thread and 5.3 us on eight, because the allocator ran nine times inside
//! the read lock; the enrichers only ever read the result, never mutate it.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use parking_lot::RwLock;
use serde_json::Value;

/// A lookup result, shared rather than copied.
pub type Fields = Arc<HashMap<String, Value>>;

/// Entries kept before eviction starts.
const DEFAULT_CAPACITY: usize = 100_000;

/// Fraction of the cache dropped when it fills, as a divisor. Evicting a
/// quarter at a time amortises the selection over many inserts.
const EVICT_DIVISOR: usize = 4;

/// The one empty result every miss hands back.
///
/// A private address has no data in any database and is the commonest input in
/// log data, so minting an `Arc` for each would put an allocation on the path
/// this cache exists to keep free.
#[must_use]
pub fn no_fields() -> Fields {
    static EMPTY: OnceLock<Fields> = OnceLock::new();
    Arc::clone(EMPTY.get_or_init(|| Arc::new(HashMap::new())))
}

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
    fields: Fields,
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

/// A bounded cache of lookup results.
pub struct Cache {
    entries: RwLock<HashMap<(Database, IpAddr), Entry>>,
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
    /// The read lock is held for a hash lookup and a refcount bump and nothing
    /// else, which is what keeps one transform thread per partition off each
    /// other's back.
    #[must_use]
    pub fn get(&self, database: Database, ip: IpAddr) -> Option<Fields> {
        let hit = self
            .entries
            .read()
            .get(&(database, ip))
            .map(|entry| Arc::clone(&entry.fields));
        if hit.is_some() {
            self.hits.fetch_add(1, Ordering::Relaxed);
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
        }
        hit
    }

    /// Store `fields` for `ip`, evicting the oldest entries if full.
    ///
    /// An empty result is cached too: a miss in the database is as worth
    /// remembering as a hit, and private ranges are the common case.
    pub fn put(&self, database: Database, ip: IpAddr, fields: Fields) {
        let mut entries = self.entries.write();

        if entries.len() >= self.capacity {
            // At least one, or a capacity under four makes the eviction a
            // no-op and the map grows without a bound.
            evict_oldest(&mut entries, (self.capacity / EVICT_DIVISOR).max(1));
        }

        entries.insert(
            (database, ip),
            Entry {
                fields,
                stored_at: now(),
            },
        );
    }

    /// Hits, misses and current size.
    #[must_use]
    pub fn stats(&self) -> Stats {
        let size = self.entries.read().len();
        Stats {
            hits: self.hits.load(Ordering::Relaxed),
            misses: self.misses.load(Ordering::Relaxed),
            size,
        }
    }

    /// Drop every entry, keeping the counters.
    pub fn clear(&self) {
        self.entries.write().clear();
    }
}

/// Remove the `count` oldest entries.
///
/// Partitioned rather than sorted: every transform thread is parked on the
/// write lock for the duration, and ordering the 75,000 entries that survive is
/// work the eviction never reads.
fn evict_oldest(entries: &mut HashMap<(Database, IpAddr), Entry>, count: usize) {
    let count = count.min(entries.len());
    if count == 0 {
        return;
    }

    let mut by_age: Vec<((Database, IpAddr), u64)> = entries
        .iter()
        .map(|(key, entry)| (*key, entry.stored_at))
        .collect();
    let (older, nth, _) = by_age.select_nth_unstable_by_key(count - 1, |(_, stored_at)| *stored_at);

    for (key, _) in older.iter().chain(std::iter::once(&*nth)) {
        entries.remove(key);
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

    fn fields(country: &str) -> Fields {
        let mut map = HashMap::new();
        map.insert("country_iso_code".to_string(), json!(country));
        Arc::new(map)
    }

    fn addr(ip: &str) -> IpAddr {
        ip.parse().expect("test address parses")
    }

    #[test]
    fn a_miss_then_a_hit() {
        let cache = Cache::default();

        assert!(cache.get(Database::City, addr("8.8.8.8")).is_none());
        cache.put(Database::City, addr("8.8.8.8"), fields("US"));
        assert_eq!(
            cache.get(Database::City, addr("8.8.8.8")),
            Some(fields("US"))
        );

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
        cache.put(Database::City, addr("8.8.8.8"), fields("US"));

        assert!(cache.get(Database::Asn, addr("8.8.8.8")).is_none());
        assert!(cache.get(Database::City, addr("8.8.8.8")).is_some());
    }

    /// IPv4 and IPv6 are distinct keys, and a v4-mapped v6 address is the one
    /// place that could collide by accident.
    #[test]
    fn address_families_do_not_collide() {
        let cache = Cache::default();
        cache.put(Database::City, addr("8.8.8.8"), fields("US"));

        assert!(
            cache
                .get(Database::City, addr("2001:4860:4860::8888"))
                .is_none()
        );
        assert!(cache.get(Database::City, addr("::ffff:8.8.8.8")).is_none());
    }

    /// An address with no data is worth remembering: private ranges and
    /// unallocated space are the common case in log data.
    #[test]
    fn an_empty_result_is_cached() {
        let cache = Cache::default();
        cache.put(Database::City, addr("203.0.113.1"), no_fields());

        assert_eq!(
            cache.get(Database::City, addr("203.0.113.1")),
            Some(no_fields())
        );
        assert_eq!(cache.stats().hits, 1);
    }

    /// A hit hands back a HANDLE, not a copy. A second hit that allocated a
    /// fresh map would be the deep clone this cache was measured to be paying.
    #[test]
    fn two_hits_share_one_result() {
        let cache = Cache::default();
        cache.put(Database::City, addr("8.8.8.8"), fields("US"));

        let first = cache.get(Database::City, addr("8.8.8.8")).expect("a hit");
        let second = cache.get(Database::City, addr("8.8.8.8")).expect("a hit");
        assert!(Arc::ptr_eq(&first, &second));
    }

    #[test]
    fn eviction_keeps_the_cache_within_capacity() {
        let cache = Cache::with_capacity(8);

        for i in 0..40 {
            cache.put(
                Database::City,
                addr(&format!("198.51.100.{i}")),
                fields("AU"),
            );
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

        cache.put(Database::City, addr("1.1.1.1"), fields("AU"));
        cache.put(Database::City, addr("2.2.2.2"), fields("AU"));
        cache.put(Database::City, addr("3.3.3.3"), fields("AU"));
        // Fills it and evicts one, which must be 1.1.1.1.
        cache.put(Database::City, addr("4.4.4.4"), fields("AU"));
        cache.put(Database::City, addr("5.5.5.5"), fields("AU"));

        assert!(cache.get(Database::City, addr("5.5.5.5")).is_some());
        assert!(cache.get(Database::City, addr("1.1.1.1")).is_none());
    }

    /// A capacity under the evict divisor still bounds the map.
    ///
    /// `capacity / EVICT_DIVISOR` is zero for anything below four, which made
    /// the eviction a no-op and let the cache grow without a ceiling. One entry
    /// could never see it: the map only exceeds its capacity on the SECOND
    /// insert.
    #[test]
    fn a_capacity_below_the_evict_divisor_still_bounds_the_map() {
        for capacity in 0..=4 {
            let cache = Cache::with_capacity(capacity);
            for i in 0..40 {
                cache.put(
                    Database::City,
                    addr(&format!("198.51.100.{i}")),
                    fields("AU"),
                );
            }
            let size = cache.stats().size;
            assert!(
                size <= capacity.max(1),
                "capacity {capacity} held {size} entries"
            );
        }
    }

    #[test]
    fn clear_drops_entries_but_keeps_counters() {
        let cache = Cache::default();
        cache.put(Database::City, addr("8.8.8.8"), fields("US"));
        let _ = cache.get(Database::City, addr("8.8.8.8"));

        cache.clear();

        assert_eq!(cache.stats().size, 0);
        assert_eq!(cache.stats().hits, 1);
    }
}
