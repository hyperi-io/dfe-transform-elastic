// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Floors over the synthetic samples: no panic, errors pinned, fields emitted.

use dfe_transforms::filebeat::{azure_activitylogs, azure_auditlogs, azure_signinlogs};

floor!(
    azure_activitylogs_raw,
    azure_activitylogs::default::Default,
    "azure",
    "activitylogs-administrative",
    0
);

floor!(
    azure_activitylogs_identity,
    azure_activitylogs::default::Default,
    "azure",
    "activitylogs-identity",
    0
);

floor!(
    azure_auditlogs_raw,
    azure_auditlogs::default::Default,
    "azure",
    "auditlogs",
    0
);

floor!(
    azure_signinlogs_raw,
    azure_signinlogs::default::Default,
    "azure",
    "signinlogs-interactive",
    0
);

floor!(
    azure_signinlogs_non_interactive,
    azure_signinlogs::default::Default,
    "azure",
    "signinlogs-non-interactive",
    0
);
