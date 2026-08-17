// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Global GeoIP enricher for use by generated transform code.
//!
//! Initialised lazily on first lookup. Auto-detects MMDB files from:
//! 1. `GEOIP_CITY_DB` / `GEOIP_ASN_DB` env vars (explicit paths)
//! 2. `GEOIP_DB_DIR` env var (directory containing `*.mmdb` files)
//! 3. Default search paths: `testdata/geoip/`, `/var/lib/dfe/geoip/`
//!
//! Approach ported from dfe-loader's auto-works GeoIP pattern.

use std::collections::HashMap;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::OnceLock;

use serde_json::Value;
use tracing::{debug, info, warn};

use super::geoip::GeoIpEnrichment;

/// Global enrichers — one per database type.
struct GlobalGeoIp {
    city: Option<GeoIpEnrichment>,
    asn: Option<GeoIpEnrichment>,
}

static GLOBAL_GEOIP: OnceLock<GlobalGeoIp> = OnceLock::new();

/// Initialise the global GeoIP enricher.
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

    GlobalGeoIp { city, asn }
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

    // 3. Default search paths (includes workspace root for dev/test)
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

/// Perform a GeoIP lookup using the global enricher.
///
/// `db_name` selects the database: "geoip_city" or "geoip_asn".
/// Returns a flat map of field names to values, or an empty map if
/// the database is not loaded or the IP is private.
pub fn geoip_lookup(db_name: &str, ip: &str) -> HashMap<String, Value> {
    let global = GLOBAL_GEOIP.get_or_init(init_global);

    // Fast-path: private IPs don't have GeoIP data
    if is_private_ip(ip) {
        return HashMap::new();
    }

    let enricher = match db_name {
        "geoip_city" | "GeoLite2-City.mmdb" | "dbip-city-lite.mmdb" => global.city.as_ref(),
        "geoip_asn" | "GeoLite2-ASN.mmdb" | "dbip-asn-lite.mmdb" => global.asn.as_ref(),
        other => {
            // Try city as default
            if other.contains("ASN") || other.contains("asn") {
                global.asn.as_ref()
            } else {
                global.city.as_ref()
            }
        }
    };

    match enricher {
        Some(e) => match e.lookup(ip) {
            Ok(result) => result,
            Err(msg) => {
                debug!(ip = ip, error = %msg, "GeoIP lookup failed");
                HashMap::new()
            }
        },
        None => HashMap::new(),
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
        let result = geoip_lookup("geoip_city", "192.168.1.1");
        assert!(result.is_empty());
    }

    #[test]
    fn invalid_ip_returns_empty() {
        let result = geoip_lookup("geoip_city", "not-an-ip");
        assert!(result.is_empty());
    }

    #[test]
    fn public_ip_lookup() {
        // 8.8.8.8 (Google DNS) should have GeoIP data if DB is loaded
        let result = geoip_lookup("geoip_city", "8.8.8.8");
        eprintln!("GeoIP 8.8.8.8 result: {:?}", result);
        // Don't assert on specific values — DB may or may not be present
        // This test verifies the lookup path doesn't panic
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
