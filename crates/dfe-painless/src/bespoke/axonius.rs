// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `axonius`'s identity passes: the devices, employees and groups an identity
//! is associated with, the breach records attached to it, and its recording
//! flags.

use chrono::{NaiveDate, NaiveDateTime};
use serde_json::{Map, Value};

use super::Entry;
use crate::helpers::painless_to_string;
use dfe_core::event::Event;

/// The list of associated devices, and where each of its members goes.
const DEVICE_FIELDS: [(&str, &str); 3] = [
    ("device_id", "device.id"),
    ("device_model", "device.model.name"),
    ("device_serial", "device.serial_number"),
];

/// The instant format the breach records are written back in: UTC to the
/// millisecond, which is what `yyyy-MM-dd'T'HH:mm:ss.SSSXXX` renders there.
const OUT_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

/// `axonius/identity`, processor `script_process_associated_entities`:
/// `device.id`, `device.model.name`, `device.serial_number`, `related.user`,
/// and the MAC addresses normalised in place.
fn process_associated_entities(event: &mut Event, _params: &Value) {
    if event
        .get("axonius.identity.associated_devices")
        .is_some_and(Value::is_array)
    {
        for path in ["device", "device.model"] {
            if !event.has(path) {
                let _ = event.set(path, Value::Object(Map::new()));
            }
        }
        let mut devices = event
            .take_array("axonius.identity.associated_devices")
            .unwrap_or_default();
        for device in &mut devices {
            let Some(members) = device.as_object_mut() else {
                continue;
            };
            for (from, to) in DEVICE_FIELDS {
                for value in listed(members.get(from)) {
                    let _ = event.append_unique(to, value);
                }
            }
            // A MAC the vendor wrote with colons is rewritten to the dashed,
            // upper-cased spelling the rest of the package uses.
            if let Some(addresses) = members
                .get_mut("device_preferred_mac_address")
                .and_then(Value::as_array_mut)
            {
                for address in addresses.iter_mut().filter(|value| !value.is_null()) {
                    let text = painless_to_string(address).replace(':', "-").to_uppercase();
                    *address = Value::from(text);
                }
            }
        }
        event.update("axonius.identity.associated_devices", Value::Array(devices));
    }

    if let Some(employees) = event
        .get_array("axonius.identity.associated_employees")
        .cloned()
    {
        if !event.has("related") {
            let _ = event.set("related", Value::Object(Map::new()));
        }
        for employee in &employees {
            for value in listed(employee.get("username")) {
                let _ = event.append_unique("related.user", value);
            }
        }
    }

    if let Some(groups) = event
        .get_array("axonius.identity.associated_groups")
        .cloned()
    {
        if !event.has("related") {
            let _ = event.set("related", Value::Object(Map::new()));
        }
        for group in &groups {
            if let Some(name) = group.get("display_name").filter(|value| !value.is_null()) {
                let _ = event.append_unique("related.user", painless_to_string(name));
            }
        }
    }
}

/// The non-null members of a list, each as the string `toString` makes of it.
fn listed(value: Option<&Value>) -> Vec<Value> {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter(|item| !item.is_null())
                .map(|item| Value::from(painless_to_string(item)))
                .collect()
        })
        .unwrap_or_default()
}

/// `axonius/identity`, processor `script_process_breaches_data`: every breach
/// record's dates, counts and flags, plus
/// `threat.enrichments.indicator.first_seen`.
///
/// A date the two formats cannot read is REMOVED rather than left as the
/// vendor wrote it, so nothing downstream date-parses a string twice.
fn process_breaches_data(event: &mut Event, params: &Value) {
    let date_fields = names(params, "dateFields");
    let bool_fields = names(params, "boolFields");
    let Some(mut records) = event.take_array("axonius.identity.breaches_data") else {
        return;
    };
    for record in &mut records {
        let Some(members) = record.as_object_mut() else {
            continue;
        };
        for field in &date_fields {
            let Some(raw) = members.get(field.as_str()) else {
                continue;
            };
            if raw.is_null() || raw.as_str() == Some("") {
                continue;
            }
            match parse_date(&painless_to_string(raw)) {
                Some(parsed) => {
                    members.insert(field.clone(), Value::from(parsed));
                }
                None => {
                    members.shift_remove(field.as_str());
                }
            }
        }
        let added = members
            .get("added_date")
            .filter(|value| !value.is_null())
            .map(painless_to_string);

        coerce_count(members);
        for field in &bool_fields {
            coerce_flag(members, field);
        }
        if let Some(added) = added {
            let _ = event.append_unique("threat.enrichments.indicator.first_seen", added);
        }
    }
    event.update("axonius.identity.breaches_data", Value::Array(records));
}

/// `axonius/identity`, the untagged recording processor: every
/// `axonius.identity.recording` flag as a boolean.
///
/// A value that is neither true nor false is REMOVED, which is how the
/// vendor's `auto_recording: "none"` leaves the document rather than staying
/// on as a string in a boolean field.
fn coerce_recording_flags(event: &mut Event, params: &Value) {
    let fields = names(params, "fields");
    let Some(mut recording) = event.get("axonius.identity.recording").cloned() else {
        return;
    };
    {
        let Some(members) = recording.as_object_mut() else {
            return;
        };
        for field in &fields {
            coerce_flag(members, field);
        }
    }
    event.update("axonius.identity.recording", recording);
}

/// The string members of a params list.
fn names(params: &Value, key: &str) -> Vec<String> {
    params
        .get(key)
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

/// One breach date in either spelling the vendor uses, as a UTC instant.
///
/// The day-of-week prefix is dropped before parsing because the vendor is
/// known to send one that disagrees with the date behind it.
fn parse_date(input: &str) -> Option<String> {
    let stripped = if input.chars().count() > 4 && input.find(',') == Some(3) {
        input[4..].trim()
    } else {
        input
    };
    if let Ok(stamp) = NaiveDateTime::parse_from_str(stripped, "%d %b %Y %H:%M:%S GMT") {
        return Some(stamp.format(OUT_FORMAT).to_string());
    }
    let date = NaiveDate::parse_from_str(stripped, "%Y-%m-%d").ok()?;
    Some(date.and_hms_opt(0, 0, 0)?.format(OUT_FORMAT).to_string())
}

/// `pwn_count` as a Java long, or gone where it is not one.
fn coerce_count(members: &mut Map<String, Value>) {
    let Some(value) = members.get("pwn_count") else {
        return;
    };
    if value.is_null() || value.is_i64() || value.is_u64() {
        return;
    }
    match painless_to_string(value).parse::<i64>() {
        Ok(count) => {
            members.insert("pwn_count".to_owned(), Value::from(count));
        }
        Err(_) => {
            members.shift_remove("pwn_count");
        }
    }
}

/// One flag as a boolean, or gone where the vendor sent something that is
/// neither true nor false.
fn coerce_flag(members: &mut Map<String, Value>, field: &str) {
    let Some(value) = members.get(field) else {
        return;
    };
    if value.is_null() || value.is_boolean() {
        return;
    }
    match painless_to_string(value).to_lowercase().as_str() {
        "1" | "true" => {
            members.insert(field.to_owned(), Value::Bool(true));
        }
        "" | "0" | "false" => {
            members.insert(field.to_owned(), Value::Bool(false));
        }
        _ => {
            members.shift_remove(field);
        }
    }
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "006e09306a16955e39ef85a98ebcc9c9edbc51a826e36678c65ece0f53b2cedb",
        source: "axonius",
        name: "process_associated_entities",
        run: process_associated_entities,
    },
    Entry {
        hash: "6e34162c8ec342b62bb947280ffab5149591130b4171c4a3169a41c69b7e5569",
        source: "axonius",
        name: "process_breaches_data",
        run: process_breaches_data,
    },
    Entry {
        hash: "33a62b6a59917cc495fb67627e3cbc9222c350d934727524adc09ab477abbfc3",
        source: "axonius",
        name: "coerce_recording_flags",
        run: coerce_recording_flags,
    },
];
