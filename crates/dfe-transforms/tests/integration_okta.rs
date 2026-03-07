// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Integration tests for Okta transforms against fixture data.

use std::path::Path;

use dfe_runtime::testutil::diff::MatchMode;
use dfe_runtime::testutil::harness::run_transform_test;
use dfe_runtime::transform::Transform;
use dfe_transforms::filebeat::okta;

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/okta/system"
);

fn run_fixture(transform: &dyn Transform, log_name: &str) {
    let log_path = Path::new(FIXTURE_DIR).join(format!("{log_name}.log"));
    let expected_path = Path::new(FIXTURE_DIR).join(format!("{log_name}.log-expected.json"));

    let result = run_transform_test(&log_path, &expected_path, transform, MatchMode::Subset)
        .unwrap_or_else(|e| panic!("transform failed: {e}"));

    println!(
        "[{}] {}/{} events matched (fixture: {log_name})",
        transform.name(),
        result.passed,
        result.total,
    );

    if !result.all_passed() {
        println!("{result}");
    }
}

#[test]
fn okta_default_system_events() {
    run_fixture(&okta::default::Default, "test-okta-system-events");
}
