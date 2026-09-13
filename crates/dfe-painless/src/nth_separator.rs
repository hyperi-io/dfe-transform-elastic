// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The prefix that ends at the Nth separator counted from the END of a value.
//!
//! gigamon's DNS pipeline calls that prefix a subdomain and writes it with a
//! helper that walks `dns.question.name` backwards counting dots, returning
//! everything before the SECOND one -- `pipelines/gigamon/ami/default.yml:995`.
//! It is the only vendored pipeline that declares a `def subdomain(String s)`,
//! so the separator, the count, the source and the target are all read OFF the
//! script rather than fixed here.
//!
//! **The vendor's definition is not the obvious one, and the captures settle
//! it.** `_ipps._tcp.local` gives `_ipps`, `229.85.115.10.in-addr.arpa` gives
//! `229.85.115.10`, and `HCT_6011-181e00a30af6._tcn_eqaHCT._tcp.local` gives
//! `HCT_6011-181e00a30af6._tcn_eqaHCT` -- everything before the SECOND-LAST
//! dot, so a four-label name keeps two labels and not one
//! (`testdata/compat/gigamon/ami/test-ami/expected.ndjson`, 29 events).
//!
//! A value carrying fewer than `count` separators writes NOTHING: the helper
//! returns null there and the caller's `if (sub != null)` skips the write.
//!
//! Java's `charAt` and `substring` index UTF-16 code units where Rust indexes
//! bytes, and the two agree on this cut: both name the text before the same
//! occurrence of the same character, so the prefix is the same text either
//! way. The walk borrows from the event, and the write is the one allocation.

use serde_json::Value;

use crate::params::{balanced, ctx_path_plain as ctx_path, skip_trivia};
use dfe_core::Event;

/// One value cut at the `count`-th separator counted from its end, the part
/// BEFORE the cut landing on `target`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NthSeparatorPrefix {
    /// The `ctx.` path the value is read from.
    source: String,
    /// The `ctx.` path the prefix lands on.
    target: String,
    /// The character the helper counts.
    separator: char,
    /// Which occurrence, counted from the end, the cut is made at.
    count: usize,
}

impl NthSeparatorPrefix {
    /// Build one from resolved parts, or decline where the count cannot fire.
    ///
    /// The helper increments before it tests, so a count of zero is a test
    /// nothing ever satisfies -- the script would return null on every value.
    /// Declining keeps the runner's meaning to one reading.
    #[must_use]
    pub fn new(source: String, target: String, separator: char, count: usize) -> Option<Self> {
        (count >= 1).then_some(Self {
            source,
            target,
            separator,
            count,
        })
    }
}

/// Write the prefix the cut selects, or nothing where the value has too few
/// separators.
///
/// Returns FALSE where the source is absent or is not a string. Painless casts
/// the argument to `String` and throws on anything else, which no runner here
/// can reproduce, so the script is left counted as unhandled rather than
/// claimed and skipped.
pub fn nth_separator_prefix(event: &mut Event, pattern: &NthSeparatorPrefix) -> bool {
    let Some(subject) = event.get_str(&pattern.source) else {
        return false;
    };
    let Some(prefix) = prefix_before_nth_last(subject, pattern.separator, pattern.count) else {
        return true;
    };
    let written = prefix.to_owned();
    let _ = event.set(&pattern.target, Value::String(written));
    true
}

/// The text before the `count`-th `separator` counted from the end, or `None`
/// where `subject` holds fewer than `count` of them.
fn prefix_before_nth_last(subject: &str, separator: char, count: usize) -> Option<&str> {
    let mut seen = 0usize;
    for (at, c) in subject.char_indices().rev() {
        if c != separator {
            continue;
        }
        seen += 1;
        if seen == count {
            return Some(&subject[..at]);
        }
    }
    None
}

/// Read the backwards-counting cut off the script, or decline.
///
/// ```painless
/// def <helper>(String <s>) {
///   int <n>;
///   for (int <i> = <s>.length()-1; <i> >= 0; <i>--) {
///     if (<s>.charAt(<i>) == (char)'<separator>') {
///       <n>++;
///       if (<n> == <count>) {
///         return <s>.substring(0, <i>);
///       }
///     }
///   }
///   return null;
/// }
/// def <result> = <helper>(ctx.<source>);
/// if (<result> != null) {
///   ctx.<target> = <result>;
/// }
/// ```
///
/// Every statement has to be one of those, and anything else declines the
/// WHOLE script: a cut made at a different occurrence, or from the other end,
/// writes a value that is present and wrong, which reads as a source needing
/// polish rather than a different matcher.
#[must_use]
pub fn parse_nth_separator_prefix(script: &str) -> Option<NthSeparatorPrefix> {
    let rest = skip_trivia(skip_trivia(script).strip_prefix("def")?);
    let opens = rest.find('(')?;
    let helper = identifier(&rest[..opens])?;
    let (parameter, rest) = balanced(&rest[opens..], '(', ')')?;
    let subject = declared(parameter, "String")?;
    let (helper_body, rest) = balanced(skip_trivia(rest), '{', '}')?;

    let (separator, count) = parse_helper(helper_body, subject)?;
    let (source, target) = parse_call(rest, helper)?;
    NthSeparatorPrefix::new(source, target, separator, count)
}

/// The helper's body: the separator it counts and the occurrence it stops at.
fn parse_helper(body: &str, subject: &str) -> Option<(char, usize)> {
    let (declaration, rest) = skip_trivia(body).split_once(';')?;
    let counter = declared(declaration, "int")?;

    let rest = skip_trivia(rest).strip_prefix("for")?;
    let (header, rest) = balanced(skip_trivia(rest), '(', ')')?;
    let mut clauses = header.splitn(3, ';');
    let (declaration, start) = clauses.next()?.split_once('=')?;
    let index = declared(declaration, "int")?;
    // The walk has to start at the last character and step down to the first,
    // because that is what makes the count run from the END.
    if !same_ignoring_spaces(start, &format!("{subject}.length()-1"))
        || !same_ignoring_spaces(clauses.next()?, &format!("{index}>=0"))
        || !same_ignoring_spaces(clauses.next()?, &format!("{index}--"))
    {
        return None;
    }

    let (loop_body, rest) = balanced(skip_trivia(rest), '{', '}')?;
    // A helper that returns anything but null when the count is never reached
    // writes on values this matcher would leave alone.
    if !same_ignoring_spaces(rest, "return null;") {
        return None;
    }

    let rest = skip_trivia(loop_body).strip_prefix("if")?;
    let (test, rest) = balanced(skip_trivia(rest), '(', ')')?;
    let separator = separator_tested(test, subject, index)?;
    let (found, rest) = balanced(skip_trivia(rest), '{', '}')?;
    if !skip_trivia(rest).is_empty() {
        return None;
    }

    let count = parse_count(found, subject, counter, index)?;
    Some((separator, count))
}

/// The counted branch: the increment, the occurrence it tests for, and the cut.
fn parse_count(block: &str, subject: &str, counter: &str, index: &str) -> Option<usize> {
    let (statement, rest) = skip_trivia(block).split_once(';')?;
    if !same_ignoring_spaces(statement, &format!("{counter}++")) {
        return None;
    }

    let rest = skip_trivia(rest).strip_prefix("if")?;
    let (test, rest) = balanced(skip_trivia(rest), '(', ')')?;
    let (tested, wanted) = test.split_once("==")?;
    if tested.trim() != counter {
        return None;
    }
    let count = wanted.trim().parse().ok()?;

    let (cut, rest) = balanced(skip_trivia(rest), '{', '}')?;
    if !skip_trivia(rest).is_empty()
        || !same_ignoring_spaces(cut, &format!("return {subject}.substring(0, {index});"))
    {
        return None;
    }
    Some(count)
}

/// The call and the guarded write after the helper: the source and the target.
fn parse_call(script: &str, helper: &str) -> Option<(String, String)> {
    let (statement, rest) = skip_trivia(script).split_once(';')?;
    let (declaration, call) = statement.split_once('=')?;
    let result = declared(declaration, "def")?;
    let call = skip_trivia(call).strip_prefix(helper)?;
    let (argument, after) = balanced(skip_trivia(call), '(', ')')?;
    if !skip_trivia(after).is_empty() {
        return None;
    }
    let source = ctx_path(argument)?;

    let rest = skip_trivia(rest).strip_prefix("if")?;
    let (test, rest) = balanced(skip_trivia(rest), '(', ')')?;
    if !same_ignoring_spaces(test, &format!("{result} != null")) {
        return None;
    }
    let (block, rest) = balanced(skip_trivia(rest), '{', '}')?;
    if !skip_trivia(rest).is_empty() {
        return None;
    }

    let (statement, tail) = skip_trivia(block).split_once(';')?;
    if !skip_trivia(tail).is_empty() {
        return None;
    }
    let (target, written) = statement.split_once('=')?;
    if written.trim() != result {
        return None;
    }
    Some((source, ctx_path(target)?))
}

/// The character an `<s>.charAt(<i>) == (char)'<c>'` test compares against.
fn separator_tested(test: &str, subject: &str, index: &str) -> Option<char> {
    let (read, literal) = test.split_once("==")?;
    if !same_ignoring_spaces(read, &format!("{subject}.charAt({index})")) {
        return None;
    }
    let quoted = skip_trivia(literal).strip_prefix("(char)")?.trim();
    let inner = quoted.strip_prefix('\'')?.strip_suffix('\'')?;
    // Exactly one character, so an escape such as `'\\'` -- which arrives as
    // two -- declines rather than being read as its first half.
    let mut chars = inner.chars();
    let separator = chars.next()?;
    chars.next().is_none().then_some(separator)
}

/// The name a `<keyword> <name>` declaration binds.
fn declared<'a>(text: &'a str, keyword: &str) -> Option<&'a str> {
    let rest = text.trim().strip_prefix(keyword)?;
    if !rest.starts_with(char::is_whitespace) {
        return None;
    }
    identifier(rest)
}

/// A bare `ctx.<path>`, with null-safe navigation stripped.
/// A local's name, or `None` where the text is not one identifier.
fn identifier(text: &str) -> Option<&str> {
    let name = text.trim();
    (!name.is_empty()
        && !name.starts_with(|c: char| c.is_ascii_digit())
        && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
    .then_some(name)
}

/// Whether two expressions read the same once every space is removed, so
/// spacing is not part of a match.
fn same_ignoring_spaces(text: &str, expected: &str) -> bool {
    text.chars()
        .filter(|c| !c.is_whitespace())
        .eq(expected.chars().filter(|c| !c.is_whitespace()))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use serde_json::json;

    use super::*;

    /// gigamon's subdomain helper, verbatim from
    /// `pipelines/gigamon/ami/default.yml:995` with the YAML fold applied.
    fn script() -> &'static str {
        "def subdomain(String s) {\n  int n;\n  for (int i = s.length()-1; i >= 0; i--) {\n    \
         if (s.charAt(i) == (char)'.') {\n      n++;\n      if (n == 2) {\n        \
         return s.substring(0, i);\n      }\n    }\n  }\n  return null;\n} \
         def sub = subdomain(ctx.dns.question.name); if (sub != null) {\n  \
         ctx.dns.question.subdomain = sub;\n}"
    }

    fn pattern() -> NthSeparatorPrefix {
        parse_nth_separator_prefix(script()).expect("the helper is recognised")
    }

    fn cut(name: &str) -> Option<Value> {
        let mut event = Event::new(json!({ "dns": { "question": { "name": name } } }));
        assert!(nth_separator_prefix(&mut event, &pattern()));
        event.get("dns.question.subdomain").cloned()
    }

    /// Every part comes OFF the script -- the separator, the occurrence, and
    /// both paths.
    #[test]
    fn every_part_is_read_off_the_script() {
        assert_eq!(
            pattern(),
            NthSeparatorPrefix {
                source: "dns.question.name".into(),
                target: "dns.question.subdomain".into(),
                separator: '.',
                count: 2,
            }
        );
    }

    /// The captured pairs, which are what settle the vendor's definition: the
    /// cut is at the SECOND-LAST dot, so a four-label name keeps two labels.
    #[test]
    fn the_cut_is_at_the_second_last_separator() {
        assert_eq!(cut("_ipps._tcp.local"), Some(json!("_ipps")));
        assert_eq!(
            cut("229.85.115.10.in-addr.arpa"),
            Some(json!("229.85.115.10"))
        );
        assert_eq!(
            cut("HCT_6011-181e00a30af6._tcn_eqaHCT._tcp.local"),
            Some(json!("HCT_6011-181e00a30af6._tcn_eqaHCT"))
        );
    }

    /// Fewer separators than the count writes NOTHING -- the helper returns
    /// null and the caller's guard skips the write.
    #[test]
    fn too_few_separators_write_nothing() {
        assert_eq!(cut("example.com"), None);
        assert_eq!(cut("localhost"), None);
        assert_eq!(cut(""), None);
    }

    /// A cut at offset zero writes an EMPTY string, which is Painless's own
    /// answer: `substring(0, 0)` is `""`, and `"" != null` holds. No captured
    /// name opens with a dot; the case is held so the reading is not lost.
    #[test]
    fn a_cut_at_the_front_writes_an_empty_string() {
        assert_eq!(cut(".tcp.local"), Some(json!("")));
    }

    /// A source that is not a string DECLINES rather than claiming the script:
    /// Painless casts it to `String` and throws, which nothing here reproduces.
    #[test]
    fn a_non_string_source_declines() {
        let mut event = Event::new(json!({ "dns": { "question": { "name": 4 } } }));
        assert!(!nth_separator_prefix(&mut event, &pattern()));
        assert!(!event.has("dns.question.subdomain"));

        let mut absent = Event::new(json!({ "dns": { "question": {} } }));
        assert!(!nth_separator_prefix(&mut absent, &pattern()));
        assert!(!absent.has("dns.question.subdomain"));
    }

    /// The occurrence is the script's number, not a constant here.
    #[test]
    fn the_count_comes_from_the_script() {
        let third = script().replace("if (n == 2)", "if (n == 3)");
        assert_ne!(third, script(), "the replacement has to bite");
        let pattern = parse_nth_separator_prefix(&third).expect("recognised");
        assert_eq!(pattern.count, 3);

        let mut event = Event::new(json!({
            "dns": { "question": { "name": "a.b.c.example.com" } }
        }));
        assert!(nth_separator_prefix(&mut event, &pattern));
        assert_eq!(event.get("dns.question.subdomain"), Some(&json!("a.b")));
    }

    /// The separator is the script's character, not a dot by assumption.
    #[test]
    fn the_separator_comes_from_the_script() {
        let dashes = script().replace("(char)'.'", "(char)'-'");
        assert_ne!(dashes, script(), "the replacement has to bite");
        let pattern = parse_nth_separator_prefix(&dashes).expect("recognised");
        assert_eq!(pattern.separator, '-');
    }

    /// One statement the reader cannot place declines the WHOLE script, so a
    /// near-miss never writes a value that is present and wrong.
    #[test]
    fn one_unreadable_statement_declines_the_whole_script() {
        for broken in [
            // The cut taken from the other end.
            script().replace("s.substring(0, i)", "s.substring(i + 1)"),
            // The walk started at the front, so the count would run forwards.
            script().replace("s.length()-1", "0"),
            // A count that no increment ever reaches.
            script().replace("if (n == 2)", "if (n == 0)"),
            // A guard over something other than the helper's own answer.
            script().replace("if (sub != null)", "if (ctx.other != null)"),
            // The answer transformed on its way to the target.
            script().replace("subdomain = sub;", "subdomain = sub.trim();"),
            // A second statement under the guard.
            script().replace(
                "ctx.dns.question.subdomain = sub;",
                "ctx.dns.question.subdomain = sub; ctx.a = sub;",
            ),
        ] {
            assert_ne!(broken, script(), "each replacement has to bite");
            assert!(
                parse_nth_separator_prefix(&broken).is_none(),
                "read a script it should have declined: {broken}"
            );
        }
    }

    /// The dispatch reaches this matcher: nothing above it in the text ladder
    /// claims the script, and nothing else is bound beside it.
    #[test]
    fn the_ladder_dispatches_the_script_here() {
        let bound = crate::common::known_patterns(script());
        assert_eq!(
            bound.len(),
            1,
            "the ladder bound {bound:?} instead of this matcher alone"
        );
    }

    /// The production path, end to end. A generated call site hands
    /// `PainlessPlan` the script with its newlines ESCAPED, so a matcher that
    /// works only on real newlines passes its own tests and does nothing in
    /// the service. This is the literal
    /// `crates/dfe-transforms/src/filebeat/gigamon_ami/default.rs:2886` holds.
    #[test]
    fn the_escaped_call_site_literal_runs() {
        // Running a plan records into the process-global stats the counting
        // tests read; one lock keeps those honest.
        let _guard = crate::stats::serialised();
        let plan = crate::plan::PainlessPlan::new(
            r"def subdomain(String s) {\n  int n;\n  for (int i = s.length()-1; i >= 0; i--) {\n    if (s.charAt(i) == (char)'.') {\n      n++;\n      if (n == 2) {\n        return s.substring(0, i);\n      }\n    }\n  }\n  return null;\n} def sub = subdomain(ctx.dns.question.name); if (sub != null) {\n  ctx.dns.question.subdomain = sub;\n}",
        );
        assert!(plan.matches(), "the call site's own literal binds nothing");

        let mut event = Event::new(json!({
            "dns": { "question": { "name": "_webdav._tcp.local" } }
        }));
        crate::plan::painless_exec_plan(&mut event, &plan).expect("the plan runs");
        assert_eq!(event.get("dns.question.subdomain"), Some(&json!("_webdav")));
    }
}
