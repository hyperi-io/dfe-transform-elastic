// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A flag written from whether one recorded instant precedes another.
//!
//! `darktrace` decides whether a model breach was acknowledged BEFORE it fired,
//! and spells out every way the answer can fail to be a comparison:
//!
//! ```painless
//! ctx.darktrace = ctx.darktrace ?: [:];
//! ctx.darktrace.model_breach_alert = ctx.darktrace?.model_breach_alert ?: [:];
//! if (ctx.darktrace?.model_breach_alert?.acknowledged == null) {
//!   ctx.darktrace.model_breach_alert.is_acknowledged = false; return;
//! }
//! if (!(ctx.darktrace.model_breach_alert.acknowledged instanceof Map)) { return; }
//! if (ctx.darktrace.model_breach_alert.acknowledged.time == null) { return; }
//! if (ctx.darktrace.model_breach_alert.time == null) {
//!   ctx.darktrace.model_breach_alert.is_acknowledged = true; return;
//! }
//! def time = ctx.darktrace.model_breach_alert.time;
//! def acknowledged = ctx.darktrace.model_breach_alert.acknowledged.time;
//! ctx.darktrace.model_breach_alert.is_acknowledged =
//!   ZonedDateTime.parse(acknowledged).isBefore(ZonedDateTime.parse(time));
//! ```
//!
//! Unclaimed it costs `is_acknowledged` on 9 of the stream's 11 events.
//!
//! **FIVE outcomes, and three of them write nothing.** An absent record answers
//! false; a record that is a bare boolean answers nothing, because the vendor
//! renames that one elsewhere; a record with no instant answers nothing, which
//! is the pipeline being deliberately non-committal; a breach with no instant of
//! its own answers true; and only the last arm compares. A matcher that wrote on
//! every path would be wrong on three of the five and the corpus would show it
//! on one event -- the acknowledged-with-no-time case, where Elasticsearch
//! leaves the field absent.

use chrono::{DateTime, FixedOffset};
use serde_json::{Map, Value};

use dfe_core::Event;

use crate::common::{block_statements, ctx_path_bound_to, if_block, painless_path};

/// The paths a time-ordered flag is decided from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeOrderFlag {
    /// Maps the script builds before it reads anything, in the order it builds
    /// them.
    ensure: Vec<String>,
    /// The record whose presence and type decide the first two arms.
    container: String,
    /// The instant that has to come first for the flag to be true.
    earlier: String,
    /// The instant it is compared against.
    later: String,
    /// The flag written.
    flag: String,
}

/// Read the ladder, or decline.
///
/// Structural throughout: the four guards are matched by what their bodies
/// WRITE rather than by their order, the `instanceof` guard has to name the same
/// record the null guard did, the compared instant has to be a member of that
/// record, and the final comparison has to be over the two locals those guards
/// named. A script that looks similar and means something else fails one of
/// them and falls through.
pub fn parse_time_order_flag(script: &str) -> Option<TimeOrderFlag> {
    let text = without_comments(script);
    if !text.contains("ZonedDateTime.parse(") || !text.contains("instanceof Map") {
        return None;
    }

    let mut ensure = Vec::new();
    let mut guards: Vec<(&str, &str)> = Vec::new();
    let mut tail: Vec<&str> = Vec::new();
    for statement in block_statements(&text)? {
        if let Some((condition, body, rest)) = if_block(statement) {
            if !rest.trim().is_empty() {
                return None;
            }
            guards.push((condition, body));
        } else if guards.is_empty() {
            // `ctx.<path> = <...> ?: [:];` -- the parent Painless will not make
            // for itself. Anything else before the guards is a statement this
            // does not reproduce.
            let (target, made) = statement.split_once('=')?;
            if !made.contains("?: [:]") {
                return None;
            }
            ensure.push(painless_path(target)?);
        } else {
            tail.push(statement);
        }
    }
    if guards.len() != 4 {
        return None;
    }

    let (container, flag) = guards
        .iter()
        .find_map(|(condition, body)| literal_arm(condition, body, "false"))?;
    let typed = guards
        .iter()
        .find(|(condition, _)| condition.contains("instanceof Map"))
        .and_then(|(condition, _)| painless_path(condition.split_once("instanceof")?.0))?;
    if typed != container {
        return None;
    }
    let (later, true_flag) = guards
        .iter()
        .find_map(|(condition, body)| literal_arm(condition, body, "true"))?;
    if true_flag != flag {
        return None;
    }
    // Whatever is left: the guard that writes nothing and names the instant
    // inside the record.
    let mut earlier = None;
    for (condition, body) in &guards {
        if condition.contains("instanceof Map") || !returns_only(body) {
            continue;
        }
        let Some(tested) = condition
            .split_once("== null")
            .and_then(|(path, _)| painless_path(path))
        else {
            continue;
        };
        earlier = Some(tested);
    }
    let earlier = earlier?;
    if !earlier.starts_with(&format!("{container}.")) {
        return None;
    }

    let comparison = tail.last()?;
    let (written, against) = comparison.split_once(".isBefore(")?;
    if painless_path(written)? != flag {
        return None;
    }
    let receiver = parsed_local(written)?;
    let argument = parsed_local(against)?;
    (ctx_path_bound_to(&text, &receiver)? == earlier
        && ctx_path_bound_to(&text, &argument)? == later)
        .then_some(TimeOrderFlag {
            ensure,
            container,
            earlier,
            later,
            flag,
        })
}

/// `if (<path> == null) { <flag> = <literal>; return; }`, as the pair it names.
fn literal_arm(condition: &str, body: &str, literal: &str) -> Option<(String, String)> {
    let written = body.split_once(&format!("= {literal}"))?.0;
    let tested = painless_path(condition.split_once("== null")?.0)?;
    Some((tested, painless_path(written)?))
}

/// Whether a guard body does nothing but leave the script.
fn returns_only(body: &str) -> bool {
    body.split(';')
        .map(str::trim)
        .filter(|statement| !statement.is_empty())
        .all(|statement| statement == "return")
}

/// The local handed to the last `ZonedDateTime.parse(` in a fragment.
fn parsed_local(fragment: &str) -> Option<String> {
    let (_, rest) = fragment.rsplit_once("ZonedDateTime.parse(")?;
    let name = rest.split(')').next()?.trim();
    (!name.is_empty()
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
        && !name.starts_with(|c: char| c.is_ascii_digit()))
    .then(|| name.to_owned())
}

/// The script with its `//` comments gone, quoted text untouched.
fn without_comments(script: &str) -> String {
    let mut out = String::with_capacity(script.len());
    let mut quote: Option<char> = None;
    let mut chars = script.chars().peekable();
    while let Some(c) = chars.next() {
        match (quote, c) {
            (Some(open), c) if c == open => {
                quote = None;
                out.push(c);
            }
            (None, '\'' | '"') => {
                quote = Some(c);
                out.push(c);
            }
            (None, '/') if chars.peek() == Some(&'/') => {
                for skipped in chars.by_ref() {
                    if skipped == '\n' {
                        out.push('\n');
                        break;
                    }
                }
            }
            _ => out.push(c),
        }
    }
    out
}

/// Decide the flag, walking the same five arms in the same order.
///
/// An instant neither side can parse writes NOTHING: `ZonedDateTime.parse`
/// throws on text it cannot read, and the processor is left having written
/// nothing rather than having guessed.
pub fn time_order_flag(event: &mut Event, pattern: &TimeOrderFlag) -> bool {
    for path in &pattern.ensure {
        if !event.has_value(path) {
            let _ = event.set(path, Value::Object(Map::new()));
        }
    }

    match event.get(&pattern.container) {
        None | Some(Value::Null) => {
            let _ = event.set(&pattern.flag, false);
            return true;
        }
        Some(Value::Object(_)) => {}
        // A bare boolean, which some versions of the vendor send. The pipeline
        // renames that one instead, so this writes nothing.
        Some(_) => return true,
    }

    if !event.has_value(&pattern.earlier) {
        return true;
    }
    if !event.has_value(&pattern.later) {
        let _ = event.set(&pattern.flag, true);
        return true;
    }

    let (Some(earlier), Some(later)) = (
        instant(event, &pattern.earlier),
        instant(event, &pattern.later),
    ) else {
        return true;
    };
    let _ = event.set(&pattern.flag, earlier < later);
    true
}

/// One field read as the offset instant `ZonedDateTime.parse` would accept.
fn instant(event: &Event, path: &str) -> Option<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc3339(event.get_str(path)?).ok()
}

#[cfg(test)]
#[path = "time_order_flag_tests.rs"]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
mod tests;
