// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `cato_networks`'s audit reshaping: the numbered maps the vendor's dotted
//! keys expand into folded back to lists, the fractional epochs cut to whole
//! milliseconds, and every key turned from camelCase to `snake_case`.
//!
//! The vendor sends `change.After.adminRoles.0.id` as ONE key, so expanding the
//! dots leaves a map keyed `"0"`, `"1"` where the record is a list. The fold
//! walks that map in Java's own iteration order, which is neither the document
//! order nor a numeric one -- thirteen entries put `"11"` and `"12"` first.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::{java_map_values as java_values_in_order, painless_to_string};

/// The subtree all four scripts work over.
const AUDIT: &str = "cato_networks.audit";

/// The scalar epochs the first pass cuts.
const FRACTIONAL_SCALARS: [&str; 4] = [
    "change.After.startDate",
    "change.After.siteUsage.startDate",
    "change.After.socketInterface.lastModified",
    "change.After.socketConnectionConfiguration.lastModified",
];

/// Every numbered map folded to a list, with the members of each item that are
/// folded once the item is reachable.
const ARRAY_FOLDS: [(&str, &[&str]); 22] = [
    ("change.After.adminRoles", &[]),
    ("change.After.allowedItems", &[]),
    ("change.After.allowedUpdatableItems", &[]),
    ("change.After.from", &[]),
    ("change.After.model.links", &[]),
    ("change.After.model.possiblePlatform", &[]),
    (
        "change.After.socketInterface.metadata.supportedDestinations",
        &[],
    ),
    ("change.After.siteConnType.connectionTypeFamilies", &[]),
    (
        "change.After.siteConnType.socketModelTypeConfiguration.links",
        &[],
    ),
    (
        "change.After.siteConnType.socketModelTypeConfiguration.possiblePlatform",
        &[],
    ),
    ("change.After.permissions", &["actions"]),
    (
        "change.After.socketsSettings.primary.links",
        &["metadata.supportedDestinations"],
    ),
    ("change.After.socketsSettings.primary.supportedAddOns", &[]),
    (
        "change.After.socketsSettings.primary.supportedMigrations",
        &["links", "possibleAddOns", "possiblePlatform"],
    ),
    ("change.After.socketsSettings.primary.type.links", &[]),
    (
        "change.After.socketsSettings.primary.type.possiblePlatform",
        &[],
    ),
    (
        "change.After.socketConnectionConfiguration.socketInterfaces",
        &["metadata.supportedDestinations"],
    ),
    (
        "change.After.socketConnectionConfiguration.socketsSettings.primary.links",
        &["metadata.supportedDestinations"],
    ),
    (
        "change.After.socketConnectionConfiguration.socketsSettings.primary.supportedAddOns",
        &[],
    ),
    (
        "change.After.socketConnectionConfiguration.socketsSettings.primary.supportedMigrations",
        &["links", "possibleAddOns", "possiblePlatform"],
    ),
    (
        "change.After.socketConnectionConfiguration.socketsSettings.primary.type.links",
        &[],
    ),
    (
        "change.After.socketConnectionConfiguration.socketsSettings.primary.type.possiblePlatform",
        &[],
    ),
];

/// Every list whose items carry a fractional epoch, and which member holds it.
const FRACTIONAL_LISTS: [(&str, &[&str]); 7] = [
    (
        "change.After.allowedUpdatableItems",
        &["creationDateMilliseconds"],
    ),
    ("change.After.permissions", &["creationDateMilliseconds"]),
    (
        "change.After.socketConnectionConfiguration.socketInterfaces",
        &["lastModified", "creationDateMilliseconds"],
    ),
    (
        "change.After.socketsSettings.primary.links",
        &["lastModified"],
    ),
    (
        "change.After.adminRoles",
        &["creationDateMilliseconds", "role.creationDateMilliseconds"],
    ),
    ("change.After.allowedItems", &["creationDateMilliseconds"]),
    ("change.After.from", &["creationDateMilliseconds"]),
];

/// `script_remove_fractional_part` in `cato_networks/audit`: the four epoch
/// fields the record carries directly.
fn cut_fractional_scalars(event: &mut Event, _params: &Value) {
    let mut path = String::new();
    for tail in FRACTIONAL_SCALARS {
        set_audit_path(&mut path, tail);
        let Some(held) = event.get(&path) else {
            continue;
        };
        if held.is_null() {
            continue;
        }
        let cut = remove_fractional(held);
        let _ = event.update(&path, cut);
    }
}

/// `script_convert_to_array` in `cato_networks/audit`: every numbered map back
/// into the list it was sent as.
fn fold_numbered_maps(event: &mut Event, _params: &Value) {
    let mut path = String::new();
    for (tail, members) in ARRAY_FOLDS {
        set_audit_path(&mut path, tail);
        fold_one(event, &path);
        if members.is_empty() {
            continue;
        }
        let Some(mut items) = event.take_array(&path) else {
            continue;
        };
        for item in &mut items {
            for member in members {
                fold_member(item, member);
            }
        }
        let _ = event.update(&path, Value::Array(items));
    }
}

/// `script_remove_fractional_part_from_lists` in `cato_networks/audit`: the
/// same epoch cut, inside the lists the fold above produced.
fn cut_fractional_in_lists(event: &mut Event, _params: &Value) {
    let mut path = String::new();
    for (tail, members) in FRACTIONAL_LISTS {
        set_audit_path(&mut path, tail);
        let Some(mut items) = event.take_array(&path) else {
            continue;
        };
        for item in &mut items {
            for member in members {
                cut_member(item, member);
            }
        }
        let _ = event.update(&path, Value::Array(items));
    }
}

/// `script_convert_camelcase_to_snake_case` in `cato_networks/audit`.
///
/// A key holding `@` is dropped along with what it holds, which is how the
/// vendor's own `@`-prefixed metadata is kept out of the rewritten record.
fn convert_camel_case_keys(event: &mut Event, _params: &Value) {
    let Some(audit) = event.get(AUDIT) else {
        return;
    };
    if audit.is_null() {
        return;
    }
    // `convert` rebuilds rather than borrowing, so the read ends here and the
    // subtree needs no copy of its own.
    let converted = convert(audit);
    let _ = event.update(AUDIT, converted);
}

/// The document path one audit tail names, written into a reused buffer.
///
/// The three passes walk 33 constant tails between them, so a `format!` each
/// is 33 allocations an event for paths the tables already determine.
fn set_audit_path(path: &mut String, tail: &str) {
    path.clear();
    path.push_str(AUDIT);
    path.push('.');
    path.push_str(tail);
}

/// Fold the map at one path into the list of its values.
fn fold_one(event: &mut Event, path: &str) {
    let Some(held) = event.get(path) else {
        return;
    };
    let Some(map) = held.as_object() else {
        return;
    };
    let folded = Value::Array(java_map_values(map));
    let _ = event.update(path, folded);
}

/// The same fold on a member of one list item, by a dotted member path.
fn fold_member(item: &mut Value, member: &str) {
    let Some(slot) = member_slot(item, member) else {
        return;
    };
    let Some(map) = slot.as_object() else {
        return;
    };
    *slot = Value::Array(java_map_values(map));
}

/// Cut the fractional part of a member of one list item.
fn cut_member(item: &mut Value, member: &str) {
    let Some(slot) = member_slot(item, member) else {
        return;
    };
    if slot.is_null() {
        return;
    }
    *slot = remove_fractional(slot);
}

/// The slot a dotted member path names inside one list item.
fn member_slot<'a>(item: &'a mut Value, member: &str) -> Option<&'a mut Value> {
    let mut current = item;
    for segment in member.split('.') {
        current = current.as_object_mut()?.get_mut(segment)?;
    }
    Some(current)
}

/// A map's values in the order the vendor's own parsed map iterates them.
fn java_map_values(map: &Map<String, Value>) -> Vec<Value> {
    java_values_in_order(map, parsed_table_size(map.len()))
}

/// The table a map Elasticsearch built from parsed content holds `entries` in.
///
/// `HashMap(Map)` sizes the table for the WHOLE map at once -- `size / 0.75`
/// plus one, rounded up to a power of two -- rather than growing it a put at a
/// time, so twelve entries sit in a table of 32 where
/// [`crate::helpers::java_table_size`]'s grown-by-adds reading would give 16.
/// The twelve `links` of a socket migration are the proof: at 16 they come out
/// `lan` first and Elasticsearch has `usb`.
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)] // Java's own float arithmetic.
fn parsed_table_size(entries: usize) -> usize {
    let wanted = ((entries as f32 / 0.75) + 1.0) as usize;
    let mut capacity = 1usize;
    while capacity < wanted {
        capacity <<= 1;
    }
    capacity
}

/// One value with its fractional part dropped, the way the vendor's own helper
/// drops it.
#[allow(clippy::cast_possible_truncation)] // Java's own `(long)` cast.
fn remove_fractional(value: &Value) -> Value {
    match value {
        Value::Number(number) => match number.as_i64() {
            Some(whole) => Value::from(whole),
            None => Value::from(number.as_f64().unwrap_or_default() as i64),
        },
        Value::String(text) if text.is_empty() => value.clone(),
        other => match painless_to_string(other).parse::<f64>() {
            Ok(parsed) => Value::from(parsed as i64),
            // Painless raises here, so nothing further of the script is written.
            Err(_) => other.clone(),
        },
    }
}

/// One value rewritten: a map by its keys, a list member by member, anything
/// else left alone.
fn convert(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut rebuilt = Map::with_capacity(map.len());
            for (key, held) in map {
                if key.contains('@') {
                    continue;
                }
                rebuilt.insert(camel_to_snake(key), convert(held));
            }
            Value::Object(rebuilt)
        }
        Value::Array(items) => Value::Array(items.iter().map(convert).collect()),
        other => other.clone(),
    }
}

/// One key from camelCase to `snake_case`.
///
/// The break goes in only where a capital FOLLOWS a non-capital, so a run of
/// capitals is lowercased whole and `IPAddress` comes out `ipaddress`.
fn camel_to_snake(key: &str) -> String {
    let mut out = String::with_capacity(key.len() + 4);
    let mut last_was_upper = false;
    for (at, current) in key.chars().enumerate() {
        if current.is_uppercase() {
            if at > 0 && !last_was_upper {
                out.push('_');
            }
            out.extend(current.to_lowercase());
            last_was_upper = true;
        } else {
            out.push(current);
            last_was_upper = false;
        }
    }
    out
}

/// Every `cato_networks` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "a47ae8f963cfea0c91da8150de9aa6794a09a086a12136eccd1092ef9f122cf6",
        source: "cato_networks",
        name: "cut_fractional_scalars",
        run: cut_fractional_scalars,
    },
    Entry {
        hash: "6f484ffff5d6ce6d4acab2b30193a2506c6534649adcc820a435ab6fb714e084",
        source: "cato_networks",
        name: "fold_numbered_maps",
        run: fold_numbered_maps,
    },
    Entry {
        hash: "0dbd44750fa9accd1b0220c7e5757b13aaab21b1517f53a54301d8c48c188691",
        source: "cato_networks",
        name: "cut_fractional_in_lists",
        run: cut_fractional_in_lists,
    },
    Entry {
        hash: "4ef0d337bb6aa00f8f45527ff1c7cae035b59d47348c888a97aebd13c4b6647f",
        source: "cato_networks",
        name: "convert_camel_case_keys",
        run: convert_camel_case_keys,
    },
];
