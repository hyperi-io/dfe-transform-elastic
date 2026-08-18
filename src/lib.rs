// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Kafka-to-Kafka transform service for Beats and Elastic Agent data.
//!
//! Consumes JSON event batches, applies the transform registered for the
//! configured source, and produces the normalised events downstream.
//!
//! Lints are configured in `Cargo.toml` `[lints]` so lib and bin share one
//! source of truth; do not restate them here as crate attributes.

pub mod cli;
pub mod config;
pub mod deployment;
pub mod envelope;
pub mod error;
pub mod metrics;
pub mod pipeline;
pub mod registry;

#[cfg(feature = "kafka")]
pub mod service;

pub use error::{Error, Result};
