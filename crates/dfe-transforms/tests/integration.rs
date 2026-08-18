// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Integration tests — single binary, submodule per source.
//!
//! All integration tests compile as ONE binary (one link cycle).
//! This gives ~3x compile-time reduction vs separate test files.
//!
//! Adding a new source? Add `mod source_name;` in the integration/ directory
//! and declare it here.

mod common;

#[path = "integration/azure.rs"]
mod azure;

#[path = "integration/crowdstrike.rs"]
mod crowdstrike;

#[path = "integration/okta.rs"]
mod okta;
