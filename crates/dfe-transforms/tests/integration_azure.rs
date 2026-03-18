// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Integration tests for Azure transforms against fixture data.

mod common;

use dfe_transforms::filebeat::{azure_activitylogs, azure_auditlogs, azure_signinlogs};

const FIXTURE_BASE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../testdata/integrations/azure"
);

#[test]
fn azure_activitylogs_raw() {
    let dir = format!("{FIXTURE_BASE}/activitylogs");
    common::run_fixture(
        &azure_activitylogs::default::Default,
        &dir,
        "test-activitylogs-raw",
    );
}

#[test]
fn azure_activitylogs_identity() {
    let dir = format!("{FIXTURE_BASE}/activitylogs");
    common::run_fixture(
        &azure_activitylogs::default::Default,
        &dir,
        "test-activitylogs-identity",
    );
}

#[test]
fn azure_activitylogs_edgecases() {
    let dir = format!("{FIXTURE_BASE}/activitylogs");
    common::run_fixture(
        &azure_activitylogs::default::Default,
        &dir,
        "test-activitylogs-edgecases",
    );
}

#[test]
fn azure_auditlogs_raw() {
    let dir = format!("{FIXTURE_BASE}/auditlogs");
    common::run_fixture(
        &azure_auditlogs::default::Default,
        &dir,
        "test-auditlogs-raw",
    );
}

#[test]
fn azure_signinlogs_raw() {
    let dir = format!("{FIXTURE_BASE}/signinlogs");
    common::run_fixture(
        &azure_signinlogs::default::Default,
        &dir,
        "test-signinlogs-raw",
    );
}

#[test]
fn azure_signinlogs_sample() {
    let dir = format!("{FIXTURE_BASE}/signinlogs");
    common::run_fixture(
        &azure_signinlogs::default::Default,
        &dir,
        "test-signinlogs-sample",
    );
}
