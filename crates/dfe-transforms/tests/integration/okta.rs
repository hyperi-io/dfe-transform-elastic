// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Floors over the synthetic samples: no panic, errors pinned, fields emitted.

use dfe_transforms::filebeat::okta;

floor!(
    okta_default_system_events,
    okta::default::Default,
    "okta",
    "system-log",
    0
);
