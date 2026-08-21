// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Common Painless script patterns implemented in Rust.
//!
//! The same handful of script shapes recur across the Elastic pipelines --
//! drop-empty, snake-case keys, sum both directions -- so they are written
//! once here rather than once per source. [`try_known_painless`] matches a
//! script against them and runs the Rust equivalent.

use std::borrow::Cow;

use serde_json::{Map, Value, json};

use crate::error::Result;
use crate::event::Event;
use crate::painless_helpers::{SnakeRule, to_snake_case};

/// A script's text with its JSON escapes resolved.
///
/// The matchers below all scan this rather than the raw literal. Borrowing
/// when there is nothing to resolve is what makes [`crate::cached_script`]
/// worth having: the macro resolves the escapes once per call site, so every
/// event after the first takes the borrow and allocates nothing.
///
/// ONE pass, not a chain of replaces. Two sequential `replace` calls got `\\"`
/// wrong -- azure's `replace("'", "\"")` came out as `replace("'", "\\"")`,
/// because the `\\` standing for a real backslash was never resolved and the
/// `\"` after it was. A script quoting a backslash is rare and the one that
/// does is unreadable to every matcher.
///
/// An escape that is not one of JSON's is passed through WHOLE: Painless
/// spells a regex literal `/\d+/`, and resolving that to `/d+/` would be a
/// different script.
pub fn normalise(script: &str) -> Cow<'_, str> {
    if !script.contains('\\') {
        return Cow::Borrowed(script);
    }

    let mut out = String::with_capacity(script.len());
    let mut chars = script.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('"') => out.push('"'),
            // A trailing backslash is not an escape at all, so it stands for
            // itself the same way `\\` does.
            Some('\\') | None => out.push('\\'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
        }
    }
    Cow::Owned(out)
}

/// A Painless script with its escapes resolved once per CALL SITE.
///
/// The script is a literal that never changes, but `painless_exec` was
/// re-resolving its escapes on every event -- two allocations over the whole
/// script text, per script, per event. Resolving at the site makes
/// [`normalise`] a borrow from then on. Same shape as [`crate::cached_grok`].
#[macro_export]
macro_rules! cached_script {
    ($script:literal $(,)?) => {{
        static SITE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        SITE.get_or_init(|| $crate::painless_common::normalise($script).into_owned())
            .as_str()
    }};
}

/// What a drop-empty script's OWN predicate says is droppable.
///
/// The shape recurs across 245 of the 351 packages with an ingest pipeline, and
/// it is not one script. Most spell the predicate
/// `v == null || v == '' || (v instanceof Map && v.size() == 0) || ...`, but 16
/// packages -- `cisco_asa` among them -- write `removeIf(v -> v == null)` and
/// mean it: an empty string stays. Applying the fullest reading to all of them
/// drops fields Elastic keeps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DropPolicy {
    /// `v == ''` appears in the predicate.
    pub empty_strings: bool,
    /// `v.size() == 0` or `v.length == 0` appears in the predicate.
    pub empty_collections: bool,
    /// The script prunes list ENTRIES as well as map values. The null-only
    /// variant's `handleList` walks without removing anything.
    pub prune_lists: bool,
    /// Values the vendor counts as empty beyond the empty string. Proofpoint
    /// reads `**` and `0` as "no value", and they are its own literals rather
    /// than anything general.
    pub sentinels: Vec<String>,
}

impl DropPolicy {
    /// The reading of a script's predicate, taken off its own text.
    #[must_use]
    pub fn read(script: &str) -> Self {
        Self {
            empty_strings: script.contains("== ''") || script.contains("== \"\""),
            empty_collections: script.contains(".size() == 0") || script.contains(".length == 0"),
            // One `removeIf` prunes the map alone; the shapes that prune both
            // spell it twice, once per collection kind.
            prune_lists: script.matches("removeIf").count() >= 2,
            sentinels: predicate_sentinels(script),
        }
    }
}

/// The non-empty literals a drop predicate compares its value against.
///
/// Only the first `if` is read -- the predicate -- so a literal from further
/// down the script cannot widen what counts as empty.
fn predicate_sentinels(script: &str) -> Vec<String> {
    let Some(predicate) = script
        .split("if (")
        .nth(1)
        .and_then(|s| s.split(')').next())
    else {
        return Vec::new();
    };
    let mut found = Vec::new();
    for term in predicate.split("||") {
        let Some((_, rest)) = term.split_once("== ") else {
            continue;
        };
        // Quoted, so the `null` keyword is not read as the string "null".
        let rest = rest.trim();
        if !rest.starts_with(['\'', '"']) {
            continue;
        }
        let literal = rest.trim_matches(['\'', '"']);
        if !literal.is_empty() && !found.iter().any(|f: &String| f == literal) {
            found.push(literal.to_string());
        }
    }
    found
}

/// Recursively drop null and empty values from the event, per `policy`.
///
/// This is the most common Painless script across all Elastic pipelines:
/// ```painless
/// boolean drop(Object o) {
///   if (o == null || o == "") return true;
///   if (o instanceof Map) { ((Map) o).values().removeIf(v -> drop(v)); return ((Map) o).size() == 0; }
///   if (o instanceof List) { ((List) o).removeIf(v -> drop(v)); return ((List) o).length == 0; }
///   return false;
/// }
/// drop(ctx);
/// ```
pub fn drop_empty_recursive(event: &mut Event, policy: &DropPolicy) {
    let inner = event.as_value_mut();
    drop_value(inner, policy);
}

fn drop_value(value: &mut Value, policy: &DropPolicy) -> bool {
    match value {
        Value::Null => true,
        Value::String(s) if s.is_empty() => policy.empty_strings,
        Value::String(s) if policy.sentinels.iter().any(|v| v == s) => true,
        Value::Object(map) => {
            let keys_to_remove: Vec<String> = map
                .iter_mut()
                .filter_map(|(k, v)| {
                    if drop_value(v, policy) {
                        Some(k.clone())
                    } else {
                        None
                    }
                })
                .collect();
            for key in keys_to_remove {
                map.remove(&key);
            }
            policy.empty_collections && map.is_empty()
        }
        Value::Array(arr) => {
            if policy.prune_lists {
                arr.retain_mut(|v| !drop_value(v, policy));
            } else {
                for item in arr.iter_mut() {
                    drop_value(item, policy);
                }
            }
            policy.empty_collections && arr.is_empty()
        }
        _ => false,
    }
}

/// Convert a Painless `keys_to_snake_case` operation.
///
/// Converts camelCase JSON object keys to `snake_case` recursively.
/// Common in Okta and other pipelines for normalising field names.
pub fn keys_to_snake_case(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let entries: Vec<(String, Value)> = map
                .iter()
                .map(|(k, v)| {
                    let snake = to_snake_case(k, SnakeRule::BeforeEveryUpper);
                    let mut v = v.clone();
                    keys_to_snake_case(&mut v);
                    (snake, v)
                })
                .collect();
            map.clear();
            for (k, v) in entries {
                map.insert(k, v);
            }
        }
        Value::Array(arr) => {
            for v in arr {
                keys_to_snake_case(v);
            }
        }
        _ => {}
    }
}

/// Extract process fields from a command line string.
///
/// Sets: `process.command_line`, process.args, process.executable
pub fn extract_process_fields(
    event: &mut Event,
    cmd_field: &str,
    target_prefix: &str,
) -> Result<()> {
    let cmd = match event.get_string(cmd_field) {
        Some(c) if !c.trim().is_empty() => c,
        _ => return Ok(()),
    };

    let trimmed = cmd.trim();
    let args: Vec<&str> = trimmed
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect();

    event.set(&format!("{target_prefix}.command_line"), json!(trimmed))?;
    event.set(&format!("{target_prefix}.args"), json!(args))?;
    if let Some(exe) = args.first() {
        event.set(&format!("{target_prefix}.executable"), json!(exe))?;
    }

    Ok(())
}

/// Convert an epoch timestamp to ISO8601 string and set on event.
///
/// Auto-detects epoch precision by magnitude (ported from dfe-loader):
/// - > 1e18 → nanoseconds
/// - > 1e15 → microseconds
/// - > 1e12 → milliseconds
/// - else   → seconds
pub fn epoch_to_timestamp(event: &mut Event, source_field: &str, target_field: &str) -> Result<()> {
    let epoch = match event.get(source_field) {
        Some(Value::Number(n)) => n.as_f64().unwrap_or(0.0),
        Some(Value::String(s)) => s.parse::<f64>().unwrap_or(0.0),
        _ => return Ok(()),
    };

    if epoch <= 0.0 {
        return Ok(());
    }

    let (secs, nanos) = if epoch > 1e18 {
        ((epoch / 1e9) as i64, ((epoch % 1e9) as u32))
    } else if epoch > 1e15 {
        ((epoch / 1e6) as i64, (((epoch % 1e6) * 1000.0) as u32))
    } else if epoch > 1e12 {
        ((epoch / 1e3) as i64, (((epoch % 1e3) * 1_000_000.0) as u32))
    } else {
        (epoch as i64, ((epoch.fract() * 1e9) as u32))
    };

    if let Some(dt) = chrono::DateTime::from_timestamp(secs, nanos) {
        // Use millisecond precision format matching Elastic convention
        event.set(
            target_field,
            json!(dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()),
        )?;
    }

    Ok(())
}

/// The ECS field a `ctx.source.X + ctx.destination.X` script totals into.
///
/// Both `bytes` and `packets` appear verbatim across the network sources.
fn sum_of_directions(script: &str) -> Option<&'static str> {
    for unit in ["bytes", "packets"] {
        let target = format!("ctx.network.{unit}");
        if script.contains(&target)
            && script.contains(&format!("ctx.source.{unit}"))
            && script.contains(&format!("ctx.destination.{unit}"))
        {
            return Some(if unit == "bytes" { "bytes" } else { "packets" });
        }
    }
    None
}

/// `network.{unit} = source.{unit} + destination.{unit}`.
///
/// Elastic's script would throw on a missing side; skipping instead is the
/// behaviour the surrounding pipeline already relies on.
///
/// The addition saturates. Both operands come off the wire, so a vendor that
/// reports a nonsense byte count must not panic a debug build or wrap to a
/// negative total in a release one.
fn try_sum_directions(event: &mut Event, unit: &str) -> bool {
    let Some(source) = event.get_i64(&format!("source.{unit}")) else {
        return true;
    };
    let Some(destination) = event.get_i64(&format!("destination.{unit}")) else {
        return true;
    };
    let total = source.saturating_add(destination);
    let _ = event.set(&format!("network.{unit}"), json!(total));
    true
}

/// `ctx.<target> = ctx.<left> + ctx.<right>`, whatever the three are called.
///
/// The directional-bytes matcher above only knows `source`/`destination` into
/// `network`, and fortinet sums `rcvddelta` and `sentdelta` into `deltabytes`.
/// Reading all three names out of the script covers both and whatever comes
/// next.
///
/// Skips when either side is absent or non-numeric: Elastic's script would
/// throw, and its `if` gates on both being a Number. The addition saturates,
/// since both operands came off the wire.
fn try_sum_of_fields(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::clean_path;

    let Some((lhs, rhs)) = script.split_once(" = ") else {
        return false;
    };
    let Some(target) = lhs.trim().rsplit("ctx.").next() else {
        return false;
    };
    let end = rhs.find([';', '\n']).unwrap_or(rhs.len());
    let Some((left, right)) = rhs[..end].split_once(" + ") else {
        return false;
    };
    let (Some(left), Some(right)) = (
        left.trim().strip_prefix("ctx."),
        right.trim().strip_prefix("ctx."),
    ) else {
        return false;
    };

    let (Some(a), Some(b)) = (
        event.get_i64(&clean_path(left)),
        event.get_i64(&clean_path(right)),
    ) else {
        return true;
    };
    let _ = event.set(&clean_path(target), json!(a.saturating_add(b)));
    true
}

/// `event.duration = <field> * 1_000_000_000`, seconds to nanoseconds.
///
/// Returns false when the field name cannot be read out of the SCRIPT: that is
/// a shape this code does not actually understand, and counting it as handled
/// would inflate the coverage figure. A field the script names but the EVENT
/// lacks is a different thing -- the script would have done nothing either.
fn try_duration_to_nanos(event: &mut Event, script: &str) -> bool {
    let Some(field) = script
        .split("Long.parseLong(ctx.")
        .nth(1)
        .and_then(|rest| rest.split(')').next())
    else {
        return false;
    };

    let seconds = event
        .get_i64(field)
        .or_else(|| event.get_str(field).and_then(|s| s.parse::<i64>().ok()));

    if let Some(seconds) = seconds {
        // A duration above ~9.2 seconds-worth of i64 nanoseconds saturates
        // rather than wrapping to a negative event.duration.
        let _ = event.set(
            "event.duration",
            json!(seconds.saturating_mul(1_000_000_000)),
        );
    }
    true
}

/// Two parallel arrays, one naming what the other holds.
///
/// Cisco's Umbrella reports every identity behind a request in one list and
/// what KIND each is in another, position for position: an AD user, a roaming
/// computer, a site. Which ECS field each kind feeds is decided by literal
/// lists in the script, and how it is written by the helper it calls -- a host
/// or user name is set only if absent, a network name is appended to a list.
/// All of it is read from the script rather than transcribed.
fn try_parallel_dispatch(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::balanced;

    let helpers = helper_targets(script);
    if helpers.is_empty() {
        return false;
    }
    let Some((variable, kinds_path, body)) = dispatch_loop(script) else {
        return false;
    };
    let Some(values_path) = ctx_path_between_markers(&body, "(ctx, ctx.", "[i]") else {
        return false;
    };

    let Some(kinds) = event.get(&kinds_path).and_then(Value::as_array).cloned() else {
        return false;
    };
    let Some(values) = event.get(&values_path).and_then(Value::as_array).cloned() else {
        return false;
    };

    // Each rule is a literal list of kinds and the helper they are handled by.
    let mut rules: Vec<(Vec<String>, String)> = Vec::new();
    let mut rest = body.as_str();
    while let Some(at) = rest.find("([") {
        let after = &rest[at + 1..];
        let Some((literals, tail)) = balanced(after, '[', ']') else {
            break;
        };
        rest = tail;
        if !tail.starts_with(&format!(".contains({variable})")) {
            continue;
        }
        let Some((block, _)) = balanced(tail.trim_start_matches(|c| c != '{'), '{', '}') else {
            break;
        };
        // The call inside the block, not the `.contains(` that opened it.
        let Some(helper) = block
            .split_once('(')
            .map(|(head, _)| head.trim().to_string())
        else {
            continue;
        };
        rules.push((quoted_members(literals), helper));
    }

    for (index, kind) in kinds.iter().enumerate() {
        let Some(kind) = kind.as_str() else { continue };
        let Some(value) = values.get(index) else {
            continue;
        };
        for (members, helper) in &rules {
            if !members.iter().any(|m| m == kind) {
                continue;
            }
            let Some((path, append)) = helpers.get(helper) else {
                continue;
            };
            if *append {
                let mut items = match event.get(path) {
                    Some(Value::Array(existing)) => existing.clone(),
                    _ => Vec::new(),
                };
                if !items.contains(value) {
                    items.push(value.clone());
                }
                let _ = event.set(path, Value::Array(items));
            } else if !event.has_value(path) {
                let _ = event.set(path, value.clone());
            }
        }
    }
    true
}

/// Each `void <name>(def ctx, def x)` helper: where it writes, and whether it
/// appends to a list rather than setting a value that is not there yet.
fn helper_targets(script: &str) -> std::collections::HashMap<String, (String, bool)> {
    use crate::painless_params::{balanced, ctx_path_before};

    let mut found = std::collections::HashMap::new();
    for segment in script.split("void ").skip(1) {
        let Some((name, rest)) = segment.split_once('(') else {
            continue;
        };
        let Some((body, _)) = balanced(rest.trim_start_matches(|c| c != '{'), '{', '}') else {
            continue;
        };
        let target = if body.contains(".add(x)") {
            ctx_path_before(body, ".add(x)").map(|path| (path, true))
        } else {
            ctx_path_before(body, "= x").map(|path| (path, false))
        };
        if let Some(target) = target {
            found.insert(name.trim().to_string(), target);
        }
    }
    found
}

/// The `for (<var> in ctx.<path>) { ... }` loop: its variable, what it walks,
/// and its body.
fn dispatch_loop(script: &str) -> Option<(String, String, String)> {
    use crate::painless_params::{balanced, clean_path};

    let at = script.rfind("for (")?;
    let rest = &script[at + "for ".len()..];
    let (header, tail) = balanced(rest, '(', ')')?;
    let (variable, walked) = header.split_once(" in ")?;
    let path = walked.trim().strip_prefix("ctx.")?;
    let (body, _) = balanced(tail.trim_start(), '{', '}')?;
    Some((
        variable.trim().to_string(),
        clean_path(path),
        body.to_string(),
    ))
}

/// The dotted `ctx.` path written between two markers.
fn ctx_path_between_markers(text: &str, open: &str, close: &str) -> Option<String> {
    use crate::painless_params::clean_path;

    let start = text.find(open)? + open.len();
    let tail = &text[start..];
    Some(clean_path(&tail[..tail.find(close)?]))
}

/// The quoted strings of a literal list.
fn quoted_members(literals: &str) -> Vec<String> {
    let mut members = Vec::new();
    let mut rest = literals;
    while let Some(open) = rest.find(['"', '\'']) {
        let quote = rest.as_bytes()[open] as char;
        let after = &rest[open + 1..];
        let Some(close) = after.find(quote) else {
            break;
        };
        members.push(after[..close].to_string());
        rest = &after[close + 1..];
    }
    members
}

/// An `hh:mm:ss` flow duration becomes a span anchored at `@timestamp`.
///
/// Cisco's ASA and FTD carry the duration of a connection in the message rather
/// than both of its ends, so the pipeline computes the missing one. Which end
/// `@timestamp` is depends on the message: a teardown is timestamped at the
/// end and the start is counted back, a start-of-flow the other way about.
/// FTD writes both readings as two branches of one script, so the branch is
/// chosen by evaluating its condition rather than by assuming a direction.
///
/// The colon form is positional, not labelled: `1:07` is a minute and seven
/// seconds, and each colon multiplies everything to its left by sixty.
fn try_flow_duration(event: &mut Event, script: &str) -> bool {
    // The field parsed is the one whose reading is scaled to nanoseconds --
    // found from the scaling, not from the first `ctx.` in the script, which
    // in FTD's version belongs to a null check several lines earlier.
    let Some(source) = script
        .find("1000000000")
        .and_then(|at| script[..at].rfind("(ctx."))
        .and_then(|at| script[at + "(ctx.".len()..].split(')').next())
    else {
        return false;
    };
    let Some(text) = event.get_as_string(source) else {
        return false;
    };
    let nanos = colon_seconds(&text).saturating_mul(1_000_000_000);
    let anchor = event.get_str("@timestamp").map(str::to_string);

    let taken = resolve_branches(event, script);
    for (path, rhs) in crate::painless_params::ctx_writes(&taken) {
        if rhs == "nanos" {
            let _ = event.set(&path, json!(nanos));
            continue;
        }
        let Some(anchor) = anchor.as_deref() else {
            continue;
        };
        // Elasticsearch renders a computed instant to milliseconds, so an end
        // derived from a whole-second duration always carries `.000`.
        let shifted = |signed: i64| {
            chrono::DateTime::parse_from_rfc3339(anchor)
                .ok()
                .map(|at| at.to_utc() + chrono::TimeDelta::nanoseconds(signed))
                .map(|at| json!(at.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()))
        };
        let value = if rhs.contains("plusNanos(") {
            shifted(nanos)
        } else if rhs.contains("minusNanos(") {
            shifted(-nanos)
        } else if bound_to_timestamp(&taken, &rhs) {
            Some(json!(anchor))
        } else {
            None
        };
        if let Some(value) = value {
            let _ = event.set(&path, value);
        }
    }
    true
}

/// Seconds from a positional `[[hh:]mm:]ss` duration.
fn colon_seconds(text: &str) -> i64 {
    let mut total: i64 = 0;
    let mut current: i64 = 0;
    for c in text.chars() {
        if let Some(digit) = c.to_digit(10) {
            current = current.saturating_mul(10).saturating_add(i64::from(digit));
        } else if c == ':' {
            total = total.saturating_add(current).saturating_mul(60);
            current = 0;
        }
    }
    total.saturating_add(current)
}

/// Is `local` a `def`/`String` bound to the event's own timestamp?
fn bound_to_timestamp(script: &str, local: &str) -> bool {
    script
        .split(';')
        .filter_map(|statement| statement.split_once('='))
        .any(|(lhs, rhs)| {
            lhs.split_whitespace().next_back() == Some(local) && rhs.contains("@timestamp")
        })
}

/// Drop the branches whose conditions do not hold, keeping the rest verbatim.
///
/// Only conditions the evaluator understands are resolved; anything else is
/// kept, because dropping a branch we could not read would silently lose the
/// writes inside it.
fn resolve_branches(event: &Event, script: &str) -> String {
    use crate::painless_params::{balanced, guard_holds};

    let mut out = String::with_capacity(script.len());
    let mut rest = script;
    while let Some(at) = rest.find("if") {
        let after = &rest[at + "if".len()..];
        let Some((test, after)) = balanced(after.trim_start(), '(', ')') else {
            out.push_str(&rest[..=at]);
            rest = &rest[at + 1..];
            continue;
        };
        let Some((block, after)) = balanced(after.trim_start(), '{', '}') else {
            out.push_str(&rest[..=at]);
            rest = &rest[at + 1..];
            continue;
        };
        out.push_str(&rest[..at]);

        let otherwise = after
            .trim_start()
            .strip_prefix("else")
            .and_then(|tail| balanced(tail.trim_start(), '{', '}'));
        let holds = guard_holds(event, test);
        if let Some((alternative, tail)) = otherwise {
            let taken = if holds { block } else { alternative };
            out.push_str(&resolve_branches(event, taken));
            rest = tail;
        } else {
            if holds {
                out.push_str(&resolve_branches(event, block));
            }
            // The block's own last statement has no terminator of its own once
            // the braces are gone, so one is added.
            out.push(';');
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// One arm of an equality ladder: the literal tested, and what it assigns.
struct LadderArm<'a> {
    literal: &'a str,
    target: String,
    value: &'a str,
}

/// An `if (x == 'a') { ctx.t = 'A' } else if (x == 'b') { ... }` ladder.
///
/// The subject is read once -- either bound to a local (`def x = ctx.a.b;`)
/// or compared inline -- and every arm assigns a string literal to a ctx
/// path. Fortinet's 11-arm IANA-number-to-transport table is the shape;
/// writing the table out by hand is how a mapping silently goes stale.
struct Ladder<'a> {
    subject: String,
    arms: Vec<LadderArm<'a>>,
}

/// Parse an equality ladder, or `None` if the script is a different shape.
fn parse_ladder(script: &str) -> Option<Ladder<'_>> {
    use crate::painless_params::clean_path;

    // The subject is whatever the FIRST `if (... == ...)` compares against.
    let first = script.find("if (")? + 4;
    let (lhs, _) = script[first..].split_once("==")?;
    let lhs = lhs.trim();

    // A local binding resolves back to the ctx path it was read from.
    let subject = match ctx_path_bound_to(script, lhs) {
        Some(path) => path,
        None => clean_path(lhs.strip_prefix("ctx.")?),
    };

    let mut arms = Vec::new();
    for segment in script.split("if (").skip(1) {
        let Some((cond, body)) = segment.split_once(')') else {
            continue;
        };
        let Some((_, rhs)) = cond.split_once("==") else {
            continue;
        };
        let (Some(literal), Some(value)) = (quoted_first(rhs), quoted_first(body)) else {
            continue;
        };
        let Some(assign) = body.find('=') else {
            continue;
        };
        let Some(target) = body[..assign]
            .trim()
            .trim_start_matches('{')
            .trim()
            .strip_prefix("ctx.")
        else {
            continue;
        };
        // Borrow the literals back out of the script rather than the owned
        // copies quoted_first returned, so an arm costs no allocation.
        let lit_at = rhs.find(&literal)?;
        let val_at = body.find(&value)?;
        arms.push(LadderArm {
            literal: &rhs[lit_at..lit_at + literal.len()],
            target: clean_path(target.trim()),
            value: &body[val_at..val_at + value.len()],
        });
    }

    (arms.len() >= 2).then_some(Ladder { subject, arms })
}

/// A version string split at its first digit -- `tls1.3` into the protocol
/// `tls` and the version `1.3`.
///
/// ```painless
/// def pat = /\d+/;
/// def tlsver = ctx.fortinet.firewall.tlsver.toLowerCase();
/// def matcher = pat.matcher(tlsver);
/// if (!matcher.find()) { return; }
/// ctx.tls.version_protocol = tlsver.substring(0, matcher.start());
/// ctx.tls.version = tlsver.substring(matcher.start(), tlsver.length());
/// if (!ctx.tls.version.contains(".")) { ctx.tls.version += ".0"; }
/// ```
///
/// Both targets are read off the `substring` assignments rather than named, so
/// the same shape over another vendor's version field lands the same way.
fn try_version_split(event: &mut Event, script: &str) -> bool {
    const HEAD: &str = ".substring(0, matcher.start())";
    const TAIL: &str = ".substring(matcher.start(),";

    let (Some((protocol_path, local)), Some((version_path, _))) = (
        ctx_target_assigned_from(script, HEAD),
        ctx_target_assigned_from(script, TAIL),
    ) else {
        return false;
    };
    let Some(source) = ctx_path_bound_to(script, &local) else {
        return false;
    };

    // The processor's own guard is `tlsver instanceof String`, so a field that
    // is absent or not a string is a no-op rather than a failure.
    let Some(raw) = event
        .get_str(strip_trailing_call(&source))
        .map(str::to_owned)
    else {
        return true;
    };
    let subject = if script.contains(".toLowerCase()") {
        raw.to_lowercase()
    } else {
        raw
    };

    // `if (!matcher.find()) { return; }` -- no digit, nothing to split, and
    // the script leaves both fields alone.
    let Some(digit) = subject.find(|c: char| c.is_ascii_digit()) else {
        return true;
    };
    let mut version = subject[digit..].to_string();
    if !version.contains('.') && script.contains(r#"+= ".0""#) {
        version.push_str(".0");
    }

    let _ = event.set(&protocol_path, subject[..digit].to_string());
    let _ = event.set(&version_path, version);
    true
}

/// A helper that splits, trims and collects several optional fields into one
/// deduplicated list.
///
/// ```painless
/// void splitTrimAdd(Set acc, String str) {
///     if (str != null && str != '') {
///         String[] parts = str.splitOnToken(';');
///         for (int i = 0; i < parts.length; i++) { acc.add(parts[i].trim()); }
///     }
/// }
/// def addressSet = new HashSet(ctx.email?.to?.address ?: []);
/// splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.ForwardAsAttachmentTo);
/// splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.ForwardTo);
/// splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.RedirectTo);
/// if (!addressSet.isEmpty()) { ctx.email.to.address = addressSet.asList(); }
/// ```
///
/// o365's forwarding rules are nearly the whole of what the runtime was still
/// skipping -- 1,483 of 1,487 scripts, this one shape.
///
/// The list keeps INSERTION order where Elastic's `HashSet` iterates by hash
/// bucket. That order carries no meaning, so `tests/compare-policy.yaml`
/// records the field as a set rather than a sequence; reproducing Java's table
/// layout would be precision nothing can check.
fn try_split_trim_collect(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::clean_path;

    let Some(separator) = script
        .split_once(".splitOnToken('")
        .and_then(|(_, rest)| rest.split_once('\'').map(|(sep, _)| sep))
    else {
        return false;
    };

    // `def <acc> = new HashSet(ctx.<seed> ?: []);`
    let Some((before, after)) = script.split_once(" = new HashSet(ctx.") else {
        return false;
    };
    let Some(accumulator) = before.split_whitespace().next_back() else {
        return false;
    };
    let seed = clean_path(after.split([' ', ')', ';']).next().unwrap_or_default());
    let Some((target, _)) = ctx_target_assigned_from(script, ".asList()") else {
        return false;
    };

    // The set is seeded from whatever the target already holds, so the script
    // adds to a list an earlier processor built rather than replacing it.
    let mut collected: Vec<String> = match event.get(&seed) {
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_owned)
            .collect(),
        Some(Value::String(one)) => vec![one.clone()],
        _ => Vec::new(),
    };

    let call = format!("({accumulator}, ctx.");
    for (at, _) in script.match_indices(&call) {
        let rest = &script[at + call.len()..];
        let Some(end) = rest.find(')') else { continue };
        let Some(text) = event.get_str(&clean_path(&rest[..end])).map(str::to_owned) else {
            continue;
        };
        // `if (str != null && str != '')` guards the WHOLE string, never the
        // parts, so `a;;b` really does collect an empty one.
        if text.is_empty() {
            continue;
        }
        for part in text.split(separator) {
            let part = part.trim();
            if !collected.iter().any(|held| held == part) {
                collected.push(part.to_string());
            }
        }
    }

    if !collected.is_empty() {
        let _ = event.set(
            &target,
            Value::Array(collected.into_iter().map(Value::String).collect()),
        );
    }
    true
}

/// One nested key collected out of every entry of a map, deduplicated.
///
/// ```painless
/// ctx.related.entity = ctx.related.entity ?: [];
/// if (ctx.azure.auditlogs.properties?.target_resources != null) {
///     for (String k : ctx.azure.auditlogs.properties.target_resources.keySet()) {
///         def resource = ctx.azure.auditlogs.properties.target_resources[k];
///         if (resource?.id != null && resource.id != '' && !ctx.related.entity.contains(resource.id)) {
///             ctx.related.entity.add(resource.id);
///         }
///     }
/// }
/// ```
///
/// The array is seeded unconditionally, so an event with no target resources
/// still gets an empty one -- that is the script's first two lines, not an
/// oversight, and a later drop-empty pass is what removes it.
fn try_collect_map_values(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::clean_path;

    let Some((head, argument)) = script.split_once(".add(") else {
        return false;
    };
    let Some(target_at) = head.rfind("ctx.") else {
        return false;
    };
    let target = clean_path(&head[target_at + "ctx.".len()..]);

    // `<binding>.id` -- the leaf is whatever is read off each entry.
    let Some(leaf) = argument
        .split(')')
        .next()
        .and_then(|arg| arg.split_once('.'))
        .map(|(_, leaf)| clean_path(leaf))
    else {
        return false;
    };

    let Some(map_path) = script
        .split_once(" : ctx.")
        .and_then(|(_, rest)| rest.split_once(".keySet()"))
        .map(|(path, _)| clean_path(path))
    else {
        return false;
    };

    let mut collected: Vec<Value> = match event.get(&target) {
        Some(Value::Array(items)) => items.clone(),
        _ => Vec::new(),
    };
    if let Some(Value::Object(entries)) = event.get(&map_path) {
        for entry in entries.values() {
            let Some(value) = entry.get(&leaf).and_then(Value::as_str) else {
                continue;
            };
            if value.is_empty() || collected.iter().any(|held| held.as_str() == Some(value)) {
                continue;
            }
            collected.push(Value::String(value.to_string()));
        }
    }
    let _ = event.set(&target, Value::Array(collected));
    true
}

/// `ctx.<path> = ctx.<path>.replace(<from>, <to>)`, guarded on the field.
///
/// azure's platform logs carry `properties` as a stringified object in
/// Python's repr -- single quotes -- so the pipeline rewrites the quotes before
/// handing it to a `json` processor. Painless's
/// `replace(CharSequence, CharSequence)` is a LITERAL replace of every
/// occurrence, not a regex.
fn try_guarded_replace(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::clean_path;

    let Some((head, arguments)) = script.split_once(".replace(") else {
        return false;
    };
    // The last bare `=` is the assignment: the guard above it is `!= null`.
    let Some(assign) = head.rfind('=').filter(|at| {
        !matches!(
            head[..*at].chars().next_back(),
            Some('!' | '=' | '<' | '>' | '+')
        )
    }) else {
        return false;
    };
    let (Some(target_at), Some(source_at)) =
        (head[..assign].rfind("ctx."), head[assign..].rfind("ctx."))
    else {
        return false;
    };
    let target = clean_path(head[target_at + "ctx.".len()..assign].trim());
    let source = clean_path(head[assign + source_at + "ctx.".len()..].trim());
    let Some((from, to)) = two_string_literals(arguments) else {
        return false;
    };

    if let Some(text) = event.get_str(&source) {
        let replaced = text.replace(&from, &to);
        let _ = event.set(&target, replaced);
    }
    true
}

/// The first two quoted literals in `text`, with their escapes resolved.
///
/// Painless takes either quote character and escapes with a backslash, so the
/// pair in `replace("'", "\"")` is a single quote and a double one.
fn two_string_literals(text: &str) -> Option<(String, String)> {
    let mut found: Vec<String> = Vec::with_capacity(2);
    let mut chars = text.chars();

    while let Some(opening) = chars.next() {
        if opening != '"' && opening != '\'' {
            continue;
        }
        let mut literal = String::new();
        loop {
            match chars.next()? {
                '\\' => match chars.next()? {
                    'n' => literal.push('\n'),
                    'r' => literal.push('\r'),
                    't' => literal.push('\t'),
                    other => literal.push(other),
                },
                c if c == opening => break,
                c => literal.push(c),
            }
        }
        found.push(literal);
        if found.len() == 2 {
            let second = found.pop()?;
            let first = found.pop()?;
            return Some((first, second));
        }
    }
    None
}

/// The ctx path assigned from `<local><marker>`, with that local's name.
fn ctx_target_assigned_from(script: &str, marker: &str) -> Option<(String, String)> {
    use crate::painless_params::clean_path;

    let head = &script[..script.find(marker)?];
    let local_at = head
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .map_or(0, |at| at + 1);
    let local = &head[local_at..];
    if local.is_empty() {
        return None;
    }
    let assigned = head[..local_at].trim_end().strip_suffix('=')?;
    let target_at = assigned.rfind("ctx.")? + "ctx.".len();
    Some((clean_path(assigned[target_at..].trim()), local.to_string()))
}

/// `a.b.toLowerCase()` as `a.b` -- a binding keeps the call it read through,
/// and the event knows nothing about a path with one on the end.
fn strip_trailing_call(path: &str) -> &str {
    match path.rfind('.') {
        Some(at) if path.ends_with("()") => &path[..at],
        _ => path,
    }
}

/// The ctx path a `def name = ctx.a.b;` binding reads, if there is one.
pub(crate) fn ctx_path_bound_to(script: &str, name: &str) -> Option<String> {
    use crate::painless_params::clean_path;

    for form in ["def ", "String ", "int ", "long "] {
        let needle = format!("{form}{name} = ctx.");
        if let Some(at) = script.find(&needle) {
            let rest = &script[at + needle.len()..];
            let end = rest.find([';', '\n']).unwrap_or(rest.len());
            return Some(clean_path(rest[..end].trim()));
        }
    }
    None
}

/// An equality ladder whose arms COLLECT into a list, then write it as a scalar
/// when one thing matched and an array when several did.
///
/// ```painless
/// def result = [];
/// if (ctx.crowdstrike.event.ConnectionDirection == "0") { result.add('egress'); }
/// else if (ctx.crowdstrike.event.ConnectionDirection == "3") {
///   result.add('egress'); result.add('ingress');
/// }
/// if (result.size() == 1) { ctx.network.direction = result[0]; }
/// else if (result.size() > 1) { ctx.network.direction = result; }
/// ```
///
/// The two-shapes-one-field ending is the part the plain ladder cannot express,
/// and `CrowdStrike`'s `network.direction` rides entirely on it -- every rename
/// of `LocalAddress` and `RemoteAddress` after it is gated on the result.
fn try_collecting_ladder(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::clean_path;

    let start = match script.find("if (ctx.") {
        Some(at) => at + "if (ctx.".len(),
        None => return false,
    };
    let Some(end) = script[start..].find("==") else {
        return false;
    };
    let subject = clean_path(script[start..start + end].trim());
    let Some(target) = collecting_target(script) else {
        return false;
    };
    let Some(value) = event.get_as_string(&subject) else {
        return true;
    };

    for segment in script.split("if (").skip(1) {
        let Some((cond, body)) = segment.split_once(") {") else {
            continue;
        };
        // Only the arms testing the subject; the size tests at the end are the
        // same shape and must not be mistaken for one.
        let Some((_, rhs)) = cond.split_once("==") else {
            continue;
        };
        if !cond.contains("ctx.") || quoted_first(rhs).as_deref() != Some(value.as_str()) {
            continue;
        }

        let collected: Vec<Value> = body
            .split(".add(")
            .skip(1)
            .filter_map(quoted_first)
            .map(Value::String)
            .collect();
        let target = clean_path(&target);
        match collected.len() {
            0 => {}
            1 => {
                let _ = event.set(&target, collected[0].clone());
            }
            _ => {
                let _ = event.set(&target, Value::Array(collected));
            }
        }
        return true;
    }
    true
}

/// The ctx path a collecting ladder writes its result to.
///
/// Read off the assignment FROM the accumulator, not off the size test: the
/// script's first `.size()` guards `ctx.network = ctx.network ?: [:]`, so
/// looking there names the parent rather than the field.
fn collecting_target(script: &str) -> Option<String> {
    let name = script
        .split_once("def ")
        .and_then(|(_, rest)| rest.split_once(" = ["))
        .map(|(name, _)| name.trim().to_string())?;
    crate::painless_params::ctx_path_before(script, &format!(" = {name};"))
}

/// Build a string out of ctx fields and literals, with an all-empty fallback.
///
/// ```painless
/// def operation = ctx.event?.action ?: '';
/// def user = ctx.user?.id ?: '';
/// def subject = ctx.o365audit?.ExchangeMetaData?.Subject ?: ctx.email?.subject ?: '';
/// if (operation.isEmpty() && user.isEmpty() && subject.isEmpty()) {
///   ctx.message = "Office365 Alert";
/// } else {
///   ctx.message = "Office365 Alert: " + operation + " detected in email sent by " + user + ...;
/// }
/// ```
///
/// A `?:` chain takes the first field that is present and non-empty, which is
/// not the same as the first that EXISTS -- o365 writes an empty subject and
/// still expects the empty branch of the chain to fall through.
fn try_concat_message(event: &mut Event, script: &str) -> bool {
    let bindings = coalesce_bindings(event, script);
    let Some((target, fallback, template)) = concat_assignments(script) else {
        return false;
    };

    let Some(built) = expand_concat(&template, &bindings) else {
        return false;
    };
    let all_empty = !bindings.is_empty() && bindings.values().all(String::is_empty);

    let _ = event.set(
        &target,
        Value::String(if all_empty { fallback } else { built }),
    );
    true
}

/// Every `def <name> = ctx.<a> ?: ctx.<b> ?: '';` in the script, resolved
/// against the event.
fn coalesce_bindings(event: &Event, script: &str) -> std::collections::BTreeMap<String, String> {
    use crate::painless_params::clean_path;

    let mut bindings = std::collections::BTreeMap::new();
    for segment in script.split("def ").skip(1) {
        let Some((name, rhs)) = segment.split_once(" = ") else {
            continue;
        };
        let name = name.trim();
        if name.is_empty() || !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            continue;
        }
        let end = rhs.find([';', '\n']).unwrap_or(rhs.len());
        let rhs = &rhs[..end];
        if !rhs.contains("?:") {
            continue;
        }

        let mut resolved = String::new();
        for alternative in rhs.split("?:") {
            let alternative = alternative.trim();
            let Some(path) = alternative.strip_prefix("ctx.") else {
                continue;
            };
            if let Some(value) = event.get_as_string(&clean_path(path))
                && !value.is_empty()
            {
                resolved = value;
                break;
            }
        }
        bindings.insert(name.to_string(), resolved);
    }
    bindings
}

/// The two assignments to one target: the all-empty literal and the template.
fn concat_assignments(script: &str) -> Option<(String, String, String)> {
    use crate::painless_params::clean_path;

    let mut target = None;
    let mut fallback = None;
    let mut template = None;

    for segment in script.split("ctx.").skip(1) {
        let Some((path, rhs)) = segment.split_once(" = ") else {
            continue;
        };
        if !path
            .chars()
            .all(|c| c.is_alphanumeric() || ".?_".contains(c))
        {
            continue;
        }
        let end = rhs.find([';', '\n']).unwrap_or(rhs.len());
        let rhs = rhs[..end].trim();

        if rhs.contains(" + ") {
            target = Some(clean_path(path));
            template = Some(rhs.to_string());
        } else if let Some(literal) = quoted_first(rhs)
            && rhs.starts_with(['"', '\''])
        {
            target = target.or_else(|| Some(clean_path(path)));
            fallback = Some(literal);
        }
    }

    Some((target?, fallback?, template?))
}

/// Evaluate a `"lit" + name + "lit"` chain against the resolved bindings.
///
/// Anything in it that is neither a literal nor a binding means the script does
/// more than this models, so the whole match is abandoned.
fn expand_concat(
    template: &str,
    bindings: &std::collections::BTreeMap<String, String>,
) -> Option<String> {
    let mut built = String::new();
    for token in template.split(" + ") {
        let token = token.trim();
        if token.starts_with(['"', '\'']) {
            built.push_str(&quoted_first(token)?);
        } else {
            built.push_str(bindings.get(token)?);
        }
    }
    Some(built)
}

/// Swap two ctx subtrees, keeping named keys on the side they belong to.
///
/// ```painless
/// def tmp = ctx.source;
/// ctx.source = ctx.destination;
/// if (ctx.source == null) { ctx.source = [:]; }
/// if (tmp?.user != null) { ctx.source.user = tmp.user; tmp.remove("user"); }
/// ctx.destination = tmp;
/// ```
///
/// fortinet's VPN logs are back to front by ECS's reckoning -- `remip` is the
/// client and `locip` the firewall, so the pipeline renames them the obvious way
/// and then swaps the whole objects. `user` stays with the source, because it
/// describes the person rather than the address.
///
/// A side that ends up with nothing is REMOVED rather than written as null:
/// that is what the captured Elasticsearch output shows for a VPN event
/// carrying only `remip`.
fn try_swap_subtrees(event: &mut Event, script: &str) -> bool {
    let Some((local, first)) = binding_of(script) else {
        return false;
    };
    let Some(second) = assigned_from_ctx(script, &first) else {
        return false;
    };
    // The third leg is what makes it a swap rather than a copy.
    if !script.contains(&format!("ctx.{second} = {local};")) {
        return false;
    }

    let was_first = event.get(&first).cloned();
    let was_second = event.get(&second).cloned();

    let mut new_first = was_second.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
    let mut new_second = was_first.unwrap_or(Value::Null);

    for key in kept_keys(script, &local) {
        let Some(moved) = new_second.get(&key).cloned() else {
            continue;
        };
        if let Some(map) = new_first.as_object_mut() {
            map.insert(key.clone(), moved);
        }
        if let Some(map) = new_second.as_object_mut() {
            map.remove(&key);
        }
    }

    write_or_remove(event, &first, new_first);
    write_or_remove(event, &second, new_second);
    true
}

/// Write `value`, or remove the path when there is nothing left to write.
fn write_or_remove(event: &mut Event, path: &str, value: Value) {
    let empty = match &value {
        Value::Null => true,
        Value::Object(map) => map.is_empty(),
        _ => false,
    };
    if empty {
        event.remove(path);
    } else {
        let _ = event.set(path, value);
    }
}

/// The `def <local> = ctx.<path>;` a script opens with.
fn binding_of(script: &str) -> Option<(String, String)> {
    use crate::painless_params::clean_path;

    let start = script.find("def ")? + "def ".len();
    let (name, rest) = script[start..].split_once(" = ctx.")?;
    let end = rest.find([';', '\n'])?;
    Some((name.trim().to_string(), clean_path(&rest[..end])))
}

/// The `ctx.<from> = ctx.<to>;` assignment, as `<to>`.
fn assigned_from_ctx(script: &str, from: &str) -> Option<String> {
    use crate::painless_params::clean_path;

    let needle = format!("ctx.{from} = ctx.");
    let start = script.find(&needle)? + needle.len();
    let rest = &script[start..];
    let end = rest.find([';', '\n'])?;
    Some(clean_path(&rest[..end]))
}

/// Every key the script moves off the local and onto the other side, from its
/// `tmp.remove("<key>")` calls.
fn kept_keys(script: &str, local: &str) -> Vec<String> {
    let needle = format!("{local}.remove(");
    let mut keys = Vec::new();
    let mut at = 0;
    while let Some(found) = script[at..].find(&needle) {
        at += found + needle.len();
        if let Some(key) = quoted_first(&script[at..]) {
            keys.push(key);
        }
    }
    keys
}

/// `if (x.equalsIgnoreCase('low') || x.equalsIgnoreCase('info')) { ctx.t = 21 }`
/// `else if (x.equalsIgnoreCase('medium')) { ctx.t = 47 } ...`
///
/// Two things the `==` ladder cannot read: an arm matching several literals,
/// and a numeric right-hand side. `CrowdStrike`'s `SeverityName` mapping is
/// both, and severity ladders are written this way across the vendor pipelines.
///
/// Only the FIRST matching arm fires, which is what an `else if` chain does.
fn try_case_insensitive_ladder(event: &mut Event, script: &str) -> bool {
    let Some(first) = script.find("if (") else {
        return false;
    };
    let Some((var, _)) = script[first + "if (".len()..].split_once('.') else {
        return false;
    };
    let Some(subject) = ctx_path_bound_to(script, var.trim()) else {
        return false;
    };
    // Absent is not a miss: every one of these scripts is gated on the field
    // being a String, so it never runs without one.
    let Some(value) = event.get_str(&subject).map(str::to_lowercase) else {
        return true;
    };

    for segment in script.split("if (").skip(1) {
        let Some((cond, body)) = segment.split_once(") {") else {
            continue;
        };
        let matched = cond
            .split(".equalsIgnoreCase(")
            .skip(1)
            .filter_map(quoted_first)
            .any(|literal| literal.to_lowercase() == value);
        if !matched {
            continue;
        }

        let Some((lhs, rhs)) = body.split(';').next().and_then(|s| s.split_once('=')) else {
            continue;
        };
        let Some(target) = lhs.trim().strip_prefix("ctx.") else {
            continue;
        };
        let Some(assigned) = painless_literal(rhs.trim()) else {
            continue;
        };
        let _ = event.set(&crate::painless_params::clean_path(target), assigned);
        return true;
    }
    true
}

/// A Painless literal as the JSON value it stands for.
///
/// A trailing `L` is Painless's long suffix and is not part of the number.
fn painless_literal(text: &str) -> Option<Value> {
    if let Some(quoted) = quoted_first(text) {
        return Some(Value::String(quoted));
    }
    let text = text.trim().trim_end_matches(['L', 'l']);
    if let Ok(int) = text.parse::<i64>() {
        return Some(json!(int));
    }
    if let Ok(float) = text.parse::<f64>() {
        return Some(json!(float));
    }
    match text {
        "true" => Some(Value::Bool(true)),
        "false" => Some(Value::Bool(false)),
        _ => None,
    }
}

/// Run an equality ladder: look the subject up, assign the matching arm.
fn try_ladder(event: &mut Event, ladder: &Ladder<'_>) -> bool {
    let subject = event
        .get_str(&ladder.subject)
        .map(String::from)
        .or_else(|| event.get_i64(&ladder.subject).map(|n| n.to_string()));

    let Some(subject) = subject else {
        return true;
    };
    if let Some(arm) = ladder.arms.iter().find(|a| a.literal == subject) {
        let _ = event.set(&arm.target, json!(arm.value));
    }
    true
}

/// The source and destination arrays of an append-if-absent script.
///
/// The shape is `for (x in ctx.A) { if (!ctx.B.contains(x)) ctx.B.add(x) }`,
/// which the network sources use to fold resolved addresses into
/// `related.ip`.
fn append_unique_fields(script: &str) -> Option<(&'static str, &'static str)> {
    let appends_uniquely = script.contains(".contains(") && script.contains(".add(");
    if !appends_uniquely {
        return None;
    }
    if script.contains("ctx.dns?.resolved_ip") && script.contains("ctx.related.ip") {
        return Some(("dns.resolved_ip", "related.ip"));
    }
    None
}

/// Re-key an array of maps into an object indexed by position.
///
/// Azure writes this out longhand -- a loop that builds `target[String.valueOf(i)]`
/// and copies each field under its `snake_case` name, guarding the optional ones:
///
/// ```painless
/// if (ctx.a.targetResources != null) {
///   ctx.a.target_resources = new HashMap();
///   for (def i = 0; i < ctx.a.targetResources.length; i++) {
///     String index = String.valueOf(i);
///     ctx.a.target_resources[index] = new HashMap();
///     ctx.a.target_resources[index].display_name = ctx.a.targetResources[i].displayName;
///     ...
///   }
///   ctx.a.properties.remove('targetResources');
/// }
/// ```
///
/// Every rename in it is `to_snake_case`, and every guard is "skip a null", so
/// the loop is those two rules applied recursively.
fn try_array_to_indexed_object(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::{clean_path, ctx_path_before};

    let Some(source) = ctx_path_before(script, " != null") else {
        return false;
    };
    let Some(target) = ctx_path_before(script, " = new HashMap()") else {
        return false;
    };
    let (source, target) = (clean_path(&source), clean_path(&target));
    if source == target {
        return false;
    }

    // A missing source is not a failure -- the script's own `if` guards it.
    let Some(Value::Array(items)) = event.get(&source).cloned() else {
        return true;
    };

    let rekeyed = index_keyed(&Value::Array(items));
    event.remove(&source);
    let _ = event.set(&target, rekeyed);
    true
}

/// Quote-aware key/value split of a whole vendor payload into one map.
///
/// Fortinet ships `key=value key2="value with spaces"` as one syslog field and
/// the pipeline hand-rolls the parse, because a plain split on space would
/// break inside the quotes:
///
/// ```painless
/// def arr = splitUnquoted(ctx.syslog5424_sd, " ");
/// for (def i = 0; i < arr?.length; i++) {
///   def kv = splitUnquoted(arr[i], "=");
///   if (kv.length == 2) { map[kv[0]] = pattern.matcher(kv[1]).replaceAll(""); }
/// }
/// ctx.fortinet.firewall = map;
/// ```
///
/// A fragment without the pair separator is skipped, which is what the
/// `kv.length == 2` guard does.
fn try_split_unquoted_kv(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::clean_path;

    let calls: Vec<&str> = script.split("splitUnquoted(").skip(1).collect();
    // The definition, then the call that splits the whole payload into tokens.
    let [_, fields, ..] = calls.as_slice() else {
        return false;
    };
    let Some(source) = fields
        .strip_prefix("ctx.")
        .and_then(|rest| rest.split(',').next())
    else {
        return false;
    };
    // The per-token split is whichever helper takes the loop variable. Newer
    // pipelines call `splitOnceByToken`, older ones `splitUnquoted` again.
    let Some(pairs) = ["splitOnceByToken(", "splitUnquoted("]
        .iter()
        .find_map(|helper| script.split(helper).find(|s| s.starts_with("arr[")))
    else {
        return false;
    };
    let (Some(field_sep), Some(pair_sep)) = (quoted_first(fields), quoted_first(pairs)) else {
        return false;
    };
    let Some(target) = crate::painless_params::ctx_path_before(script, " = map") else {
        return false;
    };

    // A missing source is not a failure -- the processor's `if` guards it.
    let Some(payload) = event.get_str(&clean_path(source)).map(str::to_string) else {
        return true;
    };

    let mut map = Map::new();
    for token in split_unquoted(&payload, &field_sep) {
        let Some((key, value)) = token.split_once(pair_sep.as_str()) else {
            continue;
        };
        map.insert(
            key.trim().to_string(),
            json!(value.trim().trim_matches('"')),
        );
    }
    let _ = event.set(&clean_path(&target), Value::Object(map));
    true
}

/// Split on `separator`, ignoring any occurrence inside double quotes.
fn split_unquoted(input: &str, separator: &str) -> Vec<String> {
    let sep = separator.chars().next().unwrap_or(' ');
    let mut out = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;

    for (i, c) in input.char_indices() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == sep && !in_quotes {
            let token = input[start..i].trim();
            if !token.is_empty() {
                out.push(token.to_string());
            }
            start = i + c.len_utf8();
        }
    }
    let last = input[start..].trim();
    if !last.is_empty() && last != separator {
        out.push(last.to_string());
    }
    out
}

/// The first single- or double-quoted string in `text`.
fn quoted_first(text: &str) -> Option<String> {
    let start = text.find(['\'', '"'])?;
    let quote = text.as_bytes()[start] as char;
    let end = text[start + 1..].find(quote)?;
    Some(text[start + 1..=start + end].to_string())
}

/// Join two optional fields, falling back to whichever one is present.
///
/// ```painless
/// String reason = ctx?.a?.failure_reason;
/// String details = ctx?.a?.additional_details;
/// if (reason != null && details != null) { ctx['message'] = reason + ' (' + details + ')'; }
/// else if (reason != null) { ctx['message'] = reason; }
/// else if (details != null) { ctx['message'] = details; }
/// ```
fn try_join_optional(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::clean_path;

    let bindings = string_bindings(script);
    let [(first, first_path), (second, second_path)] = bindings.as_slice() else {
        return false;
    };
    let Some(target) = bracket_assignment_target(script) else {
        return false;
    };

    let a = event.get_str(&clean_path(first_path)).map(str::to_string);
    let b = event.get_str(&clean_path(second_path)).map(str::to_string);

    let joined = match (a, b) {
        (Some(a), Some(b)) => {
            let Some(expr) = both_present_expression(script) else {
                return false;
            };
            concat_expression(&expr, &[(first.as_str(), &a), (second.as_str(), &b)])
        }
        (Some(a), None) => a,
        (None, Some(b)) => b,
        // Neither present -- the script assigns nothing.
        (None, None) => return true,
    };

    let _ = event.set(&target, json!(joined));
    true
}

/// The `String <name> = ctx...;` bindings, in source order.
fn string_bindings(script: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for segment in script.split("String ").skip(1) {
        let Some((name, rest)) = segment.split_once(" = ctx") else {
            continue;
        };
        let path = rest.split(';').next().unwrap_or("");
        out.push((
            name.trim().to_string(),
            path.trim_start_matches(['?', '.']).to_string(),
        ));
    }
    out
}

/// The field a `ctx['<name>'] =` assignment writes to.
fn bracket_assignment_target(script: &str) -> Option<String> {
    let start = script.find("ctx['")? + "ctx['".len();
    let end = script[start..].find('\'')?;
    Some(script[start..start + end].to_string())
}

/// The right-hand side of the branch taken when BOTH fields are present.
fn both_present_expression(script: &str) -> Option<String> {
    let head = script.find("&&")? + 2;
    let start = head + assignment_offset(&script[head..])?;
    let end = script[start..].find(';')?;
    Some(script[start..start + end].trim().to_string())
}

/// The offset just past the first ASSIGNMENT `=`, skipping `!=` `==` `<=` `>=`.
fn assignment_offset(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    for (i, c) in bytes.iter().enumerate() {
        if *c != b'=' {
            continue;
        }
        let before = i.checked_sub(1).map(|p| bytes[p]);
        if matches!(before, Some(b'!' | b'=' | b'<' | b'>')) || bytes.get(i + 1) == Some(&b'=') {
            continue;
        }
        return Some(i + 1);
    }
    None
}

/// Evaluate a `a + ' (' + b + ')'` concatenation against the bound variables.
fn concat_expression(expr: &str, bound: &[(&str, &String)]) -> String {
    let mut out = String::new();
    for token in expr.split('+') {
        let token = token.trim();
        if let Some(literal) = token
            .strip_prefix('\'')
            .and_then(|t| t.strip_suffix('\''))
            .or_else(|| token.strip_prefix('"').and_then(|t| t.strip_suffix('"')))
        {
            out.push_str(literal);
        } else if let Some((_, value)) = bound.iter().find(|(name, _)| *name == token) {
            out.push_str(value);
        }
    }
    out
}

/// Collapse an array of `{key, value}` maps into one object.
///
/// `[{key: 'k1', value: 'v1'}]` becomes `{k1: 'v1'}`, in place.
fn try_key_value_pairs(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::{clean_path, ctx_path_before};

    let Some(field) = ctx_path_before(script, " = tmp") else {
        return false;
    };
    let field = clean_path(&field);

    // Already an object, or absent -- the processor's `instanceof List` guards
    // both, so there is nothing to do either way.
    let Some(Value::Array(items)) = event.get(&field).cloned() else {
        return true;
    };

    let mut out = Map::new();
    for item in &items {
        let Some(obj) = item.as_object() else {
            continue;
        };
        let Some(key) = obj.get("key").and_then(Value::as_str) else {
            continue;
        };
        out.insert(
            key.to_string(),
            obj.get("value").cloned().unwrap_or(Value::Null),
        );
    }
    let _ = event.set(&field, Value::Object(out));
    true
}

/// An array of maps as an object keyed "0", "1", ...; keys `snake_cased`,
/// nulls dropped, recursively.
fn index_keyed(value: &Value) -> Value {
    match value {
        Value::Array(items) if items.iter().any(Value::is_object) => {
            let mut out = Map::new();
            for (i, item) in items.iter().enumerate() {
                out.insert(i.to_string(), index_keyed(item));
            }
            Value::Object(out)
        }
        Value::Object(map) => {
            let mut out = Map::new();
            for (k, v) in map {
                if v.is_null() {
                    continue;
                }
                out.insert(
                    to_snake_case(k, SnakeRule::BeforeEveryUpper),
                    index_keyed(v),
                );
            }
            Value::Object(out)
        }
        other => other.clone(),
    }
}

/// Flatten a field into an array field, one entry per element.
///
/// The vendor scripts write both branches of this and pick at runtime on the
/// source's type, so this does the same rather than guessing from the text:
///
/// ```painless
/// if (ctx.crowdstrike.event.Tags instanceof List) {
///     for (tag in ctx.crowdstrike.event.Tags) {
///         if (tag instanceof Map) { ctx.tags.add(tag["Key"] + ":" + tag["ValueString"]); }
///     }
/// } else if (ctx.crowdstrike.event.Tags instanceof String) {
///     for (value in ctx.crowdstrike.event.Tags.splitOnToken(',')) { ctx.tags.add(value.trim()); }
/// }
/// ```
fn try_append_each(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::{clean_path, ctx_path_before};

    let Some(target) = ctx_path_before(script, ".add(") else {
        return false;
    };
    let Some(source) = append_source_path(script) else {
        return false;
    };
    // The map branch joins two keys; without both there is nothing to build.
    let map_keys = quoted_after(script, "tag[");
    let separator = quoted_after(script, ".splitOnToken(")
        .into_iter()
        .next()
        .unwrap_or_else(|| ",".to_string());

    // A missing source is not a failure -- the processor's `if` guards it.
    let entries: Vec<Value> = match event.get(&source) {
        Some(Value::String(s)) => s
            .split(&separator)
            .map(str::trim)
            .filter(|p| !p.is_empty())
            .map(|p| json!(p))
            .collect(),
        Some(Value::Array(items)) => {
            if map_keys.len() < 2 {
                return false;
            }
            items
                .iter()
                .filter_map(|item| {
                    let obj = item.as_object()?;
                    let k = obj.get(&map_keys[0])?.as_str()?;
                    let v = obj.get(&map_keys[1])?.as_str()?;
                    Some(json!(format!("{k}:{v}")))
                })
                .collect()
        }
        _ => return true,
    };

    let mut existing = match event.get(&clean_path(&target)) {
        Some(Value::Array(arr)) => arr.clone(),
        _ => Vec::new(),
    };
    existing.extend(entries);
    let _ = event.set(&clean_path(&target), Value::Array(existing));
    true
}

/// The field the append reads from -- the one the `instanceof` ladder tests.
fn append_source_path(script: &str) -> Option<String> {
    use crate::painless_params::{clean_path, ctx_path_before};

    ctx_path_before(script, " instanceof List")
        .or_else(|| ctx_path_before(script, " instanceof String"))
        .or_else(|| ctx_path_before(script, ".splitOnToken("))
        .map(|p| clean_path(&p))
}

/// Every single- or double-quoted string that follows an occurrence of `after`.
fn quoted_after(script: &str, after: &str) -> Vec<String> {
    let mut found = Vec::new();
    for segment in script.split(after).skip(1) {
        let mut chars = segment.char_indices();
        let Some((_, quote)) = chars.next() else {
            continue;
        };
        if quote != '\'' && quote != '"' {
            continue;
        }
        if let Some(end) = segment[1..].find(quote) {
            found.push(segment[1..=end].to_string());
        }
    }
    found
}

/// `for (def item : ctx.<table>) { if (item.<key> == ctx.<subject>) { ... } }`
/// followed by a chain of fallback assignments to the same target.
///
/// Cisco IOS's timezone map is the shape, and at 89 hits it was the single
/// largest unhandled script in the corpus. The table lives in `ctx`, not in
/// `params`, because the deployment supplies it -- so the mapping is data the
/// matcher READS, never a table transcribed into Rust.
fn try_row_lookup_with_fallback(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::clean_path;

    let Some((item, table)) = for_binding(script) else {
        return false;
    };
    // `item.<key> == ctx.<subject>` names the column and the field to match.
    let Some((key, subject)) = script
        .split_once(&format!("if ({item}."))
        .and_then(|(_, rest)| rest.split_once(')'))
        .and_then(|(cond, _)| cond.split_once("=="))
    else {
        return false;
    };
    let key = key.trim();
    let Some(subject) = subject.trim().strip_prefix("ctx.").map(clean_path) else {
        return false;
    };
    // A hit either assigns the column or, since the vendor wrapped this in a
    // function, RETURNS it. Reading only the assignment form left cisco_ios's
    // timezone chain unmatched from the first character.
    let Some(value_col) = script
        .split_once(&format!("= {item}."))
        .or_else(|| script.split_once(&format!("return {item}.")))
        .map(|(_, rest)| rest.trim_end_matches(';'))
        .and_then(|rest| rest.split([';', '\n']).next())
    else {
        return false;
    };
    // The return form assigns nothing on a hit, so there is no target to find
    // ahead of it; the fallback arms below name their own.
    let target = ctx_assignment_target_before(script, &format!("= {item}."));

    let wanted = event.get_as_string(&subject);
    if let (Some(wanted), Some(Value::Array(rows))) = (&wanted, event.get(&table)) {
        let hit = rows.iter().find_map(|row| {
            (row.get(key).and_then(Value::as_str) == Some(wanted.as_str()))
                .then(|| row.get(value_col.trim()).cloned())
                .flatten()
        });
        if let Some(value) = hit {
            if let Some(target) = &target {
                let _ = event.set(target, value);
            }
            return true;
        }
    }

    let arm_target = apply_fallbacks(event, script, target.as_deref());
    if let Some(target) = arm_target.or(target) {
        apply_tail_default(event, script, &target);
    }
    true
}

/// `for (def <item> : ctx.<table>)` -- the loop variable and what it walks.
fn for_binding(script: &str) -> Option<(String, String)> {
    use crate::painless_params::clean_path;

    let (_, rest) = script.split_once("for (def ")?;
    let (item, rest) = rest.split_once(" : ctx.")?;
    let table = rest.split([')', ' ']).next()?;
    Some((item.trim().to_string(), clean_path(table)))
}

/// The `ctx.` path assigned immediately before `marker`.
fn ctx_assignment_target_before(script: &str, marker: &str) -> Option<String> {
    use crate::painless_params::clean_path;

    let head = &script[..script.find(marker)?];
    let start = head.rfind("ctx.")? + "ctx.".len();
    Some(clean_path(head[start..].trim_end_matches([' ', '='])))
}

/// The `if (...) { ctx.<target> = ... }` tail a lookup falls back through.
///
/// Each arm assigns either another ctx field or a literal, and is guarded on
/// that field being present or on the target still being unset. Running them
/// in order is what makes `UTC` the last resort rather than the first.
fn apply_fallbacks(event: &mut Event, script: &str, target: Option<&str>) -> Option<String> {
    use crate::painless_params::clean_path;

    let mut named = None;
    for segment in script.split("if (").skip(1) {
        let Some((cond, body)) = segment.split_once(')') else {
            continue;
        };
        let Some((lhs, assigned)) = body.split_once('=') else {
            continue;
        };
        // The arm names its own destination. The lookup's target is only the
        // default, for the older form that wrote it before the loop.
        let Some(arm_target) = lhs
            .rsplit_once("ctx.")
            .map(|(_, path)| clean_path(path.trim()))
            .or_else(|| target.map(str::to_owned))
        else {
            continue;
        };
        named = Some(arm_target.clone());
        // Bounded to its own statement: the segment runs to the end of the
        // script, so an unbounded read finds the LAST literal in it rather
        // than this arm's.
        let assigned = assigned
            .split(';')
            .next()
            .unwrap_or(assigned)
            .trim()
            .trim_start_matches(['{', ' ', '\n']);

        let guard_holds = if let Some(path) = cond.trim().strip_prefix("ctx.") {
            match path.split_once("!=") {
                Some((p, _)) => event.has_value(&clean_path(p)),
                None => match path.split_once("==") {
                    Some((p, _)) => !event.has_value(&clean_path(p)),
                    None => continue,
                },
            }
        } else {
            continue;
        };
        if !guard_holds {
            continue;
        }

        if let Some(literal) = quoted_first(assigned) {
            let _ = event.set(&arm_target, Value::String(literal));
        } else if let Some(path) = assigned.strip_prefix("ctx.") {
            let source = clean_path(path.split([';', '\n', ' ']).next().unwrap_or(path));
            if let Some(value) = event.get(&source).cloned() {
                let _ = event.set(&arm_target, value);
            }
        }
    }
    named
}

/// The unguarded `ctx.<target> = '<literal>';` a lookup chain ends on.
///
/// It sits outside every `if`, so the guarded arms above never reach it, and
/// `cisco_ios`'s `event.timezone` was left unset on 45 corpus events. It applies
/// only when no `!= null` guard in the script holds, which is what the early
/// returns above it mean: a timezone parsed off the line takes the first
/// branch and returns before the default is ever reached.
fn apply_tail_default(event: &mut Event, script: &str, target: &str) {
    use crate::painless_params::clean_path;

    if event.has_value(target) {
        return;
    }

    let any_guard_holds = script
        .split("if (")
        .skip(1)
        .filter_map(|segment| segment.split_once(')'))
        .any(|(cond, _)| {
            cond.trim()
                .strip_prefix("ctx.")
                .and_then(|path| path.split_once("!="))
                .is_some_and(|(path, _)| event.has_value(&clean_path(path)))
        });
    if any_guard_holds {
        return;
    }

    let marker = format!("ctx.{target} = ");
    if let Some(at) = script.rfind(&marker)
        && let Some(literal) = quoted_first(&script[at + marker.len()..])
    {
        let _ = event.set(target, Value::String(literal));
    }
}

/// `<map>.entrySet().removeIf(entry -> entry.getValue() == "N/A" || ...)`
///
/// The params-driven form of this lives in [`crate::painless_params`]; this is
/// the one that spells its sentinels out as literals. Fortinet ORs in a key
/// test as well -- `pat.matcher(entry.getKey()).find()` over `/\W+/` -- which
/// drops every key holding a character a vendor never means as a field name.
fn try_sentinel_removal_literal(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::{clean_path, ctx_path_before};

    // The map is either named inline or bound to a local read from ctx.
    let Some(path) = ctx_path_before(script, ".entrySet().removeIf(")
        .filter(|p| !p.contains(' '))
        .or_else(|| bound_ctx_path(script))
        .map(|p| clean_path(&p))
    else {
        return false;
    };

    let sentinels = quoted_after(script, "entry.getValue() == ");
    let drops_odd_keys = script.contains("entry.getKey()") && script.contains(r"\W+");
    if sentinels.is_empty() && !drops_odd_keys {
        return false;
    }

    if let Some(Value::Object(map)) = crate::painless_params::pointer_mut(event, &path) {
        map.retain(|k, v| {
            let sentinel = v.as_str().is_some_and(|s| sentinels.iter().any(|x| x == s));
            let odd_key = drops_odd_keys && k.chars().any(|c| !c.is_alphanumeric() && c != '_');
            !sentinel && !odd_key
        });
    }
    true
}

/// `if (ctx.<src> != null) { ctx.<dst> = ctx.<src>; }`
///
/// Azure's SAML claims arrive under URI keys, so the source is written with a
/// bracket subscript rather than a dotted path; the destination is a plain
/// ECS field. Nothing is written when the source is absent, which is what
/// stops an explicit null propagating into the ECS field.
fn try_guarded_copy(event: &mut Event, script: &str) -> bool {
    let Some((cond, body)) = script.split_once("!= null") else {
        return false;
    };
    let Some(source) = painless_path(cond) else {
        return false;
    };
    let Some((target_expr, value_expr)) = body.split_once('=') else {
        return false;
    };
    let (Some(target), Some(value)) = (painless_path(target_expr), painless_path(value_expr))
    else {
        return false;
    };
    if value != source {
        return false;
    }

    if let Some(v) = event.get(&source).cloned()
        && !v.is_null()
    {
        let _ = event.set(&target, v);
    }
    true
}

/// The LAST `ctx.` path in a fragment, as a dotted path.
///
/// Painless writes a key that is not an identifier as `['a.b/c']`, and those
/// subscripts are path SEGMENTS -- a dot inside one is part of the key, not a
/// separator, so the segment is joined whole.
pub(crate) fn painless_path(fragment: &str) -> Option<String> {
    // The root is written either `ctx.a` or `ctx['a']`, sometimes in the same
    // script, so the search is for `ctx` followed by either.
    let start = fragment
        .rfind("ctx.")
        .map(|at| at + "ctx.".len())
        .or_else(|| fragment.rfind("ctx[").map(|at| at + "ctx".len()))?;
    let mut path = String::new();
    let mut chars = fragment[start..].chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '[' => {
                let quote = chars.next().filter(|q| *q == '\'' || *q == '"')?;
                if !path.is_empty() {
                    path.push('.');
                }
                for k in chars.by_ref() {
                    if k == quote {
                        break;
                    }
                    path.push(k);
                }
                // Consume the closing bracket.
                chars.next();
            }
            '?' => {}
            c if c.is_alphanumeric() || matches!(c, '_' | '.' | '-' | '@') => path.push(c),
            _ => break,
        }
    }

    let path = path.trim_end_matches('.');
    (!path.is_empty()).then(|| path.to_string())
}

/// `ctx.<path> = ctx.<path> * <literal>` -- scale a number in place.
fn try_scale_by_literal(event: &mut Event, script: &str) -> bool {
    let Some((head, factor)) = script.rsplit_once('*') else {
        return false;
    };
    let Some(factor) = factor.trim().trim_end_matches(';').parse::<i64>().ok() else {
        return false;
    };
    let Some(target) = painless_path(head.split_once('=').map_or(head, |(t, _)| t)) else {
        return false;
    };

    if let Some(n) = event.get_as_i64(&target) {
        let _ = event.set(&target, json!(n.saturating_mul(factor)));
    }
    true
}

/// The ctx path a `def <name> = ctx.<path>;` binding at the top of a script reads.
fn bound_ctx_path(script: &str) -> Option<String> {
    let at = script.find("def ")?;
    let rest = &script[at..];
    let start = rest.find("= ctx.")? + "= ctx.".len();
    let tail = &rest[start..];
    let end = tail.find([';', '\n']).unwrap_or(tail.len());
    Some(tail[..end].trim().to_string())
}

/// Append every element of `from` into `into`, skipping ones already present.
fn try_append_unique(event: &mut Event, from: &str, into: &str) -> bool {
    let Some(Value::Array(source)) = event.get(from).cloned() else {
        return true;
    };

    let mut target = match event.get(into).cloned() {
        Some(Value::Array(existing)) => existing,
        _ => Vec::new(),
    };
    for item in source {
        if !target.contains(&item) {
            target.push(item);
        }
    }
    let _ = event.set(into, Value::Array(target));
    true
}

/// Split a URL that has no scheme into its ECS components.
///
/// panw's threat pipeline carries this as a hand-written script and says why in
/// its own comment: `uri_parts` does not cope when the scheme is absent, which
/// it always is in a PAN-OS `misc` field. The script REPLACES `ctx.url`
/// wholesale, so anything already under it is dropped rather than merged.
///
/// Splitting is on the FIRST `/` and the first `?`, which is what the script
/// does -- not a URL parser's idea of either. `url.extension` comes off the
/// last `.` in the path, so a dotted directory name feeds it, deliberately.
fn try_schemeless_url(event: &mut Event) -> bool {
    let Some(original) = event.get_string("url.original") else {
        return false;
    };

    let mut url = serde_json::Map::new();
    url.insert("original".into(), Value::String(original.clone()));

    let mut domain_port = original.as_str();
    if let Some(slash) = original.find('/') {
        domain_port = &original[..slash];
        let after = &original[slash..];

        let path = match after.find('?') {
            Some(query) => {
                url.insert("query".into(), Value::String(after[query + 1..].to_owned()));
                &after[..query]
            }
            None => after,
        };
        url.insert("path".into(), Value::String(path.to_owned()));

        if let Some(dot) = path.rfind('.') {
            url.insert(
                "extension".into(),
                Value::String(path[dot + 1..].to_owned()),
            );
        }
    } else if let Some(query) = original.find('?') {
        url.insert(
            "query".into(),
            Value::String(original[query + 1..].to_owned()),
        );
        domain_port = &original[..query];
    }

    if let Some((domain, port)) = domain_port.split_once(':') {
        url.insert("domain".into(), Value::String(domain.to_owned()));
        // The script swallows a `NumberFormatException` here, so a non-numeric
        // port leaves `url.port` unset rather than failing.
        if let Ok(port) = port.parse::<i64>() {
            url.insert("port".into(), Value::Number(port.into()));
        }
    } else {
        url.insert("domain".into(), Value::String(domain_port.to_owned()));
        // Painless would throw on an absent `ctx.destination`, so writing one
        // here would invent an object Elastic never produced.
        if event.has("destination")
            && event
                .set("destination.domain", json_str(domain_port))
                .is_err()
        {
            return false;
        }
    }

    event.set("url", Value::Object(url)).is_ok()
}

/// A `&str` as a JSON string value.
fn json_str(value: &str) -> Value {
    Value::String(value.to_owned())
}

/// Decompose a syslog PRI into ECS `log.syslog.{facility,severity}.{code,name}`.
///
/// The PRI is read from wherever the script found it: `log.syslog.priority`
/// for the generic pipelines, or a vendor field such as
/// `cisco_nexus.log.priority_number`.
///
/// Only the halves the SCRIPT writes are written. A pipeline that sets
/// `severity.code` with a `set` processor and only the facility here (cisco
/// nexus) must not gain a severity from us, and none of the vendor scripts
/// derive the `name` at all -- inventing one is an extra field, not a bonus.
fn try_syslog_priority(event: &mut Event, script: &str) -> bool {
    let pri = priority_source(script)
        .and_then(|field| read_u16(event, field))
        .or_else(|| read_u16(event, "log.syslog.priority"));

    let Some(pri) = pri else {
        return true;
    };

    let (facility, severity) = crate::syslog_pri::decompose(pri);
    let names = script.contains("name");

    if writes_syslog_half(script, "facility") {
        let _ = event.set("log.syslog.facility.code", json!(facility));
        if let Some(name) = names
            .then(|| crate::syslog_pri::facility_name(facility))
            .flatten()
        {
            let _ = event.set("log.syslog.facility.name", json!(name));
        }
    }
    if writes_syslog_half(script, "severity") {
        let _ = event.set("log.syslog.severity.code", json!(severity));
        if let Some(name) = names
            .then(|| crate::syslog_pri::severity_name(severity))
            .flatten()
        {
            let _ = event.set("log.syslog.severity.name", json!(name));
        }
    }
    true
}

/// Does the script write `log.syslog.<half>`, in any of Painless's spellings?
///
/// Plain `contains("severity")` is not enough: cisco nexus reads
/// `ctx.event.severity` to compute the FACILITY, and would otherwise gain a
/// severity code the pipeline sets from its own vendor field.
fn writes_syslog_half(script: &str, half: &str) -> bool {
    [
        format!("syslog.{half}"),
        format!("syslog['{half}']"),
        format!("syslog[\"{half}\"]"),
    ]
    .iter()
    .any(|form| script.contains(form.as_str()))
}

/// The vendor field a priority script reads, when it is not the ECS one.
fn priority_source(script: &str) -> Option<&str> {
    script
        .split("ctx.")
        .find(|s| s.starts_with("cisco_nexus.log.priority_number"))
        .map(|_| "cisco_nexus.log.priority_number")
}

/// A field as a `u16`, whether it is stored as a number or a string.
fn read_u16(event: &Event, field: &str) -> Option<u16> {
    event
        .get_i64(field)
        .and_then(|n| u16::try_from(n).ok())
        .or_else(|| event.get_str(field).and_then(|s| s.parse::<u16>().ok()))
}

/// Check if a Painless script source matches a known pattern.
///
/// Returns true if the script was handled, false if it should fall through
/// to the generic `painless_exec` stub.
pub fn try_known_painless(event: &mut Event, script: &str) -> bool {
    let normalised = normalise(script);

    // Pattern: drop null and empty values recursively. Matched on the SHAPE,
    // not the helper's name -- panw spells it `dropEmptyFields`, and keying
    // on `drop(ctx)` left every emptied object behind. What counts as empty
    // comes from the script's own predicate, which is not the same everywhere.
    if normalised.contains("removeIf")
        && normalised.contains("instanceof Map")
        && normalised.contains("instanceof List")
        && normalised.contains("(ctx)")
    {
        drop_empty_recursive(event, &DropPolicy::read(&normalised));
        return true;
    }

    // Pattern: split, trim and collect several optional fields into one list.
    // Checked early: the script also spells `.add(` and `.splitOnToken(`, which
    // a later matcher reads as a different shape entirely.
    if normalised.contains("new HashSet(") && normalised.contains(".asList()") {
        return try_split_trim_collect(event, &normalised);
    }

    // Pattern: network.bytes / network.packets as the sum of both directions.
    if let Some(total) = sum_of_directions(&normalised) {
        return try_sum_directions(event, total);
    }

    // Pattern: one ctx field as the sum of two others.
    if normalised.contains(" + ctx.") && try_sum_of_fields(event, &normalised) {
        return true;
    }

    // Pattern: seconds to nanoseconds for event.duration.
    if normalised.contains("ctx.event.duration")
        && normalised.contains("Long.parseLong")
        && normalised.contains("1000000000")
    {
        return try_duration_to_nanos(event, &normalised);
    }

    // Pattern: an `hh:mm:ss` flow duration becomes a span ending at @timestamp.
    if normalised.contains("minusNanos(") && normalised.contains(".toCharArray()") {
        return try_flow_duration(event, &normalised);
    }

    // Pattern: two parallel arrays, one naming what the other holds.
    if normalised.contains("(ctx, ctx.") && normalised.contains("[i])") {
        return try_parallel_dispatch(event, &normalised);
    }

    // Pattern: build a string out of ctx fields and literals.
    if normalised.contains("?: ''")
        && normalised.contains(".isEmpty()")
        && normalised.contains("\" + ")
        && try_concat_message(event, &normalised)
    {
        return true;
    }

    // Pattern: swap two ctx subtrees, keeping named keys on one side.
    if normalised.contains("def tmp = ctx.") && try_swap_subtrees(event, &normalised) {
        return true;
    }

    // Pattern: a ladder collecting into a list, written as scalar or array.
    if normalised.contains(".add(")
        && normalised.contains(".size()")
        && normalised.contains("else if (")
        && try_collecting_ladder(event, &normalised)
    {
        return true;
    }

    // Pattern: a case-insensitive ladder mapping one field onto a literal.
    // Tried before the `==` ladder, which cannot read either the multi-literal
    // arms or the numeric right-hand sides.
    if normalised.contains(".equalsIgnoreCase(") && normalised.contains("else if (") {
        return try_case_insensitive_ladder(event, &normalised);
    }

    // Pattern: an equality ladder mapping one field onto string literals.
    if normalised.contains("else if (")
        && let Some(ladder) = parse_ladder(&normalised)
    {
        return try_ladder(event, &ladder);
    }

    // Pattern: strip sentinel values and junk keys out of a parsed map.
    if normalised.contains(".entrySet().removeIf(") && normalised.contains("entry.getValue()") {
        return try_sentinel_removal_literal(event, &normalised);
    }

    // Pattern: look a value up in a ctx-held table of rows, else fall back.
    if normalised.contains("for (def ")
        && normalised.contains(" : ctx.")
        && try_row_lookup_with_fallback(event, &normalised)
    {
        return true;
    }

    // Pattern: split a schemeless URL into its ECS components.
    if normalised.contains("domainPort") && normalised.contains("url.original") {
        return try_schemeless_url(event);
    }

    // Pattern: split a version string at its first digit.
    if normalised.contains("matcher.start()") && try_version_split(event, &normalised) {
        return true;
    }

    // Pattern: decompose a syslog PRI into ECS facility and severity.
    if normalised.contains("log.syslog") && normalised.contains("priority") {
        return try_syslog_priority(event, &normalised);
    }

    // Pattern: append one array into another, skipping duplicates.
    if let Some((from, into)) = append_unique_fields(&normalised) {
        return try_append_unique(event, from, into);
    }

    // Pattern: quote-aware KV split of a whole vendor payload.
    if normalised.contains("splitUnquoted(") {
        return try_split_unquoted_kv(event, &normalised);
    }

    // Pattern: re-key an array of maps into an object indexed by position.
    if normalised.contains("new HashMap()") && normalised.contains("String.valueOf(") {
        return try_array_to_indexed_object(event, &normalised);
    }

    // Pattern: collapse an array of `{key, value}` maps into one object.
    if normalised.contains("[item.key] = item.value") {
        return try_key_value_pairs(event, &normalised);
    }

    // Pattern: join two optional fields, each alone if the other is absent.
    if normalised.matches("String ").count() == 2 && normalised.contains("} else if (") {
        return try_join_optional(event, &normalised);
    }

    // Pattern: flatten a field into an array, either by splitting a delimited
    // string or by joining each map's two keys.
    if normalised.contains(".add(")
        && (normalised.contains(".splitOnToken(") || normalised.contains("instanceof Map"))
    {
        return try_append_each(event, &normalised);
    }

    // Pattern: keys_to_snake_case
    if normalised.contains("keys_to_snake_case") || normalised.contains("keysToSnakeCase") {
        if let Some(field) = extract_target_field(&normalised) {
            if let Some(val) = event.get(&field).cloned() {
                let mut val = val;
                keys_to_snake_case(&mut val);
                let _ = event.set(&field, val);
            }
        } else {
            // Apply to entire event
            let inner = event.as_value_mut();
            keys_to_snake_case(inner);
        }
        return true;
    }

    try_named_painless(event, &normalised)
}

/// The matchers keyed on a vendor's FIELD NAMES rather than on a Painless
/// construct, plus the two catch-alls.
///
/// Split out of [`try_known_painless`] because they are a different kind of
/// match: a shape matcher recognises what the script DOES and works for any
/// source that writes it, where these recognise whose script it is.
fn try_named_painless(event: &mut Event, normalised: &str) -> bool {
    // Pattern: CommandLine → process fields
    if normalised.contains("CommandLine") && normalised.contains("process") {
        if normalised.contains("ParentCommandLine") {
            let _ = extract_process_fields(
                event,
                "crowdstrike.event.ParentCommandLine",
                "process.parent",
            );
        } else {
            let _ = extract_process_fields(event, "crowdstrike.event.CommandLine", "process");
        }
        return true;
    }

    // Pattern: ProcessStartTime epoch → @timestamp or process.start
    if normalised.contains("ProcessStartTime") || normalised.contains("processStartTime") {
        let _ = epoch_to_timestamp(event, "crowdstrike.event.ProcessStartTime", "process.start");
        return true;
    }

    // Pattern: email split — splitOnToken("@") → user.email, user.domain, user.name
    // Used in Okta, O365, Azure, and many other sources
    if normalised.contains("splitOnToken") && normalised.contains('@') {
        return try_email_split(event, normalised);
    }

    // Pattern: okta risk_behaviors extraction from flattened.behaviors
    // Extracts keys with value "POSITIVE" into an array
    if normalised.contains("POSITIVE") && normalised.contains("risk_behaviors") {
        return try_risk_behaviors(event);
    }

    // Pattern: Azure category → event type/category mapping via params lookup
    if normalised.contains("activitylogs")
        && normalised.contains("category")
        && normalised.contains("params.get")
    {
        return try_azure_category_to_event_type(event);
    }

    // Pattern: Azure event_category assignment, in whichever module's subtree.
    if normalised.contains("event_category") && normalised.contains("eventCategory") {
        return try_azure_event_category(event, normalised);
    }

    // Pattern: replace dots in map keys (Azure identity claims)
    // Matches: ctx.temp_claims[key.replace('.', '_')] = ...
    if normalised.contains("replace('.'") && normalised.contains("keySet()") {
        return try_replace_dots_in_keys(event, normalised);
    }

    // Pattern: okta.target array key renames + user/group extraction
    // Renames alternateId→alternate_id, displayName→display_name in each element,
    // filters detailEntry, extracts first user/usergroup targets
    if normalised.contains("alternateId")
        && normalised.contains("alternate_id")
        && normalised.contains("okta")
    {
        return try_okta_target_rename(event);
    }

    // Pattern: collect one nested key out of every entry of a map.
    if normalised.contains(".keySet()") && normalised.contains(".add(") {
        return try_collect_map_values(event, normalised);
    }

    // Pattern: rewrite one substring of a field in place.
    if normalised.contains(".replace(") {
        return try_guarded_replace(event, normalised);
    }

    // The two catch-alls below are shapes a longer script also CONTAINS, so
    // they run only after every structural matcher has declined.

    // Pattern: scale a number in place by a literal.
    if normalised.contains(" * ") && !normalised.contains("params") {
        return try_scale_by_literal(event, normalised);
    }

    // Pattern: copy one field to another when the source is set.
    if normalised.contains("!= null") && !normalised.contains("for (") {
        return try_guarded_copy(event, normalised);
    }

    false
}

/// Handle the email split Painless pattern.
///
/// Painless patterns like:
/// ```painless
/// String[] splitmail = ctx.user.id.splitOnToken("@");
/// if (splitmail.length != 2) { return; }
/// ctx.user.email = ctx.user.id;
/// ctx.user.domain = splitmail[1];
/// ctx.user.name = splitmail[0];
/// ```
///
/// Also handles prefixed variants: user.target, source.user, destination.user
fn try_email_split(event: &mut Event, script: &str) -> bool {
    // Detect which field prefix this script operates on
    let prefix = if script.contains("ctx.user.target.id") {
        "user.target"
    } else if script.contains("ctx.source.user.id") {
        "source.user"
    } else if script.contains("ctx.destination.user.id") {
        "destination.user"
    } else if script.contains("ctx.user.id") {
        "user"
    } else {
        return false;
    };

    let id_field = format!("{prefix}.id");
    let email_val = match event.get_string(&id_field) {
        Some(v) if v.contains('@') => v,
        _ => return true, // Field missing or not an email — script returns early
    };

    let parts: Vec<&str> = email_val.split('@').collect();
    if parts.len() != 2 {
        return true; // Script returns early on non-standard email
    }

    let _ = event.set(&format!("{prefix}.email"), json!(email_val));
    let _ = event.set(&format!("{prefix}.name"), json!(parts[0]));
    let _ = event.set(&format!("{prefix}.domain"), json!(parts[1]));
    true
}

/// Extract risk behaviors from `okta.debug_context.debug_data.flattened.behaviors`.
///
/// The Painless script iterates the behaviors object and collects keys
/// where the value is "POSITIVE" into an array at `risk_behaviors`.
fn try_risk_behaviors(event: &mut Event) -> bool {
    // No behaviors, or not an object -- the script returns early.
    let Some(Value::Object(behaviors)) = event
        .get("okta.debug_context.debug_data.flattened.behaviors")
        .cloned()
    else {
        return true;
    };

    let positive: Vec<Value> = behaviors
        .iter()
        .filter(|(_, v)| v.as_str() == Some("POSITIVE"))
        .map(|(k, _)| json!(k))
        .collect();

    if !positive.is_empty() {
        let _ = event.set(
            "okta.debug_context.debug_data.risk_behaviors",
            Value::Array(positive),
        );
    }

    true
}

/// Handle the Okta target array key rename + user/group extraction pattern.
///
/// The Painless script:
/// 1. Renames `alternateId→alternate_id`, `displayName→display_name` in each target element
/// 2. Filters detailEntry to only keep methodTypeUsed and methodUsedVerifiedProperties
/// 3. Extracts first "User" type target → `okta_target_user`
/// 4. Extracts first "`UserGroup`" type target → `okta_target_group`
fn try_okta_target_rename(event: &mut Event) -> bool {
    // No target array -- the script returns early.
    let Some(Value::Array(target)) = event.get("okta.target").cloned() else {
        return true;
    };

    let mut result = Vec::with_capacity(target.len());
    let mut target_user: Option<Value> = None;
    let mut target_group: Option<Value> = None;

    for item in &target {
        if let Some(obj) = item.as_object() {
            let mut new_obj = serde_json::Map::new();

            for (k, v) in obj {
                let new_key = match k.as_str() {
                    "alternateId" => "alternate_id",
                    "displayName" => "display_name",
                    // Filtered in place: the script narrows the map and drops
                    // the key only when nothing survives, so the name stays.
                    "detailEntry" => {
                        if let Some(de) = v.as_object() {
                            let filtered: serde_json::Map<String, Value> = de
                                .iter()
                                .filter(|(k, _)| {
                                    k.as_str() == "methodTypeUsed"
                                        || k.as_str() == "methodUsedVerifiedProperties"
                                })
                                .map(|(k, v)| (k.clone(), v.clone()))
                                .collect();
                            if !filtered.is_empty() {
                                new_obj.insert("detailEntry".to_string(), Value::Object(filtered));
                            }
                        }
                        continue;
                    }
                    other => other,
                };
                new_obj.insert(new_key.to_string(), v.clone());
            }

            let new_val = Value::Object(new_obj.clone());

            // Extract first user/usergroup targets
            if let Some(type_val) = new_obj.get("type").and_then(|v| v.as_str()) {
                let type_lower = type_val.to_lowercase();
                if type_lower == "user" && target_user.is_none() {
                    target_user = Some(new_val.clone());
                } else if type_lower == "usergroup" && target_group.is_none() {
                    target_group = Some(new_val.clone());
                }
            }

            result.push(new_val);
        } else {
            result.push(item.clone());
        }
    }

    let _ = event.set("okta.target", Value::Array(result));

    if let Some(user) = target_user {
        let _ = event.set("okta_target_user", user);
    }
    if let Some(group) = target_group {
        let _ = event.set("okta_target_group", group);
    }

    true
}

/// Azure category → event type mapping.
///
/// Maps activitylogs.category to event.type via params lookup:
/// write/action → `["change"]`, read → `["access"]`, delete → `["deletion"]`
fn try_azure_category_to_event_type(event: &mut Event) -> bool {
    let category = match event.get_str("azure.activitylogs.category") {
        Some(c) => c.to_lowercase(),
        None => return true, // No category — script returns early
    };

    let event_types: Option<Vec<&str>> = match category.as_str() {
        "write" | "action" => Some(vec!["change"]),
        "read" => Some(vec!["access"]),
        "delete" => Some(vec!["deletion"]),
        _ => None,
    };

    if let Some(types) = event_types {
        for t in types {
            let _ = event.set("event.type", json!([t]));
        }
    }

    true
}

/// Azure's `event_category` conditional assignment: `properties.eventCategory`
/// if present, else a literal per fallback branch.
///
/// The subtree and both literals are read out of the SCRIPT. They were
/// hardcoded to `azure.activitylogs`, and azure's four modules share this
/// script with their own prefix -- so platformlogs had its category written
/// under activitylogs, where nothing downstream reads it.
fn try_azure_event_category(event: &mut Event, script: &str) -> bool {
    use crate::painless_params::ctx_path_before;

    let Some(prefix) = ctx_path_before(script, ".event_category") else {
        return false;
    };

    // Each fallback branch assigns a literal; the last is the default and any
    // before it belongs to the `policies` test.
    let literals: Vec<String> = script
        .split(".event_category = ")
        .skip(1)
        .filter(|branch| branch.trim_start().starts_with(['\'', '"']))
        .filter_map(quoted_first)
        .collect();
    let Some(default) = literals.last() else {
        return false;
    };

    let category = if let Some(v) = event.get_str(&format!("{prefix}.properties.eventCategory")) {
        v.to_string()
    } else if literals.len() >= 2 && event.has(&format!("{prefix}.properties.policies")) {
        literals[literals.len() - 2].clone()
    } else {
        default.clone()
    };

    let _ = event.set(&format!("{prefix}.event_category"), json!(category));
    true
}

/// Replace dots with underscores in map keys at a given field path.
///
/// Common Azure pattern — identity claims have dots in URLs that Elastic normalises:
/// ```painless
/// for (String key : ctx.azure.activitylogs.identity.claims.keySet()) {
///   ctx.temp_claims[key.replace('.', '_')] = ctx.azure.activitylogs.identity.claims.get(key);
/// }
/// ctx.azure.activitylogs.identity.claims = ctx.temp_claims;
/// ```
fn try_replace_dots_in_keys(event: &mut Event, script: &str) -> bool {
    // Extract the field path by finding `ctx.<path>.keySet()`
    let field_path = if let Some(keyset_pos) = script.find(".keySet()") {
        // Walk backwards from .keySet() to find `ctx.`
        let before = &script[..keyset_pos];
        if let Some(ctx_pos) = before.rfind("ctx.") {
            let path = &before[ctx_pos + 4..];
            path.replace("?.", ".").replace('?', "")
        } else {
            return false;
        }
    } else {
        return false;
    };

    // Navigate to the parent object via JSON pointer to avoid dotted-path
    // issues with keys that contain literal dots (e.g., URL-like claim names)
    let pointer = format!("/{}", field_path.replace('.', "/"));
    let inner = event.as_value_mut();
    let resolved = inner.pointer_mut(&pointer);
    // Field missing or not an object -- skip.
    let Some(Value::Object(obj)) = resolved else {
        return true;
    };

    let new_map: Map<String, Value> = obj
        .iter()
        .map(|(k, v)| (k.replace('.', "_"), v.clone()))
        .collect();

    *obj = new_map;
    true
}

/// Try to extract a target field from a Painless script like `ctx.field_name`.
fn extract_target_field(script: &str) -> Option<String> {
    // Look for patterns like ctx.okta.request or ctx.field
    for line in script.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("ctx.") && !trimmed.contains('(') {
            let field = trimmed
                .trim_start_matches("ctx.")
                .trim_end_matches(';')
                .trim();
            if !field.is_empty() && !field.contains(' ') {
                return Some(field.replace("?.", ".").replace('?', ""));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The scripts below are the verbatim text the transform modules pass
    /// to `painless_exec`, so a change upstream shows up here as a miss.
    /// Verbatim from `pipelines/cisco_asa/default.yml`, tagged
    /// `script_process_flow_duration`. `cisco_ftd` ships it too.
    const FLOW_DURATION: &str = "long parse_hms(String s) {\n    long cur = 0, total = 0;\n    \
        for (char c: s.toCharArray()) {\n        if (c >= (char)'0' && c <= (char)'9') {\n    \
        cur = (cur*10) + (long)c - (char)'0';\n        } else if (c == (char)':') {\n    \
        total = (total + cur) * 60;\n            cur = 0;\n        }\n    }\n    \
        return total + cur;\n}\nif (ctx?.event == null) {\n    ctx['event'] = new HashMap();\n}\n\
        long nanos = parse_hms(ctx._temp_.duration_hms) * 1000000000L;\n\
        ctx.event['duration'] = nanos;\nif (ctx['@timestamp'] != null) {\n    \
        String end = ctx['@timestamp'];\n    ctx.event['end'] = end;\n    try {\n        \
        ctx.event['start'] = ZonedDateTime.ofInstant(\n            \
        Instant.parse(end).minusNanos(nanos),\n            ZoneOffset.UTC);\n    } \
        catch (Exception e) {\n    }\n}\n";

    /// The colon form is positional: `0:01:07` is 67 seconds, and the start is
    /// that far before the event's own timestamp.
    #[test]
    fn a_flow_duration_becomes_a_span_ending_at_the_timestamp() {
        let mut event = Event::new(json!({
            "@timestamp": "2018-10-10T12:34:56.000Z",
            "_temp_": { "duration_hms": "0:01:07" },
        }));

        assert!(try_known_painless(&mut event, FLOW_DURATION));

        assert_eq!(event.get("event.duration"), Some(&json!(67_000_000_000i64)));
        assert_eq!(
            event.get("event.end"),
            Some(&json!("2018-10-10T12:34:56.000Z"))
        );
        assert_eq!(
            event.get("event.start"),
            Some(&json!("2018-10-10T12:33:49.000Z"))
        );
    }

    /// With no timestamp there is nothing to count back from, so the duration
    /// is written and the span is not.
    #[test]
    fn a_flow_duration_without_a_timestamp_sets_only_the_duration() {
        let mut event = Event::new(json!({ "_temp_": { "duration_hms": "0:00:05" } }));

        assert!(try_known_painless(&mut event, FLOW_DURATION));

        assert_eq!(event.get("event.duration"), Some(&json!(5_000_000_000i64)));
        assert_eq!(event.get("event.start"), None);
        assert_eq!(event.get("event.end"), None);
    }

    /// Verbatim from `pipelines/cisco_umbrella/default.yml`, cut to two of its
    /// three helpers and their rules.
    const IDENTITIES: &str = "void setUser(def ctx, def x) {\n  if (ctx.user == null) {\n    \
        ctx.user = new HashMap();\n  }\n  if (ctx.user.name == null) {\n    \
        ctx.user.name = x;\n  }\n}\nvoid addNetwork(def ctx, def x) {\n  \
        if (ctx.network == null) {\n    ctx.network = new HashMap();\n  }\n  \
        if (ctx.network?.name == null) {\n    ArrayList al = new ArrayList();\n    \
        ctx.network.put(\"name\", al);\n  }\n  if (!ctx.network.name.contains(x)) {\n    \
        ctx.network.name.add(x);\n  }\n}\ndef i = 0;\n\
        for (cisco_identity_type in ctx.cisco.umbrella.identity_types) {\n  \
        if ([\"AD Users\"].contains(cisco_identity_type)) {\n    \
        setUser(ctx, ctx.cisco.umbrella.identities[i]);\n  }\n  \
        if ([\"Sites\", \"Internal Networks\", \"Networks\"].contains(cisco_identity_type)) {\n    \
        addNetwork(ctx, ctx.cisco.umbrella.identities[i]);\n  }\n  i++;\n}";

    /// Each identity goes where its KIND says, position for position.
    #[test]
    fn parallel_identities_go_where_their_kind_says() {
        let mut event = Event::new(json!({
            "cisco": { "umbrella": {
                "identities": ["elasticuser", "Users-Internal", "Default Site"],
                "identity_types": ["AD Users", "Internal Networks", "Sites"],
            } },
        }));

        assert!(try_known_painless(&mut event, IDENTITIES));

        assert_eq!(event.get("user.name"), Some(&json!("elasticuser")));
        assert_eq!(
            event.get("network.name"),
            Some(&json!(["Users-Internal", "Default Site"]))
        );
    }

    /// A name already there is kept: the helper only sets what is absent.
    #[test]
    fn a_name_already_set_is_not_replaced() {
        let mut event = Event::new(json!({
            "user": { "name": "already-here" },
            "cisco": { "umbrella": {
                "identities": ["elasticuser"],
                "identity_types": ["AD Users"],
            } },
        }));

        assert!(try_known_painless(&mut event, IDENTITIES));

        assert_eq!(event.get("user.name"), Some(&json!("already-here")));
    }

    /// A kind no rule names contributes nothing.
    #[test]
    fn an_unnamed_identity_kind_is_ignored() {
        let mut event = Event::new(json!({
            "cisco": { "umbrella": {
                "identities": ["something"],
                "identity_types": ["Some Future Kind"],
            } },
        }));

        assert!(try_known_painless(&mut event, IDENTITIES));

        assert_eq!(event.get("user.name"), None);
        assert_eq!(event.get("network.name"), None);
    }

    /// Verbatim from `pipelines/cisco_ftd/default.yml`. FTD writes both
    /// readings as two branches: for 430003 the timestamp is the START.
    const FLOW_BOTH_WAYS: &str = "long parse_hms(String s) {\n    long cur = 0, total = 0;\n    \
        for (char c: s.toCharArray()) {\n        cur = cur;\n    }\n    return total + cur;\n} \
        if (ctx.event == null) {\n    ctx['event'] = new HashMap();\n} \
        if (ctx?._temp_.cisco?.message_id == '430003') {\n  String start = ctx['@timestamp'];\n  \
        ctx.event['start'] = start;\n  long nanos = parse_hms(ctx._temp_.duration_hms) * \
        1000000000L;\n  ctx.event['duration'] = nanos;\n  ctx.event['end'] = \
        ZonedDateTime.ofInstant(\n      Instant.parse(start).plusNanos(nanos),\n      \
        ZoneOffset.UTC);\n} else {\n  String end = ctx['@timestamp'];\n  \
        ctx.event['end'] = end;\n  long nanos = parse_hms(ctx._temp_.duration_hms) * \
        1000000000L;\n  ctx.event['duration'] = nanos;\n  ctx.event['start'] = \
        ZonedDateTime.ofInstant(\n      Instant.parse(end).minusNanos(nanos),\n      \
        ZoneOffset.UTC);\n}\n";

    /// 430003 is timestamped at the start, so the end is counted FORWARD.
    #[test]
    fn a_start_anchored_flow_counts_the_end_forward() {
        let mut event = Event::new(json!({
            "@timestamp": "2018-10-10T12:33:49.000Z",
            "_temp_": { "cisco": { "message_id": "430003" }, "duration_hms": "0:01:07" },
        }));

        assert!(try_known_painless(&mut event, FLOW_BOTH_WAYS));

        assert_eq!(
            event.get("event.start"),
            Some(&json!("2018-10-10T12:33:49.000Z"))
        );
        assert_eq!(
            event.get("event.end"),
            Some(&json!("2018-10-10T12:34:56.000Z"))
        );
        assert_eq!(event.get("event.duration"), Some(&json!(67_000_000_000i64)));
    }

    /// Every other id is timestamped at the end, so the start is counted BACK.
    #[test]
    fn an_end_anchored_flow_counts_the_start_back() {
        let mut event = Event::new(json!({
            "@timestamp": "2018-10-10T12:34:56.000Z",
            "_temp_": { "cisco": { "message_id": "430002" }, "duration_hms": "0:01:07" },
        }));

        assert!(try_known_painless(&mut event, FLOW_BOTH_WAYS));

        assert_eq!(
            event.get("event.start"),
            Some(&json!("2018-10-10T12:33:49.000Z"))
        );
        assert_eq!(
            event.get("event.end"),
            Some(&json!("2018-10-10T12:34:56.000Z"))
        );
    }

    const SUM_BYTES: &str = "ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes";
    const SUM_PACKETS: &str = "ctx.network.packets = ctx.source.packets + ctx.destination.packets";
    const DURATION_NANOS: &str =
        "ctx.event.duration = Long.parseLong(ctx.fortinet.firewall.duration) * 1000000000";
    /// Verbatim from `pipelines/fortinet/default.yml`, trimmed to four arms.
    const IANA_LADDER: &str = "def iana_number = ctx.network.iana_number;\nif (iana_number == '0') \
                               {\n    ctx.network.transport = 'hopopt';\n} else if (iana_number == \
                               '1') {\n    ctx.network.transport = 'icmp';\n} else if (iana_number \
                               == '6') {\n    ctx.network.transport = 'tcp';\n} else if \
                               (iana_number == '17') {\n    ctx.network.transport = 'udp';\n}";
    const APPEND_DNS: &str = "def dnsIPs = ctx.dns?.resolved_ip;\nif (dnsIPs != null) {\n  \
                              for (ip in dnsIPs) {\n    if (!ctx.related.ip.contains(ip)) \
                              {\n ctx.related.ip.add(ip);\n }\n  }\n}";
    /// Verbatim from `pipelines/fortinet/event.yml`. fortinet's VPN logs are
    /// back to front by ECS's reckoning: `remip` is the client and `locip` the
    /// firewall, so the pipeline renames them the obvious way and then swaps.
    const VPN_SWAP: &str = "def tmp = ctx.source;\nctx.source = ctx.destination;\n\
                            if (ctx.source == null) { ctx.source = [:]; }\n\
                            if ( tmp?.user != null ) {\n    ctx.source.user = tmp.user;\n    \
                            tmp.remove(\"user\");\n}\nctx.destination = tmp;";

    /// Verbatim from `pipelines/o365/audit.yml`, which collects a mail rule's
    /// forwarding addresses out of three optional parameters.
    const SPLIT_TRIM_ADD: &str = "void splitTrimAdd(Set acc, String str) {\n    \
        if (str != null && str != '') {\n        String[] parts = str.splitOnToken(';');\n        \
        for (int i = 0; i < parts.length; i++) {\n            acc.add(parts[i].trim());\n        \
        }\n    }\n}\ndef addressSet = new HashSet(ctx.email?.to?.address ?: []);\n\
        splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.ForwardAsAttachmentTo); \
        splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.ForwardTo); \
        splitTrimAdd(addressSet, ctx.o365audit?.Parameters?.RedirectTo);\n\
        if (!addressSet.isEmpty()) {\n  ctx.email = ctx.email ?: [:];\n  \
        ctx.email.to = ctx.email.to ?: [:];\n  ctx.email.to.address = addressSet.asList();\n}\n";

    /// Verbatim from `pipelines/fortinet/utm.yml`, which splits `tlsver` into
    /// the two ECS `tls` fields at the version's first digit.
    const TLS_VERSION: &str = "def pat = /\\d+/; def tlsver = \
        ctx.fortinet.firewall.tlsver.toLowerCase(); def matcher = pat.matcher(tlsver); \
        if (!matcher.find()) {\n    return;\n} if (ctx.tls == null) {\n    \
        ctx.tls = new HashMap();\n} ctx.tls.version_protocol = tlsver.substring(0, \
        matcher.start()); ctx.tls.version = tlsver.substring(matcher.start(), \
        tlsver.length()); if (!ctx.tls.version.contains(\".\")) {\n    \
        ctx.tls.version += \".0\";\n}";

    /// Verbatim from `pipelines/o365/default.yml`, which builds `message` for
    /// a DLP-Exchange alert out of three fields that may each be absent.
    const DLP_MESSAGE: &str = "def operation = ctx.event?.action ?: '';\n\
        def user = ctx.user?.id ?: '';\n\
        def subject = ctx.o365audit?.ExchangeMetaData?.Subject ?: ctx.email?.subject ?: '';\n\
        if (operation.isEmpty() && user.isEmpty() && subject.isEmpty()) {\n  \
        ctx.message = \"Office365 Alert\";\n} else {\n  \
        ctx.message = \"Office365 Alert: \" + operation + \" detected in email sent by \" + \
        user + \" with subject '\" + subject + \"'\";\n}";

    /// Verbatim from `pipelines/crowdstrike/firewall_match.yml`. Every rename
    /// of `LocalAddress` and `RemoteAddress` after it is gated on the result.
    const DIRECTION: &str = "def result = [];\n\
        if (ctx.crowdstrike.event.ConnectionDirection == \"0\") {\n  result.add('egress');\n}\n\
        else if (ctx.crowdstrike.event.ConnectionDirection == \"1\") {\n  result.add('ingress');\n}\n\
        else if (ctx.crowdstrike.event.ConnectionDirection == \"3\") {\n  \
        result.add('egress');\n  result.add('ingress');\n}\n\
        if (result.size() > 0) {\n  ctx.network = ctx.network ?: [:];\n}\n\
        if (result.size() == 1) {\n  ctx.network.direction = result[0];\n}\n\
        else if (result.size() > 1) {\n  ctx.network.direction = result;\n}";

    /// Verbatim from `pipelines/fortinet/traffic.yml`. The directional matcher
    /// only knows source-plus-destination-into-network, and this is three
    /// different names.
    #[test]
    fn a_sum_reads_all_three_names_out_of_the_script() {
        let script = "ctx.fortinet.firewall.deltabytes = ctx.fortinet.firewall.rcvddelta \
                      + ctx.fortinet.firewall.sentdelta";
        let mut event = Event::new(json!({
            "fortinet": { "firewall": { "rcvddelta": 1000, "sentdelta": 304 } },
        }));

        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get("fortinet.firewall.deltabytes"),
            Some(&json!(1304))
        );
    }

    /// Elastic gates the script on both sides being a Number and would throw
    /// otherwise, so an absent side writes nothing.
    #[test]
    fn a_sum_with_a_side_missing_writes_nothing() {
        let script = "ctx.a.total = ctx.a.left + ctx.a.right";
        let mut event = Event::new(json!({ "a": { "left": 5 } }));

        assert!(try_known_painless(&mut event, script));
        assert!(!event.has("a.total"));
    }

    /// The fixture line fortinet 7.4 logs: `tlsver="tls1.3"`.
    #[test]
    fn a_version_splits_at_its_first_digit() {
        let mut event = Event::new(json!({
            "fortinet": { "firewall": { "tlsver": "TLS1.3" } },
        }));

        assert!(try_known_painless(&mut event, TLS_VERSION));
        assert_eq!(event.get_str("tls.version_protocol"), Some("tls"));
        assert_eq!(event.get_str("tls.version"), Some("1.3"));
    }

    /// The o365 fixture's own line: three addresses in one semicolon-delimited
    /// `ForwardTo`.
    #[test]
    fn split_trim_collect_gathers_every_forwarding_address() {
        let mut event = Event::new(json!({
            "o365audit": { "Parameters": {
                "ForwardTo": "external1@example.com;external2@example.com;external3@example.com",
                "RedirectTo": " spaced@example.com ",
            } },
        }));

        assert!(try_known_painless(&mut event, SPLIT_TRIM_ADD));
        assert_eq!(
            event.get("email.to.address"),
            Some(&json!([
                "external1@example.com",
                "external2@example.com",
                "external3@example.com",
                "spaced@example.com",
            ]))
        );
    }

    /// The set is SEEDED from what the target already holds, and a duplicate
    /// coming in over the top of it is dropped.
    #[test]
    fn split_trim_collect_seeds_from_the_target_and_dedups() {
        let mut event = Event::new(json!({
            "email": { "to": { "address": ["already@example.com"] } },
            "o365audit": { "Parameters": { "ForwardTo": "already@example.com;new@example.com" } },
        }));

        assert!(try_known_painless(&mut event, SPLIT_TRIM_ADD));
        assert_eq!(
            event.get("email.to.address"),
            Some(&json!(["already@example.com", "new@example.com"]))
        );
    }

    /// `if (!addressSet.isEmpty())` -- no parameters, no field.
    #[test]
    fn split_trim_collect_writes_nothing_when_it_gathers_nothing() {
        let mut event = Event::new(json!({ "o365audit": { "Parameters": {} } }));

        assert!(try_known_painless(&mut event, SPLIT_TRIM_ADD));
        assert!(!event.has("email.to.address"));
    }

    /// A version with no dot gets `.0`, which is the script's last three lines
    /// and the reason `tls1` and `tls1.0` end up the same.
    #[test]
    fn a_version_without_a_dot_gains_one() {
        let mut event = Event::new(json!({
            "fortinet": { "firewall": { "tlsver": "tls1" } },
        }));

        assert!(try_known_painless(&mut event, TLS_VERSION));
        assert_eq!(event.get_str("tls.version"), Some("1.0"));
    }

    /// `if (!matcher.find()) { return; }` -- no digit, so nothing is written
    /// and the script is still counted as run.
    #[test]
    fn a_version_with_no_digit_writes_nothing() {
        let mut event = Event::new(json!({
            "fortinet": { "firewall": { "tlsver": "unknown" } },
        }));

        assert!(try_known_painless(&mut event, TLS_VERSION));
        assert!(!event.has("tls.version"));
        assert!(!event.has("tls.version_protocol"));
    }

    /// One match writes a scalar.
    #[test]
    fn collecting_ladder_writes_a_single_match_as_a_string() {
        let mut event = Event::new(json!({
            "crowdstrike": { "event": { "ConnectionDirection": "1" } },
        }));

        assert!(try_known_painless(&mut event, DIRECTION));
        assert_eq!(event.get_str("network.direction"), Some("ingress"));
    }

    /// Several matches write an array, which is the half a plain ladder cannot
    /// express.
    #[test]
    fn collecting_ladder_writes_several_matches_as_an_array() {
        let mut event = Event::new(json!({
            "crowdstrike": { "event": { "ConnectionDirection": "3" } },
        }));

        assert!(try_known_painless(&mut event, DIRECTION));
        assert_eq!(
            event.get("network.direction"),
            Some(&json!(["egress", "ingress"]))
        );
    }

    /// The target is the field the accumulator is assigned to, not the parent
    /// the size test creates -- reading the size test named `network`.
    #[test]
    fn collecting_ladder_writes_the_field_not_its_parent() {
        let mut event = Event::new(json!({
            "crowdstrike": { "event": { "ConnectionDirection": "0" } },
        }));

        assert!(try_known_painless(&mut event, DIRECTION));
        assert_eq!(event.get_str("network.direction"), Some("egress"));
        assert!(event.get("network").is_some_and(Value::is_object));
    }

    /// A value no arm matches leaves the field unwritten, which is what an
    /// empty accumulator does.
    #[test]
    fn collecting_ladder_writes_nothing_when_no_arm_matches() {
        let mut event = Event::new(json!({
            "crowdstrike": { "event": { "ConnectionDirection": "9" } },
        }));

        assert!(try_known_painless(&mut event, DIRECTION));
        assert!(!event.has("network.direction"));
    }

    #[test]
    fn concat_builds_the_message_from_the_fields_it_names() {
        let mut event = Event::new(json!({
            "event": { "action": "DlpRuleMatch" },
            "user": { "id": "DlpAgent" },
        }));

        assert!(try_known_painless(&mut event, DLP_MESSAGE));
        assert_eq!(
            event.get_str("message"),
            Some(
                "Office365 Alert: DlpRuleMatch detected in email sent by DlpAgent with subject ''"
            )
        );
    }

    /// The `?:` chain takes the first field that is present AND non-empty, so
    /// an empty subject falls through to the next alternative.
    #[test]
    fn concat_falls_through_an_empty_alternative() {
        let mut event = Event::new(json!({
            "event": { "action": "DlpRuleMatch" },
            "o365audit": { "ExchangeMetaData": { "Subject": "" } },
            "email": { "subject": "Q3 numbers" },
        }));

        assert!(try_known_painless(&mut event, DLP_MESSAGE));
        assert_eq!(
            event.get_str("message"),
            Some(
                "Office365 Alert: DlpRuleMatch detected in email sent by  with subject 'Q3 numbers'"
            )
        );
    }

    /// Every field absent takes the other branch, which is a bare literal.
    #[test]
    fn concat_takes_the_literal_when_every_field_is_empty() {
        let mut event = Event::new(json!({ "event": { "code": "ComplianceDLPExchange" } }));

        assert!(try_known_painless(&mut event, DLP_MESSAGE));
        assert_eq!(event.get_str("message"), Some("Office365 Alert"));
    }

    #[test]
    fn vpn_swap_exchanges_source_and_destination() {
        let mut event = Event::new(json!({
            "source": { "ip": "10.0.0.1", "port": 500 },
            "destination": { "ip": "203.0.113.7", "port": 500 },
        }));

        assert!(try_known_painless(&mut event, VPN_SWAP));
        assert_eq!(event.get_str("source.ip"), Some("203.0.113.7"));
        assert_eq!(event.get_str("destination.ip"), Some("10.0.0.1"));
    }

    /// `user` describes the person, not the address, so it stays with the
    /// source rather than riding the swap across.
    #[test]
    fn vpn_swap_keeps_the_user_on_the_source() {
        let mut event = Event::new(json!({
            "source": { "ip": "10.0.0.1", "user": { "name": "derek" } },
            "destination": { "ip": "203.0.113.7" },
        }));

        assert!(try_known_painless(&mut event, VPN_SWAP));
        assert_eq!(event.get_str("source.ip"), Some("203.0.113.7"));
        assert_eq!(event.get_str("source.user.name"), Some("derek"));
        assert!(!event.has("destination.user"));
        assert_eq!(event.get_str("destination.ip"), Some("10.0.0.1"));
    }

    /// A VPN event carrying only `remip` leaves one side with nothing, and the
    /// captured Elasticsearch output has no `destination` key at all -- not a
    /// null one.
    #[test]
    fn vpn_swap_removes_a_side_left_with_nothing() {
        let mut event = Event::new(json!({ "destination": { "ip": "203.0.113.7" } }));

        assert!(try_known_painless(&mut event, VPN_SWAP));
        assert_eq!(event.get_str("source.ip"), Some("203.0.113.7"));
        assert!(!event.has("destination"), "destination survived as null");
    }

    /// Verbatim from `pipelines/crowdstrike/default.yml`.
    const APPEND_TAGS: &str = "if (ctx.crowdstrike.event.Tags instanceof List) {\n    for (tag in \
         ctx.crowdstrike.event.Tags) {\n        if (tag instanceof Map) {\n          \
         ctx.tags.add(tag[\"Key\"] + \":\" + tag[\"ValueString\"]);\n        }\n    }\n} else if \
         (ctx.crowdstrike.event.Tags instanceof String) {\n    def values = \
         ctx.crowdstrike.event.Tags.splitOnToken(',');\n    for (value in values) {\n        \
         ctx.tags.add(value.trim());\n    }\n}";

    #[test]
    fn splits_a_delimited_string_onto_the_end_of_the_array() {
        let mut event = Event::new(json!({
            "crowdstrike": { "event": { "Tags": "SensorGroupingTags/TEACHER, FalconGroupingTags/X" }},
            "tags": ["preserve_original_event"],
        }));

        assert!(try_known_painless(&mut event, APPEND_TAGS));

        assert_eq!(
            event.get("tags"),
            Some(&json!([
                "preserve_original_event",
                "SensorGroupingTags/TEACHER",
                "FalconGroupingTags/X"
            ]))
        );
    }

    /// The same script's other branch: the field arrives as maps, not a string.
    #[test]
    fn joins_each_map_pair_onto_the_end_of_the_array() {
        let mut event = Event::new(json!({
            "crowdstrike": { "event": { "Tags": [
                { "Key": "env", "ValueString": "prod" },
                { "Key": "team", "ValueString": "sec" },
            ]}},
            "tags": ["preserve_original_event"],
        }));

        assert!(try_known_painless(&mut event, APPEND_TAGS));

        assert_eq!(
            event.get("tags"),
            Some(&json!(["preserve_original_event", "env:prod", "team:sec"]))
        );
    }

    #[test]
    fn an_absent_source_leaves_the_array_alone() {
        let mut event = Event::new(json!({ "tags": ["preserve_original_event"] }));
        assert!(try_known_painless(&mut event, APPEND_TAGS));
        assert_eq!(event.get("tags"), Some(&json!(["preserve_original_event"])));
    }

    /// Shortened from `pipelines/fortinet/default.yml` -- the parts that
    /// identify the shape, not the whole 30-line definition.
    const SPLIT_UNQUOTED: &str = "def splitUnquoted(String input, String sep) {\n  def tokens = \
                                  [];\n}\ndef arr = splitUnquoted(ctx.syslog5424_sd, \" \");\n\
                                  Map map = new HashMap();\nfor (def i = 0; i < arr?.length; i++) \
                                  {\n  def kv = splitUnquoted(arr[i], \"=\");\n}\n\
                                  ctx.fortinet.firewall = map;\n";

    #[test]
    fn a_quoted_value_keeps_its_spaces() {
        let mut event = Event::new(json!({
            "syslog5424_sd": "type=\"utm\" msg=\"URL belongs to a denied category\" policyid=100602",
        }));

        assert!(try_known_painless(&mut event, SPLIT_UNQUOTED));

        assert_eq!(event.get_str("fortinet.firewall.type"), Some("utm"));
        assert_eq!(
            event.get_str("fortinet.firewall.msg"),
            Some("URL belongs to a denied category")
        );
        assert_eq!(event.get_str("fortinet.firewall.policyid"), Some("100602"));
    }

    /// The vendor's `kv.length == 2` guard: a fragment with no `=` is skipped.
    #[test]
    fn a_fragment_without_the_pair_separator_is_skipped() {
        let mut event = Event::new(json!({ "syslog5424_sd": "bare a=1" }));
        assert!(try_known_painless(&mut event, SPLIT_UNQUOTED));

        let map = event.get_object("fortinet.firewall").unwrap();
        assert_eq!(map.len(), 1);
        assert_eq!(map.get("a").and_then(Value::as_str), Some("1"));
    }

    #[test]
    fn sums_bytes_and_packets_across_directions() {
        let mut event = Event::new(json!({
            "source": { "bytes": 100, "packets": 3 },
            "destination": { "bytes": 250, "packets": 4 },
        }));

        assert!(try_known_painless(&mut event, SUM_BYTES));
        assert!(try_known_painless(&mut event, SUM_PACKETS));

        assert_eq!(event.get_i64("network.bytes"), Some(350));
        assert_eq!(event.get_i64("network.packets"), Some(7));
    }

    /// Elastic's script throws when a side is missing; skipping is what the
    /// surrounding pipeline already relies on.
    #[test]
    fn a_missing_direction_leaves_the_total_unset() {
        let mut event = Event::new(json!({ "source": { "bytes": 100 } }));
        assert!(try_known_painless(&mut event, SUM_BYTES));
        assert!(!event.has("network.bytes"));
    }

    #[test]
    fn converts_a_duration_from_seconds_to_nanoseconds() {
        let mut event = Event::new(json!({ "fortinet": { "firewall": { "duration": 42 } } }));
        assert!(try_known_painless(&mut event, DURATION_NANOS));
        assert_eq!(event.get_i64("event.duration"), Some(42_000_000_000));
    }

    /// Both operands come off the wire. A vendor reporting a nonsense count
    /// must cost a saturated total, not a debug panic or a negative release
    /// one -- these are byte counts a dashboard sums.
    #[test]
    fn a_nonsense_byte_count_saturates_rather_than_wrapping() {
        let mut event = Event::new(json!({
            "source": { "bytes": i64::MAX },
            "destination": { "bytes": 1 },
        }));

        assert!(try_known_painless(&mut event, SUM_BYTES));
        assert_eq!(event.get_i64("network.bytes"), Some(i64::MAX));
    }

    #[test]
    fn a_nonsense_duration_saturates_rather_than_wrapping() {
        let mut event = Event::new(json!({ "fortinet": { "firewall": { "duration": i64::MAX } } }));

        assert!(try_known_painless(&mut event, DURATION_NANOS));
        assert_eq!(event.get_i64("event.duration"), Some(i64::MAX));
    }

    /// A duration script whose field name this code cannot read is NOT
    /// handled. Counting it would inflate the coverage figure with scripts
    /// nothing actually ran.
    #[test]
    fn an_unreadable_duration_script_is_not_counted_as_handled() {
        let mut event = Event::new(json!({}));
        let script = "ctx.event.duration = Long.parseLong(something) * 1000000000";
        assert!(!try_known_painless(&mut event, script));
    }

    /// The vendor field is often a string, because it came out of a grok.
    #[test]
    fn a_string_duration_converts_too() {
        let mut event = Event::new(json!({ "fortinet": { "firewall": { "duration": "7" } } }));
        assert!(try_known_painless(&mut event, DURATION_NANOS));
        assert_eq!(event.get_i64("event.duration"), Some(7_000_000_000));
    }

    /// The mapping is read out of the ladder, not transcribed into Rust --
    /// a hand-written copy is what goes stale when a vendor adds a protocol.
    #[test]
    fn an_equality_ladder_assigns_the_matching_arm() {
        for (iana, transport) in [("0", "hopopt"), ("6", "tcp"), ("17", "udp")] {
            let mut event = Event::new(json!({ "network": { "iana_number": iana } }));
            assert!(try_known_painless(&mut event, IANA_LADDER));
            assert_eq!(event.get_str("network.transport"), Some(transport));
        }
    }

    /// The grok types this capture as a long, so the ladder has to compare
    /// the number's text -- Painless is doing the same widening.
    #[test]
    fn an_equality_ladder_reads_a_numeric_subject() {
        let mut event = Event::new(json!({ "network": { "iana_number": 6 } }));
        assert!(try_known_painless(&mut event, IANA_LADDER));
        assert_eq!(event.get_str("network.transport"), Some("tcp"));
    }

    /// A value no arm names leaves the target alone, rather than taking the
    /// last arm or writing a placeholder.
    #[test]
    fn an_equality_ladder_with_no_matching_arm_writes_nothing() {
        let mut event = Event::new(json!({ "network": { "iana_number": "254" } }));
        assert!(try_known_painless(&mut event, IANA_LADDER));
        assert!(!event.has("network.transport"));
    }

    /// panw's own "crude `uri_parts`", as the generator emits it.
    const SCHEMELESS_URL: &str = r#"Map url = new HashMap();
String url_original = ctx.url.original;
String domainPort = url_original;
url.original = url_original;
if (url_original.contains("/")) {
    int idxSlash = url_original.indexOf("/");
    domainPort = url_original.substring(0, idxSlash);
}
if (domainPort.indexOf(":") != -1) {
    url.domain = domainPort.splitOnToken(":")[0];
}
ctx.url = url;
"#;

    #[test]
    fn a_schemeless_url_splits_into_domain_path_and_extension() {
        let mut event = Event::new(json!({
            "url": { "original": "lorexx.cn/loader.exe" },
            "destination": {},
        }));

        assert!(try_known_painless(&mut event, SCHEMELESS_URL));
        assert_eq!(event.get("url.domain"), Some(&json!("lorexx.cn")));
        assert_eq!(event.get("url.path"), Some(&json!("/loader.exe")));
        assert_eq!(event.get("url.extension"), Some(&json!("exe")));
        assert_eq!(event.get("destination.domain"), Some(&json!("lorexx.cn")));
    }

    #[test]
    fn a_query_string_is_split_off_the_path() {
        let mut event = Event::new(json!({
            "url": { "original": "lsiu.info/evo/count.php?id=7&v=2" },
        }));

        assert!(try_known_painless(&mut event, SCHEMELESS_URL));
        assert_eq!(event.get("url.path"), Some(&json!("/evo/count.php")));
        assert_eq!(event.get("url.query"), Some(&json!("id=7&v=2")));
        assert_eq!(event.get("url.extension"), Some(&json!("php")));
    }

    #[test]
    fn a_port_is_taken_off_the_domain() {
        let mut event = Event::new(json!({ "url": { "original": "example.com:8080/a" } }));

        assert!(try_known_painless(&mut event, SCHEMELESS_URL));
        assert_eq!(event.get("url.domain"), Some(&json!("example.com")));
        assert_eq!(event.get("url.port"), Some(&json!(8080)));
    }

    /// The script swallows the parse failure, so a non-numeric port must leave
    /// `url.port` unset rather than failing the event or storing the text.
    #[test]
    fn a_non_numeric_port_leaves_the_port_unset() {
        let mut event = Event::new(json!({ "url": { "original": "example.com:abc/a" } }));

        assert!(try_known_painless(&mut event, SCHEMELESS_URL));
        assert_eq!(event.get("url.domain"), Some(&json!("example.com")));
        assert!(!event.has("url.port"));
    }

    /// `ctx.url = url` REPLACES the object, so a field already under it goes.
    #[test]
    fn the_url_object_is_replaced_not_merged() {
        let mut event = Event::new(json!({
            "url": { "original": "example.com/a", "stale": "left over" },
        }));

        assert!(try_known_painless(&mut event, SCHEMELESS_URL));
        assert!(!event.has("url.stale"));
    }

    /// Painless would throw writing through an absent `ctx.destination`, so
    /// inventing one here would produce an object Elastic never emitted.
    #[test]
    fn an_absent_destination_is_not_created() {
        let mut event = Event::new(json!({ "url": { "original": "example.com/a" } }));

        assert!(try_known_painless(&mut event, SCHEMELESS_URL));
        assert!(!event.has("destination"));
    }

    /// The `cisco_ios` timezone chain, as the current pipeline writes it: a lookup
    /// wrapped in a function, whose branches RETURN rather than assign, and
    /// whose last resort sits outside every `if`.
    const TZ_CHAIN: &str = r"String get_timezone(def ctx) {
  if (ctx._temp_?.tz != null) {
    if (ctx._conf?.tz_map != null) {
      for (def item : ctx._conf.tz_map) {
        if (item.tz_short == ctx._temp_.tz) {
          return item.tz_long;
        }
      }
    }
    if (ctx._temp_.tz.length() <= 4) {
      return ctx._temp_.tz.toUpperCase();
    }
    return ctx._temp_.tz;
  }
  if (ctx._conf?.tz_offset != null) {
      ctx.event.timezone = ctx._conf.tz_offset;
      return ctx._conf.tz_offset;
  }
  ctx.event.timezone = 'UTC';
  return 'UTC';
}
def event_timezone = get_timezone(ctx);
";

    /// With no timezone on the line and none configured, the last resort runs.
    #[test]
    fn a_lookup_chain_falls_through_to_its_unguarded_default() {
        let mut event = Event::new(json!({ "_temp_": { "cisco_timestamp": "Jul 14 2023" } }));

        assert!(try_known_painless(&mut event, TZ_CHAIN));
        assert_eq!(event.get("event.timezone"), Some(&json!("UTC")));
    }

    /// A timezone parsed off the line takes the first branch and RETURNS, so
    /// the default must not fire behind it.
    #[test]
    fn a_guard_that_holds_suppresses_the_default() {
        let mut event = Event::new(json!({ "_temp_": { "tz": "CEST" } }));

        assert!(try_known_painless(&mut event, TZ_CHAIN));
        assert!(!event.has("event.timezone"));
    }

    /// The configured offset is a fallback ARM, and it beats the default.
    #[test]
    fn a_configured_offset_wins_over_the_default() {
        let mut event = Event::new(json!({ "_conf": { "tz_offset": "+10:00" } }));

        assert!(try_known_painless(&mut event, TZ_CHAIN));
        assert_eq!(event.get("event.timezone"), Some(&json!("+10:00")));
    }

    #[test]
    fn syslog_priority_decomposes_into_facility_and_severity() {
        // Verbatim from the fortinet transform.
        const PRIORITY: &str = "if (ctx.log?.syslog?.priority != null) {\n  \
             def severity = new HashMap();\n  severity['code'] = ctx.log.syslog.priority&0x7;\n  \
             ctx.log.syslog['severity'] = severity;\n  def facility = new HashMap();\n  \
             facility['code'] = ctx.log.syslog.priority>>3;\n  \
             ctx.log.syslog['facility'] = facility;\n}";

        let mut event = Event::new(json!({ "log": { "syslog": { "priority": 165 } } }));
        assert!(try_known_painless(&mut event, PRIORITY));

        // 165 = local4(20) * 8 + notice(5).
        assert_eq!(event.get_i64("log.syslog.facility.code"), Some(20));
        assert_eq!(event.get_i64("log.syslog.severity.code"), Some(5));

        // The script derives CODES only, so a name is an extra field Elastic
        // never emits -- and every one of them was a diff against the vendor.
        assert!(!event.has("log.syslog.facility.name"));
        assert!(!event.has("log.syslog.severity.name"));
    }

    /// Cisco nexus derives only the facility here; a `set` processor earlier
    /// in the pipeline supplies the severity from the vendor's own field.
    #[test]
    fn syslog_priority_writes_only_the_half_the_script_names() {
        const FACILITY_ONLY: &str = "ctx.log.syslog.facility = new HashMap();\n\
             ctx.log.syslog.facility.code = (ctx.cisco_nexus.log.priority_number - \
             ctx.event.severity)/8;";

        let mut event = Event::new(json!({
            "cisco_nexus": { "log": { "priority_number": 165 } },
            "event": { "severity": 5 },
        }));
        assert!(try_known_painless(&mut event, FACILITY_ONLY));

        assert_eq!(event.get_i64("log.syslog.facility.code"), Some(20));
        assert!(!event.has("log.syslog.severity.code"));
    }

    #[test]
    fn append_unique_skips_duplicates_and_keeps_order() {
        let mut event = Event::new(json!({
            "dns": { "resolved_ip": ["1.1.1.1", "2.2.2.2", "1.1.1.1"] },
            "related": { "ip": ["1.1.1.1"] },
        }));

        assert!(try_known_painless(&mut event, APPEND_DNS));
        assert_eq!(
            event.get("related.ip"),
            Some(&json!(["1.1.1.1", "2.2.2.2"]))
        );
    }

    /// The destination array may not exist yet.
    #[test]
    fn append_unique_creates_the_target_array() {
        let mut event = Event::new(json!({ "dns": { "resolved_ip": ["9.9.9.9"] } }));
        assert!(try_known_painless(&mut event, APPEND_DNS));
        assert_eq!(event.get("related.ip"), Some(&json!(["9.9.9.9"])));
    }

    /// The predicate 245 of the 351 packages spell: null, empty string, empty
    /// collection, in both maps and lists.
    fn drop_everything() -> DropPolicy {
        DropPolicy {
            empty_strings: true,
            empty_collections: true,
            prune_lists: true,
            sentinels: Vec::new(),
        }
    }

    #[test]
    fn drop_empty_removes_nulls() {
        let mut event = Event::new(json!({
            "a": "keep",
            "b": null,
            "c": "",
            "d": {"e": null, "f": "keep"},
            "g": [null, "", "keep"]
        }));
        drop_empty_recursive(&mut event, &drop_everything());
        assert_eq!(event.get_str("a"), Some("keep"));
        assert!(!event.has("b"));
        assert!(!event.has("c"));
        assert!(event.has("d.f"));
        assert!(!event.has("d.e"));
    }

    /// Verbatim from `pipelines/cisco/asa/default.yml`, which 16 packages
    /// share: the predicate is `v == null` and nothing else, so an empty string
    /// and an emptied object both stay.
    const NULL_ONLY: &str = "void handleMap(Map map) {\n  for (def x : map.values()) {\n    \
        if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        \
        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null);\n}\n\
        void handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          \
        handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\n\
        handleMap(ctx);";

    #[test]
    fn a_null_only_predicate_keeps_empty_strings_and_objects() {
        let policy = DropPolicy::read(NULL_ONLY);
        assert_eq!(
            policy,
            DropPolicy {
                empty_strings: false,
                empty_collections: false,
                prune_lists: false,
                sentinels: Vec::new(),
            }
        );

        let mut event = Event::new(json!({
            "a": "keep",
            "b": null,
            "c": "",
            "d": { "e": null },
            "g": [null, "", "keep"],
        }));

        assert!(try_known_painless(&mut event, NULL_ONLY));
        assert!(!event.has("b"), "a null is still dropped");
        assert_eq!(event.get_str("c"), Some(""), "an empty string is not");
        assert!(event.has("d"), "the emptied object stays");
        assert_eq!(
            event.get("g"),
            Some(&json!([null, "", "keep"])),
            "a list this script never prunes is untouched"
        );
    }

    /// The common predicate, read off its own text rather than assumed.
    #[test]
    fn the_full_predicate_reads_as_dropping_everything() {
        let script = "boolean drop(Object o) { if (o == null || o == '') { return true; } \
            else if (o instanceof Map) { ((Map) o).values().removeIf(v -> drop(v)); \
            return ((Map) o).size() == 0; } else if (o instanceof List) { \
            ((List) o).removeIf(v -> drop(v)); return ((List) o).length == 0; } return false; } \
            drop(ctx);";

        assert_eq!(DropPolicy::read(script), drop_everything());
    }

    #[test]
    fn keys_to_snake_case_converts() {
        let mut val = json!({
            "eventType": "login",
            "clientIp": "1.2.3.4",
            "nested": {"displayName": "test"}
        });
        keys_to_snake_case(&mut val);
        assert!(val.get("event_type").is_some());
        assert!(val.get("client_ip").is_some());
        assert!(val.get("eventType").is_none());
    }

    #[test]
    fn extract_process_from_cmd() {
        let mut event = Event::new(json!({
            "crowdstrike": {"event": {"CommandLine": "C:\\Windows\\Explorer.EXE /factory"}}
        }));
        extract_process_fields(&mut event, "crowdstrike.event.CommandLine", "process").unwrap();
        assert_eq!(
            event.get_str("process.command_line"),
            Some("C:\\Windows\\Explorer.EXE /factory")
        );
        assert_eq!(
            event.get_str("process.executable"),
            Some("C:\\Windows\\Explorer.EXE")
        );
    }

    #[test]
    fn epoch_to_iso8601() {
        let mut event = Event::new(json!({"ts": 1_536_846_339}));
        epoch_to_timestamp(&mut event, "ts", "@timestamp").unwrap();
        let ts = event.get_str("@timestamp").unwrap();
        assert!(ts.starts_with("2018-09-13"));
    }

    #[test]
    fn known_painless_drop_nulls() {
        let mut event = Event::new(json!({"a": null, "b": "keep"}));
        let script = r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#;
        assert!(try_known_painless(&mut event, script));
        assert!(!event.has("a"));
        assert!(event.has("b"));
    }

    #[test]
    fn email_split_user() {
        let mut event = Event::new(json!({"user": {"id": "john@example.com"}}));
        let script = r#"String[] splitmail = ctx.user.id.splitOnToken("@"); ctx.user.email = ctx.user.id; ctx.user.domain = splitmail[1]; ctx.user.name = splitmail[0];"#;
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_str("user.email"), Some("john@example.com"));
        assert_eq!(event.get_str("user.name"), Some("john"));
        assert_eq!(event.get_str("user.domain"), Some("example.com"));
    }

    #[test]
    fn email_split_no_at_sign() {
        let mut event = Event::new(json!({"user": {"id": "not-an-email"}}));
        let script = r#"String[] splitmail = ctx.user.id.splitOnToken("@");"#;
        assert!(try_known_painless(&mut event, script));
        // Should not set email/name/domain when no @ present
        assert!(!event.has("user.email"));
    }

    #[test]
    fn email_split_target_user() {
        let mut event = Event::new(json!({"user": {"target": {"id": "admin@corp.io"}}}));
        let script = r#"String[] splitmail = ctx.user.target.id.splitOnToken("@"); ctx.user.target.email = ctx.user.target.id;"#;
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_str("user.target.email"), Some("admin@corp.io"));
        assert_eq!(event.get_str("user.target.name"), Some("admin"));
    }

    #[test]
    fn risk_behaviors_positive() {
        let mut event = Event::new(json!({
            "okta": {"debug_context": {"debug_data": {"flattened": {"behaviors": {
                "New Geo-Location": "POSITIVE",
                "New Device": "NEGATIVE",
                "Velocity": "POSITIVE"
            }}}}}
        }));
        let script = r"if POSITIVE risk_behaviors";
        assert!(try_known_painless(&mut event, script));
        let behaviors = event.get("okta.debug_context.debug_data.risk_behaviors");
        assert!(behaviors.is_some());
        let arr = behaviors.unwrap().as_array().unwrap();
        assert_eq!(arr.len(), 2);
    }

    #[test]
    fn risk_behaviors_none_positive() {
        let mut event = Event::new(json!({
            "okta": {"debug_context": {"debug_data": {"flattened": {"behaviors": {
                "New Device": "NEGATIVE"
            }}}}}
        }));
        let script = r"if POSITIVE risk_behaviors";
        assert!(try_known_painless(&mut event, script));
        // No POSITIVE entries — risk_behaviors should not be set
        assert!(!event.has("okta.debug_context.debug_data.risk_behaviors"));
    }

    #[test]
    fn okta_target_rename_and_extract() {
        let mut event = Event::new(json!({
            "okta": {"target": [
                {"type": "User", "alternateId": "user@test.com", "displayName": "Test User", "id": "001", "detailEntry": {"extra": "removed", "methodTypeUsed": "push"}},
                {"type": "UserGroup", "alternateId": "admins", "displayName": "Admins", "id": "002", "detailEntry": null}
            ]}
        }));
        let script =
            r"def target = ctx.okta.target; alternateId alternate_id displayName display_name okta";
        assert!(try_known_painless(&mut event, script));

        // Check renamed fields
        let target = event.get("okta.target").unwrap().as_array().unwrap();
        let first = target[0].as_object().unwrap();
        assert!(first.contains_key("alternate_id"));
        assert!(first.contains_key("display_name"));
        assert!(!first.contains_key("alternateId"));

        // detailEntry is narrowed in place, keeping its own name.
        let de = first.get("detailEntry").unwrap().as_object().unwrap();
        assert!(de.contains_key("methodTypeUsed"));
        assert!(!de.contains_key("extra"));

        // Check user/group extraction
        assert!(event.has("okta_target_user"));
        assert!(event.has("okta_target_group"));
    }

    #[test]
    fn replace_dots_in_keys_azure_claims() {
        let mut event = Event::new(json!({
            "azure": {"activitylogs": {"identity": {"claims": {
                "http://schemas.microsoft.com/identity/claims/id": "test123",
                "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name": "user"
            }}}}
        }));
        let script = r"if (ctx.azure.activitylogs.identity.claims != null) {\n  ctx.temp_claims = new HashMap();\n  for (String key : ctx.azure.activitylogs.identity.claims.keySet()) {\n    ctx.temp_claims[key.replace('.', '_')] = ctx.azure.activitylogs.identity.claims.get(key);\n  }\n  ctx.azure.activitylogs.identity.claims = ctx.temp_claims; ctx.remove('temp_claims');\n}";
        assert!(try_known_painless(&mut event, script));
        // Verify dots replaced with underscores in claim keys
        let claims = event
            .as_value()
            .pointer("/azure/activitylogs/identity/claims")
            .expect("claims should exist");
        let obj = claims.as_object().expect("claims should be object");
        // Original dotted keys should be replaced
        assert!(!obj.contains_key("http://schemas.microsoft.com/identity/claims/id"));
        assert!(obj.contains_key("http://schemas_microsoft_com/identity/claims/id"));
        assert_eq!(
            obj.get("http://schemas_microsoft_com/identity/claims/id")
                .unwrap(),
            "test123"
        );
    }

    #[test]
    fn azure_event_category_default() {
        let mut event = Event::new(json!({
            "azure": {"activitylogs": {"properties": {}}}
        }));
        let script = r"if (ctx?.azure?.activitylogs?.properties?.eventCategory != null) { ctx.azure.activitylogs.event_category = ctx.azure.activitylogs.properties.eventCategory; } else { ctx.azure.activitylogs.event_category = 'Administrative'; }";
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get_str("azure.activitylogs.event_category"),
            Some("Administrative")
        );
    }

    #[test]
    fn drop_empty_nested_arrays() {
        let mut event = Event::new(json!({
            "keep": "yes",
            "nested": {"arr": [null, "", {"inner": null}]}
        }));
        drop_empty_recursive(&mut event, &drop_everything());
        assert!(event.has("keep"));
        // nested.arr should be empty after removing all null/empty items
        assert!(!event.has("nested"));
    }

    #[test]
    fn keys_to_snake_case_already_snake() {
        let mut val = json!({"already_snake": "yes", "alreadylower": "yes"});
        keys_to_snake_case(&mut val);
        assert!(val.get("already_snake").is_some());
        assert!(val.get("alreadylower").is_some());
    }
}
