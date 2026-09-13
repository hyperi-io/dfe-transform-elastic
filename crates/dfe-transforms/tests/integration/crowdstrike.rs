// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Floors over committed fixtures -- no panic, errors pinned, fields emitted.
//! Parity lives in `tests/compat_corpus.rs`; see `integration/remaining.rs`.

use dfe_transforms::filebeat::crowdstrike;

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../tests/fixtures/crowdstrike/falcon"
);

#[test]
fn crowdstrike_default_sample() {
    super::common::run_floor(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-sample",
        0,
    );
}

#[test]
fn crowdstrike_default_events() {
    super::common::run_floor(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-events",
        0,
    );
}

#[test]
fn crowdstrike_default_event_stream() {
    super::common::run_floor(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-event-stream",
        0,
    );
}

#[test]
fn crowdstrike_default_audit_events() {
    super::common::run_floor(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-audit-events",
        0,
    );
}

#[test]
fn crowdstrike_default_tags() {
    super::common::run_floor(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-tags",
        0,
    );
}

#[test]
fn crowdstrike_default_tags_list() {
    super::common::run_floor(
        &crowdstrike::default::Default,
        FIXTURE_DIR,
        "test-falcon-tags-list",
        0,
    );
}
