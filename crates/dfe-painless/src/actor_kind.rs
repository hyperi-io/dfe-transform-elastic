// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! One actor record routed to a person or to an application.
//!
//! `atlassian_cloud`'s audit stream carries the same `actor` record for a human
//! and for an installed app, and only the record's own contents say which. The
//! script decides, then sends the SAME members to different destinations:
//!
//! ```painless
//! def actor = ctx.json.attributes.actor;
//! boolean isUser = false;
//! if (actor.email instanceof String && actor.email.indexOf('@') >= 0) {
//!   isUser = true;
//! } else if (actor.id instanceof String) {
//!   String id = actor.id;
//!   if (id.contains(':') || id ==~ /^[0-9a-fA-F]{8}-.../) { isUser = true; }
//! }
//! if (isUser) { ctx.user.id = actor.id; ... }
//! else { ...actor.app.id = actor.id; ... }
//! ```
//!
//! Unclaimed it costs `related.user`, `user.email`, `user.id` and `user.name`
//! on 9 of `atlassian_cloud`'s 10 events, and the app branch's `.app.id` and
//! `.app.name` on the tenth.
//!
//! **The UUID test is a byte check, not a regex.** `==~` is a FULL match and
//! the pattern is fixed-width, so `36 chars, hyphens at 8/13/18/23, hex
//! elsewhere` decides it exactly -- no `cached_regex!`, which the hot-path rule
//! forbids here anyway. The parse still demands the script spell that literal
//! pattern, so a script testing something else declines rather than being read
//! as this one.
//!
//! **The destination map is built, not incidental.** Painless will not create a
//! parent, so the script spells the ternary itself and an actor carrying
//! neither member still leaves the empty map behind. Skipping it would drop a
//! key Elasticsearch emits.

use serde_json::{Map, Value};

use dfe_core::Event;

use crate::params::{clean_path, ctx_locals, is_ctx_path};

/// The anchored UUID the classifier tests an identifier against.
///
/// Quoted whole, because the pattern IS the test: a script matching a different
/// one means something else by "this actor is a person".
const UUID_PATTERN: &str = "==~ /^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-\
                            [0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/";

/// One member copied off the actor record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorCopy {
    /// The member read off the record.
    member: String,
    /// The full path it is written to.
    target: String,
    /// Whether the script guards this copy on the ADDRESS test rather than on
    /// the member being non-null. An actor that qualified as a person by its
    /// identifier alone carries no address, and copying one anyway would write
    /// a field the vendor pipeline leaves absent.
    on_address: bool,
}

/// Where one branch writes, and what it writes there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorBranch {
    /// The map the script builds before writing into it.
    parent: String,
    /// The copies, in the order the script makes them -- which is the order the
    /// keys land in, and `preserve_order` is what parity rests on.
    copies: Vec<ActorCopy>,
}

/// An actor record, the tests that classify it, and the two destinations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActorKind {
    /// The record classified, as a ctx path.
    actor: String,
    /// The member whose `@` makes the actor a person.
    address: String,
    /// The member whose text is tested for a colon or a UUID.
    identifier: String,
    /// Where a person's members go.
    person: ActorBranch,
    /// Where everything else's go.
    application: ActorBranch,
}

/// Whether `text` is what the script's anchored UUID pattern matches.
///
/// `8-4-4-4-12` hex digits, hyphen-separated, and nothing either side. ASCII
/// only, which is all `[0-9a-fA-F]` admits.
fn is_uuid(text: &str) -> bool {
    let bytes = text.as_bytes();
    if bytes.len() != 36 {
        return false;
    }
    bytes.iter().enumerate().all(|(at, byte)| {
        if matches!(at, 8 | 13 | 18 | 23) {
            *byte == b'-'
        } else {
            byte.is_ascii_hexdigit()
        }
    })
}

/// The `instanceof String && ...indexOf('@') >= 0` test, as the script spells it.
fn address_test(local: &str, member: &str) -> String {
    format!("{local}.{member} instanceof String && {local}.{member}.indexOf('@') >= 0")
}

/// Whether `name` is a bare Painless identifier.
fn is_identifier(name: &str) -> bool {
    !name.is_empty()
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        && !name.starts_with(|c: char| c.is_ascii_digit())
}

/// The `{ ... }` block that opens at `from`, brace-counted.
///
/// Returns the block's contents and the offset just past its close. The counter
/// starts at the branch, so the `{8}` repetitions inside the UUID pattern --
/// which sit in the deciding block ABOVE it -- are behind it and cannot be
/// read as structure.
fn block_at(script: &str, from: usize) -> Option<(&str, usize)> {
    let open = script[from..].find('{')? + from;
    let mut depth = 0usize;
    for (at, c) in script[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some((&script[open + 1..open + at], open + at + 1));
                }
            }
            _ => {}
        }
    }
    None
}

/// The map a branch builds for itself, read off its `: new HashMap()`.
fn created_parent(block: &str) -> Option<String> {
    let at = block.rfind(" : new HashMap()")?;
    let path = clean_path(block[..at].rsplit_once("ctx.")?.1);
    is_ctx_path(&path).then_some(path)
}

/// Resolve a left-hand side against the map the branch built.
///
/// Written either as the full path -- `ctx.user.id` -- or through the local the
/// creation bound, `app.id` beside `def app = ctx....app != null ? ...`. A
/// left-hand side that names neither is a write somewhere else, and the arm
/// declines rather than redirecting it.
fn resolve_target(block: &str, parent: &str, lhs: &str) -> Option<String> {
    let (head, member) = lhs.rsplit_once('.')?;
    if !is_identifier(member) {
        return None;
    }
    let target = format!("{parent}.{member}");
    if let Some(path) = head.strip_prefix("ctx.") {
        return (clean_path(path) == parent).then_some(target);
    }
    let bound = format!("def {head} = ctx.{parent} ");
    block.contains(&bound).then_some(target)
}

/// The copies one branch makes, in script order.
fn branch_copies(block: &str, local: &str, address: &str, parent: &str) -> Option<Vec<ActorCopy>> {
    let assign = format!(" = {local}.");
    let mut copies = Vec::new();
    for statement in block.split(';') {
        let Some((head, member)) = statement.rsplit_once(&assign) else {
            continue;
        };
        let member = member.trim();
        if !is_identifier(member) {
            return None;
        }
        let lhs = head.rsplit(['{', '}', '\n']).next()?.trim();
        let target = resolve_target(block, parent, lhs)?;
        // Every copy the vendor makes is guarded; an unguarded one would write
        // a null where the pipeline writes nothing.
        let on_address = member == address;
        let guard = if on_address {
            address_test(local, address)
        } else {
            format!("{local}.{member} != null")
        };
        if !statement.contains(&guard) {
            return None;
        }
        copies.push(ActorCopy {
            member: member.to_owned(),
            target,
            on_address,
        });
    }
    (!copies.is_empty()).then_some(copies)
}

/// Read the classifier, or decline it.
#[must_use]
pub fn parse_actor_kind(script: &str) -> Option<ActorKind> {
    // The record, and the local the rest of the script names it by.
    let (local, actor) = ctx_locals(script).into_iter().next()?;
    if !is_identifier(&local) || !is_ctx_path(&actor) {
        return None;
    }

    // The flag the two tests set, and the branch it chooses.
    let flag = script
        .split_once("boolean ")?
        .1
        .split_once(" = false;")?
        .0
        .trim();
    if !is_identifier(flag) {
        return None;
    }

    // The address test names the member that makes an actor a person.
    let address = script
        .split_once(".indexOf('@') >= 0")?
        .0
        .rsplit_once(&format!("{local}."))?
        .1
        .to_owned();
    if !is_identifier(&address) || !script.contains(&address_test(&local, &address)) {
        return None;
    }

    // The identifier arm, and the two tests it applies. Both are demanded, so a
    // script testing something else is not read as this one.
    let identifier = script
        .split_once(&format!("else if ({local}."))?
        .1
        .split_once(" instanceof String)")?
        .0
        .to_owned();
    if !is_identifier(&identifier)
        || !script.contains(".contains(':')")
        || !script.contains(UUID_PATTERN)
    {
        return None;
    }

    let chosen = script.find(&format!("if ({flag})"))?;
    let (person_block, after) = block_at(script, chosen)?;
    let (application_block, _) = block_at(script, script[after..].find("else")? + after)?;

    let person = read_branch(person_block, &local, &address)?;
    let application = read_branch(application_block, &local, &address)?;

    Some(ActorKind {
        actor,
        address,
        identifier,
        person,
        application,
    })
}

/// One branch's destination and copies.
fn read_branch(block: &str, local: &str, address: &str) -> Option<ActorBranch> {
    let parent = created_parent(block)?;
    let copies = branch_copies(block, local, address, &parent)?;
    Some(ActorBranch { parent, copies })
}

/// Classify the actor and write its members where that says.
pub fn run_actor_kind(event: &mut Event, pattern: &ActorKind) -> bool {
    // Painless throws on a member access against anything but a map, which is
    // a pipeline error rather than a write -- so a record of another type is
    // left alone and the script counted unhandled.
    let Some(Value::Object(record)) = event.get(&pattern.actor) else {
        return false;
    };

    let addressed = record
        .get(&pattern.address)
        .and_then(Value::as_str)
        .is_some_and(|text| text.contains('@'));
    let is_person = addressed
        || record
            .get(&pattern.identifier)
            .and_then(Value::as_str)
            .is_some_and(|text| text.contains(':') || is_uuid(text));

    let branch = if is_person {
        &pattern.person
    } else {
        &pattern.application
    };

    // `!= null` rather than presence: the script's ternary keeps an existing
    // map and builds one over an explicit null.
    let writes: Vec<(String, Value)> = branch
        .copies
        .iter()
        .filter(|copy| if copy.on_address { addressed } else { true })
        .filter_map(|copy| {
            let value = record.get(&copy.member).filter(|v| !v.is_null())?;
            Some((copy.target.clone(), value.clone()))
        })
        .collect();

    if !event.has_value(&branch.parent) {
        let _ = event.set(&branch.parent, Value::Object(Map::new()));
    }
    for (target, value) in writes {
        let _ = event.set(&target, value);
    }
    true
}

#[cfg(test)]
#[path = "actor_kind_tests.rs"]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes
)]
mod tests;
