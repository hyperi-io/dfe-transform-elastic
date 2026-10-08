// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Where the Elastic test data is, on a run that has it.
//!
//! Sample logs and expectations copied from Elastic's repositories are Elastic
//! License 2.0 material and cannot ship in this repository. They are kept
//! outside it, in a `fixtures/elastic/` directory laid out the way this
//! repository's root is: `tests/fixtures/` and `tests/envelopes/` beneath it. A
//! test that reads them is `#[ignore]`d, and runs with `DFE_ELASTIC_FIXTURES`
//! pointed at that directory and `--ignored`.

use std::path::PathBuf;

/// The variable naming the Elastic test data's root.
pub const ELASTIC_FIXTURES_ENV: &str = "DFE_ELASTIC_FIXTURES";

/// The Elastic test data's root, or `None` where the variable is unset.
///
/// # Panics
///
/// Panics where the variable is set but names no directory holding
/// `tests/fixtures`, so a run that asked for the data cannot pass on none of it.
#[allow(
    clippy::panic,
    reason = "a test pointed at missing data has nothing to report but the path"
)]
#[must_use]
pub fn elastic_root() -> Option<PathBuf> {
    let root = PathBuf::from(std::env::var_os(ELASTIC_FIXTURES_ENV)?);
    assert!(
        root.join("tests/fixtures").is_dir(),
        "{ELASTIC_FIXTURES_ENV}={} holds no tests/fixtures directory",
        root.display()
    );
    Some(root)
}

/// The Elastic test data's root, for a test that cannot run without it.
///
/// # Panics
///
/// Panics where the variable is unset, and wherever [`elastic_root`] does.
#[allow(
    clippy::panic,
    reason = "an Elastic-data test run without the data must fail, not pass empty"
)]
#[must_use]
pub fn require_elastic_root() -> PathBuf {
    elastic_root().unwrap_or_else(|| {
        panic!(
            "{ELASTIC_FIXTURES_ENV} is unset. This test reads Elastic-licensed data \
             kept outside this repository: set it to that data's \
             fixtures/elastic directory"
        )
    })
}

/// The Elastic test data's `tests/fixtures/`, for a test that cannot run
/// without it.
///
/// # Panics
///
/// Panics wherever [`require_elastic_root`] does.
#[must_use]
pub fn elastic_fixtures() -> PathBuf {
    require_elastic_root().join("tests/fixtures")
}
