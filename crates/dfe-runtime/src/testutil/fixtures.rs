// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Where the repository's own test fixtures are.
//!
//! `tests/fixtures/` holds the unencumbered log samples and the synthetic set,
//! and `tests/fixtures/PROVENANCE.md` says where each comes from.

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
