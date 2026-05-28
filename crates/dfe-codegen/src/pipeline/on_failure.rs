// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! On-failure handler for Elastic ingest processors.

use serde::Deserialize;

use super::Processor;

/// A list of processors to run when the parent processor fails.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct OnFailure(pub Vec<Processor>);

/// Metadata passed to on-failure handlers (processor type and tag).
pub struct Metadata {
    pub processor_type: String,
    pub processor_tag: Option<String>,
}
