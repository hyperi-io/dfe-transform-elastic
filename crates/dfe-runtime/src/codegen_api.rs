// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! API functions called by generated transform code.
//!
//! The codegen emits calls to these free functions. They wrap the
//! enrichment modules or provide no-op stubs for features that
//! require external configuration (GeoIP databases, Painless VM).

use std::collections::HashMap;

use serde_json::Value;
use tracing::debug;

use crate::enrichment::user_agent;
use crate::error::{Result, TransformError};
use crate::event::Event;

/// Result from a registered domain lookup.
pub struct RegisteredDomainResult {
    pub registered_domain: String,
    pub top_level_domain: String,
    pub subdomain: Option<String>,
}

/// Look up GeoIP data for an IP address.
///
/// Returns a flat map of field names to values (e.g., "country_iso_code" -> "AU").
/// Uses the global auto-initialised enricher (auto-detects MMDB files).
pub fn geoip_lookup(db_name: &str, ip: &str) -> Result<HashMap<String, Value>> {
    Ok(crate::enrichment::geoip_global::geoip_lookup(db_name, ip))
}

/// Parse a User-Agent string into structured components.
pub fn parse_user_agent(ua: &str) -> Result<user_agent::UserAgentResult> {
    Ok(user_agent::parse(ua))
}

/// Compute a Community ID v1 hash for network flow identification.
pub fn community_id_v1(
    src_ip: &str,
    dst_ip: &str,
    src_port: u16,
    dst_port: u16,
    protocol: &str,
) -> std::result::Result<String, String> {
    crate::enrichment::community_id::community_id_v1(
        src_ip, dst_ip, src_port, dst_port, protocol, 0,
    )
}

/// Look up the registered domain from a full domain name.
///
/// Extracts the registered domain and TLD using simple heuristic
/// (splits on dots — full PSL lookup deferred to Phase 5).
pub fn registered_domain_lookup(domain: &str) -> Option<RegisteredDomainResult> {
    let parts: Vec<&str> = domain.rsplitn(3, '.').collect();
    if parts.len() < 2 {
        return None;
    }

    let tld = parts[0].to_string();
    let sld = parts[1];
    let registered_domain = format!("{sld}.{tld}");

    let subdomain = if parts.len() == 3 && !parts[2].is_empty() {
        Some(parts[2].to_string())
    } else {
        None
    };

    Some(RegisteredDomainResult {
        registered_domain,
        top_level_domain: tld,
        subdomain,
    })
}

/// Execute a Painless script against an event.
///
/// Tries known common patterns first (drop nulls, command line extraction,
/// keys_to_snake_case, etc.). Falls back to a no-op for unrecognised scripts.
pub fn painless_exec(event: &mut Event, script: &str) -> Result<()> {
    if crate::painless_common::try_known_painless(event, script) {
        return Ok(());
    }
    debug!(
        script_len = script.len(),
        "painless_exec: unrecognised script skipped"
    );
    Ok(())
}

/// Convert a grok pattern string to a regex pattern string.
///
/// Phase 3 will replace grok patterns with native parsers.
/// This stub expands `%{NAME:field}` to named capture groups `(?P<field>.+)`,
/// and `%{NAME}` to `(.+)`. This is a rough approximation — real grok
/// patterns have type-specific sub-patterns that will be handled by
/// dfe-parse native parsers.
pub fn grok_to_regex(pattern: &str) -> String {
    let mut result = String::with_capacity(pattern.len());
    let mut chars = pattern.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' && chars.peek() == Some(&'{') {
            chars.next(); // consume '{'
            let mut name = String::new();
            let mut field = String::new();
            let mut in_field = false;

            for ch in chars.by_ref() {
                if ch == '}' {
                    break;
                } else if ch == ':' && !in_field {
                    in_field = true;
                } else if in_field {
                    field.push(ch);
                } else {
                    name.push(ch);
                }
            }

            if !field.is_empty() {
                // Named capture: %{PATTERN:field} -> (?P<field>.+)
                let safe_field = field.replace('.', "_");
                result.push_str(&format!("(?P<{safe_field}>.+?)"));
            } else {
                // Unnamed: %{PATTERN} -> (.+)
                result.push_str("(.+?)");
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// Check whether an IP address is in a private/internal range.
///
/// Recognises RFC 1918 (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16),
/// loopback (127.0.0.0/8), and link-local (169.254.0.0/16).
pub fn is_internal_ip(ip: &str) -> bool {
    use std::net::IpAddr;
    match ip.parse::<IpAddr>() {
        Ok(IpAddr::V4(v4)) => v4.is_private() || v4.is_loopback() || v4.is_link_local(),
        Ok(IpAddr::V6(v6)) => v6.is_loopback(),
        Err(_) => false,
    }
}
