// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `darktrace`'s activity periods, transcribed.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;
use crate::helpers::{painless_mul, painless_sub};

/// The untagged period reader in `darktrace/ai_analyst_alert`: one
/// `event.duration` per activity period, in nanoseconds.
///
/// An alert carries a period per burst of activity, so the field is a LIST
/// even where the alert has only one.
fn duration_from_periods(event: &mut Event, params: &Value) {
    let Some(factor) = params.get("NANOS_IN_A_MILLI_SECOND") else {
        return;
    };
    let durations = {
        let Some(periods) = event.get_array("json.periods") else {
            return;
        };
        periods
            .iter()
            .map(|period| {
                let end = period.get("end").unwrap_or(&Value::Null);
                let start = period.get("start").unwrap_or(&Value::Null);
                painless_mul(&painless_sub(end, start), factor)
            })
            .collect()
    };
    // Painless raises on `ctx.event.duration = x` where the event has no
    // `event` object, and the processor swallows it, so nothing is written.
    if !event.has("event") {
        return;
    }
    let _ = event.set("event.duration", Value::Array(durations));
}

/// Every darktrace script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "b89fb8ee527fb568dad33d03f591aa62f83430cbddf6259dc24b7951d8bd4cdc",
    source: "darktrace",
    name: "duration_from_periods",
    run: duration_from_periods,
}];
