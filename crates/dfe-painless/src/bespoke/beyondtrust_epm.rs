// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `beyondtrust_epm`'s container image digests, transcribed.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;
use crate::helpers::painless_to_string;

/// `script_container_image_hash_all_into_related_hash` in
/// `beyondtrust_epm/event`: the bare digests behind the container image hashes.
///
/// An image hash arrives as `<algorithm>:<digest>`, sometimes inside the square
/// brackets a rendered list leaves behind, and `related.hash` holds the digest
/// alone so a search on it matches whatever other source reported the file.
fn container_digests_into_related_hash(event: &mut Event, _params: &Value) {
    let digests: Vec<String> = {
        let Some(hashes) = event.get_array("beyondtrust_epm.event.container.image.hash.all") else {
            return;
        };
        hashes
            .iter()
            .filter(|entry| !entry.is_null())
            .map(|entry| {
                let digest = painless_to_string(entry).replace(['[', ']'], "");
                match digest.find(':') {
                    Some(separator) => digest[separator + 1..].to_string(),
                    None => digest,
                }
            })
            .filter(|digest| !digest.is_empty())
            .collect()
    };
    // The script creates the list before it appends anything, and the module's
    // closing prune takes it away again where nothing landed in it.
    if !event.has_value("related.hash") {
        let _ = event.set("related.hash", Value::Array(Vec::new()));
    }
    for digest in digests {
        let _ = event.append_unique("related.hash", digest);
    }
}

/// Every `beyondtrust_epm` script transcribed here.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "825e4c5796f8b644026a63cf65d406830ae8fec109e8f9946aff2a2b57df34ea",
    source: "beyondtrust_epm",
    name: "container_digests_into_related_hash",
    run: container_digests_into_related_hash,
}];
