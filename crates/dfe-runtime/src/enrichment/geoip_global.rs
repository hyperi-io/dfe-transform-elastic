// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Global `GeoIP` enricher for use by generated transform code.
//!
//! 14 of the 60 source pipelines carry a geoip processor, so this is on the
//! path for every network-device source. An [LRU cache](super::geoip_cache)
//! fronts the MMDB readers.
//!
//! Databases are resolved once, lazily, from:
//! 1. `GEOIP_CITY_DB` / `GEOIP_ASN_DB` (explicit paths)
//! 2. `GEOIP_DB_DIR` (a directory of `*.mmdb`)
//! 3. `/var/lib/dfe/geoip`, which is where dfe-loader's downloader puts them
//!
//! Finding none is not an error: lookups return empty and the transform
//! carries on. [`enabled`] says which happened, so a deployment that expects
//! enrichment can tell it is not getting any.
//!
//! The ECS field shaping is this crate's own: dfe-loader flattens to `geo_*`
//! for `ClickHouse` columns, whereas the Elastic pipelines nest under
//! `source.geo`, `destination.geo`, `client.geo` and `server.geo`, up to four
//! per event.

use std::collections::HashMap;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::OnceLock;

use serde_json::Value;
use tracing::{debug, info, warn};

use super::geoip::GeoIpEnrichment;
use super::geoip_cache::{Cache, Database, Stats};

/// Global enrichers — one per database type.
struct GlobalGeoIp {
    city: Option<GeoIpEnrichment>,
    asn: Option<GeoIpEnrichment>,
    cache: Cache,
}

static GLOBAL_GEOIP: OnceLock<GlobalGeoIp> = OnceLock::new();

/// Whether any database loaded.
///
/// Forces initialisation. A service that expects enrichment can check this at
/// startup rather than discovering empty geo fields in production.
#[must_use]
pub fn enabled() -> bool {
    let global = GLOBAL_GEOIP.get_or_init(init_global);
    global.city.is_some() || global.asn.is_some()
}

/// Cache hits, misses and size.
#[must_use]
pub fn cache_stats() -> Stats {
    GLOBAL_GEOIP.get_or_init(init_global).cache.stats()
}

/// Initialise the global `GeoIP` enricher.
///
/// Called lazily on first `geoip_lookup()`. Searches for MMDB files
/// in standard locations. Non-fatal: if no databases found, lookups
/// return empty results instead of errors.
fn init_global() -> GlobalGeoIp {
    let city_path = find_db(
        "GEOIP_CITY_DB",
        &["dbip-city-lite.mmdb", "GeoLite2-City.mmdb"],
    );
    let asn_path = find_db("GEOIP_ASN_DB", &["dbip-asn-lite.mmdb", "GeoLite2-ASN.mmdb"]);

    let city = city_path.and_then(|p| {
        info!(path = %p.display(), "loading GeoIP City database");
        match GeoIpEnrichment::open(&p) {
            Ok(e) => Some(e),
            Err(msg) => {
                warn!(path = %p.display(), error = %msg, "failed to load City DB");
                None
            }
        }
    });

    let asn = asn_path.and_then(|p| {
        info!(path = %p.display(), "loading GeoIP ASN database");
        match GeoIpEnrichment::open(&p) {
            Ok(e) => Some(e),
            Err(msg) => {
                warn!(path = %p.display(), error = %msg, "failed to load ASN DB");
                None
            }
        }
    });

    if city.is_none() && asn.is_none() {
        debug!("no GeoIP databases found — enrichment disabled");
    }

    GlobalGeoIp {
        city,
        asn,
        cache: Cache::default(),
    }
}

/// Find a database file by env var or by searching standard directories.
fn find_db(env_var: &str, filenames: &[&str]) -> Option<PathBuf> {
    // 1. Explicit env var
    if let Ok(path) = std::env::var(env_var) {
        let p = PathBuf::from(path);
        if p.exists() {
            return Some(p);
        }
    }

    // 2. GEOIP_DB_DIR env var
    if let Ok(dir) = std::env::var("GEOIP_DB_DIR") {
        for name in filenames {
            let p = PathBuf::from(&dir).join(name);
            if p.exists() {
                return Some(p);
            }
        }
    }

    // 3. Default search paths. `/var/lib/dfe/geoip` is where dfe-loader's
    //    downloader puts them, so a shared volume needs no configuration.
    let workspace_testdata = concat!(env!("CARGO_MANIFEST_DIR"), "/../../testdata/geoip");
    let search_dirs = [
        "testdata/geoip",
        workspace_testdata,
        "/var/lib/dfe/geoip",
        "/usr/share/GeoIP",
    ];

    for dir in &search_dirs {
        for name in filenames {
            let p = PathBuf::from(dir).join(name);
            if p.exists() {
                return Some(p);
            }
        }
    }

    None
}

/// Perform a `GeoIP` lookup using the global enricher.
///
/// `db_name` selects the database: "`geoip_city`" or "`geoip_asn`".
/// Returns a flat map of field names to values, or an empty map if
/// the database is not loaded or the IP is private.
pub fn geoip_lookup(db_name: &str, ip: &str) -> HashMap<String, Value> {
    let global = GLOBAL_GEOIP.get_or_init(init_global);

    // Private ranges have no data anywhere, so they never reach the cache or
    // a database.
    if is_private_ip(ip) {
        return HashMap::new();
    }

    let database = database_for(db_name);
    if let Some(cached) = global.cache.get(database, ip) {
        return cached;
    }

    let enricher = match database {
        Database::City => global.city.as_ref(),
        Database::Asn => global.asn.as_ref(),
    };

    let fields = match enricher {
        Some(e) => match e.lookup(ip) {
            Ok(result) => result,
            Err(msg) => {
                debug!(ip = ip, error = %msg, "GeoIP lookup failed");
                HashMap::new()
            }
        },
        None => HashMap::new(),
    };

    global.cache.put(database, ip, fields.clone());
    fields
}

/// Which database a processor's `database_file` names.
///
/// The pipelines spell it several ways, and anything unrecognised is city:
/// that is what Elastic's own default is.
fn database_for(db_name: &str) -> Database {
    match db_name {
        "geoip_asn" | "GeoLite2-ASN.mmdb" | "dbip-asn-lite.mmdb" => Database::Asn,
        other if other.contains("ASN") || other.contains("asn") => Database::Asn,
        _ => Database::City,
    }
}

/// Check whether an IP is in a private/internal range.
fn is_private_ip(ip: &str) -> bool {
    match ip.parse::<IpAddr>() {
        Ok(IpAddr::V4(v4)) => {
            v4.is_private()
                || v4.is_loopback()
                || v4.is_link_local()
                || v4.is_broadcast()
                || v4.is_documentation()
                // CGNAT range 100.64.0.0/10
                || (v4.octets()[0] == 100 && (v4.octets()[1] & 0xC0) == 64)
        }
        Ok(IpAddr::V6(v6)) => v6.is_loopback() || v6.is_multicast(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_ip_detection() {
        assert!(is_private_ip("10.0.0.1"));
        assert!(is_private_ip("172.16.0.1"));
        assert!(is_private_ip("192.168.1.1"));
        assert!(is_private_ip("127.0.0.1"));
        assert!(is_private_ip("169.254.0.1"));
        assert!(is_private_ip("100.64.0.1"));
        assert!(!is_private_ip("8.8.8.8"));
        assert!(!is_private_ip("175.16.199.1"));
    }

    #[test]
    fn private_ip_returns_empty() {
        let _guard = serialised();
        let result = geoip_lookup("geoip_city", "192.168.1.1");
        assert!(result.is_empty());
    }

    #[test]
    fn invalid_ip_returns_empty() {
        let _guard = serialised();
        let result = geoip_lookup("geoip_city", "not-an-ip");
        assert!(result.is_empty());
    }

    #[test]
    fn public_ip_lookup() {
        let _guard = serialised();
        // 8.8.8.8 (Google DNS) should have GeoIP data if DB is loaded
        let result = geoip_lookup("geoip_city", "8.8.8.8");
        eprintln!("GeoIP 8.8.8.8 result: {result:?}");
        // Don't assert on specific values — DB may or may not be present
        // This test verifies the lookup path doesn't panic
    }

    /// The cache and its counters are process-global, so the tests that read
    /// them take one lock rather than racing each other.
    fn serialised() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        LOCK.lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// A repeated address must not reach the database twice. Asserted on the
    /// delta, because other tests have already moved the counters.
    #[test]
    fn a_repeated_lookup_is_served_from_the_cache() {
        let _guard = serialised();
        // Globally routable and used by no other test. The documentation
        // ranges are not an option: Rust classes them as private, so they
        // never reach the cache.
        let ip = "93.184.216.34";

        let first = geoip_lookup("geoip_city", ip);
        let after_first = cache_stats();
        let second = geoip_lookup("geoip_city", ip);
        let after_second = cache_stats();

        assert_eq!(first, second, "the cache must return the same fields");
        assert_eq!(
            after_second.hits - after_first.hits,
            1,
            "the second lookup must be served from the cache"
        );
        assert_eq!(
            after_second.misses, after_first.misses,
            "the second lookup must not reach a database"
        );
    }

    /// The city and ASN databases answer differently, so a city hit must not
    /// satisfy an ASN lookup.
    #[test]
    fn city_and_asn_lookups_are_cached_separately() {
        let _guard = serialised();
        let ip = "208.67.222.222";
        let _ = geoip_lookup("geoip_city", ip);

        let before = cache_stats();
        let _ = geoip_lookup("geoip_asn", ip);
        assert_eq!(
            cache_stats().misses - before.misses,
            1,
            "the ASN lookup must not be served by the city entry"
        );
    }

    #[test]
    fn database_names_map_to_the_right_database() {
        assert_eq!(database_for("geoip_asn"), Database::Asn);
        assert_eq!(database_for("GeoLite2-ASN.mmdb"), Database::Asn);
        assert_eq!(database_for("dbip-asn-lite.mmdb"), Database::Asn);
        assert_eq!(database_for("geoip_city"), Database::City);
        assert_eq!(database_for("GeoLite2-City.mmdb"), Database::City);
        // Elastic's own default is the city database.
        assert_eq!(database_for("anything-else"), Database::City);
    }

    /// A private address must not reach the cache at all -- it has no data in
    /// any database, and caching it would fill the map with RFC 1918 space.
    #[test]
    fn private_addresses_never_reach_the_cache() {
        let _guard = serialised();
        let before = cache_stats();
        let _ = geoip_lookup("geoip_city", "10.11.12.13");
        let after = cache_stats();

        assert_eq!(after.hits, before.hits);
        assert_eq!(after.misses, before.misses);
    }

    #[test]
    fn db_auto_detection() {
        // Force init and check if databases were found
        let global = GLOBAL_GEOIP.get_or_init(init_global);
        eprintln!(
            "City DB loaded: {}, ASN DB loaded: {}",
            global.city.is_some(),
            global.asn.is_some()
        );
    }
}
