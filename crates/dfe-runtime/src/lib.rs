// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic)]
// Painless semantics are f64-based, so the numeric helpers truncate on purpose.
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
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
    coercion as painless_coercion, collect_present as painless_collect_present,
    common as painless_common, element_mapping as painless_element_mapping,
    entity as painless_entity, expr as painless_expr, field_tables as painless_field_tables,
    gather_members as painless_gather_members, helpers as painless_helpers, hex as painless_hex,
    hoist as painless_hoist, issue_lifecycle as painless_issue_lifecycle,
    item_writes as painless_item_writes, last_element as painless_last_element,
    list_records as painless_list_records, lists as painless_lists,
    map_entries as painless_map_entries, named_arms as painless_named_arms,
    nth_separator as painless_nth_separator, pair_table as painless_pair_table,
    params as painless_params, plan as painless_plan, records as painless_records,
    scheduled_task as painless_scheduled_task, sddl as painless_sddl,
    seconds_between as painless_seconds_between, split_fanout as painless_split_fanout,
    stats as painless_stats, totals as painless_totals, windows as painless_windows,
};

pub use dfe_core::{Event, Result, TransformError};
pub use transform::{Transform, TransformChain, TransformResult};

// Re-export scalo types used by the runtime and downstream consumers
pub use scalo::kafka_config::{KafkaSource, ServiceRole};
pub use scalo::memory::{MemoryGuard, MemoryGuardConfig, MemoryPressure};

#[cfg(feature = "testutil")]
pub mod testutil;
