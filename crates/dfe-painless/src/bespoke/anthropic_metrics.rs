// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Scripts the `anthropic_metrics` package ships, transcribed by hand.

use serde_json::Value;

use super::Entry;
use dfe_core::event::Event;

/// `anthropic_metrics/usage`, processor `sum_cache_creation_tokens`: add the
/// five-minute and one-hour ephemeral cache writes into one total.
///
/// Each term is gated on `containsKey`, so a key present and null contributes
/// nothing and the total is still written.
fn sum_cache_creation_tokens(event: &mut Event, _params: &Value) {
    let mut total: i64 = 0;
    for key in [
        "json.cache_creation.ephemeral_5m_input_tokens",
        "json.cache_creation.ephemeral_1h_input_tokens",
    ] {
        if event.has(key) {
            total = total.saturating_add(event.get_as_i64(key).unwrap_or_default());
        }
    }
    let _ = event.set("anthropic.usage.cache_creation_input_tokens", total);
}

/// `anthropic_metrics/cost` and `/usage`, the workspace default: a workspace id
/// the API sent as null names the organisation's default workspace, and every
/// other null member is dropped so the renames downstream skip it.
fn default_workspace_id(event: &mut Event, _params: &Value) {
    if event.has("json.workspace_id") && !event.has_value("json.workspace_id") {
        event.update("json.workspace_id", "Default");
    }
    let Some(Value::Object(members)) = event.get("json") else {
        return;
    };
    let nulls: Vec<String> = members
        .iter()
        .filter(|(_, value)| value.is_null())
        .map(|(key, _)| key.clone())
        .collect();
    let mut path = String::from("json.");
    let mark = path.len();
    for key in nulls {
        path.truncate(mark);
        path.push_str(&key);
        event.remove(&path);
    }
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "811ca1e7c7d937af1ebac107eeaa1482f11715017461656ea3ad56c809a000e5",
        source: "anthropic_metrics",
        name: "sum_cache_creation_tokens",
        run: sum_cache_creation_tokens,
    },
    Entry {
        hash: "20f909e105ccd81059eaf32797e146f533ba06a5db97c877ba9607b5b0dfcb18",
        source: "anthropic_metrics",
        name: "default_workspace_id",
        run: default_workspace_id,
    },
];
