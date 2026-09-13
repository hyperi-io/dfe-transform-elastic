// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One member of every record in a list, replaced by the text Java renders it as.
//!
//! `darktrace`'s model-breach stream carries a boolean expression tree under
//! each triggered component, and flattens it to text before the mapping sees it:
//!
//! ```painless
//! for (component in ctx.json.triggeredComponents) {
//!   component.logic.data = component?.logic?.data.toString();
//! }
//! ```
//!
//! Unclaimed it costs `darktrace.model_breach_alert.triggered_components` on 9
//! of the stream's 11 events -- the whole array compares wrong for one nested
//! member.
//!
//! **The rendering is Java's, and that is the entire difficulty.**
//! `Map.toString()` writes `{key=value, key2=value2}` with no quotes, and the
//! ORDER is the map's own iteration order. Elasticsearch's json processor builds
//! a `HashMap`, so that order is by hash bucket, not the order the vendor
//! spelled the keys in. The corpus is the evidence: the payload arrives
//! `{"left":...,"operator":...,"right":...}` and Elasticsearch publishes
//! `{left=..., right=..., operator=AND}` at every level of the tree -- buckets 5,
//! 11 and 14 of a 16-entry table. Rendering in our own insertion order would
//! write a string that is wrong in the way that is hardest to see.
//!
//! Two limits worth stating rather than discovering. A bin that reaches eight
//! colliding keys on a table of 64 or more is TREEIFIED, and Java then orders it
//! by hash and key rather than by insertion -- no map in the tree comes close,
//! and one that did would render in a different order here. And a value that
//! arrived in a `LinkedHashMap` renders in INSERTION order in Java, which this
//! does not reproduce; every map Painless itself builds (`[:]`) is a `HashMap`,
//! as is every map the json processor parses, so the model holds for what the
//! pipelines actually carry.

use serde_json::{Map, Value};

use dfe_core::Event;

use crate::common::for_each_in;

/// A list, and the member of each of its records that becomes text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringifyMember {
    /// The list walked, as a ctx path.
    list: String,
    /// The member rewritten, relative to each record, dotted.
    member: String,
}

impl StringifyMember {
    /// Build the pattern from its parts.
    #[must_use]
    pub fn new(list: impl Into<String>, member: impl Into<String>) -> Self {
        Self {
            list: list.into(),
            member: member.into(),
        }
    }
}

/// Read `for (<item> in ctx.<list>) { <item>.<member> = <item>?.<member>.toString(); }`.
///
/// The write and the read have to name the SAME member: a loop rendering one
/// member onto another is a different intent, and claiming it here would put the
/// text where the vendor does not.
///
/// The read's `?.` go before the comparison. Painless's null-safe access changes
/// only what happens when the chain breaks, and [`stringify_member`] reproduces
/// that separately.
pub fn parse_stringify_member(script: &str) -> Option<StringifyMember> {
    let (item, list, body) = for_each_in(script)?;

    // One statement, because this reproduces one and a loop doing more is a
    // script it cannot run.
    if body.matches(';').count() != 1 {
        return None;
    }
    let assignment = body
        .trim()
        .trim_end_matches(';')
        .trim()
        .strip_suffix(".toString()")?;
    let (target, source) = assignment.split_once('=')?;

    let member = local_member(target, &item)?;
    let read = local_member(&source.replace('?', ""), &item)?;
    (member == read).then_some(StringifyMember { list, member })
}

/// `<item>.<a>.<b>` as the dotted member path `a.b`, or `None` where the fragment
/// names something other than a member of the loop's own record.
fn local_member(fragment: &str, item: &str) -> Option<String> {
    let member = fragment
        .trim()
        .strip_prefix(item)?
        .strip_prefix('.')?
        .trim();
    (!member.is_empty()
        && member
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '_' | '.')))
    .then(|| member.to_owned())
}

/// Replace the member of each record with its Java rendering.
///
/// **A member that is absent or null STOPS the walk**, keeping what the records
/// before it were given. `null.toString()` throws in Painless, and an ingest
/// processor that throws leaves the mutations it already made in place -- so a
/// matcher that skipped and carried on would write records Elasticsearch does
/// not. `darktrace`'s test-model event is exactly that: its one component
/// carries a null `logic.data`, the script fails, and the empty `logic` map is
/// pruned away with nothing written into it.
pub fn stringify_member(event: &mut Event, pattern: &StringifyMember) -> bool {
    let Some(records) = event.get(&pattern.list).and_then(Value::as_array).cloned() else {
        return true;
    };

    let mut rebuilt: Vec<Value> = Vec::with_capacity(records.len());
    let mut stopped = false;
    for mut record in records {
        if !stopped {
            match member(&record, &pattern.member) {
                Some(held) if !held.is_null() => {
                    let rendered = Value::String(java_text(held));
                    stopped = !set_member(&mut record, &pattern.member, rendered);
                }
                _ => stopped = true,
            }
        }
        rebuilt.push(record);
    }

    let _ = event.set(&pattern.list, Value::Array(rebuilt));
    true
}

/// The value at a dotted member path inside one record.
fn member<'a>(record: &'a Value, path: &str) -> Option<&'a Value> {
    let mut held = record;
    for segment in path.split('.') {
        held = held.as_object()?.get(segment)?;
    }
    Some(held)
}

/// Write one dotted member back, without creating a parent the record lacks.
fn set_member(record: &mut Value, path: &str, value: Value) -> bool {
    let (parents, leaf) = match path.rsplit_once('.') {
        Some((parents, leaf)) => (Some(parents), leaf),
        None => (None, path),
    };
    let mut held = record;
    if let Some(parents) = parents {
        for segment in parents.split('.') {
            let Some(next) = held.as_object_mut().and_then(|map| map.get_mut(segment)) else {
                return false;
            };
            held = next;
        }
    }
    let Some(map) = held.as_object_mut() else {
        return false;
    };
    map.insert(leaf.to_owned(), value);
    true
}

/// What `String.valueOf` gives for a value Painless holds.
///
/// `{k=v, k2=v2}` for a map, `[a, b]` for a list, the text itself for a string,
/// and neither quotes nor escapes anywhere -- Java's renderers write the parts
/// verbatim.
fn java_text(value: &Value) -> String {
    let mut out = String::new();
    render(value, &mut out);
    out
}

fn render(value: &Value, out: &mut String) {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(flag) => out.push_str(if *flag { "true" } else { "false" }),
        Value::Number(number) => out.push_str(&render_number(number)),
        Value::String(text) => out.push_str(text),
        Value::Array(items) => {
            out.push('[');
            for (at, item) in items.iter().enumerate() {
                if at > 0 {
                    out.push_str(", ");
                }
                render(item, out);
            }
            out.push(']');
        }
        Value::Object(members) => {
            out.push('{');
            for (at, (key, held)) in hash_order(members).into_iter().enumerate() {
                if at > 0 {
                    out.push_str(", ");
                }
                out.push_str(key);
                out.push('=');
                render(held, out);
            }
            out.push('}');
        }
    }
}

/// A number as Java prints it.
///
/// A whole `double` keeps its `.0`, which Rust's own formatting drops. Java
/// switches to `E` notation outside `[1e-3, 1e7)` and this does not; no value in
/// the vendored pipelines renders through here at that magnitude, and one that
/// did would differ in the exponent alone.
fn render_number(number: &serde_json::Number) -> String {
    if let Some(whole) = number.as_i64() {
        return whole.to_string();
    }
    if let Some(whole) = number.as_u64() {
        return whole.to_string();
    }
    let Some(float) = number.as_f64() else {
        return number.to_string();
    };
    let text = float.to_string();
    if !float.is_finite() || text.contains(['.', 'e', 'E']) {
        text
    } else {
        format!("{text}.0")
    }
}

/// The order a `java.util.HashMap` iterates these keys in.
///
/// Bucket first, then insertion -- the two things that decide a `toString()`.
/// `preserve_order` is what makes the second half readable at all: our map holds
/// the keys in the order the payload spelled them, which is the order Java's
/// `put` calls ran in.
fn hash_order(members: &Map<String, Value>) -> Vec<(&String, &Value)> {
    let capacity = table_size(members.len());
    let mut rows: Vec<(usize, usize, &String, &Value)> = members
        .iter()
        .enumerate()
        .map(|(at, (key, held))| (bucket(key, capacity), at, key, held))
        .collect();
    rows.sort_unstable_by_key(|(bucket, at, _, _)| (*bucket, *at));
    rows.into_iter()
        .map(|(_, _, key, held)| (key, held))
        .collect()
}

/// `String.hashCode`, over UTF-16 code units -- which is what Java hashes, and
/// what makes an astral character count twice.
fn java_hash(key: &str) -> i32 {
    key.encode_utf16().fold(0i32, |hash, unit| {
        hash.wrapping_mul(31).wrapping_add(i32::from(unit))
    })
}

/// `HashMap.hash`: the high half folded down, so a small table still sees it.
#[allow(clippy::cast_sign_loss)]
fn bucket(key: &str, capacity: usize) -> usize {
    let hash = java_hash(key) as u32;
    ((hash ^ (hash >> 16)) as usize) & (capacity - 1)
}

/// The table a `HashMap` holds `entries` in.
///
/// It starts at 16 and doubles whenever the size passes three quarters of the
/// table, which is the load factor Java never changes from.
fn table_size(entries: usize) -> usize {
    let mut capacity = 16usize;
    while entries > capacity - capacity / 4 && capacity < 1 << 30 {
        capacity *= 2;
    }
    capacity
}

#[cfg(test)]
#[path = "stringify_member_tests.rs"]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
mod tests;
