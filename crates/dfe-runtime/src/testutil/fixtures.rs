// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Where the repository's own test fixtures are.
//!
//! `tests/fixtures/` holds one directory per data stream: sample lines, their
//! test configs, and the documents the real Elastic engine produced for them.
//! `tests/fixtures/PROVENANCE.md` says how they are made.

use std::path::{Path, PathBuf};

/// The workspace root, from this crate's own manifest.
#[must_use]
pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// The repository's `tests/fixtures/`.
#[must_use]
pub fn fixtures() -> PathBuf {
    workspace_root().join("tests/fixtures")
}

/// The repository's `tests/envelopes/`.
#[must_use]
pub fn envelopes() -> PathBuf {
    workspace_root().join("tests/envelopes")
}

/// The documents the real Elastic engine produced for a fixture, read from its
/// `-expected.json`.
///
/// # Panics
///
/// Panics where the file is missing or carries no `expected` array, so a test
/// pointed at the wrong fixture names it.
#[allow(
    clippy::panic,
    reason = "a test pointed at a missing expectation has nothing to report but the path"
)]
#[must_use]
pub fn expected_documents(path: &Path) -> Vec<serde_json::Value> {
    let text =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{} reads: {e}", path.display()));
    let file: serde_json::Value = serde_json::from_str(&text)
        .unwrap_or_else(|e| panic!("{}: not valid JSON: {e}", path.display()));
    match file.get("expected") {
        Some(serde_json::Value::Array(docs)) => docs.clone(),
        _ => panic!("{} carries no `expected` array", path.display()),
    }
}
