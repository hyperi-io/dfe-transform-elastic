// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! extrahop's investigation identifier, transcribed.
//!
//! An investigation carries no id that is unique over time, so the pipeline
//! pairs the vendor's id with the instant the document was ingested.

use chrono::DateTime;
use serde_json::Value;

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_string;

/// `set_event_id_from_ID_and_ingest_timestamp_a000e3de` in
/// `extrahop/investigation`: `event.id` as `<id>-<epoch millis>`.
///
/// The script parses `temp.ingest_time` with `ISO_DATE_TIME` and takes the
/// instant's millisecond count, so a stamp with no offset raises there and
/// writes nothing here.
fn event_id_from_ingest_time(event: &mut Event, _params: &Value) {
    let Some(millis) = event
        .get_str("temp.ingest_time")
        .and_then(|stamp| DateTime::parse_from_rfc3339(stamp).ok())
        .map(|instant| instant.timestamp_millis())
    else {
        return;
    };
    let Some(id) = event
        .get("extrahop.investigation.id")
        .map(painless_to_string)
    else {
        return;
    };
    let _ = event.set("event.id", format!("{id}-{millis}"));
}

/// Every extrahop script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "7edce39b76f1ddd1d30c855dfa72aed85e5f28ab010cba39473fca17a3dab11b",
    source: "extrahop",
    name: "event_id_from_ingest_time",
    run: event_id_from_ingest_time,
}];
