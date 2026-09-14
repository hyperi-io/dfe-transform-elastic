// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `elastic_package_registry`'s process uptime, transcribed.

use chrono::DateTime;
use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// `script_1e179be8` in `elastic_package_registry/metrics`: whole seconds
/// between the process start and the scrape.
///
/// The difference is taken in milliseconds and divided by a thousand, which
/// truncates -- 114,162 ms is 114 seconds, not 115.
fn uptime_seconds(event: &mut Event, _params: &Value) {
    if !event.has("package_registry.start_time") || !event.has("@timestamp") {
        return;
    }
    let Some(created) = event.get_str("@timestamp").and_then(parse_instant) else {
        return;
    };
    let Some(start) = event
        .get_str("package_registry.start_time")
        .and_then(parse_instant)
    else {
        return;
    };
    let _ = event.set("package_registry.uptime", (created - start) / 1000);
}

/// One `ZonedDateTime.parse`, as milliseconds since the epoch.
fn parse_instant(text: &str) -> Option<i64> {
    DateTime::parse_from_rfc3339(text)
        .ok()
        .map(|stamp| stamp.timestamp_millis())
}

/// Every `elastic_package_registry` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "93452588d813fdef39523447eab2646e8bc8127209fca2ea2c630b93343527df",
    source: "elastic_package_registry",
    name: "uptime_seconds",
    run: uptime_seconds,
}];
