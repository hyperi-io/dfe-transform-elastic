// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Painless script parser and (future) Rust codegen.
//!
//! The ANTLR4-generated parser is fully functional — all credit to Dylan
//! for building the original Painless→VRL transpiler and getting the ANTLR4
//! grammar wired up in Rust. The VRL transpiler has been removed and will
//! be replaced with native Rust codegen.

#[allow(dead_code)]
pub mod parser;

#[allow(dead_code)]
pub mod script;

// TODO: transpiler.rs depends on VRL AST types. It will be rewritten for
// native Rust codegen. Disabled for now to allow cargo check to pass.
// pub mod transpiler;

pub use script::*;
