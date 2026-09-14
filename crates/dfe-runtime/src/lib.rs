// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

#![forbid(unsafe_code)]
// Levels live in [workspace.lints] -- only what THIS crate relaxes is here.
// Painless semantics are f64-based, so the numeric helpers truncate on purpose.
#![allow(
    clippy::redundant_closure_for_method_calls,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

//! Core runtime for dfe-transform-elastic.
//!
//! The Transform trait, grok compilation, the enrichment modules and the
//! processor-shaped API every transform is written against. The event document
//! itself lives in `dfe-core` and the Painless matchers in `dfe-painless`;
//! both are re-exported here so a generated module still needs only
//! [`prelude`].

pub mod codegen_api;
pub mod enrichment;
pub mod grok_cache;
pub mod prelude;
pub mod transform;

// The layers below, re-exported at their original paths. Over 2,800 generated
// modules and the -dev generator name them through this crate, so the split
// stays invisible to both.
pub use dfe_core::{date_formats, error, event, syslog_pri};
pub use dfe_painless::{
    action_mapping as painless_action_mapping, actor_kind as painless_actor_kind,
    append_records as painless_append_records, byte_size as painless_byte_size,
    coercion as painless_coercion, collect_by_literal as painless_collect_by_literal,
    collect_present as painless_collect_present, common as painless_common,
    deep_merge as painless_deep_merge, delimited_table as painless_delimited_table,
    dotted_keys as painless_dotted_keys, dump_into_map as painless_dump_into_map,
    element_mapping as painless_element_mapping, entity as painless_entity, expr as painless_expr,
    field_tables as painless_field_tables, gather_members as painless_gather_members,
    group_records as painless_group_records, guarded_records as painless_guarded_records,
    helpers as painless_helpers, hex as painless_hex, hoist as painless_hoist,
    indicator_expiry as painless_indicator_expiry, issue_lifecycle as painless_issue_lifecycle,
    item_writes as painless_item_writes, labelled_keys as painless_labelled_keys,
    last_element as painless_last_element, level_labels as painless_level_labels,
    list_records as painless_list_records, lists as painless_lists,
    map_entries as painless_map_entries, member_fan_out as painless_member_fan_out,
    member_kv_fold as painless_member_kv_fold, member_ladder as painless_member_ladder,
    member_tags as painless_member_tags, member_value_wrap as painless_member_value_wrap,
    named_arms as painless_named_arms, needle_table as painless_needle_table,
    nth_separator as painless_nth_separator, pair_table as painless_pair_table,
    params as painless_params, plan as painless_plan, record_fold as painless_record_fold,
    records as painless_records, rescale as painless_rescale,
    scheduled_task as painless_scheduled_task, sddl as painless_sddl,
    seconds_between as painless_seconds_between, securityhub_ocsf as painless_securityhub_ocsf,
    set_ladder as painless_set_ladder, split_fanout as painless_split_fanout,
    stats as painless_stats, stringify_member as painless_stringify_member,
    threat_artifacts as painless_threat_artifacts, time_order_flag as painless_time_order_flag,
    totals as painless_totals, typed_member_rename as painless_typed_member_rename,
    windows as painless_windows,
};

pub use dfe_core::{Event, Result, TransformError};
pub use transform::{Transform, TransformChain, TransformResult};

// Re-export scalo types used by the runtime and downstream consumers
pub use scalo::kafka_config::{KafkaSource, ServiceRole};
pub use scalo::memory::{MemoryGuard, MemoryGuardConfig, MemoryPressure};

#[cfg(feature = "testutil")]
pub mod testutil;
