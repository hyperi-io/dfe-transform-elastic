// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `citrix_adc`'s megabyte readings, transcribed.

use serde_json::{Value, json};

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_mul;

/// The two memory readings the vendor reports in megabytes, in script order.
const MEGABYTE_FIELDS: [&str; 2] = [
    "citrix_adc.system.memory.size.value",
    "citrix_adc.system.memory.usage.value",
];

/// The untagged converter in `citrix_adc/system`: both memory readings taken
/// from megabytes to bytes.
fn memory_megabytes_to_bytes(event: &mut Event, _params: &Value) {
    for path in MEGABYTE_FIELDS {
        let scaled = {
            let Some(megabytes) = event.get(path) else {
                continue;
            };
            // The script skips a null and an empty string, which is what a
            // reading the device omitted arrives as.
            if megabytes.is_null() || megabytes.as_str() == Some("") {
                continue;
            }
            painless_mul(&painless_mul(megabytes, &json!(1024)), &json!(1024))
        };
        let _ = event.update(path, scaled);
    }
}

/// Every `citrix_adc` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "7e11fb7744f66558de19fdd3171799d8296e62b92fe1f026b7b6039721269e38",
    source: "citrix_adc",
    name: "memory_megabytes_to_bytes",
    run: memory_megabytes_to_bytes,
}];
