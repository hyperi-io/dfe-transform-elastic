// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `couchbase`'s sample readings, transcribed.
//!
//! The stats endpoint answers with a minute of history per counter -- sixty
//! numbers a list -- and the package wants the latest reading. Each data
//! stream's script lifts the last element of the lists it cares about onto a
//! top-level field, which the renames a few processors later move into the
//! namespace.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// Where the query-index stats sit.
const QUERY_INDEX_SAMPLES: &str = "http.query_index.op.samples";

/// Where the cross-datacentre stats sit.
const XDCR_SAMPLES: &str = "http.xdcr.op.samples";

/// `couchbase/query_index`, the untagged `getLastElement` script in `default`:
/// the latest index and query readings.
fn query_index_samples(event: &mut Event, _params: &Value) {
    lift_last(
        event,
        QUERY_INDEX_SAMPLES,
        &[
            ("index_remaining_ram", "index_remaining_ram"),
            ("index_ram_percent", "index_ram_percent"),
            ("query_avg_req_time", "query_avg_req_time"),
            ("query_requests", "query_requests"),
            ("query_result_count", "query_result_count"),
            ("eventing/failed_count", "eventing_failed_count"),
        ],
    );
}

/// `couchbase/xdcr`, the untagged `getLastElement` script in `default`: the
/// latest replication readings.
fn xdcr_samples(event: &mut Event, _params: &Value) {
    lift_last(
        event,
        XDCR_SAMPLES,
        &[
            ("ep_dcp_xdcr_backoff", "backoff"),
            ("ep_dcp_xdcr_total_bytes", "bytes_total"),
            ("ep_dcp_xdcr_count", "count"),
            ("ep_dcp_xdcr_items_remaining", "items_remaining"),
            ("ep_dcp_xdcr_items_sent", "items_sent"),
            ("ep_oom_errors", "oom_errors"),
            ("ep_dcp_xdcr_producer_count", "producer_count"),
        ],
    );
}

/// Each named sample's last reading, written to the field beside it.
fn lift_last(event: &mut Event, samples: &str, wanted: &[(&str, &str)]) {
    let Some(samples) = event.get_object(samples).cloned() else {
        return;
    };
    for (sample, target) in wanted {
        if let Some(last) = last_reading(&samples, sample) {
            let _ = event.set(target, last);
        }
    }
}

/// The last element of one sample list.
///
/// An empty list indexes at -1 and raises in Painless, so nothing is written
/// for one.
fn last_reading(samples: &Map<String, Value>, name: &str) -> Option<Value> {
    match samples.get(name) {
        Some(Value::Array(readings)) => readings.last().cloned(),
        _ => None,
    }
}

/// Every `couchbase` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "48c871359ee6dd918dc81a26248a6192d75ee81c54a23ba2a537e4249697e84f",
        source: "couchbase",
        name: "query_index_samples",
        run: query_index_samples,
    },
    Entry {
        hash: "aad5025020d1f4996215b6081d52562f2bdb99afb31a0b81f1989fceef8a2ac3",
        source: "couchbase",
        name: "xdcr_samples",
        run: xdcr_samples,
    },
];
