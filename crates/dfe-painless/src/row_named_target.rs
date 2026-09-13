// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A params row that names its own destination path.
//!
//! The threat-intelligence packages carry one indicator per event as a
//! `{type, value}` pair, and the type decides BOTH the ECS type literal and the
//! field the value belongs in. So the row holds two members: one written to a
//! fixed path, and one that IS a path.
//!
//! ```painless
//! def mapping = params[ctx.json.ioc_type.toLowerCase()];
//! if (mapping == null) return;
//! set(ctx, "threat.indicator.type", mapping.type);
//! def value = ctx.json.ioc_value;
//! if (value == null) return;
//! set(ctx, mapping.target, value);
//! ```
//!
//! `carbonblack_edr`, `ti_mandiant_advantage_threat_intelligence` and
//! `ti_rapid7_threat_command_ioc` ship it, differing only in whitespace and in
//! whether a completion flag follows. A key the table has no row for writes
//! NOTHING, which is what leaves `carbonblack_edr`'s `query` indicators under
//! their vendor names for the rename that follows.

use serde_json::{Map, Value};

use crate::params::clean_path;
use dfe_core::event::Event;

/// The two writes, the key that selects them, and the flag that follows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowNamedTarget {
    /// The ctx path keying the table.
    key: String,
    /// Whether the key is folded before the lookup.
    lowercase: bool,
    /// The row member written to [`Self::literal_target`].
    type_member: String,
    literal_target: String,
    /// The ctx path whose value is written.
    value_source: String,
    /// The row member naming where that value lands.
    target_member: String,
    /// `ctx["<flag>"] = true;` after both writes, where the script sets one.
    flag: Option<String>,
}

/// One `set(ctx, <destination>, <source>)` application, arguments unparsed.
struct SetCall<'a> {
    destination: &'a str,
    source: &'a str,
}

/// Every `set(ctx, ...)` the script makes, in order.
///
/// Both helpers the script declares take `base`, so anchoring on the `ctx`
/// argument picks the applications and leaves the declarations out.
fn set_calls(script: &str) -> Vec<SetCall<'_>> {
    let mut calls = Vec::new();
    for (at, marker) in script.match_indices("set(ctx, ") {
        if script[..at].ends_with('_') {
            continue;
        }
        let arguments = &script[at + marker.len()..];
        let Some(end) = arguments.find(')') else {
            continue;
        };
        let Some((destination, source)) = arguments[..end].split_once(',') else {
            continue;
        };
        calls.push(SetCall {
            destination: destination.trim(),
            source: source.trim(),
        });
    }
    calls
}

/// `<local>.<member>` read against the local the lookup bound.
fn member_of<'a>(expression: &'a str, local: &str) -> Option<&'a str> {
    expression
        .strip_prefix(local)
        .and_then(|rest| rest.strip_prefix('.'))
        .filter(|member| !member.is_empty() && !member.contains('.'))
}

/// A dotted path read up to its statement's end, or `None` where it is an
/// expression rather than a field.
fn statement_path(rest: &str) -> Option<String> {
    let path = clean_path(rest.split(';').next()?);
    (!path.is_empty() && !path.contains(['(', ' ', ','])).then_some(path)
}

/// Read the lookup, both writes and the flag, or decline.
///
/// Every part comes off the script: the lookup names the key path and the local
/// it binds, the first `set` has to write a row member to a QUOTED path, and the
/// second has to write a bound ctx value to a row member. A script writing
/// anything else declines rather than binding to a runner that would invent a
/// destination.
pub fn parse_row_named_target(script: &str) -> Option<RowNamedTarget> {
    // `def <local> = params[ctx.<key>.toLowerCase()];`
    let (head, rest) = script.split_once(" = params[ctx.")?;
    let local = head.rsplit(' ').next().filter(|name| !name.is_empty())?;
    let (key, _) = rest.split_once(']')?;
    let (key, lowercase) = key
        .strip_suffix(".toLowerCase()")
        .map_or((key, false), |folded| (folded, true));
    let key = statement_path(key)?;

    // An unlisted key writes nothing at all, and without the script's own guard
    // the runner would write a null type instead.
    if !script.contains(&format!("({local} == null) return")) {
        return None;
    }

    let calls = set_calls(script);
    let [kind, value] = calls.as_slice() else {
        return None;
    };
    let literal_target = kind.destination.trim_matches(['"', '\'']);
    if literal_target == kind.destination || literal_target.is_empty() {
        return None;
    }
    let type_member = member_of(kind.source, local)?;
    let target_member = member_of(value.destination, local)?;

    // `def <bound> = ctx.<source>;` and the guard that follows it, so an absent
    // value leaves the type written and the flag unset.
    let bound = value.source;
    let (_, tail) = script.split_once(&format!("def {bound} = ctx."))?;
    let value_source = statement_path(tail)?;
    if !script.contains(&format!("({bound} == null) return")) {
        return None;
    }

    Some(RowNamedTarget {
        key,
        lowercase,
        type_member: type_member.to_string(),
        literal_target: literal_target.to_string(),
        value_source,
        target_member: target_member.to_string(),
        flag: flag_set(script),
    })
}

/// `ctx["<flag>"] = true;` written after both `set` calls.
fn flag_set(script: &str) -> Option<String> {
    let (_, rest) = script.rsplit_once("ctx[")?;
    let (name, tail) = rest.split_once(']')?;
    tail.trim_start()
        .starts_with("= true")
        .then(|| name.trim_matches(['"', '\'']).to_string())
        .filter(|name| !name.is_empty())
}

/// Write the type literal, then the value at the path its row names.
///
/// Each step reproduces one of the script's `return`s: no row writes nothing,
/// and an absent value stops after the type -- which is why the flag is set
/// last and only where both writes ran.
pub fn run_row_named_target(
    event: &mut Event,
    pattern: &RowNamedTarget,
    params: &Map<String, Value>,
) -> bool {
    let Some(key) = event.get_as_string(&pattern.key) else {
        return true;
    };
    let key = if pattern.lowercase {
        key.to_lowercase()
    } else {
        key
    };
    let Some(Value::Object(row)) = params.get(&key) else {
        return true;
    };
    let kind = row.get(&pattern.type_member).cloned();
    let target = row
        .get(&pattern.target_member)
        .and_then(Value::as_str)
        .map(str::to_string);

    if let Some(kind) = kind {
        let _ = event.set(&pattern.literal_target, kind);
    }
    // Painless reads an absent field as null, so both spellings of the script's
    // second `return` are this one.
    let Some(value) = event
        .get(&pattern.value_source)
        .filter(|held| !held.is_null())
        .cloned()
    else {
        return true;
    };
    let Some(target) = target else {
        return true;
    };
    let _ = event.set(&target, value);
    if let Some(flag) = &pattern.flag {
        let _ = event.set(flag, Value::Bool(true));
    }
    true
}
