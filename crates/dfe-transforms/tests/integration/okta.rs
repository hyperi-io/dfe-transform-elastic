// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

use dfe_transforms::filebeat::okta;

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../testdata/integrations/okta/system"
);

#[test]
fn okta_default_system_events() {
    super::common::run_fixture(
        &okta::default::Default,
        FIXTURE_DIR,
        "test-okta-system-events",
    );
}
