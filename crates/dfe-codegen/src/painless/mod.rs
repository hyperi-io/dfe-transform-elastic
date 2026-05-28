// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Painless script parser and Rust code generation.
//!
//! The ANTLR4-generated parser is fully functional — all credit to Dylan
//! for building the original Painless→VRL transpiler and getting the ANTLR4
//! grammar wired up in Rust. The VRL transpiler has been replaced with
//! native Rust codegen via the IR → emitter pipeline.

#[allow(dead_code)]
pub mod parser;

#[allow(dead_code)]
pub mod script;

pub mod emitter;
pub mod ir;
pub mod params;
pub mod visitor;

// Legacy VRL transpiler — kept as reference during Rust codegen development.
// pub mod transpiler;

pub use script::*;
