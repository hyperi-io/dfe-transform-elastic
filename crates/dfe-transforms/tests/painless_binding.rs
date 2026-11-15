// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Which matcher every generated Painless call site resolves to, read from the
//! plan the generator emitted rather than from a fixture run.
//!
//! Each site carries the plan the generator resolved, keyed by its script's
//! hash. This reads those plans back out of the generated tree and asks each
//! what it binds to.
//!
//! Unlike `painless_coverage.rs`, nothing here executes: a script whose package
//! ships no fixture is measured the same as one that runs on every event.
//!
//! `DFE_BINDING_DUMP=<path>` writes the full binding as JSON.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod sites;

use std::collections::BTreeMap;
use std::path::Path;

use serde_json::json;

/// Where the ratchets live, relative to this crate.
fn workspace_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// What one distinct script binds to, and where it is used.
struct Script {
    binding: Vec<String>,
    /// Whether the generator could emit this script's runners instead of the
    /// plan. Asked of the plan rather than kept as a list here, so the
    /// allowlist has one home.
    direct: bool,
    /// Whether a bound runner still reads the script text.
    reads_text: bool,
    uses: Vec<(String, usize)>,
}

#[test]
fn every_call_site_reports_the_matcher_it_binds_to() {
    let sites = sites::sites();

    // Keyed on the hash so a script shared by twenty packages is counted once.
    let mut scripts: BTreeMap<String, Script> = BTreeMap::new();
    for site in &sites {
        scripts
            .entry(site.hash.clone())
            .or_insert_with(|| Script {
                binding: site.plan.binding(),
                direct: site.plan.direct_call().is_some(),
                reads_text: site.plan.reads_text(),
                uses: Vec::new(),
            })
            .uses
            .push((site.file.clone(), site.line));
    }

    let unbound: Vec<&Script> = scripts.values().filter(|s| s.binding.is_empty()).collect();
    let unbound_sites: usize = unbound.iter().map(|s| s.uses.len()).sum();
    let mut families: BTreeMap<&str, usize> = BTreeMap::new();
    for script in scripts.values() {
        for pattern in &script.binding {
            let name = pattern.split(['(', ' ']).next().unwrap_or(pattern);
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
        .filter(|s| s.binding.iter().any(|pattern| pattern.contains("Never")))
        .map(|s| s.uses.len())
        .sum();
    println!("  {dead_branches} sites carry a guard the evaluator cannot read");
    // Counted statically, so it includes sites where the dead branch sits in a
    // FALLBACK pattern and something ahead of it handles the script correctly.
    let mut ratchets = dfe_runtime::testutil::ratchets::Ratchets::load(&workspace_root());
    ratchets.check(
        "dead_branch_sites",
        dead_branches,
        "A new pattern or script has added a dead branch. Establish what the \
         script binds FIRST before calling it a defect -- the pattern carrying \
         it is often a fallback something else handles, and a RISE can mean \
         the count got truer rather than the code worse. The corpus is the \
         authority on which; this number never is.",
    );
    ratchets.save();

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
                    "reads_text": script.reads_text,
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
