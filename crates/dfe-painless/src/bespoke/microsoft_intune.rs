// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `microsoft_intune`'s audit pass: the vendor's `Key = ...Value = ...` lines
//! read into a map beside the text they came from.

use serde_json::{Map, Value};

use super::Entry;
use dfe_core::event::Event;

/// The field the vendor delivers as one CRLF-separated block of text.
const DETAIL: &str = "microsoft_intune.audit.properties.additional_detail";

/// `microsoft_intune/audit`, processor `script_parse_additional_detail`:
/// `additional_detail.original` and one `additional_detail.parsed.<key>` per
/// line.
///
/// A line carries its key and its value in one run -- `Key = <key>Value =
/// <value>` -- so the key is what is left of the first `Value = ` once the
/// `Key = ` marker is taken out of it. The script splits the block with a
/// regex, which raises on anything but a string, so a detail the vendor sent
/// as an object is left alone.
fn parse_additional_detail(event: &mut Event, _params: &Value) {
    let Some(content) = event.get_string(DETAIL) else {
        return;
    };
    let mut parsed = Map::new();
    for line in content.split("\r\n") {
        if !line.contains("Key = ") || !line.contains("Value = ") {
            continue;
        }
        let mut halves = line.splitn(2, "Value = ");
        let (Some(head), Some(tail)) = (halves.next(), halves.next()) else {
            continue;
        };
        let key = head.replace("Key = ", "");
        parsed.insert(key.trim().to_owned(), Value::from(tail.trim()));
    }
    let mut detail = Map::new();
    detail.insert("original".to_owned(), Value::from(content));
    detail.insert("parsed".to_owned(), Value::Object(parsed));
    event.update(DETAIL, Value::Object(detail));
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "316d1eae5cb6f99bfdad0f9393d4ec1d17edc1cea9dd0d2baa2372b31d6a79f9",
    source: "microsoft_intune",
    name: "parse_additional_detail",
    run: parse_additional_detail,
}];
