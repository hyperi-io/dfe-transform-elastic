// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Re-exports for the transform modules.
//!
//! Each module opens with `use dfe_runtime::prelude::*;` to get
//! everything needed to write a transform function.

pub use crate::error::{Result, TransformError};
pub use crate::event::Event;
pub use crate::transform::{Transform, TransformChain, TransformResult};

pub use serde_json::{Map, Value, json};

pub use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};

// Compiled once per process, then looked up once per call site.
pub use crate::{
    cached_grok, cached_grok_mapped, cached_painless, cached_params, cached_regex, cached_script,
};

pub use crate::grok_cache::{extract_first_match, extract_first_match_traced};

pub use crate::painless_plan::{PainlessPlan, painless_exec_plan, painless_exec_plan_params};

// Matchers a caller can drive directly, having already resolved the script.
// Same functions the ladder dispatches to, so the two paths cannot diverge.
pub use crate::painless_common::{
    AllowedValueCopy, DropPolicy, EnsureAppend, EnsurePrefix, Factor, GuardedReplace, OctalString,
    MailtoUriFields, MoveMapEntry, ParametersIntoMap, RemoveEmptyChildMaps, RenameMapKeys,
    ScaleField, SplitAtDelimiter, StringOp, StringOps, SyslogPriorityScript, allowed_value_copy,
    drop_empty, ensure_append, ensure_prefix, guarded_replace, kv_into_fields, mailto_uri_fields,
    move_map_entry, octal_string, parameters_into_map, remove_empty_child_maps, rename_map_keys,
    scale_field, split_at_delimiter, string_ops, sum_directions, syslog_priority,
};
// `lookup_normalise` is NOT here: it now takes the call site's parsed literal
// tail, which is a runtime detail rather than something a generator resolves.
pub use crate::painless_params::{Fold, LookupNormaliseScript};

pub use crate::date_formats::{parse_date, parse_date_out};

pub use crate::codegen_api::{
    RegisteredDomainResult, community_id_v1, condition_eq, convert_value, csv_close_quote_gap,
    dot_expand, fingerprint_default, fingerprint_with, foreach_array, geoip_lookup, grok_to_regex,
    grok_to_regex_with_map, gsub_field, html_strip, ip_in_networks, is_internal_ip, join_values,
    kv_put, map_strings, painless_exec, painless_exec_params, parse_json_field,
    parse_json_field_to_root, parse_json_str, parse_user_agent, registered_domain_lookup,
    remove_templated, resolve_path, set_templated, sort_values, uri_parts, url_decode,
};

pub use crate::painless_helpers::{
    dedup_array, filetime_to_unix_ms, painless_add, painless_cmp, painless_div,
    painless_drop_empty, painless_eq, painless_is_empty_value, painless_keys_to_snake_case,
    painless_mod, painless_mul, painless_sub, painless_to_f64, painless_to_i64, painless_to_string,
    painless_truthy, remove_sentinel_values, template_to_string,
};
