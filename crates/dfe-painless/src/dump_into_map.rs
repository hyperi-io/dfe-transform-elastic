// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A field that arrived as a scalar, rescued into the map the pipeline expects.
//!
//! `contrast_security` delivers its payload under `event`, and a malformed
//! delivery puts a bare string there instead of the object. Everything after it
//! writes into `event.*`, so the first such write raises `type mismatch at
//! 'event': expected object, got string`, the module's own `on_failure` handler
//! raises the same error again, and the whole transform fails. The vendor's
//! FIRST processor exists to rescue exactly that, guarded on
//! `ctx.event != null && !(ctx.event instanceof Map)`.
//!
//! Unclaimed it costs all three of the source's malformed fixtures, and those
//! were the only three errors left in the whole compat corpus.
//!
//! **`Json.dump` of a scalar is the QUOTED JSON form, not the bare text.**
//! Elasticsearch writes `"\"malformed-event-payload\""` for that
//! 23-character string, quotes included, which is what `serde_json::to_string`
//! gives.
//!
//! **This reads the whole script, not the `Json.dump` keyword.** Thirteen call
//! sites across eleven sources spell `Json.dump` and twelve of them do
//! something else -- build `ctx.event` where it is null, append a
//! `preserve_original_event` tag, serialise one member of a walked list.
//! Claiming those on the keyword would write `event.original`, which
//! `tests/compare-policy.yaml` excludes from comparison, and silently drop the
//! `tags` and the members they do write. So the parse reads three statements in
//! one fixed order and declines anything carrying a fourth.

use serde_json::{Map, Value};

use dfe_core::Event;

use crate::params::clean_path;

/// A field swapped for a map holding the field's own serialised form.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DumpIntoMap {
    /// The field read, and then overwritten with the map.
    path: String,
    /// The member of that map the dump is stored under.
    member: String,
}

/// The document path a `ctx.` reference names, or `None` where it is not one.
///
/// A subscript -- quoted or numeric -- fails the character test rather than
/// resolving to something [`Event::set`] cannot walk.
fn ctx_path(reference: &str) -> Option<String> {
    let path = clean_path(reference.trim().strip_prefix("ctx.")?);
    let named = path.split('.').all(|segment| {
        !segment.is_empty() && segment.chars().all(|c| c.is_alphanumeric() || c == '_')
    });
    named.then_some(path)
}

/// Read the rescue, or decline it.
///
/// The three statements have to be the WHOLE script: read a field into a local,
/// replace the field with an empty map, store the local's JSON dump under one
/// member of it. A fourth statement means the script does something this cannot
/// reproduce, and a claim it cannot serve is worse than no claim -- the call
/// site then reports success having written a fraction of the document.
#[must_use]
pub fn parse_dump_into_map(script: &str) -> Option<DumpIntoMap> {
    let mut statements = script
        .split(';')
        .map(str::trim)
        .filter(|statement| !statement.is_empty());
    let (bind, create, store) = (statements.next()?, statements.next()?, statements.next()?);
    if statements.next().is_some() {
        return None;
    }

    // `def <local> = ctx.<path>`
    let (local, read) = bind.strip_prefix("def ")?.split_once(" = ")?;
    let local = local.trim();
    if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let path = ctx_path(read)?;

    // `ctx.<path> = new HashMap()`. Painless spells one empty map two ways and
    // means the same by both.
    let (created, empty) = create.split_once(" = ")?;
    if ctx_path(created)? != path || !matches!(empty.trim(), "new HashMap()" | "[:]") {
        return None;
    }

    // `ctx.<path>.<member> = Json.dump(<local>)`, and the local has to be the
    // one the first statement bound -- dumping anything else is a different
    // script that this would write the wrong value for.
    let (target, dumped) = store.split_once(" = ")?;
    let member = ctx_path(target)?
        .strip_prefix(&format!("{path}."))?
        .to_owned();
    if member.contains('.') {
        return None;
    }
    let inner = dumped
        .trim()
        .strip_prefix("Json.dump(")?
        .strip_suffix(')')?;
    if inner.trim() != local {
        return None;
    }

    Some(DumpIntoMap { path, member })
}

/// Replace the field with the map, or leave the document alone.
///
/// **A field with no value declines**, which is exactly the case every call
/// site's own guard excludes: the vendor gates this on
/// `ctx.event != null && !(ctx.event instanceof Map)`, and Painless reads an
/// absent field and an explicit null as the same null. Nothing in the corpus
/// says what Elasticsearch writes for a dump of null, so this writes nothing
/// rather than inventing a value for a branch that cannot be reached.
pub fn run_dump_into_map(event: &mut Event, pattern: &DumpIntoMap) -> bool {
    let Some(original) = event.get(&pattern.path).filter(|value| !value.is_null()) else {
        return false;
    };
    let Ok(dumped) = serde_json::to_string(original) else {
        return false;
    };
    let mut wrapped = Map::with_capacity(1);
    wrapped.insert(pattern.member.clone(), Value::String(dumped));
    event.set(&pattern.path, Value::Object(wrapped)).is_ok()
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#`. Keeping them character-identical is what lets a script
// be copied straight from a module into a test.
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
#[path = "dump_into_map_tests.rs"]
mod tests;
