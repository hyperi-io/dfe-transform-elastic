// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Floors over committed fixtures -- no panic, errors pinned, fields emitted.
//! Parity lives in `tests/compat_corpus.rs`; see `integration/remaining.rs`.

use dfe_transforms::filebeat::crowdstrike;

floor!(
    crowdstrike_default_sample,
    crowdstrike::default::Default,
    "crowdstrike/falcon",
    "test-falcon-sample",
    0
);

floor!(
    crowdstrike_default_events,
    crowdstrike::default::Default,
    "crowdstrike/falcon",
    "test-falcon-events",
    0
);

floor!(
    crowdstrike_default_event_stream,
    crowdstrike::default::Default,
    "crowdstrike/falcon",
    "test-event-stream",
    0
);

floor!(
    crowdstrike_default_audit_events,
    crowdstrike::default::Default,
    "crowdstrike/falcon",
    "test-falcon-audit-events",
    0
);

floor!(
    crowdstrike_default_tags,
    crowdstrike::default::Default,
    "crowdstrike/falcon",
    "test-falcon-tags",
    0
);

floor!(
    crowdstrike_default_tags_list,
    crowdstrike::default::Default,
    "crowdstrike/falcon",
    "test-falcon-tags-list",
    0
);
