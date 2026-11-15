// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Totals summed from two sides, where an absent side counts as zero.
//!
//! `sum_directions` already adds `source.<unit>` and `destination.<unit>` into
//! `network.<unit>`, and it requires BOTH sides to be numbers. suricata's
//! script does not: it reads each side through a `getOrZero` helper, so a flow
//! reporting only one direction still gets a total. It then guards every write
//! on the total being greater than zero, so an all-zero flow gets no
//! `network.*` at all.
//!
//! Those two differences are the whole of suricata's `network.bytes` and
//! `network.packets`, each wrong on 27 of its 64 events, with `network.packets`
//! alone unlocking 24 of them.
//!
//! Kept apart from `sum_directions` deliberately. That runner is shared, and
//! whatever binds it today may well be the both-required spelling; loosening it
//! would start writing totals for sources that currently and correctly write
//! none.

use serde_json::{Value, json};

use dfe_core::Event;

/// One total: where it lands, and the two fields summed into it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Total {
    /// The dotted path the total is written to.
    target: String,
    /// The two sides, either of which may be absent.
    left: String,
    right: String,
}

impl Total {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        target: impl Into<String>,
        left: impl Into<String>,
        right: impl Into<String>,
    ) -> Self {
        Self {
            target: target.into(),
            left: left.into(),
            right: right.into(),
        }
    }
}

/// Every total one script writes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SumTotals {
    /// In the order the script writes them.
    totals: Vec<Total>,
}

impl SumTotals {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(totals: Vec<Total>) -> Self {
        Self { totals }
    }

    /// The generated call site that rebuilds this pattern.
    #[cfg(feature = "codegen")]
    #[must_use]
    pub fn direct_call(&self) -> String {
        let totals: Vec<String> = self
            .totals
            .iter()
            .map(|total| {
                format!(
                    "Total::new({:?}, {:?}, {:?})",
                    total.target, total.left, total.right
                )
            })
            .collect();
        format!(
            "sum_totals(event, &SumTotals::new(vec![{}]));",
            totals.join(", ")
        )
    }
}

/// Write each total, treating an absent side as zero.
///
/// A side that is PRESENT and not a number writes nothing at all, not even the
/// totals that would have succeeded: the script reads every side into a local
/// before it writes anything, so a value the `long` helper cannot return throws
/// and the whole processor fails.
pub fn sum_totals(event: &mut Event, pattern: &SumTotals) -> bool {
    let mut written = Vec::with_capacity(pattern.totals.len());
    for total in &pattern.totals {
        let (Some(left), Some(right)) = (side(event, &total.left), side(event, &total.right))
        else {
            return true;
        };
        let sum = left.saturating_add(right);
        // Each write is guarded on its own total, so an all-zero flow gets no
        // target field rather than a zero -- which would be an extra field
        // Elasticsearch does not emit.
        if sum > 0 {
            written.push((total.target.clone(), sum));
        }
    }
    for (target, sum) in written {
        let _ = event.set(&target, json!(sum));
    }
    true
}

/// One side's value: absent is zero, present-and-not-a-number declines.
fn side(event: &Event, path: &str) -> Option<i64> {
    match event.get(path) {
        None | Some(Value::Null) => Some(0),
        Some(value) => value.as_i64(),
    }
}

/// Read the totals off a `getOrZero` script, or decline.
///
/// Every write must resolve to two `getOrZero` locals over paths the script
/// also names, or the whole script is declined -- writing some totals and not
/// others leaves the source looking like it needs polish.
#[must_use]
pub fn parse_sum_totals(script: &str) -> Option<SumTotals> {
    if !script.contains("getOrZero(") {
        return None;
    }

    // `def network=ctx['network'], source=ctx['source'], dest=ctx['destination'];`
    let mut containers: Vec<(String, String)> = Vec::new();
    let mut rest = script;
    while let Some(at) = rest.find("=ctx[") {
        let name = rest[..at].trim_end().rsplit([' ', ',']).next()?.to_owned();
        let path = rest[at + "=ctx[".len()..]
            .split(']')
            .next()?
            .trim()
            .trim_matches(['\'', '"'])
            .to_owned();
        if !name.is_empty() && !path.is_empty() {
            containers.push((name, path));
        }
        rest = &rest[at + "=ctx[".len()..];
    }

    // `def sp=getOrZero(source,'packets'), sb=getOrZero(source,'bytes'), ..`
    let mut locals: Vec<(String, String)> = Vec::new();
    let mut rest = script;
    while let Some(at) = rest.find("=getOrZero(") {
        let name = rest[..at].trim_end().rsplit([' ', ',']).next()?.to_owned();
        let arguments = rest[at + "=getOrZero(".len()..].split(')').next()?;
        let (container, member) = arguments.split_once(',')?;
        let member = member.trim().trim_matches(['\'', '"']);
        let container = container.trim();
        let base = containers
            .iter()
            .find(|(local, _)| local == container)
            .map(|(_, path)| path.clone())?;
        if name.is_empty() || member.is_empty() {
            return None;
        }
        locals.push((name, format!("{base}.{member}")));
        rest = &rest[at + "=getOrZero(".len()..];
    }
    if locals.is_empty() {
        return None;
    }

    // `network['bytes'] = sb+db;`
    let mut totals = Vec::new();
    let mut rest = script;
    while let Some(at) = rest.find("['") {
        let container = rest[..at].trim_end().rsplit([' ', '\n', '(']).next()?;
        let Some(base) = containers
            .iter()
            .find(|(local, _)| local == container)
            .map(|(_, path)| path.clone())
        else {
            rest = &rest[at + 2..];
            continue;
        };
        let after = &rest[at + 2..];
        let Some((unit, tail)) = after.split_once("']") else {
            rest = &rest[at + 2..];
            continue;
        };
        let Some(assigned) = tail.trim_start().strip_prefix('=') else {
            rest = &rest[at + 2..];
            continue;
        };
        let sum = assigned.split(';').next()?.trim();
        let Some((left, right)) = sum.split_once('+') else {
            rest = &rest[at + 2..];
            continue;
        };
        let resolve = |name: &str| {
            locals
                .iter()
                .find(|(local, _)| local == name.trim())
                .map(|(_, path)| path.clone())
        };
        let (Some(left), Some(right)) = (resolve(left), resolve(right)) else {
            rest = &rest[at + 2..];
            continue;
        };
        totals.push(Total::new(format!("{base}.{unit}"), left, right));
        rest = &rest[at + 2..];
    }

    (!totals.is_empty()).then(|| SumTotals::new(totals))
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "totals_tests.rs"]
mod tests;
