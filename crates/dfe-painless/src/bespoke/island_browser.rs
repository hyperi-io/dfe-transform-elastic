// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `island_browser`'s single-address unwrap, transcribed.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// `script_flatten_single_internal_ip` in `island_browser/device`: a device
/// reporting one internal address keeps it as a scalar.
///
/// The split ahead of this makes a list of every address, and a one-NIC device
/// is then a one-element list where the vendor's own single-address form is a
/// plain string.
fn single_internal_ip_to_scalar(event: &mut Event, _params: &Value) {
    let path = "island_browser.device.internal_ip_address";
    let Some(first) = event
        .get_array(path)
        .and_then(|addresses| addresses.first())
        .cloned()
    else {
        return;
    };
    let _ = event.update(path, first);
}

/// Every `island_browser` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "7e8e47ce705df9148006a12d86de556cda83852c668fdbe1c7c186deab55afac",
    source: "island_browser",
    name: "single_internal_ip_to_scalar",
    run: single_internal_ip_to_scalar,
}];
