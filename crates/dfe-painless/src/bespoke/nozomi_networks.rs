// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Scripts the `nozomi_networks` package ships, transcribed by hand.

use serde_json::{Map, Value};

use super::Entry;
use crate::helpers::{java_bucket, java_table_size};
use dfe_core::event::Event;

/// `nozomi_networks/asset` and `/node`, processor
/// `calculate_host_uptime_e3210bfe`: the elapsed time between the record's
/// creation and its last activity, in seconds.
///
/// Java's integer division truncates towards zero, which is what `/ 1000` on
/// two epoch-millisecond readings means here.
fn calculate_host_uptime(event: &mut Event, _params: &Value) {
    let (Some(last_activity), Some(created_at)) = (
        event.get_as_i64("json.last_activity_time"),
        event.get_as_i64("json.created_at"),
    ) else {
        return;
    };
    let _ = event.set("host.uptime", (last_activity - created_at) / 1000);
}

/// `nozomi_networks/asset`, processor `map_host_geo_location_d7eb0c46`: pair
/// the asset's latitude and longitude into the geo-point `host.geo.location`.
///
/// The pair is built in a `HashMap`, so it renders in Java bucket order rather
/// than the order the script assigns them.
fn map_host_geo_location(event: &mut Event, _params: &Value) {
    let (Some(latitude), Some(longitude)) = (
        event.get("nozomi_networks.asset.latitude").cloned(),
        event.get("nozomi_networks.asset.longitude").cloned(),
    ) else {
        return;
    };
    let mut location = Map::new();
    let table = java_table_size(2);
    let mut pair = [("lat", latitude), ("lon", longitude)];
    pair.sort_by_key(|(key, _)| java_bucket(key, table));
    for (key, value) in pair {
        location.insert(key.to_string(), value);
    }
    let _ = event.set("host.geo.location", Value::Object(location));
}

/// `nozomi_networks/alert`, processor `set_event_severity_and_label_dffd1326`:
/// the vendor's nought-to-ten scale as an ECS severity and its word.
///
/// A severity outside every band writes neither field.
fn set_event_severity_and_label(event: &mut Event, _params: &Value) {
    let Some(severity) = event.get_f64("nozomi_networks.alert.severity") else {
        return;
    };
    let arm = if (0.0..=3.0).contains(&severity) {
        Some(("low", 21))
    } else if (4.0..=6.0).contains(&severity) {
        Some(("medium", 47))
    } else if (7.0..=10.0).contains(&severity) {
        Some(("high", 73))
    } else {
        None
    };
    let Some((label, score)) = arm else {
        return;
    };
    let _ = event.set("nozomi_networks.alert.severity_label", label);
    let _ = event.set("event.severity", score);
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "a5ad050926ebbc2cdb0bf18c3912370ea7c65141139f29483403d2e2245190e8",
        source: "nozomi_networks",
        name: "calculate_host_uptime",
        run: calculate_host_uptime,
    },
    Entry {
        hash: "f210656edec3eae33ed9abf7db4991755fc1612a67f8dde167c8e7764b53df7a",
        source: "nozomi_networks",
        name: "map_host_geo_location",
        run: map_host_geo_location,
    },
    Entry {
        hash: "211f6c75d1410b7124ab8c1e2eb8856ea465124ec39d20a8b3766544c98e6021",
        source: "nozomi_networks",
        name: "set_event_severity_and_label",
        run: set_event_severity_and_label,
    },
];
