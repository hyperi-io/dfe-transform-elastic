// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The argument count and executable `mysql_enterprise` takes off
//! `process.args`, transcribed.
//!
//! `cisco_secure_endpoint/event` ships the same script, so the two call sites
//! share this one runner.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// The untagged script in `mysql_enterprise/audit`: `process.args_count`, and
/// `process.executable` from the first argument.
fn args_count_and_executable(event: &mut Event, _params: &Value) {
    let Some(args) = event.get_array("process.args") else {
        return;
    };
    let count = args.len();
    let first = args.first().cloned();
    let _ = event.set("process.args_count", count);
    if let Some(first) = first {
        let _ = event.set("process.executable", first);
    }
}

/// Every `mysql_enterprise` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "f5b39befedd4c90a4916a5cbbc74a54e658ea8f1a5b201a317e844581bdadfd6",
    source: "mysql_enterprise",
    name: "args_count_and_executable",
    run: args_count_and_executable,
}];
