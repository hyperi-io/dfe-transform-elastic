// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A params row per id, collected into the field the script writes back.
//!
//! `symantec_endpoint_security` spells this twelve ways across its two streams
//! and it is one intent: seed a collection from what the target already holds,
//! add the params row for every id read off a source, and put the collection
//! back. Nothing claimed any of them -- `IndexedLookup`'s trigger is `.put(`
//! plus `params` with no parse behind it, so it took them and its runner then
//! declined, because the table here is a MAP and that runner wants an array.
//!
//! ```painless
//! def var = new HashSet();
//! if (ctx.email != null && ctx.email.direction != null) {
//!     var = ctx.email.direction;
//! } else {
//!     if (ctx.email == null) { ctx.email = new HashMap(); }
//! }
//! for (def email : ctx.ses.cybox.emails) {
//!   def direction = email.direction_id;
//!   if (params.containsKey(direction.toString())) {
//!     var.add(params.get(direction.toString()));
//!   }
//! }
//! ctx.email.put('direction', var)
//! ```
//!
//! Five things vary and the reader takes all five: `new HashSet()` against
//! `new ArrayList()`, seeded from the target against overwriting it, a scalar
//! id against a list of them against a member of every element of a list,
//! `params.containsKey(k)`/`params.get(k)` against `params[k]`, and the
//! `continue` guard some spellings carry and others do not.

use crate::params::{
    balanced, clean_path, ctx_path_at_end, key_term, last_delimited, literal_call, scalar_text,
};
use dfe_core::Event;
use serde_json::{Map, Value};

/// Where the ids come from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowSource {
    /// Every id under one path -- a scalar, or a list of them.
    Path(String),
    /// One member of every element of a list, itself an id or a list of them.
    Member { list: String, member: String },
}

/// A params row per id, collected into one field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectParamsRows {
    source: RowSource,
    /// The field the collection is written back to.
    target: String,
    /// `new HashSet()` drops a repeat; `new ArrayList()` keeps it.
    dedup: bool,
    /// Whether the script seeds the collection from the target's own value.
    seeded: bool,
}

impl CollectParamsRows {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(source: RowSource, target: impl Into<String>, dedup: bool, seeded: bool) -> Self {
        Self {
            source,
            target: target.into(),
            dedup,
            seeded,
        }
    }
}

/// Read the source, target and collection type, or decline.
pub fn parse_collect_params_rows(script: &str) -> Option<CollectParamsRows> {
    // Container and key off the SAME `.put(`, and the collection local off its
    // second argument -- a `.put(` elsewhere in the script names a field this
    // statement does not write.
    let (at, member) = literal_call(script, ".put(")?;
    let parent = ctx_path_at_end(&script[..at])?;
    let (arguments, _) = balanced(&script[at + ".put".len()..], '(', ')')?;
    let local = arguments.rsplit(',').next()?.trim();
    if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    let dedup = match binding(script, local)?.trim() {
        "new HashSet()" => true,
        "new ArrayList()" => false,
        _ => return None,
    };

    let target = format!("{parent}.{member}");
    let seeded = script.contains(&format!(" {local} = ctx.{target}"))
        || script.contains(&format!(" {local} = ctx?.{target}"));

    Some(CollectParamsRows {
        source: row_source(script)?,
        target,
        dedup,
        seeded,
    })
}

/// The key expression the lookup is written against, resolved to a `ctx.` path.
fn row_source(script: &str) -> Option<RowSource> {
    let key = last_delimited(script, "params.containsKey(", '(', ')')
        .or_else(|| last_delimited(script, "params.get(", '(', ')'))
        .or_else(|| last_delimited(script, "params[", '[', ']'))?;
    resolve(script, key_term(&key)?)
}

/// Follow a term back through loop headers and bindings to a `ctx.` path.
fn resolve(script: &str, term: String) -> Option<RowSource> {
    if let Some(path) = ctx_rooted(&term) {
        return Some(RowSource::Path(path));
    }

    // A dotted term is a member of whatever its head resolves to; a bare one is
    // the head itself. Both heads resolve the same way, so the member is split
    // off first and the walk below is written once.
    let (head, member) = match term.split_once('.') {
        Some((head, member)) => (head.to_string(), Some(subscripted(member)?)),
        None => (term, None),
    };

    let bound = loop_iterable(script, &head).or_else(|| binding(script, &head))?;
    let bound = bound.trim().to_string();

    if let Some(path) = ctx_rooted(&bound) {
        return Some(match member {
            Some(member) => RowSource::Member { list: path, member },
            None => RowSource::Path(path),
        });
    }

    // One more hop: `for (def id : file.attribute_ids)` inside
    // `for (def file : ctx.ses.cybox.files)`. The inner loop names the member
    // and the outer names the list, so a term resolved through both is a
    // member read -- whatever the inner iterable's own member depth.
    let (outer, inner) = bound.split_once('.')?;
    let list = ctx_rooted(loop_iterable(script, outer)?.trim())?;
    member.is_none().then_some(RowSource::Member {
        list,
        member: subscripted(inner)?,
    })
}

/// A member name with any subscript taken off it.
///
/// `answer.flags[i]` inside `for (int i = 0; i < answer.flags.length; i++)` is
/// every element of `flags`, and the runner already flattens a member that
/// holds a list -- so the subscript is noise, and leaving it on names a member
/// no document carries.
fn subscripted(member: &str) -> Option<String> {
    let member = member.split('[').next()?.trim();
    (!member.is_empty() && !member.contains(['.', ' ', ')'])).then(|| member.to_string())
}

/// The path a `ctx.`-rooted expression names, or `None` for anything else.
fn ctx_rooted(expression: &str) -> Option<String> {
    let path = expression
        .trim()
        .strip_prefix("ctx.")
        .or_else(|| expression.trim().strip_prefix("ctx?."))?;
    let path = clean_path(path);
    (!path.is_empty() && !path.contains(['(', ' ', '[', ';'])).then_some(path)
}

/// What `for (def <name> : <iterable>)` walks.
fn loop_iterable(script: &str, name: &str) -> Option<String> {
    // The colon is spelled with and without a leading space across the
    // integrations, so both are tried.
    ["for (def {} :", "for (def {}:"]
        .into_iter()
        .find_map(|form| {
            let at = script.find(&form.replace("{}", name))?;
            let (header, _) = balanced(&script[at + "for ".len()..], '(', ')')?;
            Some(header.split_once(':')?.1.trim().to_string())
        })
}

/// The expression `def <name> = ...;` binds, whatever the declaration keyword.
fn binding(script: &str, name: &str) -> Option<String> {
    let marker = format!(" {name} = ");
    let at = script.find(&marker)? + marker.len();
    Some(script[at..].split(';').next()?.trim().to_string())
}

/// Write the params row for every id the source carries.
pub fn collect_params_rows(
    event: &mut Event,
    pattern: &CollectParamsRows,
    params: &Map<String, Value>,
) -> bool {
    let mut rows: Vec<Value> = if pattern.seeded {
        match event.get(&pattern.target) {
            Some(Value::Array(items)) => items.clone(),
            Some(Value::Null) | None => Vec::new(),
            Some(other) => vec![other.clone()],
        }
    } else {
        Vec::new()
    };

    for id in ids(event, &pattern.source) {
        let Some(row) = params.get(&id) else {
            continue;
        };
        if pattern.dedup && rows.contains(row) {
            continue;
        }
        rows.push(row.clone());
    }

    // Written even where it is empty, because the script's own `put` is
    // unconditional and the processor's `if` is what guards an absent source.
    let _ = event.set(&pattern.target, Value::Array(rows));
    true
}

/// Every id the source names, in the order the script would read them.
fn ids(event: &Event, source: &RowSource) -> Vec<String> {
    match source {
        RowSource::Path(path) => event.get(path).map(flatten).unwrap_or_default(),
        RowSource::Member { list, member } => {
            let Some(Value::Array(items)) = event.get(list) else {
                return Vec::new();
            };
            items
                .iter()
                .filter_map(|item| item.get(member))
                .flat_map(flatten)
                .collect()
        }
    }
}

/// A value's ids, the way Painless would stringify them for a map key.
fn flatten(value: &Value) -> Vec<String> {
    match value {
        Value::Array(items) => items.iter().flat_map(flatten).collect(),
        other => scalar_text(other).into_iter().collect(),
    }
}

#[cfg(test)]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#`.
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
#[path = "collect_rows_tests.rs"]
mod tests;
