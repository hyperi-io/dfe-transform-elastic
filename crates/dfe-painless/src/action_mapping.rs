// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! An action's ECS block chosen by a FOUR-TIER lookup over `params`.
//!
//! `atlassian_cloud` classifies every audit action with one script, and the
//! ORDER of the tiers plus where each one stops is the whole answer:
//!
//! ```painless
//! def action = ctx.event.action;
//! ctx.event.kind = 'event';
//! def mapping = null;
//! if (params.exact.containsKey(action)) { mapping = params.exact[action]; }
//! if (mapping == null && params.prefixes instanceof List) {
//!   for (def entry : params.prefixes) {
//!     if (action.startsWith(entry.prefix)) { mapping = entry; break; }
//!   }
//! }
//! if (mapping == null && action.endsWith(params.policy_suffix)) { mapping = params.policy; }
//! if (mapping == null) { mapping = params.default; }
//! ctx.event.category = mapping.category;
//! ctx.event.type = mapping.type;
//! if (mapping.containsKey('outcome')) { ctx.event.outcome = mapping.outcome; }
//! ```
//!
//! The exact table first; then the prefix list IN ITS OWN ORDER, the FIRST
//! match winning because the loop breaks -- `user_login_failed` is listed ahead
//! of `user_login` for exactly that reason; then one suffix rule; then the
//! fallback. Unclaimed it costs `event.kind`, `event.category`, `event.outcome`
//! and `event.type` on all 10 of `atlassian_cloud`'s events.
//!
//! **`params.exact` carrying the key with a NULL value is not a hit.** The
//! script re-tests `mapping == null` at every tier, so a null row falls
//! through to the next one rather than stopping the search.
//!
//! **The stamp is written before the lookup, and unconditionally.** An action
//! the tiers cannot classify still leaves `event.kind` behind.

use serde_json::{Map, Value};

use dfe_core::Event;

use crate::params::{clean_path, is_ctx_path};

/// One member of the chosen mapping, and where it goes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingWrite {
    /// The member read off the mapping.
    member: String,
    /// The field it is written to.
    target: String,
    /// Whether the script guards the write on the mapping carrying the member.
    /// An UNGUARDED write assigns Painless's null when it does not, which is a
    /// field Elasticsearch emits and we would otherwise leave absent.
    guarded: bool,
}

/// The tiers, the fields they are read out of `params` by, and what they write.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionMapping {
    /// The action every tier is looked up by.
    source: String,
    /// The field stamped before any lookup, and the literal it takes.
    stamp: (String, String),
    /// The params key holding the exact table.
    exact: String,
    /// The params key holding the ORDERED prefix list, and the member of each
    /// entry carrying its prefix.
    prefixes: (String, String),
    /// The params keys holding the suffix and the mapping it selects.
    suffix: (String, String),
    /// The params key holding the fallback mapping.
    fallback: String,
    /// The writes, in the order the script makes them.
    writes: Vec<MappingWrite>,
}

/// Whether `name` is a bare Painless identifier.
fn is_identifier(name: &str) -> bool {
    !name.is_empty()
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        && !name.starts_with(|c: char| c.is_ascii_digit())
}

/// The literal a quote opens at the head of `text`.
fn quoted_at(text: &str) -> Option<String> {
    let text = text.trim_start();
    let quote = text.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    let rest = &text[quote.len_utf8()..];
    rest.find(quote).map(|end| rest[..end].to_string())
}

/// Read the tiers, or decline them.
///
/// Every tier is demanded: a script missing one classifies differently, and
/// running three tiers where the vendor runs four picks the wrong row rather
/// than none.
#[must_use]
pub fn parse_action_mapping(script: &str) -> Option<ActionMapping> {
    // `def <action> = ctx.<source>;` opens the script, and `def <mapping> =
    // null;` names the local every tier assigns to.
    let (action, rest) = script
        .trim_start()
        .strip_prefix("def ")?
        .split_once(" = ctx.")?;
    let source = clean_path(rest.split_once(';')?.0);
    let mapping = script.split_once(" = null;")?.0.rsplit_once("def ")?.1;
    if !is_identifier(action) || !is_ctx_path(&source) || !is_identifier(mapping) {
        return None;
    }

    let stamp = parse_stamp(script)?;

    // Tier 1: `params.<exact>.containsKey(<action>)`.
    let exact = script
        .split_once("(params.")?
        .1
        .split_once(".containsKey(")?
        .0
        .to_owned();

    // Tier 2: `params.<list> instanceof List`, walked as `for (def <entry> :
    // params.<list>)` with `<action>.startsWith(<entry>.<member>)`.
    let list = script
        .split_once(" instanceof List")?
        .0
        .rsplit_once("params.")?
        .1
        .to_owned();
    let (entry, member) = script
        .split_once(".startsWith(")?
        .1
        .split_once(')')?
        .0
        .rsplit_once('.')?;
    if !script.contains(&format!("for (def {entry} : params.{list})")) {
        return None;
    }
    let member = member.to_owned();

    // Tier 3: `<action>.endsWith(params.<suffix>)` choosing `params.<policy>`.
    let at = script.find(".endsWith(params.")?;
    let suffix = script[at..]
        .split_once(".endsWith(params.")?
        .1
        .split_once(')')?
        .0
        .to_owned();
    let policy = script[at..]
        .split_once(&format!("{mapping} = params."))?
        .1
        .split_once(';')?
        .0
        .trim()
        .to_owned();

    // Tier 4: the fallback, which is the LAST mapping the script reads out of
    // `params`.
    let fallback = script
        .rsplit_once(&format!("{mapping} = params."))?
        .1
        .split_once(';')?
        .0
        .trim()
        .to_owned();

    for key in [
        exact.as_str(),
        list.as_str(),
        member.as_str(),
        suffix.as_str(),
        policy.as_str(),
        fallback.as_str(),
    ] {
        if !is_identifier(key) {
            return None;
        }
    }

    let writes = parse_writes(script, mapping);
    (!writes.is_empty()).then_some(ActionMapping {
        source,
        stamp,
        exact,
        prefixes: (list, member),
        suffix: (suffix, policy),
        fallback,
        writes,
    })
}

/// The `ctx.<field> = '<literal>';` the script stamps before it looks anything
/// up.
fn parse_stamp(script: &str) -> Option<(String, String)> {
    for statement in script.split(';') {
        let Some((lhs, rhs)) = statement.split_once(" = ") else {
            continue;
        };
        let Some(field) = lhs.trim().strip_prefix("ctx.") else {
            continue;
        };
        let Some(literal) = quoted_at(rhs) else {
            continue;
        };
        let field = clean_path(field);
        if is_ctx_path(&field) {
            return Some((field, literal));
        }
    }
    None
}

/// The `ctx.<target> = <mapping>.<member>;` writes, in script order.
fn parse_writes(script: &str, mapping: &str) -> Vec<MappingWrite> {
    let assign = format!(" = {mapping}.");
    let guard = format!("{mapping}.containsKey(");
    script
        .split(';')
        .filter_map(|statement| {
            let (head, member) = statement.rsplit_once(&assign)?;
            let member = member.trim();
            if !is_identifier(member) {
                return None;
            }
            let lhs = head.rsplit(['{', '}', '\n']).next()?.trim();
            let target = clean_path(lhs.strip_prefix("ctx.")?);
            is_ctx_path(&target).then(|| MappingWrite {
                member: member.to_owned(),
                target,
                guarded: statement.contains(&guard),
            })
        })
        .collect()
}

/// A params value that is present and not null -- what `mapping == null` asks.
fn present(value: Option<&Value>) -> Option<&Value> {
    value.filter(|value| !value.is_null())
}

/// The mapping the four tiers choose for `action`.
fn chosen<'a>(
    pattern: &ActionMapping,
    params: &'a Map<String, Value>,
    action: &str,
) -> Option<&'a Value> {
    if let Some(row) = present(
        params
            .get(&pattern.exact)
            .and_then(|table| table.get(action)),
    ) {
        return Some(row);
    }
    // The list's OWN order, and the first match wins: the script breaks out of
    // the loop, so a longer prefix listed first beats a shorter one after it.
    if let Some(Value::Array(entries)) = params.get(&pattern.prefixes.0) {
        let hit = entries.iter().find(|entry| {
            entry
                .get(&pattern.prefixes.1)
                .and_then(Value::as_str)
                .is_some_and(|prefix| action.starts_with(prefix))
        });
        if let Some(entry) = present(hit) {
            return Some(entry);
        }
    }
    if params
        .get(&pattern.suffix.0)
        .and_then(Value::as_str)
        .is_some_and(|suffix| action.ends_with(suffix))
        && let Some(row) = present(params.get(&pattern.suffix.1))
    {
        return Some(row);
    }
    present(params.get(&pattern.fallback))
}

/// Stamp the literal, then write the chosen mapping's members.
pub fn run_action_mapping(
    event: &mut Event,
    pattern: &ActionMapping,
    params: &Map<String, Value>,
) -> bool {
    // Before the lookup and before the type test, exactly where the script
    // writes it.
    let _ = event.set(&pattern.stamp.0, Value::String(pattern.stamp.1.clone()));

    // `startsWith` and `endsWith` are String methods, so an action of any other
    // type throws in Painless rather than classifying. Borrowed: the chosen
    // mapping comes out of `params`, so the action's borrow ends with the tiers.
    let Some(action) = event.get_str(&pattern.source) else {
        return true;
    };
    let Some(mapping) = chosen(pattern, params, action) else {
        return true;
    };

    for write in &pattern.writes {
        match mapping.get(&write.member) {
            Some(value) => {
                let _ = event.set(&write.target, value.clone());
            }
            // An unguarded write assigns Painless's null, which Elasticsearch
            // keeps in `_source`; a guarded one writes nothing at all.
            None if !write.guarded => {
                let _ = event.set(&write.target, Value::Null);
            }
            None => {}
        }
    }
    true
}

#[cfg(test)]
#[path = "action_mapping_tests.rs"]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
mod tests;
