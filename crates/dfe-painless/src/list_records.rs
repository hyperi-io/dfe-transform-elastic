// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A list of records, rebuilt item by item or scanned for one tagged entry.
//!
//! Both matchers read ONE loop over a list of maps and nothing else. Reading
//! the whole script is what makes them safe to place high in the ladder: a
//! statement neither of them recognises declines the script rather than
//! running half of it.
//!
//! # Rebuilt
//!
//! github's dependabot stream collects one member off every entry, and its
//! issues stream builds a record of two.
//!
//! A member the entry does not carry is written null, which is what Painless
//! does, and the `drop_empty` every one of these pipelines runs afterwards is
//! what takes it away again -- github's issues capture carries labels with a
//! `description` and labels without.
//!
//! # Scanned
//!
//! The same list read for ONE entry, keyed on a tag the script excludes.
//!
//! The value is assigned on EVERY pass, so an advisory with only a GHSA
//! identifier keeps that one and an advisory carrying a CVE keeps the CVE --
//! the break is what stops the scan, not what makes the write.
//!
//! Unclaimed the two cost github 8 events on `vulnerability.enumeration`,
//! `vulnerability.id` and `vulnerability.reference`, and 5 more on
//! `github.issues.labels`.

use serde_json::{Map, Value};

use crate::params::{balanced, clean_path, ctx_path_term as ctx_path};
use dfe_core::Event;

/// What each entry of the list becomes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EntryValue {
    /// One member taken off the entry.
    Member(String),
    /// A record of named members, in the order the script puts them.
    Record(Vec<(String, String)>),
}

/// A list rebuilt from its own entries.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ListRebuild {
    /// The list the entries come from.
    list: String,
    /// Where the rebuilt list is written.
    target: String,
    take: EntryValue,
}

impl ListRebuild {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(list: impl Into<String>, target: impl Into<String>, take: EntryValue) -> Self {
        Self {
            list: list.into(),
            target: target.into(),
            take,
        }
    }
}

/// A list scanned for the first entry whose tag is not the default one.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ScanTaggedList {
    /// The list scanned.
    list: String,
    /// The member read on every pass; the last one read is what is written.
    value_member: String,
    /// The member compared against the default tag.
    tag_member: String,
    /// The tag the script starts with, and the one the comparison excludes.
    default_tag: String,
    /// What the value is when the list is empty.
    default_value: String,
    tag_target: String,
    value_target: String,
}

impl ScanTaggedList {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        list: impl Into<String>,
        value_member: impl Into<String>,
        tag_member: impl Into<String>,
        default_tag: impl Into<String>,
        default_value: impl Into<String>,
        tag_target: impl Into<String>,
        value_target: impl Into<String>,
    ) -> Self {
        Self {
            list: list.into(),
            value_member: value_member.into(),
            tag_member: tag_member.into(),
            default_tag: default_tag.into(),
            default_value: default_value.into(),
            tag_target: tag_target.into(),
            value_target: value_target.into(),
        }
    }
}

/// One `for (<type>? <item>: <source>) { <body> }`, and what sits either side.
struct OneLoop<'a> {
    before: &'a str,
    item: &'a str,
    source: &'a str,
    body: &'a str,
    after: &'a str,
}

/// Split a script around its single loop, or decline.
///
/// A second loop means the script does more than either matcher here reads.
fn one_loop(script: &str) -> Option<OneLoop<'_>> {
    let at = script.find("for (")?;
    let opens = script[at + "for ".len()..].trim_start();
    if opens.contains("for (") {
        return None;
    }
    let (header, rest) = balanced(opens, '(', ')')?;
    let (body, after) = balanced(rest.trim_start(), '{', '}')?;
    let (item, source) = header
        .split_once(':')
        .or_else(|| header.split_once(" in "))?;
    // The declared type, where there is one, is not part of the name.
    let item = item.trim().rsplit(char::is_whitespace).next()?.trim();
    if item.is_empty() || !item.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    Some(OneLoop {
        before: &script[..at],
        item,
        source: source.trim(),
        body,
        after,
    })
}

/// `<Type>? <name> = <value>`, with the type word dropped.
///
/// Declines anything whose left-hand side is a path or a call, so a write to
/// the document and a comparison both fall through to their own readers.
fn declaration(statement: &str) -> Option<(&str, &str)> {
    let (head, value) = statement.split_once('=')?;
    if value.starts_with('=') || head.ends_with(['!', '<', '>']) {
        return None;
    }
    let head = head.trim();
    if head.contains(['(', ')', '[', ']', '.']) {
        return None;
    }
    let name = head.rsplit(char::is_whitespace).next()?;
    (!name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .then_some((name, value))
}

/// The arguments of `<receiver>.<method>(..)`, when that is the whole
/// statement.
fn call_on<'a>(statement: &'a str, receiver: &str, method: &str) -> Option<&'a str> {
    let rest = statement
        .trim()
        .strip_prefix(receiver)?
        .strip_prefix('.')?
        .strip_prefix(method)?;
    let (arguments, tail) = balanced(rest.trim_start(), '(', ')')?;
    tail.trim().is_empty().then_some(arguments)
}

/// `<item>.<member>`, where the member is a single name.
fn member_of(term: &str, item: &str) -> Option<String> {
    let term = clean_path(term);
    let member = term.strip_prefix(item)?.strip_prefix('.')?;
    (!member.is_empty()
        && member
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '@'))
    .then(|| member.to_owned())
}

/// A `ctx.<path>` term as a dotted path.
///
/// Declines a call or a subscript: `ctx.a.entrySet()` and `ctx.a[0]` name
/// something this reader cannot resolve to a field.
/// The `ctx` path a local was bound to, or the path itself when it is written
/// out at the loop header.
fn list_path(bound: &[(String, String)], source: &str) -> Option<String> {
    if let Some(path) = ctx_path(source) {
        return Some(path);
    }
    let source = source.trim();
    bound
        .iter()
        .find(|(name, _)| name == source)
        .map(|(_, path)| path.clone())
}

/// Read the rebuild, or decline it.
///
/// Every statement has to be one of the four this matcher runs -- the
/// accumulator, a local bound to a `ctx` path, the record allocation, and the
/// puts and the add inside the loop. A script with a fifth writes something
/// this reader cannot see.
#[must_use]
pub fn parse_list_rebuild(script: &str) -> Option<ListRebuild> {
    let walk = one_loop(script)?;

    let mut accumulator: Option<String> = None;
    let mut bound: Vec<(String, String)> = Vec::new();
    for statement in walk.before.split(';') {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }
        let Some((name, value)) = declaration(statement) else {
            // `Map label;` and `String label_key, label_value;` declare
            // without assigning, which is the only other form these carry.
            if statement.split_whitespace().count() >= 2
                && !statement.contains(['(', ')', '{', '}', '='])
            {
                continue;
            }
            return None;
        };
        let value = value.trim();
        if value == "new ArrayList()" {
            if accumulator.replace(name.to_owned()).is_some() {
                return None;
            }
        } else {
            bound.push((name.to_owned(), ctx_path(value)?));
        }
    }
    let accumulator = accumulator?;
    let list = list_path(&bound, walk.source)?;

    let mut record: Option<String> = None;
    let mut members: Vec<(String, String)> = Vec::new();
    let mut added: Option<String> = None;
    for statement in walk.body.split(';') {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }
        if let Some((name, value)) = declaration(statement) {
            if value.trim() != "new HashMap()" || name == accumulator {
                return None;
            }
            record = Some(name.to_owned());
            continue;
        }
        if let Some(arguments) = call_on(statement, &accumulator, "add") {
            if added.replace(arguments.trim().to_owned()).is_some() {
                return None;
            }
            continue;
        }
        let holder = record.as_deref()?;
        let arguments = call_on(statement, holder, "put")?;
        let (key, value) = arguments.split_once(',')?;
        let key = key.trim().trim_matches(['"', '\'']);
        if key.is_empty() {
            return None;
        }
        members.push((key.to_owned(), member_of(value.trim(), walk.item)?));
    }

    let added = added?;
    let take = if Some(added.as_str()) == record.as_deref() {
        if members.is_empty() {
            return None;
        }
        EntryValue::Record(members)
    } else {
        if !members.is_empty() {
            return None;
        }
        EntryValue::Member(member_of(&added, walk.item)?)
    };

    let mut target: Option<String> = None;
    for statement in walk.after.split(';') {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }
        let (head, value) = statement.split_once('=')?;
        if value.trim() != accumulator {
            return None;
        }
        if target.replace(ctx_path(head)?).is_some() {
            return None;
        }
    }

    Some(ListRebuild::new(list, target?, take))
}

/// Rebuild the list, or leave the target alone.
///
/// An absent list writes nothing: every call site is gated on it, and Painless
/// would throw rather than write an empty one.
pub fn list_rebuild(event: &mut Event, pattern: &ListRebuild) -> bool {
    let Some(Value::Array(entries)) = event.get(&pattern.list).cloned() else {
        return true;
    };
    let rebuilt: Vec<Value> = entries
        .iter()
        .map(|entry| match &pattern.take {
            EntryValue::Member(member) => entry.get(member).cloned().unwrap_or(Value::Null),
            EntryValue::Record(members) => {
                let mut record = Map::new();
                for (key, member) in members {
                    record.insert(
                        key.clone(),
                        entry.get(member).cloned().unwrap_or(Value::Null),
                    );
                }
                Value::Object(record)
            }
        })
        .collect();
    let _ = event.set(&pattern.target, Value::Array(rebuilt));
    true
}

/// Read the scan, or decline it.
#[must_use]
pub fn parse_scan_tagged_list(script: &str) -> Option<ScanTaggedList> {
    let walk = one_loop(script)?;

    let mut seeded: Vec<(String, String)> = Vec::new();
    let mut bound: Vec<(String, String)> = Vec::new();
    for statement in walk.before.split(';') {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }
        let (name, value) = declaration(statement)?;
        let value = value.trim();
        if let Some(path) = ctx_path(value) {
            bound.push((name.to_owned(), path));
        } else if value.starts_with(['"', '\'']) {
            seeded.push((name.to_owned(), value.trim_matches(['"', '\'']).to_owned()));
        } else {
            return None;
        }
    }
    // Two locals, because the script writes two fields and the tag's own seed
    // is what the comparison excludes.
    if seeded.len() != 2 {
        return None;
    }
    let list = list_path(&bound, walk.source)?;

    // `<value local> = <item>.<member>;` on every pass.
    let (first, rest) = walk.body.split_once(';')?;
    let (value_local, value_term) = declaration(first.trim())?;
    let value_member = member_of(value_term.trim(), walk.item)?;

    // `if (!<item>.<member>.equals("<tag>")) { <tag local> = <item>.<member>; break; }`
    let rest = rest.trim().strip_prefix("if")?.trim_start();
    let (guard, after_guard) = balanced(rest, '(', ')')?;
    let (inner, tail) = balanced(after_guard.trim_start(), '{', '}')?;
    if !tail.trim().is_empty() {
        return None;
    }
    let (subject, literal) = guard.trim().strip_prefix('!')?.split_once(".equals(")?;
    let tag_member = member_of(subject, walk.item)?;
    let literal = literal
        .trim()
        .strip_suffix(')')?
        .trim()
        .trim_matches(['"', '\'']);

    let mut statements = inner.split(';').map(str::trim).filter(|s| !s.is_empty());
    let (tag_local, tag_term) = declaration(statements.next()?)?;
    if member_of(tag_term.trim(), walk.item)? != tag_member {
        return None;
    }
    if statements.next()? != "break" || statements.next().is_some() {
        return None;
    }
    if tag_local == value_local {
        return None;
    }

    let seed_of = |local: &str| {
        seeded
            .iter()
            .find(|(name, _)| name == local)
            .map(|(_, seed)| seed.clone())
    };
    let default_tag = seed_of(tag_local)?;
    let default_value = seed_of(value_local)?;
    // The excluded tag has to BE the seed, or the loop is deciding on a
    // literal this reader would not write back.
    if default_tag != literal {
        return None;
    }

    // `ctx.<a> = <local>; ctx.<b> = <local>;` -- one write per local.
    let mut tag_target = None;
    let mut value_target = None;
    for statement in walk.after.split(';') {
        let statement = statement.trim();
        if statement.is_empty() {
            continue;
        }
        let (written, value) = statement.split_once('=')?;
        let path = ctx_path(written)?;
        let slot = match value.trim() {
            local if local == tag_local => &mut tag_target,
            local if local == value_local => &mut value_target,
            _ => return None,
        };
        if slot.replace(path).is_some() {
            return None;
        }
    }

    Some(ScanTaggedList::new(
        list,
        value_member,
        tag_member,
        default_tag,
        default_value,
        tag_target?,
        value_target?,
    ))
}

/// Scan the list and write both fields, or leave them alone.
///
/// An entry whose tag is absent or null does NOT stop the scan: Painless throws
/// on the `equals` call there, and stopping would write a tag the vendor never
/// chose.
pub fn scan_tagged_list(event: &mut Event, pattern: &ScanTaggedList) -> bool {
    let Some(Value::Array(entries)) = event.get(&pattern.list).cloned() else {
        return true;
    };
    let mut tag = Value::String(pattern.default_tag.clone());
    let mut value = Value::String(pattern.default_value.clone());
    for entry in &entries {
        value = entry
            .get(&pattern.value_member)
            .cloned()
            .unwrap_or(Value::Null);
        if let Some(held) = entry.get(&pattern.tag_member).filter(|v| !v.is_null())
            && held.as_str() != Some(pattern.default_tag.as_str())
        {
            tag = held.clone();
            break;
        }
    }
    let _ = event.set(&pattern.tag_target, tag);
    let _ = event.set(&pattern.value_target, value);
    true
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "list_records_tests.rs"]
mod tests;
