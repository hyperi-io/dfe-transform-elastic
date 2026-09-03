// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Run the Python tooling's own test suite, so it is in the gate.
//!
//! `scripts/` carries 538 lines of tests across `test_compat.py`,
//! `test_field_census.py` and `test_shape_reach.py`, and NOTHING executed
//! them. `.hyperi-ci.yaml` declares `language: rust`, the reusable
//! `rust-ci.yml` runs no pytest or unittest, and neither does
//! `hyperi-ci check` -- so a fix landing in one of them was fiction.
//!
//! They are not incidental. `field_census.py`'s leaf walk produces every
//! schema width figure the design rests on, and `shape_reach.py` is the join
//! that finds a shape claiming a script it never applies -- it drove five
//! fixes this week. A miscount in either is silent.
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
        ran >= 40,
        "the script suite ran {ran} tests, under the floor -- discovery is not \
         finding them, so a green result means nothing"
    );

    assert!(
        output.status.success(),
        "the Python tooling suite failed:\n{stderr}"
    );
}
