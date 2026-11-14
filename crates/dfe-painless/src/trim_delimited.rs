// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Several fields unwrapped from a surrounding delimiter the params table
//! carries, through a helper the script declares once.
//!
//! `sentinel_one_cloud_funnel` sends both command lines wrapped in quotes and
//! strips them here, on every one of its fourteen streams.
//!
//! Same intent as [`crate::common`]'s `StripSurroundingPair`, three spellings
//! apart: the delimiter is a params VALUE rather than a literal, the cut lives
//! in a helper rather than at the field, and one script does several fields.
//! Neither reader can be widened into the other without claiming what it cannot
//! write, so this is a second reader over one intent.
//!
//! `IndexedLookup` claimed it on `.put(` plus `params` with no parse behind
//! them, and its runner then declined because the table is a map rather than an
//! array. The quotes therefore survived into `process.command_line` and
//! `...event.src.process.parent.cmd_line`, which is eight of this source's
//! eighteen events.

use serde_json::{Map, Value};

use crate::group_records::local_ctx_path;
use crate::params::{balanced, ctx_path_term};
use dfe_core::Event;

/// One field read, trimmed, and written back.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TrimField {
    /// The `ctx` path the local is read from.
    source: String,
    /// The `ctx` path the trimmed value is written to.
    target: String,
}

/// Several fields unwrapped from one params-carried delimiter.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TrimDelimited {
    /// The params key holding the delimiter both ends must carry.
    delimiter: String,
    /// The fields trimmed, in the order the script writes them.
    fields: Vec<TrimField>,
}

/// Read the helper and every field it is applied to, or decline.
///
/// The helper's body is checked in full -- both ends tested against the SAME
/// argument, and a cut of exactly one character off each end. A script cutting
/// a different width means different characters, and reading it as this one
/// would truncate every value it touches.
#[must_use]
pub fn parse_trim_delimited(script: &str) -> Option<TrimDelimited> {
    // Whitespace-free, so the structural tests read one spelling rather than
    // every way the vendor's formatter could lay the same statement out. Once
    // per call site, never per event.
    let compact: String = script.chars().filter(|c| !c.is_whitespace()).collect();

    // `<Type> <name>(def <delimiter>, def <value>) {` -- the declaration has to
    // be the first thing in the script, because a call would be read as one.
    let open = script.find('(')?;
    let name = script[..open].split_whitespace().next_back()?;
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    let (arguments, _) = balanced(&script[open..], '(', ')')?;
    let mut arguments = arguments.split(',');
    let delimiter_param = arguments.next()?.split_whitespace().next_back()?;
    let value_param = arguments.next()?.split_whitespace().next_back()?;
    if arguments.next().is_some() || delimiter_param == value_param {
        return None;
    }

    // The body: both ends carry the delimiter, and the cut is exactly one
    // character off each.
    let guard = format!(
        "{value_param}.startsWith({delimiter_param})&&{value_param}.endsWith({delimiter_param})"
    );
    let cut = format!("{value_param}={value_param}.substring(1,{value_param}.length()-1)");
    if !compact.contains(&guard)
        || !compact.contains(&cut)
        || !compact.contains(&format!("return{value_param};"))
    {
        return None;
    }

    // Every `<local> = <name>(params.<key>, <local>);` call, and the field it
    // reads and writes.
    let call = format!("={name}(params.");
    let mut delimiter: Option<String> = None;
    let mut fields = Vec::new();
    for (at, _) in compact.match_indices(&call) {
        let assigned = trailing_name(&compact[..at])?;
        let (key, rest) = compact[at + call.len()..].split_once(',')?;
        let argument = rest.split(')').next()?;
        // The helper returns its argument, so a call assigning one local the
        // trim of ANOTHER is moving a value as well as trimming it.
        if argument != assigned || key.is_empty() {
            return None;
        }
        // One delimiter for the whole script: two would be two intents sharing
        // a helper, and this carries one key.
        if delimiter.get_or_insert_with(|| key.to_owned()).as_str() != key {
            return None;
        }

        // Not `crate::params::ctx_locals`: it splits on `;` and demands the
        // piece start with `def `, and the helper's closing brace is glued to
        // the first declaration by that split -- `}\ndef v1 = ctx...`.
        fields.push(TrimField {
            source: local_ctx_path(script, assigned)?,
            target: put_target(&compact, assigned)?,
        });
    }

    // A script whose helper is never called trims nothing, and `delimiter` is
    // set by the same loop that fills `fields`.
    Some(TrimDelimited {
        delimiter: delimiter?,
        fields,
    })
}

/// Where `ctx.<container>.put("<key>", <local>)` stores the local.
///
/// The container and the key are read together, because the script names the
/// destination in two halves and neither half alone is a path.
fn put_target(compact: &str, local: &str) -> Option<String> {
    let tail = format!(",{local}");
    for (at, _) in compact.match_indices(".put(") {
        let (arguments, _) = balanced(&compact[at + ".put(".len() - 1..], '(', ')')?;
        if !arguments.ends_with(&tail) {
            continue;
        }
        let key = arguments.split(',').next()?;
        let quote = key.chars().next()?;
        if quote != '\'' && quote != '"' {
            return None;
        }
        let key = key.strip_prefix(quote)?.strip_suffix(quote)?;
        let container = ctx_path_term(compact[..at].rsplit([';', '{', '}']).next()?)?;
        return (!key.is_empty()).then(|| format!("{container}.{key}"));
    }
    None
}

/// The identifier immediately before an `=`, or `None` where the assignment
/// target is not a bare local.
fn trailing_name(before: &str) -> Option<&str> {
    let name = before.rsplit([';', '{', '}', ')']).next()?;
    (!name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
        .then_some(name)
}

/// Trim the delimiter off every field that carries it at both ends.
///
/// The helper's `substring` cuts one character off each end whatever the
/// delimiter's length, so a delimiter longer than one character means the
/// script's guard and its cut disagree; this declines rather than write a value
/// neither engine produces.
#[must_use]
pub fn trim_delimited(
    event: &mut Event,
    pattern: &TrimDelimited,
    params: &Map<String, Value>,
) -> bool {
    let Some(delimiter) = params
        .get(&pattern.delimiter)
        .and_then(Value::as_str)
        .and_then(|text| {
            let mut chars = text.chars();
            let first = chars.next()?;
            chars.next().is_none().then_some(first)
        })
    else {
        return false;
    };

    for field in &pattern.fields {
        // Absent, or not a string: the script's own `instanceof String` guard,
        // which leaves the field exactly as it found it.
        let Some(text) = event.get_str(&field.source) else {
            continue;
        };
        let trimmed = if text.chars().count() >= 2
            && text.starts_with(delimiter)
            && text.ends_with(delimiter)
        {
            text[delimiter.len_utf8()..text.len() - delimiter.len_utf8()].to_owned()
        } else if field.target == field.source {
            // A value carrying no pair still goes through the helper, which
            // returns it unchanged -- so the write happens and, where it lands
            // back on the field it came from, the document does not move.
            continue;
        } else {
            text.to_owned()
        };
        let _ = event.set(&field.target, Value::String(trimmed));
    }
    true
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#`.
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
#[path = "trim_delimited_tests.rs"]
mod tests;
