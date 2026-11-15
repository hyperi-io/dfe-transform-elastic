// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A run of NAMED fields divided by one literal, rounded where the vendor
//! rounds.
//!
//! Elastic's metrics packages declare percentage fields on a 0-1 scale and the
//! devices report 0-100, so a rescaling script is the first processor in the
//! pipeline. `panw_metrics` loops a literal name list over `ctx.panw.system.cpu`
//! and then divides one dotted key beside it; `cisco_meraki_metrics` binds two
//! map subscripts to locals and divides their members through
//! `Math.round((x / 100) * 10000) / 10000.0`.
//!
//! Both are ONE intent -- rescale these fields, in place -- with the divide
//! written four ways, so this is one runner behind four readings of the target
//! rather than four patterns. Neither reaches the `GuardedDivide` catch-all:
//! that reader wants a plain `ctx.` path on both sides, and a `[fieldName]`
//! subscript and a `Math.round(` wrapper each fail it.

use std::collections::HashMap;

use serde_json::{Value, json};

use dfe_core::event::Event;

use crate::common::{last_assignment, literal_divisor, painless_path, strip_line_comments};

/// The fields one script rescales, and how.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RescaleFields {
    /// Every field the script divides, in the order it names them.
    targets: Vec<String>,
    divisor: i64,
    /// `Math.round(x * unit) / unit`, where the vendor rounds the quotient.
    ///
    /// Held as the UNIT rather than a decimal count, because the script writes
    /// the unit twice and the two have to agree for the reading to be right.
    round_unit: Option<i64>,
}

impl RescaleFields {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(targets: Vec<String>, divisor: i64, round_unit: Option<i64>) -> Self {
        Self {
            targets,
            divisor,
            round_unit,
        }
    }
}

/// Divide each named field in place, leaving an absent one alone.
///
/// Writes through [`Event::update`] rather than `set`: panw holds
/// `filesystem.use_percent` as ONE key under `panw.system`, and a dotted `set`
/// would build a nested twin beside the vendor's own value.
pub fn rescale_fields(event: &mut Event, pattern: &RescaleFields) -> bool {
    #[allow(clippy::cast_precision_loss)]
    let divisor = pattern.divisor as f64;
    for target in &pattern.targets {
        let Some(value) = event.get_f64(target) else {
            continue;
        };
        let mut scaled = value / divisor;
        if let Some(unit) = pattern.round_unit {
            // Java's `Math.round` returns a long, so the quotient is rounded
            // once and divided back -- not rounded twice.
            #[allow(clippy::cast_precision_loss)]
            let unit = unit as f64;
            scaled = (scaled * unit).round() / unit;
        }
        event.update(target, json!(scaled));
    }
    true
}

/// Read every rescale the script performs, or `None` where it does anything
/// else as well.
///
/// The whole script has to be rescales: a statement that assigns and is not one
/// means this reader has not understood it, and claiming it here would shadow
/// the matcher that had.
pub fn parse_rescale_fields(script: &str) -> Option<RescaleFields> {
    let script = strip_line_comments(script);
    let locals = locals_bound_to_paths(&script);
    let loops = loop_variables(&script);

    let mut targets: Vec<String> = Vec::new();
    let mut divisor: Option<i64> = None;
    let mut round_unit: Option<Option<i64>> = None;

    for statement in script.split([';', '\n']) {
        let statement = statement.trim();
        if statement.is_empty() || statement.starts_with("def ") {
            continue;
        }
        let Some(at) = last_assignment(statement) else {
            continue;
        };
        let target = statement[..at].trim_start_matches('{').trim();
        let divide = read_divide(statement[at + 1..].trim())?;
        // Rescaling is IN PLACE, so the two sides name the same field. Reading
        // them apart would let `ctx.a = ctx.b / 100` in, which is the
        // `GuardedDivide` pattern and already has a reader.
        if divide.read != target {
            return None;
        }
        if *divisor.get_or_insert(divide.divisor) != divide.divisor
            || *round_unit.get_or_insert(divide.round_unit) != divide.round_unit
        {
            return None;
        }
        targets.extend(resolve_targets(target, &locals, &loops)?);
    }

    let divisor = divisor?;
    if targets.is_empty() || reads_outside(&script, &targets) {
        return None;
    }
    Some(RescaleFields {
        targets,
        divisor,
        round_unit: round_unit.flatten(),
    })
}

/// Whether the script reads any field outside the subtrees it rescales.
///
/// A guard on another field is a condition this reader does not carry, and
/// running the divide regardless writes a wrong number: aws rescales
/// `CPUUtilization.avg` only where the agent has not already written
/// `host.cpu.usage` itself, and dividing an agent-written fraction a second time
/// took `42` to `0.42`. The guarded divide below reads that script and keeps the
/// guard, so declining here hands it back rather than losing it.
fn reads_outside(script: &str, targets: &[String]) -> bool {
    let roots: Vec<&str> = targets
        .iter()
        .map(|target| target.split('.').next().unwrap_or(target))
        .collect();
    script
        .match_indices("ctx.")
        .filter_map(|(at, marker)| root_segment(&script[at + marker.len()..]))
        .any(|root| !roots.contains(&root))
}

/// The first path segment after a `ctx.`, as far as the name runs.
fn root_segment(rest: &str) -> Option<&str> {
    let end = rest
        .find(|c: char| !(c.is_alphanumeric() || c == '_'))
        .unwrap_or(rest.len());
    (end > 0).then(|| &rest[..end])
}

/// One divide, as the script wrote it.
struct Divide<'a> {
    read: &'a str,
    divisor: i64,
    round_unit: Option<i64>,
}

/// Read `<x> / <n>`, or the same wrapped in the vendor's rounding.
fn read_divide(rhs: &str) -> Option<Divide<'_>> {
    let rhs = rhs.trim().trim_end_matches(';').trim();
    let (head, tail) = rhs.rsplit_once('/')?;
    let head = head.trim();

    let Some(inner) = head
        .strip_prefix("Math.round(")
        .and_then(|rest| rest.trim_end().strip_suffix(')').map(str::trim))
    else {
        return Some(Divide {
            read: head,
            divisor: literal_divisor(tail)?,
            round_unit: None,
        });
    };

    // `Math.round((x / d) * u) / u` -- the multiplier and the final divisor are
    // the same unit, and a script where they differ is doing something this has
    // not read.
    let unit = literal_divisor(tail)?;
    let (product, multiplier) = inner.rsplit_once('*')?;
    if literal_divisor(multiplier)? != unit {
        return None;
    }
    let product = product.trim();
    let product = product
        .strip_prefix('(')
        .and_then(|rest| rest.strip_suffix(')'))
        .unwrap_or(product);
    let (read, by) = product.rsplit_once('/')?;
    Some(Divide {
        read: read.trim(),
        divisor: literal_divisor(by)?,
        round_unit: Some(unit),
    })
}

/// Every `def <name> = ctx...;` in the script, as the dotted path it reads.
fn locals_bound_to_paths(script: &str) -> HashMap<&str, String> {
    let mut locals = HashMap::new();
    for statement in script.split([';', '\n']) {
        let Some(rest) = statement.trim().strip_prefix("def ") else {
            continue;
        };
        let Some((name, value)) = rest.split_once('=') else {
            continue;
        };
        let (name, value) = (name.trim(), value.trim());
        if !is_identifier(name) || !value.starts_with("ctx") {
            continue;
        }
        if let Some(path) = painless_path(value) {
            locals.insert(name, path);
        }
    }
    locals
}

/// Every `for (def <var> : <list>)` whose list is a literal of strings, as the
/// names the variable takes.
fn loop_variables(script: &str) -> HashMap<&str, Vec<String>> {
    let lists = literal_string_lists(script);
    let mut loops = HashMap::new();
    for tail in script.split("for (def ").skip(1) {
        let Some((header, _)) = tail.split_once(')') else {
            continue;
        };
        let Some((variable, list)) = header.split_once(':') else {
            continue;
        };
        let (variable, list) = (variable.trim(), list.trim());
        if !is_identifier(variable) {
            continue;
        }
        if let Some(names) = lists.get(list) {
            loops.insert(variable, names.clone());
        }
    }
    loops
}

/// Every `def <name> = ['a', 'b'];` in the script.
fn literal_string_lists(script: &str) -> HashMap<&str, Vec<String>> {
    let mut lists = HashMap::new();
    for statement in script.split([';', '\n']) {
        let Some(rest) = statement.trim().strip_prefix("def ") else {
            continue;
        };
        let Some((name, value)) = rest.split_once('=') else {
            continue;
        };
        let name = name.trim();
        if !is_identifier(name) {
            continue;
        }
        let Some(Value::Array(members)) = crate::params::literal_value(value.trim()) else {
            continue;
        };
        let names: Vec<String> = members
            .iter()
            .filter_map(|member| member.as_str().map(str::to_owned))
            .collect();
        if names.len() == members.len() && !names.is_empty() {
            lists.insert(name, names);
        }
    }
    lists
}

/// The field path (or paths) an assignment target names.
///
/// A subscript holding a LOOP VARIABLE fans the one statement out to every name
/// the list carries, which is how panw writes its eight cpu fields.
fn resolve_targets(
    target: &str,
    locals: &HashMap<&str, String>,
    loops: &HashMap<&str, Vec<String>>,
) -> Option<Vec<String>> {
    if let Some((head, subscript)) = target.split_once('[')
        && let Some(variable) = subscript.trim_end().strip_suffix(']')
        && let Some(names) = loops.get(variable.trim())
    {
        let base = resolve_scalar(head, locals)?;
        return Some(names.iter().map(|name| format!("{base}.{name}")).collect());
    }
    resolve_scalar(target, locals).map(|path| vec![path])
}

/// One assignment target as a dotted path, following a local to the `ctx.` path
/// it was bound to.
fn resolve_scalar(target: &str, locals: &HashMap<&str, String>) -> Option<String> {
    let target = target.trim();
    if target.starts_with("ctx") {
        return painless_path(target);
    }
    let end = target.find(['.', '['])?;
    let base = locals.get(&target[..end])?;
    let member = painless_path(&format!("ctx{}", &target[end..]))?;
    Some(format!("{base}.{member}"))
}

/// A bare Painless identifier, which is what a local or a loop variable is.
fn is_identifier(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "rescale_tests.rs"]
mod tests;
