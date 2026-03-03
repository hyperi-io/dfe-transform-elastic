// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! High-performance parser library replacing grok/regex patterns with native Rust.
//!
//! Provides zero-copy, SIMD-accelerated parsers for common log field types:
//! IP addresses, timestamps, integers, hostnames, URIs, and more.
//!
//! # Parser convention
//!
//! All parsers follow the winnow/nom convention: take `&str` input, return
//! `ParseResult<'_, T>` which is `Result<(&str, T), ParseError>`. The first
//! element of the success tuple is the remaining unconsumed input. The second
//! element is the parsed value (often `&str` for zero-copy).
//!
//! ```ignore
//! let (remaining, ip) = dfe_parse::ip::parse_ipv4("192.168.1.1:8080")?;
//! assert_eq!(ip, "192.168.1.1");
//! assert_eq!(remaining, ":8080");
//! ```

pub mod error;

pub mod composite;
pub mod dfa;
pub mod ip;
pub mod network;
pub mod numeric;
pub mod string;
pub mod timestamp;

pub use error::{ParseError, ParseResult};
