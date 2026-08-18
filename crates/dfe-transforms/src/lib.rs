// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

#![forbid(unsafe_code)]
#![warn(clippy::all)]
// Shapes the generator emits, not defects to fix in place. Correctness lints stay on.
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

//! Generated and hand-tuned transform modules for Elastic data sources.
//!
//! Each submodule corresponds to a Beats source or Elastic Agent integration.

pub mod auditbeat;
pub mod elastic_agent;
pub mod filebeat;
pub mod heartbeat;
pub mod metricbeat;
pub mod packetbeat;
pub mod winlogbeat;
