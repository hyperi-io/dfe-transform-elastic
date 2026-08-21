// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Re-exports for the transform modules.
//!
//! Each module opens with `use dfe_runtime::prelude::*;` to get
//! everything needed to write a transform function.

pub use crate::error::{Result, TransformError};
pub use crate::event::Event;
pub use crate::transform::{Transform, TransformChain, TransformResult};

pub use serde_json::{Value, json};

pub use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};

// Compiled once per process, then looked up once per call site.
pub use crate::{cached_grok, cached_grok_mapped, cached_params, cached_regex, cached_script};

pub use crate::date_formats::{parse_date, parse_date_out};

pub use crate::codegen_api::{
    RegisteredDomainResult, community_id_v1, csv_close_quote_gap, dot_expand, geoip_lookup,
    grok_to_regex, grok_to_regex_with_map, is_internal_ip, painless_exec, painless_exec_params,
    parse_user_agent, registered_domain_lookup, resolve_path, uri_parts,
};

pub use crate::painless_helpers::{
    dedup_array, filetime_to_unix_ms, painless_add, painless_cmp, painless_div,
    painless_drop_empty, painless_eq, painless_is_empty_value, painless_keys_to_snake_case,
    painless_mod, painless_mul, painless_sub, painless_to_f64, painless_to_i64, painless_to_string,
    painless_truthy, remove_sentinel_values,
};
