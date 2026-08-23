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
pub use crate::{
    cached_grok, cached_grok_mapped, cached_painless, cached_params, cached_regex, cached_script,
};

pub use crate::grok_cache::extract_first_match;

pub use crate::painless_plan::{PainlessPlan, painless_exec_plan, painless_exec_plan_params};

pub use crate::date_formats::{parse_date, parse_date_out};

pub use crate::codegen_api::{
    RegisteredDomainResult, community_id_v1, convert_value, csv_close_quote_gap, dot_expand,
    fingerprint_default, foreach_array, geoip_lookup, grok_to_regex, grok_to_regex_with_map,
    gsub_field, is_internal_ip, join_values, kv_put, map_strings, painless_exec,
    painless_exec_params, parse_json_field, parse_json_str, parse_user_agent,
    registered_domain_lookup, resolve_path, sort_values, uri_parts, url_decode,
};

pub use crate::painless_helpers::{
    dedup_array, filetime_to_unix_ms, painless_add, painless_cmp, painless_div,
    painless_drop_empty, painless_eq, painless_is_empty_value, painless_keys_to_snake_case,
    painless_mod, painless_mul, painless_sub, painless_to_f64, painless_to_i64, painless_to_string,
    painless_truthy, remove_sentinel_values, template_to_string,
};
