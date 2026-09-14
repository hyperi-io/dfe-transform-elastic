// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `imperva`'s custom-string gathering, transcribed.
//!
//! CEF defines six custom string slots, and the appliance sends twenty-one. The
//! first six arrive under their full `deviceCustomStringN` names and the
//! pipeline renames them one by one; `cs7` upwards have no CEF name at all, so
//! the script pairs each with its own `csNLabel` and writes the two into
//! `device.custom_stringN`. An empty value is written as the vendor sent it and
//! the module's closing prune takes it out.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// Where the CEF processor left the unnamed extensions.
const EXTENSIONS: &str = "cef.extensions";

/// The namespace the pairs are written into.
const DEVICE: &str = "imperva.securesphere.device";

/// `imperva/securesphere`, the untagged custom-string script in `default`:
/// `device.custom_string7` through `custom_string21`, each a `value` and a
/// `label`.
///
/// The run STOPS at the first slot the vendor sent neither half of, which is
/// the script's own `hasCustomString` flag rather than a scan of all fifteen.
fn gather_custom_strings(event: &mut Event, _params: &Value) {
    let Some(extensions) = event.get_object(EXTENSIONS) else {
        return;
    };

    let mut pairs: Vec<(String, Value)> = Vec::new();
    for slot in 7..=21 {
        let key = format!("cs{slot}");
        let label_key = format!("{key}Label");
        let value = extensions.get(&key);
        let label = extensions.get(&label_key);
        if value.is_none() && label.is_none() {
            break;
        }
        let mut pair = Map::with_capacity(2);
        if let Some(value) = value {
            pair.insert("value".to_owned(), value.clone());
        }
        if let Some(label) = label {
            pair.insert("label".to_owned(), label.clone());
        }
        pairs.push((format!("custom_string{slot}"), Value::Object(pair)));
    }

    // Painless raises where the target map is absent, so writing nothing is
    // what stands in for it -- `set` would build the namespace instead.
    if event.get_object(DEVICE).is_none() {
        return;
    }
    for (name, pair) in pairs {
        let _ = event.set(&format!("{DEVICE}.{name}"), pair);
    }
}

/// Every `imperva` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "826bfc3c557b5ad787f00d4653af3c58c1eb15fb406c864c7c14a40c08015c12",
    source: "imperva",
    name: "gather_custom_strings",
    run: gather_custom_strings,
}];
