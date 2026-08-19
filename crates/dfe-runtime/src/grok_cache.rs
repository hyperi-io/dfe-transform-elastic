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
        static SITE: ::std::sync::OnceLock<&'static ::regex::Regex> = ::std::sync::OnceLock::new();
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
    Ipv4Port { addr: String, port: String },
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
    /// A native parser for this pattern, when one covers it exactly.
    native: Option<Native>,
}

/// Matches nothing, ever: one character that is both non-whitespace and
/// non-non-whitespace. `$^` looks like it should work and does not -- both
/// anchors hold at position 0, so it matches the empty string.
const NEVER_MATCHES: &str = r"[^\s\S]";

static GROK: RwLock<Option<HashMap<String, &'static CompiledGrok>>> = RwLock::new(None);
static PLAIN: RwLock<Option<HashMap<String, &'static Regex>>> = RwLock::new(None);

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

    let (expanded, mut field_map) = crate::codegen_api::grok_to_regex_with_map(pattern);
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
    let port = capture_of(&format!("%{{{right}"), "PORT")?.to_string();
    Some(Native::Ipv4Port { addr, port })
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
                event.set(path, m.as_str())?;
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
            Native::Ipv4Port { addr, port } => {
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
                // The regex path captures text, so the port is set as text too
                // -- changing the type here would be a silent behaviour change.
                event.set(port, parsed_port.to_string())?;
                Ok(true)
            }
        }
    }
}

/// A plain regex literal, compiled once per distinct pattern.
///
/// For the hand-written patterns that are not grok. Prefer a `str` operation
/// where one exists -- splitting on a character does not need a regex engine.
#[must_use]
pub fn regex(pattern: &str) -> &'static Regex {
    if let Some(hit) = PLAIN
        .read()
        .ok()
        .and_then(|g| g.as_ref().and_then(|map| map.get(pattern)).copied())
    {
        return hit;
    }

    let compiled = Regex::new(pattern).unwrap_or_else(|e| {
        tracing::error!(
            pattern = pattern,
            error = %e,
            "regex does not compile; this processor will match nothing"
        );
        #[allow(clippy::expect_used)]
        Regex::new(NEVER_MATCHES).expect("the never-matching pattern is valid")
    });
    let compiled: &'static Regex = Box::leak(Box::new(compiled));

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
