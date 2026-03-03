// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Elastic ingest pipeline to Rust code generator.
//!
//! Forked from `elastic_to_vrl` — massive thanks to Dylan for the original
//! Painless parser, pipeline struct definitions, processor configs, and VRL
//! transpilation work. That codebase made this entire codegen pipeline
//! possible. The Painless ANTLR4 parser, pipeline YAML deserialisation,
//! and all 27 processor struct definitions were lifted directly from his
//! work. We've ripped out the VRL bits and are now emitting native Rust
//! transform functions instead, but the foundation is all Dylan's.
//!
//! Cheers legend.

pub mod codegen;
pub mod painless;
pub mod pipeline;

mod utils;
