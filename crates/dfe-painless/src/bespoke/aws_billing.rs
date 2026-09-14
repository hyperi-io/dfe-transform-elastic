// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `aws_billing`'s two key-sorted renderings of a parsed tag map back into a
//! JSON string.

use serde_json::Value;

use super::Entry;
use crate::helpers::painless_to_string;
use dfe_core::event::Event;

/// `aws_billing/cur`, processor `sort_tags_alphabetically`:
/// `aws_billing.cur.resource_tags`.
fn sort_tags_alphabetically(event: &mut Event, _params: &Value) {
    render_sorted(event, "temp_tags_obj", "aws_billing.cur.resource_tags");
}

/// `aws_billing/cur`, processor `sort_cost_categories_alphabetically`:
/// `aws_billing.cur.cost_category`.
fn sort_cost_categories_alphabetically(event: &mut Event, _params: &Value) {
    render_sorted(
        event,
        "temp_cost_category_obj",
        "aws_billing.cur.cost_category",
    );
}

/// Render the map at `from` as a JSON object with its keys in order, and store
/// the text at `to`.
///
/// The script builds the text a character at a time rather than serialising,
/// so nothing is escaped and a value is whatever `StringBuilder.append` makes
/// of it. `new TreeMap()` orders by `String.compareTo`, which reads UTF-16
/// units where Rust reads UTF-8 bytes -- the two agree on everything below the
/// supplementary planes.
fn render_sorted(event: &mut Event, from: &str, to: &str) {
    let Some(source) = event.get_object(from) else {
        return;
    };
    let mut sorted: Vec<(&String, &Value)> = source.iter().collect();
    sorted.sort_by_key(|(key, _)| *key);
    let mut out = String::from("{");
    for (position, (key, value)) in sorted.into_iter().enumerate() {
        if position > 0 {
            out.push(',');
        }
        out.push('"');
        out.push_str(key);
        out.push_str("\":\"");
        out.push_str(&painless_to_string(value));
        out.push('"');
    }
    out.push('}');

    // Painless raises on the assignment where the parent map is absent, and a
    // raise here would abort the rest of the pipeline.
    let parent = to.rsplit_once('.').map_or(to, |(head, _)| head);
    if !event.has(parent) {
        return;
    }
    let _ = event.set(to, out);
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "ae1155fe531af748cca124130451468b0a65a69474ab78198678c2b4190dcdb6",
        source: "aws_billing",
        name: "sort_tags_alphabetically",
        run: sort_tags_alphabetically,
    },
    Entry {
        hash: "a65f27e7cee4a52e0f3e228a440607128348cadc374e802a91414ae41536a88c",
        source: "aws_billing",
        name: "sort_cost_categories_alphabetically",
        run: sort_cost_categories_alphabetically,
    },
];
