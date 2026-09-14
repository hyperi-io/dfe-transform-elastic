// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `box_events`'s Shield alert scripts: the containers the per-category
//! builders write into, one builder per rule category, and the closing dedup.
//!
//! Every builder opens on `!ctx.rule.category.equals(<category>)` and returns,
//! so the four are alternatives over the same event and only one of them
//! reaches its body.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_string;

/// Where the vendor's alert payload sits by the time these run.
const ALERT_SUMMARY: &str = "box.additional_details.shield_alert.alert_summary";

/// The list every builder appends an indicator to.
const ENRICHMENTS: &str = "threat.enrichments";

/// The addresses the builders collect, which the pipeline's own appends extend.
const RELATED_IP: &str = "related.ip";

/// The first activity time seen, which a later `date` reads.
const SHIELD_DATE: &str = "shield_date";

/// `script_prepare_shield_objects` in `box_events/events`: the `threat` and
/// `related` maps and the `related.ip` list the per-category builders assume.
///
/// Only a `SHIELD_ALERT` gets them, and each is created only where the document
/// carries nothing at that path.
fn prepare_shield_objects(event: &mut Event, _params: &Value) {
    if event.get_str("event.action") != Some("SHIELD_ALERT") {
        return;
    }
    if !event.has_value("threat") {
        let _ = event.set("threat", Value::Object(Map::new()));
    }
    if !event.has_value("related") {
        let _ = event.set("related", Value::Object(Map::new()));
    }
    if !event.has_value(RELATED_IP) {
        let _ = event.set(RELATED_IP, Value::Array(Vec::new()));
    }
}

/// `script_suspicious_sessions` in `box_events/events`: one indicator per
/// activity of every session the vendor marked suspicious.
fn suspicious_sessions(event: &mut Event, _params: &Value) {
    if !category_is(event, "Suspicious Sessions") {
        return;
    }
    add_category(event, "network");
    add_type(event, "access");
    add_type(event, "connection");
    if !open_enrichments(event) {
        return;
    }

    let Some(sessions) = event
        .get_array(&format!("{ALERT_SUMMARY}.sessions"))
        .cloned()
    else {
        return;
    };
    for session in &sessions {
        if session.get("session_type").and_then(Value::as_str) != Some("suspicious") {
            continue;
        }
        let Some(activities) = session.get("activities").and_then(Value::as_array) else {
            continue;
        };
        for activity in activities {
            take_shield_date(event, activity.get("occurred_at"));
            let mut indicator = Map::new();
            indicator.insert("geo".to_owned(), geo_of(activity));
            indicator.insert("description".to_owned(), Value::from(described(activity)));
            indicator.insert("provider".to_owned(), member(activity, "service_name"));
            indicator.insert("type".to_owned(), Value::from("user-account"));
            let address = ip_of(activity);
            push_indicator(event, indicator);
            let _ = event.append(RELATED_IP, address);
        }
    }
}

/// `script_suspicious_locations` in `box_events/events`: one indicator per
/// alert activity, typed by whether its address holds a colon.
fn suspicious_locations(event: &mut Event, _params: &Value) {
    if !category_is(event, "Suspicious Locations") {
        return;
    }
    add_category(event, "network");
    add_type(event, "access");
    add_type(event, "connection");
    if !open_enrichments(event) {
        return;
    }

    let Some(activities) = event
        .get_array(&format!("{ALERT_SUMMARY}.alert_activities"))
        .cloned()
    else {
        return;
    };
    for activity in &activities {
        take_shield_date(event, activity.get("occurred_at"));
        let address = ip_of(activity);
        let kind = if painless_to_string(&address).contains(':') {
            "ipv6-addr"
        } else {
            "ipv4-addr"
        };
        let mut indicator = Map::new();
        indicator.insert("geo".to_owned(), geo_of(activity));
        indicator.insert("description".to_owned(), Value::from(described(activity)));
        indicator.insert("provider".to_owned(), member(activity, "service_name"));
        indicator.insert("type".to_owned(), Value::from(kind));
        push_indicator(event, indicator);
        let _ = event.append(RELATED_IP, address);
    }
}

/// `script_anomalous_download` in `box_events/events`: one indicator per
/// downloading address, all of them carrying the alert's own summary.
fn anomalous_download(event: &mut Event, _params: &Value) {
    if !category_is(event, "Anomalous Download") {
        return;
    }
    add_category(event, "file");
    add_type(event, "access");
    if !open_enrichments(event) {
        return;
    }

    if !event.has_value(ALERT_SUMMARY) {
        return;
    }
    let range = format!("{ALERT_SUMMARY}.anomaly_period.date_range");
    let start = event.get(&format!("{range}.start_date")).cloned();
    take_shield_date(event, start.as_ref());

    let description = event
        .get(&format!("{ALERT_SUMMARY}.description"))
        .cloned()
        .unwrap_or(Value::Null);
    let first_seen = start.unwrap_or(Value::Null);
    let last_seen = event
        .get(&format!("{range}.end_date"))
        .cloned()
        .unwrap_or(Value::Null);
    let sightings = event
        .get(&format!(
            "{ALERT_SUMMARY}.historical_period.downloaded_files_count"
        ))
        .cloned()
        .unwrap_or(Value::Null);

    let Some(addresses) = event
        .get_array(&format!("{ALERT_SUMMARY}.download_ips"))
        .cloned()
    else {
        return;
    };
    for entry in &addresses {
        let address = member(entry, "ip");
        let mut indicator = Map::new();
        indicator.insert("ip".to_owned(), address.clone());
        indicator.insert("description".to_owned(), description.clone());
        indicator.insert("first_seen".to_owned(), first_seen.clone());
        indicator.insert("last_seen".to_owned(), last_seen.clone());
        indicator.insert("sightings".to_owned(), sightings.clone());
        indicator.insert("type".to_owned(), Value::from("file"));
        push_indicator(event, indicator);
        let _ = event.append(RELATED_IP, address);
    }
}

/// `script_deduplicate_arrays` in `box_events/events`: the four lists the
/// builders and the pipeline's own appends both write.
///
/// `event.category` and `event.type` keep their nulls; the two address lists
/// drop theirs, which is what the script's `Objects::nonNull` filter does.
fn deduplicate_arrays(event: &mut Event, _params: &Value) {
    distinct(event, "event.category", false);
    distinct(event, "event.type", false);
    if event.has_value("related") {
        distinct(event, RELATED_IP, true);
    }
    distinct(event, &format!("{ALERT_SUMMARY}.download_ips"), true);
}

/// Whether the alert's rule category is the one a builder handles.
fn category_is(event: &Event, wanted: &str) -> bool {
    event.get_str("rule.category") == Some(wanted)
}

/// Append to `event.category`, which the pipeline opened as a list.
fn add_category(event: &mut Event, value: &str) {
    let _ = event.append("event.category", Value::from(value));
}

/// Append to `event.type`, which the pipeline opened as a list.
fn add_type(event: &mut Event, value: &str) {
    let _ = event.append("event.type", Value::from(value));
}

/// Open the enrichment list, reporting whether the containers it needs exist.
///
/// The script dereferences `ctx.threat` and `ctx.related` unguarded, so a
/// document without them raises there and writes nothing further.
fn open_enrichments(event: &mut Event) -> bool {
    if !event.has_value("threat") || !event.has_value(RELATED_IP) {
        return false;
    }
    if !event.has_value(ENRICHMENTS) {
        let _ = event.set(ENRICHMENTS, Value::Array(Vec::new()));
    }
    true
}

/// Add one indicator to the enrichment list, wrapped the way the script wraps
/// it.
fn push_indicator(event: &mut Event, indicator: Map<String, Value>) {
    let mut wrapper = Map::new();
    wrapper.insert("indicator".to_owned(), Value::Object(indicator));
    let _ = event.append(ENRICHMENTS, Value::Object(wrapper));
}

/// Keep the FIRST activity time the alert offers.
fn take_shield_date(event: &mut Event, occurred_at: Option<&Value>) {
    if event.has_value(SHIELD_DATE) {
        return;
    }
    let _ = event.set(SHIELD_DATE, occurred_at.cloned().unwrap_or(Value::Null));
}

/// One member of an activity, null where the vendor sent none.
fn member(activity: &Value, key: &str) -> Value {
    activity.get(key).cloned().unwrap_or(Value::Null)
}

/// The address an activity was seen from.
fn ip_of(activity: &Value) -> Value {
    activity
        .get("ip_info")
        .and_then(|info| info.get("ip"))
        .cloned()
        .unwrap_or(Value::Null)
}

/// The `geo` block the script builds around the activity's coordinates.
fn geo_of(activity: &Value) -> Value {
    let info = activity.get("ip_info");
    let mut location = Map::new();
    location.insert(
        "lon".to_owned(),
        info.and_then(|info| info.get("longitude"))
            .cloned()
            .unwrap_or(Value::Null),
    );
    location.insert(
        "lat".to_owned(),
        info.and_then(|info| info.get("latitude"))
            .cloned()
            .unwrap_or(Value::Null),
    );
    let mut geo = Map::new();
    geo.insert("ip".to_owned(), ip_of(activity));
    geo.insert("location".to_owned(), Value::Object(location));
    Value::Object(geo)
}

/// The sentence both activity builders write, verbatim down to the separators.
fn described(activity: &Value) -> String {
    format!(
        "IP {} was observed to {} {} {}/{} by {}",
        painless_to_string(&ip_of(activity)),
        painless_to_string(&member(activity, "event_type")),
        painless_to_string(&member(activity, "item_type")),
        painless_to_string(&member(activity, "item_path")),
        painless_to_string(&member(activity, "item_name")),
        painless_to_string(&member(activity, "service_name")),
    )
}

/// Collapse a list to its first occurrence of each member, optionally dropping
/// the nulls first.
fn distinct(event: &mut Event, path: &str, drop_null: bool) {
    let Some(items) = event.take_array(path) else {
        return;
    };
    let mut kept: Vec<Value> = Vec::with_capacity(items.len());
    for item in items {
        if drop_null && item.is_null() {
            continue;
        }
        if !kept.contains(&item) {
            kept.push(item);
        }
    }
    let _ = event.update(path, Value::Array(kept));
}

/// Every `box_events` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "02de3d2d39f0a883d833ca52736b7e129109f32bf755c0481c5cc95e290cd7b0",
        source: "box_events",
        name: "prepare_shield_objects",
        run: prepare_shield_objects,
    },
    Entry {
        hash: "03252c464bbac0ea9beb9eacd41092f7160c7e6dc4e35a8aa235f7fa766a10ad",
        source: "box_events",
        name: "suspicious_sessions",
        run: suspicious_sessions,
    },
    Entry {
        hash: "28555500f3e7c357562e1fb197c068fafdd2c5870825dba567ceb77a10b7d2f1",
        source: "box_events",
        name: "suspicious_locations",
        run: suspicious_locations,
    },
    Entry {
        hash: "d07215a263086ce447d98472997567a2c71cac50f85d07cf7c10066bb443e536",
        source: "box_events",
        name: "anomalous_download",
        run: anomalous_download,
    },
    Entry {
        hash: "0144a5876e2af01b986b1b33ebafcfe8daf38ab437be20566ba32a9c90b51885",
        source: "box_events",
        name: "deduplicate_arrays",
        run: deduplicate_arrays,
    },
];
