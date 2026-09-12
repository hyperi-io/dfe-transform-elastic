// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Several fields unwrapped from a surrounding delimiter the params table
//! carries, through a helper the script declares once.
//!
//! `sentinel_one_cloud_funnel` sends both command lines wrapped in quotes and
//! strips them here, on every one of its fourteen streams:
//!
//! ```painless
//! String trimQuotes(def doubleQuote, def v) {
//!   if (v.startsWith(doubleQuote) && v.endsWith(doubleQuote)) {
//!     v = v.substring(1, v.length() - 1);
//!   }
//!   return v;
//! }
//! def v1 = ctx.json?.src?.process?.parent?.cmdline;
//! def v2 = ctx.json?.tgt?.process?.cmdline;
//! if (v1 instanceof String && v1 != null) {
//!   v1 = trimQuotes(params.double_quote, v1);
//!   ctx.json.src.process.parent.put("cmdline", v1);
//! }
//! if (v2 instanceof String && v2 != null) {
//!   v2 = trimQuotes(params.double_quote, v2);
//!   ctx.json.tgt.process.put("cmdline", v2);
//! }
//! ```
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrimField {
    /// The `ctx` path the local is read from.
    source: String,
    /// The `ctx` path the trimmed value is written to.
    target: String,
}

/// Several fields unwrapped from one params-carried delimiter.
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#`.
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Verbatim from the generated call site in
    /// `crates/dfe-transforms/src/filebeat/sentinel_one_cloud_funnel_event/default.rs`,
    /// in the escaped one-line form a stored script reaches it in.
    const SENTINEL_ONE_TRIM_QUOTES: &str = r#"String trimQuotes(def doubleQuote, def v) {\n  if (v.startsWith(doubleQuote) && v.endsWith(doubleQuote)) {\n    v = v.substring(1, v.length() - 1);\n  }\n  return v;\n}\ndef v1 = ctx.json?.src?.process?.parent?.cmdline;\ndef v2 = ctx.json?.tgt?.process?.cmdline;\nif (v1 instanceof String && v1 != null) {\n  v1 = trimQuotes(params.double_quote, v1);\n  ctx.json.src.process.parent.put(\"cmdline\", v1);\n}\nif (v2 instanceof String && v2 != null) {\n  v2 = trimQuotes(params.double_quote, v2);\n  ctx.json.tgt.process.put(\"cmdline\", v2);\n}\n"#;

    fn double_quote() -> Map<String, Value> {
        let Value::Object(params) = json!({ "double_quote": "\"" }) else {
            unreachable!("the literal is an object")
        };
        params
    }

    #[test]
    fn the_sentinel_one_helper_names_both_command_lines() {
        assert_eq!(
            parse_trim_delimited(&crate::common::normalise(SENTINEL_ONE_TRIM_QUOTES)),
            Some(TrimDelimited {
                delimiter: "double_quote".to_owned(),
                fields: vec![
                    TrimField {
                        source: "json.src.process.parent.cmdline".to_owned(),
                        target: "json.src.process.parent.cmdline".to_owned(),
                    },
                    TrimField {
                        source: "json.tgt.process.cmdline".to_owned(),
                        target: "json.tgt.process.cmdline".to_owned(),
                    },
                ],
            })
        );
    }

    #[test]
    fn a_quoted_command_line_loses_one_quote_from_each_end() {
        let pattern =
            parse_trim_delimited(&crate::common::normalise(SENTINEL_ONE_TRIM_QUOTES)).unwrap();
        let mut event = Event::new(json!({ "json": { "src": { "process": { "parent": {
            "cmdline": "\"C:\\ProgramFiles\\Google\\chrome.exe\""
        }}}}}));
        assert!(trim_delimited(&mut event, &pattern, &double_quote()));
        assert_eq!(
            event.get_str("json.src.process.parent.cmdline"),
            Some("C:\\ProgramFiles\\Google\\chrome.exe")
        );
    }

    /// An unquoted value goes through the helper unchanged, so nothing moves.
    #[test]
    fn an_unquoted_command_line_is_left_alone() {
        let pattern =
            parse_trim_delimited(&crate::common::normalise(SENTINEL_ONE_TRIM_QUOTES)).unwrap();
        let mut event = Event::new(json!({ "json": { "tgt": { "process": {
            "cmdline": "chrome.exe --headless"
        }}}}));
        assert!(trim_delimited(&mut event, &pattern, &double_quote()));
        assert_eq!(
            event.get_str("json.tgt.process.cmdline"),
            Some("chrome.exe --headless")
        );
    }

    /// Only the OUTER pair goes: a command line whose second argument is a
    /// quoted JSON blob keeps every quote inside it.
    #[test]
    fn only_the_outer_pair_is_cut() {
        let pattern =
            parse_trim_delimited(&crate::common::normalise(SENTINEL_ONE_TRIM_QUOTES)).unwrap();
        let mut event = Event::new(json!({ "json": { "src": { "process": { "parent": {
            "cmdline": "\"C:\\ranger\\SentinelRanger.exe\"\"{\"uuid\":\"asdf1234\"}\""
        }}}}}));
        assert!(trim_delimited(&mut event, &pattern, &double_quote()));
        assert_eq!(
            event.get_str("json.src.process.parent.cmdline"),
            Some("C:\\ranger\\SentinelRanger.exe\"\"{\"uuid\":\"asdf1234\"}")
        );
    }

    /// A field the event does not carry is the script's own `instanceof String`
    /// guard, and the other field is still written.
    #[test]
    fn an_absent_field_is_skipped_and_the_other_still_runs() {
        let pattern =
            parse_trim_delimited(&crate::common::normalise(SENTINEL_ONE_TRIM_QUOTES)).unwrap();
        let mut event = Event::new(json!({ "json": { "tgt": { "process": {
            "cmdline": "\"notepad.exe\""
        }}}}));
        assert!(trim_delimited(&mut event, &pattern, &double_quote()));
        assert_eq!(event.get("json.src"), None);
        assert_eq!(
            event.get_str("json.tgt.process.cmdline"),
            Some("notepad.exe")
        );
    }

    /// A non-string is not trimmed and not stringified either.
    #[test]
    fn a_numeric_field_is_left_alone() {
        let pattern =
            parse_trim_delimited(&crate::common::normalise(SENTINEL_ONE_TRIM_QUOTES)).unwrap();
        let mut event = Event::new(json!({ "json": { "tgt": { "process": { "cmdline": 42 }}}}));
        assert!(trim_delimited(&mut event, &pattern, &double_quote()));
        assert_eq!(event.get("json.tgt.process.cmdline"), Some(&json!(42)));
    }

    /// A delimiter of more than one character makes the script's guard and its
    /// one-character cut disagree, so nothing is written.
    #[test]
    fn a_multi_character_delimiter_is_declined() {
        let pattern =
            parse_trim_delimited(&crate::common::normalise(SENTINEL_ONE_TRIM_QUOTES)).unwrap();
        let Value::Object(params) = json!({ "double_quote": "''" }) else {
            unreachable!("the literal is an object")
        };
        let mut event = Event::new(json!({ "json": { "tgt": { "process": {
            "cmdline": "''notepad.exe''"
        }}}}));
        assert!(!trim_delimited(&mut event, &pattern, &params));
        assert_eq!(
            event.get_str("json.tgt.process.cmdline"),
            Some("''notepad.exe''")
        );
    }

    /// A cut of a different width takes different characters, so the helper is
    /// not this one.
    #[test]
    fn a_helper_cutting_a_different_width_is_declined() {
        let script = r#"String trim(def d, def v) {\n  if (v.startsWith(d) && v.endsWith(d)) {\n    v = v.substring(2, v.length() - 2);\n  }\n  return v;\n}\ndef v1 = ctx.a.b;\nif (v1 instanceof String) {\n  v1 = trim(params.pair, v1);\n  ctx.a.put(\"b\", v1);\n}\n"#;
        assert!(parse_trim_delimited(&crate::common::normalise(script)).is_none());
    }

    /// A helper testing one end against a different argument is a prefix strip,
    /// not an unwrap.
    #[test]
    fn a_helper_testing_two_different_ends_is_declined() {
        let script = r#"String trim(def open, def close, def v) {\n  if (v.startsWith(open) && v.endsWith(close)) {\n    v = v.substring(1, v.length() - 1);\n  }\n  return v;\n}\ndef v1 = ctx.a.b;\nv1 = trim(params.open, params.close, v1);\nctx.a.put(\"b\", v1);\n"#;
        assert!(parse_trim_delimited(&crate::common::normalise(script)).is_none());
    }
}
