// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Integration tests for Okta transforms against fixture data.

mod common;

use dfe_transforms::filebeat::okta;

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../testdata/integrations/okta/system"
);

#[test]
fn okta_default_system_events() {
    common::run_fixture(
        &okta::default::Default,
        FIXTURE_DIR,
        "test-okta-system-events",
    );
}
