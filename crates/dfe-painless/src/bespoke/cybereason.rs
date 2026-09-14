// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `cybereason`'s payload-splitting scripts, transcribed.
//!
//! The console answers a malop query with a THREE-ELEMENT array -- the record
//! itself, then the suspicions map, then the evidence map -- so the whole
//! document has to be taken apart before any mapping can reach it. The second
//! script is the same problem one level down: the process owner's user arrives
//! under a key whose name has a dot in it, which no field path can address.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// `cybereason/logon_session`, `malop_connection`, `malop_process` and
/// `suspicions_process`, `script_to_parse_different_message_object`: the
/// record, the suspicions map and the evidence map split out of the array the
/// console answers with.
///
/// The script indexes the array without checking its length, so a short one
/// raises part way and leaves what it had already assigned. Anything that is
/// not an array is left alone instead -- Painless would read the indices as
/// map keys and blank the document.
fn split_message_objects(event: &mut Event, _params: &Value) {
    let Some(items) = event.get_array("json").cloned() else {
        return;
    };
    let Some(suspicions) = items.get(1) else {
        return;
    };
    let _ = event.set("suspicionsMap", suspicions.clone());
    let Some(evidence) = items.get(2) else {
        return;
    };
    let _ = event.set("evidenceMap", evidence.clone());
    let _ = event.set("json", items[0].clone());
}

/// `cybereason/malop_connection`, `script_to_rename_ownerProcess_user`: the
/// process owner's user moved out of the console's dotted key.
///
/// `ownerProcess.user` is one KEY holding a dot, beside the `ownerProcess`
/// key it reads as a child of, so the move is a map operation and not a field
/// path -- addressing it as a path walks into `ownerProcess` and finds
/// nothing.
fn rename_owner_process_user(event: &mut Event, _params: &Value) {
    let Some(Value::Object(element_values)) = resolve(event, "json.elementValues") else {
        return;
    };
    let Some(owner) = element_values.shift_remove("ownerProcess.user") else {
        return;
    };
    let _ = event.set(
        "cybereason.malop_connection.element_values.owner_process_user",
        owner,
    );
}

/// The value at a dotted path, for a key the path itself cannot name.
fn resolve<'a>(event: &'a mut Event, path: &str) -> Option<&'a mut Value> {
    let mut current = event.as_value_mut();
    for segment in path.split('.') {
        current = current.as_object_mut()?.get_mut(segment)?;
    }
    Some(current)
}

/// Every `cybereason` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "b9279188e0943d6338567b0b2622d835c72cbb80668347c4f2cd7c116ac4e131",
        source: "cybereason",
        name: "split_message_objects",
        run: split_message_objects,
    },
    Entry {
        hash: "4f3533b625d6ce90153415d4f8359e19815ecc901af7fc4bb1ec16d64fea55d9",
        source: "cybereason",
        name: "rename_owner_process_user",
        run: rename_owner_process_user,
    },
];
