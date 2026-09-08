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
//
// EVERY runner belongs here, not the ones a regeneration has already named:
// a `pub fn <name>(event: &mut Event, pattern: &<Type>)` is what a
// `direct_call` emits, and one missing from this list breaks the build the
// first time a source using it is regenerated. Held by
// `every_matcher_runner_reaches_the_prelude` in
// `crates/dfe-transforms/tests/prelude_exports.rs`.
pub use crate::painless_coercion::{LongCoercion, long_coercion};
pub use crate::painless_common::{
    AllowedValueCopy, BasenameCuts, CoerceBoolean, CombineFields, DedupeMapValues, DropPolicy,
    DurationWindow, EnsureAppend, EnsurePrefix, Factor, FirstPresentKeyName, FloatSecondsToNanos,
    GeoPointFromCoordinates, GuardedReplace, JoinPresentFields, KeyRewriteStep, LiteralValueMap,
    MailtoUriFields, MemberFromVariantKey, MoveMapEntry, OctalString, ParametersIntoMap,
    RemoveEmptyChildMaps, RenameMapKeys, RewriteKeys, ScaleField, SnakeCaseListElements,
    SplitAtDelimiter, StringOp, StringOps, SumMemberOverList, SyslogPriorityScript,
    UnwrapSuffixedKeys, allowed_value_copy, basename_cuts, coerce_boolean, combine_fields,
    dedupe_map_values, drop_empty, duration_window, ensure_append, ensure_prefix,
    first_present_key_name, float_seconds_to_nanos, geo_point_from_coordinates, guarded_replace,
    join_present_fields, kv_into_fields, literal_value_map, mailto_uri_fields,
    member_from_variant_key, move_map_entry, octal_string, parameters_into_map,
    remove_empty_child_maps, rename_map_keys, rewrite_keys, scale_field, snake_case_list_elements,
    split_at_delimiter, string_ops, sum_directions, sum_member_over_list, syslog_priority,
    unwrap_suffixed_keys,
};
pub use crate::painless_expr::{Expr, FloatLit, Op, ScalarExpression, scalar_expression};
pub use crate::painless_item_writes::{ItemWrites, item_writes};
pub use crate::painless_lists::{
    EnsureItem, ItemRename, ListItemRenames, ListWalk, list_item_renames,
};
pub use crate::painless_nth_separator::{NthSeparatorPrefix, nth_separator_prefix};
pub use crate::painless_totals::{SumTotals, Total, sum_totals};
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
