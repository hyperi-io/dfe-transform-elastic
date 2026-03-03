// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Rust code generation from parsed Elastic ingest pipelines.
//!
//! Takes a validated `Pipeline` struct and emits a complete `.rs` file
//! containing a struct that implements `dfe_runtime::Transform`.

mod emit;
mod processor;

pub use emit::{codegen_pipeline_body, PipelineCodegen};
