// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `rubrik`'s two managed-volume derivations, transcribed.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_sub;

/// Where the managed-volume metrics live.
const MANAGED_VOLUMES: &str = "rubrik.managed_volumes";

/// The managed-volume pipeline's untagged script: `free_size.bytes` as
/// volume less used, and `pending_snapshots.count` as total less scheduled.
///
/// Both fields are written on EVERY event, zero where the inputs are missing,
/// so a volume with no snapshot distribution still carries a count.
fn managed_volume_derivations(event: &mut Event, _params: &Value) {
    // Painless raises on the assignment where the parent map is absent.
    if !event.has(MANAGED_VOLUMES) {
        return;
    }

    let volume = format!("{MANAGED_VOLUMES}.volume_size.bytes");
    let used = format!("{MANAGED_VOLUMES}.used_size.bytes");
    let free = if event.has_value(&volume) && event.has_value(&used) {
        painless_sub(
            event.get(&volume).unwrap_or(&Value::Null),
            event.get(&used).unwrap_or(&Value::Null),
        )
    } else {
        Value::from(0)
    };
    let mut free_size = Map::new();
    free_size.insert("bytes".to_owned(), free);
    let _ = event.set(
        &format!("{MANAGED_VOLUMES}.free_size"),
        Value::Object(free_size),
    );

    let pending = if event.has_value("response.snapshotDistribution") {
        let total = counted(event, "response.snapshotDistribution.totalCount");
        let scheduled = counted(event, "response.snapshotDistribution.scheduledCount");
        painless_sub(&total, &scheduled)
    } else {
        Value::from(0)
    };
    let mut pending_snapshots = Map::new();
    pending_snapshots.insert("count".to_owned(), pending);
    let _ = event.set(
        &format!("{MANAGED_VOLUMES}.pending_snapshots"),
        Value::Object(pending_snapshots),
    );
}

/// One snapshot count, zero where the vendor sent nothing or an explicit null.
fn counted(event: &Event, path: &str) -> Value {
    event
        .get(path)
        .filter(|held| !held.is_null())
        .cloned()
        .unwrap_or(Value::from(0))
}

/// Every rubrik script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "5e47ab508e98ebec73611abea63af8284d1b2699ef642d4bd82416b0b290e0f4",
    source: "rubrik",
    name: "managed_volume_derivations",
    run: managed_volume_derivations,
}];
