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

//! Painless script matching, and the runners the matches dispatch to.
//!
//! A vendor pipeline carries Painless the generator cannot transpile, so each
//! recognised pattern gets a reader that parses the script ONCE per call site
//! and a runner that applies it per event. [`plan::PainlessPlan`] holds that
//! split: `cached_painless!` resolves the dispatch behind a site-local
//! `OnceLock`, so the ladder in [`common::known_patterns`] is cold path and
//! only the matched runner costs anything per event.
//!
//! The modules drop the `painless_` prefix the crate name already carries.

pub mod action_mapping;
pub mod actor_kind;
pub mod append_records;
pub mod byte_size;
pub mod coercion;
pub mod collect_present;
pub mod collect_rows;
pub mod common;
pub mod deep_merge;
pub mod delimited_table;
pub mod element_mapping;
pub mod entity;
pub mod expr;
pub mod field_tables;
pub mod gather_members;
pub mod group_records;
pub mod guarded_records;
pub mod helpers;
pub mod hex;
pub mod hoist;
pub mod issue_lifecycle;
pub mod item_writes;
pub mod last_element;
pub mod level_labels;
pub mod list_records;
pub mod lists;
pub mod map_entries;
pub mod member_tags;
pub mod named_arms;
pub mod nth_separator;
pub mod pair_table;
pub mod params;
pub mod plan;
pub mod records;
pub mod scheduled_task;
pub mod sddl;
pub mod seconds_between;
pub mod split_fanout;
pub mod stats;
pub mod stringify_member;
pub mod threat_artifacts;
pub mod time_order_flag;
pub mod totals;
pub mod trim_delimited;
pub mod url_action;
pub mod windows;
