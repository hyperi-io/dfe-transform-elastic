// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One member lifted out of every record of a list, into a list of its own.
//!
//! zeek's DNS stream zips its answers and TTLs into `dns.answers`, lets a
//! `foreach` processor tag each record whose data parsed as an address, then
//! takes the tags back out and drops them.
//!
//! `RecordRenames` in `painless_records` reads the nearest sibling -- suricata's
//! `for (def answer : ctx.dns.answers)` -- and cannot take this one: it walks by
//! ITEM rather than by index, its trigger is a `remove` bound to a local, and it
//! writes its gathered list only when the walk came to something, where zeek
//! writes an empty one. The empty list is the answer on 2 of the stream's
//! 7 events.

use serde_json::Value;

use crate::params::{clean_path, ctx_locals as locals};
use dfe_core::Event;

/// One member gathered out of a list's records and removed from each.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HoistMember {
    list: String,
    /// The key taken out of each record that carries it.
    member: String,
    target: String,
}

/// The `ctx` path a `def <name> = ctx...;` line reads, keyed by the local.
/// Read the hoist, or decline it.
///
/// The key gathered and the key removed have to be the same one, taken from the
/// loop's own record: a walk that moves some other member is a different script
/// and running this half of it would drop a field the vendor keeps.
#[must_use]
pub fn parse_hoist_member(script: &str) -> Option<HoistMember> {
    let bound = locals(script);

    // `def <acc> = new ArrayList();` -- what the gathered members collect into.
    let accumulator = script.split(';').find_map(|statement| {
        let (name, value) = statement.trim().strip_prefix("def ")?.split_once('=')?;
        (value.trim() == "new ArrayList()").then(|| name.trim().to_owned())
    })?;

    // `if (<local>[i].containsKey("<key>"))`
    let (head, rest) = script.split_once("[i].containsKey(")?;
    let name = head.rsplit(['(', ' ', '\n', '!']).next()?.trim();
    let list = bound
        .iter()
        .find(|(local, _)| local == name)
        .map(|(_, path)| path.clone())?;
    let (member, body) = rest.split_once(')')?;
    let member = member.trim().trim_matches(['"', '\'']).to_owned();
    if member.is_empty() {
        return None;
    }

    // `<acc>.add(<local>[i].<key>); <local>[i].remove("<key>");`
    let gathers = format!("{accumulator}.add({name}[i].{member})");
    let (dropped, _) = body
        .split_once(&format!("{name}[i].remove("))?
        .1
        .split_once(')')?;
    if !body.contains(&gathers) || dropped.trim().trim_matches(['"', '\'']) != member {
        return None;
    }

    // `ctx.<target> = <acc>;`, anchored on the LAST such write.
    let (written, _) = script.rsplit_once(&format!("= {accumulator};"))?;
    let target = written.trim().rsplit(['\n', ' ', '}', ';']).next()?;
    let target = clean_path(
        target
            .trim()
            .strip_prefix("ctx")?
            .trim_start_matches(['?', '.']),
    );
    (!target.is_empty() && !target.contains(['(', '['])).then_some(HoistMember {
        list,
        member,
        target,
    })
}

/// Lift the member out of every record that carries it.
///
/// The gathered list is written even when it is EMPTY, which is what the
/// script's unguarded `ctx.<target> = <acc>;` says.
#[must_use]
pub fn hoist_member(event: &mut Event, pattern: &HoistMember) -> bool {
    let Some(Value::Array(records)) = event.get(&pattern.list).cloned() else {
        return true;
    };

    let mut gathered = Vec::new();
    let mut kept = Vec::with_capacity(records.len());
    for record in records {
        let Value::Object(mut members) = record else {
            kept.push(record);
            continue;
        };
        if let Some(value) = members.shift_remove(pattern.member.as_str()) {
            gathered.push(value);
        }
        kept.push(Value::Object(members));
    }

    let _ = event.set(&pattern.list, Value::Array(kept));
    let _ = event.set(&pattern.target, Value::Array(gathered));
    true
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[path = "hoist_tests.rs"]
mod tests;
