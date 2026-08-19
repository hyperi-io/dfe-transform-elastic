// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Baselines are the measured match count per fixture, and only ever go up.
//!
//! Azure is 10/10.

use dfe_transforms::filebeat::{azure_activitylogs, azure_auditlogs, azure_signinlogs};

const FIXTURE_BASE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/azure");

#[test]
fn azure_activitylogs_raw() {
    let dir = format!("{FIXTURE_BASE}/activitylogs");
    super::common::run_fixture(
        &azure_activitylogs::default::Default,
        &dir,
        "test-activitylogs-raw",
        1,
    );
}

#[test]
fn azure_activitylogs_identity() {
    let dir = format!("{FIXTURE_BASE}/activitylogs");
    super::common::run_fixture(
        &azure_activitylogs::default::Default,
        &dir,
        "test-activitylogs-identity",
        1,
    );
}

#[test]
fn azure_activitylogs_edgecases() {
    let dir = format!("{FIXTURE_BASE}/activitylogs");
    super::common::run_fixture(
        &azure_activitylogs::default::Default,
        &dir,
        "test-activitylogs-edgecases",
        3,
    );
}

#[test]
fn azure_auditlogs_raw() {
    let dir = format!("{FIXTURE_BASE}/auditlogs");
    super::common::run_fixture(
        &azure_auditlogs::default::Default,
        &dir,
        "test-auditlogs-raw",
        1,
    );
}

#[test]
fn azure_signinlogs_raw() {
    let dir = format!("{FIXTURE_BASE}/signinlogs");
    super::common::run_fixture(
        &azure_signinlogs::default::Default,
        &dir,
        "test-signinlogs-raw",
        2,
    );
}

#[test]
fn azure_signinlogs_sample() {
    let dir = format!("{FIXTURE_BASE}/signinlogs");
    super::common::run_fixture(
        &azure_signinlogs::default::Default,
        &dir,
        "test-signinlogs-sample",
        2,
    );
}
