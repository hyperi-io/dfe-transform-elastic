// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Scripts the `cisco_secure_endpoint` package ships, transcribed by hand.

use serde_json::Value;

use super::Entry;
use dfe_core::event::Event;

/// `cisco_secure_endpoint/event`, processor `set-process-args_count`: count
/// the arguments and take the executable off the head of the list.
///
/// `mysql_enterprise`'s `audit` stream ships the same script character for
/// character, so this one transcription serves both call sites.
fn set_process_args_count(event: &mut Event, _params: &Value) {
    let Some(args) = event.get_array("process.args") else {
        return;
    };
    let count = i64::try_from(args.len()).unwrap_or(i64::MAX);
    let first = args.first().cloned();
    let _ = event.set("process.args_count", count);
    if let Some(first) = first {
        let _ = event.set("process.executable", first);
    }
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "f5b39befedd4c90a4916a5cbbc74a54e658ea8f1a5b201a317e844581bdadfd6",
    source: "cisco_secure_endpoint",
    name: "set_process_args_count",
    run: set_process_args_count,
}];
