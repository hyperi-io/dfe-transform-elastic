// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One RECORD built per list item that passes a literal guard, appended to an
//! ECS list.
//!
//! `proofpoint_essentials` turns its message parts into `email.attachments` this
//! way, keeping only the parts the vendor marked as attached.
//!
//! Distinct from [`crate::append_records`], which appends each record REBUILT
//! from its own keys: here every member is renamed into a nested path the
//! script names, and a guard decides which items contribute at all.
//!
//! `CollectFromList` claimed this on the guard alone and collected
//! `disposition` into `email.attachments` -- the guard's own member, in place
//! of the file record. That arm now reads what is APPENDED and declines a
//! record it did not build from a member.

use serde_json::{Map, Value};

use crate::params::clean_path;
use dfe_core::Event;

/// A guarded record build over one list.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GuardedRecords {
    /// The list walked.
    list: String,
    /// `(member, literal)` an item must match to contribute.
    guard: Option<(String, String)>,
    /// `(path inside the record, the item member it reads)`, in script order.
    writes: Vec<(String, String)>,
    /// The list the built records are appended to.
    target: String,
}

/// `for (<var> in ctx.<list>)`, as its two parts.
fn loop_over_ctx(script: &str) -> Option<(String, String)> {
    let (_, rest) = script.split_once("for (")?;
    let (head, _) = rest.split_once(')')?;
    let (var, source) = head.split_once(" in ")?;
    let var = var.trim().rsplit(' ').next()?.trim();
    let source = source.trim().strip_prefix("ctx.")?;
    (!var.is_empty()).then(|| (var.to_string(), clean_path(source)))
}

/// Read the whole pattern, or `None` where the script is a different one.
#[must_use]
pub fn parse_guarded_records(script: &str) -> Option<GuardedRecords> {
    let (var, list) = loop_over_ctx(script)?;

    // The append names both the target and the local the record was built in.
    let (head, rest) = script.split_once(".add(")?;
    let local: String = rest
        .trim_start()
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if local.is_empty() {
        return None;
    }
    let target = clean_path(
        head.rsplit(['\n', ';', '{', '}'])
            .next()?
            .trim()
            .strip_prefix("ctx.")?,
    );

    // The record must be BUILT here, not appended from the list -- that is
    // `CollectFromList`'s script, and claiming it would collect one member.
    if !script.contains(&format!("{local} = [:]"))
        && !script.contains(&format!("{local} = new HashMap()"))
    {
        return None;
    }

    let assignment = format!("{local}.");
    let source = format!("{var}.");
    let mut writes = Vec::new();
    for statement in script.split(';') {
        let statement = statement.trim_start_matches(['\n', ' ', '{', '}']).trim();
        let Some(rest) = statement.strip_prefix(&assignment) else {
            continue;
        };
        let Some((path, value)) = rest.split_once(" = ") else {
            continue;
        };
        // The scaffolding lines allocate the nested maps; only a read of the
        // item carries a value.
        let Some(member) = value.trim().strip_prefix(&source) else {
            continue;
        };
        let (path, member) = (clean_path(path.trim()), clean_path(member.trim()));
        if !path.is_empty() && !member.is_empty() {
            writes.push((path, member));
        }
    }
    if writes.is_empty() {
        return None;
    }

    Some(GuardedRecords {
        list,
        guard: literal_guard(script, &var),
        writes,
        target,
    })
}

/// `if (<var>.<member> == '<literal>')`, the test that decides which items
/// contribute.
fn literal_guard(script: &str, var: &str) -> Option<(String, String)> {
    let needle = format!("if ({var}.");
    let (_, rest) = script.split_once(&needle)?;
    let (condition, _) = rest.split_once(')')?;
    let (member, literal) = condition.split_once("==")?;
    let literal = literal.trim();
    let quote = literal.chars().next().filter(|c| matches!(c, '\'' | '"'))?;
    let inner = &literal[quote.len_utf8()..];
    let end = inner.find(quote)?;
    Some((member.trim().to_string(), inner[..end].to_string()))
}

/// Write `value` at a dotted `path` inside a record being built.
fn put_nested(record: &mut Map<String, Value>, path: &str, value: Value) {
    let Some((head, rest)) = path.split_once('.') else {
        record.insert(path.to_string(), value);
        return;
    };
    let entry = record
        .entry(head.to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    if !entry.is_object() {
        *entry = Value::Object(Map::new());
    }
    if let Value::Object(inner) = entry {
        put_nested(inner, rest, value);
    }
}

/// Build one record per passing item and append them to the target.
pub fn run_guarded_records(event: &mut Event, pattern: &GuardedRecords) -> bool {
    let Some(Value::Array(items)) = event.get(&pattern.list).cloned() else {
        return true;
    };

    let mut built = Vec::new();
    for item in &items {
        let Value::Object(members) = item else {
            continue;
        };
        if let Some((member, wanted)) = &pattern.guard
            && members.get(member).and_then(Value::as_str) != Some(wanted.as_str())
        {
            continue;
        }
        let mut record = Map::new();
        for (path, member) in &pattern.writes {
            // A member the vendor did not send writes nothing, which is what
            // Painless does with a null: the key is never created.
            match members.get(member) {
                Some(Value::Null) | None => {}
                Some(value) => put_nested(&mut record, path, value.clone()),
            }
        }
        if !record.is_empty() {
            built.push(Value::Object(record));
        }
    }
    if built.is_empty() {
        return true;
    }

    // Appended to whatever the target already holds, exactly as the script's
    // `?: []` preamble leaves it.
    let mut list = match event.get(&pattern.target) {
        Some(Value::Array(existing)) => existing.clone(),
        _ => Vec::new(),
    };
    list.extend(built);
    let _ = event.set(&pattern.target, Value::Array(list));
    true
}

// The script constant is quoted verbatim from a generated call site, which
// spells it `r#"..."#`. Keeping it character-identical is what lets a script be
// copied straight from a module into a test.
#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[allow(clippy::expect_used, clippy::needless_raw_string_hashes)]
#[path = "guarded_records_tests.rs"]
mod tests;
