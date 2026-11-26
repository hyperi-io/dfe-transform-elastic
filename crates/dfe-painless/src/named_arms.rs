// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A params table read for one key, with literal arms answering the names the
//! table does not carry.
//!
//! `claude_code` and `claude_cowork` classify their events this way -- two names
//! answered inline, one answered with no write at all, and the rest looked up.
//!
//! `GuardedCopy` claims the script and reads the arms correctly, then meets a
//! guard on a local it cannot resolve and takes the ELSE branch every time --
//! so every name the table carries was classified `host`/`info`. The three
//! fixtures that passed are the three the arms answer.
//!
//! `RowOrDefaults` is the same lookup without the arms. It declines this one at
//! the table name: `getOrDefault` is not the spelling it splits on, and the
//! text it reads instead is not a table.

use serde_json::{Map, Value};

use crate::params::{balanced, clean_path, literal_writes, skip_trivia};
use dfe_core::Event;

/// One literal arm, and what it writes before returning.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NamedArm {
    /// The subject values this arm answers.
    values: Vec<String>,
    writes: Vec<(String, Value)>,
}

/// A column taken off the row the table returns.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ArmedColumn {
    target: String,
    member: String,
}

/// Literal arms first, then a params table, then literal defaults.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ArmedTable {
    /// The `ctx.` path the key is read from.
    subject: String,
    /// Literal writes above the branching, which run for every event.
    prelude: Vec<(String, Value)>,
    arms: Vec<NamedArm>,
    /// The params member holding the table.
    table: String,
    columns: Vec<ArmedColumn>,
    defaults: Vec<(String, Value)>,
}

/// The disjunction of `<key> == '<literal>'` a guard spells, or nothing.
fn compared_values(guard: &str, key: &str) -> Option<Vec<String>> {
    let mut values = Vec::new();
    for term in guard.split("||") {
        let (subject, wanted) = term.trim().split_once("==")?;
        if subject.trim() != key {
            return None;
        }
        let wanted = wanted.trim().trim_matches(['\'', '"']).to_owned();
        if wanted.is_empty() {
            return None;
        }
        values.push(wanted);
    }
    (!values.is_empty()).then_some(values)
}

/// Each `ctx.<target> = <row>.<member>;` in a block.
fn row_columns(block: &str, row: &str) -> Vec<ArmedColumn> {
    let member_of = format!("= {row}.");
    let mut columns = Vec::new();
    for (at, _) in block.match_indices(&member_of) {
        let Some(target) = block[..at]
            .trim_end()
            .rsplit(['\n', ';', '{', '}'])
            .next()
            .map(str::trim)
            .and_then(|line| line.strip_prefix("ctx."))
        else {
            continue;
        };
        let member: String = block[at + member_of.len()..]
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        if member.is_empty() || target.contains(['(', '[']) {
            continue;
        }
        columns.push(ArmedColumn {
            target: clean_path(target),
            member,
        });
    }
    columns
}

/// Read the key, the arms, the table and both branches, or decline.
#[must_use]
pub fn parse_armed_table(script: &str) -> Option<ArmedTable> {
    // `def <row> = params.<table>.getOrDefault(<key>, null);`
    let (before_lookup, tail) = script.split_once(".getOrDefault(")?;
    let (declaration, table) = before_lookup.rsplit_once("params.")?;
    if table.is_empty() || !table.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let row = declaration
        .trim_end()
        .strip_suffix('=')?
        .trim_end()
        .rsplit([' ', '\n', ';'])
        .next()?
        .trim();
    let (key, tail) = tail.split_once(')')?;
    let key = key.split(',').next()?.trim();
    if row.is_empty() || key.is_empty() || !row.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let subject = match key.strip_prefix("ctx.") {
        Some(path) => clean_path(path),
        None => crate::common::ctx_path_bound_to(script, key)?,
    };

    // Everything above the lookup's own declaration: the literal writes, then
    // the arms.
    let above = declaration;
    let first_arm = above.find("if (").unwrap_or(above.len());
    let prelude = literal_writes(&above[..first_arm]);

    let mut arms = Vec::new();
    let mut rest = &above[first_arm..];
    while let Some(after_if) = rest.strip_prefix("if ") {
        let (guard, after) = balanced(skip_trivia(after_if), '(', ')')?;
        let (block, after) = balanced(skip_trivia(after), '{', '}')?;
        // An arm that falls through rather than returning would leave the
        // lookup to overwrite it, which is a different script.
        if !block.contains("return") {
            return None;
        }
        arms.push(NamedArm {
            values: compared_values(guard, key)?,
            writes: literal_writes(block),
        });
        rest = skip_trivia(after);
        // Only the `def <row> = ...` binding may sit between the last arm and
        // the lookup.
        if !rest.starts_with("if ") {
            if !rest.trim_start().starts_with("def ") && !rest.trim().is_empty() {
                return None;
            }
            break;
        }
    }
    if arms.is_empty() {
        return None;
    }

    // `if (<row> != null) { <columns> } else { <defaults> }`
    let branch = tail.find(&format!("if ({row} != null)"))?;
    let branch = &tail[branch + "if".len()..];
    let (_, after) = balanced(skip_trivia(branch), '(', ')')?;
    let (then, after) = balanced(skip_trivia(after), '{', '}')?;
    let otherwise = skip_trivia(after)
        .strip_prefix("else")
        .and_then(|rest| balanced(skip_trivia(rest), '{', '}'))
        .map_or_default(|(block, _)| block);

    let columns = row_columns(then, row);
    if columns.is_empty() {
        return None;
    }

    Some(ArmedTable {
        subject,
        prelude,
        arms,
        table: table.to_owned(),
        columns,
        defaults: literal_writes(otherwise),
    })
}

/// Answer the key from an arm if one names it, else from the table, else from
/// the defaults.
#[must_use]
pub fn armed_table(event: &mut Event, pattern: &ArmedTable, params: &Map<String, Value>) -> bool {
    for (target, value) in &pattern.prelude {
        let _ = event.set(target, value.clone());
    }

    // An absent subject compares equal to no arm and looks up no row, so the
    // script falls all the way through to its defaults.
    let key = event.get_as_string(&pattern.subject).unwrap_or_default();

    for arm in &pattern.arms {
        if arm.values.contains(&key) {
            for (target, value) in &arm.writes {
                let _ = event.set(target, value.clone());
            }
            // The arm returns out of the script.
            return true;
        }
    }

    let Some(row) = params.get(&pattern.table).and_then(|table| table.get(&key)) else {
        for (target, value) in &pattern.defaults {
            let _ = event.set(target, value.clone());
        }
        return true;
    };
    let row = row.clone();
    for column in &pattern.columns {
        if let Some(value) = row.get(&column.member).filter(|value| !value.is_null()) {
            let _ = event.set(&column.target, value.clone());
        }
    }
    true
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#`.
#[allow(clippy::needless_raw_string_hashes)]
#[path = "named_arms_tests.rs"]
mod tests;
