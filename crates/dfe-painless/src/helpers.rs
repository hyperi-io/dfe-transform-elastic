// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Painless semantics as Rust functions.
//!
//! An Elastic ingest pipeline's Painless scripts are dynamically typed against
//! a JSON context; these bridge that to `serde_json::Value` so the transform
//! modules can express the same semantics natively. All functions are pure --
//! no I/O, no side effects beyond operating on the provided values.

use serde_json::{Map, Value, json};

/// Painless truthiness: `null`/`false`/`0`/`""` → false, everything else → true.
///
/// Matches Painless/Java boolean coercion semantics.
#[inline]
pub fn painless_truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i != 0
            } else if let Some(f) = n.as_f64() {
                f != 0.0
            } else {
                true
            }
        }
        Value::String(s) => !s.is_empty(),
        Value::Array(_) | Value::Object(_) => true,
    }
}

/// Painless addition: string concat if either operand is a string, else numeric.
pub fn painless_add(a: &Value, b: &Value) -> Value {
    // String concatenation takes priority (Painless/Java behaviour)
    if a.is_string() || b.is_string() {
        let a_str = painless_to_string(a);
        let b_str = painless_to_string(b);
        return json!(format!("{a_str}{b_str}"));
    }

    // Numeric addition
    let a_num = painless_to_f64(a);
    let b_num = painless_to_f64(b);
    let result = a_num + b_num;

    // Preserve integer type if both inputs are integers
    if a.is_i64() && b.is_i64() {
        json!(a.as_i64().unwrap_or(0) + b.as_i64().unwrap_or(0))
    } else if a.is_u64() && b.is_u64() {
        json!(a.as_u64().unwrap_or(0) + b.as_u64().unwrap_or(0))
    } else {
        json!(result)
    }
}

/// Painless subtraction.
pub fn painless_sub(a: &Value, b: &Value) -> Value {
    if a.is_i64() && b.is_i64() {
        json!(a.as_i64().unwrap_or(0) - b.as_i64().unwrap_or(0))
    } else {
        json!(painless_to_f64(a) - painless_to_f64(b))
    }
}

/// Painless multiplication.
pub fn painless_mul(a: &Value, b: &Value) -> Value {
    if a.is_i64() && b.is_i64() {
        json!(a.as_i64().unwrap_or(0) * b.as_i64().unwrap_or(0))
    } else {
        json!(painless_to_f64(a) * painless_to_f64(b))
    }
}

/// Painless division (integer division for integers, float otherwise).
pub fn painless_div(a: &Value, b: &Value) -> Value {
    let b_val = painless_to_f64(b);
    if b_val == 0.0 {
        return Value::Null;
    }
    if a.is_i64() && b.is_i64() {
        json!(a.as_i64().unwrap_or(0) / b.as_i64().unwrap_or(1))
    } else {
        json!(painless_to_f64(a) / b_val)
    }
}

/// Painless modulo.
pub fn painless_mod(a: &Value, b: &Value) -> Value {
    let b_val = painless_to_i64(b);
    if b_val == 0 {
        return Value::Null;
    }
    json!(painless_to_i64(a) % b_val)
}

/// Convert a `Value` to `i64`. Handles strings, floats, bools.
pub fn painless_to_i64(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n
            .as_i64()
            .unwrap_or_else(|| n.as_f64().unwrap_or(0.0) as i64),
        Value::String(s) => s.parse::<i64>().unwrap_or(0),
        Value::Bool(b) => i64::from(*b),
        _ => 0,
    }
}

/// Convert a `Value` to `f64`.
pub fn painless_to_f64(v: &Value) -> f64 {
    match v {
        Value::Number(n) => n.as_f64().unwrap_or(0.0),
        Value::String(s) => s.parse::<f64>().unwrap_or(0.0),
        Value::Bool(b) if *b => 1.0,
        _ => 0.0,
    }
}

/// Convert a `Value` to its string representation.
///
/// Matches Painless `toString()` semantics.
pub fn painless_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => java_number_to_string(n),
        // Painless is Java, so a map renders `{k=v, k=v}` and a list
        // `[a, b]` -- not their JSON. aws's cloudtrail keeps a rendered copy
        // of `requestParameters` in exactly that pattern.
        Value::Array(_) | Value::Object(_) => java_to_string(v),
    }
}

/// One JSON number as Java renders it.
///
/// `String.valueOf(double)` leaves decimal notation for the magnitudes in
/// `[10^-3, 10^7)` and writes everything outside it in computerized scientific
/// notation, so bbot's epoch `1709170907.779394` is stored by Elasticsearch as
/// `1.709170907779394E9`. An INTEGER is a Java Long and has no scientific form,
/// so only a number serde could not hold as an integer takes this route.
fn java_number_to_string(n: &serde_json::Number) -> String {
    if n.is_i64() || n.is_u64() {
        return n.to_string();
    }
    let Some(value) = n.as_f64() else {
        return n.to_string();
    };
    java_double_to_string(value)
}

/// A finite `f64` as Java's `Double.toString` writes it.
fn java_double_to_string(value: f64) -> String {
    if !value.is_finite() {
        return if value.is_nan() {
            "NaN".to_string()
        } else if value > 0.0 {
            "Infinity".to_string()
        } else {
            "-Infinity".to_string()
        };
    }

    let magnitude = value.abs();
    if magnitude == 0.0 {
        return if value.is_sign_negative() {
            "-0.0".to_string()
        } else {
            "0.0".to_string()
        };
    }
    if (1e-3..1e7).contains(&magnitude) {
        // Java always writes at least one digit either side of the point.
        let decimal = format!("{value}");
        return if decimal.contains(['.', 'e', 'E']) {
            decimal
        } else {
            format!("{decimal}.0")
        };
    }

    // Rust's `{:e}` gives `1.709170907779394e9`; Java spells the exponent `E`
    // and keeps the mandatory `.0` on a whole mantissa.
    let scientific = format!("{value:e}");
    let (mantissa, exponent) = scientific.split_once('e').unwrap_or((&scientific, "0"));
    let mantissa = if mantissa.contains('.') {
        mantissa.to_string()
    } else {
        format!("{mantissa}.0")
    };
    format!("{mantissa}E{exponent}")
}

/// Render one value the way Elasticsearch's ingest MUSTACHE does.
///
/// The same as Painless's `toString` but for an ARRAY: mustache reaches the
/// document through a handler that presents a list as a map keyed by its
/// indices, so `{{package.name}}` over `["python-requests"]` renders
/// `{0=python-requests}` and not `[python-requests]`. Verbatim from
/// inspector's `event.id`, which concatenates four such fields.
#[must_use]
pub fn template_to_string(v: &Value) -> String {
    use std::fmt::Write as _;

    // Mustache renders a null the same way it renders an absent field: as
    // NOTHING. Painless's `toString` gives the four letters, and appending
    // those put a literal "null" into m365's related.user and device.id.
    if v.is_null() {
        return String::new();
    }
    let Value::Array(items) = v else {
        return painless_to_string(v);
    };
    let mut out = String::from("{");
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        // Cannot fail: writing to a String.
        let _ = write!(out, "{index}={}", template_to_string(item));
    }
    out.push('}');
    out
}

/// A value as Java's own `toString`, which is not JSON.
///
/// `AbstractMap` writes `{key=value, key=value}` with no quotes anywhere, and
/// `AbstractCollection` writes `[a, b]`. The MEMBER ORDER is a plain
/// `HashMap`'s -- Elasticsearch parses an ingest document's maps with
/// `XContentParser.map()`, not the ordered variant -- so the entries walk the
/// hash table: bucket index ascending, insertion order within a bucket. The
/// corpus's rendered cloudtrail copies are the proof, sorted keys and
/// document order both diverging from it.
#[must_use]
pub fn java_to_string(v: &Value) -> String {
    java_to_string_sized(v, &mut String::new(), &|_| None)
}

/// `java_to_string`, reading each map's table from the entry count it held
/// when it was BUILT rather than the count that survives.
///
/// A Java `HashMap` never shrinks its table on remove, so a map a pipeline's
/// empty-value prune shrank still renders through its original buckets --
/// which reorders the members whenever the prune crossed a table boundary.
/// `capacity` answers that count for a map at a document path, and `path` is
/// where in the document `v` itself sits.
pub fn java_to_string_sized(
    v: &Value,
    path: &mut String,
    capacity: &dyn Fn(&str) -> Option<usize>,
) -> String {
    match v {
        Value::Object(map) => {
            let table = java_table_size(capacity(path).unwrap_or_else(|| map.len()));
            let mut entries: Vec<(usize, usize, &String, &Value)> = map
                .iter()
                .enumerate()
                .map(|(position, (key, value))| (java_bucket(key, table), position, key, value))
                .collect();
            entries.sort_by_key(|(bucket, position, ..)| (*bucket, *position));
            let mark = path.len();
            let members: Vec<String> = entries
                .into_iter()
                .map(|(_, _, key, value)| {
                    push_segment(path, mark, key);
                    format!("{key}={}", java_to_string_sized(value, path, capacity))
                })
                .collect();
            path.truncate(mark);
            format!("{{{}}}", members.join(", "))
        }
        Value::Array(items) => {
            let mark = path.len();
            let members: Vec<String> = items
                .iter()
                .enumerate()
                .map(|(index, item)| {
                    push_segment(path, mark, &index.to_string());
                    java_to_string_sized(item, path, capacity)
                })
                .collect();
            path.truncate(mark);
            format!("[{}]", members.join(", "))
        }
        other => painless_to_string(other),
    }
}

/// Replace whatever sits past `mark` with one more dotted segment.
fn push_segment(path: &mut String, mark: usize, segment: &str) {
    path.truncate(mark);
    if !path.is_empty() {
        path.push('.');
    }
    path.push_str(segment);
}

/// The table size a default-capacity Java `HashMap` holds `entries` in:
/// 16 doubling whenever the count crosses three quarters of it.
pub(crate) fn java_table_size(entries: usize) -> usize {
    let mut capacity = 16usize;
    while entries > capacity * 3 / 4 {
        capacity *= 2;
    }
    capacity
}

/// The bucket a key lands in: Java's `String.hashCode` over UTF-16 units,
/// spread by `h ^ (h >>> 16)` and masked to the table.
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)] // Java's own int arithmetic, wrap included.
pub(crate) fn java_bucket(key: &str, table: usize) -> usize {
    let mut hash: i32 = 0;
    for unit in key.encode_utf16() {
        hash = hash.wrapping_mul(31).wrapping_add(i32::from(unit));
    }
    let spread = hash ^ ((hash as u32) >> 16) as i32;
    (spread as u32 as usize) & (table - 1)
}

/// Painless equality — null-safe, with type coercion for numbers.
pub fn painless_eq(a: &Value, b: &Value) -> bool {
    if a == b {
        return true;
    }
    // Coerce numeric types for cross-type comparison
    if a.is_number() && b.is_number() {
        return painless_to_f64(a) == painless_to_f64(b);
    }
    // Compare string to number
    if a.is_string()
        && b.is_number()
        && let Ok(n) = a.as_str().unwrap_or("").parse::<f64>()
    {
        return n == painless_to_f64(b);
    }
    if a.is_number()
        && b.is_string()
        && let Ok(n) = b.as_str().unwrap_or("").parse::<f64>()
    {
        return painless_to_f64(a) == n;
    }
    false
}

/// Painless comparison — returns ordering for `<`, `<=`, `>`, `>=`.
pub fn painless_cmp(a: &Value, b: &Value) -> Option<std::cmp::Ordering> {
    let a_f = painless_to_f64(a);
    let b_f = painless_to_f64(b);
    a_f.partial_cmp(&b_f)
}

/// Elastic's `ignore_empty_value`: null, or an empty string.
///
/// An empty array or object is a REAL value and is written -- the vendor
/// expectations carry `[]` and `{}` on Azure's properties, and treating them
/// as empty is how they went missing.
#[inline]
#[must_use]
pub fn painless_is_empty_value(value: &Value) -> bool {
    matches!(value, Value::Null) || value.as_str() == Some("")
}

/// Recursive removal of null and empty values from a `Value` tree.
///
/// Used by okta, `cisco_nexus`, and fortinet `drop` scripts.
/// Returns `true` if the value itself should be removed.
pub fn painless_drop_empty(v: &mut Value) -> bool {
    match v {
        Value::Null => true,
        Value::String(s) if s.is_empty() => true,
        Value::Object(map) => {
            let keys_to_remove: Vec<String> = map
                .iter_mut()
                .filter_map(|(k, v)| {
                    if painless_drop_empty(v) {
                        Some(k.clone())
                    } else {
                        None
                    }
                })
                .collect();
            for k in keys_to_remove {
                map.shift_remove(&k);
            }
            map.is_empty()
        }
        Value::Array(arr) => {
            arr.retain_mut(|item| !painless_drop_empty(item));
            arr.is_empty()
        }
        _ => false,
    }
}

/// Remove entries from a JSON object whose values match sentinel values.
///
/// Used by `CrowdStrike` and other pipelines that use Painless scripts like:
/// ```painless
/// ctx.crowdstrike.event.entrySet().removeIf(
///     entry -> params.values.contains(entry.getValue())
/// );
/// ```
///
/// The `sentinels` parameter contains values to remove (e.g., `null`, `""`, `"-"`, `"NA"`, `0`).
pub fn remove_sentinel_values(obj: &mut Map<String, Value>, sentinels: &[Value]) {
    obj.retain(|_, v| !sentinels.contains(v));
}

/// Convert a Windows FILETIME / LDAP timestamp to UNIX epoch milliseconds.
///
/// Windows FILETIME uses 100-nanosecond intervals since 1601-01-01.
/// Values above `0x0100000000000000` (72057594037927936) are FILETIME;
/// smaller values are already UNIX timestamps (seconds or milliseconds).
///
/// Used by `CrowdStrike` for `StartTime`, `EndTime`, `ContextTimeStamp`, etc.
/// Reference: <https://devblogs.microsoft.com/oldnewthing/20030905-02/?p=42653>
#[inline]
pub fn filetime_to_unix_ms(value: i64) -> i64 {
    const FILETIME_THRESHOLD: i64 = 0x0100_0000_0000_0000; // 72057594037927936
    const FILETIME_TO_UNIX_OFFSET_MS: i64 = 11_644_473_600_000; // ms between 1601 and 1970

    if value > FILETIME_THRESHOLD {
        (value / 10_000) - FILETIME_TO_UNIX_OFFSET_MS
    } else {
        value
    }
}

/// Deduplicate a JSON array in-place, preserving order.
///
/// Used after multiple `append` calls that may produce duplicates
/// (e.g., related.ip being appended from both source.ip and destination.ip
/// when they're the same address).
pub fn dedup_array(arr: &mut Vec<Value>) {
    let mut seen = Vec::with_capacity(arr.len());
    arr.retain(|v| {
        if seen.contains(v) {
            false
        } else {
            seen.push(v.clone());
            true
        }
    });
}

/// Recursive camelCase-to-snake_case key renaming on a `Value` tree.
///
/// Used by `azure_signinlogs` `keysToSnakeCase` script.
pub fn painless_keys_to_snake_case(v: &Value) -> Value {
    match v {
        Value::Object(map) => {
            let mut out = Map::new();
            for (k, v) in map {
                let snake_key = camel_to_snake(k);
                let converted_v = if v.is_object() {
                    painless_keys_to_snake_case(v)
                } else if let Some(arr) = v.as_array() {
                    Value::Array(
                        arr.iter()
                            .map(|item| {
                                if item.is_object() {
                                    painless_keys_to_snake_case(item)
                                } else {
                                    item.clone()
                                }
                            })
                            .collect(),
                    )
                } else {
                    v.clone()
                };
                out.insert(snake_key, converted_v);
            }
            Value::Object(out)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(painless_keys_to_snake_case).collect()),
        _ => v.clone(),
    }
}

/// Where a snake-case conversion puts its underscores.
///
/// The integrations COPY this helper between packages rather than share it, so
/// the copies have diverged. Seven of them disagree in ways the captured output
/// shows, which is why each is kept and the rule is read off the script's own
/// body -- never off the source name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnakeRule {
    /// Underscore only at a lowercase-to-uppercase transition, so
    /// `HTTPServer` stays one word.
    OnWordBreak,
    /// Underscore before every uppercase character after the first.
    BeforeEveryUpper,
    /// Underscore before an uppercase character whose predecessor was not
    /// itself uppercase. This is the rule the integrations' own `camelToSnake`
    /// helper implements -- `lastCharWasUpperCase` is set by an uppercase
    /// char and cleared by ANY other, so a digit or a dot before an uppercase
    /// still breaks the word where [`Self::OnWordBreak`] would not.
    AfterNonUpper,
    /// Underscore before the LAST uppercase of a run, which is where the next
    /// word starts: `HTTPServer` is `http_server`. cyberarkpas and the
    /// packages that copied its helper walk the run with a counter and move
    /// the separator back once they see the run end.
    AcronymRun,
    /// The separator goes in front of an uppercase only where a LETTER or DIGIT
    /// precedes it, and an uppercase run keeps its acronym whole unless a
    /// lowercase letter follows: `HTTPServer` is `http_server`,
    /// `IT_Administrators` is `it_administrators`, `cve2021Id` is `cve2021_id`.
    ///
    /// beyondtrust's `epm` and `isi` ship it as a LOOKAHEAD rather than a run
    /// counter, which is what keeps a run ended by `_`, `-`, `.`, a space or the
    /// end of the string intact. [`Self::AcronymRun`] breaks all of those and
    /// writes `i_t__administrators`.
    AcronymRunStrict,
    /// Underscore before an uppercase RUN that follows a lowercase, and an
    /// underscore already sitting in front of that lowercase is DROPPED because
    /// the match consumes it: `tag_aB` is `taga_b`.
    ///
    /// This is the regex `_?([a-z])([A-Z]+)` -> `$1_$2` that several packages
    /// write instead of walking the characters. It needs a lowercase character
    /// in FRONT of the run, so `HTTPServer` matches nothing and only loses its
    /// case -- where [`Self::BeforeEveryUpper`] writes `h_t_t_p_server`.
    CamelBreak,
    /// [`Self::CamelBreak`] without the regex's leading `_?`, so an underscore
    /// in front of the break is KEPT: `tag_aB` is `tag_a_b`. azure's
    /// `signinlogs` and cloudflare's `workers_trace` spell it this way.
    CamelBreakKeepingUnderscore,
}

/// Convert a string to `snake_case` under `rule`.
///
/// Lowercasing goes through the full `char::to_lowercase` mapping. Taking only
/// its first character drops the rest, and some codepoints lowercase to more
/// than one -- U+0130 becomes `i` plus a combining dot.
#[must_use]
pub fn to_snake_case(s: &str, rule: SnakeRule) -> String {
    // The rules that scan for a RUN cannot be expressed by the per-character
    // loop below, so each returns from its own walk.
    match rule {
        SnakeRule::AcronymRun => return acronym_run_snake(s),
        SnakeRule::AcronymRunStrict => return acronym_run_strict_snake(s),
        SnakeRule::CamelBreak => return camel_break(s, true).to_lowercase(),
        SnakeRule::CamelBreakKeepingUnderscore => return camel_break(s, false).to_lowercase(),
        SnakeRule::OnWordBreak | SnakeRule::BeforeEveryUpper | SnakeRule::AfterNonUpper => {}
    }

    let mut result = String::with_capacity(s.len() + 4);
    let mut prev_was_lowercase = false;
    let mut prev_was_uppercase = false;
    let mut first = true;

    for ch in s.chars() {
        if ch.is_uppercase() {
            let separate = match rule {
                SnakeRule::OnWordBreak => prev_was_lowercase,
                SnakeRule::BeforeEveryUpper => !first,
                SnakeRule::AfterNonUpper => !first && !prev_was_uppercase,
                SnakeRule::AcronymRun
                | SnakeRule::AcronymRunStrict
                | SnakeRule::CamelBreak
                | SnakeRule::CamelBreakKeepingUnderscore => {
                    unreachable!("the run rules return above, before this loop")
                }
            };
            if separate {
                result.push('_');
            }
            result.extend(ch.to_lowercase());
        } else {
            result.push(ch);
        }
        prev_was_lowercase = ch.is_lowercase();
        prev_was_uppercase = ch.is_uppercase();
        first = false;
    }

    result
}

/// The vendor's own `to_snake_case`, run-counter and all.
///
/// It walks a RUN of uppercase and, when the run ends, moves the separator to
/// sit before the run's LAST character -- that character starts the next word.
/// `MessageID` is `message_id`, and reading it as one underscore per uppercase
/// gave `message_i_d`, which the `rename` to `event.code` then missed.
///
/// This is cyberarkpas's rule and it is CORRECT as written -- the run's
/// separator moves back whatever ends the run. Narrowing that to "ends at a
/// lowercase letter", plus refusing to double a separator, costs cyberarkpas an
/// event and 5 fields, so it is NOT a fix to make here. beyondtrust spells that
/// narrower rule as a helper of its own and [`SnakeRule::AcronymRunStrict`]
/// carries it, bound off the script text rather than off the source.
fn acronym_run_snake(s: &str) -> String {
    // The script's own fast path: nothing after the first character is
    // uppercase, so there is no word to break.
    if !s.chars().skip(1).any(char::is_uppercase) {
        return s.to_lowercase();
    }

    let mut result = String::with_capacity(s.len() + 4);
    let mut run = 0usize;
    let mut first = true;
    for ch in s.chars() {
        if ch.is_uppercase() {
            if run == 0 && !first {
                result.push('_');
            }
            run += 1;
        } else {
            if run > 1
                && let Some(last) = result.pop()
            {
                result.push('_');
                result.push(last);
            }
            run = 0;
            first = false;
        }
        result.extend(ch.to_lowercase());
    }
    result
}

/// beyondtrust's `camelToSnake`, which LOOKS AHEAD instead of counting a run.
///
/// The vendor's own condition, verbatim from `epm`'s and `isi`'s pipelines:
///
/// ```text
/// char prev = str.charAt(i - 1);
/// boolean nextIsLower = (i + 1 < str.length()) && Character.isLowerCase(str.charAt(i + 1));
/// boolean prevIsDigit = Character.isDigit(prev);
/// if (Character.isLowerCase(prev) || prevIsDigit || (Character.isUpperCase(prev) && nextIsLower)) {
///   result.append('_');
/// }
/// ```
///
/// Every predecessor outside those three arms -- `_`, `-`, `.`, a space, the
/// start of the string -- writes nothing, which is what keeps a separator from
/// doubling and leaves a run the string ends on whole.
///
/// Two divergences from Java: `char::to_lowercase` is the full mapping where
/// `Character.toLowerCase(char)` is one codepoint, the convention
/// [`to_snake_case`] already sets; and `char::is_numeric` admits Unicode
/// `Nl`/`No` besides the `Nd` that `Character.isDigit` means.
fn acronym_run_strict_snake(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    let mut chars = s.chars().peekable();
    let mut prev: Option<char> = None;

    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            // `prev` is None only at i == 0, which is the vendor's `i > 0`.
            if let Some(prev) = prev {
                let next_is_lower = chars.peek().is_some_and(|next| next.is_lowercase());
                if prev.is_lowercase()
                    || prev.is_numeric()
                    || (prev.is_uppercase() && next_is_lower)
                {
                    result.push('_');
                }
            }
            result.extend(ch.to_lowercase());
        } else {
            // Appended UNCHANGED, so a separator already in the key survives.
            result.push(ch);
        }
        prev = Some(ch);
    }

    result
}

/// `([a-z])([A-Z]+)` replaced by `$1_$2`, the way Java's matcher walks it.
///
/// `eat_underscore` is the regex's leading `_?`. It is part of the MATCH and
/// the replacement does not write it back, so with the flag set `aB_cD` comes
/// out `a_Bc_D` and not `a_B_c_D`. The character classes are ASCII because the
/// regex's are: a non-ASCII uppercase does not start a run.
///
/// Does NOT lowercase -- the vendors spell that as a separate step, and
/// [`KeyRewriteStep::Lowercase`](crate::common::KeyRewriteStep) may or
/// may not follow.
pub(crate) fn camel_break(key: &str, eat_underscore: bool) -> String {
    let chars: Vec<char> = key.chars().collect();
    let mut out = String::with_capacity(key.len() + 4);
    let mut i = 0;
    while i < chars.len() {
        let mut at = i;
        if eat_underscore && chars[at] == '_' {
            at += 1;
        }
        if at < chars.len() && chars[at].is_ascii_lowercase() {
            let mut end = at + 1;
            while end < chars.len() && chars[end].is_ascii_uppercase() {
                end += 1;
            }
            if end > at + 1 {
                out.push(chars[at]);
                out.push('_');
                out.extend(&chars[at + 1..end]);
                i = end;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

/// Convert a camelCase or `PascalCase` string to `snake_case`.
fn camel_to_snake(s: &str) -> String {
    to_snake_case(s, SnakeRule::OnWordBreak)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Verbatim from the bbot corpus, whose `timestamp` is converted to a
    /// string: Elasticsearch stored the epoch `1709170907.779394` as
    /// `1.709170907779394E9`, because Java leaves decimal notation at 10^7.
    #[test]
    fn a_double_renders_the_way_java_renders_it() {
        assert_eq!(
            painless_to_string(&json!(1_709_170_907.779_394)),
            "1.709170907779394E9"
        );
        assert_eq!(
            painless_to_string(&json!(1.703_170_919_403_808e9)),
            "1.703170919403808E9"
        );

        // Inside [10^-3, 10^7) Java stays decimal, and always writes a digit
        // either side of the point.
        assert_eq!(painless_to_string(&json!(1234.5)), "1234.5");
        assert_eq!(painless_to_string(&json!(0.001)), "0.001");
        assert_eq!(painless_to_string(&json!(1.0)), "1.0");

        // Below the lower bound it goes scientific again.
        assert_eq!(painless_to_string(&json!(0.000_123)), "1.23E-4");

        // An INTEGER is a Java Long and never takes the scientific form, which
        // is what keeps every whole number in the tree rendering as itself.
        assert_eq!(painless_to_string(&json!(1_709_170_907_i64)), "1709170907");
        assert_eq!(painless_to_string(&json!(0)), "0");
        assert_eq!(painless_to_string(&json!(-42)), "-42");
    }

    /// Verbatim from the inspector corpus: mustache sees a list through a
    /// handler that keys it by index, so a template over one renders
    /// `{0=a, 1=b}` where Painless's own `toString` gives `[a, b]`.
    #[test]
    fn a_template_renders_a_list_keyed_by_index() {
        assert_eq!(
            template_to_string(&json!(["python-requests"])),
            "{0=python-requests}"
        );
        assert_eq!(
            template_to_string(&json!(["golang.org/x/net", "nerdctl"])),
            "{0=golang.org/x/net, 1=nerdctl}"
        );
        assert_eq!(template_to_string(&json!([])), "{}");

        // A null renders as NOTHING, the way an absent field does -- Painless
        // gives the four letters and appending those is a literal "null".
        assert_eq!(template_to_string(&Value::Null), "");

        // Everything else renders exactly as Painless does.
        assert_eq!(template_to_string(&json!("plain")), "plain");
        assert_eq!(template_to_string(&json!(11_111_111)), "11111111");
        assert_eq!(template_to_string(&json!({ "a": 1 })), "{a=1}");
    }

    /// `char::to_lowercase` yields an ITERATOR because some codepoints
    /// lowercase to more than one character. Taking only the first silently
    /// drops the rest, so a field name loses characters.
    #[test]
    fn camel_to_snake_keeps_every_character_of_a_lowercase_mapping() {
        // U+0130 LATIN CAPITAL LETTER I WITH DOT ABOVE lowercases to two
        // characters: 'i' + U+0307 COMBINING DOT ABOVE.
        assert_eq!(camel_to_snake("\u{0130}"), "i\u{0307}");
        assert_eq!(camel_to_snake("a\u{0130}"), "a_i\u{0307}");
    }

    /// Non-ASCII uppercase must be treated as uppercase, and the result must
    /// never lose or reorder characters.
    #[test]
    fn camel_to_snake_handles_non_ascii_scripts() {
        // Cyrillic and Greek have case; CJK and Arabic do not.
        assert_eq!(camel_to_snake("привет"), "привет");
        assert_eq!(camel_to_snake("日本語"), "日本語");
        assert_eq!(camel_to_snake("العربية"), "العربية");
        assert_eq!(camel_to_snake("userИмя"), "user_имя");
        assert_eq!(camel_to_snake("\u{00DF}"), "\u{00DF}");
    }

    /// The ASCII behaviour this is actually used for must not move.
    #[test]
    fn camel_to_snake_preserves_the_ascii_rule() {
        assert_eq!(camel_to_snake("userName"), "user_name");
        assert_eq!(camel_to_snake("UserName"), "user_name");
        assert_eq!(camel_to_snake("HTTPServer"), "httpserver");
        assert_eq!(camel_to_snake("already_snake"), "already_snake");
        assert_eq!(camel_to_snake(""), "");
    }

    /// A long key must not cost quadratic time. The old implementation called
    /// `chars().nth(i - 1)` on every character.
    #[test]
    fn camel_to_snake_is_linear_on_a_long_key() {
        let long = "aB".repeat(20_000);
        let out = camel_to_snake(&long);
        assert_eq!(out.chars().filter(|c| *c == '_').count(), 20_000);
    }

    /// The second rule shares the same lowercase mapping, so it must not drop
    /// characters either.
    #[test]
    fn before_every_upper_rule_keeps_the_full_lowercase_mapping() {
        assert_eq!(
            to_snake_case("a\u{0130}", SnakeRule::BeforeEveryUpper),
            "a_i\u{0307}"
        );
        assert_eq!(
            to_snake_case("HTTPServer", SnakeRule::BeforeEveryUpper),
            "h_t_t_p_server"
        );
        assert_eq!(
            to_snake_case("userName", SnakeRule::BeforeEveryUpper),
            "user_name"
        );
    }

    /// The two rules must stay distinct: collapsing them changes the key names
    /// the Elastic fixtures are matched against.
    #[test]
    fn the_two_rules_disagree_on_acronyms() {
        assert_ne!(
            to_snake_case("HTTPServer", SnakeRule::OnWordBreak),
            to_snake_case("HTTPServer", SnakeRule::BeforeEveryUpper)
        );
    }

    #[test]
    fn truthiness() {
        assert!(!painless_truthy(&Value::Null));
        assert!(!painless_truthy(&json!(false)));
        assert!(!painless_truthy(&json!(0)));
        assert!(!painless_truthy(&json!("")));
        assert!(painless_truthy(&json!(true)));
        assert!(painless_truthy(&json!(1)));
        assert!(painless_truthy(&json!("hello")));
        assert!(painless_truthy(&json!([])));
        assert!(painless_truthy(&json!({})));
    }

    #[test]
    fn addition_numeric() {
        assert_eq!(painless_add(&json!(2), &json!(3)), json!(5));
        assert_eq!(painless_add(&json!(2.5), &json!(1.5)), json!(4.0));
    }

    #[test]
    fn addition_string_concat() {
        assert_eq!(
            painless_add(&json!("hello"), &json!(" world")),
            json!("hello world")
        );
        assert_eq!(
            painless_add(&json!("count: "), &json!(42)),
            json!("count: 42")
        );
    }

    #[test]
    fn multiplication() {
        assert_eq!(painless_mul(&json!(3), &json!(4)), json!(12));
        assert_eq!(
            painless_mul(&json!(1_000_000), &json!(1_000_000_000_i64)),
            json!(1_000_000_000_000_000_i64)
        );
    }

    #[test]
    fn division_integer() {
        assert_eq!(painless_div(&json!(10), &json!(3)), json!(3));
        assert_eq!(painless_div(&json!(10), &json!(0)), Value::Null);
    }

    #[test]
    fn to_i64_conversions() {
        assert_eq!(painless_to_i64(&json!(42)), 42);
        assert_eq!(painless_to_i64(&json!("123")), 123);
        assert_eq!(painless_to_i64(&json!(3.7)), 3);
        assert_eq!(painless_to_i64(&json!(true)), 1);
        assert_eq!(painless_to_i64(&Value::Null), 0);
    }

    #[test]
    fn to_string_conversions() {
        assert_eq!(painless_to_string(&json!("hello")), "hello");
        assert_eq!(painless_to_string(&json!(42)), "42");
        assert_eq!(painless_to_string(&json!(true)), "true");
        assert_eq!(painless_to_string(&Value::Null), "null");
    }

    #[test]
    fn equality() {
        assert!(painless_eq(&json!(1), &json!(1)));
        assert!(painless_eq(&json!(1), &json!(1.0)));
        assert!(painless_eq(&json!("hello"), &json!("hello")));
        assert!(painless_eq(&Value::Null, &Value::Null));
        assert!(!painless_eq(&json!(1), &json!(2)));
        assert!(!painless_eq(&json!("a"), &json!("b")));
    }

    #[test]
    fn drop_empty_recursive() {
        let mut val = json!({
            "a": null,
            "b": "",
            "c": "keep",
            "d": {
                "e": null,
                "f": ""
            },
            "g": [null, "", "keep"]
        });
        painless_drop_empty(&mut val);
        assert_eq!(
            val,
            json!({
                "c": "keep",
                "g": ["keep"]
            })
        );
    }

    #[test]
    fn filetime_to_unix_conversion() {
        // Windows FILETIME for 2023-11-02T10:36:00.000Z
        let ft = 133_433_949_600_000_000_i64;
        let unix_ms = filetime_to_unix_ms(ft);
        let dt = chrono::DateTime::from_timestamp_millis(unix_ms).unwrap();
        assert_eq!(
            dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
            "2023-11-02T10:36:00.000Z"
        );
    }

    #[test]
    fn filetime_passthrough_unix() {
        // Already UNIX milliseconds (should pass through)
        let unix_ms = 1_698_918_960_000_i64;
        assert_eq!(filetime_to_unix_ms(unix_ms), unix_ms);
    }

    #[test]
    fn remove_sentinels() {
        let mut map = serde_json::from_value::<Map<String, Value>>(json!({
            "keep": "valid",
            "zero": 0,
            "empty": "",
            "na": "NA",
            "dash": "-",
            "null_val": null,
            "also_keep": 42
        }))
        .unwrap();
        let sentinels = vec![
            Value::Null,
            json!(""),
            json!("-"),
            json!("N/A"),
            json!("NA"),
            json!(0),
        ];
        remove_sentinel_values(&mut map, &sentinels);
        assert_eq!(map.len(), 2);
        assert!(map.contains_key("keep"));
        assert!(map.contains_key("also_keep"));
    }

    #[test]
    fn dedup_array_values() {
        let mut arr = vec![json!("a"), json!("b"), json!("a"), json!("c"), json!("b")];
        dedup_array(&mut arr);
        assert_eq!(arr, vec![json!("a"), json!("b"), json!("c")]);
    }

    #[test]
    fn dedup_array_preserves_order() {
        let mut arr = vec![json!(3), json!(1), json!(2), json!(1), json!(3)];
        dedup_array(&mut arr);
        assert_eq!(arr, vec![json!(3), json!(1), json!(2)]);
    }

    #[test]
    fn keys_to_snake_case_conversion() {
        let input = json!({
            "camelCaseIsCool": "value",
            "PascalCaseIsCooler": {
                "nestedInsideObject": [{"NestedInsideArray": true}]
            }
        });
        let result = painless_keys_to_snake_case(&input);
        assert_eq!(
            result,
            json!({
                "camel_case_is_cool": "value",
                "pascal_case_is_cooler": {
                    "nested_inside_object": [{"nested_inside_array": true}]
                }
            })
        );
    }

    #[test]
    fn camel_to_snake_examples() {
        assert_eq!(camel_to_snake("camelCase"), "camel_case");
        assert_eq!(camel_to_snake("PascalCase"), "pascal_case");
        assert_eq!(camel_to_snake("already_snake"), "already_snake");
        assert_eq!(camel_to_snake("HTTPResponse"), "httpresponse");
        assert_eq!(camel_to_snake("simple"), "simple");
    }
}
