// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Floors over committed fixtures -- no panic, errors pinned, fields emitted.
//! Parity lives in `tests/compat_corpus.rs`; see `integration/remaining.rs`.
//!
//! Azure is 10/10.

use dfe_transforms::filebeat::{azure_activitylogs, azure_auditlogs, azure_signinlogs};

floor!(
    azure_activitylogs_raw,
    azure_activitylogs::default::Default,
    "azure/activitylogs",
    "test-activitylogs-raw",
    0
);

floor!(
    azure_activitylogs_identity,
    azure_activitylogs::default::Default,
    "azure/activitylogs",
    "test-activitylogs-identity",
    0
);

floor!(
    azure_activitylogs_edgecases,
    azure_activitylogs::default::Default,
    "azure/activitylogs",
    "test-activitylogs-edgecases",
    0
);

floor!(
    azure_auditlogs_raw,
    azure_auditlogs::default::Default,
    "azure/auditlogs",
    "test-auditlogs-raw",
    0
);

floor!(
    azure_signinlogs_raw,
    azure_signinlogs::default::Default,
    "azure/signinlogs",
    "test-signinlogs-raw",
    0
);

floor!(
    azure_signinlogs_sample,
    azure_signinlogs::default::Default,
    "azure/signinlogs",
    "test-signinlogs-sample",
    0
);
