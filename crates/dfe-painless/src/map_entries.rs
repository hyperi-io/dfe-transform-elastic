// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Two config-driven rewrites over the entries of one map subtree.
//!
//! servicenow's pipeline opens with both, in this order, and the whole of the
//! rest of the module is written against what they leave behind.
//!
//! The first copies ONE entry whose key is not in the script -- the key's name
//! is held in another field.
//!
//! The second wraps every scalar under that subtree into a one-key record, the
//! key chosen by a config flag.
//!
//! Both were unbound, and the wrap is what the rest of the source reads:
//! `servicenow.event.timestamp_field.value` sets `@timestamp`,
//! `servicenow.event.port.value` and `servicenow.event.opened_by.display_value`
//! are read further down, and the fingerprint takes `sys_id.value`. With the
//! wrap not running, every one of those resolved to nothing and the raw scalar
//! sat a level too shallow -- which is the whole of the misplaced-field count,
//! rather than anything lost.
//!
//! The order matters: the copy runs first and creates
//! `servicenow.event.timestamp_field`, which the wrap then turns into
//! `servicenow.event.timestamp_field.value`. Two adjacent call sites in the
//! generated module carry that order, so this holds them together.
//!
//! The wrap REPLACES each value where it sits, through `iter_mut`. Nothing is
//! removed and nothing is reinserted, so the map's key order is untouched --
//! which servicenow's parity depends on, and which `Map::remove` would have
//! broken under `preserve_order`.

use serde_json::{Map, Value};

use crate::params::{clean_path, ctx_locals as locals, ctx_path_plain, pointer_mut};
use dfe_core::Event;

/// One entry copied out of a map, its key named by another field's VALUE.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct KeyNamedByField {
    /// The map the entry is read from.
    path: String,
    /// The field holding the KEY NAME, not the value.
    key_from: String,
    target: String,
}

/// Every scalar entry of a map wrapped into a one-key record.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WrapEntries {
    /// The map whose entries are rewritten.
    path: String,
    /// Keys the script's own `continue` leaves alone.
    skip_keys: Vec<String>,
    /// The field the wrapping key is chosen by.
    flag: String,
    /// The literal `flag` is compared against.
    flag_value: String,
    /// The key used when `flag` reads `flag_value`.
    key_when_set: String,
    /// The key used otherwise.
    key_when_unset: String,
}

/// Read the dynamically-keyed copy, or decline it.
///
/// The guard's key and the getter's key must be the SAME field: a script
/// testing one name and reading another is doing a different job, and copying
/// the wrong entry leaves no error behind.
///
/// The receiver has to be a local bound to a `ctx` path. Every other
/// `containsKey(ctx...)` in the tree is `params.containsKey(...)` -- a lookup
/// table keyed by a field, which is not this and which other arms already
/// claim.
#[must_use]
pub fn parse_key_named_by_field(script: &str) -> Option<KeyNamedByField> {
    let bound = locals(script);

    // `<local>.containsKey(ctx.<key_from>)`
    let (head, rest) = script.split_once(".containsKey(")?;
    let name = head.rsplit(['(', ' ', '\n', '!']).next()?.trim();
    let path = bound
        .iter()
        .find(|(local, _)| local == name)
        .map(|(_, path)| path.clone())?;

    let (key_arg, after) = rest.split_once(')')?;
    let key_from = ctx_path_plain(key_arg.trim())?;

    // `ctx.<target> = <local>.get(ctx.<key_from>);`
    let getter = format!("{name}.get(");
    let (written, getter_rest) = after.split_once(&getter)?;
    let (get_arg, _) = getter_rest.split_once(')')?;
    if ctx_path_plain(get_arg.trim())? != key_from {
        return None;
    }

    let target = written.trim().trim_end_matches('=').trim();
    let target = target.rsplit(['\n', ' ', '{', ';', '}']).next()?;
    let target = ctx_path_plain(&clean_path(target))?;

    (!target.is_empty()).then_some(KeyNamedByField {
        path,
        key_from,
        target,
    })
}

/// The one `<acc>.<key> = <value_local>;` an arm writes, or `None`.
///
/// Exactly one: an arm writing two members is a record this does not build,
/// and taking the first would drop the other with nothing to show for it.
fn single_member_write(arm: &str, acc: &str, value_local: &str) -> Option<String> {
    let needle = format!("{acc}.");
    let mut found: Option<String> = None;
    for piece in arm.split(needle.as_str()).skip(1) {
        let (key, rhs) = piece.split_once('=')?;
        let key = key.trim();
        if key.is_empty() || !key.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return None;
        }
        if rhs.split(';').next()?.trim() != value_local {
            return None;
        }
        if found.is_some() {
            return None;
        }
        found = Some(key.to_owned());
    }
    found
}

/// Read the one-key wrap, or decline it.
///
/// The `instanceof Map` skip is REQUIRED rather than optional: without it a
/// nested object would be wrapped too, and writing `{value: {...}}` over a map
/// the vendor already built is a corruption no error reports.
#[must_use]
pub fn parse_wrap_entries(script: &str) -> Option<WrapEntries> {
    // `for (def <entry>: ctx.<path>.entrySet())`
    let (head, body) = script.split_once(".entrySet()")?;
    let (_, header) = head.rsplit_once("for (")?;
    let (entry, subject) = header
        .split_once(':')
        .or_else(|| header.split_once(" in "))?;
    let entry = entry.trim().strip_prefix("def ").unwrap_or(entry).trim();
    if entry.is_empty() || !entry.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let path = ctx_path_plain(&clean_path(subject.trim()))?;

    // `if (<entry>.getKey() == '<key>') { continue; }`, once per skipped key.
    let mut skip_keys = Vec::new();
    for piece in body.split(&format!("{entry}.getKey() ==")).skip(1) {
        let (literal, arm) = piece.split_once(')')?;
        if !arm.split_once('}')?.0.contains("continue;") {
            return None;
        }
        let key = literal.trim().trim_matches(['"', '\'']).to_owned();
        if key.is_empty() {
            return None;
        }
        skip_keys.push(key);
    }
    // Every `getKey()` the body spells has to be one of the skips just read.
    // The guard written the other way round -- `!=`, which KEEPS one key and
    // skips the rest -- is a different script, and reading none from it would
    // wrap the key it exists to protect.
    if body.matches(&format!("{entry}.getKey()")).count() != skip_keys.len() {
        return None;
    }

    // `def <v> = <entry>.getValue();`
    let (before_value, _) = body.split_once(&format!("= {entry}.getValue()"))?;
    let value_local = before_value.trim_end().rsplit([' ', '\n']).next()?.trim();
    if value_local.is_empty() {
        return None;
    }

    // `if (<v> instanceof Map) { continue; }` -- nested maps are left alone.
    let (_, after_map_guard) = body.split_once(&format!("{value_local} instanceof Map"))?;
    if !after_map_guard.split_once('}')?.0.contains("continue;") {
        return None;
    }

    // `Map <n> = [:];` -- the record each value is wrapped into.
    let (before_acc, after_acc) = body.split_once("= [:]")?;
    let acc = before_acc.trim_end().rsplit([' ', '\n']).next()?.trim();
    if acc.is_empty() || !acc.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    // `if (ctx.<flag> == "<literal>") { <n>.<a> = <v>; } else { <n>.<b> = <v>; }`
    let (_, condition) = after_acc.split_once("if (")?;
    let (test, arms) = condition.split_once(')')?;
    let (flag_text, literal) = test.split_once("==")?;
    let flag = ctx_path_plain(&clean_path(flag_text.trim()))?;
    let flag_value = literal.trim().trim_matches(['"', '\'']).to_owned();

    let (then_arm, else_part) = arms.split_once("else")?;
    let key_when_set = single_member_write(then_arm, acc, value_local)?;
    let key_when_unset =
        single_member_write(else_part.split("setValue").next()?, acc, value_local)?;
    if key_when_set == key_when_unset {
        return None;
    }

    // The wrapped record is what goes back into the entry.
    if !body.contains(&format!("{entry}.setValue({acc})")) {
        return None;
    }

    Some(WrapEntries {
        path,
        skip_keys,
        flag,
        flag_value,
        key_when_set,
        key_when_unset,
    })
}

/// Copy the entry the named field points at.
///
/// `containsKey` is key PRESENCE, so an entry explicitly set to null is copied
/// as null rather than skipped.
#[must_use]
pub fn key_named_by_field(event: &mut Event, pattern: &KeyNamedByField) -> bool {
    let Some(key) = event.get_str(&pattern.key_from).map(ToOwned::to_owned) else {
        return true;
    };
    let Some(value) = event
        .get_object(&pattern.path)
        .and_then(|map| map.get(key.as_str()))
        .cloned()
    else {
        return true;
    };
    let _ = event.set(&pattern.target, value);
    true
}

/// Wrap every scalar entry into its one-key record.
///
/// In place through `iter_mut`, so no key moves. The flag is read once, before
/// the walk, because it names a field outside the subtree being rewritten.
#[must_use]
pub fn wrap_entries(event: &mut Event, pattern: &WrapEntries) -> bool {
    let key = if event.get_str(&pattern.flag) == Some(pattern.flag_value.as_str()) {
        &pattern.key_when_set
    } else {
        &pattern.key_when_unset
    };

    let Some(Value::Object(map)) = pointer_mut(event, &pattern.path) else {
        return true;
    };
    for (name, value) in map.iter_mut() {
        // A map is left alone, and so is any key the script names.
        if value.is_object() || pattern.skip_keys.iter().any(|skip| skip == name) {
            continue;
        }
        let mut record = Map::with_capacity(1);
        record.insert(key.clone(), value.take());
        *value = Value::Object(record);
    }
    true
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[path = "map_entries_tests.rs"]
mod tests;
