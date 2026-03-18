// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![allow(
    clippy::needless_return,
    clippy::redundant_closure_for_method_calls,
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
