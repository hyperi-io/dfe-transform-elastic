// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Scripts the `mongodb_atlas` package ships, transcribed by hand.

use serde_json::Value;

use super::Entry;
use dfe_core::event::Event;

/// Read a vendor code and write the sentence a `params` table gives it.
///
/// `getOrDefault(code, null)` writes an explicit null for a code the table does
/// not carry, which the later empty-value prune takes back out.
fn mapped_code(event: &mut Event, params: &Value, table: &str, source: &str, target: &str) {
    let Some(code) = event.get_string(source) else {
        return;
    };
    let mapped = params
        .get(table)
        .and_then(|codes| codes.get(&code))
        .cloned()
        .unwrap_or(Value::Null);
    let _ = event.set(target, mapped);
}

/// `mongodb_atlas/mongod_audit`, processor `informative_error_code`: name the
/// audit result code.
///
/// `ctx.mongodb_atlas.mongod_audit` is dereferenced without a null guard, so a
/// document whose renames never built it writes nothing at all.
fn informative_error_code(event: &mut Event, params: &Value) {
    if !matches!(
        event.get("mongodb_atlas.mongod_audit"),
        Some(Value::Object(_))
    ) {
        return;
    }
    mapped_code(
        event,
        params,
        "error_codes",
        "json.result",
        "mongodb_atlas.mongod_audit.result",
    );
}

/// `mongodb_atlas/mongod_database`, processor `informative_log_level`: name the
/// severity letter the server logged, which a later rename moves to
/// `log.level`.
fn informative_log_level(event: &mut Event, params: &Value) {
    mapped_code(event, params, "severity_levels", "json.s", "severity");
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "8b7601a8df0edcea7d843016346580b9b19126b6e20216af9a1d639ade8b1d8c",
        source: "mongodb_atlas",
        name: "informative_error_code",
        run: informative_error_code,
    },
    Entry {
        hash: "1f9eb1bffc1de602f17881b16443f62c0384ef82dcab89118931e652f4984523",
        source: "mongodb_atlas",
        name: "informative_log_level",
        run: informative_log_level,
    },
];
