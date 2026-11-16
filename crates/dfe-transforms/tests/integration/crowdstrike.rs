// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Floors over the synthetic samples: no panic, errors pinned, fields emitted.

use dfe_transforms::filebeat::crowdstrike;

floor!(
    crowdstrike_default_sample,
    crowdstrike::default::Default,
    "crowdstrike",
    "falcon-detection-summary",
    0
);

floor!(
    crowdstrike_default_events,
    crowdstrike::default::Default,
    "crowdstrike",
    "falcon-incident-and-response",
    0
);

floor!(
    crowdstrike_default_audit_events,
    crowdstrike::default::Default,
    "crowdstrike",
    "falcon-audit",
    0
);
