// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The registry against the tree it dispatches into, and against upstream.
//!
//! `registry.rs` is hand-maintained on purpose -- a stream's `intake()` and
//! dataset are judgements the generator cannot make. The direction that cannot
//! rot is registry to module: an entry naming a module that does not exist
//! fails to compile. The other two directions are checked by nothing, and a
//! generated module with no entry is unreachable at runtime in silence.
//!
//! So this asserts the two unchecked directions rather than generating the
//! table, which would move the same boundary without closing it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use dfe_transform_elastic::registry;

/// Below this the scan has broken rather than the tree having shrunk.
const MIN_DATA_STREAMS: usize = 900;

fn generated_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("crates/dfe-transforms/src/filebeat")
}

/// The registry name for a data-stream directory.
///
/// A package starting with a digit cannot be a Rust identifier, so the module
/// carries a leading underscore the registry name does not: `_1password_audit`
/// is registered as `1password_audit`.
fn registry_name(directory: &str) -> String {
    format!(
        "filebeat.{}.default",
        directory.strip_prefix('_').unwrap_or(directory)
    )
}

fn data_stream_directories() -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let root = generated_root();
    let entries = std::fs::read_dir(&root)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", root.display()));
    for entry in entries.flatten() {
        if entry.path().join("default.rs").is_file()
            && let Some(name) = entry.file_name().to_str()
        {
            found.insert(name.to_owned());
        }
    }
    found
}

/// A generated entry point with no registry entry cannot be deployed, and
/// nothing else says so.
#[test]
fn every_generated_data_stream_is_reachable_through_the_registry() {
    let directories = data_stream_directories();
    assert!(
        directories.len() >= MIN_DATA_STREAMS,
        "found {} data streams under {}, expected at least {MIN_DATA_STREAMS} -- the scan is broken",
        directories.len(),
        generated_root().display()
    );

    let unreachable: Vec<String> = directories
        .iter()
        .map(|directory| registry_name(directory))
        .filter(|name| registry::lookup(name).is_none())
        .collect();

    assert!(
        unreachable.is_empty(),
        "{} generated data stream(s) have no registry entry, so no deployment can name them:\n  {}",
        unreachable.len(),
        unreachable.join("\n  ")
    );
}

/// Vendored data streams with no generated module: onboarding backlog, not
/// exclusions. Every one carries a `default.yml`, so every one is generatable.
///
/// Removing a line here is what onboarding a source looks like. A name
/// arriving that is not on this list is upstream having added a data stream,
/// which is the churn this file exists to make loud.
const NOT_YET_ONBOARDED: &[&str] = &[
    "azure/aadgraphactivitylogs",
    "checkpoint/firewall",
    "cribl/logs",
    "cribl/metrics",
    "crowdstrike/fdr",
    "entityanalytics_entra_id/entity",
    "eset_protect/device",
    "jamf_protect/telemetry_legacy",
    "prometheus/remote_write",
];

/// `DFE_VENDORED_PIPELINES` names the vendored pipelines tree this one is
/// generated from. A data stream upstream has added shows up here as a name
/// with no module, rather than as a line in a regeneration report nobody reads.
///
/// Skipped when the variable is unset, which is a fresh clone and every CI
/// runner.
#[test]
fn every_vendored_pipeline_has_a_generated_module() {
    let Some(pipelines) = std::env::var_os("DFE_VENDORED_PIPELINES") else {
        eprintln!("DFE_VENDORED_PIPELINES unset -- skipped");
        return;
    };
    let pipelines = Path::new(&pipelines);
    assert!(
        pipelines.is_dir(),
        "DFE_VENDORED_PIPELINES={} is not a directory",
        pipelines.display()
    );

    let generated = data_stream_directories();
    let mut missing = Vec::new();
    let mut seen = 0usize;

    for package in std::fs::read_dir(pipelines).into_iter().flatten().flatten() {
        let Some(package_name) = package.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        for stream in std::fs::read_dir(package.path())
            .into_iter()
            .flatten()
            .flatten()
        {
            if !stream.path().is_dir() {
                continue;
            }
            let Some(stream_name) = stream.file_name().to_str().map(str::to_owned) else {
                continue;
            };
            seen += 1;
            let directory = format!("{package_name}_{stream_name}");
            let prefixed = format!("_{directory}");
            if !generated.contains(&directory) && !generated.contains(&prefixed) {
                missing.push(format!("{package_name}/{stream_name}"));
            }
        }
    }

    assert!(
        seen > 0,
        "read no data streams under {}",
        pipelines.display()
    );
    missing.sort();

    let recorded: BTreeSet<&str> = NOT_YET_ONBOARDED.iter().copied().collect();
    let found: BTreeSet<&str> = missing.iter().map(String::as_str).collect();

    let arrived: Vec<&str> = found.difference(&recorded).copied().collect();
    assert!(
        arrived.is_empty(),
        "{} vendored data stream(s) of {seen} have no generated module and are not recorded -- \
         upstream has added them. Regenerate, then give each a mod.rs entry, a registry entry \
         and an Origin:\n  {}",
        arrived.len(),
        arrived.join("\n  ")
    );

    let onboarded: Vec<&str> = recorded.difference(&found).copied().collect();
    assert!(
        onboarded.is_empty(),
        "{} recorded stream(s) now have a module -- drop them from NOT_YET_ONBOARDED:\n  {}",
        onboarded.len(),
        onboarded.join("\n  ")
    );
}
