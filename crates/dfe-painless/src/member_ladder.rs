// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A ladder banding ONE member of every element of a list, the last match won.
//!
//! `google_secops` reads the severity word off each security result and writes a
//! numeric `event.severity` for it:
//!
//! ```painless
//! if (ctx.google_secops?.alert_v2?.event?.security_result instanceof List) {
//!   for (list in ctx.google_secops?.alert_v2?.event.security_result) {
//!     if (list["severity"] != null && list["severity"] != '') {
//!       if (list["severity"].equalsIgnoreCase('critical')) { ctx.event.severity = 99 }
//!       else if (list["severity"].equalsIgnoreCase('high')) { ctx.event.severity = 73 }
//!       ...
//!     }
//!   }
//! }
//! ```
//!
//! Its `alert` stream writes the same ladder as `.toUpperCase() == 'CRITICAL'`
//! against an upper-case literal, so both calls are read here.
//!
//! **The write is an ASSIGNMENT inside the loop, so the LAST element that
//! matches an arm decides the value** -- not the first, and not the highest
//! band. An element matching no arm leaves whatever an earlier one wrote.
//!
//! Distinct from [`crate::common::KnownPattern::CaseInsensitiveLadder`], which
//! takes a SCALAR subject read straight off the document. That reader declines
//! a subject reached by subscript on a loop variable, and its arm returns the
//! empty plan, so `alert_v2` was claimed and served by nothing until this
//! pattern was placed ahead of it.

use dfe_core::Event;
use serde_json::Value;

use crate::common::{for_each_in, if_block, painless_literal};
use crate::params::clean_path;

/// The list walked, the member banded, and the bands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberLadder {
    /// The document path of the list.
    list: String,
    /// The key read off each element.
    key: String,
    /// The field every arm writes.
    target: String,
    /// `(word, value)` in the order the script tests them, words folded.
    bands: Vec<(String, Value)>,
}

/// Read the loop, the banded member and its bands, or decline.
///
/// A WHOLE-SCRIPT match: the optional `instanceof List` guard, one loop, an
/// optional presence guard on the member, and a ladder whose every arm writes
/// the same field from a literal. Anything else declines, because a ladder
/// sitting beside other work is a script this reader has not read.
pub fn parse_member_ladder(script: &str) -> Option<MemberLadder> {
    let script = script.trim();
    // The `instanceof List` gate is what the loop needs anyway: a field that is
    // not a list iterates nothing here, the same answer Painless reaches.
    let looped = match if_block(script) {
        Some((_, body, after)) if after.trim().is_empty() => body,
        _ => script,
    };
    let (item, list, body) = for_each_in(looped)?;

    // `if (x["k"] != null && x["k"] != '')` guards the ladder in both spellings
    // and tests exactly what the arms read, so it is stepped over rather than
    // reproduced. A guard doing anything else SELECTS which elements are
    // banded, which this reader does not reproduce.
    let ladder = match if_block(body.trim()) {
        Some((condition, inner, after)) if after.trim().is_empty() => {
            if !presence_guard(condition, &item) {
                return None;
            }
            inner.trim()
        }
        _ => body.trim(),
    };

    let mut key: Option<String> = None;
    let mut target: Option<String> = None;
    let mut bands = Vec::new();
    let mut chain = ladder;

    loop {
        let (condition, arm, tail) = if_block(chain)?;
        let (named, word) = banded(condition.trim(), &item)?;
        if key.get_or_insert_with(|| named.clone()) != &named {
            return None;
        }
        let (path, value) = written(arm)?;
        if target.get_or_insert_with(|| path.clone()) != &path {
            return None;
        }
        bands.push((word, value));

        let tail = tail.trim_start();
        let Some(next) = tail.strip_prefix("else") else {
            // Text after the last arm is work this reader has not read.
            if !tail.is_empty() {
                return None;
            }
            break;
        };
        // A closing `else` bands every word the ladder does not name, and this
        // reader resolves a band from a named word alone.
        if next.trim_start().starts_with('{') {
            return None;
        }
        chain = next.trim_start();
    }

    let (key, target) = (key?, target?);
    // One band is an `if`, not a ladder, and every reader of a lone guarded
    // write has a better claim on that.
    (bands.len() > 1).then_some(MemberLadder {
        list,
        key,
        target,
        bands,
    })
}

/// Whether a guard tests only that the member the arms read is present.
fn presence_guard(condition: &str, item: &str) -> bool {
    condition.split("&&").all(|term| {
        term.split_once("!=").is_some_and(|(subject, wanted)| {
            subject.trim().starts_with(item) && matches!(wanted.trim(), "null" | "''" | "\"\"")
        })
    })
}

/// Band every element in turn, so the last one that matches decides the value.
pub fn member_ladder(event: &mut Event, pattern: &MemberLadder) -> bool {
    let Some(Value::Array(elements)) = event.get(&pattern.list) else {
        return true;
    };

    let mut written: Option<Value> = None;
    for element in elements {
        let Some(held) = element.get(&pattern.key).and_then(Value::as_str) else {
            continue;
        };
        if held.is_empty() {
            continue;
        }
        let folded = held.to_lowercase();
        if let Some((_, value)) = pattern.bands.iter().find(|(word, _)| word == &folded) {
            written = Some(value.clone());
        }
    }

    if let Some(value) = written {
        let _ = event.set(&pattern.target, value);
    }
    true
}

/// `<item>["<key>"].equalsIgnoreCase('<word>')` or the `toUpperCase() ==`
/// spelling, as the key and the word FOLDED.
///
/// Both calls compare without case, so the word is folded once here and the
/// runner folds the value it reads.
fn banded(condition: &str, item: &str) -> Option<(String, String)> {
    for (call, closes) in [(".equalsIgnoreCase(", true), (".toUpperCase() ==", false)] {
        let Some((subject, argument)) = condition.split_once(call) else {
            continue;
        };
        let key = subscript(subject.trim(), item)?;
        let argument = argument.trim();
        let argument = if closes {
            argument.strip_suffix(')')?
        } else {
            argument
        };
        let quote = argument
            .chars()
            .next()
            .filter(|c| *c == '\'' || *c == '"')?;
        let word = argument.strip_prefix(quote)?.strip_suffix(quote)?;
        if word.is_empty() || word.contains(quote) {
            return None;
        }
        return Some((key, word.to_lowercase()));
    }
    None
}

/// The key in `<item>["<key>"]`, where the receiver is the loop variable.
///
/// A receiver that is anything else is a different pattern: the scalar ladder
/// reads a bare local, and a `ctx.` path there is a field, not a member.
fn subscript(subject: &str, item: &str) -> Option<String> {
    let inside = subject.strip_prefix(item)?.trim().strip_prefix('[')?;
    let quote = inside.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    let (key, rest) = inside.strip_prefix(quote)?.split_once(quote)?;
    (!key.is_empty() && rest.trim() == "]").then(|| key.to_owned())
}

/// The one `ctx.<path> = <literal>` an arm's body must be.
fn written(arm: &str) -> Option<(String, Value)> {
    let statement = arm.trim();
    // Painless takes a newline for a statement end where the vendor left the
    // semicolon off, which these arms do.
    let statement = statement.strip_suffix(';').unwrap_or(statement).trim();
    if statement.contains(';') || statement.contains('\n') {
        return None;
    }
    let (target, value) = statement.split_once('=')?;
    let path = clean_path(target.trim().strip_prefix("ctx.")?.trim());
    if path.is_empty() || path.contains(char::is_whitespace) {
        return None;
    }
    Some((path, painless_literal(value.trim())?))
}

#[cfg(test)]
#[path = "member_ladder_tests.rs"]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
mod tests;
