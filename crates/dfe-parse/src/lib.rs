// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! High-performance parser library replacing grok/regex patterns with native Rust.
//!
//! Provides zero-copy, SIMD-accelerated parsers for common log field types:
//! IP addresses, timestamps, integers, hostnames, URIs, and more.

pub mod composite;
pub mod dfa;
pub mod ip;
pub mod network;
pub mod numeric;
pub mod string;
pub mod timestamp;
