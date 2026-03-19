// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::redundant_closure_for_method_calls
)]

//! Core runtime for dfe-transform-elastic.
//!
//! Provides the Event type, Transform trait, and enrichment modules
//! used by generated transform code.

pub mod codegen_api;
pub mod enrichment;
pub mod error;
pub mod event;
pub mod painless_common;
pub mod painless_helpers;
pub mod prelude;
pub mod transform;

pub use error::{Result, TransformError};
pub use event::Event;
pub use transform::{Transform, TransformChain, TransformResult};

#[cfg(feature = "testutil")]
pub mod testutil;
