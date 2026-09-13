// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A member renamed in every record of a list, a value that is not a map
//! wrapped in one on the way.
//!
//! The vendor ships one key carrying either a map or a scalar, and
//! Elasticsearch's mapping needs a map either way -- so the pipeline normalises
//! the scalar into a single-key map rather than declining it:
//!
//! ```painless
//! for (def entity : ctx.trend_micro_vision_one.alert.impact_scope.entities) {
//!   if (entity.containsKey('entity_value')) {
//!     def ev = entity.remove('entity_value');
//!     if (ev instanceof Map) {
//!       entity.put('value', ev);
//!     } else {
//!       entity.put('value', ['account_value': ev]);
//!     }
//!   }
//! }
//! ```
//!
//! Distinct from [`crate::typed_member_rename::TypedMemberRename`], which is
//! the conditional half of the same idea: its type test decides WHETHER the key
//! moves and leaves the other spelling under the vendor's own name. Here every
//! occurrence moves and the type test decides only what it is wrapped in, so
//! that reader declines this script rather than renaming half of it.
//!
//! The renamed key lands at the END of the record, which is where Painless's
//! remove-then-put leaves it: `shift_remove` and a plain insert, never
//! `Map::remove`, which under `preserve_order` is a swap.

use dfe_core::Event;
use serde_json::{Map, Value};

use crate::common::{if_block, painless_path};

/// The list walked, the member renamed, and the key a scalar is wrapped under.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberValueWrap {
    /// The document path of the list of records.
    list: String,
    /// The member the record carries.
    from: String,
    /// The member it is renamed to.
    to: String,
    /// The single key a value that is not a map is wrapped under.
    wrap: String,
}

/// Rename the member in every record, wrapping a value that is not a map.
pub fn member_value_wrap(event: &mut Event, pattern: &MemberValueWrap) -> bool {
    let Some(Value::Array(mut records)) = event.get(&pattern.list).cloned() else {
        return true;
    };

    for record in &mut records {
        let Some(members) = record.as_object_mut() else {
            continue;
        };
        // `containsKey` is the whole guard, so a member the record does carry
        // moves whatever it holds -- an explicit null included.
        let Some(held) = members.shift_remove(pattern.from.as_str()) else {
            continue;
        };
        let wrapped = if held.is_object() {
            held
        } else {
            let mut single = Map::new();
            single.insert(pattern.wrap.clone(), held);
            Value::Object(single)
        };
        members.insert(pattern.to.clone(), wrapped);
    }

    let _ = event.set(&pattern.list, Value::Array(records));
    true
}

/// Read the walk, the rename and the wrapping key, or decline.
///
/// A WHOLE-SCRIPT match: one loop, one guarded block, and a body that is the
/// remove and the two puts and nothing else.
pub fn parse_member_value_wrap(script: &str) -> Option<MemberValueWrap> {
    let (record, walked, body) = walk(script.trim())?;
    let list = painless_path(walked)?;

    let (guard, guarded, after) = if_block(body.trim())?;
    if !after.trim().is_empty() {
        return None;
    }
    let from = quoted_call(guard.trim(), &format!("{record}.containsKey("))?;

    // The removed value is bound to a local, which is what both puts write.
    let (statement, rest) = guarded.trim().split_once(';')?;
    let (declaration, removed) = statement.split_once('=')?;
    if removed.trim() != format!("{record}.remove('{from}')")
        && removed.trim() != format!("{record}.remove(\"{from}\")")
    {
        return None;
    }
    let local = declaration.trim().strip_prefix("def ")?.trim();
    if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    let (test, mapped, tail) = if_block(rest.trim())?;
    if test.trim() != format!("{local} instanceof Map") {
        return None;
    }
    let to = put_of(mapped, &record, local)?;

    let otherwise = tail.trim().strip_prefix("else")?.trim_start();
    let close = crate::common::matching_brace(otherwise)?;
    if !otherwise[close + 1..].trim().is_empty() {
        return None;
    }
    // Both arms have to write the SAME member, or the script is choosing
    // between two names and this reader has read only one of them.
    let (wrapped_to, wrap) = wrapped_put(&otherwise[1..close], &record, local)?;
    if wrapped_to != to || to == from {
        return None;
    }

    Some(MemberValueWrap {
        list,
        from,
        to,
        wrap,
    })
}

/// `for (def <item> : ctx.<path>) { <body> }`, the loop being the whole script.
fn walk(script: &str) -> Option<(String, &str, &str)> {
    let (head, rest) = script.split_once("for (def ")?;
    if !head.trim().is_empty() {
        return None;
    }
    let (item, rest) = rest.split_once(':')?;
    let item = item.trim();
    if item.is_empty() || !item.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let (walked, rest) = rest.split_once(')')?;
    let walked = walked.trim();
    if !walked.starts_with("ctx.") {
        return None;
    }
    let rest = rest.trim_start();
    let close = crate::common::matching_brace(rest)?;
    if !rest[close + 1..].trim().is_empty() {
        return None;
    }
    Some((item.to_owned(), walked, &rest[1..close]))
}

/// The quoted argument of a call opening `text`.
fn quoted_call(text: &str, call: &str) -> Option<String> {
    let argument = text.strip_prefix(call)?.trim_start();
    let quote = argument
        .chars()
        .next()
        .filter(|c| *c == '\'' || *c == '"')?;
    let (inner, rest) = argument.strip_prefix(quote)?.split_once(quote)?;
    (!inner.is_empty() && rest.trim() == ")").then(|| inner.to_owned())
}

/// `<record>.put('<to>', <local>);` as the target member.
fn put_of(body: &str, record: &str, local: &str) -> Option<String> {
    let statement = body.trim().strip_suffix(';')?;
    let argument = statement.strip_prefix(&format!("{record}.put("))?;
    let quote = argument
        .chars()
        .next()
        .filter(|c| *c == '\'' || *c == '"')?;
    let (to, rest) = argument.strip_prefix(quote)?.split_once(quote)?;
    let written = rest.trim().strip_prefix(',')?.trim().strip_suffix(')')?;
    (!to.is_empty() && written.trim() == local).then(|| to.to_owned())
}

/// `<record>.put('<to>', ['<wrap>': <local>]);` as the target and wrapping key.
fn wrapped_put(body: &str, record: &str, local: &str) -> Option<(String, String)> {
    let statement = body.trim().strip_suffix(';')?;
    let argument = statement.strip_prefix(&format!("{record}.put("))?;
    let quote = argument
        .chars()
        .next()
        .filter(|c| *c == '\'' || *c == '"')?;
    let (to, rest) = argument.strip_prefix(quote)?.split_once(quote)?;
    let literal = rest
        .trim()
        .strip_prefix(',')?
        .trim()
        .strip_suffix(')')?
        .trim();
    let inside = literal.strip_prefix('[')?.strip_suffix(']')?;
    let quote = inside.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    let (wrap, rest) = inside.strip_prefix(quote)?.split_once(quote)?;
    let written = rest.trim().strip_prefix(':')?.trim();
    (!to.is_empty() && !wrap.is_empty() && written == local)
        .then(|| (to.to_owned(), wrap.to_owned()))
}

#[cfg(test)]
#[path = "member_value_wrap_tests.rs"]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
mod tests;
