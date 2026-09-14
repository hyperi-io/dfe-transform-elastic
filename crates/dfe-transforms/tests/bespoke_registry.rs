// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Every bespoke runner stands in for a script the generated tree still ships.
//!
//! A transcription is keyed by the hash of the script's text, so a vendor
//! pipeline change re-keys the script and the runner silently stops applying:
//! the ladder takes the script back and the corpus reports a regression nobody
//! can attribute. This reads every `cached_painless!` literal out of the
//! generated tree and refuses a registered hash that matches none of them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use dfe_runtime::painless_bespoke as bespoke;
use dfe_runtime::painless_plan::PainlessPlan;

/// The macro every generated call site wraps its script literal in.
const SITE: &str = "cached_painless!(";

/// Below this the scanner has stopped seeing the tree rather than the tree
/// having shrunk. The same floor as `painless_binding.rs`, for the same reason.
const MIN_SITES: usize = 1_600;

fn generated_tree() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// The raw-string literal at or after `at`, and where it ends.
fn raw_literal(src: &str, at: usize) -> Option<(String, usize)> {
    let bytes = src.as_bytes();
    let mut i = at;
    while bytes.get(i).is_some_and(u8::is_ascii_whitespace) {
        i += 1;
    }
    if bytes.get(i) != Some(&b'r') {
        return None;
    }
    i += 1;
    let hashes_at = i;
    while bytes.get(i) == Some(&b'#') {
        i += 1;
    }
    let close = format!("\"{}", "#".repeat(i - hashes_at));
    if bytes.get(i) != Some(&b'"') {
        return None;
    }
    i += 1;
    let end = src[i..].find(&close)? + i;
    Some((src[i..end].to_string(), end + close.len()))
}

/// The hash of every script literal in the generated tree, with one file
/// that carries it.
fn shipped_hashes() -> BTreeMap<String, String> {
    let mut files = Vec::new();
    rust_files(&generated_tree(), &mut files);
    let mut hashes = BTreeMap::new();
    let mut sites = 0;
    for file in files {
        let Ok(src) = std::fs::read_to_string(&file) else {
            continue;
        };
        let mut from = 0;
        while let Some(hit) = src[from..].find(SITE) {
            let after = from + hit + SITE.len();
            let (script, end) = raw_literal(&src, after)
                .unwrap_or_else(|| panic!("{}: no raw string at byte {after}", file.display()));
            let plan = PainlessPlan::new(&script);
            hashes
                .entry(bespoke::script_hash(plan.text()))
                .or_insert_with(|| file.display().to_string());
            sites += 1;
            from = end;
        }
    }
    assert!(
        sites >= MIN_SITES,
        "scanner found {sites} call sites, below the {MIN_SITES} floor"
    );
    hashes
}

#[test]
fn every_bespoke_hash_matches_a_shipped_script() {
    let shipped = shipped_hashes();
    let mut seen = HashSet::new();
    for entry in bespoke::entries() {
        assert!(
            seen.insert(entry.hash),
            "{entry:?} shares its hash with an earlier entry"
        );
        assert!(
            shipped.contains_key(entry.hash),
            "{entry:?} is keyed by {} and no cached_painless! literal in the tree \
             hashes to it -- the vendor script changed, or the hash was typed wrong",
            entry.hash
        );
    }
}

#[test]
fn a_registered_script_resolves_to_its_entry() {
    let shipped = shipped_hashes();
    for entry in bespoke::entries() {
        let file = &shipped[entry.hash];
        let src = std::fs::read_to_string(file).unwrap();
        let mut from = 0;
        let mut found = false;
        while let Some(hit) = src[from..].find(SITE) {
            let after = from + hit + SITE.len();
            let (script, end) = raw_literal(&src, after).unwrap();
            let plan = PainlessPlan::new(&script);
            if bespoke::script_hash(plan.text()) == entry.hash {
                assert_eq!(
                    plan.binding().first().map(String::as_str),
                    Some(format!("{entry:?}").as_str())
                );
                found = true;
            }
            from = end;
        }
        assert!(found, "{entry:?}: {file} no longer carries its script");
    }
}
