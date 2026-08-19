// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Baselines are the measured match count per fixture, and only ever go up.

use dfe_transforms::filebeat::okta;

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/okta/system"
);

// The two unmatched events are a device_integrator JSON parse and a
// target detailEntry edge case.
#[test]
fn okta_default_system_events() {
    super::common::run_fixture(
        &okta::default::Default,
        FIXTURE_DIR,
        "test-okta-system-events",
        24,
    );
}
