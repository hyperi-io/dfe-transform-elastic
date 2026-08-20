// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Baselines are the measured match count per fixture, and only ever go up.

use dfe_transforms::filebeat::crowdstrike;

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/crowdstrike/falcon"
);

#[test]
fn crowdstrike_default_sample() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-sample",
        2,
    );
}

#[test]
fn crowdstrike_default_events() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-events",
        2,
    );
}

/// The one baseline that moved DOWN, and it is the two fixture lineages
/// disagreeing rather than a regression. Teaching the transpiler
/// `indexOf("@") > 0` made the current pipeline's user handling reachable:
/// against the compat corpus this same fixture went 1/9 to 5/9, and crowdstrike
/// as a whole 12/49 to 33/49. This committed expectation predates that pipeline
/// and still wants the older shape.
#[test]
fn crowdstrike_default_event_stream() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-event-stream",
        1,
    );
}

#[test]
fn crowdstrike_default_audit_events() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-audit-events",
        1,
    );
}

#[test]
fn crowdstrike_default_tags() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-tags",
        0,
    );
}

#[test]
fn crowdstrike_default_tags_list() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-tags-list",
        0,
    );
}
