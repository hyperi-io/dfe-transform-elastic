// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Floors over committed fixtures -- no panic, errors pinned, fields emitted.
//! Parity lives in `tests/compat_corpus.rs`; see `integration/remaining.rs`.

use dfe_transforms::filebeat::okta;

// The two unmatched events are a device_integrator JSON parse and a
// target detailEntry edge case.
floor!(
    okta_default_system_events,
    okta::default::Default,
    "okta/system",
    "test-okta-system-events",
    0
);
