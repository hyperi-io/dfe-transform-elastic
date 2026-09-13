// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Transforms for the okta integration.
//!
//! The three beyond `default` are mid-pipeline stages it also inlines, not
//! intake entry points: they read the post-unwrap shape `default` builds.

pub mod default;
pub mod ecs_category_type;
pub mod no_use_flattened_debug;
pub mod use_flattened_debug;
