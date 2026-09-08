// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One field decided by a truth table over TWO others.
//!
//! zeek reports a connection's locality as two booleans and ECS wants one word,
//! so the pipeline writes the four-row table out longhand -- a null guard, then
//! one `if` per row, each returning once it has written:
//!
//! ```painless
//! if (ctx.zeek?.connection?.local_orig == null ||
//!     ctx.zeek?.connection?.local_resp == null) {
//!   return;
//! }
//! if (ctx.zeek.connection.local_orig == true &&
//!     ctx.zeek.connection.local_resp == true) {
//!   ctx.network.direction = "internal";
//!   return;
//! }
//! ...
//! ```
//!
//! The rows are sequential `if`s rather than an `else if` chain, so
//! `EqualityLadder` -- which is gated on `else if (` and reads ONE subject --
//! cannot see them. Unclaimed the script costs zeek `network.direction` on all
//! 18 events of its connection stream.

use serde_json::Value;

use crate::Event;
use crate::painless_params::clean_path;

/// A field written from the pair of values two other fields hold.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairTable {
    /// The two `ctx` paths the rows compare, in the order the null guard names.
    left: String,
    right: String,
    /// `((left literal, right literal), value)`, in the script's own order --
    /// the first row that matches wins, which is what each arm's `return` says.
    rows: Vec<((Value, Value), Value)>,
    target: String,
}

/// A bare Painless literal: `true`, `false`, a number, or a quoted string.
fn literal(text: &str) -> Option<Value> {
    let text = text.trim();
    match text {
        "true" => return Some(Value::Bool(true)),
        "false" => return Some(Value::Bool(false)),
        "null" | "" => return None,
        _ => {}
    }
    if let Some(quoted) = text
        .strip_prefix(['"', '\''])
        .and_then(|rest| rest.strip_suffix(['"', '\'']))
    {
        return Some(Value::String(quoted.to_owned()));
    }
    text.parse::<i64>()
        .ok()
        .map(Value::from)
        .or_else(|| text.parse::<f64>().ok().map(Value::from))
}

/// The `ctx` path a comparison reads, with the null-safe `?.` dropped.
fn subject(text: &str) -> Option<String> {
    let path = clean_path(
        text.trim()
            .strip_prefix("ctx")?
            .trim_start_matches(['?', '.']),
    );
    (!path.is_empty() && !path.contains(['(', ' ', '['])).then_some(path)
}

/// Read the table, or decline it.
///
/// Every row has to test the SAME two paths in the same order and write the
/// same target: a script whose arms read anything else is a different script,
/// and running half of it would write a word the vendor does not.
#[must_use]
pub fn parse_pair_table(script: &str) -> Option<PairTable> {
    let mut blocks = script.split("if (").skip(1);

    // `if (<a> == null || <b> == null) { return; }` -- what makes writing
    // nothing on an absent field the script's own answer rather than a guess.
    let (guard, body) = blocks.next()?.split_once(')')?;
    if !body
        .split_once('{')?
        .1
        .split_once('}')?
        .0
        .trim()
        .starts_with("return")
    {
        return None;
    }
    let mut absent = guard.split("||");
    let left = subject(absent.next()?.split_once("==")?.0)?;
    let right = subject(absent.next()?.split_once("==")?.0)?;
    if absent.next().is_some() || left == right {
        return None;
    }

    let mut rows = Vec::new();
    let mut target: Option<String> = None;
    for block in blocks {
        let (guard, tail) = block.split_once(')')?;
        let mut terms = guard.split("&&");
        let (left_term, left_value) = terms.next()?.split_once("==")?;
        let (right_term, right_value) = terms.next()?.split_once("==")?;
        if terms.next().is_some() || subject(left_term)? != left || subject(right_term)? != right {
            return None;
        }

        let written = tail.split_once('{')?.1.split_once('}')?.0;
        let (assignment, rest) = written.trim().split_once(';')?;
        if !rest.trim().starts_with("return") {
            return None;
        }
        let (path, value) = assignment.split_once('=')?;
        let path = subject(path)?;
        if *target.get_or_insert(path.clone()) != path {
            return None;
        }
        rows.push((
            (literal(left_value)?, literal(right_value)?),
            literal(value)?,
        ));
    }
    if rows.is_empty() {
        return None;
    }

    Some(PairTable {
        left,
        right,
        rows,
        target: target?,
    })
}

/// Write the row the two fields select, or leave the target alone.
///
/// An absent or null field on either side writes nothing, and so does a pair
/// no row names -- both are what the script's own guards and `return`s say.
#[must_use]
pub fn pair_table(event: &mut Event, pattern: &PairTable) -> bool {
    let (Some(left), Some(right)) = (
        event.get(&pattern.left).filter(|held| !held.is_null()),
        event.get(&pattern.right).filter(|held| !held.is_null()),
    ) else {
        return true;
    };

    let selected = pattern
        .rows
        .iter()
        .find(|((want_left, want_right), _)| *want_left == *left && *want_right == *right)
        .map(|(_, value)| value.clone());
    if let Some(value) = selected {
        let _ = event.set(&pattern.target, value);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Verbatim from the generated call site in
    /// `crates/dfe-transforms/src/filebeat/zeek_connection/default.rs`, in the
    /// escaped one-line form the call site holds.
    const ZEEK_DIRECTION: &str = r#"if (ctx.zeek?.connection?.local_orig == null ||\n    ctx.zeek?.connection?.local_resp == null) {\n  return;\n}\nif (ctx.zeek.connection.local_orig == true &&\n    ctx.zeek.connection.local_resp == true) {\n  ctx.network.direction = \"internal\";\n  return;\n}\nif (ctx.zeek.connection.local_orig == true &&\n    ctx.zeek.connection.local_resp == false) {\n  ctx.network.direction = \"outbound\";\n  return;\n}\nif (ctx.zeek.connection.local_orig == false &&\n    ctx.zeek.connection.local_resp == true) {\n  ctx.network.direction = \"inbound\";\n  return;\n}\nif (ctx.zeek.connection.local_orig == false &&\n    ctx.zeek.connection.local_resp == false) {\n  ctx.network.direction = \"external\";\n  return;\n}"#;

    fn direction(local_orig: &Value, local_resp: &Value) -> Option<Value> {
        let mut event = Event::new(json!({ "zeek": { "connection": {
            "local_orig": local_orig,
            "local_resp": local_resp,
        }}}));
        assert!(crate::painless_common::try_known_painless(
            &mut event,
            ZEEK_DIRECTION
        ));
        event.get("network.direction").cloned()
    }

    /// The two paths, the target, and every row -- reading three rows would
    /// leave one locality on whatever an earlier processor had written.
    fn row(left: bool, right: bool, value: &str) -> ((Value, Value), Value) {
        (
            (Value::Bool(left), Value::Bool(right)),
            Value::String(value.to_owned()),
        )
    }

    #[test]
    fn the_zeek_locality_pair_writes_the_row_it_selects() {
        assert_eq!(
            parse_pair_table(&crate::painless_common::normalise(ZEEK_DIRECTION)),
            Some(PairTable {
                left: "zeek.connection.local_orig".to_owned(),
                right: "zeek.connection.local_resp".to_owned(),
                rows: vec![
                    row(true, true, "internal"),
                    row(true, false, "outbound"),
                    row(false, true, "inbound"),
                    row(false, false, "external"),
                ],
                target: "network.direction".to_owned(),
            })
        );

        // All four rows, asserted on the WRITTEN value rather than the parse.
        for (left, right, want) in [
            (true, true, "internal"),
            (true, false, "outbound"),
            (false, true, "inbound"),
            (false, false, "external"),
        ] {
            assert_eq!(
                direction(&json!(left), &json!(right)),
                Some(json!(want)),
                "{left}/{right}"
            );
        }

        // The script's own null guard: an explicit null on either side writes
        // nothing, and so does an absent field.
        assert_eq!(direction(&json!(true), &json!(null)), None);
        assert_eq!(direction(&json!(null), &json!(false)), None);

        let mut absent = Event::new(json!({ "zeek": { "connection": {} } }));
        assert!(crate::painless_common::try_known_painless(
            &mut absent,
            ZEEK_DIRECTION
        ));
        assert_eq!(absent.get("network.direction"), None);
    }

    /// A row whose arms write two different targets is a different script.
    #[test]
    fn a_table_writing_two_targets_is_declined() {
        let script = r#"if (ctx.a.one == null || ctx.a.two == null) {\n  return;\n}\nif (ctx.a.one == true && ctx.a.two == true) {\n  ctx.t.one = \"x\";\n  return;\n}\nif (ctx.a.one == true && ctx.a.two == false) {\n  ctx.t.two = \"y\";\n  return;\n}"#;
        assert!(parse_pair_table(&crate::painless_common::normalise(script)).is_none());
    }

    /// A row that tests a THIRD field reads more than the guard admits.
    #[test]
    fn a_table_row_reading_a_third_field_is_declined() {
        let script = r#"if (ctx.a.one == null || ctx.a.two == null) {\n  return;\n}\nif (ctx.a.one == true && ctx.a.three == true) {\n  ctx.t.x = \"x\";\n  return;\n}"#;
        assert!(parse_pair_table(&crate::painless_common::normalise(script)).is_none());
    }

    /// An arm that falls through rather than returning leaves the later rows
    /// live, so the first match is not the answer.
    #[test]
    fn a_table_arm_without_its_return_is_declined() {
        let script = r#"if (ctx.a.one == null || ctx.a.two == null) {\n  return;\n}\nif (ctx.a.one == true && ctx.a.two == true) {\n  ctx.t.x = \"x\";\n}"#;
        assert!(parse_pair_table(&crate::painless_common::normalise(script)).is_none());
    }
}
