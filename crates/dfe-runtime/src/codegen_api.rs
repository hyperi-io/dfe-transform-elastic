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
use crate::error::Result;
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
/// Expands `%{NAME:field}` to named capture groups with type-appropriate
/// sub-patterns. Returns `(regex_string, field_map)` where field_map maps
/// safe capture names back to original dotted field paths.
///
/// Phase 3 will replace grok with native dfe-parse parsers.
pub fn grok_to_regex(pattern: &str) -> String {
    grok_to_regex_with_map(pattern).0
}

/// Like `grok_to_regex` but also returns a map of capture_name → original_field_path.
///
/// This is needed because regex capture names can't contain dots, so
/// `user.name` becomes `user_name` in the regex. The map lets callers
/// restore the original dotted path when setting fields.
pub fn grok_to_regex_with_map(
    pattern: &str,
) -> (String, std::collections::HashMap<String, String>) {
    let mut result = String::with_capacity(pattern.len());
    let mut field_map = std::collections::HashMap::new();
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

            let sub_pattern = grok_pattern_regex(&name);

            if !field.is_empty() {
                // Strip Elastic type suffix (e.g., "source.ip:ip" → "source.ip")
                let field_name = field.split(':').next().unwrap_or(&field);
                let safe_field = field_name.replace('.', "_");
                field_map.insert(safe_field.clone(), field_name.to_string());
                result.push_str(&format!("(?P<{safe_field}>{sub_pattern})"));
            } else {
                result.push_str(&format!("({sub_pattern})"));
            }
        } else {
            result.push(c);
        }
    }

    (result, field_map)
}

/// Map well-known grok pattern names to their regex equivalents.
fn grok_pattern_regex(name: &str) -> &'static str {
    match name {
        "USER" | "USERNAME" => r"[a-zA-Z0-9._-]+",
        "IP" | "IPV4" => r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}",
        "IPV6" => r"[0-9a-fA-F:]+",
        "POSINT" => r"\d+",
        "INT" => r"[+-]?\d+",
        "NUMBER" => r"[+-]?(?:\d+\.?\d*|\.\d+)",
        "NOTSPACE" => r"\S+",
        "GREEDYDATA" => r".*",
        "DATA" => r".*?",
        "WORD" => r"\w+",
        "HOSTNAME" => r"[a-zA-Z0-9._-]+",
        "MAC" => r"(?:[0-9a-fA-F]{2}[:-]){5}[0-9a-fA-F]{2}",
        "BASE10NUM" => r"[+-]?(?:\d+\.?\d*|\.\d+)",
        "EMAILADDRESS" => r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}",
        "URI" | "URIPROTO" => r"\S+",
        "PATH" | "UNIXPATH" | "WINPATH" => r"[^\s]+",
        // Azure custom patterns (from pipeline pattern_definitions)
        "SUBID" => {
            r"(?:\{)?[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}(?:\})?"
        }
        "GROUPID" | "PROVIDERNAME" | "NAMESPACE" | "RULE" | "NAME" => r"[^/]+",
        "MONTHDAY" => r"\d{1,2}",
        "MONTH" => r"\w+",
        "YEAR" => r"\d{4}",
        "HOUR" => r"\d{2}",
        "MINUTE" => r"\d{2}",
        "SECOND" => r"\d{2}",
        "TIMESTAMP_ISO8601" => {
            r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:?\d{2})?"
        }
        _ => ".+?", // fallback for unknown patterns
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // --- is_internal_ip ---

    #[test]
    fn internal_ip_rfc1918_class_a() {
        assert!(is_internal_ip("10.0.0.1"));
        assert!(is_internal_ip("10.255.255.255"));
    }

    #[test]
    fn internal_ip_rfc1918_class_b() {
        assert!(is_internal_ip("172.16.0.1"));
        assert!(is_internal_ip("172.31.255.255"));
        assert!(!is_internal_ip("172.32.0.1"));
    }

    #[test]
    fn internal_ip_rfc1918_class_c() {
        assert!(is_internal_ip("192.168.0.1"));
        assert!(is_internal_ip("192.168.255.255"));
    }

    #[test]
    fn internal_ip_loopback() {
        assert!(is_internal_ip("127.0.0.1"));
        assert!(is_internal_ip("::1"));
    }

    #[test]
    fn internal_ip_link_local() {
        assert!(is_internal_ip("169.254.0.1"));
    }

    #[test]
    fn internal_ip_public() {
        assert!(!is_internal_ip("8.8.8.8"));
        assert!(!is_internal_ip("175.16.199.1"));
        assert!(!is_internal_ip("1.1.1.1"));
    }

    #[test]
    fn internal_ip_invalid() {
        assert!(!is_internal_ip("not-an-ip"));
        assert!(!is_internal_ip(""));
    }

    // --- registered_domain_lookup ---

    #[test]
    fn registered_domain_simple() {
        let r = registered_domain_lookup("www.example.com").unwrap();
        assert_eq!(r.registered_domain, "example.com");
        assert_eq!(r.top_level_domain, "com");
        assert_eq!(r.subdomain.as_deref(), Some("www"));
    }

    #[test]
    fn registered_domain_no_subdomain() {
        let r = registered_domain_lookup("example.com").unwrap();
        assert_eq!(r.registered_domain, "example.com");
        assert_eq!(r.top_level_domain, "com");
        assert!(r.subdomain.is_none());
    }

    #[test]
    fn registered_domain_bare_tld() {
        assert!(registered_domain_lookup("com").is_none());
    }

    #[test]
    fn registered_domain_empty() {
        assert!(registered_domain_lookup("").is_none());
    }

    // --- grok_to_regex ---

    #[test]
    fn grok_simple_ip_field() {
        let regex = grok_to_regex("%{IP:source.ip}");
        assert!(regex.contains("(?P<source_ip>"));
    }

    #[test]
    fn grok_field_map_restores_dots() {
        let (_, map) = grok_to_regex_with_map("%{USER:user.name}");
        assert_eq!(map.get("user_name").unwrap(), "user.name");
    }

    #[test]
    fn grok_no_field() {
        let regex = grok_to_regex("%{NOTSPACE}");
        assert!(regex.contains(r"\S+"));
        assert!(!regex.contains("(?P<"));
    }

    #[test]
    fn grok_multiple_patterns() {
        let regex = grok_to_regex("%{IP:src}:%{POSINT:port}");
        assert!(regex.contains("(?P<src>"));
        assert!(regex.contains("(?P<port>"));
    }

    #[test]
    fn grok_unknown_pattern_fallback() {
        let regex = grok_to_regex("%{UNKNOWN_THING:field}");
        assert!(regex.contains(".+?")); // fallback
    }

    // --- parse_user_agent ---

    #[test]
    fn parse_ua_returns_ok() {
        let result = parse_user_agent("Mozilla/5.0 (Windows NT 10.0) Chrome/91.0");
        assert!(result.is_ok());
        let ua = result.unwrap();
        assert_eq!(ua.name.as_deref(), Some("Chrome"));
    }

    #[test]
    fn parse_ua_empty() {
        let result = parse_user_agent("");
        assert!(result.is_ok());
    }

    // --- painless_exec ---

    #[test]
    fn painless_exec_unknown_script_noop() {
        let mut event = Event::new(json!({"field": "value"}));
        let result = painless_exec(&mut event, "unknown_script_that_does_nothing();");
        assert!(result.is_ok());
        // Field should be unchanged
        assert_eq!(event.get_str("field"), Some("value"));
    }

    #[test]
    fn painless_exec_drop_empty_known() {
        let mut event = Event::new(json!({"a": "", "b": "keep", "c": null}));
        let result = painless_exec(
            &mut event,
            r#"boolean drop(Object o) { if (o == null || o == "") { return true; } }"#,
        );
        assert!(result.is_ok());
    }

    // --- community_id_v1 ---

    #[test]
    fn community_id_tcp() {
        let result = community_id_v1("1.2.3.4", "5.6.7.8", 1234, 80, "tcp");
        assert!(result.is_ok());
        let id = result.unwrap();
        assert!(id.starts_with("1:"));
    }

    #[test]
    fn community_id_invalid_ip() {
        let result = community_id_v1("not-an-ip", "5.6.7.8", 1234, 80, "tcp");
        assert!(result.is_err());
    }

    // --- geoip_lookup ---

    #[test]
    fn geoip_lookup_no_db() {
        // Without MMDB files loaded, should return empty map (not panic)
        let result = geoip_lookup("geoip_city", "8.8.8.8");
        assert!(result.is_ok());
    }

    // --- grok edge cases ---

    #[test]
    fn grok_type_suffix_stripped() {
        // Elastic grok uses %{IP:field:type} — the :type must be stripped
        let (regex, map) = grok_to_regex_with_map("%{IP:source.ip:ip}");
        assert!(regex.contains("(?P<source_ip>"));
        assert!(!regex.contains(":ip"));
        assert_eq!(map.get("source_ip").unwrap(), "source.ip");
    }

    #[test]
    fn grok_empty_pattern() {
        let regex = grok_to_regex("");
        assert_eq!(regex, "");
    }

    #[test]
    fn grok_literal_only() {
        let regex = grok_to_regex("hello world");
        assert_eq!(regex, "hello world");
    }

    #[test]
    fn grok_consecutive_patterns() {
        let regex = grok_to_regex("%{IP:src}%{POSINT:port}");
        assert!(regex.contains("(?P<src>"));
        assert!(regex.contains("(?P<port>"));
    }

    #[test]
    fn grok_long_type_suffix() {
        let (regex, map) = grok_to_regex_with_map("%{NUMBER:count:long}");
        assert!(regex.contains("(?P<count>"));
        assert!(!regex.contains(":long"));
        assert_eq!(map.get("count").unwrap(), "count");
    }

    // --- Boundary value tests ---

    #[test]
    fn internal_ip_boundary_first_last() {
        assert!(is_internal_ip("10.0.0.0"));
        assert!(is_internal_ip("10.255.255.255"));
        assert!(is_internal_ip("192.168.0.0"));
        assert!(is_internal_ip("192.168.255.255"));
        assert!(!is_internal_ip("0.0.0.0")); // not private
        assert!(!is_internal_ip("255.255.255.255")); // broadcast
    }

    #[test]
    fn community_id_boundary_ips() {
        // Loopback
        let result = community_id_v1("127.0.0.1", "127.0.0.1", 80, 80, "tcp");
        assert!(result.is_ok());
        // IPv6 loopback
        let result = community_id_v1("::1", "::1", 80, 80, "tcp");
        assert!(result.is_ok());
        // Zero port
        let result = community_id_v1("1.2.3.4", "5.6.7.8", 0, 0, "icmp");
        assert!(result.is_ok());
        // Max port
        let result = community_id_v1("1.2.3.4", "5.6.7.8", 65535, 65535, "tcp");
        assert!(result.is_ok());
    }

    #[test]
    fn community_id_unknown_protocol_errors() {
        let result = community_id_v1("1.2.3.4", "5.6.7.8", 80, 80, "unknown_protocol");
        // Unknown protocols are rejected, not defaulted — correct behaviour
        assert!(result.is_err());
    }

    #[test]
    fn registered_domain_deeply_nested() {
        let r = registered_domain_lookup("deep.sub.example.com").unwrap();
        // rsplitn(3, '.') gives ["com", "example", "deep.sub"]
        assert_eq!(r.registered_domain, "example.com");
        assert_eq!(r.subdomain.as_deref(), Some("deep.sub"));
    }

    #[test]
    fn registered_domain_single_char() {
        let r = registered_domain_lookup("a.b");
        assert!(r.is_some());
        let r = r.unwrap();
        assert_eq!(r.registered_domain, "a.b");
    }

    #[test]
    fn grok_regex_actually_matches() {
        // Verify generated regex can actually match input
        let regex_str = grok_to_regex("^%{IP:src}:%{POSINT:port}$");
        let re = regex::Regex::new(&regex_str).expect("regex should compile");
        let caps = re.captures("10.0.0.1:8080").expect("should match");
        assert_eq!(caps.name("src").unwrap().as_str(), "10.0.0.1");
        assert_eq!(caps.name("port").unwrap().as_str(), "8080");
    }

    #[test]
    fn grok_regex_rejects_non_matching() {
        let regex_str = grok_to_regex("^%{IP:src}$");
        let re = regex::Regex::new(&regex_str).expect("regex should compile");
        assert!(re.captures("not-an-ip").is_none());
        assert!(re.captures("").is_none());
    }

    #[test]
    fn parse_ua_returns_other_for_unknown() {
        let result = parse_user_agent("SomeRandomBot/1.0").unwrap();
        assert_eq!(result.name.as_deref(), Some("Other"));
    }

    // --- Mutation resistance test ---
    // If you comment out the type-suffix stripping in grok_to_regex_with_map,
    // this test MUST fail (verifies the test catches the bug)
    #[test]
    fn grok_type_suffix_compiles_as_valid_regex() {
        let (regex, _) = grok_to_regex_with_map("%{IP:source.ip:ip}:%{POSINT:port:long}");
        // This MUST compile — if type suffix isn't stripped, it won't
        let re = regex::Regex::new(&regex);
        assert!(
            re.is_ok(),
            "grok regex with type suffixes should compile: {:?}",
            re.err()
        );
    }
}
