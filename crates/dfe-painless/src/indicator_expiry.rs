// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! When a threat indicator stops being worth matching, dated forward from when
//! it was last seen.
//!
//! `ti_cif3` gives each indicator type its own shelf life -- 45 days for an
//! address, 90 for a domain, 365 for a URL or a file hash -- and anything else
//! the lifetime the integration was configured with. The date it lands on
//! becomes `cif3.deleted_at`, and the label becomes `cif3.expiration_duration`.
//!
//! The equality ladder alone reads the LABELS and nothing else, so
//! `cif3.expiration_duration` was right and `cif3.deleted_at` was absent on
//! every event -- the whole source finished on that one field. A `date`
//! processor normalises what this writes, so the string only has to be
//! ISO-8601; it does not have to be the one Java's `toString` would print.

use chrono::{DateTime, SecondsFormat, TimeDelta};
use serde_json::Value;

use crate::params::{balanced, clean_path, literal_writes, skip_trivia};
use dfe_core::event::Event;

/// The calendar unit a `plus...` call adds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
enum Unit {
    Days,
    Hours,
    Minutes,
}

impl Unit {
    /// The call spelling, and the unit it names.
    const CALLS: [(&'static str, Self); 3] = [
        (".plusDays(", Self::Days),
        (".plusHours(", Self::Hours),
        (".plusMinutes(", Self::Minutes),
    ];

    /// `n` of this unit as a span.
    ///
    /// The timestamps carry a fixed offset rather than a named zone, so a day
    /// is 24 hours here -- which is what Java's own `plusDays` works out to in
    /// a zone that never shifts.
    fn span(self, n: i64) -> Option<TimeDelta> {
        match self {
            Self::Days => TimeDelta::try_days(n),
            Self::Hours => TimeDelta::try_hours(n),
            Self::Minutes => TimeDelta::try_minutes(n),
        }
    }

    /// The letter `dfe_core::date_formats::iso8601_plus` names this unit by.
    fn letter(self) -> char {
        match self {
            Self::Days => 'd',
            Self::Hours => 'h',
            Self::Minutes => 'm',
        }
    }
}

/// One indicator type and the shelf life it earns.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct ExpiryArm {
    /// The type values this arm covers.
    types: Vec<String>,
    unit: Unit,
    amount: i64,
    /// The label the arm writes alongside the date.
    label: String,
}

/// The configured fallback, for a type no arm names.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct ConfiguredExpiry {
    /// The `ctx.` path holding a `<n><unit>` duration.
    source: String,
    /// The unit letters the script knows, each with what it means.
    units: Vec<(char, Unit)>,
    /// What an unknown unit falls back to.
    default_unit: Unit,
    default_amount: i64,
    /// The complaint appended when it does, and where.
    message: String,
    message_target: String,
}

/// The whole expiry calculation, read once per call site.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IndicatorExpiry {
    /// The timestamps to date from, in the order the script tests them.
    seen: Vec<String>,
    /// The field naming the indicator's type.
    kind: String,
    arms: Vec<ExpiryArm>,
    configured: ConfiguredExpiry,
    /// Where the label is written.
    duration_target: String,
    /// Where the date is written.
    target: String,
}

/// The first quoted run in `text`.
fn quoted(text: &str) -> Option<&str> {
    let start = text.find(['\'', '"'])?;
    let quote = text.as_bytes()[start] as char;
    let end = text[start + 1..].find(quote)?;
    Some(&text[start + 1..=start + end])
}

/// The `if (<test>) { <block> }` at the head of `text`, and what follows it.
fn if_block(chunk: &str) -> Option<(&str, &str, &str)> {
    let at = chunk.find("if (")?;
    let (test, tail) = balanced(skip_trivia(&chunk[at + "if".len()..]), '(', ')')?;
    let (block, tail) = balanced(skip_trivia(tail), '{', '}')?;
    Some((test, block, tail))
}

/// The `plus...` call in `block`: which unit, and the LITERAL amount.
///
/// `None` where the amount is an expression -- the configured branch computes
/// it from the document, and reading `Long.parseLong(time_value)` as a number
/// would freeze one event's duration into the pattern.
fn literal_span(block: &str) -> Option<(Unit, i64)> {
    let (call, unit) = Unit::CALLS
        .iter()
        .find_map(|(call, unit)| block.contains(call).then_some((*call, *unit)))?;
    let (_, rest) = block.split_once(call)?;
    let amount = rest.split(')').next()?.trim().trim_end_matches(['L', 'l']);
    amount.parse::<i64>().ok().map(|amount| (unit, amount))
}

/// The unit a `plus...` call names, whatever its argument.
fn span_unit(block: &str) -> Option<Unit> {
    Unit::CALLS
        .iter()
        .find_map(|(call, unit)| block.contains(call).then_some(*unit))
}

/// `ctx.<path> == '<literal>'` terms OR'd together, all naming one path.
fn type_test(test: &str) -> Option<(String, Vec<String>)> {
    let mut path: Option<String> = None;
    let mut literals = Vec::new();
    for term in test.split("||") {
        let (lhs, rhs) = term.split_once("==")?;
        let named = clean_path(lhs.trim().strip_prefix("ctx.")?.trim());
        if named.is_empty() || named.contains(['(', ' ', ',']) {
            return None;
        }
        if path.get_or_insert_with(|| named.clone()) != &named {
            return None;
        }
        literals.push(quoted(rhs)?.to_string());
    }
    let path = path?;
    (!literals.is_empty()).then_some((path, literals))
}

/// Read the timestamps, the per-type arms and the configured fallback, or
/// decline.
///
/// Every part is structural and every part is demanded: the dates come from the
/// script's own `ZonedDateTime.parse` calls, each arm from the types it tests
/// and the span it adds, and the fallback from the unit letters the script
/// spells. A script that computes an expiry some other way declines rather than
/// binding to a runner that would date it wrong -- which is worse than leaving
/// the field absent, because a wrong expiry silently retires a live indicator.
pub fn parse_indicator_expiry(script: &str) -> Option<IndicatorExpiry> {
    let mut seen = Vec::new();
    for chunk in script.split("ZonedDateTime.parse(ctx.").skip(1) {
        let path = clean_path(chunk.split(')').next()?.trim());
        if path.is_empty() || path.contains(['(', ' ', ',']) {
            return None;
        }
        seen.push(path);
    }
    if seen.is_empty() {
        return None;
    }

    // `<expiry> = <base>.plusDays(...)` -- the two locals the script turns on.
    let (head, _) = script.split_once(".plus")?;
    let statement = head.rsplit([';', '\n', '{', '}']).next()?.trim();
    let (expiry_local, base_local) = statement.split_once('=')?;
    let expiry_local = expiry_local.trim();
    let base_local = base_local.trim();
    if expiry_local.is_empty() || base_local.is_empty() || expiry_local == base_local {
        return None;
    }

    // `ctx.<target> = <expiry>;`, which is the script's last statement.
    let assigned = format!("= {expiry_local};");
    let at = script.rfind(&assigned)?;
    let target = clean_path(
        script[..at]
            .trim_end()
            .rsplit([' ', '\n', '\t', '{', '}', ';'])
            .next()?
            .strip_prefix("ctx.")?,
    );
    if target.is_empty() {
        return None;
    }

    // The per-type arms. An `if` whose test is not a type comparison is not an
    // arm: the configured branch's own `instanceof` and unit tests fall through
    // here and are read below.
    let mut kind: Option<String> = None;
    let mut duration_target: Option<String> = None;
    let mut arms = Vec::new();
    let mut rest = script;
    while let Some((test, block, tail)) = if_block(rest) {
        rest = tail;
        let Some((path, types)) = type_test(test) else {
            continue;
        };
        let Some((unit, amount)) = literal_span(block) else {
            continue;
        };
        let writes = literal_writes(block);
        let [(written, Value::String(label))] = writes.as_slice() else {
            return None;
        };
        if kind.get_or_insert_with(|| path.clone()) != &path
            || duration_target.get_or_insert_with(|| written.clone()) != written
        {
            return None;
        }
        arms.push(ExpiryArm {
            types,
            unit,
            amount,
            label: label.clone(),
        });
    }
    if arms.is_empty() {
        return None;
    }

    Some(IndicatorExpiry {
        seen,
        kind: kind?,
        arms,
        configured: parse_configured(script, base_local)?,
        duration_target: duration_target?,
        target,
    })
}

/// The `else` branch: the configured duration, the units it accepts, and what
/// an unknown unit falls back to.
fn parse_configured(script: &str, base_local: &str) -> Option<ConfiguredExpiry> {
    // `def <dur> = ctx.<source>; ... if (<dur> instanceof String)`
    let (before, branch) = script.split_once(" instanceof String")?;
    let local = before.trim_end().rsplit([' ', '\n', '\t', '(']).next()?;
    let source = crate::common::ctx_path_bound_to(script, local)?;

    // Each `if (<unit local> == '<letter>') { ... .plus<Unit>(Long.parseLong(...)) }`.
    let mut units = Vec::new();
    let mut fallback = None;
    let mut rest = branch;
    while let Some((test, block, tail)) = if_block(rest) {
        rest = tail;
        if !block.contains(&format!("{base_local}.plus")) {
            continue;
        }
        let Some(letter) = quoted(test).and_then(|held| {
            let mut chars = held.chars();
            chars.next().filter(|_| chars.next().is_none())
        }) else {
            continue;
        };
        let unit = span_unit(block)?;
        units.push((letter, unit));
        // The arm after the last unit is the `else`, which the walk reaches as
        // the same block's tail -- its span is a literal where the units' are
        // parsed from the document.
        if let Some((unit, amount)) = literal_span(tail) {
            fallback = Some((unit, amount, tail));
        }
    }
    if units.is_empty() {
        return None;
    }
    let (default_unit, default_amount, unknown) = fallback?;

    // `ctx.<path>.add('<message>')` -- the complaint that goes with the default.
    let (before, after) = unknown.split_once(".add(")?;
    let message = quoted(after)?.to_string();
    let message_target = clean_path(
        before
            .trim_end()
            .rsplit([' ', '\n', '\t', '{', '}', ';'])
            .next()?
            .strip_prefix("ctx.")?,
    );
    if message.is_empty() || message_target.is_empty() {
        return None;
    }

    Some(ConfiguredExpiry {
        source,
        units,
        default_unit,
        default_amount,
        message,
        message_target,
    })
}

/// Date the indicator forward and write both fields.
///
/// A timestamp neither candidate carries, or one that will not parse, writes
/// nothing: Painless throws there and the call site's own `on_failure` is what
/// answers, so inventing a date would be worse than the absence.
pub fn indicator_expiry(event: &mut Event, pattern: &IndicatorExpiry) -> bool {
    let Some(base) = pattern
        .seen
        .iter()
        .find_map(|path| event.get_str(path))
        .and_then(|text| DateTime::parse_from_rfc3339(text).ok())
    else {
        return true;
    };

    let kind = event.get_string(&pattern.kind).unwrap_or_default();
    let matched = pattern
        .arms
        .iter()
        .find(|arm| arm.types.iter().any(|held| held == &kind));

    let (unit, amount) = if let Some(arm) = matched {
        let _ = event.set(&pattern.duration_target, Value::String(arm.label.clone()));
        (arm.unit, arm.amount)
    } else {
        // The label is the configured value itself, written through whatever
        // type it arrives as, and only read as a duration when it is text.
        let Some(held) = event.get(&pattern.configured.source).cloned() else {
            return true;
        };
        let _ = event.set(&pattern.duration_target, held.clone());
        let Value::String(text) = held else {
            return true;
        };
        let Some(span) = configured_span(event, &pattern.configured, &text) else {
            return true;
        };
        span
    };

    let Some(span) = unit.span(amount) else {
        return true;
    };
    let Some(expiry) = base.checked_add_signed(span) else {
        return true;
    };
    let _ = event.set(
        &pattern.target,
        Value::String(expiry.to_rfc3339_opts(SecondsFormat::Millis, true)),
    );
    true
}

/// The span a `<n><unit>` duration names, complaining where the unit is not one
/// the script knows.
fn configured_span(
    event: &mut Event,
    configured: &ConfiguredExpiry,
    text: &str,
) -> Option<(Unit, i64)> {
    let letter = text.chars().next_back()?;
    let Some(unit) = configured
        .units
        .iter()
        .find_map(|(spelling, unit)| (*spelling == letter).then_some(*unit))
    else {
        // The unknown-unit branch ignores the number entirely and says so.
        let _ = event.append(
            &configured.message_target,
            Value::String(configured.message.clone()),
        );
        return Some((configured.default_unit, configured.default_amount));
    };
    // A number Java's `parseLong` would throw on kills the script before it
    // writes the date, so nothing is written here either.
    let amount = text[..text.len() - letter.len_utf8()].parse::<i64>().ok()?;
    Some((unit, amount))
}

/// The same shelf life, chosen by a FLAG rather than by the indicator's type.
///
/// `ti_strider` dates every indicator ninety days from when it arrives, except
/// an archived one, which expires as it lands.
///
/// Neither reader beside it reaches this. [`IndicatorExpiry`] above demands
/// per-TYPE arms and a configured fallback; `DatePlusDays` in
/// [`crate::common`] reads the single-expression form and declines this one
/// because the base is bound to a LOCAL before the adder sees it. Nothing
/// claimed the script, so `expires_at` was absent on every indicator carrying a
/// timestamp.
///
/// Both arms are read off the script rather than assumed: which one shortens
/// the life is the vendor's to say, and a matcher that assumed the flagged arm
/// expires the record would silently invert a source that means the opposite.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FlaggedExpiry {
    /// The timestamp both arms date from.
    base: String,
    /// The field the script tests, and the value that selects the first arm.
    flag: String,
    wanted: Value,
    /// The span each arm adds, `None` where the arm writes the base as it
    /// stands.
    matched: Option<(Unit, i64)>,
    unmatched: Option<(Unit, i64)>,
    /// Where the date lands. Both arms write it.
    target: String,
}

/// Date the record forward by whichever arm the flag selects.
///
/// A base the event does not carry, or one that will not parse, writes
/// nothing: Painless throws in `ZonedDateTime.parse` and the processor's own
/// `if` is what guards it, so inventing a date would be worse than the absence.
pub fn flagged_expiry(event: &mut Event, pattern: &FlaggedExpiry) -> bool {
    let Some(base) = event.get_string(&pattern.base) else {
        return true;
    };
    // The literal comparison a guard makes: absent and explicitly null are one
    // value, and nothing else compares equal to a number or a word.
    let selected = event.get(&pattern.flag) == Some(&pattern.wanted);
    let arm = if selected {
        pattern.matched
    } else {
        pattern.unmatched
    };
    // Zero days through the same renderer, so the unmoved arm is written in the
    // same words as the moved one rather than echoed back as it arrived.
    let (unit, amount) = arm.unwrap_or((Unit::Days, 0));
    if let Some(moved) = dfe_core::date_formats::iso8601_plus(&base, unit.letter(), amount, 0) {
        let _ = event.set(&pattern.target, Value::String(moved));
    }
    true
}

/// Read the base, the flag and both arms off the script, or decline.
///
/// Whole-script: one parse into a local, then one `if`/`else` and nothing after
/// it. A script that also writes elsewhere declines rather than being claimed
/// and half run.
#[must_use]
pub fn parse_flagged_expiry(script: &str) -> Option<FlaggedExpiry> {
    // `ZonedDateTime <local> = ZonedDateTime.parse(ctx[...]);`
    let (head, tail) = script.split_once("ZonedDateTime.parse(")?;
    let local = head
        .rsplit_once('=')?
        .0
        .trim()
        .rsplit(char::is_whitespace)
        .next()?;
    if local.is_empty() {
        return None;
    }
    let (subject, rest) = tail.split_once(");")?;
    // `ctx['@timestamp']` or `ctx.a.b` -- the subscript is how a script names
    // the one field no dotted path can spell.
    let base = clean_path(
        subject
            .trim()
            .strip_prefix("ctx")?
            .trim_start_matches('.')
            .trim_matches(['[', ']', '\'', '"']),
    );
    if base.is_empty() || base.contains([' ', '(', ')']) {
        return None;
    }

    let rest = skip_trivia(skip_trivia(rest).strip_prefix("if")?);
    let (test, tail) = balanced(rest, '(', ')')?;
    let (then, tail) = balanced(skip_trivia(tail), '{', '}')?;
    let tail = skip_trivia(skip_trivia(tail).strip_prefix("else")?);
    let (alt, tail) = balanced(tail, '{', '}')?;
    if !skip_trivia(tail).is_empty() {
        return None;
    }

    let (flag, wanted) = flag_test(test)?;
    let (matched_target, matched) = arm_write(then, local)?;
    let (unmatched_target, unmatched) = arm_write(alt, local)?;
    // Two arms writing different fields is a different script, and one where
    // neither adds a span is a copy rather than an expiry.
    if matched_target != unmatched_target || (matched.is_none() && unmatched.is_none()) {
        return None;
    }

    Some(FlaggedExpiry {
        base,
        flag,
        wanted,
        matched,
        unmatched,
        target: matched_target,
    })
}

/// `ctx.<path> == <literal>` as the field and the value it is compared against.
fn flag_test(test: &str) -> Option<(String, Value)> {
    let (subject, wanted) = test.split_once("==")?;
    let path = clean_path(subject.trim().strip_prefix("ctx.")?);
    if path.is_empty() || path.contains([' ', '(', ')']) {
        return None;
    }
    Some((path, crate::common::painless_literal(wanted.trim())?))
}

/// One arm: the path it writes, and the span it adds on the way.
fn arm_write(block: &str, local: &str) -> Option<(String, Option<(Unit, i64)>)> {
    let (statement, tail) = skip_trivia(block).split_once(';')?;
    if !skip_trivia(tail).is_empty() {
        return None;
    }
    let (assigned, written) = statement.split_once('=')?;
    let target = clean_path(assigned.trim().strip_prefix("ctx.")?);
    if target.is_empty() || target.contains([' ', '(', ')']) {
        return None;
    }

    let written = written.trim();
    if written == local {
        return Some((target, None));
    }
    let rest = written.strip_prefix(local)?;
    let (call, unit) = Unit::CALLS
        .iter()
        .find_map(|(call, unit)| rest.starts_with(call).then_some((*call, *unit)))?;
    let amount: i64 = rest[call.len()..]
        .split(')')
        .next()?
        .trim()
        .trim_end_matches(['L', 'l'])
        .parse()
        .ok()?;
    Some((target, Some((unit, amount))))
}

#[cfg(all(test, dfe_script_tests))]
#[rustfmt::skip]
#[allow(clippy::unwrap_used, clippy::panic)]
#[path = "indicator_expiry_tests.rs"]
mod tests;
