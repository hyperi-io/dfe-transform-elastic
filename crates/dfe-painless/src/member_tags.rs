// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! ECS lists appended to when a record's own label carries a word.
//!
//! `darktrace` classifies a model breach from the components that triggered it,
//! and the only evidence is the metric's display label:
//!
//! ```painless
//! for (component in ctx.json.triggeredComponents) {
//!   if (component?.metric?.label?.toLowerCase().contains('connection')) {
//!     ctx.event?.type?.add('connection');
//!     if (ctx.event.category == null) { ctx.event.category = new ArrayList(); }
//!     ctx.event.category.add('network');
//!   }
//! }
//! ```
//!
//! Unclaimed it costs `event.type` and `event.category` on the two events whose
//! components are connection metrics.
//!
//! **The two appends are NOT the same operation, and reading them alike is
//! wrong on one of them.** `ctx.event?.type?.add(...)` is null-safe: an absent
//! `event.type` is LEFT absent. `event.category` is CREATED when null and then
//! appended, which is why the vendor spells the guard out. So each append
//! carries how it reaches its list, read off the script rather than assumed --
//! `create` from the guard, `null_safe` from the `?` in the path.

use serde_json::Value;

use dfe_core::Event;

use crate::common::{block_statements, for_each_in, if_block, painless_path};

/// One literal appended to one list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagAppend {
    /// The list appended to, as a ctx path.
    target: String,
    /// The literal appended.
    value: String,
    /// Whether the script creates the list when it is null, which is the only
    /// way a value reaches a list the event does not already carry.
    create: bool,
    /// Whether the script reaches the list through `?.`, which no-ops on an
    /// absent one where a bare path throws.
    null_safe: bool,
}

/// A list of records, the member word that selects one, and what to append.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberTags {
    /// The list walked, as a ctx path.
    list: String,
    /// The member read off each record, dotted and relative to it.
    member: String,
    /// The word looked for in that member.
    word: String,
    /// Whether the script lower-cases the member before looking.
    lower: bool,
    /// The appends, in the order the script makes them.
    appends: Vec<TagAppend>,
}

/// Read the loop, the membership test, and every append under it.
///
/// The parse demands the WHOLE loop: one `if` over the member and nothing else,
/// and a body of appends and their create-guards alone. A loop doing anything
/// more falls through to whatever else can read it, rather than being claimed
/// here and half-run.
pub fn parse_member_tags(script: &str) -> Option<MemberTags> {
    let (item, list, body) = for_each_in(script)?;
    let (condition, block, rest) = if_block(&body)?;
    if !rest.trim().is_empty() {
        return None;
    }

    let (subject, tail) = condition.split_once(".contains(")?;
    let word = quoted(tail)?;
    // The `?` go first, so `label?.toLowerCase()` reads as one path and one call
    // rather than leaving a bare separator behind.
    let subject = subject.replace('?', "");
    let (path, lower) = match subject.trim().strip_suffix(".toLowerCase()") {
        Some(path) => (path, true),
        None => (subject.trim(), false),
    };
    let member = path
        .trim()
        .strip_prefix(item.as_str())?
        .strip_prefix('.')?
        .trim();
    if member.is_empty()
        || !member
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '_' | '.'))
    {
        return None;
    }

    let mut created: Vec<String> = Vec::new();
    let mut appends: Vec<TagAppend> = Vec::new();
    for statement in block_statements(block)? {
        if let Some(target) = create_guard(statement) {
            created.push(target);
            continue;
        }
        let (reach, argument) = statement.split_once(".add(")?;
        let target = painless_path(reach)?;
        appends.push(TagAppend {
            create: created.contains(&target),
            null_safe: reach.contains('?'),
            target,
            value: quoted(argument)?,
        });
    }
    if appends.is_empty() {
        return None;
    }

    Some(MemberTags {
        list,
        member: member.to_owned(),
        word,
        lower,
        appends,
    })
}

/// `if (ctx.<path> == null) { ctx.<path> = new ArrayList(); }`, as the path it
/// creates.
///
/// Both halves have to name the same path: a guard on one field that builds
/// another is not this.
fn create_guard(statement: &str) -> Option<String> {
    let (condition, block, rest) = if_block(statement)?;
    if !rest.trim().is_empty() {
        return None;
    }
    let guarded = painless_path(condition.split_once("== null")?.0)?;
    let (target, made) = block.split_once('=')?;
    if !made.contains("new ArrayList()") {
        return None;
    }
    (painless_path(target)? == guarded).then_some(guarded)
}

/// The first single- or double-quoted literal in `text`.
fn quoted(text: &str) -> Option<String> {
    let mut chars = text.char_indices().skip_while(|(_, c)| c.is_whitespace());
    let (open_at, quote) = chars.next().filter(|(_, c)| matches!(c, '\'' | '"'))?;
    let rest = &text[open_at + quote.len_utf8()..];
    let end = rest.find(quote)?;
    Some(rest[..end].to_owned())
}

/// Append to every list the selected records call for.
///
/// **A record whose member cannot be read STOPS the walk.** The vendor reaches
/// it through `?.` and then calls `.contains` on the result, so a null anywhere
/// in the chain throws -- and an ingest processor that throws keeps the writes
/// it already made. The same reasoning governs a bare (not null-safe) append
/// onto a list the event does not carry.
pub fn member_tags(event: &mut Event, pattern: &MemberTags) -> bool {
    let Some(records) = event.get(&pattern.list).and_then(Value::as_array).cloned() else {
        return true;
    };

    for record in &records {
        let Some(text) = member(record, &pattern.member).and_then(Value::as_str) else {
            return true;
        };
        let haystack = if pattern.lower {
            text.to_lowercase()
        } else {
            text.to_owned()
        };
        if !haystack.contains(&pattern.word) {
            continue;
        }
        for append in &pattern.appends {
            if append.create && !event.has_value(&append.target) {
                let _ = event.set(&append.target, Value::Array(Vec::new()));
            }
            match event.get(&append.target).and_then(Value::as_array) {
                Some(held) => {
                    let mut grown = held.clone();
                    grown.push(Value::String(append.value.clone()));
                    let _ = event.set(&append.target, Value::Array(grown));
                }
                // `ctx.a?.b?.add(...)` on an absent list is a no-op; the same
                // call written bare throws, and the walk stops there.
                None if append.null_safe => {}
                None => return true,
            }
        }
    }
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

#[cfg(test)]
#[path = "member_tags_tests.rs"]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
mod tests;
