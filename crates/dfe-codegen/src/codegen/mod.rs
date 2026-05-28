// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Rust code generation from parsed Elastic ingest pipelines.
//!
//! Takes a validated `Pipeline` struct and emits a complete `.rs` file
//! containing a struct that implements `dfe_runtime::Transform`.

pub mod condition;
mod emit;
mod processor;

pub use emit::{PipelineCodegen, codegen_pipeline_body};
