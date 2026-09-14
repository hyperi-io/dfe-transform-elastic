// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `ti_ticura`'s enrichment merge, transcribed.
//!
//! An indicator carries country, industry and actor lists twice -- the feed's
//! own and whatever the enrichment added -- and the pipeline folds each pair
//! into one deduplicated, sorted list under `merged`.

use std::cmp::Ordering;

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::{dedup_array, painless_cmp};

/// The map the merged lists are written into.
const MERGED: &str = "ticura.indicator.merged";

/// Each merge: the feed's own list, the enrichment's, and where the pair goes.
const MERGES: [(&str, &str, &str); 3] = [
    (
        "ticura.indicator.countries",
        "ticura.indicator.additional_info.countries",
        "ticura.indicator.merged.countries",
    ),
    (
        "ticura.indicator.industries",
        "ticura.indicator.additional_info.industries",
        "ticura.indicator.merged.industries",
    ),
    (
        "ticura.indicator.actors",
        "ticura.indicator.additional_info.actors",
        "ticura.indicator.merged.actors",
    ),
];

/// `painless_merge_enrichment` in `ti_ticura/indicator`: `merged.countries`,
/// `merged.industries` and `merged.actors`.
///
/// A pair that contributes nothing still writes an empty list, which the
/// pipeline's closing empty-value prune removes -- so an indicator with no
/// actors ends with no `merged.actors` key at all.
fn merge_enrichment(event: &mut Event, _params: &Value) {
    // The script reads `ctx.ticura.indicator.merged` with no safe navigation,
    // so an indicator missing its own namespace raises there and writes
    // nothing here either.
    if !event.get("ticura.indicator").is_some_and(Value::is_object) {
        return;
    }
    if !event.get(MERGED).is_some_and(Value::is_object) {
        let _ = event.set(MERGED, Value::Object(Map::new()));
    }
    for (own, enriched, target) in MERGES {
        let merged = merged_list(event, own, enriched);
        let _ = event.set(target, Value::Array(merged));
    }
}

/// One pair, deduplicated by value and then sorted the way `sort(null)` sorts.
fn merged_list(event: &Event, own: &str, enriched: &str) -> Vec<Value> {
    let mut merged = Vec::new();
    for path in [own, enriched] {
        match event.get(path) {
            None | Some(Value::Null) => {}
            Some(Value::Array(items)) => merged.extend(items.iter().cloned()),
            Some(single) => merged.push(single.clone()),
        }
    }
    dedup_array(&mut merged);
    merged.sort_by(natural_order);
    merged
}

/// Java's natural ordering for what these lists hold.
///
/// `String.compareTo` walks UTF-16 units rather than the UTF-8 bytes Rust's own
/// `Ord` compares, and the two disagree above the basic plane.
fn natural_order(one: &Value, two: &Value) -> Ordering {
    match (one, two) {
        (Value::String(one), Value::String(two)) => one.encode_utf16().cmp(two.encode_utf16()),
        _ => painless_cmp(one, two).unwrap_or(Ordering::Equal),
    }
}

/// Every `ti_ticura` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "c7d0c4616684f1d92d3d47b6860e27fe6e23808f4030cb808d9c39c65356ed49",
    source: "ti_ticura",
    name: "merge_enrichment",
    run: merge_enrichment,
}];
