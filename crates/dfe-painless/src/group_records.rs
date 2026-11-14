// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Every record of a list appended to a bucket its own member names.
//!
//! `ti_opencti` fans an indicator's observables out by STIX entity type, so a
//! hostname lands under `opencti.observable.hostname` and a file under
//! `opencti.observable.file`, whatever order the vendor sent them in.
//!
//! The bucket name is a VALUE, so it cannot be read off the script and the rest
//! of the pipeline cannot be written without it: every processor after this one
//! is guarded on `opencti.observable.<entity>`, which is 62% of the source's
//! wrong fields and every downstream `related.*` and `threat.indicator.*` the
//! observable feeds.
//!
//! The key steps are ORDERED and the order is the meaning -- `StixFile` folds
//! to `file` BEFORE the lowercase, and reading them as a set would leave
//! `stixfile` to match nothing.

use serde_json::{Map, Value};

use crate::params::{clean_path, is_ctx_path};
use dfe_core::Event;

/// What the script does to a record's member before filing under it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum KeyStep {
    Lowercase,
    Uppercase,
    /// Java's `String.replace(CharSequence, CharSequence)`, which is literal
    /// and replaces every occurrence -- the same contract as Rust's.
    Replace {
        from: String,
        to: String,
    },
}

/// A list's records filed into buckets named by one of their own members.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GroupRecords {
    /// The list walked.
    list: String,
    /// The member of each element that holds the record, empty where the
    /// element IS the record.
    record: String,
    /// The member of the record whose value names the bucket.
    key: String,
    /// What the script does to that value, in the order it does it.
    steps: Vec<KeyStep>,
    /// The map the buckets live under.
    target: String,
}

/// Read the grouping, or decline it.
///
/// Anchored on the ONE append: a script with two is filing two different things
/// and reading half of it would claim the call site while writing part of what
/// the vendor writes.
#[must_use]
pub fn parse_group_records(script: &str) -> Option<GroupRecords> {
    let mut appends = script.match_indices(".add(");
    let (at, _) = appends.next()?;
    if appends.next().is_some() {
        return None;
    }
    let (head, rest) = (&script[..at], &script[at + ".add(".len()..]);
    let element = rest.split(')').next()?.trim();

    // `ctx.<target>[<key_var>].add(` -- the subject says the bucket is named by
    // a value rather than spelled in the script.
    let (container, key_var) = head.trim_end().strip_suffix(']')?.rsplit_once('[')?;
    let key_var = key_var.trim();
    let container = container
        .trim_end()
        .rsplit(['\n', '\r', '\t', ' ', '{', '}', ';'])
        .next()?;
    let target = clean_path(container.strip_prefix("ctx.")?);
    if key_var.is_empty() || !is_ctx_path(&target) {
        return None;
    }

    // `if (!ctx.<target>.containsKey(<key_var>)) { ctx.<target>[<key_var>] = []; }`
    // -- what makes the bucket a LIST appended to rather than a value replaced.
    if !script.contains(&format!(".containsKey({key_var})"))
        || !seeds_empty_list(script, &target, key_var)
    {
        return None;
    }

    // `<local>[<index>]` plus whatever member holds the record.
    let (local, after) = element.split_once('[')?;
    let record = member_suffix(after.split_once(']')?.1)?;
    let local = local.trim();
    if local.is_empty() {
        return None;
    }

    // `<type> <key_var> = <element><member>;` -- the member the bucket is named
    // by, read off the same element the append files.
    let key = script.split(';').find_map(|statement| {
        let (lhs, rhs) = statement.split_once('=')?;
        (lhs.trim_end().rsplit([' ', '\n', '\r', '\t']).next()? == key_var)
            .then_some(())
            .and_then(|()| member_suffix(rhs.trim().strip_prefix(element)?))
            .filter(|member| !member.is_empty())
    })?;

    let mut steps = Vec::new();
    for statement in script.split(';') {
        let Some(call) = statement
            .trim()
            .strip_prefix(key_var)
            .and_then(|rest| rest.trim_start().strip_prefix('='))
            .and_then(|rhs| rhs.trim_start().strip_prefix(key_var))
            .and_then(|call| call.strip_prefix('.'))
        else {
            continue;
        };
        // A step this cannot read declines the WHOLE script: a key normalised
        // by three calls and filed under two of them names a bucket nothing
        // downstream is guarded on.
        steps.push(read_key_step(call)?);
    }

    Some(GroupRecords {
        list: local_ctx_path(script, local)?,
        record,
        key,
        steps,
        target,
    })
}

/// Whether the script seeds the named bucket with an empty list.
fn seeds_empty_list(script: &str, target: &str, key_var: &str) -> bool {
    let seed = format!("ctx.{target}[{key_var}] =");
    script.split(&seed).skip(1).any(|rest| {
        let value = rest.trim_start();
        value.starts_with("[]") || value.starts_with("new ArrayList()")
    })
}

/// The member a trailing accessor names, or the empty string where there is
/// none.
///
/// Both spellings, because the same pipeline writes both: `['node']` on the
/// element and `.node` on a local are one access to Painless.
///
/// Shared with [`crate::append_records`], which walks the same `edges[i]['node']`
/// list the same way and differs only in where the record lands.
pub(crate) fn member_suffix(rest: &str) -> Option<String> {
    let rest = rest.trim();
    if rest.is_empty() {
        return Some(String::new());
    }
    if let Some(inner) = rest.strip_prefix('[') {
        let (key, tail) = inner.split_once(']')?;
        if !tail.trim().is_empty() {
            return None;
        }
        let key = key.trim();
        let quote = key.chars().next()?;
        if quote != '\'' && quote != '"' {
            return None;
        }
        let key = key.strip_prefix(quote)?.strip_suffix(quote)?;
        return (!key.is_empty()).then(|| key.to_owned());
    }
    let key = rest.strip_prefix('.')?;
    (!key.is_empty() && key.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .then(|| key.to_owned())
}

/// One `<key_var> = <key_var>.<call>` step, or `None` where it is not one this
/// can reproduce.
fn read_key_step(call: &str) -> Option<KeyStep> {
    let call = call.trim();
    if call == "toLowerCase()" {
        return Some(KeyStep::Lowercase);
    }
    if call == "toUpperCase()" {
        return Some(KeyStep::Uppercase);
    }
    let (from, to) = call
        .strip_prefix("replace(")?
        .strip_suffix(')')?
        .split_once(',')?;
    let from = quoted(from)?;
    // Replacing the empty string inserts between every character, which is a
    // different operation and not one any pipeline here spells.
    (!from.is_empty()).then_some(KeyStep::Replace {
        from,
        to: quoted(to)?,
    })
}

/// A single- or double-quoted literal's contents.
fn quoted(text: &str) -> Option<String> {
    let text = text.trim();
    let quote = text.chars().next()?;
    if quote != '\'' && quote != '"' {
        return None;
    }
    Some(text.strip_prefix(quote)?.strip_suffix(quote)?.to_owned())
}

/// The `ctx` path a local is DECLARED from.
///
/// `crate::params::ctx_locals` reads `def <name> = ctx...` only, and this
/// script declares its list `ArrayList observables = ctx.observables.edges;`.
/// A declarator is demanded either way: a bare `<name> = ...` is a
/// reassignment and says nothing about where the list came from.
///
/// Shared with [`crate::append_records`] and [`crate::trim_delimited`], whose
/// locals are declared the same way.
pub(crate) fn local_ctx_path(script: &str, local: &str) -> Option<String> {
    script.split(';').find_map(|statement| {
        let (lhs, rhs) = statement.trim().split_once('=')?;
        let mut words = lhs.split_whitespace();
        if words.next_back()? != local || words.next().is_none() {
            return None;
        }
        let path = clean_path(rhs.trim().strip_prefix("ctx.")?);
        is_ctx_path(&path).then_some(path)
    })
}

/// File every record under the bucket its own member names.
///
/// A record whose key is absent, is not text, or normalises to nothing is
/// SKIPPED rather than filed somewhere chosen for it -- a bucket downstream is
/// not guarded on holds the record where nothing will ever read it, and leaves
/// no error behind.
#[must_use]
pub fn group_records(event: &mut Event, pattern: &GroupRecords) -> bool {
    let Some(Value::Array(elements)) = event.get(&pattern.list) else {
        return true;
    };

    let mut filed: Vec<(String, Value)> = Vec::with_capacity(elements.len());
    for element in elements {
        let record = if pattern.record.is_empty() {
            Some(element)
        } else {
            element.get(pattern.record.as_str())
        };
        let Some(record) = record else { continue };
        let Some(name) = record.get(pattern.key.as_str()).and_then(Value::as_str) else {
            continue;
        };

        let mut key = name.to_owned();
        for step in &pattern.steps {
            key = match step {
                KeyStep::Lowercase => key.to_lowercase(),
                KeyStep::Uppercase => key.to_uppercase(),
                KeyStep::Replace { from, to } => key.replace(from.as_str(), to),
            };
        }
        if !key.is_empty() {
            filed.push((key, record.clone()));
        }
    }
    if filed.is_empty() {
        return true;
    }

    // The two `containsKey` guards above the loop create the container, but
    // only ever as an empty map -- so creating it here, where there is
    // something to put in it, writes the same document.
    let mut buckets = match event.get(&pattern.target) {
        Some(Value::Object(held)) => held.clone(),
        _ => Map::new(),
    };
    for (key, record) in filed {
        match buckets.get_mut(&key) {
            Some(Value::Array(bucket)) => bucket.push(record),
            // A bucket that is not a list is what `.add()` throws on, so
            // nothing is written over it.
            Some(_) => {}
            None => {
                buckets.insert(key, Value::Array(vec![record]));
            }
        }
    }
    let _ = event.set(&pattern.target, Value::Object(buckets));
    true
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[path = "group_records_tests.rs"]
mod tests;
