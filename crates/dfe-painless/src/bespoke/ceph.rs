// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `ceph`'s client operation total, transcribed.
//!
//! The OSD pool stats endpoint reports reads and writes per second separately,
//! and the package wants the one number a dashboard plots. The script adds the
//! two and leaves it at the top level for the rename that follows.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_add;

/// The write half of the rate.
const WRITES: &str = "json.client_io_rate.write_op_per_sec";

/// The read half.
const READS: &str = "json.client_io_rate.read_op_per_sec";

/// `ceph/osd_pool_stats`, the untagged total script in `default`:
/// `total_activity`, the two client operation rates added together.
///
/// The guard admits an event carrying only ONE of the halves, and the addition
/// then raises on the other -- so both have to be there for anything to be
/// written.
fn client_operation_total(event: &mut Event, _params: &Value) {
    if !rate(event, WRITES) && !rate(event, READS) {
        return;
    }
    let (Some(writes), Some(reads)) = (event.get(WRITES).cloned(), event.get(READS).cloned())
    else {
        return;
    };
    let total = painless_add(&painless_add(&Value::from(0), &writes), &reads);
    let _ = event.set("total_activity", total);
}

/// Whether one half is present, non-null and not the empty string.
fn rate(event: &Event, path: &str) -> bool {
    match event.get(path) {
        None | Some(Value::Null) => false,
        Some(Value::String(text)) => !text.is_empty(),
        Some(_) => true,
    }
}

/// Every `ceph` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "edfa0c5307cf75c5a7ed5d4cfa7724dd19b34cdb70322b44b6daeaddf1c4ad0c",
    source: "ceph",
    name: "client_operation_total",
    run: client_operation_total,
}];
