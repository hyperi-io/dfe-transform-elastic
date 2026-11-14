// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Run the Python tooling's own test suite and the generated tables' drift
//! check, so both are in the gate.
//!
//! `scripts/` carries tests beside its tools, and `.hyperi-ci.yaml` declares
//! `language: rust`, so the reusable `rust-ci.yml` runs no pytest or unittest
//! and neither does `hyperi-ci check`. This test is what runs them.
//!
//! `field_census.py`'s leaf walk produces every schema width figure the design
//! rests on, so a miscount there is silent without it.
//!
//! Shelling out is the way in because `hyperi-ci` offers no per-repo hook for
//! a second language and lives in another repository. `python3` is present by
//! construction: `hyperi-ci` is itself a Python program, so a runner that can
//! run the gate can run this.

use std::path::Path;
use std::process::Command;

/// Repository root, one level above this crate's manifest.
fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn the_python_tooling_suite_passes() {
    let scripts = repo_root().join("scripts");
    assert!(
        scripts.is_dir(),
        "no scripts/ directory at {} -- this test is looking in the wrong place",
        scripts.display()
    );

    let output = Command::new("python3")
        .args([
            "-m",
            "unittest",
            "discover",
            "-s",
            "scripts",
            "-p",
            "test_*.py",
            "-v",
        ])
        .current_dir(repo_root())
        .output();

    let output = match output {
        Ok(output) => output,
        Err(err) => {
            // CI runs hyperi-ci, which is Python, so python3 missing there is
            // a broken runner rather than an unsupported environment.
            assert!(
                std::env::var_os("CI").is_none(),
                "python3 will not start in CI ({err}) -- the gate itself is a \
                 Python program, so this must RUN here, not skip"
            );
            eprintln!("python3 will not start ({err}); skipping the script suite");
            return;
        }
    };

    let stderr = String::from_utf8_lossy(&output.stderr);
    // unittest writes its progress and its summary to stderr.
    println!("{stderr}");

    // A discover that matches nothing reports OK over zero tests, which is the
    // same vacuous green this file exists to remove.
    let ran: usize = stderr
        .lines()
        .find_map(|line| line.strip_prefix("Ran "))
        .and_then(|rest| rest.split_whitespace().next())
        .and_then(|count| count.parse().ok())
        .unwrap_or(0);
    assert!(
        ran >= 35,
        "the script suite ran {ran} tests, under the floor -- discovery is not \
         finding them, so a green result means nothing"
    );

    assert!(
        output.status.success(),
        "the Python tooling suite failed:\n{stderr}"
    );
}

/// `src/detect/labels.rs` and `src/detect/markers.rs` match what the pinned
/// dfe-schemas wheel generates, so a pin bump without a regeneration fails here.
/// The wheel comes from `DFE_SCHEMAS_WHEEL`, the digest-named cache or `PyPI`. With
/// none of them (exit 3) the check skips locally and fails in CI.
#[test]
fn the_detect_tables_match_the_pinned_catalogue() {
    let output = Command::new("python3")
        .args(["scripts/gen_detect_tables.py", "--check"])
        .current_dir(repo_root())
        .output();
    let output = match output {
        Ok(output) => output,
        Err(err) => {
            assert!(
                std::env::var_os("CI").is_none(),
                "python3 will not start in CI ({err}), so the drift check cannot be skipped"
            );
            eprintln!("python3 will not start ({err}); skipping the drift check");
            return;
        }
    };
    let stderr = String::from_utf8_lossy(&output.stderr);
    if output.status.code() == Some(3) {
        assert!(
            std::env::var_os("CI").is_none(),
            "CI cannot reach PyPI for the pinned dfe-schemas wheel:\n{stderr}"
        );
        eprintln!("no pinned dfe-schemas wheel available; skipping the drift check:\n{stderr}");
        return;
    }
    assert!(
        output.status.success(),
        "the generated detect tables are stale or could not be generated:\n{stderr}"
    );
}
