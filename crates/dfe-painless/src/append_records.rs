// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Every record of a list appended to ONE list, rebuilt without its null
//! members.
//!
//! `ti_opencti` folds an indicator's external references this way, and it is
//! the only thing `opencti.indicator.external_reference.*` is ever built from.
//!
//! Sibling of [`crate::group_records`], which walks the same list and files
//! each record into a bucket one of its own members NAMES. Here there is one
//! target and no key, and the inner copy loop is the difference that decides
//! the output: the record is rebuilt from its non-null members rather than
//! appended whole.
//!
//! `CollectMapValues` claimed this on `.keySet()` plus `.add(` with no parse
//! behind it, and then declined at run time -- its argument reader wants
//! `<binding>.<leaf>` and the argument here is a bare local with no dot in it.
//! The script counted as handled on every event and wrote nothing.

use serde_json::{Map, Value};

use crate::group_records::{local_ctx_path, member_suffix};
use crate::params::{clean_path, is_ctx_path};
use dfe_core::Event;

/// A list's records appended to one target list, their null members dropped.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AppendRecords {
    /// The list walked.
    list: String,
    /// The member of each element that holds the record, empty where the
    /// element IS the record.
    record: String,
    /// The list appended to.
    target: String,
}

/// Read the append, or decline it.
///
/// Anchored on the ONE append, the same way [`crate::group_records`] is: a
/// script with two is appending two different things, and reading half of it
/// claims the call site while writing part of what the vendor writes.
#[must_use]
pub fn parse_append_records(script: &str) -> Option<AppendRecords> {
    let mut appends = script.match_indices(".add(");
    let (at, _) = appends.next()?;
    if appends.next().is_some() {
        return None;
    }
    let (head, rest) = (&script[..at], &script[at + ".add(".len()..]);

    // The accumulator, which has to be a bare local: a dotted argument is
    // `CollectMapValues`'s intent -- one member read off each entry -- and this
    // one appends the whole rebuilt record.
    let accumulator = rest.split(')').next()?.trim();
    if accumulator.is_empty()
        || !accumulator
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return None;
    }

    // `ctx.<target>.add(` with no subscript. A subscript is the bucket spelling
    // `group_records` reads, and a script carrying one names its target at run
    // time rather than in the text.
    let container = head.trim_end();
    if container.ends_with(']') {
        return None;
    }
    let target = clean_path(
        container
            .rsplit(['\n', '\r', '\t', ' ', '{', '}', ';'])
            .next()?
            .strip_prefix("ctx.")?,
    );
    if !is_ctx_path(&target) || target.contains('[') {
        return None;
    }

    // Whitespace-free, so the structural tests below read one spelling rather
    // than every way the vendor's formatter could lay the same statement out.
    // Once per call site, never per event.
    let compact: String = script.chars().filter(|c| !c.is_whitespace()).collect();

    // `ctx.<target> = [];` -- what makes the target a LIST appended to rather
    // than a value an earlier processor left there.
    if !compact.contains(&format!("ctx.{target}=["))
        && !compact.contains(&format!("ctx.{target}=newArrayList()"))
    {
        return None;
    }

    // `def <accumulator> = [:];` -- the empty map the copy loop fills.
    if !compact.contains(&format!("{accumulator}=[:]"))
        && !compact.contains(&format!("{accumulator}=newHashMap()"))
    {
        return None;
    }

    // `for (def <key_var> : <element>.keySet())`, and the element is what says
    // which list is walked and which member of it holds the record.
    let keys_at = script.find(".keySet()")?;
    let (before_element, element) = script[..keys_at].rsplit_once(" : ")?;
    let element = element.trim();
    let key_var = before_element.split_whitespace().next_back()?;
    if key_var.is_empty() || element.is_empty() {
        return None;
    }

    // The copy itself, and the null guard that is the whole reason the record
    // is rebuilt instead of appended: a member the vendor left null is dropped.
    let element_compact: String = element.chars().filter(|c| !c.is_whitespace()).collect();
    if !compact.contains(&format!("{element_compact}[{key_var}]!=null"))
        || !compact.contains(&format!(
            "{accumulator}[{key_var}]={element_compact}[{key_var}]"
        ))
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

    Some(AppendRecords {
        list: local_ctx_path(script, local)?,
        record,
        target,
    })
}

/// Append every record, without its null members, to the target list.
///
/// The seed and the append both sit INSIDE the vendor's loop, so a list with no
/// elements leaves no target behind at all -- and neither does an absent one,
/// because the script's `if (edges == null) return;` ends it before the seed.
/// Writing an empty list on either would be a key Elasticsearch does not have.
#[must_use]
pub fn append_records(event: &mut Event, pattern: &AppendRecords) -> bool {
    let Some(Value::Array(elements)) = event.get(&pattern.list) else {
        return true;
    };

    let mut appended: Vec<Value> = Vec::with_capacity(elements.len());
    for element in elements {
        let record = if pattern.record.is_empty() {
            Some(element)
        } else {
            element.get(pattern.record.as_str())
        };
        // Painless throws on `null.keySet()`, so a missing record is not a
        // record this appends an empty map for.
        let Some(Value::Object(members)) = record else {
            continue;
        };
        let kept: Map<String, Value> = members
            .iter()
            .filter(|(_, value)| !value.is_null())
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect();
        appended.push(Value::Object(kept));
    }
    if appended.is_empty() {
        return true;
    }

    let mut held = match event.get(&pattern.target) {
        Some(Value::Array(items)) => items.clone(),
        _ => Vec::new(),
    };
    held.extend(appended);
    let _ = event.set(&pattern.target, Value::Array(held));
    true
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[path = "append_records_tests.rs"]
mod tests;
