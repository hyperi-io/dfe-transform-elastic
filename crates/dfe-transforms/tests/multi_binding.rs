// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Scripts that bind to MORE THAN ONE pattern, across the whole generated tree.
//!
//! Dispatch runs the params pattern, then the text patterns in order, and stops
//! at the first that returns `true` (`painless_plan.rs:126-146`). A runner that
//! cannot decline therefore shadows every pattern after it. gdacs lost
//! `BranchCopies` that way for as long as the source existed: its
//! `geo_point_from_coordinates` wrote the centroid, returned `true`
//! unconditionally, and the two field copies behind it never ran.
//!
//! Scanning for runners that never return `false` is the WRONG search -- about
//! 170 do, and it is harmless where the runner is its script's only binding.
//! The hazard needs BOTH halves, so this counts the other one.
//!
//! The generated modules are the source of script text on purpose: they carry
//! the literal in full, where `DFE_PAINLESS_UNHANDLED` groups scripts by their
//! first 200 characters and cannot be parsed back.

use std::collections::BTreeSet;
use std::path::Path;

use dfe_runtime::painless_plan::PainlessPlan;

/// Scripts binding to more than one pattern, as of 2026-09-07.
///
/// **Not a defect count.** Where the first pattern DECLINES, dispatch falls
/// through and the second runs correctly -- 20 of these are
/// `SentinelRemoval + SentinelRemovalLiteral`, and the params half declines on
/// a script carrying no `params` block. The number is the SEARCH SPACE for
/// shadowing, cut from 2,456 call sites to this.
///
/// A FLOOR, not a ratchet: it moves when the ladder gains or loses an arm, not
/// on ordinary parity work. A RISE means a script that used to resolve to one
/// matcher now resolves to two -- check whether the first returns `true`
/// without doing the whole job, which is how gdacs lost `BranchCopies`.
const MULTI_BINDING: usize = 62;

/// The `cached_painless!` literals a generated file holds, in full.
///
/// rustfmt breaks a long macro call across lines, so only 578 of the 2,456
/// call sites spell `cached_painless!(r#"` contiguously. The whitespace skip
/// is what reaches the rest.
fn scripts_in(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = text;
    while let Some(at) = rest.find("cached_painless!(") {
        let after = &rest[at + "cached_painless!(".len()..];
        let opened = after.trim_start();
        let Some(body) = opened.strip_prefix("r#\"") else {
            rest = after;
            continue;
        };
        let Some(end) = body.find("\"#") else {
            break;
        };
        found.push(body[..end].to_string());
        rest = &body[end..];
    }
    found
}

fn generated_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn every_script() -> BTreeSet<String> {
    let mut held = BTreeSet::new();
    let mut stack = vec![generated_root().join("src")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rs")
                && let Ok(text) = std::fs::read_to_string(&path)
            {
                held.extend(scripts_in(&text));
            }
        }
    }
    held
}

#[test]
fn the_scan_finds_scripts_at_all() {
    // A floor on the scan itself: a macro rename would empty it silently and
    // every assertion below would pass on nothing.
    let scripts = every_script();
    assert!(
        scripts.len() > 500,
        "only {} scripts found -- has cached_painless! been renamed?",
        scripts.len()
    );
}

#[test]
fn the_shadowing_search_space_has_not_grown() {
    let mut multi: Vec<(usize, String)> = every_script()
        .into_iter()
        .filter_map(|script| {
            let names = PainlessPlan::new(&script).binding();
            (names.len() > 1).then(|| {
                let heads: Vec<&str> = names
                    .iter()
                    .map(|name| name.split(['(', ' ', '{']).next().unwrap_or_default())
                    .collect();
                (
                    names.len(),
                    format!(
                        "{} <- {}",
                        heads.join(" + "),
                        &script[..script.len().min(72)]
                    ),
                )
            })
        })
        .collect();
    multi.sort();

    let report: Vec<&String> = multi.iter().map(|(_, line)| line).collect();
    assert_eq!(
        multi.len(),
        MULTI_BINDING,
        "multi-binding scripts moved from {MULTI_BINDING}:\n{report:#?}"
    );
}
