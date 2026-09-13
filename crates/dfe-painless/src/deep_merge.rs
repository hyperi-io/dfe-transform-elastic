// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One map merged recursively into another, on the vendor's own empty-wins
//! policy.
//!
//! `keycloak`'s log pipeline parses its ECS-formatted payload into `json`,
//! expands the dotted keys the vendor ships, and then lifts the whole tree onto
//! the root:
//!
//! ```painless
//! def mergeMaps(Map map1, Map map2) {
//!   for (def key : map2.keySet()) {
//!     if (!map1.containsKey(key)
//!         || map1[key] == null
//!         || map1[key] == ""
//!         || (map1[key] instanceof Map && map1[key].isEmpty())) {
//!       map1[key] = map2[key];
//!     } else if (map1[key] != map2[key]) {
//!       if (map1[key] instanceof Map && map2[key] instanceof Map) {
//!         map1[key] = mergeMaps(map1[key], map2[key]);
//!       } else if (map1[key] instanceof List) {
//!         def combined = new LinkedHashSet(map1[key]);
//!         if (map2[key] instanceof List) {
//!           combined.addAll(map2[key]);
//!         } else if (map2[key] != null) {
//!           combined.add(map2[key]);
//!         }
//!         map1[key] = new ArrayList(combined);
//!       }
//!     }
//!   }
//!   return map1;
//! }
//! mergeMaps(ctx, ctx.json);
//! ```
//!
//! Unmatched it costs the whole ECS capture -- all 18 events and every one of
//! their 216 fields. `message`, `log.*`, `host.*` and `process.*` reach the
//! root only through this merge, and every processor after it reads them there.
//!
//! **The empty-wins guard IS the policy.** `ecs.version` is already `8.11.0` on
//! the root when the merge runs and the payload carries `1.12.2`. The two maps
//! differ, so they recurse, and the held STRING is not replaced -- a merge that
//! let the arriving side win would ship the vendor's ECS version instead of the
//! pipeline's.
//!
//! `ti_opencti_indicator` ships a function of the same name and the opposite
//! guard, `if (map1.containsKey(key) && map1[key] != map2[key])`, taking the
//! arriving value in its ELSE arm and coercing a held scalar into a list on the
//! way. It also reaches the document through a `mergeListOfMaps` fold over seven
//! guarded targets rather than one call, so it is a different pattern and this
//! reader declines it.

use serde_json::{Map, Value};

use dfe_core::Event;

use crate::params::{clean_path, is_ctx_path};

/// A recursive merge, and the two maps the terminal call names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepMerge {
    /// The map merged INTO. EMPTY for `ctx` itself, which is the document root.
    into: String,
    /// The map merged FROM, which is a path because merging the whole document
    /// into something writes nothing anywhere.
    from: String,
}

/// The identifier a slice ENDS in, empty where it ends in anything else.
fn trailing_identifier(text: &str) -> &str {
    let width: usize = text
        .chars()
        .rev()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .map(char::len_utf8)
        .sum();
    &text[text.len() - width..]
}

/// One argument of the terminal call as a path, `ctx` itself reading as the
/// root.
///
/// A subscript is declined along with everything else that is not a dotted
/// path: `Event::get` resolves dots, and handing it `a[b]` looks up a key
/// spelled that way.
fn operand(text: &str) -> Option<String> {
    let text = clean_path(text.trim());
    if text == "ctx" {
        return Some(String::new());
    }
    let path = text.strip_prefix("ctx.")?;
    (is_ctx_path(path) && !path.contains(['[', ']'])).then(|| path.to_owned())
}

/// Read the merge, or decline it.
///
/// Every clause of the vendor's policy is matched in its own spelling, with the
/// script's parameter, key and set names substituted in. A body that spells a
/// DIFFERENT policy declines here rather than being run under this one, which
/// is the whole difference between the two `mergeMaps` in the tree.
#[must_use]
pub fn parse_deep_merge(script: &str) -> Option<DeepMerge> {
    // The merge has to be the WHOLE script: two mentions of `ctx`, and both of
    // them the terminal call's own arguments. Anything else is a script this
    // would reproduce half of.
    if script.matches("ctx").count() != 2 {
        return None;
    }

    // `def <name>(Map <first>, Map <second>) {`
    let (declaration, rest) = script.split_once("(Map ")?;
    let name = trailing_identifier(declaration);
    let (first, rest) = rest.split_once(", Map ")?;
    let (second, body) = rest.split_once(')')?;
    let (first, second) = (first.trim(), second.trim());
    if name.is_empty()
        || first.is_empty()
        || second.is_empty()
        || first == second
        || trailing_identifier(first) != first
        || trailing_identifier(second) != second
    {
        return None;
    }

    // `for (def <key> : <second>.keySet())` -- the loop walks the ARRIVING map,
    // so a script iterating the held one is a different merge.
    let (before_keys, _) = body.split_once(".keySet()")?;
    if trailing_identifier(before_keys) != second {
        return None;
    }
    let key = trailing_identifier(before_keys.rsplit_once(':')?.0.trim_end());
    if key.is_empty() {
        return None;
    }

    let held = format!("{first}[{key}]");
    let arriving = format!("{second}[{key}]");

    // Clause 1: absent, null, empty string, or empty map -- take the arriving
    // value. Painless spells an empty string literal either way.
    let empty_string = format!("{held} == ");
    if !body.contains(&format!("!{first}.containsKey({key})"))
        || !body.contains(&format!("{held} == null"))
        || !(body.contains(&format!("{empty_string}\"\""))
            || body.contains(&format!("{empty_string}''")))
        || !body.contains(&format!("{held} instanceof Map && {held}.isEmpty()"))
        || !body.contains(&format!("{held} = {arriving};"))
    {
        return None;
    }

    // Clause 2: the two differ, and both are maps -- recurse.
    if !body.contains(&format!("else if ({held} != {arriving})"))
        || !body.contains(&format!(
            "{held} instanceof Map && {arriving} instanceof Map"
        ))
        || !body.contains(&format!("{held} = {name}({held}, {arriving})"))
    {
        return None;
    }

    // Clause 3: the two differ and the HELD side is a list -- union, deduped,
    // insertion order kept, and the arriving side added whole when it is not a
    // list of its own.
    let (before_set, _) = body.split_once(&format!(" = new LinkedHashSet({held})"))?;
    let set = trailing_identifier(before_set);
    if set.is_empty()
        || !body.contains(&format!("else if ({held} instanceof List)"))
        || !body.contains(&format!("{arriving} instanceof List"))
        || !body.contains(&format!("{set}.addAll({arriving})"))
        || !body.contains(&format!("else if ({arriving} != null)"))
        || !body.contains(&format!("{set}.add({arriving})"))
        || !body.contains(&format!("{held} = new ArrayList({set})"))
    {
        return None;
    }

    // The terminal call, which is the LAST of the two -- the other is the
    // recursion, and its arguments are subscripts rather than paths.
    let (_, call) = script.rsplit_once(&format!("{name}("))?;
    let (arguments, _) = call.split_once(')')?;
    let (into, from) = arguments.split_once(',')?;
    let (into, from) = (operand(into)?, operand(from)?);

    // A map merged into itself writes nothing, and a map merged into its own
    // descendant is the move this cannot reproduce by cloning the source.
    if from.is_empty()
        || into == from
        || (!into.is_empty() && into.starts_with(&format!("{from}.")))
    {
        return None;
    }

    Some(DeepMerge { into, from })
}

/// The four values the script's first clause treats as nothing held.
///
/// An empty LIST is deliberately absent: the vendor guards only `instanceof Map
/// && isEmpty()`, so an empty list falls through to the union instead, and the
/// two differ the moment the arriving value is a scalar.
fn reads_as_empty(held: &Value) -> bool {
    match held {
        Value::Null => true,
        Value::String(text) => text.is_empty(),
        Value::Object(entries) => entries.is_empty(),
        _ => false,
    }
}

/// `LinkedHashSet::add` -- appended only where the value is not already there.
///
/// A linear scan rather than a hash set: `serde_json::Value` implements neither
/// `Hash` nor `Ord`, so there is no set to put it in without wrapping every
/// element first.
fn add_unique(into: &mut Vec<Value>, value: &Value) {
    if !into.contains(value) {
        into.push(value.clone());
    }
}

/// `new LinkedHashSet(held)`, then `addAll` or `add`.
///
/// The set is built from the HELD list first, so a duplicate already there is
/// dropped -- which the script only does once the two sides differ.
fn union_in_place(items: &mut Vec<Value>, arriving: &Value) {
    let mut combined: Vec<Value> = Vec::with_capacity(items.len() + 1);
    for item in items.iter() {
        add_unique(&mut combined, item);
    }
    match arriving {
        Value::Array(more) => {
            for item in more {
                add_unique(&mut combined, item);
            }
        }
        // `combined.add(...)` sits under the script's own `!= null`.
        Value::Null => {}
        one => add_unique(&mut combined, one),
    }
    *items = combined;
}

/// The vendor's merge, key by key.
///
/// Recursion is bounded by the document's own nesting, which `serde_json` caps
/// at 128 levels when it parses.
fn merge_into(into: &mut Map<String, Value>, from: &Map<String, Value>) {
    for (key, arriving) in from {
        let Some(held) = into.get_mut(key) else {
            into.insert(key.clone(), arriving.clone());
            continue;
        };
        if reads_as_empty(held) {
            *held = arriving.clone();
            continue;
        }
        if held == arriving {
            continue;
        }
        match held {
            Value::Object(held) => {
                if let Value::Object(arriving) = arriving {
                    merge_into(held, arriving);
                }
            }
            Value::Array(items) => union_in_place(items, arriving),
            // A held scalar is never replaced by an arriving one.
            _ => {}
        }
    }
}

/// Merge the map, or decline.
///
/// The source is CLONED because it usually sits inside the target -- keycloak
/// merges `ctx.json` into `ctx` -- so the merge reads it while writing over it.
/// Painless assigns by reference and leaves the two names sharing one object;
/// keycloak removes `json` immediately afterwards and never reads the second
/// name, so a copy is what the rest of the pipeline sees either way.
///
/// A source that is not a map means the PATH is wrong rather than the document
/// being empty: the call site gates this script on the source being an object,
/// so answering true there would report a success the pipeline does not have.
pub fn run_deep_merge(event: &mut Event, pattern: &DeepMerge) -> bool {
    let Some(Value::Object(from)) = event.get(&pattern.from).cloned() else {
        return false;
    };
    let into = if pattern.into.is_empty() {
        Some(event.as_value_mut())
    } else {
        crate::params::pointer_mut(event, &pattern.into)
    };
    let Some(Value::Object(into)) = into else {
        return false;
    };
    merge_into(into, &from);
    true
}

#[cfg(test)]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#` and carry the vendor's own escaped newlines.
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes,
    clippy::unreadable_literal
)]
#[path = "deep_merge_tests.rs"]
mod tests;
