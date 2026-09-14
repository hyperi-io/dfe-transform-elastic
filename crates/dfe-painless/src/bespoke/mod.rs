// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Runners transcribed by hand, keyed by the script each stands in for.
//!
//! The ladder recognises a script's TEXT and dispatches to a runner written
//! for that pattern. A script that appears once in the catalogue costs a whole
//! matcher and pays back one event, and a matcher gated on a keyword can claim
//! a script it cannot run. For those the Painless is transcribed into one
//! function local to its source and registered here against the hash of the
//! script's text. [`crate::plan::PainlessPlan`] consults this table before
//! the params matcher and the ladder, once per call site, so a registered
//! script bypasses whatever claimed it and the hot path pays nothing.
//!
//! One module per source, one `fn` per script, every entry in that module's
//! `ENTRIES`. A function is promoted to a pattern the day a second source
//! spells the same script.
//!
//! The key is [`script_hash`]. `crates/dfe-transforms/tests/bespoke_registry.rs`
//! holds every registered hash to a script in the generated tree, so a vendor
//! change fails loudly instead of falling back to the ladder in silence.
//!
//! Transcribing Painless onto [`Event`]: `ctx.a.b` reads as `event.get("a.b")`
//! and writes as `event.set`; a path the script reads back after writing goes
//! through `Event::update`, because `set` splits a dotted key that `get`
//! honours flat. `x != null` is `has_value`, `containsKey` is `has`, and a
//! removal is `Event::remove` -- never `Map::remove`, which reorders the
//! document under `preserve_order`. `new HashSet()` iterates in Java bucket
//! order, which `crate::helpers::java_bucket` reproduces. `params.x` is the
//! `params` value handed to the runner. The processor's `if:` guard is not
//! part of the script and stays where the generator put it.

use std::collections::HashMap;
use std::fmt;
use std::sync::OnceLock;

use serde_json::Value;
use sha2::{Digest, Sha256};

use dfe_core::event::Event;

mod airlock_digital;
mod arista_ngfw;
mod azure_ai_foundry;
mod extrahop;
mod symantec_endpoint_security;
mod ti_recordedfuture;
mod ti_ticura;
mod wiz;

/// A transcribed script's effect on one event.
///
/// `params` is the pipeline's `params` block verbatim, [`Value::Null`] when
/// the processor carries none.
pub type Runner = fn(&mut Event, &Value);

/// One transcribed script.
pub struct Entry {
    /// [`script_hash`] of the script this stands in for.
    pub hash: &'static str,
    /// The source the transcription lives under, for the census.
    pub source: &'static str,
    /// The function's name, for the census.
    pub name: &'static str,
    /// The transcription.
    pub run: Runner,
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bespoke({}::{})", self.source, self.name)
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Eq for Entry {}

/// Every source module's entries, in registration order.
const ENTRIES: &[&[Entry]] = &[
    airlock_digital::ENTRIES,
    arista_ngfw::ENTRIES,
    azure_ai_foundry::ENTRIES,
    extrahop::ENTRIES,
    symantec_endpoint_security::ENTRIES,
    ti_recordedfuture::ENTRIES,
    ti_ticura::ENTRIES,
    wiz::ENTRIES,
];

/// The key a script is registered under.
///
/// The script with its escapes resolved, every run of whitespace collapsed to
/// one space, both ends trimmed, hashed with SHA-256 and rendered as lowercase
/// hex. Whitespace is normalised so the YAML source, the generated literal and
/// a hand-pasted copy all key the same.
#[must_use]
pub fn script_hash(script: &str) -> String {
    let mut hasher = Sha256::new();
    let mut started = false;
    let mut pending_space = false;
    let mut buf = [0u8; 4];
    for ch in script.chars() {
        if ch.is_whitespace() {
            pending_space = started;
            continue;
        }
        if pending_space {
            hasher.update(b" ");
            pending_space = false;
        }
        hasher.update(ch.encode_utf8(&mut buf).as_bytes());
        started = true;
    }
    format!("{:x}", hasher.finalize())
}

/// The transcription registered for a script, if any.
#[must_use]
pub fn lookup(script: &str) -> Option<&'static Entry> {
    table().get(script_hash(script).as_str()).copied()
}

/// Every registered transcription.
pub fn entries() -> impl Iterator<Item = &'static Entry> {
    ENTRIES.iter().flat_map(|module| module.iter())
}

fn table() -> &'static HashMap<&'static str, &'static Entry> {
    static TABLE: OnceLock<HashMap<&'static str, &'static Entry>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut map = HashMap::new();
        for entry in entries() {
            assert!(
                map.insert(entry.hash, entry).is_none(),
                "two bespoke runners are registered under {}",
                entry.hash
            );
        }
        map
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_ignores_whitespace_layout_but_not_content() {
        let one = script_hash("ctx.a = 1;\n  ctx.b = 2;  ");
        assert_eq!(one, script_hash("ctx.a = 1; ctx.b = 2;"));
        assert_eq!(one.len(), 64);
        assert_ne!(one, script_hash("ctx.a = 1;ctx.b = 2;"));
        assert_ne!(one, script_hash("ctx.a = 1; ctx.b = 3;"));
    }

    #[test]
    fn unregistered_script_has_no_entry() {
        assert!(lookup("def nothing() { return 1; }").is_none());
    }
}
