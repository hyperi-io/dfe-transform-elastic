// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Core runtime for dfe-transform-elastic.
//!
//! Provides the Event type, Transform trait, and enrichment modules
//! used by generated transform code.

pub mod enrichment;
pub mod error;
pub mod event;
pub mod prelude;
pub mod transform;

pub use error::{Result, TransformError};
pub use event::Event;
pub use transform::{Transform, TransformChain, TransformResult};
