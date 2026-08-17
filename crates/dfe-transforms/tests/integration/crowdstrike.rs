// SPDX-License-Identifier: FSL-1.1-ALv2
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
        7,
    );
}

#[test]
fn crowdstrike_default_events() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-events",
        3,
    );
}

#[test]
fn crowdstrike_default_event_stream() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-event-stream",
        8,
    );
}

#[test]
fn crowdstrike_default_audit_events() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-audit-events",
        13,
    );
}

// CSPM events need sub-pipeline routing and ResourceAttributes JSON decode,
// neither of which the transform does yet.
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
