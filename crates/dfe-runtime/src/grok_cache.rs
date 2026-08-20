// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Grok and regex patterns, compiled once per process instead of per event.
//!
//! Every grok site in the transforms used to convert the pattern string and
//! call `Regex::new` inside the transform function, so a 20,000-event batch
//! rebuilt the same DFA 20,000 times. Compiling a regex is microseconds;
//! matching one is nanoseconds. That ratio is the whole reason this exists.
//!
//! The pattern set is FIXED at compile time -- every caller passes a string
//! literal -- so the map fills during the first batch and is read-only after
//! that. Entries are leaked deliberately: they live for the life of the
//! process anyway, and `&'static` lets callers hold the compiled form without
//! a guard or a refcount on the hot path.
//!
//! A pattern that will not compile is logged once and answered with a regex
//! that matches nothing, rather than panicking. A malformed pattern is a
//! defect in one processor; taking the pod down over it stalls a partition.

use std::collections::HashMap;
use std::sync::RwLock;

use regex::Regex;

/// The compiled form of a grok pattern literal, looked up once per CALL SITE.
///
/// [`grok`]'s shared `RwLock` costs 29 ns a lookup on one thread and 1,522 ns
/// on eight, because every worker bounces the same reader-count atomic. A
/// per-site `OnceLock` in front of it holds at ~1 ns under the same load.
///
/// The map stays underneath, so sites sharing a pattern share one instance.
#[macro_export]
macro_rules! cached_grok {
    ($pattern:literal $(,)?) => {{
        static SITE: ::std::sync::OnceLock<&'static $crate::grok_cache::CompiledGrok> =
            ::std::sync::OnceLock::new();
        *SITE.get_or_init(|| $crate::grok_cache::grok($pattern))
    }};
}

/// As [`cached_grok`], plus capture names the pattern carries as raw groups.
///
/// A pipeline may define its own grok name and capture it into a dotted path.
/// Inlining the definition turns that into a plain `(?P<a_b>...)` group, which
/// the expander never sees as `%{NAME:a.b}` and so never maps -- the value
/// would land on `a_b` and every later processor would miss it. The pairs here
/// are that mapping, supplied by whoever built the pattern.
#[macro_export]
macro_rules! cached_grok_mapped {
    ($pattern:literal, [$(($capture:literal, $path:literal)),* $(,)?] $(,)?) => {{
        static SITE: ::std::sync::OnceLock<&'static $crate::grok_cache::CompiledGrok> =
            ::std::sync::OnceLock::new();
        *SITE.get_or_init(|| $crate::grok_cache::grok_mapped($pattern, &[$(($capture, $path)),*]))
    }};
}

/// A plain regex literal, looked up once per CALL SITE. See [`cached_grok`].
#[macro_export]
macro_rules! cached_regex {
    ($pattern:literal $(,)?) => {{
        static SITE: ::std::sync::OnceLock<&'static $crate::grok_cache::Pattern> =
            ::std::sync::OnceLock::new();
        *SITE.get_or_init(|| $crate::grok_cache::regex($pattern))
    }};
}

/// A grok pattern whose whole shape a native parser can handle.
///
/// Even against an already-compiled regex, `dfe-parse` is roughly 4-5x faster
/// on these -- 79.7ns to 16.3ns for a bare address, 77.2ns to 18.4ns for an
/// address and port. That margin is why the native path exists; it is not
/// worth the divergence for shapes where the margin is not there.
///
/// Only whole-pattern matches qualify. A pattern with literal text around the
/// captures stays on the regex path, because the regex engine is genuinely
/// good at that and hand-rolling it would be a source of bugs.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Native {
    /// `^%{IPV4:field}$`
    Ipv4 { field: String },
    /// `^%{IPV4:addr}:%{PORT:port}$`
    Ipv4Port {
        addr: String,
        port: String,
        /// Whether the port capture declared a numeric type, which the regex
        /// path honours -- the two must write the same type.
        port_numeric: bool,
    },
}

/// A grok pattern in its compiled form.
pub struct CompiledGrok {
    /// The expanded regex. Always present, and always correct -- the native
    /// path below is an optimisation over it, never a replacement.
    pub regex: Regex,
    /// Capture name to original dotted field path. Regex capture names cannot
    /// contain dots, so `user.name` is captured as `user_name` and restored
    /// through this map.
    pub field_map: HashMap<String, String>,
    /// Capture names Elastic's `:long` / `:int` / `:float` suffix types as a
    /// number. Without this every one lands as a string and the expectations
    /// -- and every numeric comparison downstream -- see the wrong type.
    pub numeric: HashMap<String, bool>,
    /// A native parser for this pattern, when one covers it exactly.
    native: Option<Native>,
}

/// Matches nothing, ever: one character that is both non-whitespace and
/// non-non-whitespace. `$^` looks like it should work and does not -- both
/// anchors hold at position 0, so it matches the empty string.
const NEVER_MATCHES: &str = r"[^\s\S]";

static GROK: RwLock<Option<HashMap<String, &'static CompiledGrok>>> = RwLock::new(None);
static PLAIN: RwLock<Option<HashMap<String, &'static Pattern>>> = RwLock::new(None);

/// The compiled form of a grok `pattern`, built once per distinct pattern.
///
/// # Panics
///
/// Does not panic on a malformed pattern -- see the module docs. The `expect`
/// on the fallback cannot fire: [`NEVER_MATCHES`] is a valid regex, pinned by
/// a test.
#[must_use]
pub fn grok(pattern: &str) -> &'static CompiledGrok {
    grok_mapped(pattern, &[])
}

/// As [`grok`], with `extra` capture-name to dotted-path pairs merged into the
/// field map for groups the expander cannot see. See [`cached_grok_mapped`].
///
/// # Panics
///
/// Does not panic -- see [`grok`].
#[must_use]
pub fn grok_mapped(pattern: &str, extra: &[(&str, &str)]) -> &'static CompiledGrok {
    if let Some(hit) = GROK
        .read()
        .ok()
        .and_then(|g| g.as_ref().and_then(|map| map.get(pattern)).copied())
    {
        return hit;
    }

    let (expanded, mut field_map, numeric) = crate::codegen_api::grok_to_regex_typed(pattern);
    for (capture, path) in extra {
        field_map.insert((*capture).to_string(), (*path).to_string());
    }
    let regex = Regex::new(&expanded).unwrap_or_else(|e| {
        tracing::error!(
            grok = pattern,
            expanded = expanded,
            error = %e,
            "grok pattern does not compile; this processor will match nothing"
        );
        #[allow(clippy::expect_used)]
        Regex::new(NEVER_MATCHES).expect("the never-matching pattern is valid")
    });

    let compiled: &'static CompiledGrok = Box::leak(Box::new(CompiledGrok {
        regex,
        field_map,
        numeric,
        native: native_form(pattern),
    }));

    if let Ok(mut guard) = GROK.write() {
        // Another thread may have inserted the same pattern first. Keep the
        // winner so every caller shares one instance; the loser is leaked and
        // that is a bounded, one-off cost per pattern.
        return guard
            .get_or_insert_with(HashMap::new)
            .entry(pattern.to_string())
            .or_insert(compiled);
    }
    compiled
}

/// Recognise the whole-pattern shapes a native parser covers.
///
/// Deliberately literal: it matches the exact pattern strings the transforms
/// use rather than parsing grok generally. A near-miss must fall through to
/// the regex, never guess.
fn native_form(pattern: &str) -> Option<Native> {
    let body = pattern.strip_prefix('^')?.strip_suffix('$')?;

    if let Some(field) = capture_of(body, "IPV4") {
        return Some(Native::Ipv4 {
            field: field.to_string(),
        });
    }

    // Split on the literal colon BETWEEN the two captures, not on the colon
    // inside `%{IPV4:field}` -- `split_once(':')` finds the wrong one.
    let (left, right) = body.split_once("}:%{")?;
    let addr = capture_of(&format!("{left}}}"), "IPV4")?.to_string();
    let port_spec = format!("%{{{right}");
    let port = capture_of(&port_spec, "PORT")?.to_string();
    Some(Native::Ipv4Port {
        addr,
        port,
        port_numeric: port_spec.ends_with(":long}")
            || port_spec.ends_with(":int}")
            || port_spec.ends_with(":float}")
            || port_spec.ends_with(":double}"),
    })
}

/// The field name in `%{TYPE:field}`, when `text` is exactly that and nothing
/// else.
fn capture_of<'a>(text: &'a str, kind: &str) -> Option<&'a str> {
    let inner = text
        .strip_prefix("%{")?
        .strip_suffix('}')?
        .strip_prefix(kind)?
        .strip_prefix(':')?;
    // A second `%{` would mean there is more in the pattern than this capture.
    if inner.contains('%') || inner.contains('}') {
        return None;
    }
    // `%{IPV4:src:ip}` -- the regex expansion drops the type suffix, so this
    // must drop it too or the two paths write different field paths.
    Some(inner.split_once(':').map_or(inner, |(field, _)| field))
}

impl CompiledGrok {
    /// Extract this pattern's captures from `input` into `event`.
    ///
    /// Returns whether the pattern matched. Uses the native parser when the
    /// pattern has one and falls back to the regex otherwise, so a caller does
    /// not need to know or care which ran.
    ///
    /// # Errors
    ///
    /// Propagates a failure to set a field on the event.
    pub fn extract_into(
        &self,
        input: impl AsRef<str>,
        event: &mut crate::Event,
    ) -> crate::Result<bool> {
        let input = input.as_ref();
        if let Some(native) = &self.native {
            return Self::extract_native(native, input, event);
        }

        let Some(caps) = self.regex.captures(input) else {
            return Ok(false);
        };
        for name in self.regex.capture_names().flatten() {
            if let Some(m) = caps.name(name) {
                let path = self.field_map.get(name).map_or(name, String::as_str);
                match m.as_str().parse::<i64>() {
                    Ok(n) if self.numeric.contains_key(name) => event.set(path, n)?,
                    _ => event.set(path, m.as_str())?,
                }
            }
        }
        Ok(true)
    }

    fn extract_native(
        native: &Native,
        input: &str,
        event: &mut crate::Event,
    ) -> crate::Result<bool> {
        match native {
            Native::Ipv4 { field } => match dfe_parse::ip::parse_ipv4(input) {
                // Anchored: a trailing remainder means the whole input was not
                // an address, which is what `^...$` demands.
                Ok(("", addr)) => {
                    event.set(field, addr)?;
                    Ok(true)
                }
                _ => Ok(false),
            },
            Native::Ipv4Port {
                addr,
                port,
                port_numeric,
            } => {
                let Ok((rest, parsed_addr)) = dfe_parse::ip::parse_ipv4(input) else {
                    return Ok(false);
                };
                let Some(rest) = rest.strip_prefix(':') else {
                    return Ok(false);
                };
                // Not `parse_port`, which rejects 0: firewall logs carry port
                // 0 for ICMP, and failing here would lose the address too.
                let Ok((tail, parsed_port)) = dfe_parse::numeric::parse_nonneg_int(rest) else {
                    return Ok(false);
                };
                if !tail.is_empty() || parsed_port > 65535 {
                    return Ok(false);
                }
                event.set(addr, parsed_addr)?;
                if *port_numeric {
                    // Bounded above by the 65535 check, so this always fits.
                    event.set(port, i64::try_from(parsed_port).unwrap_or_default())?;
                } else {
                    event.set(port, parsed_port.to_string())?;
                }
                Ok(true)
            }
        }
    }
}

/// A compiled pattern, on whichever engine can express it.
///
/// `regex` refuses lookaround by design, and the vendor pipelines use it --
/// cisco nexus formats a MAC with `(..)(?!$)`, azure excludes a literal with
/// `((?!AUTHORIZATIONRULES).)*`. Those used to compile to nothing and match
/// nothing, silently. `fancy_regex` is the fallback ONLY: it is reached when
/// `regex` rejects the pattern, so nothing on the hot path changes.
pub enum Pattern {
    /// The linear-time engine, which is every pattern that compiles on it.
    Fast(Regex),
    /// The backtracking engine, for lookaround and backreferences.
    Backtracking(fancy_regex::Regex),
}

impl Pattern {
    /// Replace every match, leaving the input untouched when there are none.
    #[must_use]
    pub fn replace_all<'t>(&self, text: &'t str, replacement: &str) -> std::borrow::Cow<'t, str> {
        match self {
            Self::Fast(re) => re.replace_all(text, replacement),
            Self::Backtracking(re) => re.replace_all(text, replacement),
        }
    }

    /// Split on every match.
    #[must_use]
    pub fn split(&self, text: &str) -> Vec<String> {
        match self {
            Self::Fast(re) => re.split(text).map(str::to_string).collect(),
            // A backtracking split can fail mid-way; the text as one piece is
            // the same answer a pattern that never matched would give.
            Self::Backtracking(re) => re
                .split(text)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_or_else(
                    |_| vec![text.to_string()],
                    |v| v.into_iter().map(str::to_string).collect(),
                ),
        }
    }

    /// Split on the first `limit - 1` matches, keeping the rest whole.
    #[must_use]
    pub fn splitn(&self, text: &str, limit: usize) -> Vec<String> {
        match self {
            Self::Fast(re) => re.splitn(text, limit).map(str::to_string).collect(),
            Self::Backtracking(re) => re
                .splitn(text, limit)
                .collect::<std::result::Result<Vec<_>, _>>()
                .map_or_else(
                    |_| vec![text.to_string()],
                    |v| v.into_iter().map(str::to_string).collect(),
                ),
        }
    }

    /// Whether the pattern matches anywhere in the text.
    #[must_use]
    pub fn is_match(&self, text: &str) -> bool {
        match self {
            Self::Fast(re) => re.is_match(text),
            Self::Backtracking(re) => re.is_match(text).unwrap_or(false),
        }
    }
}

/// A plain regex literal, compiled once per distinct pattern.
///
/// For the hand-written patterns that are not grok. Prefer a `str` operation
/// where one exists -- splitting on a character does not need a regex engine.
#[must_use]
pub fn regex(pattern: &str) -> &'static Pattern {
    if let Some(hit) = PLAIN
        .read()
        .ok()
        .and_then(|g| g.as_ref().and_then(|map| map.get(pattern)).copied())
    {
        return hit;
    }

    let compiled = match Regex::new(pattern) {
        Ok(re) => Pattern::Fast(re),
        Err(fast_err) => match fancy_regex::Regex::new(pattern) {
            Ok(re) => Pattern::Backtracking(re),
            Err(slow_err) => {
                tracing::error!(
                    pattern = pattern,
                    error = %fast_err,
                    backtracking_error = %slow_err,
                    "regex does not compile on either engine; this processor will match nothing"
                );
                #[allow(clippy::expect_used)]
                Pattern::Fast(
                    Regex::new(NEVER_MATCHES).expect("the never-matching pattern is valid"),
                )
            }
        },
    };
    let compiled: &'static Pattern = Box::leak(Box::new(compiled));

    if let Ok(mut guard) = PLAIN.write() {
        return guard
            .get_or_insert_with(HashMap::new)
            .entry(pattern.to_string())
            .or_insert(compiled);
    }
    compiled
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    #[test]
    fn the_same_pattern_returns_the_same_instance() {
        let first = grok("%{USER:user.name}");
        let second = grok("%{USER:user.name}");
        assert!(
            std::ptr::eq(first, second),
            "a repeated pattern must be compiled once, not twice"
        );
    }

    #[test]
    fn distinct_patterns_do_not_share() {
        let a = grok("%{WORD:a.b}");
        let b = grok("%{WORD:c.d}");
        assert!(!std::ptr::eq(a, b));
    }

    #[test]
    fn the_field_map_restores_dotted_paths() {
        let compiled = grok("%{USER:user.name}");
        assert_eq!(
            compiled.field_map.get("user_name").map(String::as_str),
            Some("user.name"),
            "the dotted path must survive the capture-name rewrite"
        );
    }

    /// Elastic's `:long` is a type, not part of the field name. Dropping it
    /// left every numeric capture a string.
    #[test]
    fn a_typed_capture_lands_as_a_number() {
        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            grok("^%{NUMBER:network.bytes:long} %{WORD:event.action}$")
                .extract_into("2282 blocked", &mut event)
                .expect("extraction")
        );

        assert_eq!(event.get_i64("network.bytes"), Some(2282));
        assert_eq!(event.get_str("event.action"), Some("blocked"));
    }

    /// `%{SYSLOG5424PRI}` is written without a field name because Elastic's
    /// own definition carries the destination.
    #[test]
    fn a_bare_pri_still_captures_its_priority() {
        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            grok("%{SYSLOG5424PRI}%{GREEDYDATA:rest}$")
                .extract_into("<188>date=2020-04-23", &mut event)
                .expect("extraction")
        );

        assert_eq!(event.get_i64("log.syslog.priority"), Some(188));
    }

    /// `cisco_ios` wraps its whole syslog preamble in an optional group, so both
    /// the priority and the hostname are captured from inside one.
    #[test]
    fn a_capture_inside_an_optional_group_is_written() {
        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            grok("^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?%{WORD:event.action}$")
                .extract_into("<190>3132517: blocked", &mut event)
                .expect("extraction")
        );

        assert_eq!(event.get_i64("log.syslog.priority"), Some(190));
        assert_eq!(event.get_str("event.action"), Some("blocked"));
    }

    /// The same group, not participating. An absent optional capture must
    /// leave the field unset rather than writing an empty string.
    #[test]
    fn an_optional_group_that_does_not_participate_writes_nothing() {
        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            grok("^(?:<%{NONNEGINT:log.syslog.priority:long}>)?%{WORD:event.action}$")
                .extract_into("blocked", &mut event)
                .expect("extraction")
        );

        assert!(!event.has("log.syslog.priority"));
        assert_eq!(event.get_str("event.action"), Some("blocked"));
    }

    /// The `cisco_ios` header pattern, exactly as the generator emits it, against a
    /// line from its own fixtures.
    #[test]
    fn the_cisco_ios_header_pattern_captures_its_preamble() {
        let mut event = crate::Event::new(serde_json::json!({}));
        let compiled = grok_mapped(
            r"^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?%{SYSLOGTIMESTAMP} %{IP} (?:(?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?)): )?(?:%{NUMBER:cisco.ios.sequence}: )?(?:(?P<cisco_ios_uptime>(?:(?:\d{1,4}:\d{2}:\d{2}|(?:(\d+)y)?(?:(\d+)w)?(?:(\d+)d)?(?:(\d+)h)?(?:(\d+)m)?(?:(\d+)s)?)))|(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\d{1,2}|[+-]\d{2}:\d{2})?)))?)): %{GREEDYDATA:_temp_.message}$",
            &[
                ("log_syslog_hostname", "log.syslog.hostname"),
                ("cisco_ios_uptime", "cisco.ios.uptime"),
                ("_temp__cisco_timestamp", "_temp_.cisco_timestamp"),
                ("_temp__tz", "_temp_.tz"),
            ],
        );

        let matched = compiled
            .extract_into(
                "<190>3132517: Jul 13 08:23:43 192.168.100.2 sw01: 3132779: Jul 14 2023 08:23:43.398 UTC: %FOO-6-BAR: Test header format",
                &mut event,
            )
            .expect("extraction");

        assert!(matched, "the header pattern did not match its own fixture");
        assert_eq!(event.get_i64("log.syslog.priority"), Some(190));
        assert_eq!(event.get_str("log.syslog.hostname"), Some("sw01"));
        assert_eq!(event.get_str("cisco.ios.sequence"), Some("3132779"));
    }

    /// Elastic's `QUOTEDSTRING` takes any of the three quote characters.
    /// Reading only the double form left `cisco_meraki`'s `ssid=''` unmatched,
    /// and the grok that failed carried the whole key-value line with it.
    #[test]
    fn a_quoted_string_takes_any_of_the_three_quotes() {
        for (input, expected) in [
            (r#"ssid="home""#, r#""home""#),
            ("ssid='home'", "'home'"),
            ("ssid=`home`", "`home`"),
            ("ssid=''", "''"),
        ] {
            let mut event = crate::Event::new(serde_json::json!({}));
            assert!(
                grok("^ssid=%{QS:network.name}$")
                    .extract_into(input, &mut event)
                    .expect("extraction"),
                "{input} did not match"
            );
            assert_eq!(event.get_str("network.name"), Some(expected), "{input}");
        }
    }

    #[test]
    fn a_plain_pattern_is_cached_too() {
        let first = regex(r"\d{6}$");
        let second = regex(r"\d{6}$");
        assert!(std::ptr::eq(first, second));
        assert!(first.is_match("abc123456"));
    }

    /// A malformed pattern must not take the process down -- one broken
    /// processor is not worth a stalled partition.
    #[test]
    fn a_malformed_pattern_matches_nothing_instead_of_panicking() {
        let compiled = regex("(unclosed");
        assert!(!compiled.is_match("unclosed"));
        assert!(!compiled.is_match(""));
    }

    // -- native path ------------------------------------------------------

    /// Run a pattern BOTH ways over the same input and require identical
    /// results. This is what makes the native path safe to enable: it is only
    /// ever an optimisation if it cannot disagree with the regex.
    fn assert_paths_agree(pattern: &str, input: &str) {
        let compiled = grok(pattern);
        assert!(
            compiled.native.is_some(),
            "{pattern} was expected to take the native path"
        );

        let mut native_event = crate::Event::new(serde_json::json!({}));
        let native_matched = compiled
            .extract_into(input, &mut native_event)
            .expect("native extraction");

        // The same work with the native path forced off.
        let regex_only = CompiledGrok {
            regex: compiled.regex.clone(),
            field_map: compiled.field_map.clone(),
            numeric: compiled.numeric.clone(),
            native: None,
        };
        let mut regex_event = crate::Event::new(serde_json::json!({}));
        let regex_matched = regex_only
            .extract_into(input, &mut regex_event)
            .expect("regex extraction");

        assert_eq!(
            native_matched, regex_matched,
            "{pattern} on {input:?}: native and regex disagree on whether it matched"
        );
        assert_eq!(
            native_event.as_value(),
            regex_event.as_value(),
            "{pattern} on {input:?}: native and regex produced different fields"
        );
    }

    #[test]
    fn native_and_regex_agree_on_addresses() {
        for input in [
            "192.168.1.100",
            "10.0.0.1",
            "255.255.255.255",
            "0.0.0.0",
            // Non-matches matter as much as matches.
            "not-an-ip",
            "",
            "192.168.1",
            "192.168.1.100.5",
            " 192.168.1.1",
            "192.168.1.1 ",
            "::1",
        ] {
            assert_paths_agree("^%{IPV4:source.ip}$", input);
        }
    }

    #[test]
    fn native_and_regex_agree_on_address_and_port() {
        for input in [
            "10.0.0.7:443",
            "192.168.1.1:1",
            "8.8.8.8:65535",
            // Non-matches.
            "10.0.0.7:",
            "10.0.0.7",
            // Port 0 is valid input here -- see the parser comment.
            "10.0.0.7:0",
            "10.0.0.7:443:8080",
            "host:443",
            "",
        ] {
            assert_paths_agree("^%{IPV4:_temp.src_ip}:%{PORT:sport}$", input);
        }
    }

    /// Where the two paths deliberately disagree, pinned so it cannot drift.
    ///
    /// Our grok expansions do not range-check: `%{IPV4}` is `\d{1,3}` per
    /// octet and `%{PORT}` is bare digits, so both accept values that are not
    /// valid. `dfe-parse` validates. Elastic's own patterns validate too, so
    /// the native path is the more faithful of the two -- but it is still a
    /// behaviour change, and the fixture match rates are what show it is safe.
    #[test]
    fn the_native_path_rejects_values_the_loose_regex_accepts() {
        for (pattern, input) in [
            ("^%{IPV4:source.ip}$", "192.168.1.256"),
            ("^%{IPV4:_temp.src_ip}:%{PORT:sport}$", "10.0.0.7:65536"),
        ] {
            let compiled = grok(pattern);
            let mut event = crate::Event::new(serde_json::json!({}));

            assert!(
                !compiled
                    .extract_into(input, &mut event)
                    .expect("extraction"),
                "{input} is out of range and the native parser must reject it"
            );
            assert!(
                compiled.regex.is_match(input),
                "{input}: the regex still accepts it -- this is the divergence, not a stale test"
            );
        }
    }

    /// Grok's type suffix (`%{IPV4:src:ip}`) is stripped by the regex
    /// expansion, so the native path must strip it too or the two write
    /// different field paths for the same pattern.
    #[test]
    fn native_and_regex_agree_on_type_suffixed_captures() {
        assert_paths_agree("^%{IPV4:source.ip:ip}$", "192.168.1.100");
        assert_paths_agree(
            "^%{IPV4:_temp.src_ip:ip}:%{PORT:sport:long}$",
            "10.0.0.7:443",
        );
    }

    /// A pattern with literal text around the captures must NOT be claimed by
    /// the native path -- the regex engine is the right tool for those.
    #[test]
    fn patterns_with_literals_stay_on_the_regex() {
        for pattern in [
            "^src=%{IPV4:source.ip}$",
            "%{IPV4:source.ip}",
            "^%{IPV4:a} %{PORT:b}$",
            "^%{WORD:a}$",
            "^%{IPV4:a}:%{WORD:b}$",
        ] {
            assert!(
                grok(pattern).native.is_none(),
                "{pattern} must not be claimed by the native path"
            );
        }
    }

    /// The point of the whole module: matching must not rebuild the regex.
    #[test]
    fn repeated_lookups_are_the_same_compiled_regex() {
        let pattern = "%{IPV4:source.ip}";
        let addresses: Vec<&'static CompiledGrok> = (0..1000).map(|_| grok(pattern)).collect();
        let first = addresses[0];
        assert!(
            addresses.iter().all(|c| std::ptr::eq(*c, first)),
            "1000 lookups must all return the one compiled instance"
        );
    }
}
