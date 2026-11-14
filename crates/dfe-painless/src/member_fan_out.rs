// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One list walked, each named member it carries written to a field of its own.
//!
//! `ti_cybersixgill` turns its STIX `external_references` into the ECS tactic
//! block and two vendor sub-objects in a single walk.
//!
//! It was the source's whole debt -- 21 fields over four events, and every one
//! of them -- while the census reported the script wholly unbound.
//!
//! # What separates it from [`crate::gather_members`]
//!
//! That matcher ACCUMULATES: each record adds to a list and the list is written
//! once at the end under a `size() > 0` guard. This one does not accumulate at
//! all. Each write lands on a scalar field and runs again on the next record
//! that carries the member, so the LAST record carrying a member is what
//! survives. A reader that gathered these into lists would write four
//! descriptions where Elasticsearch has one.
//!
//! # The empty maps are part of the answer
//!
//! The three `new HashMap()` allocations run before the walk and are NOT
//! guarded on the walk finding anything. On three of cybersixgill's four
//! captured events no reference carries a `url` or a `positive_rate`, and
//! Elasticsearch's document still holds `cybersixgill.virustotal` as `{}` --
//! the source's own closing prune removes nulls and leaves empty maps alone.
//! Skipping the allocation as tidying would lose a field per event.
//!
//! # Whole grammar or decline
//!
//! The walk body must be nothing but `if (<var>?.<member> != null) { one
//! write }` blocks, and the script must end at the loop. Anything else declines
//! the WHOLE script rather than writing the part it understood: a source that
//! gets some of its block and not the rest reads as needing polish rather than
//! a different matcher, which is the trap this project has fallen into before.

use serde_json::{Map, Value};

use crate::params::{balanced, clean_path};
use dfe_core::Event;

/// One member, and the field it lands on.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MemberWrite {
    member: String,
    target: String,
    /// The script wraps the value in a one-element list, which is how ECS wants
    /// `threat.tactic.*`.
    wrapped: bool,
}

impl MemberWrite {
    /// The plain write: the member's value straight onto the field.
    #[must_use]
    pub fn new(member: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            member: member.into(),
            target: target.into(),
            wrapped: false,
        }
    }

    /// The value written as a one-element list.
    #[must_use]
    pub fn wrapped(mut self) -> Self {
        self.wrapped = true;
        self
    }
}

/// A list's members fanned out to fields of their own.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MemberFanOut {
    list: String,
    /// Maps the script opens before the walk, which survive it empty.
    allocations: Vec<String>,
    writes: Vec<MemberWrite>,
}

impl MemberFanOut {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(list: impl Into<String>, allocations: &[&str], writes: Vec<MemberWrite>) -> Self {
        Self {
            list: list.into(),
            allocations: allocations.iter().map(|p| (*p).to_owned()).collect(),
            writes,
        }
    }
}

/// The `ctx.` path a `for` walks, through the local a script usually binds it
/// to first.
fn walked_list(head: &str, subject: &str) -> Option<String> {
    if let Some(rest) = subject.strip_prefix("ctx") {
        let path = clean_path(rest.trim_start_matches(['?', '.']));
        return (!path.is_empty() && !path.contains(['(', '[', ' '])).then_some(path);
    }
    if !subject.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let (_, bound) = head.rsplit_once(&format!(" {subject} = "))?;
    let expression = bound.split_once(';')?.0.trim();
    let rest = expression.strip_prefix("ctx")?;
    let path = clean_path(rest.trim_start_matches(['?', '.']));
    (!path.is_empty() && !path.contains(['(', '[', ' '])).then_some(path)
}

/// The `ctx.<path> = new HashMap();` allocations a script makes before walking.
fn allocations(head: &str) -> Vec<String> {
    let mut found = Vec::new();
    for statement in head.split(';') {
        let Some((assigned, _)) = statement.split_once("= new HashMap()") else {
            continue;
        };
        let Some(path) = assigned
            .trim()
            .strip_prefix("ctx")
            .map(|rest| clean_path(rest.trim_start_matches(['?', '.'])))
        else {
            continue;
        };
        if !path.is_empty() && !path.contains(['(', '[']) && !found.contains(&path) {
            found.push(path);
        }
    }
    found
}

/// One `if (<var>?.<member> != null) { ctx.<target> = <var>.<member>; }`.
///
/// The guard and the write must name the SAME member, or the script is reading
/// one field and writing another and this is not the pattern.
fn member_write(var: &str, condition: &str, body: &str) -> Option<MemberWrite> {
    let member = condition
        .trim()
        .strip_prefix(var)?
        .trim_start_matches(['?', '.'])
        .split_once("!=")
        .filter(|(_, tail)| tail.trim() == "null")?
        .0
        .trim()
        .to_owned();
    if member.is_empty() || !member.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    // Exactly one statement, or the guard covers more than this write.
    let statement = body.trim().strip_suffix(';')?;
    if statement.contains(';') {
        return None;
    }
    let (assigned, value) = statement.split_once(" = ")?;
    let target = assigned
        .trim()
        .strip_prefix("ctx")
        .map(|rest| clean_path(rest.trim_start_matches(['?', '.'])))
        .filter(|path| !path.is_empty() && !path.contains(['(', '[']))?;

    let value = value.trim();
    let (value, wrapped) = match balanced(value, '[', ']') {
        Some((inner, after)) if after.trim().is_empty() => (inner.trim(), true),
        _ => (value, false),
    };
    // The value written has to be the very member the guard tested.
    (value.trim_end_matches(['?', '.']) == format!("{var}.{member}").trim_end_matches('.')
        || value == format!("{var}?.{member}"))
    .then_some(MemberWrite {
        member,
        target,
        wrapped,
    })
}

/// Read the list, the members it gives up and where each lands, or decline.
#[must_use]
pub fn parse_member_fan_out(script: &str) -> Option<MemberFanOut> {
    let (head, rest) = script.split_once("for (")?;
    let (var, rest) = rest.split_once(" : ")?;
    let var = var.trim().trim_start_matches("def ").trim();
    if var.is_empty() || !var.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let (subject, rest) = rest.split_once(')')?;
    let list = walked_list(head, subject.trim())?;

    let (body, tail) = balanced(rest.trim_start(), '{', '}')?;
    // The walk is the whole script. A tail is another statement this reader has
    // not looked at, and claiming the script would silently drop it.
    if !tail.trim().is_empty() {
        return None;
    }

    let mut writes = Vec::new();
    let mut rest = body.trim();
    while !rest.is_empty() {
        let after_if = rest.strip_prefix("if")?.trim_start();
        let (condition, after) = balanced(after_if, '(', ')')?;
        let (inner, after) = balanced(after.trim_start(), '{', '}')?;
        writes.push(member_write(var, condition, inner)?);
        rest = after.trim();
    }
    (!writes.is_empty()).then(|| MemberFanOut {
        list,
        allocations: allocations(head),
        writes,
    })
}

/// Write each member the walk finds, the last record carrying it winning.
#[must_use]
pub fn member_fan_out(event: &mut Event, pattern: &MemberFanOut) -> bool {
    let Some(Value::Array(records)) = event.get(&pattern.list).cloned() else {
        // The call site is guarded on the list being present, and a `for` over
        // anything but a list throws rather than writing a partial answer.
        return true;
    };

    // Before the walk, and not guarded on it finding anything -- an allocation
    // that stays empty is a field Elasticsearch keeps.
    for path in &pattern.allocations {
        let _ = event.set(path, Value::Object(Map::new()));
    }

    for write in &pattern.writes {
        // The script assigns on every record carrying the member, so the last
        // one is what the document ends up with.
        let Some(value) = records
            .iter()
            .rev()
            .find_map(|record| record.get(&write.member).filter(|v| !v.is_null()))
        else {
            continue;
        };
        let value = if write.wrapped {
            Value::Array(vec![value.clone()])
        } else {
            value.clone()
        };
        let _ = event.set(&write.target, value);
    }
    true
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#`.
#[allow(clippy::needless_raw_string_hashes)]
#[path = "member_fan_out_tests.rs"]
mod tests;
