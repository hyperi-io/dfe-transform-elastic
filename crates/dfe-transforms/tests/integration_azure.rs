// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Integration tests for Azure transforms against fixture data.

use std::path::Path;

use dfe_runtime::testutil::diff::MatchMode;
use dfe_runtime::testutil::harness::run_transform_test;
use dfe_runtime::transform::Transform;
use dfe_transforms::filebeat::{azure_activitylogs, azure_auditlogs, azure_signinlogs};

const FIXTURE_BASE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/azure");

fn run_fixture(transform: &dyn Transform, subdir: &str, log_name: &str) {
    let dir = Path::new(FIXTURE_BASE).join(subdir);
    let log_path = dir.join(format!("{log_name}.log"));
    let expected_path = dir.join(format!("{log_name}.log-expected.json"));

    let result = run_transform_test(&log_path, &expected_path, transform, MatchMode::Subset)
        .unwrap_or_else(|e| panic!("transform failed: {e}"));

    println!(
        "[{}] {}/{} events matched (fixture: {subdir}/{log_name})",
        transform.name(),
        result.passed,
        result.total,
    );

    if !result.all_passed() {
        println!("{result}");
    }
}

#[test]
fn azure_activitylogs_default() {
    run_fixture(
        &azure_activitylogs::default::Default,
        "activitylogs",
        "activitylogs",
    );
}

#[test]
fn azure_auditlogs_default() {
    run_fixture(&azure_auditlogs::default::Default, "auditlogs", "auditlogs");
}

#[test]
fn azure_signinlogs_default() {
    run_fixture(
        &azure_signinlogs::default::Default,
        "signinlogs",
        "signinlogs",
    );
}
