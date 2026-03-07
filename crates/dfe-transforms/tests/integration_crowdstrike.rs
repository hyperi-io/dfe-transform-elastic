// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Integration tests for CrowdStrike transforms against fixture data.

use std::path::Path;

use dfe_runtime::testutil::diff::MatchMode;
use dfe_runtime::testutil::harness::run_transform_test;
use dfe_runtime::transform::Transform;
use dfe_transforms::filebeat::crowdstrike;

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/crowdstrike/falcon"
);

fn run_fixture(transform: &dyn Transform, log_name: &str) {
    let log_path = Path::new(FIXTURE_DIR).join(format!("{log_name}.log"));
    let expected_path = Path::new(FIXTURE_DIR).join(format!("{log_name}.log-expected.json"));

    if !log_path.exists() {
        panic!("fixture not found: {}", log_path.display());
    }
    if !expected_path.exists() {
        panic!("expected output not found: {}", expected_path.display());
    }

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
fn crowdstrike_default_sample() {
    run_fixture(&crowdstrike::default::Default, "test-falcon-sample");
}

#[test]
fn crowdstrike_default_events() {
    run_fixture(&crowdstrike::default::Default, "test-falcon-events");
}

#[test]
fn crowdstrike_default_event_stream() {
    run_fixture(&crowdstrike::default::Default, "test-event-stream");
}

#[test]
fn crowdstrike_default_audit_events() {
    run_fixture(&crowdstrike::default::Default, "test-falcon-audit-events");
}

#[test]
fn crowdstrike_default_tags() {
    run_fixture(&crowdstrike::default::Default, "test-falcon-tags");
}

#[test]
fn crowdstrike_default_tags_list() {
    run_fixture(&crowdstrike::default::Default, "test-falcon-tags-list");
}
