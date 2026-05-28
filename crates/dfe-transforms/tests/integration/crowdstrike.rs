// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_transforms::filebeat::crowdstrike;

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../testdata/integrations/crowdstrike/falcon"
);

#[test]
fn crowdstrike_default_sample() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-sample",
    );
}

#[test]
fn crowdstrike_default_events() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-events",
    );
}

#[test]
fn crowdstrike_default_event_stream() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-event-stream",
    );
}

#[test]
fn crowdstrike_default_audit_events() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-audit-events",
    );
}

#[test]
fn crowdstrike_default_tags() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-tags",
    );
}

#[test]
fn crowdstrike_default_tags_list() {
    super::common::run_fixture(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-tags-list",
    );
}
