// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `netbox`'s site location, as the ordered pair a `geo_point` is read from.

use serde_json::Value;

use super::Entry;
use dfe_core::event::Event;

/// `netbox/devices`, the untagged coordinates processor:
/// `netbox.device.coordinates`.
///
/// Longitude goes FIRST, which is the reverse of the order the two fields are
/// usually spoken in and the order a `geo_point` array is read in.
fn coordinates(event: &mut Event, _params: &Value) {
    let pair = match (
        event.get("netbox.device.longitude"),
        event.get("netbox.device.latitude"),
    ) {
        (Some(longitude), Some(latitude)) => vec![longitude.clone(), latitude.clone()],
        _ => return,
    };
    let _ = event.set("netbox.device.coordinates", Value::Array(pair));
}

/// The transcription this module registers.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "a351927eeb0597b4007f5e4eb50667c6572dfc68a86700af5d06010c4561c0be",
    source: "netbox",
    name: "coordinates",
    run: coordinates,
}];
