// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
// Painless semantics are f64-based, so the numeric helpers truncate on purpose.
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::redundant_closure_for_method_calls,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
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

// Re-export scalo types used by the runtime and downstream consumers
pub use scalo::kafka_config::{KafkaSource, ServiceRole};
pub use scalo::memory::{MemoryGuard, MemoryGuardConfig, MemoryPressure};

#[cfg(feature = "testutil")]
pub mod testutil;
