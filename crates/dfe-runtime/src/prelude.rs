// SPDX-License-Identifier: FSL-1.1-ALv2
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Re-exports for generated transform code.
//!
//! Generated modules use `use dfe_runtime::prelude::*;` to get
//! everything needed to write a transform function.

pub use crate::error::{Result, TransformError};
pub use crate::event::Event;
pub use crate::transform::{Transform, TransformChain, TransformResult};

pub use serde_json::{json, Value};

pub use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};

pub use crate::codegen_api::{
    community_id_v1, geoip_lookup, grok_to_regex, is_internal_ip, painless_exec, parse_user_agent,
    registered_domain_lookup, RegisteredDomainResult,
};

pub use crate::painless_helpers::{
    painless_add, painless_cmp, painless_div, painless_drop_empty, painless_eq,
    painless_keys_to_snake_case, painless_mod, painless_mul, painless_sub, painless_to_f64,
    painless_to_i64, painless_to_string, painless_truthy,
};
