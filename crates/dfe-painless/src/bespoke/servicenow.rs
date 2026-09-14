// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `servicenow`'s URL escaping and oversize-string trim, transcribed.
//!
//! A record's `url` display value is a query the console wrote, and it carries
//! the two characters `java.net.URI` will not take: a space, and the `^` the
//! encoded-query syntax separates its terms with. The script runs in the
//! `on_failure` of the first `uri_parts`, escapes those two, and hands the
//! result to a second `uri_parts` -- so without it `_tmp_url` never exists, the
//! second processor raises, and the event ends as a `pipeline_error` with no
//! `url.path` or `url.query` at all.
//!
//! The other script is the closing sweep that keeps a record's free-text field
//! under Lucene's own term limit. Four data streams ship it verbatim --
//! `qualys_vmdr`'s two and `vectra_rux`'s audit stream as well as this one --
//! and the registry keys on the script rather than the source, so the one
//! transcription serves them all.

use serde_json::Value;

use dfe_core::Event;

use super::Entry;

/// Where the console's own query sits.
const DISPLAY_VALUE: &str = "servicenow.event.url.display_value";

/// Where the escaped copy goes, for the `uri_parts` that follows.
const TARGET: &str = "_tmp_url";

/// `servicenow/event`, the `encode` script in `default`.
///
/// The vendor's own condition reads `(a-z) || (A-Z) || (0-9) || (c != ' ' && c
/// != '^')`, and the last term makes every other character legal -- so exactly
/// two are escaped, however the first three clauses read.
fn encode_url(event: &mut Event, _params: &Value) {
    use std::fmt::Write as _;

    let Some(path) = event.get_string(DISPLAY_VALUE) else {
        return;
    };
    let mut out = String::with_capacity(path.len());
    for c in path.chars() {
        match c {
            ' ' | '^' => {
                // `Integer.toHexString` prints LOWER case and pads nothing.
                let _ = write!(out, "%{:x}", c as u32);
            }
            kept => out.push(kept),
        }
    }
    let _ = event.set(TARGET, out);
}

/// The length above which the script cuts, and the length it cuts to. Both are
/// counted in Java `char`s, which are UTF-16 code units.
const TOO_LONG: usize = 32766;
const KEPT: usize = 32700;

/// `servicenow/event`, the `filterMassive` script in `default`: every string in
/// the document, however deep, trimmed where it would not fit a keyword term.
fn filter_massive(event: &mut Event, _params: &Value) {
    trim_massive(event.as_value_mut());
}

/// The walk itself, over one value.
fn trim_massive(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for held in map.values_mut() {
                trim_massive(held);
            }
        }
        Value::Array(items) => {
            for item in items {
                trim_massive(item);
            }
        }
        Value::String(text) => {
            // UTF-8 bytes are never fewer than UTF-16 units, so a short string
            // is answered without counting anything.
            if text.len() <= TOO_LONG {
                return;
            }
            let units: Vec<u16> = text.encode_utf16().collect();
            if units.len() <= TOO_LONG {
                return;
            }
            let mut cut = String::from_utf16_lossy(&units[..KEPT]);
            cut.push_str(" (truncated)");
            *text = cut;
        }
        _ => {}
    }
}

/// Every `servicenow` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "87628cef8192eb1a35da23964eff32ce29f6c7a66b2522ce28693d82ab0fc76a",
        source: "servicenow",
        name: "encode_url",
        run: encode_url,
    },
    Entry {
        hash: "d8ba8ac79df9c02625cd5a1ec454a2adcc3895cdd71ccd12e4822a6d1a17819e",
        source: "servicenow",
        name: "filter_massive",
        run: filter_massive,
    },
];
