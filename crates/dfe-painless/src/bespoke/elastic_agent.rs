// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Scripts the `elastic_agent` package ships, transcribed by hand.

use serde_json::Value;

use super::Entry;
use dfe_core::event::Event;

/// `elastic_agent/status_change_logs`, processor `script_derive_health_status`:
/// fold the agent's status into the three health words, passing anything
/// unrecognised through unchanged.
fn derive_health_status(event: &mut Event, _params: &Value) {
    let Some(status) = event.get_str("status") else {
        return;
    };
    let health = match status {
        "online" => "healthy",
        "error" | "degraded" => "unhealthy",
        "updating" | "enrolling" | "unenrolling" => "updating",
        other => other,
    }
    .to_string();
    let _ = event.set("health_status", health);
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "c8fe8b1355f6749cc12c4ed0c742e186253ea883bb336d37082aabb244be5fec",
    source: "elastic_agent",
    name: "derive_health_status",
    run: derive_health_status,
}];
