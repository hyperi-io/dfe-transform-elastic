// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `symantec_endpoint_security`'s raw-data wrapper, its two duration scales and
//! its incident resolution label.
//!
//! `ses.raw_data` is mapped `flattened`, which holds an object and nothing
//! else, so anything the vendor sends as a scalar or as a list of scalars is
//! moved under a `value` key rather than rejected at index time.

use serde_json::{Map, Value, json};

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_f64;

/// The untagged script in `symantec_endpoint_security/event`: `ses.raw_data`
/// wrapped so the `flattened` mapping can hold it.
///
/// A list is wrapped WHOLE the moment any element is not an object -- the
/// script breaks out on the first one and puts the list itself under `value`,
/// not the element it stopped on.
fn wrap_raw_data(event: &mut Event, _params: &Value) {
    let Some(held) = event.get("ses.raw_data") else {
        return;
    };
    let wrap = match held {
        Value::Object(_) => false,
        Value::Array(items) => items.iter().any(|item| !item.is_object()),
        _ => true,
    };
    if !wrap {
        return;
    }
    let held = held.clone();
    let mut wrapper = Map::with_capacity(1);
    wrapper.insert("value".to_string(), held);
    let _ = event.set("ses.raw_data", Value::Object(wrapper));
}

/// `set_event_duration_from_event_duration` in
/// `symantec_endpoint_security/event`: `event.duration` from
/// `ses.event_duration`.
fn duration_from_event_duration(event: &mut Event, params: &Value) {
    scale_to_duration(event, params, "ses.event_duration");
}

/// `set_event_duration` in `symantec_endpoint_security/event`'s
/// `pipeline_category_security`: `event.duration` from `ses.duration`.
fn duration_from_duration(event: &mut Event, params: &Value) {
    scale_to_duration(event, params, "ses.duration");
}

/// The multiply both duration scripts spell, with the factor read from `params`.
///
/// Two whole numbers multiply as Java longs and WRAP, which is the script's own
/// arithmetic rather than an Elasticsearch defect: a duration of 12,345,678,901
/// seconds does not fit in the long `event.duration` is mapped as, either
/// engine.
fn scale_to_duration(event: &mut Event, params: &Value, source: &str) {
    let Some(factor) = params.get("S_TO_NS") else {
        return;
    };
    let Some(scaled) = event
        .get(source)
        .map(|held| match (held.as_i64(), factor.as_i64()) {
            (Some(value), Some(factor)) => json!(value.wrapping_mul(factor)),
            _ => json!(painless_to_f64(held) * painless_to_f64(factor)),
        })
    else {
        return;
    };
    let _ = event.set("event.duration", scaled);
}

/// `painless_set_resolution` in the incident data stream:
/// `ses.incident.resolution` from the resolution code.
///
/// The code is ONE-BASED against the label table, so reading it as an index
/// answers every incident with the label of the next resolution along.
fn resolution_from_id(event: &mut Event, params: &Value) {
    let Some(labels) = params.get("Resolution").and_then(Value::as_array) else {
        return;
    };
    let Some(code) = event.get_as_i64("ses.incident.resolution_id") else {
        return;
    };
    let Some(label) = usize::try_from(code - 1)
        .ok()
        .and_then(|index| labels.get(index))
    else {
        return;
    };
    let label = label.clone();
    let _ = event.set("ses.incident.resolution", label);
}

/// Every `symantec_endpoint_security` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "a6ee67078a0a442107d7daea9357b62d346862758404f78621ce04d67b334b09",
        source: "symantec_endpoint_security",
        name: "wrap_raw_data",
        run: wrap_raw_data,
    },
    Entry {
        hash: "ee89a887f9108abc79ce8ea538ca7fa2d3ed07dbff24bfa2b142966bba018e74",
        source: "symantec_endpoint_security",
        name: "duration_from_event_duration",
        run: duration_from_event_duration,
    },
    Entry {
        hash: "cc60b41239ff4d226b7ee401d0459e1f1859e18b9a79c40da99c7f05c941d810",
        source: "symantec_endpoint_security",
        name: "duration_from_duration",
        run: duration_from_duration,
    },
    Entry {
        hash: "d4d8580b8bd84d6add154d40efe4f4f1db4694a398f2232553c0487252de8b4b",
        source: "symantec_endpoint_security",
        name: "resolution_from_id",
        run: resolution_from_id,
    },
];
