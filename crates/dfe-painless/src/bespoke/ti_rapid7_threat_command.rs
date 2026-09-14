// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `ti_rapid7_threat_command`'s indicator expiry, including the retirement
//! that ends the window at the event itself.

use serde_json::Value;

use super::Entry;
use dfe_core::date_formats::iso8601_plus;
use dfe_core::event::Event;

/// The sightings the window is measured from, first present winning.
const BASES: [&str; 3] = [
    "threat.indicator.modified_at",
    "threat.indicator.last_seen",
    "threat.indicator.first_seen",
];

/// How long an indicator lasts when the configured span names a unit with no
/// arm in the script.
const DEFAULT_DAYS: i64 = 90;

/// `ti_rapid7_threat_command/ioc`, processor `script-default-deleted_at`:
/// `rapid7.tc.ioc.deleted_at`.
///
/// A RETIRED indicator expires at this event's own timestamp, whatever the
/// configured span says; every other one expires that span after whichever
/// sighting the ladder finds first.
fn ioc_deleted_at(event: &mut Event, _params: &Value) {
    if event
        .get_str("rapid7.tc.ioc.status")
        .is_some_and(|status| status.to_lowercase().contains("retire"))
    {
        let Some(at) = event.get_string("@timestamp") else {
            return;
        };
        if let Some(expiry) = iso8601_plus(&at, 'd', 0, 0) {
            let _ = event.set("rapid7.tc.ioc.deleted_at", expiry);
        }
        return;
    }

    let Some(base) = BASES.iter().find_map(|path| event.get_as_string(path)) else {
        return;
    };
    // A span held as anything but a string never reaches the unit ladder, so
    // it takes the default the same way an unreadable unit does.
    let configured = event.get_string("_conf.ioc_expiration_duration");
    let expiry = match configured.as_deref().and_then(split_span) {
        Some((unit, Ok(count))) => iso8601_plus(&base, unit, count, 0),
        // `Long.parseLong` throws on a count that is not a number, which takes
        // the whole script with it.
        Some((_, Err(_))) => return,
        None => iso8601_plus(&base, 'd', DEFAULT_DAYS, 0),
    };
    if let Some(expiry) = expiry {
        let _ = event.set("rapid7.tc.ioc.deleted_at", expiry);
    }
}

/// A configured span split into the unit the script has an arm for and the
/// count in front of it, or `None` where the unit is one it does not read.
fn split_span(span: &str) -> Option<(char, Result<i64, std::num::ParseIntError>)> {
    let mut characters = span.chars();
    let unit = characters.next_back()?;
    matches!(unit, 'd' | 'h' | 'm').then(|| (unit, characters.as_str().parse::<i64>()))
}

/// The transcription this module registers.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "56369e1eccf514805fe993224048e8d2c7a9a5f76867ea867908cf67a2ba06d1",
    source: "ti_rapid7_threat_command",
    name: "ioc_deleted_at",
    run: ioc_deleted_at,
}];
