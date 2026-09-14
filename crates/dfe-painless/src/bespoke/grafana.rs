// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `grafana`'s suffixed duration string, transcribed.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// `script_parse_duration` in `grafana/logs`: `event.duration` in nanoseconds
/// from the duration string the log line carries.
///
/// The two-letter suffixes are tested first, so the bare `s` arm only claims
/// what `ms`, `us` and `ns` have already declined. `µs` and `us` are the same
/// unit spelled two ways -- Grafana emits whichever the Go runtime chose.
fn duration_from_duration_str(event: &mut Event, _params: &Value) {
    let Some(duration) = event.get_string("_temp.duration_str") else {
        return;
    };
    let scaled = if let Some(head) = duration.strip_suffix("ms") {
        scale(head, 1_000_000.0)
    } else if let Some(head) = duration
        .strip_suffix("\u{b5}s")
        .or_else(|| duration.strip_suffix("us"))
    {
        scale(head, 1_000.0)
    } else if let Some(head) = duration.strip_suffix("ns") {
        scale(head, 1.0)
    } else if let Some(head) = duration.strip_suffix('s') {
        scale(head, 1_000_000_000.0)
    } else {
        return;
    };
    let Some(nanos) = scaled else {
        return;
    };
    // Painless raises on `ctx.event.duration = x` where the event has no
    // `event` object, and the processor swallows it, so nothing is written.
    if !event.has("event") {
        return;
    }
    let _ = event.set("event.duration", nanos);
}

/// The number in front of a unit suffix, scaled to nanoseconds.
///
/// Java's `Double.parseDouble` trims its input and raises on anything else,
/// which the caller reads as "write nothing".
fn scale(head: &str, factor: f64) -> Option<i64> {
    head.trim()
        .parse::<f64>()
        .ok()
        .map(|value| (value * factor) as i64)
}

/// Every grafana script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "7aacaa534b10b69ff2fce1bdbc0af3da054e5a46192e7a46bd4aa8569d312eb7",
    source: "grafana",
    name: "duration_from_duration_str",
    run: duration_from_duration_str,
}];
