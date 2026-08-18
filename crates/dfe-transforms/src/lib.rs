// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

#![forbid(unsafe_code)]
#![warn(clippy::all)]
// These modules mirror the structure of an Elastic ingest pipeline processor
// for processor, which is deliberately not idiomatic Rust. Correctness lints
// stay on; only the style ones that fight that shape are off.
#![allow(
    clippy::needless_return,
    clippy::redundant_closure_for_method_calls,
    clippy::collapsible_if,
    clippy::redundant_closure_call,
    unused_assignments,
    unused_variables,
    unused_imports,
    dead_code
)]

//! Transform modules for Elastic data sources.
//!
//! Each submodule corresponds to a Beats source or Elastic Agent integration,
//! and each transform applies one Elastic ingest pipeline's processors to an
//! event natively -- no interpreter, no pipeline definition read at runtime.

pub mod auditbeat;
pub mod elastic_agent;
pub mod filebeat;
pub mod heartbeat;
pub mod metricbeat;
pub mod packetbeat;
pub mod winlogbeat;
