// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `zoom`'s phone webhook: the call's own span copied onto ECS and measured.

use serde_json::Value;

use super::Entry;
use dfe_core::event::Event;

/// Where the call's end always sits.
const CALL_END: &str = "zoom.phone.call_end_time";

/// `zoom/webhook`, the untagged phone processor for a call that was never
/// answered: `event.start`, `event.end` and `event.duration`.
///
/// The span runs from the moment the phone started ringing, which is the only
/// start such a call has.
fn ringing_call_span(event: &mut Event, _params: &Value) {
    call_span(event, "zoom.phone.ringing_start_time");
}

/// `zoom/webhook`, the untagged phone processor for a call that was answered:
/// `event.start`, `event.end` and `event.duration`.
///
/// The span runs from the answer, so the time the caller spent waiting is not
/// counted as call time.
fn answered_call_span(event: &mut Event, _params: &Value) {
    call_span(event, "zoom.phone.answer_start_time");
}

/// Copy a call's start and end onto `event.*` and write the span between them
/// in nanoseconds.
///
/// Painless raises where `ctx.event` is absent, so a document without one is
/// left alone rather than given a fresh map.
fn call_span(event: &mut Event, start_path: &str) {
    if !event.has("event") {
        return;
    }
    let (Some(start), Some(end)) = (event.get(start_path).cloned(), event.get(CALL_END).cloned())
    else {
        return;
    };
    let _ = event.set("event.start", start);
    let _ = event.set("event.end", end);
    let (Some(from), Some(to)) = (
        instant_nanos(event, "event.start"),
        instant_nanos(event, "event.end"),
    ) else {
        return;
    };
    let _ = event.set("event.duration", to - from);
}

/// One ISO-8601 instant as nanoseconds since the epoch, or `None` where
/// `ZonedDateTime.parse` would have raised.
fn instant_nanos(event: &Event, path: &str) -> Option<i64> {
    chrono::DateTime::parse_from_rfc3339(event.get_str(path)?)
        .ok()?
        .timestamp_nanos_opt()
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "bd6d1866042593995a2ab3687de6c3e96d110536c721d88d4896d83e65a26e55",
        source: "zoom",
        name: "ringing_call_span",
        run: ringing_call_span,
    },
    Entry {
        hash: "54139bce8752b415925ad4faa9a724d192c2615b80dc51d5b66478856d68a828",
        source: "zoom",
        name: "answered_call_span",
        run: answered_call_span,
    },
];
