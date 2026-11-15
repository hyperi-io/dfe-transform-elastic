// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A numbered severity level renamed to the word its own record type uses.
//!
//! qualys ships one table of level words and two ways of reaching it. Its
//! `knowledge_base` stream spells the ladder out, and bands an informational
//! finding differently from a vulnerability.
//!
//! The `asset_host_detection` stream writes the same table through `params`, and
//! `crate::params::TableGate` is what lets that spelling keep the same
//! membership test.
//!
//! THREE ways to write nothing, and the script needs all three. A type in no
//! group keeps the digit it arrived with; so does a level the chosen group does
//! not band, because "Information Gathered" stops at 3 and a 4 there is still a
//! 4; and so does a record carrying no type at all, which the script's own
//! `instanceof` guard returns on before it reaches a band.

use dfe_core::Event;
use serde_json::json;

use crate::common::{if_block, painless_path, vocabulary_tested};
use crate::params::clean_path;

/// A level, the field that picks its band table, and the tables.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LevelLabels {
    /// The numbered level, read as text and parsed.
    level: String,
    /// The field deciding which group of bands applies.
    kind: String,
    /// The field every band writes.
    target: String,
    /// The groups in the order the script tests them.
    groups: Vec<LevelGroup>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct LevelGroup {
    /// The kinds this group covers.
    kinds: Vec<String>,
    /// `(level, label)` in the order the script bands them.
    bands: Vec<(i64, String)>,
}

/// Read the two-level ladder, or decline.
///
/// Structural: the level comes from the `parseLong` the script actually writes,
/// the kind from the local every group guard tests, and each group's bands from
/// its own body. Every band must write the same field, and that field must be
/// the only one the script writes -- a ladder sitting beside other work is a
/// different script, and claiming it would drop the rest.
pub fn parse_level_labels(script: &str) -> Option<LevelLabels> {
    let (head, rest) = script.split_once("Long.parseLong(ctx.")?;
    let (path, after) = rest.split_once(')')?;
    let level = clean_path(path.trim());
    if level.is_empty() || level.contains(char::is_whitespace) {
        return None;
    }
    let level_local = declared_before(head)?;

    let mut kind_local: Option<String> = None;
    let mut target: Option<String> = None;
    let mut groups = Vec::new();
    let mut chain = after.trim_start().strip_prefix(';')?;

    while let Some((condition, body, tail)) = if_block(chain) {
        let (kinds, tested) = kinds_tested(condition)?;
        if kind_local.get_or_insert_with(|| tested.clone()) != &tested {
            return None;
        }
        groups.push(LevelGroup {
            kinds,
            bands: bands_written(body, &level_local, &mut target)?,
        });
        // A bare `else` is a catch-all group with no kinds to name, so the
        // reader stops rather than guessing which types it covers.
        let Some(next) = tail.trim_start().strip_prefix("else") else {
            break;
        };
        chain = next.trim_start();
    }

    let target = target?;
    if groups.is_empty() || groups.iter().all(|group| group.bands.is_empty()) {
        return None;
    }
    if crate::params::ctx_writes(script)
        .iter()
        .any(|(path, _)| path != &target)
    {
        return None;
    }

    Some(LevelLabels {
        level,
        kind: ctx_path_of(script, &kind_local?)?,
        target,
        groups,
    })
}

/// The local a `def <name> = ` declaration just before `head` ends binds.
fn declared_before(head: &str) -> Option<String> {
    let statement = head.rsplit([';', '\n', '{', '}']).next()?.trim();
    let name = statement
        .strip_suffix('=')?
        .trim()
        .rsplit(char::is_whitespace)
        .next()?;
    (!name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .then(|| name.to_owned())
}

/// The ctx path `local` is bound to, from its `def <local> = ctx.<path>;`.
fn ctx_path_of(script: &str, local: &str) -> Option<String> {
    let path = clean_path(
        script
            .split_once(&format!(" {local} = ctx."))?
            .1
            .split([';', '\n'])
            .next()?
            .trim(),
    );
    (!path.is_empty() && !path.contains(char::is_whitespace)).then_some(path)
}

/// The kinds a group's guard covers, and the local it tests them against.
///
/// Two spellings: a list membership, and a single equality for a group of one.
fn kinds_tested(condition: &str) -> Option<(Vec<String>, String)> {
    if let Some((list, subject)) = condition.split_once("].contains(") {
        let tested = subject.trim().strip_suffix(')')?.trim().to_owned();
        let kinds = vocabulary_tested(&format!("{list}].contains({tested})"), &tested)?;
        return Some((kinds, tested));
    }
    let (tested, literal) = condition.split_once("==")?;
    let tested = tested.trim().to_owned();
    if tested.is_empty() || !tested.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let kind = literal
        .trim()
        .strip_prefix(['\'', '"'])?
        .strip_suffix(['\'', '"'])?;
    (!kind.is_empty()).then(|| (vec![kind.to_owned()], tested))
}

/// A group's inner `<local> == <n>` ladder, as `(level, label)` pairs.
///
/// Every arm writes the same field, and `target` carries that agreement across
/// the groups: two groups writing different fields are two patterns.
fn bands_written(
    body: &str,
    level_local: &str,
    target: &mut Option<String>,
) -> Option<Vec<(i64, String)>> {
    let mut bands = Vec::new();
    let mut chain = body.trim_start();

    while let Some((condition, arm, tail)) = if_block(chain) {
        let level: i64 = condition
            .trim()
            .strip_prefix(level_local)?
            .trim()
            .strip_prefix("==")?
            .trim()
            .parse()
            .ok()?;

        let (lhs, rhs) = arm.split(';').next()?.split_once('=')?;
        let path = painless_path(lhs)?;
        if target.get_or_insert_with(|| path.clone()) != &path {
            return None;
        }
        let label = rhs
            .trim()
            .strip_prefix(['\'', '"'])?
            .strip_suffix(['\'', '"'])?;
        if label.is_empty() {
            return None;
        }
        bands.push((level, label.to_owned()));

        let Some(next) = tail.trim_start().strip_prefix("else") else {
            break;
        };
        chain = next.trim_start();
    }

    (!bands.is_empty()).then_some(bands)
}

/// Rename the level to its group's word, or leave it exactly as it arrived.
pub fn level_labels(event: &mut Event, pattern: &LevelLabels) -> bool {
    let Some(kind) = event.get_str(&pattern.kind) else {
        return true;
    };
    let Some(level) = event
        .get_str(&pattern.level)
        .and_then(|text| text.trim().parse::<i64>().ok())
    else {
        return true;
    };
    let Some(group) = pattern
        .groups
        .iter()
        .find(|group| group.kinds.iter().any(|named| named == kind))
    else {
        return true;
    };
    if let Some((_, label)) = group.bands.iter().find(|(banded, _)| *banded == level) {
        let _ = event.set(&pattern.target, json!(label));
    }
    true
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "level_labels_tests.rs"]
mod tests;
