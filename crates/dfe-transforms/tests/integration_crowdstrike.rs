// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Integration tests for CrowdStrike transforms against fixture data.

mod common;

use dfe_transforms::filebeat::crowdstrike;

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../testdata/integrations/crowdstrike/falcon"
);

#[test]
fn crowdstrike_default_sample() {
    common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-sample",
    );
}

#[test]
fn crowdstrike_default_events() {
    common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-events",
    );
}

#[test]
fn crowdstrike_default_event_stream() {
    common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-event-stream",
    );
}

#[test]
fn crowdstrike_default_audit_events() {
    common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-audit-events",
    );
}

#[test]
fn crowdstrike_default_tags() {
    common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-tags",
    );
}

#[test]
fn crowdstrike_default_tags_list() {
    common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-tags-list",
    );
}
