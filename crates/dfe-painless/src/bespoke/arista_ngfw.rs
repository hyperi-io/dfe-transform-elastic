// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `arista_ngfw`'s interface naming from the agent's own configuration.
//!
//! The vendor logs an interface by NUMBER and the agent carries the operator's
//! names for interfaces 1 and 2 in `_conf`, so the script copies whichever pair
//! the number selects onto the egress and the ingress side independently.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// `script_16ed71f2` in `arista_ngfw/log`: `observer.{egress,ingress}.
/// interface.{alias,name}` from `_conf`.
///
/// The two sides are separate ladders in the script, so an event whose egress
/// and ingress interfaces differ takes a different `_conf` pair on each.
fn interface_names_from_conf(event: &mut Event, _params: &Value) {
    name_side(
        event,
        "observer.egress.interface.id",
        "observer.egress.interface.alias",
        "observer.egress.interface.name",
    );
    name_side(
        event,
        "observer.ingress.interface.id",
        "observer.ingress.interface.alias",
        "observer.ingress.interface.name",
    );
}

/// One side's ladder: the id is compared as a STRING, which is what the
/// `convert` processor ahead of the script leaves it as.
fn name_side(event: &mut Event, id: &str, alias: &str, name: &str) {
    let (conf_alias, conf_name) = match event.get_str(id) {
        Some("1") => ("_conf.interface_id_1_alias", "_conf.interface_id_1_name"),
        Some("2") => ("_conf.interface_id_2_alias", "_conf.interface_id_2_name"),
        _ => return,
    };
    copy_configured(event, conf_alias, alias);
    copy_configured(event, conf_name, name);
}

/// Copy one `_conf` entry, leaving the target alone where the operator set none.
fn copy_configured(event: &mut Event, from: &str, to: &str) {
    let Some(configured) = event.get(from).filter(|v| !v.is_null()).cloned() else {
        return;
    };
    let _ = event.set(to, configured);
}

/// Every `arista_ngfw` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "a867495fff14aa6763ddbff4dee88aeb7abd4c858a9cde3b98a2fc307e3cebed",
    source: "arista_ngfw",
    name: "interface_names_from_conf",
    run: interface_names_from_conf,
}];
