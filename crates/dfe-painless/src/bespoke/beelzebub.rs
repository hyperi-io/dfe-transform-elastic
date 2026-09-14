// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `beelzebub`'s request-body size, so a search can filter on how big a body was.

use serde_json::Value;

use super::Entry;
use dfe_core::event::Event;

/// `beelzebub/logs`, the untagged length processor: `http.request.body.bytes`
/// counted off the body text itself.
///
/// Java's `String.length()` counts UTF-16 units rather than bytes, so a
/// multi-byte character still counts one and a character outside the BMP
/// counts two.
fn body_length(event: &mut Event, _params: &Value) {
    let Some(content) = event.get_str("http.request.body.content") else {
        return;
    };
    let units = content.chars().map(char::len_utf16).sum::<usize>();
    let _ = event.set(
        "http.request.body.bytes",
        i64::try_from(units).unwrap_or(i64::MAX),
    );
}

/// The transcription this module registers.
pub const ENTRIES: &[Entry] = &[Entry {
    hash: "02f4e9612b4ac3810d50c63f55b66052336bd0f5e4f2e1b0bc815c2e9d1c63ce",
    source: "beelzebub",
    name: "body_length",
    run: body_length,
}];
