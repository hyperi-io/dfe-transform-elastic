// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

#![forbid(unsafe_code)]
// Levels live in [workspace.lints] -- only what THIS crate relaxes is here.
#![allow(
    clippy::redundant_closure_for_method_calls,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

//! The primitives every other crate here is written against.
//!
//! Held apart from `dfe-runtime` so the Painless matchers can reach them
//! without depending on the grok and processor layers that sit above. Nothing
//! in here knows about a vendor, a pipeline or a transform: it is the event
//! document, its error type, and the two parsers that a matcher and a
//! processor both need.

pub mod date_formats;
pub mod error;
pub mod event;
pub mod regex_cache;
pub mod syslog_pri;

pub use error::{Result, TransformError};
pub use event::Event;
