// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Named members of every record of a list, each a list of `{key, value}`
//! records folded into one map.
//!
//! `google_secops` ships three such members on every detection and folds them all
//! in one pass:
//!
//! ```painless
//! String[] kvFields = new String[] {"detection_fields", "outcomes", "rule_labels"};
//! for (def detection : ctx.google_secops.alert_v2.detection) {
//!   for (def fieldName : kvFields) {
//!     if (!(detection[fieldName] instanceof List)) { continue; }
//!     def flat = new HashMap();
//!     for (def entry : detection[fieldName]) {
//!       if (entry?.key == null || entry.key == '') { continue; }
//!       if (entry.value != null && entry.value != '') { flat[entry.key] = entry.value; }
//!       else if (entry.source != null && entry.source != '') { flat[entry.key] = entry.source; }
//!     }
//!     if (flat.isEmpty()) { detection.remove(fieldName); }
//!     else { detection[fieldName] = flat; }
//!   }
//! }
//! ```
//!
//! Distinct from [`crate::common::KnownPattern::KeyValuePairs`], which folds ONE
//! list at a document path and writes the fold back over it. That reader
//! declines a second loop, and the member here is two walks down.
//!
//! **An empty fold REMOVES the member rather than writing an empty map**, which
//! is a different document: `google_secops`'s fourth detection carries a
//! `detection_fields` list of entries with no usable value, and Elasticsearch
//! emits no `detection_fields` at all.

use dfe_core::Event;
use serde_json::{Map, Value};

use crate::common::{matching_brace, painless_path};

/// The list walked, the members folded, and the members of each entry read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberKvFold {
    /// The document path of the list of records.
    list: String,
    /// The members of each record that are folded, in the script's order.
    members: Vec<String>,
    /// The entry member holding the key.
    key: String,
    /// The entry members tried in order; the first that holds a usable value
    /// is the one stored.
    values: Vec<String>,
}

/// Fold each named member of each record, or remove it where nothing survives.
pub fn member_kv_fold(event: &mut Event, pattern: &MemberKvFold) -> bool {
    let Some(Value::Array(mut records)) = event.get(&pattern.list).cloned() else {
        return true;
    };

    for record in &mut records {
        let Some(members) = record.as_object_mut() else {
            continue;
        };
        for name in &pattern.members {
            // `!(x instanceof List)` continues, so a member that is absent or
            // already folded is left exactly as it stands.
            let Some(Value::Array(entries)) = members.get(name.as_str()) else {
                continue;
            };
            let mut flat = Map::new();
            for entry in entries {
                let Some(key) = entry.get(&pattern.key).and_then(Value::as_str) else {
                    continue;
                };
                if key.is_empty() {
                    continue;
                }
                if let Some(value) = pattern
                    .values
                    .iter()
                    .find_map(|member| entry.get(member).filter(|held| usable(held)))
                {
                    flat.insert(key.to_owned(), value.clone());
                }
            }
            if flat.is_empty() {
                // `shift_remove`, never `Map::remove`: under `preserve_order`
                // that is a swap and would drop the record's last key into the
                // freed slot.
                members.shift_remove(name.as_str());
            } else {
                members.insert(name.clone(), Value::Object(java_order(flat)));
            }
        }
    }

    let _ = event.set(&pattern.list, Value::Array(records));
    true
}

/// Whether `!= null && != ''` admits this value.
///
/// Painless compares a number against `''` by value, so a zero and a `false`
/// both pass -- only a null and an empty string are refused.
fn usable(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::String(text) => !text.is_empty(),
        _ => true,
    }
}

/// The order a `HashMap` hands its entries to the serialiser.
fn java_order(flat: Map<String, Value>) -> Map<String, Value> {
    let table = crate::helpers::java_table_size(flat.len());
    let mut entries: Vec<(String, Value)> = flat.into_iter().collect();
    entries.sort_by_key(|(key, _)| crate::helpers::java_bucket(key, table));
    entries.into_iter().collect()
}

/// Read the walk, the folded members and the entry members, or decline.
///
/// A WHOLE-SCRIPT match on the three nested loops: the outer walk over a
/// document list, the middle walk over a declared array of member names, and
/// the inner fold. A script doing anything beside this declines.
pub fn parse_member_kv_fold(script: &str) -> Option<MemberKvFold> {
    let (head, rest) = script.split_once("new String[] {")?;
    let members = names(rest.split_once('}')?.0)?;
    let names_local = declared(head)?;
    if !rest.contains(&names_local) {
        return None;
    }

    // Outer: the records, read off the document.
    let (record, walked, outer, tail) = for_each_def(script)?;
    if !tail.trim().is_empty() {
        return None;
    }
    if !walked.starts_with("ctx.") {
        return None;
    }
    let list = painless_path(walked)?;

    // Middle: the member names, read off the declared array.
    let (name, named, body, after) = for_each_def(outer)?;
    if named != names_local || !after.trim().is_empty() {
        return None;
    }

    // The member is reached by subscript on the record in every statement that
    // touches it, so the three spellings have to agree before anything is read.
    let member = format!("{record}[{name}]");
    if !body.contains(&format!("!({member} instanceof List)"))
        || !body.contains(&format!("{record}.remove({name})"))
        || !body.contains(&format!("{member} = "))
    {
        return None;
    }

    // Inner: the entries of that member.
    let (entry, entries, fold, _) = for_each_def(body)?;
    if entries != member {
        return None;
    }

    let key = guarded_member(fold, &entry)?;
    let values = stored(fold, &entry, &key)?;
    (!members.is_empty() && !values.is_empty()).then_some(MemberKvFold {
        list,
        members,
        key,
        values,
    })
}

/// `for (def <item> : <expression>) { <body> }`, as the item, the RAW
/// expression walked, the body, and what follows.
///
/// [`crate::common`]'s reader resolves the expression to a document path, and
/// two of the three loops here walk a local rather than the document.
fn for_each_def(text: &str) -> Option<(String, &str, &str, &str)> {
    let (_, rest) = text.split_once("for (def ")?;
    let (item, rest) = rest.split_once(':')?;
    let item = item.trim();
    if item.is_empty() || !item.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let (walked, rest) = rest.split_once(')')?;
    let rest = rest.trim_start();
    let close = matching_brace(rest)?;
    Some((
        item.to_owned(),
        walked.trim(),
        &rest[1..close],
        &rest[close + 1..],
    ))
}

/// The quoted members of a `new String[] { "a", "b" }` initialiser.
fn names(inside: &str) -> Option<Vec<String>> {
    inside
        .split(',')
        .map(|member| {
            let member = member.trim();
            let quote = member.chars().next().filter(|c| *c == '\'' || *c == '"')?;
            let inner = member.strip_prefix(quote)?.strip_suffix(quote)?;
            (!inner.is_empty()).then(|| inner.to_owned())
        })
        .collect()
}

/// The local the `new String[]` initialiser is bound to.
fn declared(head: &str) -> Option<String> {
    // The name sits before the `=`, and the declaration's type before that.
    let name = head
        .trim_end()
        .strip_suffix('=')?
        .trim()
        .rsplit(char::is_whitespace)
        .next()?;
    (!name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .then(|| name.to_owned())
}

/// The entry member the fold's opening guard requires, read off that guard.
///
/// `if (entry?.key == null || entry.key == '') { continue; }` names the key and
/// says an empty one is skipped, which is what the runner reproduces.
fn guarded_member(fold: &str, entry: &str) -> Option<String> {
    let (_, rest) = fold.split_once(&format!("{entry}?."))?;
    let named = rest.split_once("==")?.0.trim();
    (!named.is_empty() && named.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .then(|| named.to_owned())
}

/// The entry members the fold stores, in the order it tries them.
///
/// Every one has to be written under the SAME key and into the same map, or the
/// fold is doing something this reader has not read.
fn stored(fold: &str, entry: &str, key: &str) -> Option<Vec<String>> {
    let subscript = format!("[{entry}.{key}] = {entry}.");
    let mut values = Vec::new();
    let mut accumulator: Option<&str> = None;
    for (at, _) in fold.match_indices(&subscript) {
        let named = fold[at + subscript.len()..]
            .split([';', '\n', ' ', '}'])
            .next()?
            .trim();
        if named.is_empty() || !named.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return None;
        }
        let into = fold[..at].rsplit(['\n', ';', '{', '}']).next()?.trim();
        if !into.chars().all(|c| c.is_alphanumeric() || c == '_')
            || accumulator.get_or_insert(into) != &into
        {
            return None;
        }
        values.push(named.to_owned());
    }
    (!values.is_empty()).then_some(values)
}

#[cfg(test)]
#[path = "member_kv_fold_tests.rs"]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
mod tests;
