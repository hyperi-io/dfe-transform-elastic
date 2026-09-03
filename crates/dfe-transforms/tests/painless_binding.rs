// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Which matcher every generated Painless call site resolves to, decided from
//! the source literal rather than from a fixture run.
//!
//! The runtime ladder re-derives that answer from the script TEXT on first
//! touch. This reads the same literals straight out of the generated tree and
//! asks [`PainlessPlan`] what each one binds to.
//!
//! Unlike `painless_coverage.rs`, nothing here executes: a script whose package
//! ships no fixture is measured the same as one that runs on every event.
//!
//! `DFE_BINDING_DUMP=<path>` writes the full binding as JSON.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use dfe_runtime::painless_plan::PainlessPlan;
use serde_json::json;
use sha2::{Digest, Sha256};

/// The macro every generated call site wraps its script literal in.
const SITE: &str = "cached_painless!(";

/// Below this the scanner has stopped seeing the tree rather than the tree
/// having shrunk -- 3,490 sites when this was written.
const MIN_SITES: usize = 3_000;

/// Call sites whose bound shape carries a branch that can never be taken.
///
/// Counted statically, so it includes sites where the dead branch sits in a
/// FALLBACK shape and something ahead of it handles the script correctly --
/// `jamf_protect_telemetry` still counts here after its fix, because
/// `Basename` now wins at run time while `GuardedCopy` still binds behind it.
/// Ratchets down as guards are made readable, never as evidence on its own.
const DEAD_BRANCH_SITES: usize = 203;

/// One generated call site.
struct Site {
    file: String,
    line: usize,
    script: String,
}

/// Every `.rs` file under `dir`, depth first.
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

/// The raw-string literal starting at or after `at`, and where it ends.
///
/// Every call site passes a raw string, so the delimiter is `r` plus a run of
/// `#` -- counted, because the closing run has to match it exactly or a script
/// containing `"#` would truncate here and bind to the wrong matcher.
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

/// Every call site in the generated tree, in file order.
fn sites(root: &Path) -> Vec<Site> {
    let mut files = Vec::new();
    rust_files(root, &mut files);
    files.sort();

    let mut found = Vec::new();
    for file in files {
        let Ok(src) = std::fs::read_to_string(&file) else {
            continue;
        };
        let name = file
            .strip_prefix(root)
            .unwrap_or(&file)
            .display()
            .to_string();
        let mut from = 0;
        while let Some(hit) = src[from..].find(SITE) {
            let after = from + hit + SITE.len();
            let Some((script, end)) = raw_literal(&src, after) else {
                panic!("{name}: cached_painless! at byte {after} has no raw-string literal");
            };
            found.push(Site {
                file: name.clone(),
                line: src[..after].matches('\n').count() + 1,
                script,
            });
            from = end;
        }
    }
    found
}

/// The generated transforms live beside this crate's `tests/`.
fn transforms_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// What one distinct script binds to, and where it is used.
struct Script {
    binding: Vec<String>,
    /// Whether the generator could emit this script's runners instead of the
    /// script. Asked of the plan rather than kept as a list here, so the
    /// allowlist has one home.
    direct: bool,
    chars: usize,
    head: String,
    uses: Vec<(String, usize)>,
}

#[test]
fn every_call_site_reports_the_matcher_it_binds_to() {
    let sites = sites(&transforms_root());
    assert!(
        sites.len() >= MIN_SITES,
        "found only {} call sites under {} -- the scanner is blind, not the tree empty",
        sites.len(),
        transforms_root().display()
    );

    // Keyed on the literal so a script shared by twenty packages is planned
    // once.
    let mut scripts: BTreeMap<String, Script> = BTreeMap::new();
    for site in &sites {
        let key = format!("{:x}", Sha256::digest(site.script.as_bytes()));
        scripts
            .entry(key)
            .or_insert_with(|| {
                let plan = PainlessPlan::new(&site.script);
                Script {
                    binding: plan.binding(),
                    direct: plan.direct_call().is_some(),
                    chars: plan.text().chars().count(),
                    head: plan.text().chars().take(120).collect(),
                    uses: Vec::new(),
                }
            })
            .uses
            .push((site.file.clone(), site.line));
    }

    let unbound: Vec<&Script> = scripts.values().filter(|s| s.binding.is_empty()).collect();
    let unbound_sites: usize = unbound.iter().map(|s| s.uses.len()).sum();
    let mut families: BTreeMap<&str, usize> = BTreeMap::new();
    for script in scripts.values() {
        for shape in &script.binding {
            let name = shape.split(['(', ' ']).next().unwrap_or(shape);
            *families.entry(name).or_default() += script.uses.len();
        }
    }

    println!(
        "painless binding: {} call sites, {} distinct scripts, {} matcher families",
        sites.len(),
        scripts.len(),
        families.len()
    );
    println!(
        "  unbound: {} distinct, {unbound_sites} sites",
        unbound.len()
    );
    let direct_sites: usize = scripts
        .values()
        .filter(|s| s.direct)
        .map(|s| s.uses.len())
        .sum();
    println!(
        "  the generator can emit {direct_sites} sites directly ({}%)",
        direct_sites * 100 / sites.len()
    );
    // A guard the evaluator cannot read never holds, so the branch behind it
    // is dead and the script takes the other one whatever the data says.
    let dead_branches: usize = scripts
        .values()
        .filter(|s| s.binding.iter().any(|shape| shape.contains("Never")))
        .map(|s| s.uses.len())
        .sum();
    println!("  {dead_branches} sites carry a guard the evaluator cannot read");
    assert!(
        dead_branches <= DEAD_BRANCH_SITES,
        "{dead_branches} sites now carry an unreadable guard, up from \
         {DEAD_BRANCH_SITES} -- a new shape or script has added a dead branch. \
         Establish what the script binds FIRST before calling it a defect: the \
         shape carrying the dead branch is often a fallback something else \
         handles."
    );

    let mut ranked: Vec<(&&str, &usize)> = families.iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(a.1));
    for (name, count) in ranked.iter().take(15) {
        println!("  {count:>5} sites  {name}");
    }

    if let Ok(path) = std::env::var("DFE_BINDING_DUMP") {
        let rows: Vec<serde_json::Value> = scripts
            .iter()
            .map(|(sha, script)| {
                json!({
                    "sha256": sha,
                    "chars": script.chars,
                    "head": script.head,
                    "binding": script.binding,
                    "direct": script.direct,
                    "uses": script.uses.iter()
                        .map(|(file, line)| json!({ "file": file, "line": line }))
                        .collect::<Vec<_>>(),
                })
            })
            .collect();
        let dump = json!({
            "sites": sites.len(),
            "distinct": scripts.len(),
            "unbound_distinct": unbound.len(),
            "unbound_sites": unbound_sites,
            "direct_sites": direct_sites,
            "families": families,
            "scripts": rows,
        });
        std::fs::write(&path, serde_json::to_string_pretty(&dump).unwrap()).unwrap();
        println!("  wrote {path}");
    }
}
