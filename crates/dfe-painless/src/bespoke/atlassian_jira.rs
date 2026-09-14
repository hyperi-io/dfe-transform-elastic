// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `atlassian_jira`'s audit source-address split, transcribed.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// The comma-split addresses the audit record carries.
const SOURCE_IPS: &str = "_tmp.source_ip";

/// The audit pipeline's untagged split script: `source.address` from the first
/// address, `jira.audit.additional_source_ips` from the rest.
///
/// `source` is REPLACED with a fresh map before either write, so whatever the
/// earlier `remoteAddress` rename left there does not survive.
fn split_source_addresses(event: &mut Event, _params: &Value) {
    let Some(addresses) = event.get_array(SOURCE_IPS).cloned() else {
        return;
    };
    let _ = event.set("source", Value::Object(Map::new()));
    let Some((first, rest)) = addresses.split_first() else {
        return;
    };
    let _ = event.set("source.address", first.clone());
    if rest.is_empty() {
        return;
    }
    // Painless raises on the assignment where the parent map is absent.
    if !event.has("jira.audit") {
        return;
    }
    let _ = event.set(
        "jira.audit.additional_source_ips",
        Value::Array(rest.to_vec()),
    );
}

/// Every `atlassian_jira` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "089b844ec3456e1c46149a09e9cb539f0c54ff18a4f91708f66cedc6e02825d3",
    source: "atlassian_jira",
    name: "split_source_addresses",
    run: split_source_addresses,
}];
