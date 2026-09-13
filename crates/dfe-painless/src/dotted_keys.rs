// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Entries lifted out of a map by a key that carries DOTS.
//!
//! OpenTelemetry names a resource attribute `service.name` and sends it as one
//! key, so the map holds a single entry called `service.name` rather than a
//! `service` map with a `name` member. Every path reader in this crate walks
//! dots as separators, which is why none of them reaches the entry -- and why
//! the vendor binds the map to a local and subscripts it instead:
//!
//! ```painless
//! def attrs = ctx.aws.bedrock_agentcore.resource.attributes;
//!
//! if (attrs.containsKey('service.name')) {
//!   if (ctx.service == null) ctx.service = new HashMap();
//!   ctx.service.name = attrs['service.name'];
//! }
//! ```
//!
//! `aws_bedrock_agentcore` ships that script three times, once per stream, and
//! the three differ only in where the attribute map sits. Nothing claimed any
//! of them: the guard is a `containsKey` call, which the general guard reader
//! answers as unreadable, and the body opens with a BRACELESS `if`, which the
//! statement walk stops at. `service.name` was therefore absent on all seven of
//! the source's captured events, and the `dissect` behind it -- which cuts that
//! value into `agent_name` and `endpoint_name` -- had nothing to cut.

use serde_json::Value;

use crate::params::{balanced, ctx_path_plain as ctx_path, skip_trivia};
use dfe_core::Event;

/// Entries copied out of ONE map, each under its own `containsKey` guard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DottedKeyCopies {
    /// The `ctx.` path the map sits at.
    map: String,
    /// Each key read, and the `ctx.` path its value lands on, in script order.
    copies: Vec<(String, String)>,
}

/// Copy every entry the script names, skipping the keys this event's map does
/// not carry.
///
/// The key is looked up in the MAP rather than walked as a path: walking
/// `service.name` would ask for a `service` child holding a `name`, which is
/// not what the vendor sends and would write nothing while reporting a write.
pub fn dotted_key_copies(event: &mut Event, pattern: &DottedKeyCopies) -> bool {
    for (key, target) in &pattern.copies {
        let Some(value) = event
            .get(&pattern.map)
            .and_then(Value::as_object)
            .and_then(|entries| entries.get(key))
            .cloned()
        else {
            continue;
        };
        let _ = event.set(target, value);
    }
    true
}

/// Read the map, the keys and their targets off the script, or decline.
///
/// Whole-script: the binding, then nothing but guarded copies out of that one
/// local. A block that writes anything else declines the WHOLE parse rather
/// than the one block, because a matcher that claims the guard and skips a
/// statement writes a document the vendor never wrote -- and one that is
/// present and half right reads as a source needing polish.
#[must_use]
pub fn parse_dotted_key_copies(script: &str) -> Option<DottedKeyCopies> {
    let rest = skip_trivia(skip_trivia(script).strip_prefix("def")?);
    let (name, binding) = rest.split_once('=')?;
    let local = identifier(name)?;
    let (path, rest) = binding.split_once(';')?;
    let map = ctx_path(path)?;

    let mut copies: Vec<(String, String)> = Vec::new();
    let mut rest = skip_trivia(rest);
    while !rest.is_empty() {
        let after = skip_trivia(rest.strip_prefix("if")?);
        let (test, tail) = balanced(after, '(', ')')?;
        let key = guarded_key(test, local)?;
        let (block, tail) = balanced(skip_trivia(tail), '{', '}')?;
        let target = copied_to(block, local, &key)?;
        // One key written twice would make the later block's target the only
        // one that survives, which is not what two blocks mean.
        if copies.iter().any(|(held, _)| held == &key) {
            return None;
        }
        copies.push((key, target));
        rest = skip_trivia(tail);
    }

    (!copies.is_empty()).then_some(DottedKeyCopies { map, copies })
}

/// The key a `<local>.containsKey('<key>')` test asks about.
fn guarded_key(test: &str, local: &str) -> Option<String> {
    let call = skip_trivia(skip_trivia(test).strip_prefix(local)?).strip_prefix(".containsKey")?;
    let (argument, after) = balanced(skip_trivia(call), '(', ')')?;
    // The guard is the whole test: a second clause is a condition this reader
    // does not model, and running the copy anyway would apply it where the
    // vendor does not.
    if !skip_trivia(after).is_empty() {
        return None;
    }
    quoted(argument.trim())
}

/// The one `ctx.` path a guarded block copies the entry onto.
fn copied_to(block: &str, local: &str, key: &str) -> Option<String> {
    let subscripts = [format!("{local}['{key}']"), format!("{local}[\"{key}\"]")];
    let mut target = None;
    let mut rest = skip_trivia(block);
    while !rest.is_empty() {
        let (statement, tail) = rest.split_once(';')?;
        rest = skip_trivia(tail);
        // `if (ctx.<parent> == null) ctx.<parent> = new HashMap();` -- the
        // container the copy needs, which `Event::set` builds anyway. It is
        // braceless, so the split above takes it whole.
        if statement.contains("new HashMap(") || statement.contains("new ArrayList(") {
            continue;
        }
        let (assigned, read) = statement.rsplit_once('=')?;
        if !subscripts.iter().any(|form| form == read.trim()) {
            return None;
        }
        if target.replace(ctx_path(assigned)?).is_some() {
            return None;
        }
    }
    target
}

/// The text inside a leading pair of quotes, and only where that is the whole
/// of `text`.
fn quoted(text: &str) -> Option<String> {
    let quote = text.chars().next().filter(|c| matches!(c, '\'' | '"'))?;
    let inner = text.strip_prefix(quote)?.strip_suffix(quote)?;
    (!inner.is_empty() && !inner.contains(quote)).then(|| inner.to_owned())
}

/// A local's name, or `None` where the text is not one identifier.
fn identifier(text: &str) -> Option<&str> {
    let name = text.trim();
    (!name.is_empty()
        && !name.starts_with(|c: char| c.is_ascii_digit())
        && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
    .then_some(name)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use serde_json::json;

    use super::*;

    /// Verbatim from
    /// `pipelines/aws_bedrock_agentcore/runtime_application_logs/default.yml`,
    /// with the YAML block fold applied.
    fn script() -> String {
        crate::common::normalise(
            r"def attrs = ctx.aws.bedrock_agentcore.resource.attributes;\n\nif (attrs.containsKey('service.name')) {\n  if (ctx.service == null) ctx.service = new HashMap();\n  ctx.service.name = attrs['service.name'];\n}\n",
        )
        .into_owned()
    }

    fn pattern() -> DottedKeyCopies {
        parse_dotted_key_copies(&script()).expect("the attribute copy is recognised")
    }

    /// The map, the key and the target all come OFF the script.
    #[test]
    fn every_part_is_read_off_the_script() {
        assert_eq!(
            pattern(),
            DottedKeyCopies {
                map: "aws.bedrock_agentcore.resource.attributes".into(),
                copies: vec![("service.name".into(), "service.name".into())],
            }
        );
    }

    /// The captured attribute map: a dotted key among three others, none of
    /// which the script names.
    #[test]
    fn the_dotted_key_is_read_out_of_the_map() {
        let mut event = Event::new(json!({ "aws": { "bedrock_agentcore": { "resource": {
            "attributes": {
                "cloud.provider": "aws",
                "service.name": "customersupport.DEFAULT",
                "cloud.platform": "aws_bedrock_agentcore",
            }
        } } } }));
        assert!(dotted_key_copies(&mut event, &pattern()));
        assert_eq!(
            event.get("service.name"),
            Some(&json!("customersupport.DEFAULT"))
        );
        assert!(!event.has("cloud.platform"));
    }

    /// A map without the key writes nothing, which is the script's own
    /// `containsKey` guard.
    #[test]
    fn a_map_without_the_key_writes_nothing() {
        for attributes in [json!({ "cloud.provider": "aws" }), json!({})] {
            let mut event = Event::new(json!({ "aws": { "bedrock_agentcore": { "resource": {
                "attributes": attributes
            } } } }));
            assert!(dotted_key_copies(&mut event, &pattern()));
            assert!(!event.has("service.name"));
        }
    }

    /// The map is read as a MAP. A document that nests the key as a path
    /// instead holds a different thing, and taking it would write a value the
    /// vendor's own script cannot see.
    #[test]
    fn a_nested_path_of_the_same_name_is_not_the_entry() {
        let mut event = Event::new(json!({ "aws": { "bedrock_agentcore": { "resource": {
            "attributes": { "service": { "name": "nested" } }
        } } } }));
        assert!(dotted_key_copies(&mut event, &pattern()));
        assert!(!event.has("service.name"));
    }

    /// One statement the reader cannot place declines the WHOLE script.
    #[test]
    fn one_unreadable_statement_declines_the_whole_script() {
        for broken in [
            // A copy out of some other map.
            script().replace("= attrs['service.name']", "= ctx.other"),
            // A second clause in the guard, which is a condition this does not
            // model.
            script().replace(
                "if (attrs.containsKey('service.name'))",
                "if (attrs.containsKey('service.name') && ctx.a == null)",
            ),
            // A key the guard and the read disagree on.
            script().replace("= attrs['service.name']", "= attrs['cloud.provider']"),
            // A write the guard does not cover.
            script().replace(
                "ctx.service.name = attrs['service.name'];",
                "ctx.service.name = attrs['service.name']; ctx.a = 1;",
            ),
            // The map bound to something that is not a ctx path.
            script().replace("= ctx.aws.bedrock_agentcore.resource.attributes;", "= [:];"),
        ] {
            assert_ne!(broken, script(), "each replacement has to bite");
            assert!(
                parse_dotted_key_copies(&broken).is_none(),
                "read a script it should have declined: {broken}"
            );
        }
    }

    /// The other two streams ship the same script against their own map, so
    /// the parse has to follow the binding rather than a fixed path.
    #[test]
    fn the_gateway_and_memory_streams_read_their_own_maps() {
        for stream in ["gateway", "memory"] {
            let script = script().replace(
                "ctx.aws.bedrock_agentcore.resource",
                &format!("ctx.aws.bedrock_agentcore.{stream}.resource"),
            );
            let pattern = parse_dotted_key_copies(&script).expect("recognised");
            assert_eq!(
                pattern.map,
                format!("aws.bedrock_agentcore.{stream}.resource.attributes")
            );
        }
    }

    /// The dispatch reaches this matcher, and nothing else is bound beside it.
    #[test]
    fn the_ladder_dispatches_the_script_here() {
        let bound = crate::common::known_patterns(&script());
        assert_eq!(
            bound.len(),
            1,
            "the ladder bound {bound:?} instead of this matcher alone"
        );
    }

    /// The production path. A generated call site hands `PainlessPlan` the
    /// script with its newlines ESCAPED, so a matcher that works only on real
    /// newlines passes its own tests and does nothing in the service.
    #[test]
    fn the_escaped_call_site_literal_runs() {
        let _guard = crate::stats::serialised();
        let plan = crate::plan::PainlessPlan::new(
            r"def attrs = ctx.aws.bedrock_agentcore.resource.attributes;\n\nif (attrs.containsKey('service.name')) {\n  if (ctx.service == null) ctx.service = new HashMap();\n  ctx.service.name = attrs['service.name'];\n}\n",
        );
        assert!(plan.matches(), "the call site's own literal binds nothing");

        let mut event = Event::new(json!({ "aws": { "bedrock_agentcore": { "resource": {
            "attributes": { "service.name": "ordermanager-gw-abc12def34" }
        } } } }));
        crate::plan::painless_exec_plan(&mut event, &plan).expect("the plan runs");
        assert_eq!(
            event.get("service.name"),
            Some(&json!("ordermanager-gw-abc12def34"))
        );
    }
}
