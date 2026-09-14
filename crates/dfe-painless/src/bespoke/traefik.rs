// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `traefik`'s header gathering, transcribed.
//!
//! The JSON access log flattens every header into one key per header, prefixed
//! by the side it came from -- `request_Accept`, `downstream_Content-Type`,
//! `origin_Date`. The script sorts them back into the three ECS maps and
//! lower-cases each name, and it runs AFTER the renames, so a header the
//! pipeline has already lifted out of `temp` is no longer there to gather.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// Where the `json` processor put the parsed access log.
const TEMP: &str = "temp";

/// `traefik/access`, the untagged header script in `format_json`: the three
/// header maps, one per side of the proxy.
fn collect_headers(event: &mut Event, _params: &Value) {
    let Some(temp) = event.get_object(TEMP) else {
        return;
    };

    let mut downstream = Map::new();
    let mut request = Map::new();
    let mut origin = Map::new();
    for (name, value) in temp {
        // `String.replace` takes out EVERY occurrence, not just the prefix.
        if name.starts_with("downstream_") {
            downstream.insert(header_name(name, "downstream_"), value.clone());
        } else if name.starts_with("request_") {
            request.insert(header_name(name, "request_"), value.clone());
        } else if name.starts_with("origin_") {
            origin.insert(header_name(name, "origin_"), value.clone());
        }
    }

    if !request.is_empty() {
        let _ = event.set("http.request.headers", Value::Object(request));
    }
    if !downstream.is_empty() {
        let _ = event.set("http.response.headers", Value::Object(downstream));
    }
    if !origin.is_empty() {
        let _ = event.set("traefik.access.origin.headers", Value::Object(origin));
    }
}

/// One header's name with its side stripped off and lower-cased.
fn header_name(name: &str, prefix: &str) -> String {
    name.replace(prefix, "").to_lowercase()
}

/// Every `traefik` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "597ef01e6f2e50f4aa63b8a4df84ce4446f6d71a8ac9883c07eeaa3dbf401ea8",
    source: "traefik",
    name: "collect_headers",
    run: collect_headers,
}];
