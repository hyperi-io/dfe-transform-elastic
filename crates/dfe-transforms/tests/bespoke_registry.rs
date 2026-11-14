// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Every bespoke runner stands in for a script the generated tree still ships.
//!
//! A transcription is keyed by the hash of the script's text, so a vendor
//! pipeline change re-keys the script and the runner silently stops applying:
//! the ladder takes the script back and the corpus reports a regression nobody
//! can attribute. This reads every `compiled_painless!` hash out of the
//! generated tree and refuses a registered hash that matches none of them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod sites;

use std::collections::{BTreeMap, HashSet};

use dfe_runtime::painless_bespoke as bespoke;

/// The hash of every call site in the generated tree, with the bindings its
/// plan resolved to.
fn shipped() -> BTreeMap<String, Vec<String>> {
    sites::sites()
        .into_iter()
        .map(|site| (site.hash, site.plan.binding()))
        .collect()
}

#[test]
fn every_bespoke_hash_matches_a_shipped_script() {
    let shipped = shipped();
    let mut seen = HashSet::new();
    for entry in bespoke::entries() {
        assert!(
            seen.insert(entry.hash),
            "{entry:?} shares its hash with an earlier entry"
        );
        assert!(
            shipped.contains_key(entry.hash),
            "{entry:?} is keyed by {} and no compiled_painless! site in the tree \
             carries it -- the vendor script changed, or the hash was typed wrong",
            entry.hash
        );
    }
}

#[test]
fn a_registered_script_resolves_to_its_entry() {
    let shipped = shipped();
    for entry in bespoke::entries() {
        assert_eq!(
            shipped[entry.hash].first().map(String::as_str),
            Some(format!("{entry:?}").as_str()),
            "{entry:?} is not the first binding of its script"
        );
    }
}
