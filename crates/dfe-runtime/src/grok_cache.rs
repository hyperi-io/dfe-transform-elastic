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

/// A grok pattern in its compiled form.
pub struct CompiledGrok {
    /// The expanded regex.
    pub regex: Regex,
    /// Capture name to original dotted field path. Regex capture names cannot
    /// contain dots, so `user.name` is captured as `user_name` and restored
    /// through this map.
    pub field_map: HashMap<String, String>,
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
    if let Some(hit) = GROK
        .read()
        .ok()
        .and_then(|g| g.as_ref().and_then(|map| map.get(pattern)).copied())
    {
        return hit;
    }

    let (expanded, field_map) = crate::codegen_api::grok_to_regex_with_map(pattern);
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

    let compiled: &'static CompiledGrok = Box::leak(Box::new(CompiledGrok { regex, field_map }));

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
