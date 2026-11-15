// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One record assembled from whichever named fields the event carries.
//!
//! Suricata reports a DNS answer as four flat fields -- `rrname`, `rdata`,
//! `rrtype`, `ttl` -- and ECS wants them as one member of `dns.answers` under
//! its own names. The pipeline reads each into a local, puts the ones that are
//! present into a map, and writes the map as a ONE-ELEMENT list when it came to
//! anything.
//!
//! The trailing arm is part of the same matcher rather than one of its own,
//! because `try_known_painless` claims a WHOLE script: reading only the record
//! would leave `dns.resolved_ip` to a second pass that has no way to run.
//!
//! Suricata writes it at three call sites and nothing else in the tree does.
//! Unclaimed it costs the source `dns.answers` on 17 events, `dns.resolved_ip`
//! on 6 and `related.ip` on 6 behind it.

use serde_json::{Map, Value};

use crate::params::{clean_path, ctx_locals as locals};
use dfe_core::Event;

/// The second write: one field copied out when another names a listed value.
///
/// Both paths are resolved at PARSE time through the locals the script bound,
/// so nothing has to carry a local's name into the run.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RecordAlso {
    /// The `ctx` path whose value decides.
    decider: String,
    /// The values that trigger the write, in the order the script tests them.
    values: Vec<String>,
    /// The `ctx` path that is written.
    member: String,
    /// Where it goes, as a one-element list.
    target: String,
}

/// A record built from the fields the event carries, written as a list of one.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RecordFromFields {
    /// `(key in the record, ctx path)`, in the order the script writes them --
    /// the document is insertion-ordered, so the order is behaviour.
    members: Vec<(String, String)>,
    target: String,
    also: Option<RecordAlso>,
}

/// Read the assembly, or decline it.
///
/// Every member has to come from a local the script bound to a `ctx` path, and
/// the guard on each has to name that same local: a record whose members are
/// computed, or guarded on something else, is a different script and running
/// half of it would write a record the vendor does not.
#[must_use]
pub fn parse_record_from_fields(script: &str) -> Option<RecordFromFields> {
    let bound = locals(script);
    let path_of = |name: &str| {
        bound
            .iter()
            .find(|(local, _)| local == name)
            .map(|(_, path)| path.clone())
    };

    // `def <rec> = [:];` -- the map the members are collected into.
    let record = script.split(';').find_map(|statement| {
        let (name, value) = statement.trim().strip_prefix("def ")?.split_once('=')?;
        (value.trim() == "[:]").then(|| name.trim().to_owned())
    })?;

    // `if (<local> != null) { <rec>["<key>"] = <local>; }`
    let mut members = Vec::new();
    for block in script.split("if (").skip(1) {
        let Some((guard, body)) = block.split_once(')') else {
            continue;
        };
        let Some(local) = guard.trim().strip_suffix("!= null").map(str::trim) else {
            continue;
        };
        let Some(assignment) = body
            .split_once('{')
            .and_then(|(_, rest)| rest.split_once('}'))
        else {
            continue;
        };
        let Some((key, value)) = assignment.0.trim().split_once('=') else {
            continue;
        };
        let key = key
            .trim()
            .strip_prefix(&format!("{record}["))
            .and_then(|rest| rest.strip_suffix(']'))
            .map(|inner| inner.trim().trim_matches(['"', '\'']).to_owned());
        let (Some(key), Some(path)) = (key, path_of(local)) else {
            continue;
        };
        if value.trim().trim_end_matches(';').trim() != local {
            return None;
        }
        members.push((key, path));
    }
    if members.is_empty() {
        return None;
    }

    // `if (!<rec>.isEmpty()) { ctx.<target> = [<rec>]; }`
    let (_, written) = script.split_once(&format!("!{record}.isEmpty()"))?;
    let written = written.split_once('{')?.1.split_once('}')?.0;
    let (target, value) = written.trim().split_once('=')?;
    let target = clean_path(target.trim().strip_prefix("ctx.")?);
    if target.is_empty() || value.trim().trim_end_matches(';').trim() != format!("[{record}]") {
        return None;
    }

    Some(RecordFromFields {
        members,
        target,
        also: parse_also(script, &bound),
    })
}

/// `if (<local> == "<v>" || <local> == "<v>") { ctx.<t> = [<other>]; }`
fn parse_also(script: &str, bound: &[(String, String)]) -> Option<RecordAlso> {
    let (_, rest) = script.rsplit_once("if (")?;
    let (guard, body) = rest.split_once(')')?;

    let path_of = |name: &str| {
        bound
            .iter()
            .find(|(local, _)| local == name)
            .map(|(_, path)| path.clone())
    };

    let mut decider = None;
    let mut values = Vec::new();
    for term in guard.split("||") {
        let (local, literal) = term.trim().split_once("==")?;
        let local = local.trim().to_owned();
        if *decider.get_or_insert(local.clone()) != local {
            return None;
        }
        values.push(literal.trim().trim_matches(['"', '\'']).to_owned());
    }
    let decider = path_of(&decider?)?;
    if values.is_empty() {
        return None;
    }

    let written = body.split_once('{')?.1.split_once('}')?.0;
    let (target, value) = written.trim().split_once('=')?;
    let target = clean_path(target.trim().strip_prefix("ctx.")?);
    let member = path_of(
        value
            .trim()
            .trim_end_matches(';')
            .trim()
            .strip_prefix('[')?
            .strip_suffix(']')?
            .trim(),
    )?;
    if target.is_empty() {
        return None;
    }

    Some(RecordAlso {
        decider,
        values,
        member,
        target,
    })
}

/// Build the record, or leave both targets alone.
///
/// A member the event does not carry is left out rather than written null,
/// which is what the script's own `!= null` guard says -- and an empty record
/// writes nothing at all, which is why an event with no DNS section comes out
/// with no `dns.answers`.
#[must_use]
pub fn record_from_fields(event: &mut Event, pattern: &RecordFromFields) -> bool {
    let mut record = Map::new();
    for (key, path) in &pattern.members {
        if let Some(value) = event.get(path).filter(|value| !value.is_null()) {
            record.insert(key.clone(), value.clone());
        }
    }
    if !record.is_empty() {
        let _ = event.set(&pattern.target, Value::Array(vec![Value::Object(record)]));
    }

    if let Some(also) = &pattern.also {
        let decides = event
            .get_str(&also.decider)
            .is_some_and(|held| also.values.iter().any(|value| value == held));
        if decides && let Some(value) = event.get(&also.member).cloned() {
            let _ = event.set(&also.target, Value::Array(vec![value]));
        }
    }
    true
}

/// The collect beside a rename walk: one member gathered when another names a
/// listed value.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RenameCollect {
    /// The OLD key whose value decides.
    decider: String,
    values: Vec<String>,
    /// The OLD key whose value is gathered.
    member: String,
    /// Where the gathered list lands, when it came to anything.
    target: String,
}

/// Every record of a list renamed onto ECS keys, one member gathered on the way.
///
/// The suricata DNS answers that arrive as a LIST take the same three names as
/// the flat form above, but a record at a time.
///
/// The rename MOVES the key to the end, because Painless removes it and then
/// assigns a new one -- so the runner uses `shift_remove` and a plain insert,
/// which is the same order Elasticsearch's own map keeps.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RecordRenames {
    list: String,
    /// `(old key, new key)`, in the order the script renames them.
    renames: Vec<(String, String)>,
    collect: Option<RenameCollect>,
}

/// Read the rename walk, or decline it.
///
/// Every rename has to remove from the loop's own item and write back to it: a
/// walk that moves a member somewhere else is a different script.
#[must_use]
pub fn parse_record_renames(script: &str) -> Option<RecordRenames> {
    let (head, rest) = script.split_once(" : ctx")?;
    let item = head.rsplit_once("for (def ")?.1.trim().to_owned();
    let (list, body) = rest.split_once(')')?;
    let list = clean_path(list.trim().trim_start_matches(['?', '.']));
    if list.is_empty() || item.is_empty() {
        return None;
    }

    // `def <local> = <item>.remove("<old>");` bound to the key it took.
    let taken = format!("= {item}.remove(");
    let mut locals: Vec<(String, String)> = Vec::new();
    for statement in body.split(';') {
        let Some((name, argument)) = statement.split_once(&taken) else {
            continue;
        };
        let key = argument.split_once(')')?.0.trim().trim_matches(['"', '\'']);
        let name = name.trim().rsplit_once("def ")?.1.trim();
        locals.push((name.to_owned(), key.to_owned()));
    }

    // `if (<local> != null) { <item>["<new>"] = <local>; }`
    let mut renames = Vec::new();
    for block in body.split("if (").skip(1) {
        let Some((guard, tail)) = block.split_once(')') else {
            continue;
        };
        let Some(local) = guard.trim().strip_suffix("!= null").map(str::trim) else {
            continue;
        };
        let Some(assignment) = tail.split_once('{').and_then(|(_, r)| r.split_once('}')) else {
            continue;
        };
        let Some((target, value)) = assignment.0.trim().split_once('=') else {
            continue;
        };
        let Some(new) = target
            .trim()
            .strip_prefix(&format!("{item}["))
            .and_then(|rest| rest.strip_suffix(']'))
            .map(|inner| inner.trim().trim_matches(['"', '\'']).to_owned())
        else {
            continue;
        };
        if value.trim().trim_end_matches(';').trim() != local {
            return None;
        }
        let old = locals
            .iter()
            .find(|(name, _)| name == local)
            .map(|(_, key)| key.clone())?;
        renames.push((old, new));
    }
    if renames.is_empty() {
        return None;
    }

    Some(RecordRenames {
        list,
        renames,
        collect: parse_rename_collect(script, body, &locals),
    })
}

/// `if (<local> == "<v>" || ...) { <acc>.add(<other>); }` and the write after
/// the loop.
fn parse_rename_collect(
    script: &str,
    body: &str,
    locals: &[(String, String)],
) -> Option<RenameCollect> {
    let key_of = |name: &str| {
        locals
            .iter()
            .find(|(local, _)| local == name)
            .map(|(_, key)| key.clone())
    };

    // The gathering block, found by its `.add(` rather than by position: the
    // `if` after the loop tests the accumulator's size and would be taken by a
    // search from the end.
    let (guard, added) = body
        .split("if (")
        .skip(1)
        .filter_map(|block| block.split_once(')'))
        .find(|(guard, tail)| guard.contains("==") && tail.contains(".add("))?;
    let mut decider = None;
    let mut values = Vec::new();
    for term in guard.split("||") {
        let (local, literal) = term.trim().split_once("==")?;
        let local = local.trim().to_owned();
        if *decider.get_or_insert(local.clone()) != local {
            return None;
        }
        values.push(literal.trim().trim_matches(['"', '\'']).to_owned());
    }
    let decider = key_of(&decider?)?;

    let (accumulator, member) = added.split_once(".add(")?;
    let accumulator = accumulator.trim().trim_start_matches('{').trim();
    let member = key_of(member.split_once(')')?.0.trim())?;

    // `if (<acc>.size() > 0) { ctx.<target> = <acc>; }` after the loop.
    let (_, written) = script.split_once(&format!("{accumulator}.size() > 0"))?;
    let written = written.split_once('{')?.1.split_once('}')?.0;
    let (target, value) = written.trim().split_once('=')?;
    let target = clean_path(target.trim().strip_prefix("ctx.")?);
    if target.is_empty() || value.trim().trim_end_matches(';').trim() != accumulator {
        return None;
    }

    Some(RenameCollect {
        decider,
        values,
        member,
        target,
    })
}

/// Rename every record's members in place, gathering the one the guard names.
///
/// A record that is not a map is left as it stands, and a key it does not carry
/// is not created -- both are what the script's own `!= null` guard says.
#[must_use]
pub fn record_renames(event: &mut Event, pattern: &RecordRenames) -> bool {
    let Some(Value::Array(records)) = event.get(&pattern.list).cloned() else {
        return true;
    };

    let mut gathered = Vec::new();
    let mut renamed = Vec::with_capacity(records.len());
    for record in records {
        let Value::Object(mut members) = record else {
            renamed.push(record);
            continue;
        };
        // The rename MOVES the key to the end, which is what removing and
        // re-assigning does in Painless.
        let mut taken: Vec<(String, Value)> = Vec::new();
        for (old, new) in &pattern.renames {
            if let Some(value) = members.shift_remove(old.as_str()) {
                taken.push((old.clone(), value.clone()));
                members.insert(new.clone(), value);
            }
        }
        if let Some(collect) = &pattern.collect {
            let held = |key: &str| taken.iter().find(|(k, _)| k == key).map(|(_, v)| v);
            if held(&collect.decider)
                .and_then(Value::as_str)
                .is_some_and(|value| collect.values.iter().any(|want| want == value))
                && let Some(value) = held(&collect.member)
            {
                gathered.push(value.clone());
            }
        }
        renamed.push(Value::Object(members));
    }

    let _ = event.set(&pattern.list, Value::Array(renamed));
    if let Some(collect) = &pattern.collect
        && !gathered.is_empty()
    {
        let _ = event.set(&collect.target, Value::Array(gathered));
    }
    true
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "records_tests.rs"]
mod tests;
