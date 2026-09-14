// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `ti_crowdstrike`'s indicator value, filed under the ECS field its own IOC
//! type names.

use serde_json::Value;

use super::Entry;
use dfe_core::event::Event;

/// `ti_crowdstrike/ioc`, the untagged STIX mapping processor:
/// `threat.indicator.type`, and the value copied to the field that type asks
/// for.
///
/// Every write descends `ctx.threat.indicator` with no null guard, so a
/// document that never built it raises and the processor's own `on_failure`
/// takes the whole thing -- not even the type is written.
fn ioc_value_by_type(event: &mut Event, params: &Value) {
    if !matches!(event.get("threat.indicator"), Some(Value::Object(_))) {
        return;
    }
    let Some(ioc_type) = event.get_string("ti_crowdstrike.ioc.type") else {
        return;
    };
    let Some(mapping) = params.get(&ioc_type).and_then(Value::as_str) else {
        return;
    };
    let target = match (ioc_type.as_str(), mapping) {
        // An IP is left alone here so the conversion checks further down the
        // pipeline get to decide which of the two address fields it lands in.
        ("domain", _) => "threat.indicator.url.domain",
        ("md5", "file") => "threat.indicator.file.hash.md5",
        ("sha256", "file") => "threat.indicator.file.hash.sha256",
        ("sha1", "file") => "threat.indicator.file.hash.sha1",
        _ => "",
    };
    let _ = event.set("threat.indicator.type", mapping.to_owned());
    if target.is_empty() {
        return;
    }
    let Some(value) = event.get("ti_crowdstrike.ioc.value").cloned() else {
        return;
    };
    let _ = event.set(target, value);
}

/// The transcription this module registers.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "9fd8779b9492dd20ea7af33db55516649f6eaa727bda464962b546b0f7faa9f5",
    source: "ti_crowdstrike",
    name: "ioc_value_by_type",
    run: ioc_value_by_type,
}];
