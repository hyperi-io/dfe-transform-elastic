// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Regex compilation and the per-pattern cache.
//!
//! Split from grok compilation because the two layer differently: a compiled
//! pattern is a primitive carrying no vendor knowledge, while a compiled GROK
//! needs the pattern registry and the capture-type rules above it. Keeping the
//! primitive here is what lets the Painless matchers reach a cached regex
//! without depending on the grok layer.

use std::collections::HashMap;
use std::sync::RwLock;

use regex::Regex;

/// Every distinct plain-regex pattern compiled so far.
static PLAIN: RwLock<Option<HashMap<String, &'static Pattern>>> = RwLock::new(None);

/// Matches nothing, ever: one character that is both non-whitespace and
/// non-non-whitespace. `$^` looks like it should work and does not -- both
/// anchors hold at position 0, so it matches the empty string.
pub const NEVER_MATCHES: &str = r"[^\s\S]";

/// A site-local cache over [`regex`], resolved once per call site.
///
/// The shared map underneath is the dedup layer and must not be read on the hot
/// path: its reader-count atomic bounces between cores once the service runs
/// one transform thread per partition.
#[macro_export]
macro_rules! cached_regex {
    ($pattern:literal $(,)?) => {{
        static SITE: ::std::sync::OnceLock<&'static $crate::regex_cache::Pattern> =
            ::std::sync::OnceLock::new();
        *SITE.get_or_init(|| $crate::regex_cache::regex($pattern))
    }};
}

/// A compiled pattern, on whichever engine can express it.
///
/// `regex` refuses lookaround by design, and the vendor pipelines use it --
/// cisco nexus formats a MAC with `(..)(?!$)`, azure excludes a literal with
/// `((?!AUTHORIZATIONRULES).)*`. Those used to compile to nothing and match
/// nothing, silently. `fancy_regex` is the fallback ONLY: it is reached when
/// `regex` rejects the pattern, so nothing on the hot path changes.
#[derive(Clone)]
pub enum Pattern {
    /// The linear-time engine, which is every pattern that compiles on it.
    Fast(Regex),
    /// The backtracking engine, for lookaround and backreferences.
    Backtracking(fancy_regex::Regex),
}

impl Pattern {
    /// Compile on the fast engine, falling back to the backtracking one.
    ///
    /// `source` is what the author wrote -- the grok, or the pattern itself --
    /// and only reaches the log line, so a reader is told which call site to
    /// go and look at rather than the expansion they never typed.
    #[must_use]
    pub fn compile(expanded: &str, source: &str) -> Self {
        match Regex::new(expanded) {
            Ok(re) => Self::Fast(re),
            Err(fast_err) => match fancy_regex::Regex::new(expanded) {
                Ok(re) => Self::Backtracking(re),
                Err(slow_err) => {
                    tracing::error!(
                        source = source,
                        expanded = expanded,
                        error = %fast_err,
                        backtracking_error = %slow_err,
                        "pattern does not compile on either engine; this processor will match nothing"
                    );
                    #[allow(clippy::expect_used)]
                    Self::Fast(
                        Regex::new(NEVER_MATCHES).expect("the never-matching pattern is valid"),
                    )
                }
            },
        }
    }

    /// The fast engine's regex, when the pattern compiled on it.
    ///
    /// For benchmarks that must measure one engine and not a dispatch.
    #[must_use]
    pub fn fast(&self) -> Option<&Regex> {
        match self {
            Self::Fast(re) => Some(re),
            Self::Backtracking(_) => None,
        }
    }

    /// Every capture group's name, in group order, `None` for the unnamed.
    #[must_use]
    pub fn capture_names(&self) -> Vec<Option<&str>> {
        match self {
            Self::Fast(re) => re.capture_names().collect(),
            Self::Backtracking(re) => re.capture_names().collect(),
        }
    }

    /// Replace every match, leaving the input untouched when there are none.
    ///
    /// The replacement is read the way Java reads it, because that is what the
    /// vendor wrote it for: `$1` followed by another digit is group 1 and a
    /// LITERAL digit unless a group of the longer number exists. Rust takes the
    /// longest run of word characters as the name, so checkpoint's
    /// `$10$2:$3` over three groups asked for group 10, got nothing, and
    /// dropped the sign off every offset it normalised.
    #[must_use]
    pub fn replace_all<'t>(&self, text: &'t str, replacement: &str) -> std::borrow::Cow<'t, str> {
        let bounded = self.bind_replacement(replacement);
        match self {
            Self::Fast(re) => re.replace_all(text, bounded.as_ref()),
            Self::Backtracking(re) => re.replace_all(text, bounded.as_ref()),
        }
    }

    /// The replacement as Rust's engine has to read it to mean what Java's
    /// `Matcher.replaceAll` means by it.
    ///
    /// Two spellings differ. Java braces nothing, so a `$N` whose digits run
    /// past the groups the pattern has needs bracing here. And Java treats a
    /// backslash as an ESCAPE that yields the next character as itself, where
    /// Rust's engine has no escape at all and writes the backslash out -- so
    /// `cisco_ftd`'s `\\` collapsed two backslashes to two rather than to one and
    /// corrupted every `DOMAIN\user` it split.
    ///
    /// Borrows when neither applies, which is every replacement bar a handful
    /// in the whole catalogue.
    fn bind_replacement<'r>(&self, replacement: &'r str) -> std::borrow::Cow<'r, str> {
        let ambiguous = |bytes: &[u8], at: usize| {
            bytes.get(at + 1).is_some_and(u8::is_ascii_digit)
                && bytes.get(at + 2).is_some_and(|b| b.is_ascii_alphanumeric())
        };
        let bytes = replacement.as_bytes();
        if !replacement.contains('\\')
            && !replacement
                .match_indices('$')
                .any(|(at, _)| ambiguous(bytes, at))
        {
            return std::borrow::Cow::Borrowed(replacement);
        }

        let groups = self.capture_names().len();
        let mut out = String::with_capacity(replacement.len() + 8);
        let mut rest = replacement;
        while let Some(at) = rest.find(['$', '\\']) {
            out.push_str(&rest[..at]);
            let escape = rest.as_bytes()[at] == b'\\';
            rest = &rest[at + 1..];
            if escape {
                let mut escaped = rest.chars();
                match escaped.next() {
                    // An escaped `$` is text, and text is how Rust's engine
                    // reads a doubled one.
                    Some('$') => out.push_str("$$"),
                    Some(c) => out.push(c),
                    // Java throws on a trailing backslash; keeping it is the
                    // narrower answer.
                    None => out.push('\\'),
                }
                rest = escaped.as_str();
                continue;
            }
            let digits = rest
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(rest.len());
            if digits == 0 {
                out.push('$');
                continue;
            }
            // Java keeps taking digits while the number is still a group it
            // has; the rest of them are text.
            let mut take = 0;
            while take < digits
                && rest[..=take]
                    .parse::<usize>()
                    .is_ok_and(|number| number < groups)
            {
                take += 1;
            }
            let taken = take.max(1);
            out.push_str("${");
            out.push_str(&rest[..taken]);
            out.push('}');
            rest = &rest[taken..];
        }
        out.push_str(rest);
        std::borrow::Cow::Owned(out)
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

    // Compiled outside the write lock, so two threads racing the same pattern
    // both build one and the loser's copy is leaked. Bounded by the number of
    // distinct patterns times the threads that race them during warm-up, and
    // never on the steady-state path -- holding the lock across a compile would
    // park every transform thread instead.
    let compiled: &'static Pattern = Box::leak(Box::new(Pattern::compile(pattern, pattern)));

    if let Ok(mut guard) = PLAIN.write() {
        return guard
            .get_or_insert_with(HashMap::new)
            .entry(pattern.to_string())
            .or_insert(compiled);
    }
    compiled
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verbatim from `pipelines/cisco_ftd/default.yml`, which collapses a run
    /// of backslashes before splitting `DOMAIN\user`. Java reads the `\\` in
    /// the replacement as ONE backslash; writing both out left the run intact
    /// and the split saw a name it could not cut.
    #[test]
    fn a_backslash_in_the_replacement_escapes_the_character_after_it() {
        let pattern = Pattern::compile(r"\\{2,}", r"\\{2,}");
        assert_eq!(pattern.replace_all(r"CORP\\\user", r"\\"), r"CORP\user");

        // Verbatim from `ti_opencti_indicator`, which halves a doubled
        // registry separator.
        let doubled = Pattern::compile(r"\\\\", r"\\\\");
        assert_eq!(
            doubled.replace_all(r"HKLM\\SOFTWARE\\Bad", r"\\"),
            r"HKLM\SOFTWARE\Bad"
        );

        // Verbatim from `sentinel_one_cloud_funnel_event`, which halves four
        // to two.
        let quadrupled = Pattern::compile(r"\\\\\\\\", r"\\\\\\\\");
        assert_eq!(quadrupled.replace_all(r"a\\\\b", r"\\\\"), r"a\\b");
    }

    /// An escaped `$` is TEXT in Java, so it must not become a group
    /// reference once the replacement reaches Rust's engine.
    #[test]
    fn an_escaped_dollar_stays_text() {
        let pattern = Pattern::compile("(a)", "(a)");
        assert_eq!(pattern.replace_all("a", r"\$1"), "$1");
        // An unescaped one is still the group.
        assert_eq!(pattern.replace_all("a", "$1"), "a");
    }

    /// Java keeps taking digits while the number is still a group it has, so a
    /// `$10` over three groups is group 1 followed by a literal zero.
    ///
    /// Verbatim from the checkpoint offset normaliser, which asked for group
    /// 10, got nothing, and dropped the sign off every offset.
    #[test]
    fn a_group_number_binds_only_as_far_as_the_pattern_has_groups() {
        let pattern = Pattern::compile(r"([+-])(\d):(\d\d)", r"([+-])(\d):(\d\d)");
        assert_eq!(pattern.replace_all("-5:00", "$10$2:$3"), "-05:00");
    }

    /// A replacement with neither an escape nor an ambiguous group reference
    /// is handed to the engine as it arrived.
    #[test]
    fn an_ordinary_replacement_is_not_rebuilt() {
        let pattern = Pattern::compile("(a)(b)", "(a)(b)");
        assert!(matches!(
            pattern.bind_replacement("${1}_${2}"),
            std::borrow::Cow::Borrowed(_)
        ));
    }
}
