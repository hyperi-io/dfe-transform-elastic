// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `ti_misp`'s microsecond-to-millisecond trim on the attribute sighting
//! stamps, transcribed.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// The two stamps the vendor sends in microseconds.
const SIGHTINGS: [&str; 2] = ["misp.attribute.first_seen", "misp.attribute.last_seen"];

/// The `threat_attributes` pipeline's untagged trim: a SIXTEEN-character epoch
/// loses its last three digits.
///
/// The date processor behind this reads milliseconds, so an untrimmed stamp
/// parses a thousand times too far out -- 2020 becomes the year 52101.
fn microseconds_to_milliseconds(event: &mut Event, _params: &Value) {
    for path in SIGHTINGS {
        let Some(stamp) = event.get_str(path) else {
            continue;
        };
        if stamp.chars().count() != 16 {
            continue;
        }
        let trimmed: String = stamp.chars().take(13).collect();
        let _ = event.update(path, trimmed);
    }
}

/// Every `ti_misp` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "c251879607a2bf21fb4980093185015f81bf2490a5544787546735df6fe0ce0d",
    source: "ti_misp",
    name: "microseconds_to_milliseconds",
    run: microseconds_to_milliseconds,
}];
