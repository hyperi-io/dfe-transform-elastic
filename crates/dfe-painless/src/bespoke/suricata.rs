// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `suricata`'s two eve passes: the trailing dot trimmed off a TLS server
//! name, and the flow's span written as a nanosecond duration.

use serde_json::Value;

use super::Entry;
use dfe_core::event::Event;

/// `suricata/eve`, processor `suricata_trim_tls_sni`:
/// `suricata.eve.tls.sni` without the trailing dot a fully qualified name
/// carries.
///
/// Everything ECS reads the name from -- `destination.domain`,
/// `related.hosts`, `tls.client.server_name` -- is copied out of it after this
/// runs, so one trim reaches all four.
fn trim_tls_sni(event: &mut Event, _params: &Value) {
    let Some(sni) = event.get_string("suricata.eve.tls.sni") else {
        return;
    };
    let Some(trimmed) = sni.strip_suffix('.') else {
        return;
    };
    let trimmed = trimmed.to_owned();
    event.update("suricata.eve.tls.sni", trimmed);
}

/// `suricata/eve`, the untagged flow-span processor: `event.duration` in
/// nanoseconds.
///
/// The script parses both ends inside a `try` and writes nothing where either
/// fails, so a flow with no end keeps no duration. A span that runs backwards
/// is dropped as well.
fn flow_duration(event: &mut Event, _params: &Value) {
    if !event.has_value("event") {
        return;
    }
    let (Some(start), Some(end)) = (
        instant_nanos(event, "event.start"),
        instant_nanos(event, "event.end"),
    ) else {
        return;
    };
    if start > end {
        return;
    }
    let _ = event.set("event.duration", end - start);
}

/// One ISO-8601 instant as nanoseconds since the epoch, or `None` where
/// `Instant.parse` would have raised.
fn instant_nanos(event: &Event, path: &str) -> Option<i64> {
    chrono::DateTime::parse_from_rfc3339(event.get_str(path)?)
        .ok()?
        .timestamp_nanos_opt()
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "cf62239bdb15883ec349cf9e658e797c2c4b8b21f4f01d98058c0ee4e546da1f",
        source: "suricata",
        name: "trim_tls_sni",
        run: trim_tls_sni,
    },
    Entry {
        hash: "8e303327b531f01b62d6873b2d874615ea57eeaace24002330dd89cf74e744b4",
        source: "suricata",
        name: "flow_duration",
        run: flow_duration,
    },
];
