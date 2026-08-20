// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The free functions the transform modules call.
//!
//! One flat namespace, re-exported through [`crate::prelude`], so a transform
//! reads as a sequence of processor calls. They wrap the enrichment modules,
//! or stub what needs configuration this build may not have (`GeoIP`
//! databases, a Painless interpreter).

use std::collections::HashMap;
use std::sync::LazyLock;

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

/// Look up `GeoIP` data for an IP address.
///
/// Returns a flat map of field names to values (e.g., "`country_iso_code`" -> "AU").
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
/// `keys_to_snake_case`, etc.). Falls back to a no-op for unrecognised scripts.
pub fn painless_exec(event: &mut Event, script: &str) -> Result<()> {
    painless_exec_params(event, script, &serde_json::Value::Null)
}

/// Execute a Painless script that carries a `params` block.
///
/// The recurring params shapes -- sentinel lists, field lists, lookup tables --
/// read their whole behaviour out of `params`, so the script text alone cannot
/// run them. The generated code passes the pipeline's params block verbatim.
pub fn painless_exec_params(
    event: &mut Event,
    script: &str,
    params: &serde_json::Value,
) -> Result<()> {
    if crate::painless_params::try_params_painless(event, script, params) {
        crate::painless_stats::record_handled();
        return Ok(());
    }
    if crate::painless_common::try_known_painless(event, script) {
        crate::painless_stats::record_handled();
        return Ok(());
    }
    // Counted, because an uncounted skip is indistinguishable from a script
    // that did nothing.
    crate::painless_stats::record_unhandled(script);
    debug!(
        script_len = script.len(),
        "painless_exec: unrecognised script skipped"
    );
    Ok(())
}

/// Convert a grok pattern string to a regex pattern string.
///
/// Expands `%{NAME:field}` to named capture groups with type-appropriate
/// sub-patterns. Returns `(regex_string, field_map)` where `field_map` maps
/// safe capture names back to original dotted field paths.
///
/// Phase 3 will replace grok with native dfe-parse parsers.
pub fn grok_to_regex(pattern: &str) -> String {
    grok_to_regex_with_map(pattern).0
}

/// Like `grok_to_regex` but also returns a map of `capture_name` → `original_field_path`.
///
/// This is needed because regex capture names can't contain dots, so
/// `user.name` becomes `user_name` in the regex. The map lets callers
/// restore the original dotted path when setting fields.
pub fn grok_to_regex_with_map(
    pattern: &str,
) -> (String, std::collections::HashMap<String, String>) {
    let (regex, field_map, _) = grok_to_regex_typed(pattern);
    (regex, field_map)
}

/// As [`grok_to_regex_with_map`], plus the captures Elastic types as numbers.
///
/// A `%{NUMBER:bytes:long}` suffix is a type, not part of the field name, and
/// dropping it leaves every numeric field a string.
#[must_use]
pub fn grok_to_regex_typed(
    pattern: &str,
) -> (
    String,
    std::collections::HashMap<String, String>,
    std::collections::HashMap<String, bool>,
) {
    let mut result = String::with_capacity(pattern.len());
    let mut field_map = std::collections::HashMap::new();
    let mut numeric = std::collections::HashMap::new();
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

            if field.is_empty() {
                use std::fmt::Write as _;
                // A few builtins carry their own destination, as Elastic's own
                // registry defines them -- used bare, they still capture.
                if let Some((safe, path, inner)) = grok_implicit_capture(&name) {
                    field_map.insert(safe.to_string(), path.to_string());
                    numeric.insert(safe.to_string(), true);
                    let _ = write!(result, "{inner}");
                } else {
                    let _ = write!(result, "({sub_pattern})");
                }
            } else {
                use std::fmt::Write as _;
                // Strip Elastic type suffix (e.g., "source.ip:ip" → "source.ip")
                let mut parts = field.splitn(2, ':');
                let field_name = parts.next().unwrap_or(&field);
                let mut safe_field = field_name.replace('.', "_");

                // Java allows one field name in several alternation branches
                // and Rust's engine rejects a duplicate group name outright, so
                // the whole pattern fails to compile and matches nothing. Each
                // repeat gets a group of its own pointing at the same field;
                // only the branch that matched writes.
                let repeats = field_map.values().filter(|p| *p == field_name).count();
                if repeats > 0 {
                    safe_field = format!("{safe_field}__{}", repeats + 1);
                }

                if matches!(parts.next(), Some("long" | "int" | "float" | "double")) {
                    numeric.insert(safe_field.clone(), true);
                }
                field_map.insert(safe_field.clone(), field_name.to_string());
                let _ = write!(result, "(?P<{safe_field}>{sub_pattern})");
            }
        } else {
            result.push(c);
        }
    }

    (result, field_map, numeric)
}

/// Builtins whose Elastic definition captures a field of its own.
///
/// `%{SYSLOG5424PRI}` is written without a field name throughout the vendor
/// pipelines because the destination is part of the pattern. Returns
/// `(safe capture name, dotted path, the regex to emit)`.
fn grok_implicit_capture(name: &str) -> Option<(&'static str, &'static str, &'static str)> {
    match name {
        "SYSLOG5424PRI" => Some((
            "log_syslog_priority",
            "log.syslog.priority",
            r"<(?P<log_syslog_priority>\d{1,5})>",
        )),
        _ => None,
    }
}

/// A month name, abbreviated or spelled out.
///
/// `\w+` would do here too, but it also matches a bare word, so a pattern
/// meant to anchor on a date matches text that holds none.
const MONTH: &str = concat!(
    r"(?:Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?",
    r"|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)",
);

/// A clock time. Elastic's `SECOND` carries an optional fraction, so a
/// pattern anchored on `%{TIME}` has to accept `13:20:48.739`.
const TIME: &str = r"\d{1,2}:\d{2}(?::\d{2}(?:[.,]\d+)?)?";

/// `%{MONTH} +%{MONTHDAY} %{TIME}` -- the BSD syslog date, fraction and all.
const SYSLOG_TIMESTAMP: &str = concat!(
    r"(?:Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?",
    r"|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)",
    r" +\d{1,2} \d{1,2}:\d{2}(?::\d{2}(?:[.,]\d+)?)?",
);

/// Cisco's syslog date: the year may sit on either side of the time, and the
/// seconds may carry a fraction -- `Jan  6 2022 20:52:12.861`.
const CISCO_TIMESTAMP: &str = concat!(
    r"(?:Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?",
    r"|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)",
    r" +\d{1,2}(?: \d{4})? \d{2}:\d{2}:\d{2}(?:\.\d+)?(?: \d{4})?",
);

/// Elastic's own `IPV6`, verbatim from logstash-patterns-core's ecs-v1 set.
///
/// The `[0-9a-fA-F:]+` this replaces matched any run of hex and colons -- a
/// bare `2a02` included -- and `IP` carried no v6 branch at all, so a grok
/// reading a v6 address failed outright and took every capture in the pattern
/// with it. `cisco_ios`'s syslog header is exactly that shape.
const IPV6: &str = r"((([0-9A-Fa-f]{1,4}:){7}([0-9A-Fa-f]{1,4}|:))|(([0-9A-Fa-f]{1,4}:){6}(:[0-9A-Fa-f]{1,4}|((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(([0-9A-Fa-f]{1,4}:){5}(((:[0-9A-Fa-f]{1,4}){1,2})|:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(([0-9A-Fa-f]{1,4}:){4}(((:[0-9A-Fa-f]{1,4}){1,3})|((:[0-9A-Fa-f]{1,4})?:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){3}(((:[0-9A-Fa-f]{1,4}){1,4})|((:[0-9A-Fa-f]{1,4}){0,2}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){2}(((:[0-9A-Fa-f]{1,4}){1,5})|((:[0-9A-Fa-f]{1,4}){0,3}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){1}(((:[0-9A-Fa-f]{1,4}){1,6})|((:[0-9A-Fa-f]{1,4}){0,4}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(:(((:[0-9A-Fa-f]{1,4}){1,7})|((:[0-9A-Fa-f]{1,4}){0,5}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:)))(%.+)?";

/// The simplified `IPV4`. Elastic's own carries `(?<![0-9])` look-around,
/// which the regex crate rejects -- and a grok that will not compile matches
/// nothing at all, which is worse than accepting `999.999.999.999`.
const IPV4: &str = r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}";

/// A hostname as Elastic defines it, for the composites below.
const HOSTNAME: &str = r"[a-zA-Z0-9._-]+";

/// `%{IP}` is `(?:%{IPV6}|%{IPV4})` and `%{IPORHOST}` is `(?:%{IP}|%{HOSTNAME})`,
/// v6 first, exactly as Elastic orders them. Built once because `concat!` will
/// not expand a const, and this runs when a pattern compiles, never per event.
static IP: LazyLock<String> = LazyLock::new(|| format!("(?:{IPV6}|{IPV4})"));
static IPORHOST: LazyLock<String> = LazyLock::new(|| format!("(?:{IPV6}|{IPV4}|{HOSTNAME})"));

/// Map well-known grok pattern names to their regex equivalents.
fn grok_pattern_regex(name: &str) -> &'static str {
    match name {
        "USER" | "USERNAME" | "HOSTNAME" => HOSTNAME,
        "IP" => IP.as_str(),
        "IPV4" => IPV4,
        "IPV6" => IPV6,
        "POSINT" | "PORT" | "NONNEGINT" => r"\d+",
        "INT" => r"[+-]?\d+",
        "NUMBER" | "BASE10NUM" => r"[+-]?(?:\d+\.?\d*|\.\d+)",
        "NOTSPACE" | "URI" | "URIPROTO" => r"\S+",
        "GREEDYDATA" => r".*",
        "DATA" => r".*?",
        "WORD" => r"\w+",
        "MONTH" => MONTH,
        "MAC" => r"(?:[0-9a-fA-F]{2}[:-]){5}[0-9a-fA-F]{2}",
        "EMAILADDRESS" => r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}",
        "PATH" | "UNIXPATH" | "WINPATH" => r"[^\s]+",
        // Azure custom patterns (from pipeline pattern_definitions)
        "SUBID" => {
            r"(?:\{)?[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}(?:\})?"
        }
        // A path segment. `PROVIDER` on the catch-all `.+?` matched lazily,
        // so `/providers/Microsoft.aadiam` yielded "M".
        "GROUPID" | "PROVIDERNAME" | "PROVIDER" | "NAMESPACE" | "RULE" | "NAME" => r"[^/]+",
        "MONTHDAY" | "MONTHNUM" => r"\d{1,2}",
        "YEAR" => r"\d{4}",
        "HOUR" | "MINUTE" | "SECOND" => r"\d{2}",
        // Whitespace, not "anything". The catch-all below made every pattern
        // containing %{SPACE} match arbitrary text -- 152 sites' worth.
        "SPACE" => r"\s*",
        "TIME" => TIME,
        "IPORHOST" | "SYSLOGHOST" => IPORHOST.as_str(),
        // Elastic accepts the abbreviation or the full name, either case.
        "DAY" => {
            r"(?i:Mon(?:day)?|Tue(?:sday)?|Wed(?:nesday)?|Thu(?:rsday)?|Fri(?:day)?|Sat(?:urday)?|Sun(?:day)?)"
        }
        "SYSLOGPRI" => r"<\d+>",
        "SYSLOG5424PRI" => r"<\d{1,5}>",
        // Printable ASCII minus space, `=`, `]` and `"` -- RFC 5424's own
        // definition, which is what bounds a structured-data name.
        "SYSLOG5424PRINTASCII" => r"[!#-<>-\\\^-~]+",
        "SYSLOGTIMESTAMP" => SYSLOG_TIMESTAMP,
        "CISCOTIMESTAMP" => CISCO_TIMESTAMP,
        "CISCOMAC" => r"(?:[A-Fa-f0-9]{4}\.){2}[A-Fa-f0-9]{4}",
        // Elastic quotes with any of the three, and reading only the double
        // form left cisco_meraki's `ssid=''` unmatched -- which took the whole
        // key-value line with it, on every airmarshal event.
        "QS" | "QUOTEDSTRING" => r#"(?:"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'|`(?:[^`\\]|\\.)*`)"#,
        "LOGLEVEL" => r"(?i:emerg|alert|crit|err|warn|notice|info|debug|trace)\w*",
        "TIMESTAMP_ISO8601" => {
            r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:?\d{2})?"
        }
        // Unknown name. `.+?` captures arbitrary text rather than failing, so
        // the field is populated with the wrong thing and nothing says so --
        // `unknown_grok_patterns` exists to make that visible.
        _ => ".+?",
    }
}

/// Pattern names still falling through to the catch-all, and how often.
///
/// The vendor-specific ones (`CISCO_*`, `NEXUS_*`, `IPV6PORTSEP`, ...) come
/// from `pattern_definitions` in the upstream ingest pipelines and have to be
/// carried across before they can be defined here.
#[must_use]
pub fn is_known_grok_pattern(name: &str) -> bool {
    grok_pattern_regex(name) != ".+?"
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

/// Resolve a field PATH that carries mustache references, against the event.
///
/// A processor's `field` may name itself from the document -- `cisco_meraki`
/// renames `cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_mac`, so the
/// subtree it reads is whatever the event's subtype says. Emitting the template
/// verbatim gave a path nothing ever matched.
///
/// Returns `None` when a reference resolves to nothing, because the name it
/// would build has an empty segment and matches no field either.
#[must_use]
pub fn resolve_path(event: &Event, template: &str) -> Option<String> {
    if !template.contains("{{") {
        return Some(template.to_string());
    }

    let mut resolved = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(open) = rest.find("{{") {
        resolved.push_str(&rest[..open]);

        // Mustache spells an unescaped reference with three braces and an
        // escaped one with two; a field path wants the value either way.
        let after = &rest[open..];
        let inner = after.trim_start_matches('{');
        let close = "}".repeat(after.len() - inner.len());
        let (name, tail) = inner.split_once(&close)?;

        let value = event.get_as_string(name.trim())?;
        if value.is_empty() {
            return None;
        }
        resolved.push_str(&value);
        rest = tail;
    }
    resolved.push_str(rest);
    Some(resolved)
}

/// The pieces of a URI reference, borrowed from the string they came from.
struct UriRef<'a> {
    scheme: Option<&'a str>,
    user_info: Option<&'a str>,
    host: Option<&'a str>,
    port: Option<&'a str>,
    path: &'a str,
    query: Option<&'a str>,
    fragment: Option<&'a str>,
}

/// Split a URI reference into its parts, per RFC 3986's generic syntax.
///
/// A relative reference is the case that matters and the one a URL crate will
/// not take: fortinet's `url` field is a bare path far more often than a whole
/// URL, and rejecting those loses `url.path` on every one of them.
fn split_uri(uri: &str) -> UriRef<'_> {
    let (rest, fragment) = match uri.split_once('#') {
        Some((before, after)) => (before, Some(after)),
        None => (uri, None),
    };
    let (rest, query) = match rest.split_once('?') {
        Some((before, after)) => (before, Some(after)),
        None => (rest, None),
    };

    // A scheme runs to the first `:`, but only when nothing before it could
    // make that colon part of a path or an authority instead.
    let (rest, scheme) = match rest.find(':') {
        Some(colon) if is_scheme(&rest[..colon]) => (&rest[colon + 1..], Some(&rest[..colon])),
        _ => (rest, None),
    };

    let Some(after_slashes) = rest.strip_prefix("//") else {
        return UriRef {
            scheme,
            user_info: None,
            host: None,
            port: None,
            path: rest,
            query,
            fragment,
        };
    };

    let end = after_slashes.find('/').unwrap_or(after_slashes.len());
    let (authority, path) = after_slashes.split_at(end);

    let (user_info, host_port) = match authority.rsplit_once('@') {
        Some((user, host)) => (Some(user), host),
        None => (None, authority),
    };

    // Only a colon after the closing bracket separates an IPv6 host from its
    // port -- the address is full of them.
    let colon = match host_port.rfind(']') {
        Some(bracket) => host_port[bracket..].find(':').map(|at| bracket + at),
        None => host_port.rfind(':'),
    };
    let (host, port) = match colon {
        Some(at) => (&host_port[..at], Some(&host_port[at + 1..])),
        None => (host_port, None),
    };

    UriRef {
        scheme,
        user_info,
        host: Some(host),
        port,
        path,
        query,
        fragment,
    }
}

/// Whether `candidate` is a scheme: `ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )`.
fn is_scheme(candidate: &str) -> bool {
    let mut chars = candidate.chars();
    chars.next().is_some_and(char::is_alphabetic)
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
}

/// The extension of a path, or `None` when its last segment has no dot.
///
/// Read off the corpus rather than assumed: Elasticsearch 9.2.2 gives
/// `/api/v2/cmdb/log.fortianalyzer/setting` NO extension, so the dot has to be
/// in the LAST segment -- a dotted directory earlier in the path does not
/// count. `/virus/eicar.com` gives `com` and `/config/` gives none.
fn path_extension(path: &str) -> Option<&str> {
    let segment = path.rsplit('/').next()?;
    let dot = segment.rfind('.')?;
    let extension = &segment[dot + 1..];
    (!extension.is_empty()).then_some(extension)
}

/// Split `field` into ECS `url.*` components under `target`.
///
/// Elastic's `uri_parts` processor. Returns whether anything was written, so
/// the caller can run its `on_failure` block: a value that is not a string, or
/// an empty one, writes nothing.
///
/// # Errors
///
/// Returns [`crate::TransformError`] if a component cannot be set.
pub fn uri_parts(
    event: &mut Event,
    field: &str,
    target: &str,
    keep_original: bool,
    remove_if_successful: bool,
) -> Result<bool> {
    let Some(original) = event.get_string(field) else {
        return Ok(false);
    };
    if original.is_empty() {
        return Ok(false);
    }

    let uri = split_uri(&original);
    let mut parts = serde_json::Map::new();

    if let Some(scheme) = uri.scheme {
        parts.insert("scheme".into(), Value::String(scheme.to_owned()));
    }
    if let Some(user_info) = uri.user_info {
        parts.insert("user_info".into(), Value::String(user_info.to_owned()));
        match user_info.split_once(':') {
            Some((username, password)) => {
                parts.insert("username".into(), Value::String(username.to_owned()));
                parts.insert("password".into(), Value::String(password.to_owned()));
            }
            None => {
                parts.insert("username".into(), Value::String(user_info.to_owned()));
            }
        }
    }
    if let Some(host) = uri.host.filter(|h| !h.is_empty()) {
        parts.insert("domain".into(), Value::String(host.to_owned()));
    }
    // A port that is not a number is left out rather than stored as text: the
    // ECS field is numeric and Elastic's processor drops it the same way.
    if let Some(port) = uri.port.and_then(|p| p.parse::<u32>().ok()) {
        parts.insert("port".into(), Value::Number(port.into()));
    }
    if !uri.path.is_empty() {
        parts.insert("path".into(), Value::String(uri.path.to_owned()));
        if let Some(extension) = path_extension(uri.path) {
            parts.insert("extension".into(), Value::String(extension.to_owned()));
        }
    }
    if let Some(query) = uri.query {
        parts.insert("query".into(), Value::String(query.to_owned()));
    }
    if let Some(fragment) = uri.fragment {
        parts.insert("fragment".into(), Value::String(fragment.to_owned()));
    }
    if keep_original {
        parts.insert("original".into(), Value::String(original.clone()));
    }

    // Elastic replaces the target wholesale with the parsed object, so a
    // scalar sitting there is gone before the parts land. Writing leaf by leaf
    // into a string instead fails on the FIRST leaf and loses every part with
    // it -- which is what happened to `url` on cisco_meraki's security events,
    // where the processor reads and writes the same field.
    if event.get(target).is_some_and(|v| !v.is_object()) {
        event.remove(target);
    }

    // Set leaf by leaf rather than replacing the target: the fortinet pipeline
    // writes `url.domain` from another field before and after this runs, and a
    // wholesale replace would discard it.
    for (key, value) in parts {
        event.set(&format!("{target}.{key}"), value)?;
    }

    if remove_if_successful && field != target {
        event.remove(field);
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // --- resolve_path ---

    // --- duplicate grok capture names ---

    /// Verbatim from `pipelines/cisco/ios/default.yml`. Java takes one field
    /// name in several alternation branches; Rust's engine rejects a duplicate
    /// group name outright, so the whole pattern failed to compile and the
    /// `BADAUTH` grok matched nothing on eighteen events.
    #[test]
    fn a_field_named_twice_compiles_and_both_branches_write_it() {
        let pattern = r"from %{DATA:source.address}(\(%{INT:source.port}\)|\:%{INT:source.port})";
        let (expanded, field_map, _) = grok_to_regex_typed(pattern);

        let re = regex::Regex::new(&expanded).expect("a repeated field name still compiles");
        assert_eq!(
            field_map.values().filter(|p| *p == "source.port").count(),
            2,
            "both groups must point at the same field"
        );

        for input in ["from 192.168.0.1(64999)", "from 192.168.0.1:64999"] {
            let caps = re.captures(input).unwrap_or_else(|| panic!("{input}"));
            let port = field_map
                .iter()
                .filter(|(_, path)| *path == "source.port")
                .find_map(|(group, _)| caps.name(group))
                .unwrap_or_else(|| panic!("{input}"));
            assert_eq!(port.as_str(), "64999", "{input}");
        }
    }

    /// Verbatim from `pipelines/cisco/meraki/events.yml`, where the subtree a
    /// rename reads is named by the event's own subtype.
    #[test]
    fn resolve_path_names_a_field_from_the_document() {
        let event = Event::new(json!({
            "cisco_meraki": { "event_subtype": "disassociation" },
        }));

        assert_eq!(
            resolve_path(
                &event,
                "cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_mac"
            )
            .as_deref(),
            Some("cisco_meraki.disassociation.client_mac")
        );
    }

    /// A reference that resolves to nothing would build a path with an empty
    /// segment, which matches no field -- so it is no path at all.
    #[test]
    fn resolve_path_refuses_an_unresolvable_reference() {
        let event = Event::new(json!({ "cisco_meraki": { "event_subtype": "" } }));
        assert_eq!(
            resolve_path(
                &event,
                "cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_mac"
            ),
            None
        );

        let empty = Event::new(json!({}));
        assert_eq!(resolve_path(&empty, "a.{{{missing}}}.b"), None);
    }

    /// A path with no reference in it is itself, and costs nothing to ask for.
    #[test]
    fn resolve_path_passes_a_plain_path_through() {
        let event = Event::new(json!({}));
        assert_eq!(
            resolve_path(&event, "client.mac").as_deref(),
            Some("client.mac")
        );
    }

    /// Mustache spells an escaped reference with two braces and an unescaped
    /// one with three; a field path wants the value either way.
    #[test]
    fn resolve_path_reads_both_brace_forms() {
        let event = Event::new(json!({ "kind": "assoc" }));
        assert_eq!(
            resolve_path(&event, "a.{{kind}}.b").as_deref(),
            Some("a.assoc.b")
        );
        assert_eq!(
            resolve_path(&event, "a.{{{kind}}}.b").as_deref(),
            Some("a.assoc.b")
        );
    }

    // --- uri_parts ---

    /// Every one of these is a real fortinet input paired with what
    /// Elasticsearch 9.2.2 produced for it, taken from `testdata/compat`.
    #[test]
    fn uri_parts_matches_the_captured_elasticsearch_output() {
        let cases: &[(&str, Value)] = &[
            ("/config/", json!({ "path": "/config/" })),
            ("/", json!({ "path": "/" })),
            (
                "http://172.16.200.55/virus/eicar.com",
                json!({
                    "scheme": "http",
                    "domain": "172.16.200.55",
                    "path": "/virus/eicar.com",
                    "extension": "com",
                }),
            ),
            (
                "/ips/sig1.pdf",
                json!({ "path": "/ips/sig1.pdf", "extension": "pdf" }),
            ),
            (
                "/api/v2/monitor/system/usb-log?vdom=root",
                json!({ "path": "/api/v2/monitor/system/usb-log", "query": "vdom=root" }),
            ),
            // The dotted DIRECTORY must not become an extension.
            (
                "/api/v2/cmdb/log.fortianalyzer/setting?vdom=root",
                json!({ "path": "/api/v2/cmdb/log.fortianalyzer/setting", "query": "vdom=root" }),
            ),
            (
                "https://172.16.200.88/dlp/files/fortiauto.pdf",
                json!({
                    "scheme": "https",
                    "domain": "172.16.200.88",
                    "path": "/dlp/files/fortiauto.pdf",
                    "extension": "pdf",
                }),
            ),
        ];

        for (input, expected) in cases {
            let mut event = Event::new(json!({ "src": input }));
            assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());
            assert_eq!(event.get("url"), Some(expected), "input: {input}");
        }
    }

    #[test]
    fn uri_parts_splits_an_authority_with_credentials_and_a_port() {
        let mut event =
            Event::new(json!({ "src": "https://bob:hunter2@example.com:8443/a?b=1#c" }));
        assert!(uri_parts(&mut event, "src", "url", true, false).unwrap());

        assert_eq!(event.get_str("url.scheme"), Some("https"));
        assert_eq!(event.get_str("url.domain"), Some("example.com"));
        assert_eq!(event.get("url.port"), Some(&json!(8443)));
        assert_eq!(event.get_str("url.user_info"), Some("bob:hunter2"));
        assert_eq!(event.get_str("url.username"), Some("bob"));
        assert_eq!(event.get_str("url.password"), Some("hunter2"));
        assert_eq!(event.get_str("url.path"), Some("/a"));
        assert_eq!(event.get_str("url.query"), Some("b=1"));
        assert_eq!(event.get_str("url.fragment"), Some("c"));
        assert_eq!(
            event.get_str("url.original"),
            Some("https://bob:hunter2@example.com:8443/a?b=1#c")
        );
    }

    /// An IPv6 authority is full of colons, so only the one after the closing
    /// bracket separates the port.
    #[test]
    fn uri_parts_keeps_an_ipv6_host_whole() {
        let mut event = Event::new(json!({ "src": "http://[2001:db8::1]:8080/x" }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());

        assert_eq!(event.get_str("url.domain"), Some("[2001:db8::1]"));
        assert_eq!(event.get("url.port"), Some(&json!(8080)));

        let mut event = Event::new(json!({ "src": "http://[2001:db8::1]/x" }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());
        assert_eq!(event.get_str("url.domain"), Some("[2001:db8::1]"));
        assert_eq!(event.get("url.port"), None);
    }

    /// The caller runs its `on_failure` on a false return, so a value that is
    /// not a usable string must say so rather than writing an empty subtree.
    #[test]
    fn uri_parts_reports_what_it_could_not_use() {
        let mut event = Event::new(json!({ "src": "", "num": 7 }));
        assert!(!uri_parts(&mut event, "src", "url", false, false).unwrap());
        assert!(!uri_parts(&mut event, "num", "url", false, false).unwrap());
        assert!(!uri_parts(&mut event, "absent", "url", false, false).unwrap());
        assert!(!event.has("url"));
    }

    /// The fortinet pipeline writes `url.domain` from the hostname field and
    /// then parses a path-only `url` over the top, so the parse must add to the
    /// subtree rather than replace it.
    #[test]
    fn uri_parts_adds_to_the_target_rather_than_replacing_it() {
        let mut event = Event::new(json!({ "src": "/config/", "url": { "domain": "elastic.co" } }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());

        assert_eq!(event.get_str("url.domain"), Some("elastic.co"));
        assert_eq!(event.get_str("url.path"), Some("/config/"));
    }

    /// `cisco_meraki`'s security events parse `url` INTO `url`. A string sitting
    /// on the target has to go first -- writing `url.scheme` into a string
    /// fails on that first leaf and every other part is lost with it, which is
    /// how the event ended up with a scalar `url` and no parts at all.
    #[test]
    fn uri_parts_replaces_a_scalar_sitting_on_the_target() {
        let mut event = Event::new(json!({ "url": "http://www.eicar.org/download/eicar.com.txt" }));
        assert!(uri_parts(&mut event, "url", "url", true, false).unwrap());

        assert_eq!(
            event.get_str("url.original"),
            Some("http://www.eicar.org/download/eicar.com.txt")
        );
        assert_eq!(event.get_str("url.scheme"), Some("http"));
        assert_eq!(event.get_str("url.domain"), Some("www.eicar.org"));
        assert_eq!(event.get_str("url.path"), Some("/download/eicar.com.txt"));
        assert_eq!(event.get_str("url.extension"), Some("txt"));
    }

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
        // Reaches the process-global cache, so it takes the same lock the
        // enrichment tests do.
        let _guard = crate::enrichment::geoip_global::test_guard();
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
        // Verify the compiled regex can actually match input
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
