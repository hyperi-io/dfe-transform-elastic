// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! How far the shape ladder reaches into packages it has never seen.
//!
//! `painless_coverage.rs` measures the ladder against the packages it was
//! BUILT for, by running transforms over their fixtures. That number cannot
//! say what onboarding the next package costs, because a package with no
//! transform has nothing to run.
//!
//! This asks the question statically instead: take every Painless script in
//! every ingest pipeline Elastic ships, ask whether any matcher claims its
//! text, and report what nothing claims -- clustered, and ranked by how many
//! packages each cluster unblocks. That ranking is the worklist.
//!
//! Triggering is not the same as being RIGHT; `compat_corpus.rs` is what
//! measures that. A high reach here means the next package is mostly a
//! generate-and-verify job rather than a write-new-shapes job.
//!
//! Reads a clone it does not own, so it skips when the clone is absent.
//!
//! ```text
//! DFE_INTEGRATIONS=/projects/elastic-stuff/integrations \
//!     cargo test -p dfe-transforms --test shape_reach -- --nocapture
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use dfe_runtime::painless_plan::PainlessPlan;

/// Where the `elastic/integrations` clone lives.
fn clone_root() -> PathBuf {
    std::env::var_os("DFE_INTEGRATIONS").map_or_else(
        || PathBuf::from("/projects/elastic-stuff/integrations"),
        PathBuf::from,
    )
}

/// The `<package>/<data stream>` pairs `sources.yaml` already declares.
fn onboarded() -> BTreeSet<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../sources.yaml");
    let Ok(text) = std::fs::read_to_string(&path) else {
        return BTreeSet::new();
    };
    let Ok(doc) = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&text) else {
        return BTreeSet::new();
    };

    let mut pairs = BTreeSet::new();
    if let Some(sources) = doc
        .get("sources")
        .and_then(serde_yaml_ng::Value::as_mapping)
    {
        for source in sources.values() {
            let package = source.get("package").and_then(serde_yaml_ng::Value::as_str);
            let stream = source
                .get("data_stream")
                .and_then(serde_yaml_ng::Value::as_str);
            if let (Some(package), Some(stream)) = (package, stream) {
                pairs.insert(format!("{package}/{stream}"));
            }
        }
    }
    pairs
}

/// Every `script` processor's source, however deeply the pipeline nests it.
///
/// A `foreach` wraps its processor, and an `on_failure` holds a list of them,
/// so the walk is recursive rather than a scan of the top-level `processors`.
fn scripts_in(value: &serde_yaml_ng::Value, found: &mut Vec<String>) {
    match value {
        serde_yaml_ng::Value::Mapping(map) => {
            for (key, held) in map {
                if key.as_str() == Some("script")
                    && let Some(source) = held.get("source").and_then(serde_yaml_ng::Value::as_str)
                {
                    found.push(source.to_string());
                }
                scripts_in(held, found);
            }
        }
        serde_yaml_ng::Value::Sequence(items) => {
            for item in items {
                scripts_in(item, found);
            }
        }
        _ => {}
    }
}

/// A script's shape, with the details that vary between two copies removed.
///
/// Two scripts differing only in their field names and literals are ONE piece
/// of work, so the cluster key drops quoted text, digits and whitespace runs.
fn signature(script: &str) -> String {
    let mut out = String::with_capacity(120);
    let mut chars = script.chars().peekable();
    let mut space = false;

    while let Some(c) = chars.next() {
        if out.len() >= 110 {
            break;
        }
        match c {
            '"' | '\'' => {
                for skipped in chars.by_ref() {
                    if skipped == c {
                        break;
                    }
                }
                out.push('_');
            }
            c if c.is_ascii_digit() => out.push('#'),
            c if c.is_whitespace() => {
                if !space {
                    out.push(' ');
                }
                space = true;
                continue;
            }
            c => out.push(c),
        }
        space = false;
    }
    out
}

struct Reach {
    matched: usize,
    total: usize,
}

#[test]
fn how_far_the_ladder_reaches_into_unseen_packages() {
    let root = clone_root();
    if !root.join("packages").is_dir() {
        println!("no integrations clone at {}, skipping", root.display());
        return;
    }

    let known = onboarded();
    let mut per_package: BTreeMap<String, Reach> = BTreeMap::new();
    // The cluster, and every package that would be unblocked by claiming it.
    let mut unmatched: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut examples: BTreeMap<String, String> = BTreeMap::new();

    let packages = std::fs::read_dir(root.join("packages")).expect("packages directory");
    for package in packages.filter_map(Result::ok) {
        let name = package.file_name().to_string_lossy().into_owned();
        let streams = package.path().join("data_stream");
        let Ok(entries) = std::fs::read_dir(&streams) else {
            continue;
        };

        for stream in entries.filter_map(Result::ok) {
            let label = format!("{name}/{}", stream.file_name().to_string_lossy());
            if known.contains(&label) {
                continue;
            }
            let pipelines = stream.path().join("elasticsearch/ingest_pipeline");
            let Ok(files) = std::fs::read_dir(&pipelines) else {
                continue;
            };

            for file in files.filter_map(Result::ok) {
                let Ok(text) = std::fs::read_to_string(file.path()) else {
                    continue;
                };
                let Ok(doc) = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&text) else {
                    continue;
                };

                let mut found = Vec::new();
                scripts_in(&doc, &mut found);
                for script in found {
                    let reach = per_package.entry(name.clone()).or_insert(Reach {
                        matched: 0,
                        total: 0,
                    });
                    reach.total += 1;
                    if PainlessPlan::new(&script).matches() {
                        reach.matched += 1;
                    } else {
                        let key = signature(&script);
                        unmatched
                            .entry(key.clone())
                            .or_default()
                            .insert(name.clone());
                        examples.entry(key).or_insert(script);
                    }
                }
            }
        }
    }

    let matched: usize = per_package.values().map(|r| r.matched).sum();
    let total: usize = per_package.values().map(|r| r.total).sum();
    assert!(total > 0, "no scripts found under {}", root.display());

    println!("\n=== reach into un-onboarded packages ===");
    println!("packages with scripts : {}", per_package.len());
    println!("scripts               : {total}");
    println!(
        "claimed by a matcher  : {matched} ({:.1}%)",
        100.0 * matched as f64 / total as f64
    );
    println!("unmatched clusters    : {}", unmatched.len());

    let mut ranked: Vec<_> = unmatched.iter().collect();
    ranked.sort_by_key(|(key, packages)| (std::cmp::Reverse(packages.len()), (*key).clone()));

    println!("\n=== unmatched clusters, by packages unblocked ===");
    for (key, packages) in ranked.iter().take(40) {
        println!("  {:3} packages  {key}", packages.len());
    }

    println!("\n=== the ten widest clusters, in full ===");
    for (key, packages) in ranked.iter().take(10) {
        println!(
            "\n-- {} packages: {:?}",
            packages.len(),
            packages.iter().take(6).collect::<Vec<_>>()
        );
        println!("{}", examples[*key]);
    }

    println!("\n=== packages with the most unclaimed scripts ===");
    let mut worst: Vec<_> = per_package
        .iter()
        .map(|(name, reach)| (reach.total - reach.matched, name, reach))
        .collect();
    worst.sort_by_key(|(missing, name, _)| (std::cmp::Reverse(*missing), (*name).clone()));
    for (missing, name, reach) in worst.iter().take(30) {
        if *missing == 0 {
            break;
        }
        println!("  {missing:5} unclaimed of {:5}  {name}", reach.total);
    }
}
