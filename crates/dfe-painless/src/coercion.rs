// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! A value that may arrive as text or as a number, written back as a number.
//!
//! Qualys reports its WAS findings from an XML back end, so every id, code and
//! count reaches the pipeline either as the JSON number the API sent or as the
//! string the XML carried. The integration coerces each one before anything
//! downstream reads it, and writes the coercion longhand every time:
//! `instanceof String` picks `Long.parseLong`, and everything else takes a
//! `(long)` cast.
//!
//! Three placements of that one coercion, which is why they are one matcher and
//! not three:
//!
//! - a SCALAR, read from one path and written to another (often the same one);
//! - a MEMBER of every item of a list, coerced in place while the members are
//!   collected into a list of their own;
//! - every ELEMENT of a list, collected back as its string form.
//!
//! Unclaimed it costs `qualys_was` every one of its fifteen events:
//! `vulnerability.id`, `detection_score`, `times_detected`, `web_app.tags`,
//! `wasc_references` and `owasp_references` are all written by one of the three.
//!
//! **Where Painless would have THROWN, this writes nothing.** `Long.parseLong`
//! over text that is not a number, and a `(long)` cast over a boolean, a
//! container or a null, all raise -- and every call site in the tree wraps the
//! script in the vendor's own `on_failure` handler, which is what the document
//! is left with. Writing a substitute instead would put a number Elasticsearch
//! never emitted into the field.

use serde_json::{Map, Value};

use dfe_core::Event;

/// One coercion, in the placement its script spells.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LongCoercion {
    /// `if (ctx.<source> instanceof String) { ctx.<target> = Long.parseLong(ctx.<source>); }`
    /// `else { ctx.<target> = (long)ctx.<source>; }`
    Scalar {
        /// The path read, whichever type it holds.
        source: String,
        /// The path written. The same as `source` where the vendor coerces in
        /// place, a sibling where it also moves the value.
        target: String,
    },
    /// A member lifted out of every item of a list, one of its own members
    /// coerced on the way, and the lifted members collected into `target`.
    ///
    /// `def l = new ArrayList(); for (i in ctx.<list>) { if (i.<member>?.<key> != null)`
    /// `{ ... } l.add(i.<member>); } ctx.<target> = l;`
    CollectMembers {
        /// The list walked.
        list: String,
        /// The member of each item that is collected.
        member: String,
        /// The member of THAT which is coerced, in place.
        key: String,
        /// Where the collected members land.
        target: String,
    },
    /// Every element of a list collected as its string form, a number rendered
    /// through a `(long)` cast first so a fractional one loses its point.
    ///
    /// `ctx.<target> = new ArrayList(); for (e in ctx.<list>) { if (e instanceof String)`
    /// `{ ctx.<target>.add(e); } else { ctx.<target>.add(((long)e).toString()); } }`
    CollectStrings {
        /// The list walked.
        list: String,
        /// The list of strings built from it.
        target: String,
    },
}

/// Apply the coercion this script spells.
///
/// Always reports the script HANDLED: every placement's guard is the vendor's
/// own, and a source the guard excludes leaves the document exactly as
/// Elasticsearch leaves it.
pub fn long_coercion(event: &mut Event, pattern: &LongCoercion) -> bool {
    match pattern {
        LongCoercion::Scalar { source, target } => {
            if let Some(number) = event.get(source).and_then(as_long) {
                let _ = event.set(target, Value::from(number));
            }
        }
        LongCoercion::CollectMembers {
            list,
            member,
            key,
            target,
        } => collect_members(event, list, member, key, target),
        LongCoercion::CollectStrings { list, target } => collect_strings(event, list, target),
    }
    true
}

/// Lift `member` out of every item, coercing its `key`, and collect the lot.
///
/// An item that carries no such member contributes nothing. An item whose
/// member is present but whose `key` is absent is collected UNCHANGED -- the
/// script's `!= null` guard skips only the coercion, not the collect.
fn collect_members(event: &mut Event, list: &str, member: &str, key: &str, target: &str) {
    let Some(items) = event.get(list).and_then(Value::as_array) else {
        return;
    };
    let collected: Vec<Value> = items
        .iter()
        .filter_map(|item| item.get(member))
        .map(|held| match held {
            Value::Object(members) => Value::Object(coerce_member(members.clone(), key)),
            other => other.clone(),
        })
        .collect();
    let _ = event.set(target, Value::Array(collected));
}

/// Coerce one member in place, keeping its position.
///
/// `insert` over a key the map already holds keeps that key where it was --
/// which is what Painless does, and what a later `convert: string` over the
/// same map renders through.
fn coerce_member(mut members: Map<String, Value>, key: &str) -> Map<String, Value> {
    if let Some(number) = members.get(key).and_then(as_long) {
        members.insert(key.to_owned(), Value::from(number));
    }
    members
}

/// Collect every element as its string form.
///
/// An element that is neither text nor a number is SKIPPED: Painless's
/// `(long)` cast would have thrown on it, and there is no value to write in
/// its place.
fn collect_strings(event: &mut Event, list: &str, target: &str) {
    let Some(items) = event.get(list).and_then(Value::as_array) else {
        return;
    };
    let collected: Vec<Value> = items
        .iter()
        .filter_map(|item| match item {
            Value::String(text) => Some(Value::String(text.clone())),
            Value::Number(_) => as_long(item).map(|number| Value::from(number.to_string())),
            _ => None,
        })
        .collect();
    let _ = event.set(target, Value::Array(collected));
}

/// The long a `Long.parseLong` or a `(long)` cast yields, or `None` where
/// Painless would have thrown.
///
/// `Long.parseLong` does not trim, so neither does this. A fractional number
/// truncates towards zero, which is what the cast does.
fn as_long(value: &Value) -> Option<i64> {
    match value {
        Value::String(text) => text.parse::<i64>().ok(),
        #[allow(clippy::cast_possible_truncation)]
        Value::Number(number) => number
            .as_i64()
            .or_else(|| number.as_f64().map(|float| float as i64)),
        _ => None,
    }
}

/// Read whichever of the three placements this script spells.
#[must_use]
pub fn parse_long_coercion(script: &str) -> Option<LongCoercion> {
    parse_scalar(script)
        .or_else(|| parse_collect_members(script))
        .or_else(|| parse_collect_strings(script))
}

/// `if (ctx.<source> instanceof String) { ctx.<t> = Long.parseLong(ctx.<source>); }
/// else { ctx.<t> = (long)ctx.<source>; }`
///
/// Both arms have to write ONE target and read the GUARDED source. A script
/// whose arms disagree is doing something else -- `tenable_io`'s `acr_score`
/// falls back to `Double.parseDouble` in a `catch`, and reading it as this
/// would drop the fallback silently.
fn parse_scalar(script: &str) -> Option<LongCoercion> {
    let (head, rest) = script.split_once(" instanceof String)")?;
    // The WHOLE script, and the whole condition: an `&&`, a negation, or an
    // OUTER guard around the pair means the arms run on more than the type,
    // and running them regardless would write where the vendor does not.
    let (before, guard) = head.rsplit_once("if (")?;
    if !before.trim().is_empty() {
        return None;
    }
    let source = ctx_path(guard)?;

    let (then_arm, after) = rest.trim_start().strip_prefix('{')?.split_once('}')?;
    let (else_arm, tail) = after
        .trim_start()
        .strip_prefix("else")?
        .trim_start()
        .strip_prefix('{')?
        .split_once('}')?;
    if !tail.trim().is_empty() {
        return None;
    }

    let (target, parsed) = sole_assignment(then_arm)?;
    let (cast_target, cast) = sole_assignment(else_arm)?;
    if target != cast_target {
        return None;
    }
    let parsed_source = ctx_path(parsed.strip_prefix("Long.parseLong(")?.strip_suffix(')')?)?;
    let cast_source = ctx_path(cast.strip_prefix("(long)")?)?;
    if parsed_source != source || cast_source != source {
        return None;
    }
    Some(LongCoercion::Scalar { source, target })
}

/// `def l = new ArrayList(); for (i in ctx.<list>) { ... l.add(i.<member>); }
/// ctx.<target> = l;`
///
/// The coercion is REQUIRED, not incidental: a collect with none in it is a
/// different pattern, and claiming one here would run this matcher's member
/// lift over a script that meant something else by the same loop.
fn parse_collect_members(script: &str) -> Option<LongCoercion> {
    let (head, rest) = script.split_once(" = new ArrayList()")?;
    let accumulator = local_name(head)?;

    let (item, list) = loop_over_ctx(rest)?;

    // `<accumulator>.add(<item>.<member>)` -- the member lifted out of each item.
    let added = script
        .split_once(&format!("{accumulator}.add("))?
        .1
        .split(')')
        .next()?
        .trim();
    let member = clean(added.strip_prefix(&format!("{item}."))?);
    if member.is_empty() || !is_path(&member) {
        return None;
    }

    // `Long.parseLong(<item>.<member>.<key>)`, with the `(long)` arm beside it.
    let prefix = format!("{item}.{member}.");
    let key = script
        .split_once(&format!("Long.parseLong({prefix}"))?
        .1
        .split(')')
        .next()?
        .trim()
        .to_owned();
    if key.is_empty() || key.contains('.') || !is_path(&key) {
        return None;
    }
    if !script.contains(&format!("(long){prefix}{key}")) {
        return None;
    }

    // `ctx.<target> = <accumulator>;` -- what the loop was building.
    let (assigned, _) = script.rsplit_once(&format!(" = {accumulator};"))?;
    let target = trailing_ctx_path(assigned)?;
    Some(LongCoercion::CollectMembers {
        list,
        member,
        key,
        target,
    })
}

/// `ctx.<target> = new ArrayList(); for (e in ctx.<list>) { if (e instanceof String)
/// { ctx.<target>.add(e); } else { ctx.<target>.add(((long)e).toString()); } }`
///
/// The accumulator is a CTX PATH here rather than a local, which is what tells
/// this apart from the member collect above.
fn parse_collect_strings(script: &str) -> Option<LongCoercion> {
    let (head, rest) = script.split_once(" = new ArrayList()")?;
    let target = trailing_ctx_path(head)?;

    let (item, list) = loop_over_ctx(rest)?;

    // Both arms, or the loop is not the string collect: the element goes in as
    // itself when it is text and through the cast when it is not.
    let add = format!("ctx.{target}.add(");
    if !rest.contains(&format!("{item} instanceof String)"))
        || !rest.contains(&format!("{add}{item})"))
        || !rest.contains(&format!("{add}((long){item}).toString())"))
    {
        return None;
    }
    Some(LongCoercion::CollectStrings { list, target })
}

/// The element variable and the ctx list of a `for (<item> in ctx.<list>)`.
fn loop_over_ctx(script: &str) -> Option<(String, String)> {
    let (head, body) = script.split_once(" in ctx.")?;
    let item = head.rsplit_once("for (")?.1.trim().to_owned();
    if item.is_empty() || !is_path(&item) {
        return None;
    }
    let list = clean(body.split(')').next()?);
    if list.is_empty() || !is_path(&list) {
        return None;
    }
    Some((item, list))
}

/// The one `ctx.<path> = <expression>` a branch arm holds, or `None` where it
/// holds none or more than one.
fn sole_assignment(arm: &str) -> Option<(String, String)> {
    let mut statements = arm.split(';').map(str::trim).filter(|s| !s.is_empty());
    let statement = statements.next()?;
    if statements.next().is_some() {
        return None;
    }
    let (lhs, rhs) = statement.split_once('=')?;
    let target = ctx_path(lhs)?;
    Some((target, rhs.trim().to_owned()))
}

/// The local a `def <name> =` binds.
fn local_name(head: &str) -> Option<String> {
    let name = head
        .rsplit(['\n', ';', '{', '}'])
        .next()?
        .trim()
        .strip_prefix("def ")?
        .trim()
        .to_owned();
    (!name.is_empty() && is_path(&name)).then_some(name)
}

/// The dotted path a fragment IS, when the fragment is nothing but that path
/// prefixed `ctx.`.
///
/// Anchored at BOTH ends: a search for the last `ctx.` would read the tail of
/// a compound condition as the whole of it, and the arms would then be checked
/// against a source the guard never named.
fn ctx_path(fragment: &str) -> Option<String> {
    let path = clean(fragment.trim().strip_prefix("ctx.")?);
    (!path.is_empty() && is_path(&path)).then_some(path)
}

/// The dotted path a longer fragment ENDS with, read back to its own `ctx.`.
///
/// For a whole statement rather than one term. Still anchored at the end: the
/// path has to run to the fragment's last character, so a compound expression
/// declines rather than yielding its tail.
fn trailing_ctx_path(fragment: &str) -> Option<String> {
    let tail = fragment.trim_end();
    ctx_path(&tail[tail.rfind("ctx.")?..])
}

/// Whether every character could belong to a field path.
fn is_path(term: &str) -> bool {
    term.chars()
        .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@'))
}

/// A dotted path with its null-safe navigation gone.
fn clean(term: &str) -> String {
    term.trim().replace("?.", ".").trim().to_owned()
}

/// A run of fields decoded in place through `Long.decode`.
///
/// snort's grok captures five packet-header fields with `%{BASE16NUM}`, so each
/// arrives as text in whichever base the sensor printed, and the pipeline
/// decodes them all in one script:
///
/// ```painless
/// if (ctx.snort?.ip?.tos != null && ctx.snort.ip.tos instanceof String) {
///     ctx.snort.ip.tos = Long.decode(ctx.snort.ip.tos);
/// } ...
/// ```
///
/// Unclaimed it is the whole source: `snort` scores 0 of 15 events on these five
/// fields alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedFields {
    /// Every path the script decodes, in the order it writes them.
    paths: Vec<String>,
}

/// Read the run of decodes, or decline it.
///
/// Every arm has to guard, decode and write the SAME path: a script decoding
/// one field into another is a different one, and running it would move a value.
#[must_use]
pub fn parse_decoded_fields(script: &str) -> Option<DecodedFields> {
    let mut paths = Vec::new();
    for block in script.split("if (").skip(1) {
        let (guard, body) = block.split_once(')')?;
        let (present, typed) = guard.split_once("&&")?;
        let path = clean(
            present
                .trim()
                .strip_suffix("!= null")?
                .strip_prefix("ctx")?,
        )
        .trim_start_matches('.')
        .to_owned();
        if path.is_empty() || !is_path(&path) {
            return None;
        }
        // The type guard and the write both name the field the guard tested.
        if clean(typed).trim() != format!("ctx.{path} instanceof String") {
            return None;
        }
        let written = body.split_once('{')?.1.split_once('}')?.0;
        if clean(written).trim().trim_end_matches(';').trim()
            != format!("ctx.{path} = Long.decode(ctx.{path})")
        {
            return None;
        }
        paths.push(path);
    }
    (!paths.is_empty()).then_some(DecodedFields { paths })
}

/// Decode each field in place, leaving anything that is not text alone.
///
/// Java's `Long.decode` reads a leading `0x`, `0X` or `#` as hexadecimal, a
/// leading `0` as octal and everything else as decimal, and takes a sign before
/// the prefix. Text it cannot read THROWS, so the field is left as it stands --
/// the call site carries the vendor's own `on_failure` handler, and a substitute
/// would be a number Elasticsearch never emitted.
#[must_use]
pub fn decoded_fields(event: &mut Event, pattern: &DecodedFields) -> bool {
    for path in &pattern.paths {
        let Some(text) = event.get_str(path) else {
            continue;
        };
        if let Some(number) = java_decode(text) {
            let _ = event.set(path, Value::from(number));
        }
    }
    true
}

/// Java's `Long.decode`, which is not `parse` with a base.
fn java_decode(text: &str) -> Option<i64> {
    let text = text.trim();
    let (negative, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text.strip_prefix('+').unwrap_or(text)),
    };

    let (radix, digits) = if let Some(rest) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
        .or_else(|| digits.strip_prefix('#'))
    {
        (16, rest)
    } else if digits.len() > 1 && digits.starts_with('0') {
        (8, &digits[1..])
    } else {
        (10, digits)
    };

    let value = i64::from_str_radix(digits, radix).ok()?;
    Some(if negative { -value } else { value })
}

#[cfg(test)]
// The script constants are quoted verbatim from generated call sites, which
// spell them `r#"..."#`. Keeping them character-identical is what lets a script
// be copied straight from a module into a test.
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::needless_raw_string_hashes,
    clippy::unreadable_literal
)]
mod tests {
    use super::*;
    use crate::common::normalise;
    use serde_json::json;

    /// Verbatim from the call sites in
    /// `crates/dfe-transforms/src/filebeat/qualys_was_vulnerability/default.rs`.
    ///
    /// Written in the ESCAPED form the tree stores, and normalised here: a
    /// stored script arrives as ONE line with its newlines escaped, so a test
    /// written with real newlines passes while production still fails.
    const SCALAR: &str = r#"if (ctx.json.detection.detectionScore instanceof String) {\n  ctx.qualys_was.vulnerability.detection_score = Long.parseLong(ctx.json.detection.detectionScore);\n} else {\n  ctx.qualys_was.vulnerability.detection_score = (long)ctx.json.detection.detectionScore;\n}\n"#;

    const SCALAR_IN_PLACE: &str = r#"if (ctx.qualys_was.vulnerability.id instanceof String) {\n  ctx.qualys_was.vulnerability.id = Long.parseLong(ctx.qualys_was.vulnerability.id);\n} else {\n  ctx.qualys_was.vulnerability.id = (long)ctx.qualys_was.vulnerability.id;\n}\n"#;

    const COLLECT_MEMBERS: &str = r#"def tagsList = new ArrayList(); for (tag in ctx.json.detection.webApp.tags.list) {\n  if (tag.Tag?.id != null) {\n    if (tag.Tag.id instanceof String) {\n      tag.Tag.id = Long.parseLong(tag.Tag.id);\n    } else {\n      tag.Tag.id = (long)tag.Tag.id;\n    }\n   }     \n   tagsList.add(tag.Tag);        \n} ctx.qualys_was.vulnerability.web_app.tags = tagsList;\n"#;

    const COLLECT_OWASP: &str = r#"def owaspList = new ArrayList(); for (owasp in ctx.json.detection.owasp.list) {\n  if (owasp.OWASP?.code != null) {\n    if (owasp.OWASP.code instanceof String) {\n      owasp.OWASP.code = Long.parseLong(owasp.OWASP.code);\n    } else {\n      owasp.OWASP.code = (long)owasp.OWASP.code;\n    }\n  }\n  owaspList.add(owasp.OWASP);\n} ctx.qualys_was.vulnerability.owasp_references = owaspList;\n"#;

    const COLLECT_STRINGS: &str = r#"ctx.vulnerability.id = new ArrayList(); for (cwe in ctx.json.detection.cwe.list) {\n  if (cwe instanceof String) {\n    ctx.vulnerability.id.add(cwe);\n  } else {\n    ctx.vulnerability.id.add(((long)cwe).toString());\n  } \n  \n}\n"#;

    fn parse(script: &str) -> Option<LongCoercion> {
        parse_long_coercion(&normalise(script))
    }

    #[test]
    fn the_scalar_reads_both_of_its_paths() {
        assert_eq!(
            parse(SCALAR),
            Some(LongCoercion::Scalar {
                source: "json.detection.detectionScore".into(),
                target: "qualys_was.vulnerability.detection_score".into(),
            })
        );
        assert_eq!(
            parse(SCALAR_IN_PLACE),
            Some(LongCoercion::Scalar {
                source: "qualys_was.vulnerability.id".into(),
                target: "qualys_was.vulnerability.id".into(),
            })
        );
    }

    #[test]
    fn a_string_and_a_number_both_land_as_a_number() {
        let pattern = parse(SCALAR).unwrap();

        let mut event = Event::new(json!({ "json": { "detection": { "detectionScore": 50 } } }));
        assert!(long_coercion(&mut event, &pattern));
        assert_eq!(
            event.get("qualys_was.vulnerability.detection_score"),
            Some(&json!(50))
        );

        let mut event = Event::new(json!({ "json": { "detection": { "detectionScore": "50" } } }));
        assert!(long_coercion(&mut event, &pattern));
        assert_eq!(
            event.get("qualys_was.vulnerability.detection_score"),
            Some(&json!(50))
        );
    }

    /// `Long.parseLong` throws on text that is not a number and the vendor's
    /// `on_failure` catches it, so the target stays absent rather than taking
    /// a substitute.
    #[test]
    fn text_that_is_not_a_number_writes_nothing() {
        let pattern = parse(SCALAR).unwrap();
        let mut event = Event::new(json!({ "json": { "detection": { "detectionScore": "n/a" } } }));
        assert!(long_coercion(&mut event, &pattern));
        assert!(
            event
                .get("qualys_was.vulnerability.detection_score")
                .is_none()
        );
    }

    #[test]
    fn the_member_collect_lifts_every_item_and_keeps_its_key_order() {
        let pattern = parse(COLLECT_MEMBERS).unwrap();
        assert_eq!(
            pattern,
            LongCoercion::CollectMembers {
                list: "json.detection.webApp.tags.list".into(),
                member: "Tag".into(),
                key: "id".into(),
                target: "qualys_was.vulnerability.web_app.tags".into(),
            }
        );

        let mut event = Event::new(json!({
            "json": { "detection": { "webApp": { "tags": { "list": [
                { "Tag": { "id": "12348765", "name": "Tag:1" } },
                { "Tag": { "id": 23459876, "name": "Tag:2" } }
            ] } } } }
        }));
        assert!(long_coercion(&mut event, &pattern));
        let tags = event.get("qualys_was.vulnerability.web_app.tags").unwrap();
        assert_eq!(
            tags,
            &json!([
                { "id": 12348765, "name": "Tag:1" },
                { "id": 23459876, "name": "Tag:2" }
            ])
        );
        // The coerced key keeps the position it arrived in.
        let keys: Vec<&str> = tags[0]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, ["id", "name"]);
    }

    /// The same collect, spelled over a different member and a different key,
    /// so the reader is taking them off the script rather than off qualys's
    /// `Tag`.
    #[test]
    fn the_owasp_twin_reads_its_own_member_and_key() {
        assert_eq!(
            parse(COLLECT_OWASP),
            Some(LongCoercion::CollectMembers {
                list: "json.detection.owasp.list".into(),
                member: "OWASP".into(),
                key: "code".into(),
                target: "qualys_was.vulnerability.owasp_references".into(),
            })
        );
    }

    /// The `!= null` guard skips only the COERCION. An item whose member has
    /// no such key is still collected, whole.
    #[test]
    fn an_item_without_the_coerced_key_is_still_collected() {
        let pattern = parse(COLLECT_MEMBERS).unwrap();
        let mut event = Event::new(json!({
            "json": { "detection": { "webApp": { "tags": { "list": [
                { "Tag": { "name": "Tag:1" } }
            ] } } } }
        }));
        assert!(long_coercion(&mut event, &pattern));
        assert_eq!(
            event.get("qualys_was.vulnerability.web_app.tags"),
            Some(&json!([{ "name": "Tag:1" }]))
        );
    }

    #[test]
    fn the_string_collect_renders_every_element() {
        let pattern = parse(COLLECT_STRINGS).unwrap();
        assert_eq!(
            pattern,
            LongCoercion::CollectStrings {
                list: "json.detection.cwe.list".into(),
                target: "vulnerability.id".into(),
            }
        );

        let mut event = Event::new(json!({
            "json": { "detection": { "cwe": { "list": [79, "89"] } } }
        }));
        assert!(long_coercion(&mut event, &pattern));
        assert_eq!(event.get("vulnerability.id"), Some(&json!(["79", "89"])));
    }

    /// `tenable_io` coerces the same field with a `Double.parseDouble` fallback
    /// in a `catch`, and its `else` is a second `instanceof` ladder rather than
    /// an arm. Read as this pattern it would drop both. Verbatim from
    /// `tenable_io_asset/default.rs`.
    #[test]
    fn the_tenable_score_with_its_double_fallback_is_declined() {
        let script = r#"if (ctx.json.acr_score instanceof String) {\n  try {\n    long acr_score = Long.parseLong(ctx.json.acr_score);\n    ctx.tenable_io.asset.acr_score = acr_score;\n  } catch (NumberFormatException e) {\n    double acr_score = Double.parseDouble(ctx.json.acr_score);\n    ctx.tenable_io.asset.acr_score = (long) acr_score; \n  }\n  return;\n} if (ctx.json.acr_score instanceof int || ctx.json.acr_score instanceof long) {\n  ctx.tenable_io.asset.acr_score = ctx.json.acr_score;\n}"#;
        assert_eq!(parse(script), None);
    }

    /// axonius coerces through a NAMED HELPER over a params list of fields.
    /// The coercion is the same; the placement is a function call, and there
    /// is no one source or target to read.
    #[test]
    fn a_coercion_held_in_a_helper_is_declined() {
        let script = r#"def convertToLong(def value) {\n  if (value instanceof String) {\n    return Long.parseLong(value);\n  } else if (value instanceof Number) {\n    return ((long) value).longValue();\n  } else {\n    throw new Exception('Unsupported type');\n  }\n}\nfor (def f : params.longFields) {\n  ctx.a[f] = convertToLong(ctx.a[f]);\n}"#;
        assert_eq!(parse(script), None);
    }

    /// sysdig's helper reads the same two arms into a LOCAL, then rescales it
    /// by magnitude. Both arms name the parameter rather than a ctx path, so
    /// nothing here can claim it. Verbatim from `sysdig_event/default.rs`.
    #[test]
    fn the_sysdig_epoch_helper_is_declined() {
        let script = r#"def parseDate(def rawtimestamp) {\n  long timestamp;\n  if (rawtimestamp instanceof String) {\n    timestamp = Long.parseLong(rawtimestamp);\n  } else if (rawtimestamp instanceof long) {\n    timestamp = (long) rawtimestamp;\n  }\n  return timestamp;\n} if (ctx.json?.timestamp != null) {\n  ctx.json.timestamp = parseDate(ctx.json.timestamp);\n}"#;
        assert_eq!(parse(script), None);
    }

    /// cloudflare reads the same two arms into a local `t`, then rescales it
    /// by magnitude and writes THAT. Claiming it would write the raw epoch.
    /// Verbatim from `cloudflare_logpull/default.rs`.
    #[test]
    fn the_cloudflare_epoch_arms_are_declined() {
        let script = r#"try {\n  long t;\n  if (ctx.json.EdgeStartTimestamp instanceof String) {\n    t = Long.parseLong(ctx.json.EdgeStartTimestamp);\n  } else if (ctx.json.EdgeStartTimestamp instanceof Number) {\n    t = (long)(ctx.json.EdgeStartTimestamp);\n  } else {\n    return;\n  }\n  if (t > (long)(1e18)) {\n    ctx.json.EdgeStartTimestamp = t/(long)(1e6)\n  }\n} catch (Exception e) {}"#;
        assert_eq!(parse(script), None);
    }

    /// A pair of arms that write DIFFERENT targets is not one coercion, and
    /// claiming it would drop whichever target the reader did not take.
    #[test]
    fn arms_that_disagree_on_the_target_are_declined() {
        let script = r#"if (ctx.a.b instanceof String) {\n  ctx.x = Long.parseLong(ctx.a.b);\n} else {\n  ctx.y = (long)ctx.a.b;\n}"#;
        assert_eq!(parse(script), None);
    }

    /// Arms that read a DIFFERENT field from the one the guard typed would
    /// coerce whichever the reader picked, on the other one's type.
    #[test]
    fn arms_that_read_past_the_guarded_source_are_declined() {
        let script = r#"if (ctx.a.b instanceof String) {\n  ctx.x = Long.parseLong(ctx.a.c);\n} else {\n  ctx.x = (long)ctx.a.c;\n}"#;
        assert_eq!(parse(script), None);
    }

    /// A compound guard means the arms run on more than the type.
    #[test]
    fn a_compound_guard_is_declined() {
        let script = r#"if (ctx.a.b != null && ctx.a.b instanceof String) {\n  ctx.a.b = Long.parseLong(ctx.a.b);\n} else {\n  ctx.a.b = (long)ctx.a.b;\n}"#;
        assert_eq!(parse(script), None);
    }

    /// An OUTER guard around the pair is the same thing spelled over two
    /// lines: reading only the inner one would coerce where the vendor's own
    /// condition never runs the processor at all.
    #[test]
    fn a_coercion_nested_under_another_guard_is_declined() {
        let script = r#"if (ctx.a.on != null) {\n  if (ctx.a.b instanceof String) {\n    ctx.a.b = Long.parseLong(ctx.a.b);\n  } else {\n    ctx.a.b = (long)ctx.a.b;\n  }\n}"#;
        assert_eq!(parse(script), None);
    }

    /// A statement AFTER the pair is work the runner cannot do, and claiming
    /// the script would drop it.
    #[test]
    fn a_statement_after_the_arms_is_declined() {
        let script = r#"if (ctx.a.b instanceof String) {\n  ctx.a.b = Long.parseLong(ctx.a.b);\n} else {\n  ctx.a.b = (long)ctx.a.b;\n}\nctx.a.seen = true;\n"#;
        assert_eq!(parse(script), None);
    }

    /// An arm holding a SECOND statement is doing more than the coercion, and
    /// running only the coercion would drop the rest.
    #[test]
    fn an_arm_with_a_second_statement_is_declined() {
        let script = r#"if (ctx.a.b instanceof String) {\n  ctx.a.b = Long.parseLong(ctx.a.b);\n  ctx.seen = true;\n} else {\n  ctx.a.b = (long)ctx.a.b;\n}"#;
        assert_eq!(parse(script), None);
    }

    /// A collect with no coercion in it is a different pattern. The member
    /// lift alone would still be the wrong matcher for it.
    #[test]
    fn a_collect_with_no_coercion_is_declined() {
        let script = r#"def l = new ArrayList(); for (t in ctx.a.list) {\n  l.add(t.Tag);\n} ctx.b.tags = l;\n"#;
        assert_eq!(parse(script), None);
    }

    /// checkpoint parses a cleaned-up string with no type guard at all, so the
    /// scalar reader never starts.
    #[test]
    fn a_bare_parse_with_no_type_guard_is_declined() {
        let script = r#"String packetsStr = ctx.checkpoint.packets.trim();\nctx.checkpoint.packets = Long.parseLong(packetsStr);\n"#;
        assert_eq!(parse(script), None);
    }

    /// Verbatim from `snort_log/default.rs`, in the escaped one-line form the
    /// call site holds.
    #[test]
    fn a_run_of_fields_is_decoded_in_place() {
        let script = r#"if (ctx.snort?.ip?.tos != null && ctx.snort.ip.tos instanceof String) {\n    ctx.snort.ip.tos = Long.decode(ctx.snort.ip.tos);\n} if (ctx.snort?.eth?.length != null && ctx.snort.eth.length instanceof String) {\n    ctx.snort.eth.length = Long.decode(ctx.snort.eth.length);\n} if (ctx.snort?.tcp?.ack != null && ctx.snort.tcp.ack instanceof String) {\n    ctx.snort.tcp.ack = Long.decode(ctx.snort.tcp.ack);\n} if (ctx.snort?.tcp?.seq != null && ctx.snort.tcp.seq instanceof String) {\n    ctx.snort.tcp.seq = Long.decode(ctx.snort.tcp.seq);\n} if (ctx.snort?.tcp?.window != null && ctx.snort.tcp.window instanceof String) {\n    ctx.snort.tcp.window = Long.decode(ctx.snort.tcp.window);\n}"#;
        let mut event = Event::new(serde_json::json!({ "snort": {
            "ip": { "tos": "0x0" },
            "eth": { "length": "0x3C" },
            "tcp": { "ack": "0x0", "seq": "0xF9D5A8CE", "window": "0x2000" },
        }}));

        assert!(crate::common::try_known_painless(&mut event, script));
        assert_eq!(event.get("snort.ip.tos"), Some(&serde_json::json!(0)));
        assert_eq!(event.get("snort.eth.length"), Some(&serde_json::json!(60)));
        assert_eq!(
            event.get("snort.tcp.seq"),
            Some(&serde_json::json!(4_191_529_166_i64))
        );
        assert_eq!(
            event.get("snort.tcp.window"),
            Some(&serde_json::json!(8192))
        );

        // A field already decoded is not text, so the guard skips it and the
        // value stands.
        let mut done = Event::new(serde_json::json!({ "snort": { "ip": { "tos": 16 } } }));
        assert!(crate::common::try_known_painless(&mut done, script));
        assert_eq!(done.get("snort.ip.tos"), Some(&serde_json::json!(16)));
    }

    /// Java's `decode` is not `parse` with a base: the prefix chooses it, and a
    /// bare leading zero means OCTAL.
    #[test]
    fn decode_reads_the_base_off_the_prefix() {
        assert_eq!(java_decode("0x1F"), Some(31));
        assert_eq!(java_decode("#1F"), Some(31));
        assert_eq!(java_decode("017"), Some(15));
        assert_eq!(java_decode("17"), Some(17));
        assert_eq!(java_decode("0"), Some(0));
        assert_eq!(java_decode("-0x10"), Some(-16));
        // Text it cannot read THROWS in Painless, so nothing is written.
        assert_eq!(java_decode("0x"), None);
        assert_eq!(java_decode("09"), None);
    }

    /// A decode that writes somewhere else moves a value, so it is declined.
    #[test]
    fn a_decode_into_another_field_is_declined() {
        let script = r#"if (ctx.a.x != null && ctx.a.x instanceof String) {\n    ctx.a.y = Long.decode(ctx.a.x);\n}"#;
        assert!(parse_decoded_fields(&crate::common::normalise(script)).is_none());
    }
}
