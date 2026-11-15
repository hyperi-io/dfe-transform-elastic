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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "nth_separator_tests.rs"]
mod tests;
