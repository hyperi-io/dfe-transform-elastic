// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Common Painless script patterns implemented in Rust.
//!
//! The same handful of script patterns recur across the Elastic pipelines --
//! drop-empty, snake-case keys, sum both directions -- so they are written
//! once here rather than once per source. [`try_known_painless`] matches a
//! script against them and runs the Rust equivalent.

use std::borrow::Cow;

use serde_json::{Map, Value, json};

use crate::error::Result;
use crate::event::Event;
use crate::painless_helpers::{SnakeRule, to_snake_case};
use crate::painless_params::{Program, clean_path};

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
/// [`normalise`] a borrow from then on. Same pattern as [`crate::cached_grok`].
#[macro_export]
macro_rules! cached_script {
    ($script:literal $(,)?) => {{
        static SITE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
        SITE.get_or_init(|| $crate::painless_common::normalise($script).into_owned())
            .as_str()
    }};
}

/// Read `<local> = ctx.<a>.substring(ctx.<b>.length())` and where the local
/// finally lands, as a [`KnownPattern::PrefixTail`].
fn parse_prefix_tail(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let at = script.find(".substring(ctx.")?;
    let before = &script[..at];
    let source = clean_path(&before[before.rfind("ctx.")? + 4..]);
    let after = &script[at + ".substring(ctx.".len()..];
    let (prefix, _) = after.split_once(".length())")?;
    let prefix = clean_path(prefix);

    // The local the tail is bound to: the last word before the `=`.
    let assignment = before.rfind('=')?;
    let local = before[..assignment].split_whitespace().next_back()?;

    // Where the local is finally stored: `ctx.<target> = <local>;`, last.
    let store = format!(" = {local};");
    let store_at = script.rfind(&store)?;
    let head = &script[..store_at];
    let target = clean_path(&head[head.rfind("ctx.")? + 4..]);
    if target == source {
        return None;
    }

    Some(KnownPattern::PrefixTail {
        source,
        prefix,
        strip_comma: script.contains(".startsWith(',')"),
        target,
    })
}

/// The tail of `source` past `prefix`'s length, one leading comma dropped
/// where the script does, stored at `target`.
fn try_prefix_tail(
    event: &mut Event,
    source: &str,
    prefix: &str,
    strip_comma: bool,
    target: &str,
) -> bool {
    let (Some(text), Some(prefix)) = (event.get_str(source), event.get_str(prefix)) else {
        return true;
    };
    let Some(mut tail) = text.get(prefix.len()..) else {
        return true;
    };
    if strip_comma {
        tail = tail.strip_prefix(',').unwrap_or(tail);
    }
    let tail = tail.to_string();
    let _ = event.set(target, tail);
    true
}

/// Read `x.add(ctx.<scalar>); for (v in ctx.<array>) { x.add(v); }
/// ctx.<target> = x;` as a [`KnownPattern::PrependToArray`].
fn parse_prepend_to_array(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let add_at = script.find(".add(ctx.")?;
    let after = &script[add_at + ".add(ctx.".len()..];
    let (scalar, _) = after.split_once(')')?;
    let scalar = clean_path(scalar);

    let for_at = script.find(" in ctx.")?;
    let after = &script[for_at + " in ctx.".len()..];
    let (array, _) = after.split_once(')')?;
    let array = clean_path(array);

    // The local built up: the word before `.add(ctx.`.
    let head = &script[..add_at];
    let local = head
        .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()?;
    let store = format!(" = {local};");
    let store_at = script.rfind(&store)?;
    let head = &script[..store_at];
    let target = clean_path(&head[head.rfind("ctx.")? + 4..]);

    Some(KnownPattern::PrependToArray {
        scalar,
        array,
        target,
    })
}

/// `target = [scalar] + array's elements`, the reconstruction half of the
/// identities dance.
fn try_prepend_to_array(event: &mut Event, scalar: &str, array: &str, target: &str) -> bool {
    let Some(first) = event.get(scalar).cloned() else {
        return true;
    };
    let Some(Value::Array(rest)) = event.get(array).cloned() else {
        return true;
    };
    let mut out = Vec::with_capacity(rest.len() + 1);
    out.push(first);
    out.extend(rest);
    let _ = event.set(target, Value::Array(out));
    true
}

/// The fields a `splitStr` batch names, as dotted paths.
///
/// The base is the `def <ss> = ctx.<base>;` binding, and each call names a
/// member of it -- `splitStr(ss, 'key')` or `splitStr(ss.sub, 'key')`.
fn parse_split_pipe_fields(script: &str) -> Option<Vec<String>> {
    use crate::painless_params::clean_path;

    // The one local bound to a ctx path that the calls pass.
    let (local, base) = local_and_ctx_path(script)?;

    let mut fields = Vec::new();
    for call in script.split("splitStr(").skip(1) {
        let Some((arguments, _)) = call.split_once(')') else {
            continue;
        };
        // The helper's own definition has typed parameters, not a call.
        let Some((holder, key)) = arguments.split_once(',') else {
            continue;
        };
        let holder = holder.trim();
        let Some(key) = quoted_first(key) else {
            continue;
        };
        if holder == local {
            fields.push(format!("{base}.{key}"));
        } else if let Some(sub) = holder.strip_prefix(&format!("{local}.")) {
            fields.push(format!("{base}.{}.{key}", clean_path(sub)));
        }
    }
    (!fields.is_empty()).then_some(fields)
}

/// Read the one-field token split: subject binding, separator, optional
/// `Integer.parseInt`, and the list's final store.
fn parse_split_token_field(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let (local, source) = local_and_ctx_path(script)?;
    let call = format!("{local}.splitOnToken(");
    let at = script.find(&call)?;
    let separator = quoted_first(&script[at + call.len()..])?;
    if separator.chars().count() != 1 {
        return None;
    }

    // The ArrayList the loop fills, and where it is stored.
    let list_decl = script.find("= new ArrayList()")?;
    let list = script[..list_decl].split_whitespace().next_back()?;
    let store = format!(" = {list};");
    let store_at = script.rfind(&store)?;
    let before = &script[..store_at];
    let target = clean_path(&before[before.rfind("ctx.")? + 4..]);

    // In place only: a script storing the list somewhere ELSE does more than
    // this pattern, and claiming it would write a wrong array.
    if target != source {
        return None;
    }

    // A SECOND split inside the loop, whose first piece is what the script
    // keeps: zscaler's dictionary names arrive as `<name>: <description>`.
    let head = script[at + call.len()..]
        .split_once(".splitOnToken(")
        .and_then(|(_, rest)| quoted_first(rest))
        .filter(|_| script.contains("[0]"))
        .and_then(|sep| (sep.chars().count() == 1).then(|| sep.chars().next()))
        .flatten();

    Some(KnownPattern::SplitTokenField(Box::new(SplitToken {
        source,
        separator: separator.chars().next()?,
        parse_int: script.contains("Integer.parseInt("),
        target,
        head,
        // The loop spelling keeps interior empties and always stores.
        drop_empty: false,
        remove_if_empty: false,
    })))
}

/// The same split, written as a stream: `Stream.of(ctx.<path>.splitOnToken(
/// '<sep>')).filter(s -> !s.isEmpty()).collect(Collectors.toList())`, stored
/// back in place, and the field REMOVED when nothing survives.
///
/// `ti_anomali` uses it for a value its own separator fences (`,10015,`),
/// which is exactly what the loop spelling above cannot express -- Java's
/// split eats a trailing empty but keeps a leading one.
fn parse_stream_split_filter(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let (_, rest) = script.split_once("Stream.of(ctx.")?;
    let (path, after) = rest.split_once(".splitOnToken(")?;
    let source = clean_path(path.trim());
    let separator = quoted_first(after)?;
    let mut separator = separator.chars();
    let (Some(separator), None) = (separator.next(), separator.next()) else {
        return None;
    };

    // In place only. A script storing the list elsewhere does more than this.
    let stored = crate::painless_params::ctx_writes(script)
        .into_iter()
        .any(|(target, _)| target == source);
    if source.is_empty() || !stored || !script.contains(".isEmpty()") {
        return None;
    }

    Some(KnownPattern::SplitTokenField(Box::new(SplitToken {
        target: source.clone(),
        source,
        separator,
        parse_int: false,
        head: None,
        drop_empty: true,
        // `if (lst.size() > 0) { ... } else { ctx.<path>.remove(...) }`.
        remove_if_empty: script.contains(".remove("),
    })))
}

/// Split each named field's string on `|` in place, empties kept, exactly as
/// the `splitStr` helper does -- a string with no pipe becomes a one-element
/// list.
fn run_split_pipe_fields(event: &mut Event, fields: &[String]) -> bool {
    for field in fields {
        if let Some(text) = event.get_str(field).map(str::to_string)
            && !text.is_empty()
        {
            let pieces: Vec<Value> = text
                .split('|')
                .map(|p| Value::String(p.to_string()))
                .collect();
            let _ = event.set(field, Value::Array(pieces));
        }
    }
    true
}

/// One field split on its token into a list, in place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SplitToken {
    source: String,
    separator: char,
    /// Keep only the pieces that parse as 32-bit integers --
    /// `Integer.parseInt`'s range, since Painless skips the ones that throw.
    parse_int: bool,
    target: String,
    /// A second separator whose FIRST piece is what each member keeps.
    head: Option<char>,
    /// Drop EVERY empty piece, not just the trailing ones Java's split eats.
    /// The stream spelling filters on `!s.isEmpty()`, which is how
    /// `ti_anomali` handles a value fenced by its own separator (`,10015,`).
    drop_empty: bool,
    /// Remove the field outright when nothing survives the filter. Paired with
    /// `drop_empty` in the stream spelling, and separate because "keep no
    /// empties" and "delete the field" are two decisions.
    remove_if_empty: bool,
}

fn run_split_token_field(event: &mut Event, pattern: &SplitToken) -> bool {
    let &SplitToken {
        ref source,
        separator,
        parse_int,
        ref target,
        head,
        drop_empty,
        remove_if_empty,
    } = pattern;

    if let Some(text) = event.get_str(source).map(str::to_string) {
        let mut pieces: Vec<&str> = text.split(separator).collect();
        if drop_empty {
            // The stream spelling filters on `!s.isEmpty()`, so a LEADING
            // empty goes too -- which Java's split keeps.
            pieces.retain(|piece| !piece.is_empty());
        } else {
            // Java's split drops trailing empty pieces.
            while pieces.last() == Some(&"") {
                pieces.pop();
            }
        }
        // A second split whose FIRST piece is what the script keeps --
        // zscaler's `<name>: <description>` dictionary entries.
        if let Some(head) = head {
            pieces = pieces
                .iter()
                .map(|piece| piece.split(head).next().unwrap_or(piece))
                .collect();
        }
        let values: Vec<Value> = if parse_int {
            pieces
                .iter()
                .filter_map(|p| p.parse::<i32>().ok())
                .map(|n| Value::from(i64::from(n)))
                .collect()
        } else {
            pieces
                .iter()
                .map(|p| Value::String((*p).to_string()))
                .collect()
        };
        // The vendor's `if (lst.size() > 0)` -- an empty result deletes the
        // field rather than storing an empty array.
        if values.is_empty() && remove_if_empty {
            event.remove(target);
        } else {
            let _ = event.set(target, Value::Array(values));
        }
    }
    true
}

/// A string trimmed into one field, and its pieces into a list in another.
///
/// `cisco_secure_endpoint` carries the whole command line as one string under
/// `command_line.arguments` and owes ECS both forms of it:
/// `process.command_line` verbatim and `process.args` as the pieces. One
/// script, because the trim happens once and both targets carry it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TrimThenSplit {
    /// The `ctx.` path the string is read from.
    source: String,
    /// Where the trimmed string is stored.
    text: String,
    /// Where its non-empty pieces are stored, as a list.
    list: String,
    /// The one character the split cuts on.
    separator: char,
}

/// The single ORDINARY character a Painless regex literal cuts on, read off
/// `/<c>/.split(`.
///
/// A regex is a language, and this reads one character of it. `/\s+/` cuts in
/// places a space never does, so anything the engine reads as SYNTAX is
/// declined rather than taken for the character it looks like.
fn regex_split_char(script: &str) -> Option<char> {
    let at = script.find("/.split(")?;
    let open = script[..at].rfind('/')?;
    let mut body = script[open + 1..at].chars();
    let (Some(c), None) = (body.next(), body.next()) else {
        return None;
    };
    (!r"\^$.|?*+()[]{}/".contains(c)).then_some(c)
}

/// Read the trim-then-split pair off the script.
///
/// Every clause the vendor wrote changes what is stored -- the trim, the
/// empty-string guard, the per-piece empty drop, and the `size() > 0` guard
/// that keeps an empty list unwritten -- so all four have to be there. A
/// script missing one of them stores something else, and claiming it would
/// write the wrong value under the right name.
fn parse_trim_then_split(script: &str) -> Option<TrimThenSplit> {
    let (local, source) = local_and_ctx_path(script)?;
    let separator = regex_split_char(script)?;

    // The trim, the guard that keeps an all-space value unstored, and the
    // split reading the trimmed local rather than some other string. Painless
    // takes either quote for the empty literal.
    let empty_test = |name: &str| {
        script.contains(&format!("{name} != \"\"")) || script.contains(&format!("{name} != ''"))
    };
    if !script.contains(&format!("{local} = {local}.trim()"))
        || !empty_test(&local)
        || !script.contains(&format!(".split({local})"))
    {
        return None;
    }

    // The list the loop fills, the loop variable, the per-piece empty drop,
    // and the guard before the list is stored.
    let declaration = script.find("= [];")?;
    let pieces = script[..declaration].split_whitespace().next_back()?;
    let item = script
        .split_once("for (")?
        .1
        .split_once(':')?
        .0
        .trim()
        .rsplit(' ')
        .next()?;
    if !script.contains(&format!("{pieces}.add({item})"))
        || !empty_test(item)
        || !script.contains(&format!("{pieces}.size() > 0"))
    {
        return None;
    }

    // Exactly two ctx writes, the trimmed string FIRST: a third write is a
    // script doing more than this pattern, and the order is what the stored
    // document comes back in.
    let writes = crate::painless_params::ctx_writes(script);
    let [(text, holds_text), (list, holds_list)] = writes.as_slice() else {
        return None;
    };
    (holds_text == &local && holds_list == pieces && text != list).then(|| TrimThenSplit {
        source,
        text: text.clone(),
        list: list.clone(),
        separator,
    })
}

/// Store the trimmed string, then its non-empty pieces.
///
/// The pieces go in SECOND because that is the order the script writes them,
/// and the map preserves insertion order.
fn run_trim_then_split(event: &mut Event, pattern: &TrimThenSplit) -> bool {
    let Some(text) = event.get_str(&pattern.source) else {
        // The script's own `!= null` guard: nothing read, nothing written.
        return false;
    };
    let text = text.trim().to_string();
    if text.is_empty() {
        return false;
    }
    let pieces: Vec<Value> = text
        .split(pattern.separator)
        .filter(|piece| !piece.is_empty())
        .map(|piece| Value::String(piece.to_owned()))
        .collect();
    let _ = event.set(&pattern.text, Value::String(text));
    if !pieces.is_empty() {
        let _ = event.set(&pattern.list, Value::Array(pieces));
    }
    true
}

/// Base64-decode one field into another; text that will not decode is left
/// alone, which is where Elastic's engine throws to `on_failure` instead.
fn run_decode_base64(event: &mut Event, source: &str, target: &str) -> bool {
    use base64::Engine as _;
    if let Some(text) = event.get_str(source).map(str::to_string)
        && let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(&text)
        && let Ok(decoded) = String::from_utf8(bytes)
    {
        let _ = event.set(target, json!(decoded));
    }
    true
}

/// Store a split's token count, Java's trailing-empty drop included.
fn run_token_count(event: &mut Event, source: &str, separator: &str, target: &str) -> bool {
    if let Some(text) = event.get_str(source).map(str::to_string) {
        let mut pieces: Vec<&str> = text.split(separator).collect();
        while pieces.last() == Some(&"") {
            pieces.pop();
        }
        let count = i64::try_from(pieces.len()).unwrap_or(i64::MAX);
        let _ = event.set(target, json!(count));
    }
    true
}

/// Trim every string member of a list, in place.
fn run_trim_list(event: &mut Event, field: &str) -> bool {
    let Some(Value::Array(members)) = event.get(field) else {
        return true;
    };
    let trimmed: Vec<Value> = members
        .iter()
        .map(|member| match member {
            Value::String(text) => json!(text.trim()),
            other => other.clone(),
        })
        .collect();
    let _ = event.set(field, Value::Array(trimmed));
    true
}

/// Append a constant to a list once any member of another list starts with the
/// script's prefix -- cloudfront's `localhost:8080` becoming `127.0.0.1`.
fn run_starts_with_append(
    event: &mut Event,
    source: &str,
    prefix: &str,
    target: &str,
    value: &str,
) -> bool {
    let Some(Value::Array(members)) = event.get(source) else {
        return true;
    };
    let hits = members
        .iter()
        .filter(|member| member.as_str().is_some_and(|text| text.starts_with(prefix)))
        .count();
    if hits == 0 {
        return true;
    }

    let mut list = match event.get(target) {
        Some(Value::Array(existing)) => existing.clone(),
        _ => Vec::new(),
    };
    // Once per matching member, which is what the loop does -- no dedup.
    list.extend(std::iter::repeat_n(json!(value), hits));
    let _ = event.set(target, Value::Array(list));
    true
}

/// A guarded concatenation: each clause contributes only when every field it
/// names is there and non-empty, and an empty result is written nowhere.
fn run_concat_parts(event: &mut Event, script: &ConcatScript) -> bool {
    let mut built = String::new();
    for clause in &script.clauses {
        let mut piece = String::new();
        let mut complete = true;
        for term in clause {
            match term {
                ConcatTerm::Literal(text) => piece.push_str(text),
                ConcatTerm::Field(path) => match event.get(path) {
                    Some(Value::String(text)) if !text.is_empty() => piece.push_str(text),
                    Some(Value::Number(n)) => piece.push_str(&n.to_string()),
                    Some(Value::Bool(b)) => piece.push_str(if *b { "true" } else { "false" }),
                    _ => complete = false,
                },
            }
        }
        if complete {
            built.push_str(&piece);
        }
    }
    if !built.is_empty() {
        let _ = event.set(&script.target, json!(built));
    }
    true
}

/// elb's `tlsv12` split at the `v`: the head is the protocol, the tail the
/// version, dotted after its first digit when it does not already carry one.
/// A token that does not split in two leaves the event alone.
fn run_tls_version_split(event: &mut Event, source: &str) -> bool {
    let Some(raw) = event.get_str(source).map(str::to_string) else {
        return true;
    };
    let parts: Vec<&str> = raw.split('v').collect();
    if parts.len() != 2 {
        return true;
    }
    let version = if parts[1].contains('.') {
        parts[1].to_string()
    } else {
        let mut chars = parts[1].chars();
        match chars.next() {
            Some(first) => format!("{first}.{}", chars.as_str()),
            None => return true,
        }
    };
    let _ = event.set("tls.version", json!(version));
    let _ = event.set("tls.version_protocol", json!(parts[0].to_lowercase()));
    true
}

/// cloudtrail's `ConsoleLogin` extras: three of `additionalEventData`'s keys
/// under `console_login.additional_eventdata`, two of them as booleans.
///
/// The vendor reads `MobileVersion` and `MFAUsed` as `!= 'No'`, so anything
/// that is not the literal `No` -- `Yes` included -- is true. Nothing is
/// written at all unless at least one of the three is present, and the whole
/// script returns early on any other `eventName`.
fn run_console_login_event_data(event: &mut Event) -> bool {
    if event.get_str("json.eventName") != Some("ConsoleLogin") {
        return true;
    }

    let mut aed = Map::new();
    let mut read = |source: &str, target: &str, as_bool: bool| {
        let Some(value) = event.get(source).filter(|v| !v.is_null()) else {
            return;
        };
        let stored = if as_bool {
            json!(value.as_str() != Some("No"))
        } else {
            value.clone()
        };
        aed.insert(target.to_string(), stored);
    };
    read(
        "json.additionalEventData.MobileVersion",
        "mobile_version",
        true,
    );
    read("json.additionalEventData.LoginTo", "login_to", false);
    read("json.additionalEventData.MFAUsed", "mfa_used", true);

    if !aed.is_empty() {
        let _ = event.set(
            "aws.cloudtrail.console_login.additional_eventdata",
            Value::Object(aed),
        );
    }
    true
}

/// checkpoint's dropped-packet tuples: each `<ip,port,ip,port,proto;iface>`
/// entry becomes a structured map, the sampled marker is noted, and the raw
/// field goes once anything parsed. A port that will not parse is where the
/// script's own parseLong threw, so the remaining writes stop there.
fn run_checkpoint_packets(event: &mut Event) -> bool {
    let Some(raw) = event.get_str("checkpoint.packets").map(str::to_string) else {
        return true;
    };
    let mut text = raw.trim().to_string();
    if text.starts_with("(sample")
        && let Some(close) = text.find(')')
    {
        let _ = event.set("checkpoint.packets_data_is_sampled", json!(true));
        text = text[close + 1..].trim().to_string();
    }
    if let Some(stripped) = text.strip_suffix("\";") {
        text = stripped.to_string();
    }

    let mut parsed: Vec<Value> = Vec::new();
    for entry in text.split('>') {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        let entry = entry.strip_prefix('<').unwrap_or(entry);
        let mut packet = Map::new();
        let mut parts = entry.split(';');
        let tuple = parts.next().unwrap_or("");
        if let Some(interface) = parts.next() {
            packet.insert("interface".into(), json!({ "name": interface }));
        }
        let fields: Vec<&str> = tuple.split(',').collect();
        if fields.len() >= 5 {
            let (Ok(src_port), Ok(dst_port)) = (fields[1].parse::<i64>(), fields[3].parse::<i64>())
            else {
                return true;
            };
            packet.insert(
                "source".into(),
                json!({ "ip": fields[0], "port": src_port }),
            );
            packet.insert(
                "destination".into(),
                json!({ "ip": fields[2], "port": dst_port }),
            );
            packet.insert("network".into(), json!({ "iana_number": fields[4] }));
            parsed.push(Value::Object(packet));
        }
    }

    if !parsed.is_empty() {
        let _ = event.set("checkpoint.packets_dropped", Value::Array(parsed));
        event.remove("checkpoint.packets");
    }
    true
}

/// The multi-resource sibling: the same dispatch with every write an
/// APPEND, so each field becomes an array across the finding's resources.
#[allow(clippy::too_many_lines)] // A transliteration, as its single sibling is.
fn run_securityhub_multi(event: &mut Event, resources: &[Value]) -> bool {
    for path in [
        "resource.type",
        "resource.id",
        "resource.name",
        "user.name",
        "user.id",
        "host.id",
        "host.ip",
        "host.name",
        "orchestrator.type",
        "orchestrator.cluster.id",
        "orchestrator.cluster.name",
        "orchestrator.cluster.version",
        "orchestrator.resource.id",
        "orchestrator.resource.name",
        "orchestrator.resource.type",
        "cloud.instance.id",
        "cloud.instance.name",
        "cloud.service.name",
        "cloud.availability_zone",
    ] {
        if !event.has_value(path) {
            let _ = event.set(path, json!([]));
        }
    }

    for res in resources {
        let kind = res
            .get("Type")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let _ = event.append(
            "resource.type",
            res.get("Type").cloned().unwrap_or(Value::Null),
        );
        let _ = event.append("resource.id", res.get("Id").cloned().unwrap_or(Value::Null));
        let Some(id) = res.get("Id").and_then(Value::as_str).map(str::to_string) else {
            return true;
        };
        let tokens: Vec<&str> = id.split(':').collect();

        let details = res.get("Details");
        let detail = |member: &str| -> Option<&Value> {
            details
                .and_then(|d| d.get(&kind))
                .and_then(|t| t.get(member))
        };
        let res_name = detail("Name").and_then(Value::as_str).map_or_else(
            || (*tokens.last().unwrap_or(&"")).to_string(),
            str::to_string,
        );
        let _ = event.append("resource.name", json!(res_name.clone()));

        if details.is_some() {
            for (wanted, member, target) in [
                ("AwsIamUser", "UserName", "user.name"),
                ("AwsIamAccessKey", "UserName", "user.name"),
                ("AwsS3Bucket", "OwnerName", "user.name"),
                ("AwsIamUser", "UserId", "user.id"),
                ("AwsS3Bucket", "OwnerId", "user.id"),
                ("AwsEcsContainer", "Name", "host.name"),
            ] {
                if kind == wanted
                    && let Some(v) = detail(member).cloned()
                {
                    let _ = event.append(target, v);
                }
            }
            if kind == "AwsEc2Instance" {
                for member in ["IpV4Addresses", "IpV6Addresses"] {
                    if let Some(Value::Array(addresses)) = detail(member) {
                        for address in addresses.clone() {
                            if address.is_string() {
                                let _ = event.append("host.ip", address);
                            }
                        }
                    }
                }
            }
            if matches!(kind.as_str(), "AwsEcsCluster" | "AwsEcsTask")
                && let Some(v) = details
                    .and_then(|d| d.get("AwsEcsCluster"))
                    .and_then(|t| t.get("ClusterArn"))
                    .cloned()
            {
                let _ = event.append("orchestrator.cluster.id", v);
            }
            for (member, target) in [
                ("Arn", "orchestrator.cluster.id"),
                ("Name", "orchestrator.cluster.name"),
                ("Version", "orchestrator.cluster.version"),
            ] {
                if kind == "AwsEksCluster"
                    && let Some(v) = detail(member).cloned()
                {
                    let _ = event.append(target, v);
                }
            }
            if kind == "AwsEcsCluster"
                && let Some(v) = detail("ClusterName").cloned()
            {
                let _ = event.append("orchestrator.cluster.name", v);
            }
            if matches!(
                kind.as_str(),
                "AwsEc2Subnet" | "AwsRedshiftCluster" | "AwsDmsReplicationInstance"
            ) && let Some(v) = detail("AvailabilityZone").cloned()
            {
                let _ = event.append("cloud.availability_zone", v);
            }
            if matches!(
                kind.as_str(),
                "AwsEc2VpcEndpointService" | "AwsElbLoadBalancer" | "AwsRdsDbCluster"
            ) && let Some(Value::Array(zones)) = detail("AvailabilityZones")
            {
                for zone in zones.clone() {
                    let _ = event.append("cloud.availability_zone", zone);
                }
            }
            if kind == "AwsAutoScalingAutoScalingGroup"
                && let Some(Value::Array(zones)) = detail("AvailabilityZones")
            {
                for zone in zones.clone() {
                    if let Some(v) = zone.get("Value").cloned() {
                        let _ = event.append("cloud.availability_zone", v);
                    }
                }
            }
            if kind == "AwsElbv2LoadBalancer"
                && let Some(Value::Array(zones)) = detail("AvailabilityZones")
            {
                for zone in zones.clone() {
                    if let Some(v) = zone.get("ZoneName").cloned() {
                        let _ = event.append("cloud.availability_zone", v);
                    }
                }
            }
        }

        if kind == "AwsEc2Instance" {
            let _ = event.append("host.id", json!(id.clone()));
            let _ = event.append("cloud.instance.id", json!(id.clone()));
            let _ = event.append("cloud.instance.name", json!(res_name.clone()));
        }
        if kind.starts_with("AwsEks") || kind.starts_with("AwsEcs") {
            let _ = event.append("orchestrator.resource.id", json!(id.clone()));
            let _ = event.append("orchestrator.resource.name", json!(res_name));
            let _ = event.append("orchestrator.resource.type", json!(kind.clone()));
            let orchestrator = if kind.starts_with("AwsEks") {
                "kubernetes"
            } else {
                "ecs"
            };
            let _ = event.append("orchestrator.type", json!(orchestrator));
        }
        if tokens.len() > 2 {
            let _ = event.append("cloud.service.name", json!(tokens[2]));
        }
    }
    true
}

/// m365's process and file fields off the alert evidence list, transliterated.
///
/// Every collection is a `HashSet` the script then SORTS, and an executable is
/// the image file's path and name joined by whichever separator the path
/// already uses. A one-member executable set is written as a scalar and a
/// larger one as a list, which is the script's own distinction.
fn run_m365_process_evidence(event: &mut Event, source: &str) -> bool {
    let Some(Value::Array(evidence)) = event.get(source).cloned() else {
        return true;
    };

    let mut executables: Vec<String> = Vec::new();
    let mut parent_executables: Vec<String> = Vec::new();
    let mut file_sizes: Vec<Value> = Vec::new();
    let mut pids: Vec<Value> = Vec::new();
    let mut parent_pids: Vec<Value> = Vec::new();
    let mut entity_ids: Vec<Value> = Vec::new();
    let mut parent_entity_ids: Vec<Value> = Vec::new();

    let add = |set: &mut Vec<Value>, value: Option<&Value>| {
        if let Some(value) = value
            && !set.contains(value)
        {
            set.push(value.clone());
        }
    };
    let add_executable = |set: &mut Vec<String>, image: Option<&Value>| {
        let Some(image) = image else { return };
        let name = image.get("name").and_then(Value::as_str);
        let Some(name) = name else { return };
        let joined = match image.get("path").and_then(Value::as_str) {
            Some(path) => {
                let separator = if path.contains('\\') { '\\' } else { '/' };
                let mut joined = path.to_string();
                if !joined.ends_with(separator) {
                    joined.push(separator);
                }
                joined.push_str(name);
                joined
            }
            None => name.to_string(),
        };
        if !set.contains(&joined) {
            set.push(joined);
        }
    };
    // `<pid>|<creation time>|<device id>`, the three-part key the pipeline
    // then fingerprints.
    let entity_id = |item: &Value, process: &str| -> Option<Value> {
        let pid = item.get(process)?.get("id")?;
        let created = item.get(process)?.get("creation_datetime")?.as_str()?;
        let device = item.get("mde_device_id")?.as_str()?;
        Some(json!(format!(
            "{}|{created}|{device}",
            crate::painless_helpers::painless_to_string(pid)
        )))
    };

    for item in &evidence {
        add_executable(&mut executables, item.get("image_file"));
        add_executable(
            &mut parent_executables,
            item.get("parent_process").and_then(|p| p.get("image_file")),
        );

        match item.get("odata_type").and_then(Value::as_str) {
            Some("#microsoft.graph.security.fileEvidence") => add(
                &mut file_sizes,
                item.get("file_details").and_then(|d| d.get("size")),
            ),
            Some("#microsoft.graph.security.processEvidence") => {
                add(&mut pids, item.get("process").and_then(|p| p.get("id")));
                add(
                    &mut parent_pids,
                    item.get("parent_process").and_then(|p| p.get("id")),
                );
                add(&mut entity_ids, entity_id(item, "process").as_ref());
                add(
                    &mut parent_entity_ids,
                    entity_id(item, "parent_process").as_ref(),
                );
            }
            _ => {}
        }
    }

    for (path, mut values) in [
        ("file.size", file_sizes),
        ("process.pid", pids),
        ("process.parent.pid", parent_pids),
        ("process.entity_id", entity_ids),
        ("process.parent.entity_id", parent_entity_ids),
    ] {
        if values.is_empty() {
            continue;
        }
        values.sort_by(|a, b| {
            crate::painless_helpers::painless_to_string(a)
                .cmp(&crate::painless_helpers::painless_to_string(b))
        });
        let _ = event.set(path, Value::Array(values));
    }
    for (path, mut values) in [
        ("process.executable", executables),
        ("process.parent.executable", parent_executables),
    ] {
        if values.is_empty() {
            continue;
        }
        values.sort();
        let _ = match values.len() {
            1 => event.set(path, json!(values.remove(0))),
            _ => event.set(path, json!(values)),
        };
    }
    true
}

/// route53's answers rebuilt into ECS, feeding `related.ip` and
/// `related.hosts` as they go.
///
/// The vendor's `Class`/`Type`/`Rdata` become `class`/`type`/`data`, one
/// trailing dot is stripped off the data, and a CNAME repeats its data as the
/// answer's `name`.
fn run_route53_answers(event: &mut Event) -> bool {
    let Some(Value::Array(answers)) = event.get("dns.answers").cloned() else {
        return true;
    };

    let mut rebuilt = Vec::with_capacity(answers.len());
    let mut addresses = Vec::new();
    let mut hosts = Vec::new();
    for answer in &answers {
        let mut new_answer = Map::new();
        for (from, to) in [("Class", "class"), ("Type", "type")] {
            if let Some(value) = answer.get(from) {
                new_answer.insert(to.to_string(), value.clone());
            }
        }
        if let Some(rdata) = answer.get("Rdata").and_then(Value::as_str) {
            let data = rdata.strip_suffix('.').unwrap_or(rdata).to_string();
            let kind = new_answer
                .get("type")
                .and_then(Value::as_str)
                .map(str::to_string);
            let kind = kind.as_deref();
            if kind == Some("CNAME") {
                new_answer.insert("name".to_string(), json!(data.clone()));
            }
            match kind {
                Some("A" | "AAAA") => addresses.push(data.clone()),
                Some("CNAME" | "PTR") => hosts.push(data.clone()),
                _ => {}
            }
            new_answer.insert("data".to_string(), json!(data));
        }
        rebuilt.push(Value::Object(new_answer));
    }

    let _ = event.set("dns.answers", Value::Array(rebuilt));
    for (path, values) in [("related.ip", addresses), ("related.hosts", hosts)] {
        for value in values {
            let _ = event.append(path, json!(value));
        }
    }
    true
}

/// The ctx path a `def <name> = (ctx.<p> == null) ? false : ctx.<p>;` reads.
fn ternary_default_path(script: &str, name: &str) -> Option<String> {
    use crate::painless_params::clean_path;

    let (_, rest) = script.split_once(&format!(" {name} = (ctx."))?;
    let path = clean_path(rest.split("==").next()?);
    (!path.is_empty()
        && path
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@')))
    .then_some(path)
}

/// gcp's long-running operation: one session, opened by the first entry and
/// closed by the last.
///
/// An entry that is BOTH writes nothing -- the operation began and ended
/// inside it, so there is no session to bracket.
fn run_long_operation_session(event: &mut Event, first: &str, last: &str) -> bool {
    let flag = |path: &str| event.get(path).and_then(Value::as_bool).unwrap_or(false);
    let (first, last) = (flag(first), flag(last));
    if first && last {
        return true;
    }

    let _ = event.append("event.category", json!("session"));
    if first {
        let _ = event.append("event.type", json!("start"));
    }
    if last {
        let _ = event.append("event.type", json!("end"));
    }
    true
}

/// The fields a `splitStr(<map>, '<key>')` script splits, as full ctx paths.
fn parse_split_on_pipe(script: &str) -> Vec<String> {
    let Some(root) = ctx_path_bound_to(script, "ed") else {
        return Vec::new();
    };
    let Some(local) = script
        .split_once(" = ctx.")
        .and_then(|(head, _)| head.rsplit(char::is_whitespace).next())
        .map(str::to_string)
    else {
        return Vec::new();
    };

    let mut fields = Vec::new();
    for call in script.split("splitStr(").skip(1) {
        let Some((args, _)) = call.split_once(')') else {
            continue;
        };
        let Some((subject, key)) = args.split_once(", ") else {
            continue;
        };
        let key = key.trim().trim_matches('\'');
        // The helper's own subject is its parameter, not a field.
        let Some(tail) = subject.trim().strip_prefix(&local) else {
            continue;
        };
        if !tail.is_empty() && !tail.starts_with('.') {
            continue;
        }
        fields.push(format!("{root}{tail}.{key}"));
    }
    fields
}

/// Split each named field on `|`, in place.
///
/// The helper leaves anything that is not a NON-EMPTY string alone, so a field
/// already split stays split and an empty one stays empty rather than becoming
/// a one-element list.
fn run_split_on_pipe(event: &mut Event, fields: &[String]) -> bool {
    for field in fields {
        let Some(text) = event.get_str(field) else {
            continue;
        };
        if text.is_empty() {
            continue;
        }
        let parts: Vec<Value> = text.split('|').map(|part| json!(part)).collect();
        let _ = event.set(field, Value::Array(parts));
    }
    true
}

/// zscaler's parallel attachment columns and where each lands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AttachmentZip {
    source: String,
    target: String,
    /// The key each entry's map is stored under -- `file`.
    wrapper: String,
    /// `(member of the source map, dotted path inside the wrapped map)`.
    columns: Vec<(String, String)>,
}

/// One `<receiver>.put('<key>', <value>)` call.
struct PutCall {
    receiver: String,
    key: String,
    value: String,
}

/// Every `.put(` call in the script, in source order.
fn parse_put_calls(script: &str) -> Vec<PutCall> {
    let mut calls = Vec::new();
    for (head, tail) in script.split(".put('").zip(script.split(".put('").skip(1)) {
        let Some(receiver) = head
            .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
            .next()
        else {
            continue;
        };
        let Some((key, rest)) = tail.split_once('\'') else {
            continue;
        };
        // The value runs to the `put(`'s OWN closing paren, so a nested call
        // keeps its parens rather than being cut at the first one.
        let rest = rest.trim_start_matches(',').trim_start();
        let mut depth = 1usize;
        let mut end = None;
        for (at, c) in rest.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(at);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(end) = end else { continue };
        calls.push(PutCall {
            receiver: receiver.to_string(),
            key: key.to_string(),
            value: rest[..end].trim().to_string(),
        });
    }
    calls
}

impl AttachmentZip {
    /// Read the `att` binding, the `instanceof List` columns, and the target.
    fn parse(script: &str) -> Option<Self> {
        use crate::painless_params::clean_path;

        let source = ctx_path_bound_to(script, "att")?;
        let (head, _) = script.split_once(".add(")?;
        let target = clean_path(head.rsplit("ctx.").next()?);
        let entry = script.split(".add(").nth(1)?.split(')').next()?.trim();

        // `def <local> = att.<member> instanceof List ? ...`
        let mut members: Vec<(String, String)> = Vec::new();
        for line in script.lines() {
            let Some((head, _)) = line.split_once(" instanceof List ?") else {
                continue;
            };
            let Some((declaration, subject)) = head.rsplit_once(" = ") else {
                continue;
            };
            let (Some(local), Some(member)) = (
                declaration.rsplit(char::is_whitespace).next(),
                subject.trim().rsplit('.').next(),
            ) else {
                continue;
            };
            members.push((local.to_string(), member.to_string()));
        }

        // The entry is one map put under one key -- `item.put('file', file)`.
        let calls = parse_put_calls(script);
        let wrap = calls.iter().find(|call| call.receiver == entry)?;
        let (wrapper, inner) = (wrap.key.clone(), wrap.value.clone());

        let member_of = |value: &str| -> Option<String> {
            let local = value.strip_suffix(".get(i)")?;
            members
                .iter()
                .find(|(name, _)| name == local)
                .map(|(_, member)| member.clone())
        };

        let mut columns = Vec::new();
        for call in &calls {
            if call.receiver != inner {
                continue;
            }
            if let Some(member) = member_of(&call.value) {
                columns.push((member, call.key.clone()));
                continue;
            }
            // A nested map reaches the entry through a put of its own, so its
            // path is this key plus the one the value was stored under.
            for nested in calls.iter().filter(|c| c.receiver == call.value) {
                if let Some(member) = member_of(&nested.value) {
                    columns.push((member, format!("{}.{}", call.key, nested.key)));
                }
            }
        }

        (!columns.is_empty()).then_some(Self {
            source,
            target,
            wrapper,
            columns,
        })
    }
}

/// zscaler's severity score: the highest any of a field's values earns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MaxByContains {
    source: String,
    target: String,
    /// `(substrings that win this score, the score)`, in ladder order.
    arms: Vec<(Vec<String>, i64)>,
}

impl MaxByContains {
    fn parse(script: &str) -> Option<Self> {
        use crate::painless_params::clean_path;

        let source = ctx_path_bound_to(script, "raw")?;
        let (assignment, _) = script.split_once(" = maxSev;")?;
        let target = clean_path(assignment.rsplit("ctx.").next()?);

        let pieces: Vec<&str> = script.split("cur = ").collect();
        let mut arms = Vec::new();
        for index in 1..pieces.len() {
            let Some(score) = pieces[index]
                .split(';')
                .next()
                .and_then(|n| n.trim().parse::<i64>().ok())
            else {
                continue;
            };
            let condition = pieces[index - 1].rsplit("if (").next()?;
            let literals: Vec<String> = condition
                .split('\'')
                .skip(1)
                .step_by(2)
                .map(str::to_string)
                .collect();
            if !literals.is_empty() {
                arms.push((literals, score));
            }
        }

        (!arms.is_empty()).then_some(Self {
            source,
            target,
            arms,
        })
    }
}

/// Score every value the field holds and keep the highest.
///
/// The ladder is first-match per value, so a string containing two of the
/// substrings scores the EARLIER arm rather than the higher one.
fn run_max_by_contains(event: &mut Event, pattern: &MaxByContains) -> bool {
    let values = match event.get(&pattern.source) {
        Some(Value::Array(values)) => values.clone(),
        Some(value @ Value::String(_)) => vec![value.clone()],
        _ => return true,
    };

    let mut highest = 0i64;
    for value in values.iter().filter_map(Value::as_str) {
        let folded = value.to_lowercase();
        if let Some((_, score)) = pattern
            .arms
            .iter()
            .find(|(literals, _)| literals.iter().any(|lit| folded.contains(lit)))
            && *score > highest
        {
            highest = *score;
        }
    }

    if highest > 0 {
        let _ = event.set(&pattern.target, json!(highest));
    }
    true
}

/// Parallel columns zipped into a list of flat maps, one per index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ColumnZip {
    source: String,
    target: String,
    /// `(member of the source map, key in each entry)`.
    columns: Vec<(String, String)>,
    /// The member whose length bounds the loop.
    driver: String,
    /// Driver values that skip the entry entirely.
    skip: Vec<String>,
}

impl ColumnZip {
    fn parse(script: &str) -> Option<Self> {
        use crate::painless_params::clean_path;

        // `def <root> = ctx.<path>;` is the first binding in the script.
        let (head, tail) = script.split_once(" = ctx.")?;
        let root = head.rsplit(char::is_whitespace).next()?.to_string();
        let source = clean_path(tail.split([';', '\n']).next()?);

        let (assignment, _) = script.rsplit_once(" = out;")?;
        let target = clean_path(assignment.rsplit("ctx.").next()?);

        let mut members: Vec<(String, String)> = Vec::new();
        for line in script.lines() {
            let Some((head, _)) = line.split_once(" instanceof List ?") else {
                continue;
            };
            let Some((declaration, subject)) = head.rsplit_once(" = ") else {
                continue;
            };
            let subject = subject.trim();
            let (Some(local), Some(member)) = (
                declaration.rsplit(char::is_whitespace).next(),
                subject.strip_prefix(&format!("{root}.")),
            ) else {
                continue;
            };
            members.push((local.to_string(), member.to_string()));
        }

        // The loop bound names one local; that column drives the zip.
        let driver_local = script
            .split_once(".size(); i++)")
            .and_then(|(head, _)| head.rsplit("i < ").next())?
            .trim()
            .to_string();
        let (_, driver) = members.iter().find(|(name, _)| *name == driver_local)?;

        // The driver's own value is bound before the skip test, so a literal
        // compared against THAT local is a skip.
        let bound = script
            .split_once(&format!("= {driver_local}.get(i)"))
            .and_then(|(head, _)| head.rsplit(char::is_whitespace).nth(1))?;
        let skip = script
            .lines()
            .filter(|line| line.contains("continue") && line.contains(&format!("{bound} ==")))
            .flat_map(|line| line.split('\'').skip(1).step_by(2))
            .map(str::to_string)
            .collect();

        let mut columns = Vec::new();
        for call in parse_put_calls(script) {
            if call.value == bound {
                columns.push((driver.clone(), call.key));
                continue;
            }
            if let Some(local) = call.value.strip_suffix(".get(i)")
                && let Some((_, member)) = members.iter().find(|(name, _)| name == local)
            {
                columns.push((member.clone(), call.key));
            }
        }

        (!columns.is_empty()).then_some(Self {
            source,
            target,
            columns,
            driver: driver.clone(),
            skip,
        })
    }
}

/// Zip parallel columns into a list of maps, the driver bounding the loop.
///
/// The driver's own placeholder values -- the vendor writes `None` for "no
/// dictionary" -- drop the whole entry, and the list is written only when
/// something survived.
fn run_zip_columns(event: &mut Event, zip: &ColumnZip) -> bool {
    let column = |member: &str| -> Option<Vec<Value>> {
        match event.get(&format!("{}.{member}", zip.source)) {
            Some(Value::Array(values)) => Some(values.clone()),
            _ => None,
        }
    };
    let Some(driver) = column(&zip.driver) else {
        return true;
    };
    let columns: Vec<(&str, Vec<Value>)> = zip
        .columns
        .iter()
        .filter_map(|(member, key)| column(member).map(|values| (key.as_str(), values)))
        .collect();

    let mut out = Vec::new();
    for (index, entry) in driver.iter().enumerate() {
        match entry.as_str() {
            Some(name) if !zip.skip.iter().any(|dropped| dropped == name) => {}
            _ => continue,
        }
        let mut item = Map::new();
        for (key, values) in &columns {
            if let Some(value) = values.get(index) {
                item.insert((*key).to_string(), value.clone());
            }
        }
        out.push(Value::Object(item));
    }

    if !out.is_empty() {
        let _ = event.set(&zip.target, Value::Array(out));
    }
    true
}

/// Zip parallel attachment columns into one `{"file": {...}}` per index.
///
/// The list length is the LONGEST column, and a column that runs out simply
/// contributes nothing to the remaining entries.
fn run_zip_attachments(event: &mut Event, zip: &AttachmentZip) -> bool {
    let mut columns: Vec<(&str, Vec<Value>)> = Vec::new();
    for (member, path) in &zip.columns {
        if let Some(Value::Array(values)) = event.get(&format!("{}.{member}", zip.source)) {
            columns.push((path, values.clone()));
        }
    }
    let count = columns.iter().map(|(_, v)| v.len()).max().unwrap_or(0);
    if count == 0 {
        return true;
    }

    for index in 0..count {
        let mut file = Map::new();
        for (path, values) in &columns {
            let Some(value) = values.get(index) else {
                continue;
            };
            // `hash.md5` nests one level; every other slot is a plain key.
            match path.split_once('.') {
                Some((outer, inner)) => {
                    let entry = file
                        .entry(outer.to_string())
                        .or_insert_with(|| Value::Object(Map::new()));
                    if let Value::Object(map) = entry {
                        map.insert(inner.to_string(), value.clone());
                    }
                }
                None => {
                    file.insert((*path).to_string(), value.clone());
                }
            }
        }
        let mut item = Map::new();
        item.insert(zip.wrapper.clone(), Value::Object(file));
        let _ = event.append(&zip.target, Value::Object(item));
    }
    true
}

/// The `<target> = isTruthy(<source>)` assignments a script spells, in order.
fn parse_truthy_assignments(script: &str) -> Vec<(String, String)> {
    use crate::painless_params::clean_path;

    const CALL: &str = " = isTruthy(ctx.";
    let mut pairs = Vec::new();
    for (head, tail) in script.split(CALL).zip(script.split(CALL).skip(1)) {
        let Some(target) = head.rsplit("ctx.").next() else {
            continue;
        };
        let Some(source) = tail.split(')').next() else {
            continue;
        };
        // A target carrying whitespace is some other expression, not a path.
        let target = clean_path(target);
        if target.contains(char::is_whitespace) || target.is_empty() {
            continue;
        }
        pairs.push((target, clean_path(source)));
    }
    pairs
}

/// m365's `isTruthy`: a vendor flag read as a boolean whatever it was typed as.
///
/// Only a value the helper RESOLVES is written -- it returns null for anything
/// else, and a null assignment leaves no field behind. `false` is a resolved
/// value, so it is written like any other.
fn run_truthy_assignments(event: &mut Event, pairs: &[(String, String)]) -> bool {
    for (target, source) in pairs {
        let resolved = match event.get(source) {
            Some(Value::Bool(flag)) => Some(*flag),
            Some(Value::Number(number)) if !number.is_f64() => match number.as_i64() {
                Some(1) => Some(true),
                Some(0) => Some(false),
                _ => None,
            },
            Some(Value::String(text)) => match text.as_str() {
                "1" | "true" => Some(true),
                "0" | "false" => Some(false),
                _ => None,
            },
            _ => None,
        };
        if let Some(flag) = resolved {
            let _ = event.set(target, json!(flag));
        }
    }
    true
}

/// The address a reverse-lookup question names, back into `related.ip`.
///
/// `143.69.2.81.in-addr.arpa` is 81.2.69.143 with its octets reversed, and
/// `ip6.arpa` the same over single hex NIBBLES, four to a group. The script
/// re-groups them without compressing anything, so a leading zero survives --
/// `2a02:cf40:0add:...`, not `2a02:cf40:add:...`.
fn run_reverse_lookup_address(event: &mut Event) -> bool {
    let Some(name) = event.get_str("dns.question.name") else {
        return true;
    };

    let address = if name.contains(".in-addr.arpa") {
        let labels = name.replace(".in-addr.arpa", "");
        labels.split('.').rev().collect::<Vec<_>>().join(".")
    } else if name.contains(".ip6.arpa") {
        let labels = name.replace(".ip6.arpa", "");
        let nibbles: Vec<&str> = labels.split('.').rev().collect();
        let mut out = String::with_capacity(nibbles.len() + nibbles.len() / 4);
        for (index, nibble) in nibbles.iter().enumerate() {
            out.push_str(nibble);
            if index % 4 == 3 && index + 1 != nibbles.len() {
                out.push(':');
            }
        }
        out
    } else {
        return true;
    };

    let _ = event.append_unique("related.ip", json!(address));
    true
}

/// m365's identity fields off the same alert evidence list -- the sibling of
/// [`run_m365_process_evidence`], keyed on the evidence `odata_type`.
fn run_m365_identity_evidence(event: &mut Event, source: &str) -> bool {
    let Some(Value::Array(evidence)) = event.get(source).cloned() else {
        return true;
    };
    // `ctx.process.user = new HashMap()` runs whatever the evidence holds.
    if !event.has_value("process.user") {
        let _ = event.set("process.user", json!({}));
    }

    let mut sets: Vec<(&str, Vec<String>)> = vec![
        ("cloud.provider", Vec::new()),
        ("group.name", Vec::new()),
        ("host.id", Vec::new()),
        ("user.domain", Vec::new()),
        ("user.name", Vec::new()),
        ("user.id", Vec::new()),
        ("process.user.id", Vec::new()),
        ("process.user.name", Vec::new()),
    ];
    let mut add = |index: usize, value: Option<&str>| {
        if let Some(value) = value
            && !sets[index].1.iter().any(|held| held == value)
        {
            sets[index].1.push(value.to_string());
        }
    };

    for item in &evidence {
        let account = |member: &str| {
            item.get("user_account")
                .and_then(|a| a.get(member))
                .and_then(Value::as_str)
        };
        match item.get("odata_type").and_then(Value::as_str) {
            Some("#microsoft.graph.security.securityGroupEvidence") => {
                add(1, item.get("display_name").and_then(Value::as_str));
            }
            Some("#microsoft.graph.security.deviceEvidence") => {
                add(2, item.get("mde_device_id").and_then(Value::as_str));
            }
            Some(
                "#microsoft.graph.security.mailboxEvidence"
                | "#microsoft.graph.security.userEvidence",
            ) => {
                add(3, account("domain_name"));
                add(5, account("user_principal_name"));
                add(4, account("account_name"));
            }
            Some("#microsoft.graph.security.processEvidence") => {
                add(6, account("azure_ad_user_id"));
                add(7, account("account_name"));
            }
            _ => {}
        }
        // The cloud provider is read off EVERY evidence entry, whatever its
        // type, and only azure is recognised.
        if item
            .get("vm_metadata")
            .and_then(|m| m.get("cloud_provider"))
            .and_then(Value::as_str)
            .is_some_and(|provider| provider.eq_ignore_ascii_case("azure"))
        {
            add(0, Some("azure"));
        }
    }

    for (path, mut values) in sets {
        if values.is_empty() {
            continue;
        }
        values.sort();
        let _ = event.set(path, json!(values));
    }
    true
}

/// `ctx.<f>.removeIf(v -> v == '<literal>')` as a
/// [`KnownPattern::RemoveListValue`].
fn parse_remove_list_value(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let at = script.find(".removeIf(")?;
    let before = &script[..at];
    let subject = &before[before.rfind("ctx.")? + 4..];
    // A ctx FIELD and nothing else. crowdstrike's argv split calls the same
    // method on a local and on an entry set, and reading back to the nearest
    // `ctx.` claimed those scripts for a field they never touch.
    if !subject
        .chars()
        .all(|c| c.is_alphanumeric() || "._?".contains(c))
    {
        return None;
    }
    let field = clean_path(subject);
    let (_, lambda) = script[at..].split_once("->")?;
    let lambda = lambda.split(')').next()?;
    // ONE literal comparison and nothing else. A drop-empty predicate is a
    // chain of them over several sentinels, and reading its first literal
    // would claim that whole script for a single removal.
    if lambda.matches("==").count() != 1 || lambda.contains('|') || lambda.contains('&') {
        return None;
    }
    let (_, rhs) = lambda.split_once("==")?;
    // Painless's OWN escapes, which `normalise` does not touch -- it resolves
    // the JSON layer only. `'\\'` in the script is one backslash, and m365
    // removes exactly that from `file.path`.
    let value = quoted_first(rhs)?.replace("\\\\", "\\");
    (!field.is_empty()).then_some(KnownPattern::RemoveListValue { field, value })
}

/// Drop every member of a list equal to one literal.
fn run_remove_list_value(event: &mut Event, field: &str, value: &str) -> bool {
    let Some(Value::Array(members)) = event.get(field) else {
        return true;
    };
    let kept: Vec<Value> = members
        .iter()
        .filter(|member| member.as_str() != Some(value))
        .cloned()
        .collect();
    let _ = event.set(field, Value::Array(kept));
    true
}

/// `ctx.<f> = ctx.<f>.substring(0, ctx.<f>.length() - 1)` -- drop the last
/// character.
///
/// `mysql_enterprise`'s audit lines arrive with the trailing comma of the array
/// they came from, and this is what removes it. Unclaimed, the `json`
/// processor after it had invalid JSON on every event and the source scored
/// 1.3% of its fields.
fn parse_drop_last_char(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let (before, after) = script.split_once(" = ctx.")?;
    let target = clean_path(&before[before.rfind("ctx.")? + 4..]);
    let (source, tail) = after.split_once(".substring(0, ctx.")?;

    // The SAME field on both sides, and nothing but the length expression
    // after it -- a substring of one field into another is a different pattern.
    if clean_path(source) != target || target.is_empty() {
        return None;
    }
    let tail = tail.strip_prefix(source)?;
    tail.trim_start().strip_prefix(".length() - 1);")?;

    Some(KnownPattern::DropLastChar(target))
}

/// Remove the final character of a string field.
fn run_drop_last_char(event: &mut Event, field: &str) -> bool {
    let Some(text) = event.get_string(field) else {
        return true;
    };
    // By CHARACTER, so a multi-byte final character leaves valid UTF-8.
    let mut chars = text.chars();
    chars.next_back();
    let _ = event.set(field, Value::String(chars.as_str().to_owned()));
    true
}

/// `ctx.<f> = ctx.<f>.substring(<n>);` -- drop the first `n` characters.
///
/// `cloudflare_logpush`'s firewall pipeline takes the leading `?` off
/// `url.query` this way. The count is READ rather than pinned at one: the
/// statement is a general one and a matcher written for the literal would
/// decline the next pipeline that cuts two.
fn parse_drop_leading_chars(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    // ONE whole statement. Anything else in the script would be silently
    // dropped, which is the half-run this ladder refuses.
    let statement = script.trim().strip_suffix(';')?;
    if statement.contains(';') {
        return None;
    }

    let (before, after) = statement.split_once(" = ctx.")?;
    let target = clean_path(before.strip_prefix("ctx.")?);
    let (source, count) = after.split_once(".substring(")?;

    // The SAME field on both sides -- a substring of one field into another
    // keeps two values and is a different pattern.
    if target.is_empty() || clean_path(source) != target {
        return None;
    }

    // A bare character count and nothing else. A second argument is a cut with
    // an end, and an expression in place of the literal is a count this cannot
    // know without running the script.
    let count: usize = count.strip_suffix(')')?.parse().ok()?;
    Some(KnownPattern::DropLeadingChars {
        field: target,
        count,
    })
}

/// Remove the first `count` characters of a string field.
fn run_drop_leading_chars(event: &mut Event, field: &str, count: usize) -> bool {
    let Some(text) = event.get_string(field) else {
        return true;
    };
    // By CHARACTER, matching Painless. A count past the end writes nothing:
    // Java's `substring` throws there, so the vendor's processor fails and
    // leaves the field as it was rather than storing an empty string.
    if text.chars().count() < count {
        return true;
    }
    let kept: String = text.chars().skip(count).collect();
    let _ = event.set(field, Value::String(kept));
    true
}

/// A list deduplicated in place, and UNWRAPPED when one member is left.
///
/// suricata writes `destination.domain` as a list and then collapses it, so
/// the field is a bare string wherever the answers agreed and a list only
/// where they did not. The unwrap is the part that matters -- leaving a
/// one-member list is a different document from a string.
fn parse_dedupe_unwrap(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let raw = script.split_once(" = ctx.")?.1.split_once(';')?.0;
    if raw.is_empty()
        || !raw
            .chars()
            .all(|c| c.is_alphanumeric() || "._?".contains(c))
    {
        return None;
    }
    let field = clean_path(raw);
    (!field.is_empty()).then_some(KnownPattern::DedupeUnwrap(field))
}

/// Drop repeated members, keeping first-seen order, then unwrap a single one.
fn run_dedupe_unwrap(event: &mut Event, field: &str) -> bool {
    let Some(Value::Array(items)) = event.get(field).cloned() else {
        return true;
    };

    let mut kept: Vec<Value> = Vec::with_capacity(items.len());
    for item in items {
        if !kept.contains(&item) {
            kept.push(item);
        }
    }

    let value = match kept.len() {
        1 => kept.into_iter().next().unwrap_or(Value::Null),
        _ => Value::Array(kept),
    };
    let _ = event.set(field, value);
    true
}

/// A numbered CSV column MAP collapsed into a list, in key order.
///
/// `symantec_endpoint`'s csv processor writes `_csv_array.00`..`.50` as
/// separate fields, and this script puts them through a `TreeMap` to get the
/// columns back in order as a list. The keys are zero-padded, so natural
/// string order IS column order. A surrounding pair of SINGLE quotes comes
/// off each value -- the csv processor quotes on `"` and never sees them.
fn parse_csv_map_to_array(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let source = script.split_once("putAll(ctx.")?.1.split_once(')')?.0;
    let target = script
        .split_once("ctx['")?
        .1
        .split_once("'] = columnArray")?
        .0;
    let ok = |raw: &str| {
        !raw.is_empty()
            && raw
                .chars()
                .all(|c| c.is_alphanumeric() || "._?".contains(c))
    };
    if !ok(source) || !ok(target) {
        return None;
    }
    Some(KnownPattern::CsvMapToArray {
        source: clean_path(source),
        target: clean_path(target),
    })
}

/// Collapse the column map into a list, unquoting each member.
fn run_csv_map_to_array(event: &mut Event, source: &str, target: &str) -> bool {
    let Some(Value::Object(columns)) = event.get(source).cloned() else {
        return true;
    };

    let mut keys: Vec<&String> = columns.keys().collect();
    keys.sort_unstable();

    let mut ordered: Vec<Value> = Vec::with_capacity(keys.len());
    for key in keys {
        let Some(value) = columns.get(key) else {
            continue;
        };
        // Only a string carries quotes to strip, and a lone `'` is not a
        // PAIR -- the vendor's `substring(1, length - 1)` would throw on it.
        match value.as_str() {
            Some(text) if text.len() > 1 && text.starts_with('\'') && text.ends_with('\'') => {
                ordered.push(Value::String(text[1..text.len() - 1].to_owned()));
            }
            _ => ordered.push(value.clone()),
        }
    }

    let _ = event.set(target, Value::Array(ordered));
    true
}

/// `^([a-zA-Z][a-zA-Z0-9 \(\)-]{0,28}):(?:\s(.+)|\s)?` under `matches()`, by
/// hand. Returns the raw key and the value, which is absent where the member
/// is a bare `Key:` or `Key: `.
///
/// `matches()` demands the WHOLE member, which is what makes this parseable
/// without a regex: `:` is outside the key class, so the key is exactly the
/// run before the first colon, and `.` never crosses a line terminator, so a
/// member carrying one does not match at all.
fn colon_key_value(member: &str) -> Option<(&str, Option<&str>)> {
    let is_space = |c: char| matches!(c, ' ' | '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r');

    let colon = member.find(':')?;
    let key = &member[..colon];
    if key.len() > 29
        || !key.starts_with(|c: char| c.is_ascii_alphabetic())
        || !key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, ' ' | '(' | ')' | '-'))
    {
        return None;
    }

    let rest = &member[colon + 1..];
    if rest.is_empty() {
        return Some((key, None));
    }
    let mut tail = rest.chars();
    if !tail.next().is_some_and(is_space) {
        return None;
    }
    let value = tail.as_str();
    if value.is_empty() {
        return Some((key, None));
    }
    if value.contains(['\n', '\r']) {
        return None;
    }
    Some((key, Some(value)))
}

/// The vendor's key normalisation: lowercase, spaces to underscores, then
/// every parenthesis dropped.
fn colon_key(raw: &str) -> String {
    raw.to_lowercase()
        .chars()
        .filter_map(|c| match c {
            ' ' => Some('_'),
            '(' | ')' => None,
            _ => Some(c),
        })
        .collect()
}

/// `Key: value` columns into a map, and their KEYS into a fingerprint.
///
/// This is the half of `symantec_endpoint`'s parse that scores: the labelled
/// columns become `symantec_endpoint.log.*` through a later rename, and the
/// joined key list identifies which of the fourteen log layouts the line came
/// from, so the params table can name the unlabelled columns. An unmatched
/// column contributes `NONE`, which is why the fingerprints are mostly holes.
fn parse_csv_colon_pairs(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let list = script.split_once("ctx.")?.1.split_once(".forEach(")?.0;
    let map_target = script.split_once("ctx['")?.1.split_once("'] = keyValue")?.0;
    let fingerprint_target = script
        .rsplit_once("ctx['")?
        .1
        .split_once("'] = String.join(")?
        .0;
    let ok = |raw: &str| {
        !raw.is_empty()
            && raw
                .chars()
                .all(|c| c.is_alphanumeric() || "._?".contains(c))
    };
    if !ok(list) || !ok(map_target) || !ok(fingerprint_target) {
        return None;
    }

    // The alias table is a literal in the script, so it is read from there
    // rather than pinned here -- a package that ships a different one gets
    // its own aliases for free.
    let block = script
        .split_once("unmodifiableMap([")?
        .1
        .split_once("])")?
        .0;
    let mut tokens: Vec<String> = Vec::new();
    let mut rest = block;
    while let Some((_, after)) = rest.split_once('\'') {
        let Some((token, tail)) = after.split_once('\'') else {
            break;
        };
        tokens.push(token.to_owned());
        rest = tail;
    }

    Some(KnownPattern::CsvColonPairs {
        list: clean_path(list),
        map_target: clean_path(map_target),
        fingerprint_target: clean_path(fingerprint_target),
        aliases: tokens
            .as_chunks::<2>()
            .0
            .iter()
            .map(|[from, to]| (from.clone(), to.clone()))
            .collect(),
    })
}

/// Read each `Key: value` column into the map, every key into the fingerprint.
fn run_csv_colon_pairs(
    event: &mut Event,
    list: &str,
    map_target: &str,
    fingerprint_target: &str,
    aliases: &[(String, String)],
) -> bool {
    let Some(Value::Array(members)) = event.get(list).cloned() else {
        return true;
    };

    let mut pairs = serde_json::Map::new();
    let mut fingerprint: Vec<String> = Vec::with_capacity(members.len());

    for member in &members {
        // `NONE` is the placeholder an unlabelled column contributes, and the
        // params table's fingerprints are written against it.
        let mut key = "NONE".to_owned();
        if let Some(text) = member.as_str()
            && let Some((raw, value)) = colon_key_value(text)
        {
            key = colon_key(raw);
            if let Some((_, alias)) = aliases.iter().find(|(from, _)| *from == key) {
                key.clone_from(alias);
            }
            if let Some(value) = value {
                // Java's `trim` cuts at U+0020, not at Unicode whitespace.
                let trimmed = value.trim_matches(|c: char| c <= ' ');
                if !trimmed.is_empty() {
                    pairs.insert(key.clone(), Value::String(trimmed.to_owned()));
                }
            }
        }
        fingerprint.push(key);
    }

    // An all-unlabelled line writes no map at all, and the params table is
    // what names its columns.
    if !pairs.is_empty() {
        let _ = event.set(map_target, Value::Object(pairs));
    }
    let _ = event.set(fingerprint_target, fingerprint.join("|"));
    true
}

/// A tag list scrubbed into names, with the prefixed ones lifted into a
/// marking map.
///
/// `ti_misp` ships this twice -- once per stream -- and it is TWO writes in
/// one script: every tag name with a couple of characters stripped out, and
/// the `tlp:` ones stripped of their prefix and upper-cased under
/// `threat.indicator.marking`. Claiming only the first write scored the
/// `tags` field and left the marking missing on 16 events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TagsAndMarking {
    /// The list of tag objects.
    source: String,
    /// The member of each object holding the name.
    member: String,
    /// The characters taken out of every name.
    strip: Vec<String>,
    /// Where the scrubbed names go.
    scrubbed: String,
    /// Whether they are APPENDED there rather than assigned -- the `threat`
    /// stream writes `ctx.tags.addAll(tags)` onto a list something else has
    /// already started, where `threat_attributes` assigns its own `temp_tags`.
    append: bool,
    /// The marking map, and the key the selected names go under.
    marking: String,
    key: String,
    /// The prefix that selects a name, and which then comes off it.
    prefix: String,
}

/// Every `.replace('<what>', '')` argument in a chain.
///
/// The text is what PAINLESS sees, so `'\\'` is one backslash -- the pipeline
/// strips a literal backslash out of tag names, not an escape.
fn replaced_away(chain: &str) -> Vec<String> {
    let mut out = Vec::new();
    for piece in chain.split(".replace('").skip(1) {
        let Some((what, rest)) = piece.split_once("', '") else {
            continue;
        };
        if rest.starts_with("')") {
            out.push(what.replace("\\\\", "\\"));
        }
    }
    out
}

fn parse_tags_and_marking(script: &str) -> Option<TagsAndMarking> {
    use crate::painless_params::clean_path;

    let source = script.split_once("= ctx.")?.1.split_once(".stream()")?.0;
    let member = script
        .split_once(".map(t -> t.")?
        .1
        .split_once(".replace(")?
        .0;
    let prefix = script
        .split_once(".startsWith('")?
        .1
        .split_once("')")?
        .0
        .to_owned();
    // The two writes are the last two statements and the marking is the very
    // last, so the scrubbed list is the assignment immediately before it.
    let (before_marking, after) = script.rsplit_once(" = [ '")?;
    let marking = painless_path(before_marking)?;
    let key = after.split_once('\'')?.0.to_owned();
    let head = &before_marking[..before_marking.rfind("ctx.")?];
    let statement = head.rsplit_once("ctx.")?.1;
    let (scrubbed, append) = match statement.split_once(".addAll(") {
        Some((path, _)) => (path, true),
        None => (statement.split_once(" = ")?.0, false),
    };

    let ok = |raw: &str| {
        !raw.is_empty()
            && raw
                .chars()
                .all(|c| c.is_alphanumeric() || "._?".contains(c))
    };
    if !ok(source) || !ok(member) || !ok(scrubbed) || prefix.is_empty() || key.is_empty() {
        return None;
    }

    // The scrub chain is the one inside the FIRST map, before the filter.
    let chain = script.split_once(".map(t -> t.")?.1;
    let strip = replaced_away(chain.split_once(".filter(").map_or(chain, |(head, _)| head));
    (!strip.is_empty()).then(|| TagsAndMarking {
        source: clean_path(source),
        member: clean_path(member),
        strip,
        scrubbed: clean_path(scrubbed),
        append,
        marking,
        key,
        prefix,
    })
}

/// Scrub the tag names, and lift the prefixed ones into the marking.
fn run_tags_and_marking(event: &mut Event, pattern: &TagsAndMarking) -> bool {
    let Some(Value::Array(tags)) = event.get(&pattern.source).cloned() else {
        return true;
    };

    let mut names: Vec<Value> = Vec::with_capacity(tags.len());
    let mut selected: Vec<Value> = Vec::new();
    for tag in &tags {
        let Some(raw) = tag.get(&pattern.member).and_then(Value::as_str) else {
            continue;
        };
        let mut name = raw.to_owned();
        for what in &pattern.strip {
            name = name.replace(what.as_str(), "");
        }
        // The vendor selects on the PREFIX and then replaces every occurrence
        // of it, which is the same thing wherever it appears once.
        if name.starts_with(pattern.prefix.as_str()) {
            selected.push(Value::String(
                name.replace(&pattern.prefix, "").to_uppercase(),
            ));
        }
        names.push(Value::String(name));
    }

    if pattern.append {
        for name in names {
            let _ = event.append(&pattern.scrubbed, name);
        }
    } else {
        let _ = event.set(&pattern.scrubbed, Value::Array(names));
    }
    // Assigned WHOLE, so anything already under the marking goes.
    let mut marking = serde_json::Map::new();
    marking.insert(pattern.key.clone(), Value::Array(selected));
    let _ = event.set(&pattern.marking, Value::Object(marking));
    true
}

/// `fortinet_fortiproxy`'s KV loop, which is stormshield's idea with five
/// behavioural differences -- hence its own runner rather than a widened
/// [`parse_kv_into_namespace`].
///
/// The namespace opens with `[:]` rather than `new HashMap()`, a closing
/// quote only counts when a space or the end follows it, the pair guard is
/// the START cursor rather than the split, only a LEADING or trailing quote
/// comes off the value, and a pair whose value is `N/A` or whose key holds a
/// non-word character is dropped outright.
fn parse_kv_into_fields(script: &str) -> Option<KnownPattern> {
    let target = script.split_once("ctx[\"")?.1.split_once("\"] = [:]")?.0;
    (!target.is_empty()
        && target
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.'))
    .then(|| KnownPattern::KvIntoFields(target.to_owned()))
}

/// Split `message` into `key=value` pairs under `target`, fortiproxy's way.
pub fn kv_into_fields(event: &mut Event, target: &str) -> bool {
    let Some(message) = event.get_string("message") else {
        return true;
    };
    let bytes = message.as_bytes();
    let n = bytes.len();

    let mut fields = serde_json::Map::new();
    let (mut kv_start, mut kv_split) = (0usize, 0usize);
    let mut in_quote = false;

    for (i, byte) in bytes.iter().enumerate() {
        if *byte == b'"' {
            // A quote inside a value does not close it; only one with a
            // space after it, or at the very end, does.
            if in_quote && i < n - 1 && bytes[i + 1] != b' ' {
                continue;
            }
            in_quote = !in_quote;
        }
        if in_quote {
            continue;
        }
        if *byte == b'=' {
            kv_split = i;
        }
        if *byte == b' ' || i == n - 1 {
            if i != kv_start {
                let end = if i == n - 1 { i + 1 } else { i };
                if let (Some(key), Some(raw)) = (
                    message.get(kv_start..kv_split),
                    message.get(kv_split + 1..end),
                ) {
                    let value = raw.strip_prefix('"').unwrap_or(raw);
                    let value = value.strip_suffix('"').unwrap_or(value);
                    // The vendor drops the pair rather than storing either.
                    let key_is_word =
                        !key.is_empty() && key.chars().all(|c| c.is_alphanumeric() || c == '_');
                    if value != "N/A" && key_is_word {
                        fields.insert(key.to_owned(), Value::String(value.to_owned()));
                    }
                }
            }
            kv_start = i + 1;
            kv_split = i + 1;
        }
    }

    let _ = event.set(target, Value::Object(fields));
    true
}

/// A hand-written quote-aware KV splitter writing into one namespace.
///
/// stormshield's WHOLE parse is this one script -- the vendor walks the
/// message character by character rather than using a `kv` processor, so
/// nothing downstream of it had any input at all and the source scored 3.3%
/// of its fields.
fn parse_kv_into_namespace(script: &str) -> Option<KnownPattern> {
    let target = script
        .split_once("ctx[\"")?
        .1
        .split_once("\"] = new HashMap()")?
        .0;
    (!target.is_empty()
        && target
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '.'))
    .then(|| KnownPattern::KvIntoNamespace(target.to_owned()))
}

/// Split `message` into `key=value` pairs under `target`.
///
/// A transliteration of the vendor's own loop, its cursor arithmetic
/// included: a quote TOGGLES the state and everything inside one is skipped,
/// the last character closes the final pair, and the value has every quote
/// removed rather than just a surrounding pair.
fn run_kv_into_namespace(event: &mut Event, target: &str) -> bool {
    let Some(message) = event.get_string("message") else {
        return true;
    };
    let bytes = message.as_bytes();
    let n = bytes.len();

    let mut pairs = serde_json::Map::new();
    let (mut kv_start, mut kv_split) = (0usize, 0usize);
    let mut in_quote = false;

    for (i, byte) in bytes.iter().enumerate() {
        if *byte == b'"' {
            in_quote = !in_quote;
        }
        if in_quote {
            continue;
        }
        if *byte == b'=' {
            kv_split = i;
        }
        if *byte == b' ' || i == n - 1 {
            if kv_start != kv_split {
                let end = if i == n - 1 { n } else { i };
                // Slicing is by BYTE, so a multi-byte character in the middle
                // of a pair yields None rather than panicking.
                if let (Some(key), Some(value)) = (
                    message.get(kv_start..kv_split),
                    message.get(kv_split + 1..end),
                ) {
                    pairs.insert(key.to_owned(), Value::String(value.replace('"', "")));
                }
            }
            kv_start = i + 1;
            kv_split = i + 1;
        }
    }

    // The vendor creates the map before the loop, so an unparseable message
    // still leaves an empty one behind.
    let _ = event.set(target, Value::Object(pairs));
    true
}

/// A lookup whose TABLE lives in the document rather than in `params`.
///
/// bitdefender ships its tenant list on the event and names the organisation
/// from it, so the table is per-document and no params matcher can see it.
fn parse_ctx_table_lookup(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let plain = |path: &str| {
        !path.is_empty()
            && path
                .chars()
                .all(|c| c.is_alphanumeric() || "._?".contains(c))
    };

    let (before_table, after_table) = script.split_once(" = ctx.")?;
    let table_local = identifier_before(before_table)?.to_owned();
    let (table, rest) = after_table.split_once(';')?;

    let (before_key, after_key) = rest.split_once(" = ctx.")?;
    let key_local = identifier_before(before_key)?.to_owned();
    let key = after_key.split_once(';')?.0;

    // The guard has to name the SAME two locals, or this is another script
    // that merely opens the same way.
    if !script.contains(&format!("{table_local}.containsKey({key_local})")) {
        return None;
    }
    let assign = format!(" = {table_local}[{key_local}];");
    let head = &script[..script.find(&assign)?];
    let target = clean_path(&head[head.rfind("ctx.")? + 4..]);

    if !plain(table) || !plain(key) || target.is_empty() {
        return None;
    }
    Some(KnownPattern::CtxTableLookup {
        table: clean_path(table),
        key: clean_path(key),
        target,
    })
}

/// Write the row `key` selects out of the document's own table.
fn run_ctx_table_lookup(event: &mut Event, table: &str, key: &str, target: &str) -> bool {
    let row = {
        let Some(Value::Object(rows)) = event.get(table) else {
            return true;
        };
        let Some(selector) = event.get(key).map(|value| match value {
            Value::String(text) => text.clone(),
            other => other.to_string(),
        }) else {
            return true;
        };
        rows.get(&selector).cloned()
    };
    if let Some(row) = row {
        let _ = event.set(target, row);
    }
    true
}

/// Split a firehose record on spaces, keeping quoted runs whole.
///
/// The quotes stay IN the token, which is what the vendor's `StringBuilder`
/// does, and the S3 host-header test depends on it.
fn firehose_tokens(message: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut inside = false;

    for c in message.chars() {
        if c == '"' {
            inside = !inside;
            current.push(c);
        } else if c == ' ' && !inside {
            if !current.is_empty() {
                tokens.push(std::mem::take(&mut current));
            }
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

/// Name the AWS log type a firehose record carries.
///
/// A transliteration of awsfirehose's own classifier, ladder order included --
/// the route53 arms have to stay in this order, and the resolver test would
/// otherwise claim the public records.
fn firehose_dataset(event: &Event, message: &str, lower: &str) -> Option<&'static str> {
    // `ctx['aws.kinesis.name']` is a bracket subscript of a flat dotted key at
    // the ROOT, which is a literal read and not a path walk.
    let literal = |key: &str| -> Option<&str> { event.as_value().as_object()?.get(key)?.as_str() };
    let group_holds = |needle: &str| {
        literal("aws.cloudwatch.log_group").is_some_and(|value| value.contains(needle))
    };

    if literal("aws.kinesis.name").is_some_and(|v| v.contains("aws-waf-logs-"))
        || group_holds("aws-waf-logs-")
    {
        return Some("aws.waf");
    }
    if ["webaclid", "terminatingrule", "httpsource", "rulegrouplist"]
        .iter()
        .all(|needle| lower.contains(needle))
    {
        return Some("aws.waf");
    }
    if literal("aws.cloudwatch.log_stream").is_some_and(|v| v.contains("CloudTrail")) {
        return Some("aws.cloudtrail");
    }
    // A transit gateway names itself; an ENI or NAT record is known by its
    // action instead, and v9+ regional NAT reports the interface as `-`.
    if (lower.contains("tgw-") && lower.contains("transitgateway"))
        || ((lower.contains("eni-") || lower.contains("nat-"))
            && ["accept", "reject", "nodata", "skipdata"]
                .iter()
                .any(|action| lower.contains(action)))
    {
        return Some("aws.vpcflow");
    }
    if [
        "\"firewall_name\":",
        "\"availability_zone\":",
        "\"event_timestamp\":",
        "\"event\":",
    ]
    .iter()
    .all(|key| lower.contains(key))
    {
        return Some("aws.firewall_logs");
    }
    if [
        "\"version\":",
        "\"account_id\":",
        "\"region\":",
        "\"vpc_id\":",
        "\"query_timestamp\":",
    ]
    .iter()
    .all(|key| lower.contains(key))
    {
        return Some("aws.route53_resolver_logs");
    }
    if group_holds("/aws/route53/") {
        // The vendor's chain STOPS on the group name, so a record from this
        // group that fails the pattern test is left unnamed rather than falling
        // through to the token checks.
        return (message.contains('T')
            && message.contains('Z')
            && message.contains("NOERROR")
            && message.contains("UDP"))
        .then_some("aws.route53_public_logs");
    }
    if lower.contains("\"requestid\":")
        && lower.contains("\"ip\":")
        && (lower.contains("\"requesttime\":") || lower.contains("\"request_time\":"))
        && (lower.contains("\"httpmethod\":") || lower.contains("\"eventtype\":"))
    {
        return Some("aws.apigateway_logs");
    }

    let tokens = firehose_tokens(message);
    if tokens.len() >= 24 {
        if tokens[23].contains("s3") && tokens[23].contains("amazonaws.com") {
            return Some("aws.s3access");
        }
        if ["SOAP.", "REST.", "BATCH.", "WEBSITE.", "S3."]
            .iter()
            .any(|prefix| tokens[7].starts_with(prefix))
        {
            return Some("aws.s3access");
        }
    }

    // Tokenising splits the CloudFront timestamp in two, which is why 33 is
    // the count and why the date and time are checked as separate tokens.
    if tokens.len() == 33 {
        let (date, time) = (tokens[0].as_bytes(), tokens[1].as_bytes());
        let dated = date.len() == 10 && date[4] == b'-' && date[7] == b'-';
        let timed = time.len() == 8 && time[2] == b':' && time[5] == b':';
        return (dated && timed).then_some("aws.cloudfront_logs");
    }
    if tokens
        .first()
        .is_some_and(|first| matches!(first.as_str(), "http" | "https" | "tcp" | "tls" | "udp"))
    {
        return Some("aws.elb_logs");
    }
    // A classic ELB, known by two host:port tokens and a number after them.
    // The vendor reads token 4 behind a `length >= 4` guard and catches the
    // index error, so four tokens name nothing.
    if matches!(tokens.len(), 15 | 22 | 29)
        && tokens.len() >= 5
        && tokens[2].contains(':')
        && tokens[3].contains(':')
        && tokens[4].parse::<f64>().is_ok()
    {
        return Some("aws.elb_logs");
    }
    None
}

/// Write the dataset the firehose classifier names, if it names one.
fn run_firehose_dataset(event: &mut Event) -> bool {
    let Some(message) = event.get_string("message") else {
        return true;
    };
    let lower = message.to_lowercase();
    if let Some(dataset) = firehose_dataset(event, &message, &lower) {
        let _ = event.set("event.dataset", Value::String(dataset.to_owned()));
    }
    true
}

/// The sorted key names of the FIRST nested map holding an inner map.
///
/// awsfirehose collects every metric name out of `aws.<service>.metrics` so a
/// `fingerprint` can key the document by which metrics it carries. The list is
/// written whether or not anything was found, exactly as the vendor does, and
/// a `remove` drops it once the fingerprint has been taken.
fn parse_nested_key_names(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let source = script
        .split_once("for (entry in ctx.")?
        .1
        .split_once(".entrySet()")?
        .0;
    if source.is_empty()
        || !source
            .chars()
            .all(|c| c.is_alphanumeric() || "._".contains(c))
    {
        return None;
    }
    let inner = script.split_once(".get(\"")?.1.split('"').next()?;
    let assigned = script.rsplit_once(" = metricNames;")?.0;
    let target = clean_path(&assigned[assigned.rfind("ctx.")? + 4..]);

    (!inner.is_empty() && !target.is_empty()).then_some(KnownPattern::NestedKeyNames {
        source: clean_path(source),
        inner: inner.to_owned(),
        target,
    })
}

/// Write the sorted keys of the first `inner` map found under `source`.
fn run_nested_key_names(event: &mut Event, source: &str, inner: &str, target: &str) -> bool {
    let mut names: Vec<String> = Vec::new();
    if let Some(Value::Object(top)) = event.get(source) {
        for value in top.values() {
            let Some(Value::Object(members)) = value.as_object().and_then(|m| m.get(inner)) else {
                continue;
            };
            names.extend(members.keys().cloned());
            break;
        }
    }
    names.sort();
    let _ = event.set(
        target,
        Value::Array(names.into_iter().map(Value::String).collect()),
    );
    true
}

/// `ctx.<target> = ZonedDateTime.parse(ctx['<source>']).plusDays(<n>)` as a
/// [`KnownPattern::DatePlusDays`].
///
/// The source is read through a bracket subscript because the field it names
/// is `@timestamp`, which no dotted path can spell.
fn parse_date_plus_days(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    // The assignment's spacing is the vendor's, not ours -- ti_eset writes two
    // spaces after the `=`, so the operator is found rather than matched.
    let (before, after) = script.split_once("ZonedDateTime.parse(")?;
    let assigned = before[..before.rfind('=')?].trim_end();
    // The WHOLE path, not its last segment: the script declares the parent map
    // first, so the nearest `ctx.` is the assignment's own.
    let target = clean_path(&assigned[assigned.rfind("ctx.")? + 4..]);
    let (subject, tail) = after.split_once(").plusDays(")?;
    let days: i64 = tail
        .split(')')
        .next()?
        .trim()
        .trim_end_matches(['L', 'l'])
        .parse()
        .ok()?;

    // `ctx['@timestamp']` or `ctx.a.b`, the two spellings the source takes.
    let source = subject
        .trim()
        .strip_prefix("ctx")
        .map(|rest| rest.trim_start_matches('.'))
        .map(|rest| rest.trim_matches(['[', ']', '\'', '"']))?;

    (!target.is_empty() && !source.is_empty()).then_some(KnownPattern::DatePlusDays {
        source: clean_path(source),
        target,
        days,
    })
}

/// Write `source` advanced by a whole number of days to `target`.
fn run_date_plus_days(event: &mut Event, source: &str, target: &str, days: i64) -> bool {
    let Some(text) = event.get_string(source) else {
        return true;
    };
    if let Some(moved) = crate::date_formats::iso8601_plus(&text, 'd', days, 0) {
        let _ = event.set(target, Value::String(moved));
    }
    true
}

/// `ctx.<f>.values().removeIf(v -> v == '<literal>')` as a
/// [`KnownPattern::RemoveMapValue`].
///
/// The MAP counterpart of [`parse_remove_list_value`], which declines this
/// because `<f>.values()` carries parentheses. A source that writes a sentinel
/// rather than omitting a field prunes the whole map in one line -- squid
/// writes `-` for "no value" in a dozen grok captures, so this single script
/// is what stands between its `_tmp` scratch map and every field derived from
/// it.
fn parse_remove_map_value(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let at = script.find(".values().removeIf(")?;
    let before = &script[..at];
    let subject = &before[before.rfind("ctx.")? + 4..];
    // A ctx FIELD and nothing else, on the same reading as the list version:
    // a call on a local or on an entry set is a different script.
    if !subject
        .chars()
        .all(|c| c.is_alphanumeric() || "._?".contains(c))
    {
        return None;
    }
    let field = clean_path(subject);
    let (_, lambda) = script[at..].split_once("->")?;
    let lambda = lambda.split(')').next()?;
    // ONE literal comparison, for the reason the list version gives: a
    // drop-empty predicate chains several sentinels, and claiming it on its
    // first literal would take the whole script for a single removal.
    if lambda.matches("==").count() != 1 || lambda.contains('|') || lambda.contains('&') {
        return None;
    }
    let (_, rhs) = lambda.split_once("==")?;
    let value = quoted_first(rhs)?.replace("\\\\", "\\");
    (!field.is_empty()).then_some(KnownPattern::RemoveMapValue { field, value })
}

/// Drop every ENTRY of a map whose value equals one literal.
///
/// Rebuilt by filtering rather than removed key by key: under `preserve_order`
/// the map is an `IndexMap`, and collecting the survivors in iteration order is
/// exactly what a run of `shift_remove` would leave, without going near the
/// forbidden `Map::remove`.
fn run_remove_map_value(event: &mut Event, field: &str, value: &str) -> bool {
    let Some(Value::Object(map)) = event.get(field).cloned() else {
        return true;
    };
    let kept: serde_json::Map<String, Value> = map
        .into_iter()
        .filter(|(_, member)| member.as_str() != Some(value))
        .collect();
    let _ = event.set(field, Value::Object(kept));
    true
}

/// `for (int i=0; i<ctx.<f>.length; i++) { if (ctx.<f>[i] == '<v>') {
/// ctx.<f>.remove(i); } }` as the same [`KnownPattern::RemoveListValue`]
/// [`parse_remove_list_value`] reads off `.removeIf(`.
///
/// The loop walks FORWARD while removing from the same list, so a match
/// immediately after a removed element is skipped -- upstream's own bug.
/// coredns only ever removes `QR`, which a DNS header carries at most once,
/// so the skip has no visible effect and the captured Elasticsearch output
/// matches a plain "drop every match" filter. Every literal between the
/// extracted field and value is matched exactly, so a longer script sharing
/// the same opening -- sysmon's V4MAPPED address conversion spells `for (def
/// i = 0; i < ctx.dns.resolved_ip.length; i++)` -- falls through instead of
/// losing the rest of its work.
fn parse_indexed_list_removal(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let tail = script.strip_prefix("for (int i=0; i<ctx.")?;
    let (path_raw, tail) = tail.split_once(".length; i++) {")?;
    if path_raw.is_empty() {
        return None;
    }
    let tail = tail.trim_start().strip_prefix("if (ctx.")?;
    let tail = tail.strip_prefix(path_raw)?;
    let tail = tail.strip_prefix("[i] == ")?;
    let value = quoted_first(tail)?;
    let tail = tail.trim_start().strip_prefix(&format!("'{value}') {{"))?;
    let tail = tail.trim_start().strip_prefix("ctx.")?;
    let tail = tail.strip_prefix(path_raw)?;
    let tail = tail.trim_start().strip_prefix(".remove(i);")?;
    let tail = tail.trim_start().strip_prefix('}')?; // closes the if
    let tail = tail.trim_start().strip_prefix('}')?; // closes the for
    if !tail.trim().is_empty() {
        return None;
    }

    let field = clean_path(path_raw);
    (!field.is_empty()).then_some(KnownPattern::RemoveListValue { field, value })
}

/// gcp audit's `related.entity`, transliterated.
///
/// The `isKubernetes` gate is the whole point of it: for a k8s cluster the
/// resource name, the response user and the principal email are all SUPPRESSED,
/// and adding them anyway put an API path and a service account on every
/// kubernetes audit event Elasticsearch leaves bare.
fn run_gcp_related_entity(event: &mut Event) -> bool {
    let proto = "json.protoPayload";
    let is_kubernetes = matches!(
        event.get_str("json.resource.type"),
        Some("k8s_cluster" | "gke_cluster" | "kubernetes")
    );

    let mut entities: Vec<String> = Vec::new();
    let add = |entities: &mut Vec<String>, value: Option<&Value>| {
        if let Some(text) = value.and_then(Value::as_str).filter(|t| !t.is_empty())
            && !entities.iter().any(|held| held == text)
        {
            entities.push(text.to_string());
        }
    };
    let add_path = |entities: &mut Vec<String>, path: &str| add(entities, event_get(event, path));
    let members = |path: &str| -> Vec<Value> {
        match event.get(path) {
            Some(Value::Array(items)) => items.clone(),
            _ => Vec::new(),
        }
    };

    add_path(&mut entities, &format!("{proto}.request.parent"));
    if !is_kubernetes {
        add_path(&mut entities, &format!("{proto}.resourceName"));
        add_path(&mut entities, &format!("{proto}.response.user"));
        add_path(
            &mut entities,
            &format!("{proto}.authenticationInfo.principalEmail"),
        );
    }
    for member in ["principalSubject", "serviceAccountKeyName"] {
        add_path(
            &mut entities,
            &format!("{proto}.authenticationInfo.{member}"),
        );
    }
    for info in members(&format!(
        "{proto}.authenticationInfo.serviceAccountDelegationInfo"
    )) {
        add(&mut entities, info.get("principalSubject"));
        for party in ["firstPartyPrincipal", "thirdPartyPrincipal"] {
            add(
                &mut entities,
                info.get(party).and_then(|p| p.get("principalEmail")),
            );
        }
    }

    match event.get_str(&format!("{proto}.serviceName")) {
        Some("compute.googleapis.com") => {
            for (list, member) in [
                ("networkInterfaces", "network"),
                ("serviceAccounts", "email"),
                ("disks", "source"),
            ] {
                for entry in members(&format!("{proto}.request.{list}")) {
                    add(&mut entities, entry.get(member));
                }
            }
        }
        Some("cloudresourcemanager.googleapis.com") => {
            for path in [
                format!("{proto}.request.policy.bindings"),
                format!("{proto}.response.bindings"),
            ] {
                for binding in members(&path) {
                    add(&mut entities, binding.get("role"));
                    if let Some(Value::Array(list)) = binding.get("members") {
                        for member in list {
                            add(&mut entities, Some(member));
                        }
                    }
                }
            }
        }
        Some("iamcredentials.googleapis.com") => {
            for entry in members(&format!("{proto}.metadata.identityDelegationChain")) {
                add(&mut entities, Some(&entry));
            }
        }
        _ => {}
    }

    if !entities.is_empty() {
        // A TreeSet, so the result is sorted -- the script says as much.
        entities.sort();
        let _ = event.set("related.entity", json!(entities));
    }
    true
}

/// Borrow one field, so the closures above can hold `&mut Vec` and still read.
fn event_get<'e>(event: &'e Event, path: &str) -> Option<&'e Value> {
    event.get(path)
}

/// mimecast's `related.*` collection: which paths feed the user and host sets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MailRelatedScript {
    /// Read as a display name -- added to the users whole.
    names: Vec<String>,
    /// Read as an email address -- the local part and the whole address join
    /// the users, the domain joins the hosts.
    addresses: Vec<String>,
    /// A list whose members carry `displayableName` and `emailAddress`.
    lists: Vec<String>,
    /// A map whose `emailAddress` REPLACES it on the event before anything
    /// else, so the field stops being an object.
    lift: Option<String>,
}

/// Read mimecast's `Populate related.* fields` script.
fn parse_mail_related(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let ctx_path_at = |text: &str| -> Option<String> {
        let path = text.strip_prefix("ctx.")?;
        let end = path
            .find(|c: char| !(c.is_alphanumeric() || c == '.' || c == '_' || c == '?'))
            .unwrap_or(path.len());
        Some(clean_path(&path[..end]))
    };

    let mut addresses = Vec::new();
    for segment in script.split("splitmail(").skip(1) {
        if let Some(path) = ctx_path_at(segment) {
            addresses.push(path);
        }
    }

    let mut names = Vec::new();
    for segment in script.split("users.add(").skip(1) {
        if let Some(path) = ctx_path_at(segment)
            && !addresses.contains(&path)
            && !names.contains(&path)
        {
            names.push(path);
        }
    }

    let mut lists = Vec::new();
    for segment in script.split("for (def ").skip(1) {
        if let Some((_, tail)) = segment.split_once(':')
            && let Some(path) = ctx_path_at(tail.trim_start())
        {
            lists.push(path);
        }
    }

    // `ctx.<p> = ctx.<p>.emailAddress` under an `instanceof Map` guard.
    let lift = script
        .find(".emailAddress;")
        .and_then(|at| script[..at].rfind("ctx.").map(|s| &script[s + 4..at]))
        .map(clean_path)
        .filter(|path| !path.is_empty());

    (!addresses.is_empty() || !names.is_empty()).then_some(KnownPattern::MailRelated(Box::new(
        MailRelatedScript {
            names,
            addresses,
            lists,
            lift,
        },
    )))
}

/// Collect every display name and email address the script names into sorted
/// `related.user` and `related.hosts` lists.
fn run_mail_related(event: &mut Event, script: &MailRelatedScript) -> bool {
    if let Some(path) = &script.lift
        && event.get(path).is_some_and(Value::is_object)
    {
        let address = event.get(&format!("{path}.emailAddress")).cloned();
        match address {
            Some(value) => {
                let _ = event.set(path, value);
            }
            // Painless writes the null back, and the field stops being a map.
            None => {
                let _ = event.set(path, Value::Null);
            }
        }
    }

    let mut users: Vec<String> = Vec::new();
    let mut hosts: Vec<String> = Vec::new();
    let add = |set: &mut Vec<String>, value: String| {
        if !set.contains(&value) {
            set.push(value);
        }
    };

    for path in &script.names {
        if let Some(name) = event.get_str(path).map(str::to_string) {
            add(&mut users, name);
        }
    }
    let split = |users: &mut Vec<String>, hosts: &mut Vec<String>, address: &str| {
        if let Some((local, domain)) = address.split_once('@')
            && !domain.contains('@')
        {
            add(users, local.to_string());
            add(hosts, domain.to_string());
        }
        add(users, address.to_string());
    };
    for path in &script.addresses {
        if let Some(address) = event.get_str(path).map(str::to_string) {
            split(&mut users, &mut hosts, &address);
        }
    }
    for path in &script.lists {
        // Painless throws on a missing list, and the processor swallows it --
        // which means NOTHING is written, because the writes come after.
        let Some(Value::Array(members)) = event.get(path).cloned() else {
            return true;
        };
        for member in members {
            if let Some(name) = member.get("displayableName").and_then(Value::as_str) {
                add(&mut users, name.to_string());
            }
            if let Some(address) = member.get("emailAddress").and_then(Value::as_str) {
                split(&mut users, &mut hosts, address);
            }
        }
    }

    if !users.is_empty() && !event.has_value("related.user") {
        users.sort();
        let _ = event.set("related.user", json!(users));
    }
    if !hosts.is_empty() && !event.has_value("related.hosts") {
        hosts.sort();
        let _ = event.set("related.hosts", json!(hosts));
    }
    true
}

/// The EC2 members inspector reads off one resource, and where each lands.
const INSPECTOR_EC2_FIELDS: [(&str, &str); 3] = [
    ("type", "cloud.machine.type"),
    ("type", "host.type"),
    ("platform", "host.os.platform"),
];

/// The ECS `host.os.type` a platform name implies, in the script's own order.
const INSPECTOR_OS_TYPES: [(&str, &str); 3] = [
    ("windows", "windows"),
    ("linux", "linux"),
    ("macos", "macos"),
];

/// inspector's resource extraction: the single-resource script writes each
/// field as a SCALAR, its multi-resource sibling appends to a list. One
/// implementation, because the two scripts differ only in that.
fn run_inspector_resources(event: &mut Event, multi: bool) -> bool {
    let Some(Value::Array(resources)) = event.get("aws.inspector.resources").cloned() else {
        return true;
    };
    // Each script's own guard: `size() == 1` for one, `size() > 1` for the
    // other, so the wrong one leaves the event alone.
    if multi == (resources.len() <= 1) {
        return true;
    }

    let write = |event: &mut Event, path: &str, value: Option<Value>| {
        let Some(value) = value.filter(|v| !v.is_null()) else {
            return;
        };
        if multi {
            let _ = event.append(path, value);
        } else {
            let _ = event.set(path, value);
        }
    };

    for res in &resources {
        let member = |path: &[&str]| -> Option<Value> {
            path.iter()
                .try_fold(res, |value, key| value.get(*key))
                .cloned()
        };

        write(event, "resource.id", member(&["id"]));
        write(event, "resource.name", member(&["tags", "Name"]));
        write(event, "resource.type", member(&["type"]));
        write(event, "cloud.region", member(&["region"]));

        if member(&["type"]).as_ref().and_then(Value::as_str) != Some("AWS_EC2_INSTANCE") {
            continue;
        }
        write(event, "cloud.instance.id", member(&["id"]));
        write(event, "host.id", member(&["id"]));
        write(event, "host.name", member(&["tags", "Name"]));
        for (key, target) in INSPECTOR_EC2_FIELDS {
            write(
                event,
                target,
                member(&["details", "aws", "ec2_instance", key]),
            );
        }
        // `host.ip` is a list in BOTH scripts -- the single-resource one seeds
        // it with `[]` and then adds, rather than assigning.
        for key in ["ipv4_addresses", "ipv6_addresses"] {
            if let Some(Value::Array(addresses)) = member(&["details", "aws", "ec2_instance", key])
            {
                for address in addresses {
                    let _ = event.append("host.ip", address);
                }
            }
        }
        // And `host.os.type` is assigned in both, even in the multi script.
        if let Some(platform) = member(&["details", "aws", "ec2_instance", "platform"])
            .as_ref()
            .and_then(Value::as_str)
            .map(str::to_lowercase)
            && let Some((_, os)) = INSPECTOR_OS_TYPES
                .iter()
                .find(|(needle, _)| platform.contains(needle))
        {
            let _ = event.set("host.os.type", json!(*os));
        }
    }
    true
}

/// securityhub's single-resource extraction, transliterated. Only the
/// one-resource case is handled here, as the script itself says; the
/// multi-resource sibling is a separate script.
#[allow(clippy::too_many_lines)] // A transliteration; splitting it would hide the script's order.
fn run_securityhub_resource(event: &mut Event, source: &str) -> bool {
    let Some(Value::Array(resources)) = event.get(source).cloned() else {
        return true;
    };

    for (path, empty) in [
        ("resource", json!({})),
        ("user", json!({})),
        ("host", json!({})),
        ("host.ip", json!([])),
        ("orchestrator", json!({})),
        ("orchestrator.cluster", json!({})),
        ("orchestrator.resource", json!({})),
        ("cloud", json!({})),
        ("cloud.instance", json!({})),
        ("cloud.service", json!({})),
    ] {
        if !event.has_value(path) {
            let _ = event.set(path, empty);
        }
    }

    if resources.len() != 1 {
        return true;
    }
    let res = &resources[0];
    let kind = res
        .get("Type")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    if let Some(v) = res.get("Type").cloned() {
        let _ = event.set("resource.type", v);
    }
    if let Some(v) = res.get("Id").cloned() {
        let _ = event.set("resource.id", v);
    }
    let Some(id) = res.get("Id").and_then(Value::as_str).map(str::to_string) else {
        return true;
    };
    let tokens: Vec<&str> = id.split(':').collect();

    let details = res.get("Details");
    let detail = |member: &str| -> Option<&Value> {
        details
            .and_then(|d| d.get(&kind))
            .and_then(|t| t.get(member))
    };
    let res_name = detail("Name").and_then(Value::as_str).map_or_else(
        || (*tokens.last().unwrap_or(&"")).to_string(),
        str::to_string,
    );
    let _ = event.set("resource.name", json!(res_name.clone()));

    if details.is_some() {
        for (wanted, member, target) in [
            ("AwsIamUser", "UserName", "user.name"),
            ("AwsIamAccessKey", "UserName", "user.name"),
            ("AwsS3Bucket", "OwnerName", "user.name"),
            ("AwsIamUser", "UserId", "user.id"),
            ("AwsS3Bucket", "OwnerId", "user.id"),
            ("AwsEcsContainer", "Name", "host.name"),
        ] {
            if kind == wanted
                && let Some(v) = detail(member).cloned()
            {
                let _ = event.set(target, v);
            }
        }

        if kind == "AwsEc2Instance" {
            for member in ["IpV4Addresses", "IpV6Addresses"] {
                if let Some(Value::Array(addresses)) = detail(member) {
                    for address in addresses.clone() {
                        if address.is_string() {
                            let _ = event.append("host.ip", address);
                        }
                    }
                }
            }
        }

        // The ECS arm reads AwsEcsCluster's details whatever the type says.
        if matches!(kind.as_str(), "AwsEcsCluster" | "AwsEcsTask")
            && let Some(v) = details
                .and_then(|d| d.get("AwsEcsCluster"))
                .and_then(|t| t.get("ClusterArn"))
                .cloned()
        {
            let _ = event.set("orchestrator.cluster.id", v);
        }
        for (member, target) in [
            ("Arn", "orchestrator.cluster.id"),
            ("Name", "orchestrator.cluster.name"),
            ("Version", "orchestrator.cluster.version"),
            ("Endpoint", "orchestrator.cluster.url"),
        ] {
            if kind == "AwsEksCluster"
                && let Some(v) = detail(member).cloned()
            {
                let _ = event.set(target, v);
            }
        }
        if kind == "AwsEcsCluster"
            && let Some(v) = detail("ClusterName").cloned()
        {
            let _ = event.set("orchestrator.cluster.name", v);
        }

        if matches!(
            kind.as_str(),
            "AwsEc2Subnet" | "AwsRedshiftCluster" | "AwsDmsReplicationInstance"
        ) && let Some(v) = detail("AvailabilityZone").cloned()
        {
            let _ = event.set("cloud.availability_zone", v);
        }
        if matches!(
            kind.as_str(),
            "AwsEc2VpcEndpointService" | "AwsElbLoadBalancer" | "AwsRdsDbCluster"
        ) && let Some(Value::Array(zones)) = detail("AvailabilityZones")
        {
            for zone in zones.clone() {
                let _ = event.set("cloud.availability_zone", zone);
            }
        }
        if kind == "AwsAutoScalingAutoScalingGroup"
            && let Some(Value::Array(zones)) = detail("AvailabilityZones")
        {
            for zone in zones.clone() {
                if let Some(v) = zone.get("Value").cloned() {
                    let _ = event.set("cloud.availability_zone", v);
                }
            }
        }
        if kind == "AwsEc2LaunchTemplate"
            && let Some(v) = detail("LaunchTemplateData")
                .and_then(|d| d.get("Placement"))
                .and_then(|p| p.get("AvailabilityZone"))
                .cloned()
        {
            let _ = event.set("cloud.availability_zone", v);
        }
        if kind == "AwsElbv2LoadBalancer"
            && let Some(Value::Array(zones)) = detail("AvailabilityZones")
        {
            for zone in zones.clone() {
                if let Some(v) = zone.get("ZoneName").cloned() {
                    let _ = event.set("cloud.availability_zone", v);
                }
            }
        }
    }

    if kind == "AwsEc2Instance" {
        let _ = event.set("host.id", json!(id.clone()));
        let _ = event.set("cloud.instance.id", json!(id.clone()));
        let _ = event.set("cloud.instance.name", json!(res_name.clone()));
    }
    if kind.starts_with("AwsEks") || kind.starts_with("AwsEcs") {
        let _ = event.set("orchestrator.resource.id", json!(id.clone()));
        let _ = event.set("orchestrator.resource.name", json!(res_name));
        let _ = event.set("orchestrator.resource.type", json!(kind.clone()));
        let orchestrator = if kind.starts_with("AwsEks") {
            "kubernetes"
        } else {
            "ecs"
        };
        let _ = event.set("orchestrator.type", json!(orchestrator));
    }
    if tokens.len() > 2 {
        let _ = event.set("cloud.service.name", json!(tokens[2]));
    }
    true
}

/// One arm of a loop-over-category ladder: the members it matches, any extra
/// ctx equality it requires, the LIST literal it assigns, and whether it
/// only fires while the target is still unset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CategoryArm {
    list: String,
    literals: Vec<String>,
    extras: Vec<(String, String)>,
    target: String,
    values: Vec<String>,
    only_if_unset: bool,
}

/// Read the sequential `for (<v> in ctx.<list>) { if (<v> == 'a' || ...) {
/// ctx.<t> = ['x']; break; } }` blocks, later ones gated on the target
/// still being null.
fn parse_category_type_ladder(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let mut arms = Vec::new();
    let mut cursor = 0usize;
    while let Some(rel) = script[cursor..].find("for (") {
        let at = cursor + rel;
        let only_if_unset = script[..at]
            .rfind("if (")
            .is_some_and(|guard| script[guard..at].contains("== null"));

        let rest = &script[at + "for (".len()..];
        let (header, tail) = rest.split_once(')')?;
        let (var, list) = header.split_once(" in ctx.")?;
        let var = var.trim();

        let (condition, tail) = tail.split_once("if (")?.1.split_once(") {")?;
        let mut literals = Vec::new();
        let mut extras = Vec::new();
        for disjunct in condition.split("||") {
            for term in disjunct.split("&&") {
                let Some((lhs, rhs)) = term.split_once("==") else {
                    continue;
                };
                let Some(literal) = quoted_first(rhs) else {
                    continue;
                };
                let lhs = lhs.trim();
                if lhs == var {
                    literals.push(literal);
                } else if lhs.contains("ctx") {
                    let cleaned =
                        clean_path(lhs.trim_start_matches("ctx?.").trim_start_matches("ctx."));
                    extras.push((cleaned, literal));
                }
            }
        }

        let (assignment, _) = tail.split_once(';')?;
        let (lhs, rhs) = assignment.split_once('=')?;
        let target = clean_path(lhs.trim().strip_prefix("ctx.")?);
        let values = quoted_members(rhs);
        if literals.is_empty() || values.is_empty() {
            return None;
        }

        arms.push(CategoryArm {
            list: clean_path(list.trim()),
            literals,
            extras,
            target,
            values,
            only_if_unset,
        });
        cursor = at + "for (".len();
    }

    (!arms.is_empty()).then_some(KnownPattern::CategoryTypeLadder(arms))
}

/// Run the ladder: each arm in order, matching any member of its list.
fn run_category_type_ladder(event: &mut Event, arms: &[CategoryArm]) -> bool {
    for arm in arms {
        if arm.only_if_unset && event.has_value(&arm.target) {
            continue;
        }
        let extras_hold = arm
            .extras
            .iter()
            .all(|(path, value)| event.get_str(path) == Some(value.as_str()));
        if !extras_hold {
            continue;
        }
        let Some(Value::Array(members)) = event.get(&arm.list) else {
            continue;
        };
        let hit = members
            .iter()
            .filter_map(Value::as_str)
            .any(|member| arm.literals.iter().any(|l| l == member));
        if hit {
            let values: Vec<Value> = arm
                .values
                .iter()
                .map(|v| Value::String(v.clone()))
                .collect();
            let _ = event.set(&arm.target, Value::Array(values));
        }
    }
    true
}

/// Read the source and target of a snake-cased map copy.
///
/// `for (def item : ctx.<source>.entrySet())` names the map being walked, and
/// the trailing `ctx.<target> = <local>` names where the rebuilt one lands.
/// Both halves must be present: the loop alone could be any of a dozen patterns,
/// and the assignment alone says nothing about what is being copied.
fn parse_snake_key_map_copy(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let loop_at = script.find(" : ctx.")?;
    let (source, _) = script[loop_at + " : ctx.".len()..].split_once(".entrySet()")?;

    let assignment = script
        .lines()
        .rev()
        .map(|line| line.trim().trim_end_matches(';'))
        .find(|line| line.starts_with("ctx.") && line.contains(" = "))?;
    let (target, _) = assignment.split_once(" = ")?;

    Some(KnownPattern::SnakeKeyMapCopy {
        source: clean_path(source),
        target: clean_path(&target["ctx.".len()..]),
    })
}

/// Read the angle-strip helper's call sites: `ctx.<p> = <name>(ctx.<p>);`
/// scalars, and the loop rebuilding a list through the same helper.
fn parse_strip_angle_pairs(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let name_at = script.find("(def input)")?;
    let head = &script[..name_at];
    let name = head
        .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()?;
    if name.is_empty() {
        return None;
    }

    let mut scalars = Vec::new();
    for site in script.split(&format!("= {name}(ctx.")).skip(1) {
        if let Some((path, _)) = site.split_once(')') {
            scalars.push(clean_path(path));
        }
    }

    let mut lists = Vec::new();
    for site in script.split("for (").skip(1) {
        if let Some(at) = site.find(" in ctx.")
            && let Some((path, rest)) = site[at + " in ctx.".len()..].split_once(')')
            && rest
                .split('}')
                .next()
                .is_some_and(|body| body.contains(&format!("{name}(")))
        {
            lists.push(clean_path(path));
        }
    }

    (!scalars.is_empty() || !lists.is_empty())
        .then_some(KnownPattern::StripAnglePairs { scalars, lists })
}

/// One field unwrapped from a surrounding pair of characters, in place.
///
/// ```painless
/// if (ctx.a.b.containsKey('message') && ctx.a.b.message != null && ctx.a.b.message instanceof String) {
///   if (ctx.a.b.message.startsWith('"') && ctx.a.b.message.endsWith('"') && ctx.a.b.message.length() >= 2) {
///     ctx.a.b.message = ctx.a.b.message.substring(1, ctx.a.b.message.length() - 1);
///   }
/// }
/// ```
///
/// `infoblox_threat_defense`'s CEF `message` arrives quoted, and it is copied
/// to `message` and renamed to `infoblox_threat_defense.event.message` after
/// this runs -- so a miss here is wrong on all three.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StripSurroundingPair {
    field: String,
    open: char,
    close: char,
}

/// Read the pair, the field, and the cut that takes one character off each end.
///
/// The cut is required to be exactly `substring(1, <field>.length() - 1)`: a
/// script cutting a different width means different characters, and reading it
/// as this one would truncate the value.
fn parse_strip_surrounding_pair(script: &str) -> Option<StripSurroundingPair> {
    let opens_at = script.find(".startsWith(")?;
    let open = single_char(&quoted_argument(&script[opens_at + ".startsWith(".len()..])?)?;
    let field = painless_path(&script[..opens_at])?;

    let closes_at = script.find(".endsWith(")?;
    let close = single_char(&quoted_argument(&script[closes_at + ".endsWith(".len()..])?)?;
    if painless_path(&script[..closes_at])? != field {
        return None;
    }

    // The write lands back on the same field, off the same field's own value.
    let cuts_at = script.find(".substring(")?;
    let head = &script[..cuts_at];
    if painless_path(head)? != field {
        return None;
    }
    let (assigned, _) = head.rsplit_once('=')?;
    if painless_path(assigned)? != field {
        return None;
    }

    let (start, tail) = script[cuts_at + ".substring(".len()..].split_once(',')?;
    if start.trim() != "1" {
        return None;
    }
    let (measured, after) = tail.split_once(".length()")?;
    if painless_path(measured)? != field || after.split(')').next()?.replace(' ', "") != "-1" {
        return None;
    }

    Some(StripSurroundingPair { field, open, close })
}

/// A literal of exactly one character, or `None`.
fn single_char(literal: &str) -> Option<char> {
    let mut chars = literal.chars();
    let first = chars.next()?;
    chars.next().is_none().then_some(first)
}

/// Strip the pair where both ends carry it and something sits between them.
fn run_strip_surrounding_pair(event: &mut Event, pattern: &StripSurroundingPair) -> bool {
    // Absent, or not a string: the script's own `containsKey` and `instanceof
    // String` guards. Declining rather than claiming it keeps the miss
    // countable, since a matcher answering true reads as handled.
    let Some(text) = event.get_str(&pattern.field) else {
        return false;
    };
    let stripped = (text.chars().count() >= 2
        && text.starts_with(pattern.open)
        && text.ends_with(pattern.close))
    .then(|| text[pattern.open.len_utf8()..text.len() - pattern.close.len_utf8()].to_string());

    // A present value carrying no pair is the inner `if` not holding, which
    // writes nothing and is still a run.
    if let Some(stripped) = stripped {
        let _ = event.set(&pattern.field, json!(stripped));
    }
    true
}

/// cloudtrail's resources pass: ARN and accountId rename to their snake
/// names (appended, as a Java put is), and duplicates of the
/// `arn_account_type` composite collapse -- last one wins, keeping the first's
/// position, and the survivors come out in the dedup map's own HASH order.
fn run_resources_rename_dedup(event: &mut Event, source: &str) -> bool {
    use crate::painless_helpers::{java_bucket, java_table_size, painless_to_string};

    let Some(Value::Array(items)) = event.get(source).cloned() else {
        return true;
    };

    let mut unique: Vec<(String, Value)> = Vec::new();
    for item in items {
        let Value::Object(original) = item else {
            continue;
        };
        let mut resource = original;
        if let Some(value) = resource.shift_remove("ARN") {
            resource.insert("arn".into(), value);
        }
        if let Some(value) = resource.shift_remove("accountId") {
            resource.insert("account_id".into(), value);
        }
        let part = |k: &str| resource.get(k).map(painless_to_string).unwrap_or_default();
        let key = format!("{}_{}_{}", part("arn"), part("account_id"), part("type"));
        let value = Value::Object(resource);
        if let Some(existing) = unique.iter_mut().find(|(k, _)| *k == key) {
            existing.1 = value;
        } else {
            unique.push((key, value));
        }
    }

    let table = java_table_size(unique.len());
    let mut ordered: Vec<(usize, usize, Value)> = unique
        .into_iter()
        .enumerate()
        .map(|(position, (key, value))| (java_bucket(&key, table), position, value))
        .collect();
    ordered.sort_by_key(|(bucket, position, _)| (*bucket, *position));

    let _ = event.set(
        source,
        Value::Array(ordered.into_iter().map(|(_, _, v)| v).collect()),
    );
    true
}

/// Read `ctx.put("<t>", new HashMap()); for (<v> in ctx.<s>) {
/// ctx.<t>.put(<v>.<k>, <v>.<val>); }` as a [`KnownPattern::NameValueFold`].
fn parse_name_value_fold(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let at = script.find(", new HashMap())")?;
    let before = &script[..at];
    let open = before.rfind(".put(")?;
    let target = quoted_first(&before[open..])?;

    let for_at = script.find(" in ctx.")?;
    let after = &script[for_at + " in ctx.".len()..];
    let (source, _) = after.split_once(')')?;
    let var = script[..for_at]
        .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()?;

    let call = format!("{target}.put({var}.");
    let call_at = script.find(&call)?;
    let arguments = &script[call_at + call.len()..];
    let (key_member, rest) = arguments.split_once(',')?;
    let value_member = rest
        .trim()
        .strip_prefix(&format!("{var}."))?
        .split(')')
        .next()?;

    Some(KnownPattern::NameValueFold {
        source: clean_path(source),
        target,
        key_member: key_member.trim().to_string(),
        value_member: value_member.trim().to_string(),
    })
}

/// The same fold written as an INDEXED loop -- aws/waf's request headers.
///
/// `ctx.<t>[ctx.<s>[i].name] = ctx.<s>[i].value` inside
/// `for (def i = 0; i < ctx.<s>.length; i++)`. The outcome is
/// [`KnownPattern::NameValueFold`]'s, so only the reading differs: the map is
/// subscripted rather than `put` to, and the element is reached by index
/// rather than by a loop variable.
fn parse_indexed_name_value_fold(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let assignment = script
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("ctx.") && line.contains("] = ctx."))?;
    let (lhs, rhs) = assignment.split_once("] = ctx.")?;

    // `ctx.<target>[ctx.<source>[i].<key>`
    let (target, subscript) = lhs.strip_prefix("ctx.")?.split_once("[ctx.")?;
    let (source, key_tail) = subscript.split_once('[')?;
    let key_member = key_tail.split_once("].")?.1;

    // `<source>[i].<value>;`
    let value_member = rhs.trim_end_matches(';').split_once("].")?.1;

    let source = clean_path(source);
    let target = clean_path(target);
    (!source.is_empty() && !target.is_empty() && !key_member.is_empty() && !value_member.is_empty())
        .then_some(KnownPattern::NameValueFold {
            source,
            target,
            key_member: key_member.to_string(),
            value_member: value_member.to_string(),
        })
}

/// The fold itself: the target becomes a fresh map of each element's
/// key member to its value member; an element with no key is skipped where
/// Java would take a null key JSON cannot spell.
fn run_name_value_fold(
    event: &mut Event,
    source: &str,
    target: &str,
    key_member: &str,
    value_member: &str,
) -> bool {
    let Some(Value::Array(items)) = event.get(source).cloned() else {
        return true;
    };
    let mut folded = Map::new();
    for item in &items {
        let Some(key) = item.get(key_member).and_then(Value::as_str) else {
            continue;
        };
        let value = item.get(value_member).cloned().unwrap_or(Value::Null);
        folded.insert(key.to_string(), value);
    }
    let _ = event.set(target, Value::Object(folded));
    true
}

/// Read the outcome-from-tags pattern: the action plus a dot prefixes the tag
/// whose value decides success or failure.
fn parse_outcome_from_tags(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let plus = script.find(" + '.'")?;
    let before = &script[..plus];
    let action_field = clean_path(&before[before.rfind("ctx.")? + 4..]);

    let for_at = script.find(" in ctx.")?;
    let after = &script[for_at + " in ctx.".len()..];
    let (tags, _) = after.split_once(')')?;

    let put_at = script.find(".put(\"outcome\"")?;
    let head = &script[..put_at];
    let target = clean_path(&head[head.rfind("ctx.")? + 4..]);

    Some(KnownPattern::OutcomeFromTags {
        action_field,
        tags: clean_path(tags),
        target: format!("{target}.outcome"),
    })
}

/// The LAST tag whose name starts with `<action>.` decides: "true" is
/// success, "false" failure, case folded. Writing needs the target's parent
/// to exist, which is where the script's own put would have thrown.
fn run_outcome_from_tags(event: &mut Event, action_field: &str, tags: &str, target: &str) -> bool {
    let Some(action) = event.get_str(action_field).map(str::to_string) else {
        return true;
    };
    let Some(Value::Array(items)) = event.get(tags).cloned() else {
        return true;
    };
    let Some((parent, _)) = target.rsplit_once('.') else {
        return true;
    };
    if event.get(parent).is_none() {
        return true;
    }

    let prefix = format!("{action}.");
    let mut outcome: Option<&str> = None;
    for item in &items {
        let (Some(name), Some(value)) = (
            item.get("name").and_then(Value::as_str),
            item.get("value").and_then(Value::as_str),
        ) else {
            continue;
        };
        if name.starts_with(&prefix) {
            match value.to_lowercase().as_str() {
                "true" => outcome = Some("success"),
                "false" => outcome = Some("failure"),
                _ => {}
            }
        }
    }
    let _ = event.set(target, outcome.map_or(Value::Null, Value::from));
    true
}

/// Read `ctx.<t> = ctx.<s>.splitOnToken("<sep>").length;` as a
/// [`KnownPattern::TokenCount`].
fn parse_token_count(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let at = script.find(".splitOnToken(")?;
    let after = &script[at + ".splitOnToken(".len()..];
    let (arguments, rest) = after.split_once(')')?;
    if !rest.starts_with(".length") {
        return None;
    }
    let separator = quoted_first(arguments)?;

    let before = &script[..at];
    let source = clean_path(&before[before.rfind("ctx.")? + 4..]);
    let eq = before.rfind(" = ")?;
    let head = &before[..eq];
    let target = clean_path(&head[head.rfind("ctx.")? + 4..]);
    if target == source || separator.is_empty() {
        return None;
    }

    Some(KnownPattern::TokenCount {
        source,
        separator,
        target,
    })
}

/// The tail of every list member whose named field carries a prefix.
///
/// ```painless
/// def prefix = 'Downloaded package: ';
/// def packages = new ArrayList();
/// for (def ev : ctx.json.events) {
///   def desc = ev?.event_description;
///   if (desc instanceof String && desc.startsWith(prefix)) {
///     packages.add(desc.substring(prefix.length()));
///   }
/// }
/// if (!packages.isEmpty()) {
///   if (ctx.kolide.auth.downloaded_packages == null) {
///     ctx.kolide.auth.downloaded_packages = packages;
///   } else {
///     ctx.kolide.auth.downloaded_packages.addAll(packages);
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SuffixesByPrefix {
    source: String,
    member: String,
    prefix: String,
    target: String,
}

/// The identifier a fragment ends on.
fn parse_suffixes_by_prefix(script: &str) -> Option<SuffixesByPrefix> {
    use crate::painless_params::clean_path;

    let (head, tail) = script.split_once(".substring(")?;
    let (accumulator, item) = head.rsplit_once(".add(")?;
    let accumulator = identifier_before(accumulator)?;
    let item = identifier_before(item)?;
    let prefix = quoted_first(declared_expression(
        script,
        tail.split_once(".length()")?.0.trim(),
    )?)?;

    let source = script
        .split_once("for (")?
        .1
        .split_once(" : ctx.")?
        .1
        .split_once(')')?
        .0;
    let member = declared_expression(script, item)?.trim().split_once('.')?.1;
    let target = script
        .split_once(&format!("= {accumulator};"))?
        .0
        .rsplit_once("ctx.")?
        .1;

    Some(SuffixesByPrefix {
        source: clean_path(source.trim()),
        member: member.trim().to_string(),
        prefix,
        target: clean_path(target.trim()),
    })
}

/// Collect the tails, appending where the target already holds a list.
fn run_suffixes_by_prefix(event: &mut Event, pattern: &SuffixesByPrefix) -> bool {
    let Some(Value::Array(items)) = event.get(&pattern.source) else {
        return true;
    };
    let collected: Vec<Value> = items
        .iter()
        .filter_map(|item| item.get(&pattern.member)?.as_str())
        .filter_map(|text| text.strip_prefix(pattern.prefix.as_str()))
        .map(|tail| Value::String(tail.to_string()))
        .collect();
    if collected.is_empty() {
        return true;
    }

    let grown = match event.get(&pattern.target) {
        Some(Value::Array(existing)) => {
            let mut grown = existing.clone();
            grown.extend(collected);
            Value::Array(grown)
        }
        _ => Value::Array(collected),
    };
    let _ = event.set(&pattern.target, grown);
    true
}

/// Read `def <l> = []; <l>.add(ctx.<s>); ctx.<t> = <l>;` or its one-line
/// spelling `ctx.<t> = [ctx.<s>];` as a [`KnownPattern::WrapValueInList`].
///
/// The literal is written with and without a space inside the bracket:
/// `amazon_security_lake` closes it up and kolide's `osquery_status` does not.
fn parse_wrap_value_in_list(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let literal = ["= [ctx.", "= [ ctx."]
        .iter()
        .find_map(|open| script.find(open).map(|at| (at, open.len())));

    let (source, target) = if let Some((at, opened)) = literal {
        // The list literal holds the source outright, so there is no local.
        let source = script[at + opened..].split(']').next()?;
        let head = &script[..at];
        (
            clean_path(source),
            clean_path(&head[head.rfind("ctx.")? + 4..]),
        )
    } else {
        let add_at = script.find(".add(ctx.")?;
        let after = &script[add_at + ".add(ctx.".len()..];
        let (source, _) = after.split_once(')')?;

        let local = script[..add_at]
            .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
            .next()?;
        let store = format!(" = {local};");
        let store_at = script.rfind(&store)?;
        let before = &script[..store_at];
        (
            clean_path(source),
            clean_path(&before[before.rfind("ctx.")? + 4..]),
        )
    };

    // A multi-element literal reads as one unusable path, and the runner would
    // then claim the script and write nothing.
    if !source
        .chars()
        .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_'))
    {
        return None;
    }

    // The removal has to name the source's OWN parent, or an unrelated field
    // with the same leaf name would take the source's place.
    let (parent, leaf) = source.rsplit_once('.').unwrap_or(("", source.as_str()));
    let owner = if parent.is_empty() {
        "ctx".to_string()
    } else {
        format!("ctx.{parent}")
    };
    let remove_source = script.contains(&format!("{owner}.remove('{leaf}')"))
        || script.contains(&format!("{owner}.remove(\"{leaf}\")"));

    Some(KnownPattern::WrapValueInList {
        source,
        target,
        remove_source,
    })
}

/// Read `ctx.<t> = ctx.<a>[ctx.<a>.length-1];` as a
/// [`KnownPattern::LastElement`].
fn parse_last_element(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let at = script.find("[ctx.")?;
    let after = &script[at + "[ctx.".len()..];
    let array = clean_path(after.split_once(".length-1]")?.0);

    let before = &script[..at];
    let subject = clean_path(&before[before.rfind("ctx.")? + 4..]);
    if subject != array {
        return None;
    }
    let (lhs, _) = before.split_once('=')?;
    let target = clean_path(lhs.trim().strip_prefix("ctx.")?);
    Some(KnownPattern::LastElement { array, target })
}

/// Read `ctx.<a>[i] = ctx.<a>[i].trim()` as a [`KnownPattern::TrimListInPlace`].
fn parse_trim_list(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let at = script.find(".trim()")?;
    let before = &script[..at];
    let read = clean_path(before[before.rfind("ctx.")? + 4..].strip_suffix("[i]")?);

    // The same list on both sides, or this is some other loop entirely. The
    // LAST `=` before the read is the assignment: the loop header has its own.
    let (assignment, _) = before.rsplit_once('=')?;
    let written = clean_path(
        assignment[assignment.rfind("ctx.")? + 4..]
            .trim()
            .strip_suffix("[i]")?,
    );
    (read == written && !read.is_empty()).then_some(KnownPattern::TrimListInPlace(read))
}

/// Read cloudfront's localhost edge case as a [`KnownPattern::StartsWithAppend`]:
/// a constant appended to a list when a member of another list has a prefix.
fn parse_starts_with_append(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let at = script.find(".startsWith(")?;
    let prefix = quoted_argument(&script[at + ".startsWith(".len()..])?;

    let loop_at = script.find(" : ctx")?;
    let after = &script[loop_at + " : ctx".len()..];
    let source = clean_path(
        after
            .trim_start_matches(['.', '?'])
            .split([')', ' ', ';'])
            .next()?,
    );

    // The append comes AFTER the prefix test, which is what makes it the
    // consequence rather than some earlier write.
    let add_at = at + script[at..].find(".add(")?;
    let value = quoted_argument(&script[add_at + ".add(".len()..])?;
    let target = bracket_path(&script[..add_at])?;

    (!source.is_empty() && !target.is_empty()).then_some(KnownPattern::StartsWithAppend {
        source,
        prefix,
        target,
        value,
    })
}

/// The contents of the leading `'...'` or `"..."` of an argument list.
fn quoted_argument(text: &str) -> Option<String> {
    let quote = text.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    let rest = &text[quote.len_utf8()..];
    Some(rest[..rest.find(quote)?].to_string())
}

/// `ctx['a']['b']` at the end of `text`, as the dotted path `a.b`.
fn bracket_path(text: &str) -> Option<String> {
    let start = text.rfind("ctx[")?;
    let mut path = String::new();
    let mut rest = &text[start + 3..];
    while let Some(open) = rest.strip_prefix('[') {
        let name = quoted_argument(open)?;
        if !path.is_empty() {
            path.push('.');
        }
        path.push_str(&name);
        rest = &rest[open.find(']')? + 2..];
    }
    (!path.is_empty()).then_some(path)
}

/// A string built up piece by piece, each piece guarded on the field it names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConcatScript {
    /// Where the finished string lands, when it is not empty.
    target: String,
    /// One per `+=`, in order. A clause whose fields are not all present and
    /// non-empty contributes nothing, which is what its own `if` says.
    clauses: Vec<Vec<ConcatTerm>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ConcatTerm {
    Literal(String),
    Field(String),
}

/// Read cloudfront's `url.full` assembly as a [`KnownPattern::ConcatParts`].
///
/// `def full = ""` then a run of guarded `full += ...`, and the result assigned
/// to a ctx field when it came to something.
fn parse_concat_parts(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let at = script.find("def ")?;
    let after = &script[at + 4..];
    let (var, _) = after.split_once('=')?;
    let var = var.trim();
    if var.is_empty() || var.contains(char::is_whitespace) {
        return None;
    }

    let append = format!("{var} +=");
    let mut clauses = Vec::new();
    let mut rest = after;
    while let Some(start) = rest.find(&append) {
        let expression = &rest[start + append.len()..];
        let (expression, tail) = expression.split_once(';')?;
        clauses.push(parse_concat_terms(expression)?);
        rest = tail;
    }
    if clauses.len() < 2 {
        return None;
    }

    // The assignment out: `ctx.<target> = <var>`, the only place the finished
    // string can go.
    let assignment = script.rfind(&format!("= {var}"))?;
    let before = &script[..assignment];
    let target = clean_path(&before[before.rfind("ctx.")? + 4..]);
    (!target.is_empty()).then_some(KnownPattern::ConcatParts(ConcatScript { target, clauses }))
}

/// One `+=` expression as its literal and ctx-field terms.
fn parse_concat_terms(expression: &str) -> Option<Vec<ConcatTerm>> {
    let mut terms = Vec::new();
    for piece in split_outside_quotes(expression, '+') {
        let piece = piece.trim();
        if piece.is_empty() {
            continue;
        }
        if let Some(literal) = piece.strip_prefix('"').and_then(|p| p.strip_suffix('"')) {
            terms.push(ConcatTerm::Literal(literal.to_string()));
        } else {
            let path = piece.strip_prefix("ctx.").or(piece.strip_prefix("ctx?."))?;
            terms.push(ConcatTerm::Field(crate::painless_params::clean_path(path)));
        }
    }
    (!terms.is_empty()).then_some(terms)
}

/// Split on `sep`, ignoring any that sits inside a double-quoted literal.
fn split_outside_quotes(text: &str, sep: char) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut quoted = false;
    for (at, c) in text.char_indices() {
        if c == '"' {
            quoted = !quoted;
        } else if c == sep && !quoted {
            parts.push(&text[start..at]);
            start = at + c.len_utf8();
        }
    }
    parts.push(&text[start..]);
    parts
}

/// A string assembled by ONE assignment: named fields and literal separators
/// joined with `+`.
///
/// fortimanager and cloudfront write the idiom with every detail moved -- the
/// namespace, the number of parts, and whether the target is also one of the
/// sources -- so all of that is read off the script rather than assumed:
///
/// ```painless
/// ctx._temp.date = ctx._temp.date + 'T' + ctx._temp.time + ctx._temp.tz;
/// ctx._tmp.timestamp = ctx._tmp.date + 'T' + ctx._tmp.time;
/// ```
///
/// Distinct from [`ConcatScript`], which accumulates into a LOCAL over a run of
/// guarded `+=` statements and assigns it out once at the end.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConcatAssignment {
    /// Where the joined string lands. Often one of the sources -- fortimanager
    /// rewrites `_temp.date` in place -- but cloudfront writes a sibling.
    target: String,
    /// The right-hand side in order, each term a field read or literal text.
    terms: Vec<ConcatTerm>,
}

/// Read a one-statement `ctx.<target> = <term> + <term> ...` as a
/// [`KnownPattern::ConcatAssignment`].
///
/// The WHOLE script has to be that statement, optionally wrapped in a single
/// `if` whose every test is `!= null` on a field the statement joins -- the
/// guard fortimanager writes inline and cloudfront puts on the processor
/// instead. Anything else in the guard means the script does something this
/// does not read, so it declines rather than running half of it.
fn parse_concat_assignment(script: &str) -> Option<ConcatAssignment> {
    let (guard, statement) = if_wrapped(script.trim())
        .map_or((None, script.trim()), |(guard, body)| (Some(guard), body));

    // ONE statement, and the whole of it -- a second would be a script this
    // reads only part of.
    let statement = statement.trim().strip_suffix(';')?;
    if statement.contains(';') {
        return None;
    }

    let (lhs, rhs) = split_assignment(statement)?;
    let target = ctx_field_path(lhs)?;
    let terms = concat_assignment_terms(rhs)?;

    let mut fields: Vec<String> = terms
        .iter()
        .filter_map(|term| match term {
            ConcatTerm::Field(path) => Some(path.clone()),
            ConcatTerm::Literal(_) => None,
        })
        .collect();
    // A literal separator is what tells a string join from arithmetic. Without
    // one this is `ctx.a = ctx.b + ctx.c`, which `ScalarExpression` owns and
    // reads as a sum.
    if fields.len() < 2 || fields.len() == terms.len() {
        return None;
    }

    if let Some(guard) = guard {
        fields.sort();
        fields.dedup();
        // Exactly the fields the join reads. A guard over a SUBSET would let a
        // null through, and Painless renders one as the four letters -- a value
        // worth declining rather than writing.
        if null_guard_paths(guard)? != fields {
            return None;
        }
    }

    Some(ConcatAssignment { target, terms })
}

/// A script that is nothing but `if (<guard>) { <body> }`, as its two halves.
///
/// `None` where the script is not one guarded block: an `else` arm, or anything
/// at all after the closing brace, is a different pattern.
fn if_wrapped(script: &str) -> Option<(&str, &str)> {
    let after = script.strip_prefix("if")?.trim_start().strip_prefix('(')?;
    let mut depth = 1usize;
    let mut close = None;
    for (at, c) in after.char_indices() {
        if c == '(' {
            depth += 1;
        } else if c == ')' {
            depth -= 1;
            if depth == 0 {
                close = Some(at);
                break;
            }
        }
    }
    let close = close?;
    let body = after[close + 1..].trim_start().strip_prefix('{')?;
    let end = body.rfind('}')?;
    body[end + 1..]
        .trim()
        .is_empty()
        .then(|| (&after[..close], &body[..end]))
}

/// The two halves of a plain `=` assignment.
///
/// `None` where the first `=` belongs to a comparison or a compound operator,
/// which means the statement assigns nothing this can read.
fn split_assignment(statement: &str) -> Option<(&str, &str)> {
    let bytes = statement.as_bytes();
    let at = statement.find('=')?;
    if bytes.get(at + 1) == Some(&b'=')
        || at
            .checked_sub(1)
            .is_some_and(|index| b"=!<>+-*/%&|^".contains(&bytes[index]))
    {
        return None;
    }
    Some((&statement[..at], &statement[at + 1..]))
}

/// The right-hand side as its ordered terms: a quoted literal, or one plain
/// `ctx` field read, joined by `+`.
fn concat_assignment_terms(expression: &str) -> Option<Vec<ConcatTerm>> {
    let mut terms = Vec::new();
    let mut rest = expression.trim();
    loop {
        let consumed = if let Some(quote) = rest.chars().next().filter(|c| *c == '\'' || *c == '"')
        {
            let body = &rest[quote.len_utf8()..];
            let end = body.find(quote)?;
            terms.push(ConcatTerm::Literal(body[..end].to_string()));
            quote.len_utf8() + end + quote.len_utf8()
        } else {
            // Up to the next `+`; a plain field read never holds one.
            let end = rest.find('+').unwrap_or(rest.len());
            terms.push(ConcatTerm::Field(ctx_field_path(&rest[..end])?));
            end
        };
        rest = rest[consumed..].trim_start();
        let Some(next) = rest.strip_prefix('+') else {
            break;
        };
        rest = next.trim_start();
    }
    rest.is_empty().then_some(terms)
}

/// One plain `ctx` field read as its dotted path -- `ctx.a.b`, `ctx?.a?.b` and
/// `ctx['a']['b']` all resolve, and nothing may follow it.
///
/// STRICTER than [`painless_path`], which stops at the first character it does
/// not accept and so reads `ctx.a.toString()` as the path `a.toString`. Here a
/// method call or an operator after the path means the term is not a plain
/// read, and the match declines rather than naming a field no event carries.
///
/// A dotted segment takes no `-`, which [`painless_path`] does accept: Painless
/// reads `ctx.a-ctx.b` as SUBTRACTION, so a hyphen there is an operator and
/// never part of a key. A hyphenated key is spelled `ctx['a-b']` and still
/// resolves through the subscript arm.
fn ctx_field_path(term: &str) -> Option<String> {
    let mut rest = term.trim().strip_prefix("ctx")?;
    let mut path = String::new();
    loop {
        if let Some(after) = rest.strip_prefix('?') {
            rest = after;
        } else if let Some(after) = rest.strip_prefix('.') {
            let end = after
                .find(|c: char| !(c.is_alphanumeric() || matches!(c, '_' | '@')))
                .unwrap_or(after.len());
            if end == 0 {
                return None;
            }
            if !path.is_empty() {
                path.push('.');
            }
            path.push_str(&after[..end]);
            rest = &after[end..];
        } else if let Some(after) = rest.strip_prefix('[') {
            let quote = after.chars().next().filter(|c| *c == '\'' || *c == '"')?;
            let body = &after[quote.len_utf8()..];
            let end = body.find(quote)?;
            if !path.is_empty() {
                path.push('.');
            }
            path.push_str(&body[..end]);
            rest = body[end + quote.len_utf8()..].strip_prefix(']')?;
        } else {
            break;
        }
    }
    (rest.trim().is_empty() && !path.is_empty()).then_some(path)
}

/// The fields a `&&` chain of `<field> != null` tests names, sorted.
///
/// `None` where any test is something else -- the guard then means more than
/// "every part is there", which is all the runner reproduces.
fn null_guard_paths(guard: &str) -> Option<Vec<String>> {
    let mut paths = Vec::new();
    for test in guard.split("&&") {
        let (field, null) = test.split_once("!=")?;
        if null.trim() != "null" {
            return None;
        }
        paths.push(ctx_field_path(field)?);
    }
    paths.sort();
    paths.dedup();
    Some(paths)
}

/// Join the parts, writing nothing unless every field the script names is
/// there.
///
/// Painless renders an absent part as the four letters `null`, and the guard
/// the vendor writes is there to stop exactly that -- so a missing part means
/// write nothing. Every read happens before the write, which is what lets
/// fortimanager join `_temp.date` back into itself.
fn run_concat_assignment(event: &mut Event, script: &ConcatAssignment) -> bool {
    let mut joined = String::new();
    for term in &script.terms {
        match term {
            ConcatTerm::Literal(text) => joined.push_str(text),
            ConcatTerm::Field(path) => match event.get(path) {
                // Borrowed, because every part of a timestamp is a string and
                // rendering one would copy it before appending it.
                Some(Value::String(text)) => joined.push_str(text),
                Some(value) if !value.is_null() => {
                    joined.push_str(&crate::painless_helpers::painless_to_string(value));
                }
                _ => return true,
            },
        }
    }
    let _ = event.set(&script.target, json!(joined));
    true
}

/// Read elb's `tlsv12` split as a [`KnownPattern::TlsVersionSplit`].
///
/// s3access spells the same thing `ctx.<p>.toLowerCase().splitOnToken("v")`,
/// and taking everything before the split read `toLowerCase()` as a segment of
/// the path -- so the field resolved to nothing and eight events lost both
/// `tls.version` and `tls.version_protocol`. The run lowercases the protocol
/// half itself, so dropping the call changes nothing else.
fn parse_tls_version_split(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let at = script.find(".splitOnToken(")?;
    let before = &script[..at];
    let path = clean_path(&before[before.rfind("ctx.")? + 4..]);
    let source = path
        .split('.')
        .take_while(|segment| !segment.contains('('))
        .collect::<Vec<_>>()
        .join(".");
    (!source.is_empty()).then_some(KnownPattern::TlsVersionSplit { source })
}

/// `ctx.<t> = ctx.<s>.decodeBase64();` as a [`KnownPattern::DecodeBase64`].
fn parse_decode_base64(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let at = script.find(".decodeBase64()")?;
    let before = &script[..at];
    let source = clean_path(&before[before.rfind("ctx.")? + 4..]);
    let (lhs, _) = before.split_once('=')?;
    let target = clean_path(lhs.trim().strip_prefix("ctx.")?);
    (!target.is_empty() && !source.is_empty())
        .then_some(KnownPattern::DecodeBase64 { source, target })
}

/// What a drop-empty script's OWN predicate says is droppable.
///
/// The pattern recurs across 245 of the 351 packages with an ingest pipeline, and
/// it is not one script. Most spell the predicate
/// `v == null || v == '' || (v instanceof Map && v.size() == 0) || ...`, but 16
/// packages -- `cisco_asa` among them -- write `removeIf(v -> v == null)` and
/// mean it: an empty string stays. Applying the fullest reading to all of them
/// drops fields Elastic keeps.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools)] // Independent predicate axes read off the script, not a state machine.
pub struct DropPolicy {
    /// `v == null` appears in the predicate. vpcflow's dash-removal drops
    /// ONLY the dash, and reading nulls into it dropped values the script
    /// keeps.
    pub nulls: bool,
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
    /// The script prunes the root's DIRECT values only. tychon walks
    /// `keySet()` and removes at the top level, so descending would take
    /// nested nulls it keeps.
    pub shallow: bool,
}

impl DropPolicy {
    /// A policy that drops nothing.
    ///
    /// The base a generated literal builds on, so adding an axis to this
    /// struct does not invalidate every file the generator has written.
    #[must_use]
    pub fn none() -> Self {
        Self {
            nulls: false,
            empty_strings: false,
            empty_collections: false,
            prune_lists: false,
            sentinels: Vec::new(),
            shallow: false,
        }
    }

    /// The reading of a script's predicate, taken off its own text.
    #[must_use]
    pub fn read(script: &str) -> Self {
        let (is_empty_strings, is_empty_collections) = is_empty_axes(script);
        Self {
            nulls: script.contains("== null"),
            empty_strings: script.contains("== ''")
                || script.contains("== \"\"")
                || is_empty_strings,
            // `.isEmpty()` is the third spelling of the collection test and
            // aws/waf's only one, so every empty list and map it ships
            // survived: four per event. Which axis it means is the guard's
            // to say -- see `is_empty_axes`.
            empty_collections: script.contains(".size() == 0")
                || script.contains(".length == 0")
                || is_empty_collections,
            // A list is pruned by a `removeIf` whose receiver is the list
            // itself. `map.values().removeIf` prunes the MAP, which is
            // cisco_asa's only one; gcp's recursive helper walks the map
            // through an iterator and spells `removeIf` once, for the list.
            prune_lists: script
                .split(".removeIf(")
                .take(script.matches(".removeIf(").count())
                .any(|head| !head.trim_end().ends_with("values()")),
            sentinels: predicate_sentinels(script),
            // The recursive spellings this reads all descend; only the
            // `keySet()` walk sets it, and it does so at its own trigger.
            shallow: false,
        }
    }
}

/// Which drop axes a script's `.isEmpty()` calls report on, as
/// `(empty_strings, empty_collections)`.
///
/// `.isEmpty()` is ONE spelling with TWO intents, and reading it as the
/// collection test everywhere inverted the policy on both axes for the two
/// sources that mean the other one. `mysql_enterprise` and oracle prune with
///
/// ```painless
/// map.values().removeIf(v -> v instanceof String && v.isEmpty() == true);
/// ```
///
/// -- empty STRINGS only, every empty map kept -- while aws/waf writes
/// `((v instanceof List || v instanceof Map) && v.isEmpty())` and sets the
/// string axis separately with `v == ""`.
///
/// The guard SHARING the call's conjunction is what disambiguates it, so the
/// window read is bounded by `||` and by the statement and block punctuation
/// either side: an `instanceof String` in some other branch is not this call's
/// guard. An UNGUARDED `.isEmpty()` keeps the collection reading every other
/// script rests on -- salesforce and `ti_socradar` both `return ((Map) o)
/// .isEmpty()` out of a helper, with the `instanceof` an `if` away.
fn is_empty_axes(script: &str) -> (bool, bool) {
    const CALL: &str = ".isEmpty()";
    let (mut strings, mut collections) = (false, false);
    for (at, _) in script.match_indices(CALL) {
        let term = conjunction_around(script, at);
        let on_string = term.contains("instanceof String");
        let on_collection = term.contains("instanceof List") || term.contains("instanceof Map");
        strings |= on_string;
        collections |= on_collection || !on_string;
    }
    (strings, collections)
}

/// The `&&` conjunction the byte at `at` sits in.
///
/// Bounded by `||` -- the alternatives of a predicate chain are separate
/// terms -- and by `;`, `{` and `}`, so a guard in a neighbouring statement or
/// block cannot be read as this term's.
fn conjunction_around(script: &str, at: usize) -> &str {
    let punctuation = |c: char| matches!(c, ';' | '{' | '}');
    // `;{}` are ASCII, so one past the match is a char boundary.
    let mut start = script[..at].rfind(punctuation).map_or(0, |i| i + 1);
    if let Some(i) = script[start..at].rfind("||") {
        start += i + "||".len();
    }
    let rest = &script[at..];
    let end = rest
        .find(punctuation)
        .unwrap_or(rest.len())
        .min(rest.find("||").unwrap_or(rest.len()));
    &script[start..at + end]
}

/// The non-empty literals a drop predicate compares its value against.
///
/// Two spellings carry the predicate: the first `if (` of the recursive
/// helper, and zscaler's `boolean dropScalar(v) { return v == null || ... }`
/// form, whose chains sit after `return`. Both are read; a literal from an
/// unrelated statement cannot join because only `== '<quoted>'` terms count.
fn predicate_sentinels(script: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut collect = |chain: &str| {
        for term in chain.split("||") {
            let Some((_, rest)) = term.split_once("== ") else {
                continue;
            };
            // Quoted, so the `null` keyword is not read as the string "null"
            // -- and read to the CLOSING quote, so a lambda's trailing `)`
            // stays out of the literal.
            let rest = rest.trim();
            let Some(quote) = rest.chars().next().filter(|c| matches!(c, '\'' | '"')) else {
                continue;
            };
            let inner = &rest[quote.len_utf8()..];
            let Some(end) = inner.find(quote) else {
                continue;
            };
            let literal = &inner[..end];
            if !literal.is_empty() && !found.iter().any(|f: &String| f == literal) {
                found.push(literal.to_string());
            }
        }
    };

    if let Some(predicate) = script
        .split("if (")
        .nth(1)
        .and_then(|s| s.split(')').next())
    {
        collect(predicate);
    }
    for chain in script.split("return ").skip(1) {
        if let Some(chain) = chain.split(';').next() {
            collect(chain);
        }
    }
    // vpcflow's dash removal carries its literal in the lambda itself:
    // `removeIf(v -> v instanceof String && v == "-")`.
    for lambda in script.split("removeIf(").skip(1) {
        if let Some(body) = lambda.split(';').next() {
            collect(body);
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
    let mut shrunk = Vec::new();
    drop_value(
        event.as_value_mut(),
        policy,
        &mut Marks::new(String::new()),
        &mut shrunk,
    );
    for (path, entries) in shrunk {
        event.record_map_capacity(path, entries);
    }
}

/// Prune the subtree at `root`, which is where in the document it sits -- a
/// recorded path has to read the same as the one a later render walks to.
/// A value appended to a list the script first ensures exists.
///
/// The `?:` preamble that builds each level is ceremony [`Event::append`] does
/// for free -- it creates the array and the path it hangs from -- so only the
/// two paths matter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnsureAppend {
    source: String,
    target: String,
}

impl EnsureAppend {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
        }
    }
}

/// `ctx.<target> = ctx.<target> ?: []; ctx.<target>.add(ctx.<source>);`
fn parse_ensure_append(script: &str) -> Option<EnsureAppend> {
    let (head, tail) = script.rsplit_once(" ?: [];")?;
    let target = painless_path(head)?;
    let argument = tail.split_once(".add(")?.1.split(')').next()?;
    // A nested call in the argument is a different pattern, not this one.
    if argument.contains('(') {
        return None;
    }
    let source = painless_path(argument)?;
    (!source.is_empty() && !target.is_empty()).then(|| EnsureAppend::new(source, target))
}

/// Append the source onto the target list, creating it where it is absent.
pub fn ensure_append(event: &mut Event, pattern: &EnsureAppend) -> bool {
    if let Some(value) = event.get(&pattern.source).cloned() {
        let _ = event.append(&pattern.target, value);
    }
    true
}

/// A composite key built by joining whichever of several fields are present.
///
/// gdacs identifies an event by up to three ids, and the join is what makes
/// them one `event.id`. The ABSENT ones contribute nothing rather than an
/// empty segment, which is why the guards are per field and not one guard
/// around the lot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinPresentFields {
    sources: Vec<String>,
    separator: String,
    target: String,
}

impl JoinPresentFields {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        sources: Vec<String>,
        separator: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            sources,
            separator: separator.into(),
            target: target.into(),
        }
    }
}

/// `if (ctx.a != null) { p.add(ctx.a.toString()); } ... ctx.t = String.join("-", p);`
///
/// The accumulator's NAME is not the trigger -- gdacs calls it `parts` and it
/// is the only source in the corpus that does, so keying on the spelling would
/// claim exactly one script and miss the next vendor to write `ids` or `bits`.
/// The join names the local, and the local's `.add(` calls name the fields.
fn parse_join_present_fields(script: &str) -> Option<JoinPresentFields> {
    let (head, join) = script.rsplit_once("String.join(")?;
    // Only the assignment itself, not everything the script did before it.
    let assignment = head.rsplit(['{', '}', ';', '\n']).next()?;
    let target = painless_path(assignment.trim_end().strip_suffix('=')?)?;

    let (separator, rest) = join.strip_prefix('"')?.split_once('"')?;
    let local = rest
        .trim_start_matches([',', ' '])
        .split(')')
        .next()?
        .trim();
    if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    // Every value the local collects, in the order the script adds them --
    // order is the key's meaning, so a set would be wrong here.
    let needle = format!("{local}.add(");
    let mut sources = Vec::new();
    for chunk in script.split(&needle).skip(1) {
        // To the end of the STATEMENT, not the first `)`: `.toString()`
        // carries a pair of its own and splitting on `)` lands inside it.
        let argument = chunk.split_once(");").map_or(chunk, |(head, _)| head);
        let path = argument.strip_suffix(".toString()").unwrap_or(argument);
        // Anything but a plain field read -- a nested call, a literal, an
        // expression -- is a different pattern, and this one declines rather
        // than guess at it.
        if path.contains('(') {
            return None;
        }
        sources.push(painless_path(path)?);
    }

    // One field joined to nothing is a copy, and a copy has its own pattern.
    (sources.len() > 1 && !target.is_empty())
        .then(|| JoinPresentFields::new(sources, separator, target))
}

/// Join the present sources in order; write nothing when none of them are.
pub fn join_present_fields(event: &mut Event, pattern: &JoinPresentFields) -> bool {
    let mut parts: Vec<String> = Vec::with_capacity(pattern.sources.len());
    for source in &pattern.sources {
        if let Some(value) = event.get_as_string(source) {
            parts.push(value);
        }
    }
    if !parts.is_empty() {
        let _ = event.set(&pattern.target, json!(parts.join(&pattern.separator)));
    }
    true
}

/// Keys carrying a suffix rewritten without it, unwrapping a lone value.
///
/// gitlab's structured log names every measurement `<thing>.values` and holds
/// a LIST under it, even when there is one reading. The script renames the key
/// without the suffix and takes the single value out of its list, keeping the
/// list only where there is genuinely more than one.
///
/// Leaving it unbound put every measurement one level too deep -- 688 missing
/// fields matched by 688 extra ones on its application stream, which is the
/// signature the debt classifier calls a raw shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnwrapSuffixedKeys {
    container: String,
    /// The suffix a key must end with, without its separator.
    suffix: String,
    /// How many characters the script takes off, separator included.
    trim: usize,
}

impl UnwrapSuffixedKeys {
    /// The generated call site that rebuilds this pattern.
    #[cfg(feature = "codegen")]
    pub(crate) fn direct_call(&self) -> String {
        format!(
            "unwrap_suffixed_keys(event, &UnwrapSuffixedKeys::new({}.into(), {}.into(), {}));",
            rust_str(&self.container),
            rust_str(&self.suffix),
            self.trim,
        )
    }

    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(container: String, suffix: String, trim: usize) -> Self {
        Self {
            container,
            suffix,
            trim,
        }
    }
}

/// `for (f in ctx.a.keySet()) { if (f.endsWith('values')) { ... f.length() - 7 ... } }`
fn parse_unwrap_suffixed_keys(script: &str) -> Option<UnwrapSuffixedKeys> {
    let (head, _) = script.split_once(".keySet()")?;
    let container = clean_path(head.rsplit("ctx.").next()?.trim());
    if container.is_empty() || container.contains(char::is_whitespace) {
        return None;
    }

    let suffix = script
        .split_once(".endsWith(")?
        .1
        .trim_start()
        .strip_prefix(['\'', '"'])?
        .split(['\'', '"'])
        .next()?
        .to_owned();
    if suffix.is_empty() {
        return None;
    }

    // The script's own arithmetic, not a guess from the suffix: the separator
    // it strips with the name is part of the count.
    let trim: usize = script
        .split_once(".length() - ")?
        .1
        .split([')', ' ', ';'])
        .next()?
        .parse()
        .ok()?;
    (trim >= suffix.len()).then(|| UnwrapSuffixedKeys::new(container, suffix, trim))
}

/// Rewrite each suffixed key, taking a lone value out of its list.
pub fn unwrap_suffixed_keys(event: &mut Event, pattern: &UnwrapSuffixedKeys) -> bool {
    let Some(container) = event
        .get(&pattern.container)
        .and_then(Value::as_object)
        .cloned()
    else {
        return false;
    };

    let mut rebuilt = serde_json::Map::with_capacity(container.len());
    let mut rewrote = false;
    for (key, value) in container {
        let renamed = key
            .ends_with(&pattern.suffix)
            .then(|| key.len().checked_sub(pattern.trim))
            .flatten()
            .map(|end| key[..end].to_owned())
            .filter(|renamed| !renamed.is_empty());

        let Some(renamed) = renamed else {
            rebuilt.insert(key, value);
            continue;
        };
        rewrote = true;
        // A single reading comes out of its list; several stay a list.
        let unwrapped = match value {
            Value::Array(mut values) if values.len() == 1 => values.remove(0),
            other => other,
        };
        rebuilt.insert(renamed, unwrapped);
    }

    if !rewrote {
        return false;
    }
    let _ = event.set(&pattern.container, Value::Object(rebuilt));
    true
}

/// A list of `{name, value}` parameters fanned out into a map.
///
/// Google Workspace ships every per-application detail as one list of objects,
/// each naming itself and carrying its value under whichever of six keys suits
/// its type. Seven of its streams write the same loop -- calendar, chat, meet,
/// `data_studio`, vault, chrome, keep -- and the per-application block is
/// absent without it.
///
/// The value keys are tried in the ORDER the script writes them, because that
/// is what its `else if` ladder does: the first one present wins.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParametersIntoMap {
    /// The list of parameter objects.
    source: String,
    /// The map each one is written into.
    target: String,
    /// The key naming the parameter, and whether its value is case-folded.
    name_key: String,
    lowercase: bool,
    /// The value keys, in the script's own order.
    value_keys: Vec<String>,
}

impl ParametersIntoMap {
    /// The generated call site that rebuilds this pattern.
    #[cfg(feature = "codegen")]
    pub(crate) fn direct_call(&self) -> String {
        let keys: Vec<String> = self
            .value_keys
            .iter()
            .map(|key| format!("{}.into()", rust_str(key)))
            .collect();
        format!(
            "parameters_into_map(event, &ParametersIntoMap::new({}.into(), {}.into(), {}.into(), {}, vec![{}]));",
            rust_str(&self.source),
            rust_str(&self.target),
            rust_str(&self.name_key),
            self.lowercase,
            keys.join(", "),
        )
    }

    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        source: String,
        target: String,
        name_key: String,
        lowercase: bool,
        value_keys: Vec<String>,
    ) -> Self {
        Self {
            source,
            target,
            name_key,
            lowercase,
            value_keys,
        }
    }
}

/// `for (def p : ctx.a.list) { ... ctx.b[name] = p.value; else if p.intValue ... }`
fn parse_parameters_into_map(script: &str) -> Option<ParametersIntoMap> {
    // The loop names the element variable and the list it walks.
    let (head, rest) = script.split_once(" : ctx.")?;
    let var = head.rsplit("def ").next()?.trim().to_owned();
    if var.is_empty() || var.contains(char::is_whitespace) {
        return None;
    }
    let source = clean_path(rest.split(')').next()?.trim());

    // The guard names the key that identifies each parameter.
    let name_key = script
        .split_once(&format!("{var}."))?
        .1
        .split([' ', '.', ')', ';'])
        .next()?
        .to_owned();
    if name_key.is_empty() {
        return None;
    }
    let lowercase = script.contains(&format!("{var}.{name_key}.toLowerCase()"));

    // Every write, in order: the target map and the value key it reads.
    let re = crate::cached_regex!(
        r"ctx\.([A-Za-z0-9_.]+)\[[A-Za-z0-9_]+\]\s*=\s*[A-Za-z0-9_]+\.([A-Za-z0-9_]+)"
    )
    .fast()?;
    let mut target = None;
    let mut value_keys = Vec::new();
    for caps in re.captures_iter(script) {
        let path = clean_path(caps.get(1)?.as_str());
        let key = caps.get(2)?.as_str().to_owned();
        if key == name_key {
            continue;
        }
        match &target {
            // Two different maps in one loop is a different pattern.
            Some(seen) if *seen != path => return None,
            _ => target = Some(path),
        }
        if !value_keys.contains(&key) {
            value_keys.push(key);
        }
    }
    let target = target?;
    (!value_keys.is_empty() && !source.is_empty())
        .then(|| ParametersIntoMap::new(source, target, name_key, lowercase, value_keys))
}

/// Write each parameter under its own name, merging into what is there.
pub fn parameters_into_map(event: &mut Event, pattern: &ParametersIntoMap) -> bool {
    let Some(parameters) = event
        .get(&pattern.source)
        .and_then(Value::as_array)
        .cloned()
    else {
        return false;
    };

    // MERGED, not replaced: the script's `?: [:]` keeps whatever is there.
    let mut written = event
        .get(&pattern.target)
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();

    let mut wrote = false;
    for parameter in &parameters {
        let Some(name) = parameter.get(&pattern.name_key).and_then(Value::as_str) else {
            continue;
        };
        let name = if pattern.lowercase {
            name.to_lowercase()
        } else {
            name.to_owned()
        };
        // The first key the parameter HOLDS, the way the `else if` ladder runs.
        let Some(value) = pattern
            .value_keys
            .iter()
            .filter_map(|key| parameter.get(key))
            .find(|value| !value.is_null())
        else {
            continue;
        };
        written.insert(name, value.clone());
        wrote = true;
    }

    if !wrote {
        return false;
    }
    let _ = event.set(&pattern.target, Value::Object(written));
    true
}

/// Every key of a map rewritten by a single character replacement.
///
/// `juniper_srx`'s keys arrive kebab-cased -- `source-address` -- and a script
/// swaps the hyphen for an underscore across the whole map. This is NOT the
/// camel-case converter: [`snake_case_apply`] declines a helper with no
/// `Character.isUpperCase` in it precisely so a bare `replace` does not claim
/// a runner that cannot apply it, which left this one bound to nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenameMapKeys {
    container: String,
    from: char,
    to: char,
}

impl RenameMapKeys {
    /// The generated call site that rebuilds this pattern.
    #[cfg(feature = "codegen")]
    pub(crate) fn direct_call(&self) -> String {
        format!(
            "rename_map_keys(event, &RenameMapKeys::new({}.into(), {:?}, {:?}));",
            rust_str(&self.container),
            self.from,
            self.to,
        )
    }

    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(container: String, from: char, to: char) -> Self {
        Self {
            container,
            from,
            to,
        }
    }
}

/// `ctx.a = ctx.a.entrySet().stream().collect(toMap(e -> e.getKey().replace('-', '_'), ..))`
fn parse_rename_map_keys(script: &str) -> Option<RenameMapKeys> {
    let (head, rest) = script.split_once(".getKey().replace(")?;
    // Both characters of the swap, in the order the script writes them.
    let mut literals = crate::painless_common::quoted_members(rest.split(')').next()?);
    let to = literals.pop()?.chars().next()?;
    let from = literals.pop()?.chars().next()?;

    // The container is the assignment's target, which the script also reads.
    let (target, _) = head.split_once(" = ")?;
    let container = clean_path(target.trim().strip_prefix("ctx.")?);
    if container.is_empty() {
        return None;
    }
    Some(RenameMapKeys::new(container, from, to))
}

/// The same rebuild written as a stream one-liner, with a CHAIN of key
/// transforms rather than the single replacement [`parse_rename_map_keys`]
/// reads.
///
/// ```painless
/// ctx.a.b = ctx.a.b.entrySet().stream()
///     .collect(Collectors.toMap(entry -> entry.getKey().toLowerCase(), Map.Entry::getValue));
/// ```
///
/// oracle's kv processor writes the vendor's own uppercase headings -- `DBID`,
/// `SESSIONID`, `USERHOST` -- and every `rename` after this script names the
/// lowercase form, so an unmatched fold misses all six at once. `userhost` is
/// the one that feeds `server.address`, which feeds `server.domain` and
/// `related.hosts`.
///
/// Two forms are declined rather than half-applied. A single `.replace(a, b)`
/// belongs to [`parse_rename_map_keys`], whose call sites already exist. A
/// value lambda that is not the identity -- `juniper_srx`'s
/// `e -> e.getValue().trim()` -- is a different script, and claiming it would
/// write the keys right and the values wrong.
fn parse_stream_rewrite_keys(script: &str) -> Option<RewriteKeys> {
    use crate::painless_params::{balanced, clean_path};

    let at = script.find("Collectors.toMap")?;
    let (arguments, _) = balanced(&script[at + "Collectors.toMap".len()..], '(', ')')?;
    let comma = top_level_comma(arguments)?;
    let (key_lambda, value_lambda) = arguments.split_at(comma);
    if !hands_the_value_through(value_lambda[1..].trim()) {
        return None;
    }

    // Everything the key lambda does AFTER reading the key, in written order.
    let steps = key_rewrite_steps(key_lambda.split_once(".getKey()")?.1)?;
    if steps.is_empty() || matches!(steps.as_slice(), [KeyRewriteStep::ReplaceChars(..)]) {
        return None;
    }

    // The container is the assignment's target, which the script also reads.
    let (target, _) = script[..at].split_once(" = ")?;
    let path = clean_path(target.trim().strip_prefix("ctx.")?);
    if path.is_empty() {
        return None;
    }
    Some(RewriteKeys::new(path.clone(), path, steps))
}

/// `event.duration`, `event.start` and `event.end` from ONE duration field.
///
/// ```painless
/// ctx.event.duration = Integer.parseInt(ctx.sophos.xg.duration) * 1000000000L;
/// ctx.event.start = ctx['@timestamp'];
/// ZonedDateTime start = ZonedDateTime.parse(ctx.event.start);
/// ctx.event.end = start.plus(ctx.event.duration, ChronoUnit.NANOS);
/// ```
///
/// `sophos` and `juniper_srx` write it identically, differing only in the field
/// the duration comes from. Unmatched it costs sophos all three fields on 63 of
/// its events, and `event.start` alone unlocks 56 of them.
///
/// The three writes are one pattern rather than three, because the second and
/// third are derived from the first: reading the arithmetic and leaving the
/// dates would write a duration with no window around it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DurationWindow {
    /// The field holding the duration, in whole units.
    source: String,
    /// Nanoseconds per unit -- `1_000_000_000` where the vendor counts seconds.
    scale: i64,
    /// The timestamp the window starts at, which `event.start` is copied from.
    base: String,
}

impl DurationWindow {
    /// The generated call site that rebuilds this pattern.
    #[cfg(feature = "codegen")]
    pub(crate) fn direct_call(&self) -> String {
        format!(
            "duration_window(event, &DurationWindow::new({}.into(), {}, {}.into()));",
            rust_str(&self.source),
            self.scale,
            rust_str(&self.base),
        )
    }

    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(source: String, scale: i64, base: String) -> Self {
        Self {
            source,
            scale,
            base,
        }
    }
}

/// Write the duration and the window around it.
///
/// A duration that will not parse leaves all three alone: Painless would have
/// thrown on `Integer.parseInt`, and the vendor's processor carries no
/// `on_failure`, so the document keeps what it had.
pub fn duration_window(event: &mut Event, pattern: &DurationWindow) -> bool {
    let Some(units) = event.get(&pattern.source).and_then(|value| match value {
        Value::String(text) => text.trim().parse::<i64>().ok(),
        other => other.as_i64(),
    }) else {
        return true;
    };
    let (Some(nanos), Some(start)) = (
        units.checked_mul(pattern.scale),
        event.get_string(&pattern.base),
    ) else {
        return true;
    };
    let Some(end) = crate::date_formats::iso8601_plus_nanos(&start, nanos) else {
        return true;
    };
    let _ = event.set("event.duration", json!(nanos));
    let _ = event.set("event.start", json!(start));
    let _ = event.set("event.end", json!(end));
    true
}

/// Read the four statements, or decline.
fn parse_duration_window(script: &str) -> Option<DurationWindow> {
    use crate::painless_params::{clean_path, subject_path};

    // Every write must be present, or this is a different script that happens
    // to compute a duration.
    if !script.contains("ctx.event.duration, ChronoUnit.NANOS")
        || !script.contains("ZonedDateTime.parse(")
    {
        return None;
    }

    // `ctx.event.duration = Integer.parseInt(ctx.<source>) * <scale>;`
    let (_, tail) = script.split_once("ctx.event.duration = ")?;
    let (parsed, rest) = tail.split_once(')')?;
    let parsed = parsed.trim();
    let source = clean_path(
        parsed
            .strip_prefix("Integer.parseInt(ctx.")
            .or_else(|| parsed.strip_prefix("Long.parseLong(ctx."))?,
    );
    let scale: i64 = rest
        .split(';')
        .next()?
        .trim()
        .strip_prefix('*')?
        .trim()
        .trim_end_matches(['L', 'l'])
        .parse()
        .ok()?;
    if source.is_empty() || scale <= 0 {
        return None;
    }

    // `ctx.event.start = ctx['@timestamp'];`
    let (_, base) = script.split_once("ctx.event.start = ")?;
    let base = subject_path(base.split(';').next()?.trim());
    let base = clean_path(base.strip_prefix("ctx.")?);
    if base.is_empty() {
        return None;
    }
    Some(DurationWindow::new(source, scale, base))
}

/// A named member copied out of whichever of several sibling keys is present.
///
/// A payload that carries exactly one of a family of keys, and a pipeline that
/// wants the same member out of whichever one arrived. tetragon's is the case:
/// its event is `process_exec` or `process_exit` or one of five more, each
/// holding a `process` and most holding a `parent`, and the pipeline lifts both
/// to a temporary so the twenty renames after it can name ONE path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberFromVariantKey {
    /// The map whose keys are the variants.
    root: String,
    /// Where the lifted members are collected.
    target: String,
    /// One per member the script lifts, with the keys it accepts for it.
    /// tetragon's two differ: `process_loader` carries a process and no parent.
    lifts: Vec<(String, Vec<String>)>,
}

impl MemberFromVariantKey {
    /// The generated call site that rebuilds this pattern.
    #[cfg(feature = "codegen")]
    pub(crate) fn direct_call(&self) -> String {
        let lifts: Vec<String> = self
            .lifts
            .iter()
            .map(|(member, keys)| {
                let keys: Vec<String> = keys.iter().map(|key| rust_str(key)).collect();
                format!("({}.into(), vec![{}])", rust_str(member), {
                    let owned: Vec<String> =
                        keys.iter().map(|key| format!("{key}.into()")).collect();
                    owned.join(", ")
                })
            })
            .collect();
        format!(
            "member_from_variant_key(event, &MemberFromVariantKey::new({}.into(), {}.into(), vec![{}]));",
            rust_str(&self.root),
            rust_str(&self.target),
            lifts.join(", "),
        )
    }

    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(root: String, target: String, lifts: Vec<(String, Vec<String>)>) -> Self {
        Self {
            root,
            target,
            lifts,
        }
    }
}

/// Copy each member out of whichever variant key the document carries.
///
/// The root's keys are walked in the order they arrive and the LAST match wins,
/// which is what Painless's loop does. In practice a payload carries one.
pub fn member_from_variant_key(event: &mut Event, pattern: &MemberFromVariantKey) -> bool {
    let Some(root) = event.get(&pattern.root).and_then(Value::as_object).cloned() else {
        return true;
    };
    for (member, keys) in &pattern.lifts {
        for (key, value) in &root {
            if !keys.iter().any(|allowed| allowed == key) {
                continue;
            }
            if let Some(found) = value.get(member) {
                let _ = event.set(&format!("{}.{member}", pattern.target), found.clone());
            }
        }
    }
    true
}

/// ```painless
/// void run(Map map) {
///   for (def k : map?.a?.b?.keySet()) {
///     if (k == "one" || k == "two") {
///       if (map?._tmp_ == null) { map["_tmp_"] = new HashMap(); }
///       map["_tmp_"]["process"] = map.a.b[k].process;
///     }
///   }
/// }
/// run(ctx);
/// ```
///
/// Every write is read, and a write this cannot read declines the whole script:
/// lifting one member and not another leaves the renames after it half fed,
/// which reads as a source needing polish rather than one needing a matcher.
fn parse_member_from_variant_key(script: &str) -> Option<MemberFromVariantKey> {
    use crate::painless_params::clean_path;

    let (head, body) = script.split_once(".keySet()")?;
    let root = clean_path(head.rsplit_once("map?")?.1.trim_start_matches('.'));
    if root.is_empty() {
        return None;
    }

    // The right-hand side every lift shares, which also pins the loop variable.
    let reads = format!("= map.{root}[k].");
    let mut target: Option<String> = None;
    let mut lifts: Vec<(String, Vec<String>)> = Vec::new();
    let mut from = 0usize;
    while let Some(offset) = body[from..].find(&reads) {
        let at = from + offset;
        let member = body[at + reads.len()..]
            .split_once(';')?
            .0
            .trim()
            .to_owned();
        if member.is_empty() || !member.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return None;
        }

        // The left-hand side, `map["<target>"]["<member>"]`, names where it
        // lands. A different member either side is a script this misreads.
        let assigned = body[..at].trim_end();
        let (holder, written) = assigned.rsplit_once("][")?;
        if written.trim_end_matches(']').trim_matches(['"', '\'']) != member {
            return None;
        }
        let holder = holder.rsplit_once("map[")?.1.trim_matches(['"', '\'']);
        if target.get_or_insert_with(|| holder.to_owned()) != holder {
            return None;
        }

        // The guard above it, and the keys it names.
        let guard = assigned.rfind("if (k ==")?;
        let condition = &assigned[guard..assigned[guard..].find(')')? + guard];
        let keys: Vec<String> = quoted_members(condition);
        if keys.is_empty() {
            return None;
        }
        lifts.push((member, keys));
        from = at + reads.len();
    }

    // A `for` body doing anything else is not this pattern. The allocation
    // guard and the loop's own header are all that may sit beside the lifts.
    let leftover = body.replace('\n', " ");
    if leftover.contains(".put(") || leftover.contains("remove(") {
        return None;
    }
    if lifts.is_empty() {
        return None;
    }
    Some(MemberFromVariantKey::new(root, target?, lifts))
}

/// A whole subtree's keys rewritten at every depth, then MOVED to a new path.
///
/// ```painless
/// String normalize(String str) { return str.replace('-', '_'); }
/// def normalizeFields(def obj) {
///   if (obj instanceof Map) { ... newObj.put(normalize(entry.getKey()),
///                                            normalizeFields(entry.getValue())); ... }
///   else if (obj instanceof List) { ... newList.add(normalizeFields(item)); ... }
///   return obj;
/// }
/// if (ctx.json != null) {
///   ctx.ti_flashpoint = ctx.ti_flashpoint ?: [:];
///   ctx.ti_flashpoint.alert = normalizeFields(ctx.json);
///   ctx.remove('json');
/// }
/// ```
///
/// `ti_flashpoint` ships it on all three streams and scores 0 of 9 events
/// at 8.9% of its fields without it: the payload stays under `json.*`, which is
/// the whole of its 215 extra, and every `date`, `rename` and `convert` after it
/// names `ti_flashpoint.<stream>.*` and finds nothing.
///
/// The recursion and the removal are both read off the script rather than
/// assumed. A one-level rewrite that leaves the source where it is
/// ([`parse_rewrite_keys`]) is a different script with a different result.
fn parse_recursive_rewrite_keys(script: &str) -> Option<RewriteKeys> {
    use crate::painless_params::clean_path;

    // The helper must recurse through BOTH containers, or a nested map keeps
    // the vendor's spelling and half the payload lands unrenamed.
    let (helper, _) = script.split_once("(def ")?;
    let helper = helper.rsplit_once(' ')?.1;
    if helper.is_empty() || !script.contains("instanceof List") {
        return None;
    }
    let recurses = format!("{helper}(entry.getValue())");
    if !script.contains(&recurses) || !script.contains(&format!("{helper}(item)")) {
        return None;
    }

    // The call that stores the result names both ends of the move.
    let call = format!(" = {helper}(ctx.");
    let at = script.find(&call)?;
    let source = clean_path(script[at + call.len()..].split_once(')')?.0.trim());
    let target = clean_path(script[..at].trim_end().rsplit_once("ctx.")?.1);
    if source.is_empty() || target.is_empty() || source == target {
        return None;
    }

    // Everything the key helper does, read from its own body.
    let define = script.find(&format!("String {}(", key_helper(script)?))?;
    let steps = key_rewrite_steps(&script[define..at])?;
    if steps.is_empty() {
        return None;
    }

    let removes = script.contains(&format!("ctx.remove('{source}')"))
        || script.contains(&format!("ctx.remove(\"{source}\")"));
    let pattern = RewriteKeys::new(source, target, steps).recursive();
    Some(if removes {
        pattern.removing_source()
    } else {
        pattern
    })
}

/// The name of the `String`-returning helper the key is rebuilt through.
fn key_helper(script: &str) -> Option<&str> {
    let (head, _) = script.split_once("(String ")?;
    let name = head.rsplit_once(' ')?.1;
    (!name.is_empty()).then_some(name)
}

/// The same fold written as an explicit loop into a local map.
///
/// ```painless
/// def lowercaseMap = [:];
/// for(def entry : ctx.sophos.xg.entrySet()){
///   lowercaseMap.put(entry.getKey().toLowerCase(), entry.getValue());
/// }
/// ctx.sophos.xg = lowercaseMap;
/// ```
///
/// A third spelling of what [`parse_rewrite_keys`] and
/// [`parse_stream_rewrite_keys`] already read, and the one `sophos` writes. It
/// is load-bearing there rather than cosmetic: `event.duration` comes from
/// `ctx.sophos.xg.responsetime`, and `event.start` and `event.end` from two
/// more lowercase keys, so with the fold unmatched all three are read off names
/// the document does not carry. 63 events apiece, and 56 unlocked behind
/// `event.start` alone.
fn parse_loop_rewrite_keys(script: &str) -> Option<RewriteKeys> {
    use crate::painless_params::clean_path;

    // The loop, its variable, and the map it walks.
    let (head, body) = script.split_once(".entrySet()")?;
    let (before, subject) = head.rsplit_once(" : ")?;
    let source = clean_path(subject.trim().strip_prefix("ctx.")?);
    let item = before.split_whitespace().next_back()?;
    if source.is_empty() || item.is_empty() {
        return None;
    }

    // `<local>.put(<key expression>, <item>.getValue())`
    let put = body.find(".put(")?;
    let local = body[..put].trim_end().rsplit(['{', '\n', ' ']).next()?;
    let (arguments, _) = crate::painless_params::balanced(&body[put + ".put".len()..], '(', ')')?;
    let comma = top_level_comma(arguments)?;
    let (key, value) = arguments.split_at(comma);
    if value[1..].trim() != format!("{item}.getValue()") {
        return None;
    }

    let steps = key_rewrite_steps(key.split_once(&format!("{item}.getKey()"))?.1)?;
    if steps.is_empty() {
        return None;
    }

    // The store, which is where the rebuilt map lands.
    let store = body.rfind(&format!("= {local};"))?;
    let target = clean_path(body[..store].trim_end().rsplit_once("ctx.")?.1);
    if target.is_empty() {
        return None;
    }
    Some(RewriteKeys::new(source, target, steps))
}

/// The offset of the comma separating `toMap`'s two lambdas, which is the only
/// one not inside a call of its own -- `.replace(' ', '_')` carries one too.
fn top_level_comma(arguments: &str) -> Option<usize> {
    let mut depth = 0usize;
    let mut quoted = false;
    for (index, c) in arguments.char_indices() {
        match c {
            '\'' | '"' => quoted = !quoted,
            '(' if !quoted => depth += 1,
            ')' if !quoted => depth = depth.checked_sub(1)?,
            ',' if !quoted && depth == 0 => return Some(index),
            _ => {}
        }
    }
    None
}

/// Whether the value half of a `toMap` hands the value through untouched.
fn hands_the_value_through(lambda: &str) -> bool {
    if lambda == "Map.Entry::getValue" {
        return true;
    }
    lambda
        .split_once(" -> ")
        .is_some_and(|(bound, body)| body.trim() == format!("{}.getValue()", bound.trim()))
}

/// Rebuild the map with each key rewritten, in the order it already had.
pub fn rename_map_keys(event: &mut Event, pattern: &RenameMapKeys) -> bool {
    let Some(container) = event
        .get(&pattern.container)
        .and_then(Value::as_object)
        .cloned()
    else {
        return false;
    };
    let mut rebuilt = serde_json::Map::with_capacity(container.len());
    for (key, value) in container {
        rebuilt.insert(key.replace(pattern.from, &pattern.to.to_string()), value);
    }
    let _ = event.set(&pattern.container, Value::Object(rebuilt));
    true
}

/// One replacement a key-normalising helper spells out, in its own order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyRewriteStep {
    /// `_?([a-z])([A-Z]+)` replaced by `$1_$2`: a word break wherever a
    /// lowercase run meets an uppercase one. The optional leading underscore is
    /// CONSUMED by the match and not written back, which is why this is not
    /// [`crate::painless_helpers::to_snake_case`].
    CamelBreak,
    /// `.toLowerCase()`.
    Lowercase,
    /// `\p{C}` dropped -- the control characters a CSV heading carries.
    DropControl,
    /// Every character of the set replaced by one character, or dropped where
    /// the script gives no replacement.
    ReplaceChars(String, Option<char>),
}

/// Every key of a map rewritten by the replacements the script spells out.
///
/// Microsoft's usage reports arrive as their own CSV headings turned into JSON
/// keys -- `Deleted Item Quota (Byte)`, `Prohibit Send/Receive Quota (Byte)` --
/// so `o365_metrics` ships a helper per data stream that lowercases them, swaps
/// spaces, hyphens and slashes for underscores, and drops the parentheses and
/// the byte-order mark. `qualys_vmdr` and `aws.lambda_logs` ship the same thing.
///
/// Every `convert`, `rename`, `date` and `fingerprint` after it names the
/// RESULT, so leaving the keys as the report wrote them misses the whole tail of
/// the pipeline -- and the `fingerprint` carries no `ignore_missing`, so it
/// fails the document and marks it `pipeline_error`.
///
/// The steps are read OFF the script rather than pinned to one package: the
/// fourteen o365 streams spell four different helpers between them, and a
/// fifth would otherwise need its own arm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewriteKeys {
    /// The map whose keys are rewritten.
    source: String,
    /// Where the rebuilt map is stored. The same path, except where the script
    /// moves it -- `aws.lambda_logs` reads `parsed.record.metrics` and writes
    /// `aws.lambda.metrics`, leaving the original where it was.
    target: String,
    /// Applied left to right, the order the script applies them.
    steps: Vec<KeyRewriteStep>,
    /// Whether the script walks nested maps and lists rather than one level.
    recursive: bool,
    /// Whether the source is taken away once the rebuilt map is stored, which
    /// makes this a MOVE. Leaving it behind emits the whole payload twice.
    remove_source: bool,
}

impl RewriteKeys {
    /// The generated call site that rebuilds this pattern.
    #[cfg(feature = "codegen")]
    pub(crate) fn direct_call(&self) -> String {
        let steps: Vec<String> = self
            .steps
            .iter()
            .map(|step| match step {
                KeyRewriteStep::CamelBreak => "KeyRewriteStep::CamelBreak".to_owned(),
                KeyRewriteStep::Lowercase => "KeyRewriteStep::Lowercase".to_owned(),
                KeyRewriteStep::DropControl => "KeyRewriteStep::DropControl".to_owned(),
                KeyRewriteStep::ReplaceChars(chars, with) => format!(
                    "KeyRewriteStep::ReplaceChars({}.into(), {})",
                    rust_str(chars),
                    with.map_or_else(|| "None".to_owned(), |c| format!("Some({c:?})")),
                ),
            })
            .collect();
        // The two extras are BUILDERS rather than arguments, so the 21 call
        // sites that want neither stay exactly as they were written.
        let mut call = format!(
            "RewriteKeys::new({}.into(), {}.into(), vec![{}])",
            rust_str(&self.source),
            rust_str(&self.target),
            steps.join(", "),
        );
        if self.recursive {
            call.push_str(".recursive()");
        }
        if self.remove_source {
            call.push_str(".removing_source()");
        }
        format!("rewrite_keys(event, &{call});")
    }

    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(source: String, target: String, steps: Vec<KeyRewriteStep>) -> Self {
        Self {
            source,
            target,
            steps,
            recursive: false,
            remove_source: false,
        }
    }

    /// The script walks nested maps and lists, not just the top level.
    #[must_use]
    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }

    /// The script takes the source away once it has stored the rebuilt map.
    #[must_use]
    pub fn removing_source(mut self) -> Self {
        self.remove_source = true;
        self
    }
}

/// The character class of a Java regex, as the literal characters it holds.
///
/// A RANGE is declined rather than guessed at, so `[a-z]` never reaches the
/// runner as three characters. A hyphen first or last in the class is a
/// literal, which is exactly how these helpers spell `[ -]`.
fn decode_char_class(class: &str) -> Option<String> {
    let mut out = String::with_capacity(class.len());
    let mut chars = class.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                let escaped = chars.next()?;
                if escaped == 'u' {
                    let hex: String = chars.by_ref().take(4).collect();
                    if hex.len() != 4 {
                        return None;
                    }
                    out.push(char::from_u32(u32::from_str_radix(&hex, 16).ok()?)?);
                } else if "\\()[]{}.-+*?^$|/".contains(escaped) {
                    out.push(escaped);
                } else {
                    return None;
                }
            }
            '-' if !out.is_empty() && chars.peek().is_some() => return None,
            _ => out.push(c),
        }
    }
    (!out.is_empty()).then_some(out)
}

/// The regex a `.matcher(` at `at` is called on, written inline or bound to a
/// local by `def <name> = /.../;` first.
fn regex_before_matcher(text: &str, at: usize) -> Option<String> {
    let head = &text[..at];
    let read_to_slash = |body: &str| -> Option<String> {
        let mut escaped = false;
        for (i, c) in body.char_indices() {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '/' {
                return Some(body[..i].to_owned());
            }
        }
        None
    };

    if let Some(body) = head.strip_suffix('/') {
        // The literal's OPENING slash: the last unescaped one before the close.
        let mut open = None;
        for (i, c) in body.char_indices() {
            if c == '/' && !body[..i].ends_with('\\') {
                open = Some(i);
            }
        }
        return Some(body[open? + 1..].to_owned());
    }

    let name: String = head
        .chars()
        .rev()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect::<Vec<char>>()
        .into_iter()
        .rev()
        .collect();
    if name.is_empty() {
        return None;
    }
    read_to_slash(text.split_once(&format!("{name} = /"))?.1)
}

/// One `<regex> -> <replacement>` pair as the step it performs, or nothing when
/// the pair is not one this runner can carry out.
fn key_rewrite_step(pattern: &str, replacement: &str) -> Option<KeyRewriteStep> {
    // The exact spelling both packages use. A camel-break without the `_?` is
    // a DIFFERENT rewrite -- it keeps the underscore the match would eat -- so
    // it is declined rather than folded in here.
    if pattern == "_?([a-z])([A-Z]+)" && replacement == "$1_$2" {
        return Some(KeyRewriteStep::CamelBreak);
    }
    if pattern == "\\p{C}" && replacement.is_empty() {
        return Some(KeyRewriteStep::DropControl);
    }
    let chars = decode_char_class(pattern.strip_prefix('[')?.strip_suffix(']')?)?;
    let mut replacement = replacement.chars();
    let with = replacement.next();
    if replacement.next().is_some() {
        return None;
    }
    Some(KeyRewriteStep::ReplaceChars(chars, with))
}

/// Every rewrite one span of the script performs, in the order it writes them.
///
/// Declines the whole span on a replacement it cannot carry out: a key rewritten
/// by only SOME of the script's steps lands under a name no later processor
/// reads, which is worse than leaving the script unclaimed.
fn key_rewrite_steps(text: &str) -> Option<Vec<KeyRewriteStep>> {
    let mut found: Vec<(usize, KeyRewriteStep)> = Vec::new();

    for (at, _) in text.match_indices(".toLowerCase()") {
        found.push((at, KeyRewriteStep::Lowercase));
    }

    for (at, _) in text.match_indices(".matcher(") {
        let pattern = regex_before_matcher(text, at)?;
        let arguments = &text[at + ".matcher(".len()..];
        let call = arguments.split_once(".replaceAll(")?.1;
        found.push((at, key_rewrite_step(&pattern, &quoted_first(call)?)?));
    }

    // `.replace("/", "_")` -- a literal swap chained onto the same expression.
    for (at, _) in text.match_indices(".replace(") {
        let arguments = text[at + ".replace(".len()..].split_once(')')?.0;
        let literals = quoted_members(arguments);
        let [from, to] = literals.as_slice() else {
            return None;
        };
        let mut to = to.chars();
        let with = to.next();
        if to.next().is_some() || from.chars().count() != 1 {
            return None;
        }
        found.push((at, KeyRewriteStep::ReplaceChars(from.clone(), with)));
    }

    found.sort_by_key(|(at, _)| *at);
    Some(found.into_iter().map(|(_, step)| step).collect())
}

/// Read the map, the helper it rebuilds each key through, and where it lands.
///
/// ```painless
/// String underscore(String s) { return /[ -]/.matcher(s).replaceAll('_'); }
/// def out = [:];
/// for (def item : ctx.a.b.entrySet()) { out[underscore(item.getKey())] = item.getValue(); }
/// ctx.a.b = out;
/// ```
fn parse_rewrite_keys(script: &str) -> Option<RewriteKeys> {
    use crate::painless_params::clean_path;

    let path_ok = |path: &str| {
        !path.is_empty()
            && path
                .chars()
                .all(|c| c.is_alphanumeric() || "._?".contains(c))
    };

    let loop_at = script.find(" : ctx.")?;
    let source = script[loop_at + " : ctx.".len()..]
        .split_once(".entrySet()")?
        .0;
    if !path_ok(source) {
        return None;
    }

    let store = script.rfind(" = out")?;
    let before_store = &script[..store];
    let target = &before_store[before_store.rfind("ctx.")? + "ctx.".len()..];
    if !path_ok(target) {
        return None;
    }

    // The loop body, and the helper it names. Anything the body does to the key
    // runs BEFORE the helper does, so the two spans are read in that order and
    // never by their position in the text -- the helper is DEFINED first and
    // CALLED last.
    let body = &script[loop_at..store];
    if !body.contains("] = item.getValue()") {
        return None;
    }
    let call_at = body.find("out[")?;
    let helper = body[call_at + "out[".len()..].split_once('(')?.0;
    if helper.is_empty() || !helper.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    // The definition, which precedes the loop, and its braced body.
    let head = &script[..loop_at];
    let define_at = head.find(&format!("{helper}("))?;
    let open = head[define_at..].find('{')? + define_at;
    let mut depth = 0usize;
    let mut close = None;
    for (i, c) in head[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    close = Some(open + i);
                    break;
                }
            }
            _ => {}
        }
    }

    let mut steps = key_rewrite_steps(&body[..call_at])?;
    steps.extend(key_rewrite_steps(&head[open..close?])?);
    if steps.is_empty() {
        return None;
    }

    Some(RewriteKeys::new(
        clean_path(source),
        clean_path(target),
        steps,
    ))
}

/// `_?([a-z])([A-Z]+)` replaced by `$1_$2`, the way Java's matcher walks it.
///
/// The optional underscore is part of the MATCH and the replacement does not
/// write it back, so `aB_cD` comes out `a_Bc_D` and not `a_B_c_D`.
fn camel_break(key: &str) -> String {
    let chars: Vec<char> = key.chars().collect();
    let mut out = String::with_capacity(key.len() + 4);
    let mut i = 0;
    while i < chars.len() {
        let mut at = i;
        if chars[at] == '_' {
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

/// One key through the script's steps, left to right.
fn rewrite_key(key: &str, steps: &[KeyRewriteStep]) -> String {
    let mut key = key.to_owned();
    for step in steps {
        key = match step {
            KeyRewriteStep::CamelBreak => camel_break(&key),
            KeyRewriteStep::Lowercase => key.to_lowercase(),
            KeyRewriteStep::DropControl => key.chars().filter(|c| !c.is_control()).collect(),
            KeyRewriteStep::ReplaceChars(chars, with) => key
                .chars()
                .filter_map(|c| if chars.contains(c) { *with } else { Some(c) })
                .collect(),
        };
    }
    key
}

/// Rebuild the map with every key rewritten, in the order it already had.
pub fn rewrite_keys(event: &mut Event, pattern: &RewriteKeys) -> bool {
    let Some(entries) = event
        .get(&pattern.source)
        .and_then(Value::as_object)
        .cloned()
    else {
        return false;
    };
    let mut rebuilt = serde_json::Map::with_capacity(entries.len());
    for (key, value) in entries {
        let value = if pattern.recursive {
            rewrite_nested(value, &pattern.steps)
        } else {
            value
        };
        rebuilt.insert(rewrite_key(&key, &pattern.steps), value);
    }
    // The removal is half the point where the script MOVES the payload: leaving
    // the source behind emits every field of it a second time.
    if pattern.remove_source && pattern.source != pattern.target {
        event.remove(&pattern.source);
    }
    let _ = event.set(&pattern.target, Value::Object(rebuilt));
    true
}

/// The same key rewrite applied at every depth, through lists as well as maps.
fn rewrite_nested(value: Value, steps: &[KeyRewriteStep]) -> Value {
    match value {
        Value::Object(members) => Value::Object(
            members
                .into_iter()
                .map(|(key, value)| (rewrite_key(&key, steps), rewrite_nested(value, steps)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(
            items
                .into_iter()
                .map(|item| rewrite_nested(item, steps))
                .collect(),
        ),
        scalar => scalar,
    }
}

/// One entry MOVED out of a map, addressed by a key that holds a dot.
///
/// The rename a `rename` processor cannot do: the key is `imageFile.md5String`
/// literally, so a dotted path would read two fields. cybereason writes the
/// value straight to its target; `f5_bigip` builds a `HashMap` around it first
/// and merges that, which lands in the same place.
///
/// The removal is half the point -- leaving the original behind emits a field
/// Elasticsearch does not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveMapEntry {
    /// The map the key sits in.
    container: String,
    /// The literal key, dot and all.
    key: String,
    target: String,
}

impl MoveMapEntry {
    /// The generated call site that rebuilds this pattern.
    #[cfg(feature = "codegen")]
    pub(crate) fn direct_call(&self) -> String {
        format!(
            "move_map_entry(event, &MoveMapEntry::new({}.into(), {}.into(), {}.into()));",
            rust_str(&self.container),
            rust_str(&self.key),
            rust_str(&self.target),
        )
    }

    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(container: String, key: String, target: String) -> Self {
        Self {
            container,
            key,
            target,
        }
    }
}

/// `def obj = ctx.a.remove("b.c"); ctx.d = obj;` and the `put` spelling of it.
fn parse_move_map_entry(script: &str) -> Option<MoveMapEntry> {
    let (head, rest) = script.split_once(".remove(")?;
    let container = clean_path(head.rsplit("ctx.").next()?.trim());
    // The receiver must BE a ctx path, not merely follow one.
    if container.is_empty()
        || container
            .chars()
            .any(|c| !(c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@')))
    {
        return None;
    }
    let key = rest
        .trim_start()
        .strip_prefix(['\'', '"'])?
        .split(['\'', '"'])
        .next()?
        .to_owned();
    if key.is_empty() {
        return None;
    }

    // Written straight to its target, or put into a map that is merged in.
    // Both spellings end at one path.
    let target = if let Some((before, _)) = script.split_once(" = obj;") {
        clean_path(before.rsplit("ctx.").next()?.trim())
    } else {
        let (before, _) = script.rsplit_once(", obj)")?;
        let (path, sub) = before.rsplit_once(".put(")?;
        let sub = sub.trim().trim_matches(['\'', '"']);
        let path = clean_path(path.rsplit("ctx.").next()?.trim());
        if sub.is_empty() || path.is_empty() {
            return None;
        }
        format!("{path}.{sub}")
    };
    if target.is_empty() || target.contains(char::is_whitespace) {
        return None;
    }
    Some(MoveMapEntry::new(container, key, target))
}

/// Take the entry out of the map and write it at the target.
pub fn move_map_entry(event: &mut Event, pattern: &MoveMapEntry) -> bool {
    let Some(mut container) = event
        .get(&pattern.container)
        .and_then(Value::as_object)
        .cloned()
    else {
        return false;
    };
    // `shift_remove`, so taking the key out does not move another into its slot.
    let Some(value) = container.shift_remove(&pattern.key) else {
        return false;
    };
    let _ = event.set(&pattern.container, Value::Object(container));
    let _ = event.set(&pattern.target, value);
    true
}

/// The ECS email block, read out of a `mailto:` URI's query string.
///
/// `eset_protect` ships the whole mail as one URI and pulls `from`, `subject`
/// and `attachment` back out of it. The `from` value holds RFC 5321 addresses
/// in angle brackets, and only those are taken -- the display name around them
/// is not an address.
///
/// A URI with none of the three writes NOTHING, which is the script's own
/// early return rather than an empty block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MailtoUriFields {
    source: String,
    target: String,
}

impl MailtoUriFields {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(source: String, target: String) -> Self {
        Self { source, target }
    }
}

/// `String uri = ctx.a.b; ... /(?:\?|&)from=([^&]+)/.matcher(uri) ... ctx.email['from']`
fn parse_mailto_uri_fields(script: &str) -> Option<MailtoUriFields> {
    // Every one of the three parameters, or this is a different reader.
    for parameter in ["from=", "subject=", "attachment="] {
        if !script.contains(parameter) {
            return None;
        }
    }
    let (head, _) = script.split_once(".matcher(")?;
    let source = clean_path(
        head.split_once("= ctx.")?
            .1
            .split([';', '\n'])
            .next()?
            .trim(),
    );
    let (target_head, _) = script.split_once("['from']")?;
    let target = clean_path(target_head.rsplit("ctx.").next()?.trim());
    if source.is_empty() || target.is_empty() {
        return None;
    }
    Some(MailtoUriFields::new(source, target))
}

/// Read the three parameters, and write only what the URI actually carries.
pub fn mailto_uri_fields(event: &mut Event, pattern: &MailtoUriFields) -> bool {
    let Some(uri) = event.get_string(&pattern.source) else {
        return false;
    };

    // `fast()` because both patterns are plain: no lookaround, so neither
    // falls through to the backtracking engine, which exposes no captures.
    let Some(query) =
        crate::cached_regex!(r"(?:\?|&)(?P<name>from|subject|attachment)=(?P<value>[^&]+)").fast()
    else {
        return false;
    };
    let parameter = |name: &str| -> Option<String> {
        query
            .captures_iter(&uri)
            .find(|caps| caps.name("name").is_some_and(|m| m.as_str() == name))
            .and_then(|caps| Some(caps.name("value")?.as_str().to_owned()))
    };

    // Only the bracketed addresses: the display name around them is not one.
    let Some(bracketed) = crate::cached_regex!(
        r"<\s*([A-Za-z0-9.!#$%&'*+/?^_`{|}~-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,})\s*>"
    )
    .fast() else {
        return false;
    };
    let addresses: Vec<Value> = parameter("from")
        .into_iter()
        .flat_map(|value| {
            bracketed
                .captures_iter(&value)
                .filter_map(|caps| Some(json!(caps.get(1)?.as_str())))
                .collect::<Vec<_>>()
        })
        .collect();
    let subject = parameter("subject");
    let attachment = parameter("attachment");

    // The script's own early return: nothing actionable, nothing written.
    if addresses.is_empty() && subject.is_none() && attachment.is_none() {
        return false;
    }

    if !addresses.is_empty() {
        let _ = event.set(
            &format!("{}.from.address", pattern.target),
            Value::Array(addresses),
        );
    }
    if let Some(subject) = subject {
        let _ = event.set(&format!("{}.subject", pattern.target), json!(subject));
    }
    if let Some(attachment) = attachment {
        let _ = event.set(
            &format!("{}.attachments", pattern.target),
            json!([{ "file": { "name": attachment } }]),
        );
    }
    true
}

/// A field split at a delimiter, its halves written to named targets.
///
/// The pair of scripts a dissect leaves behind. envoyproxy's `dest` holds
/// `address:port` and its `proto` holds `HTTP/1.1`, and each is cut with
/// `indexOf` and two `substring`s. Both became REACHABLE only once
/// [`EnsurePrefix`] made the dissect succeed, which is why they turned up as
/// newly unbound rather than being there all along.
///
/// A value with no delimiter is NOT handled: Painless's `substring` throws on
/// the `-1` that `indexOf` returns, which is the vendor's own failure path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplitAtDelimiter {
    source: String,
    delimiter: String,
    /// Where the part BEFORE the delimiter goes, when the script writes it.
    head: Option<String>,
    /// Where the part AFTER it goes.
    tail: Option<String>,
    /// A whole value that means "no data" -- the source is dropped and
    /// nothing is written.
    sentinel: Option<String>,
    /// Whether the source is removed once it has been read.
    remove_source: bool,
}

impl SplitAtDelimiter {
    /// The generated call site that rebuilds this pattern.
    #[cfg(feature = "codegen")]
    pub(crate) fn direct_call(&self) -> String {
        let option = |value: &Option<String>| {
            value.as_ref().map_or_else(
                || "None".to_owned(),
                |v| format!("Some({}.into())", rust_str(v)),
            )
        };
        format!(
            "split_at_delimiter(event, &SplitAtDelimiter::new({}.into(), {}.into(), {}, {}, {}, {}));",
            rust_str(&self.source),
            rust_str(&self.delimiter),
            option(&self.head),
            option(&self.tail),
            option(&self.sentinel),
            self.remove_source,
        )
    }

    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        source: String,
        delimiter: String,
        head: Option<String>,
        tail: Option<String>,
        sentinel: Option<String>,
        remove_source: bool,
    ) -> Self {
        Self {
            source,
            delimiter,
            head,
            tail,
            sentinel,
            remove_source,
        }
    }
}

/// `def p = ctx.a.indexOf(':'); ctx.b = ctx.a.substring(0, p); ...`
fn parse_split_at_delimiter(script: &str) -> Option<SplitAtDelimiter> {
    // The cut names the source and the delimiter. The vendors write a SPACE
    // before the paren -- `ctx.dest.indexOf (':')` -- in both of these.
    let (head_text, rest) = script.split_once(".indexOf")?;
    let source = clean_path(head_text.rsplit("ctx.").next()?.trim());
    // The receiver must be that ctx path, not merely follow one: sentinel_one
    // binds `def path = ...` and calls `path.indexOf`, where reading back to
    // the last `ctx.` captures half a statement and still parses.
    if source.is_empty()
        || source
            .chars()
            .any(|c| !(c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@')))
    {
        return None;
    }
    let delimiter = rest
        .trim_start()
        .strip_prefix('(')?
        .trim_start()
        .strip_prefix(['\'', '"'])?
        .split(['\'', '"'])
        .next()?
        .to_owned();
    if source.is_empty() || delimiter.is_empty() {
        return None;
    }

    // Each half is claimed by the substring that reads it: `(0, p)` is the
    // part before the delimiter, `(p+1, ..)` the part after.
    let target_of = |needle: &str| -> Option<String> {
        let (before, _) = script.split_once(needle)?;
        let assignment = before.rsplit(['\n', ';', '{', '}']).next()?;
        let path = assignment.split('=').next()?.trim();
        let path = clean_path(path.strip_prefix("ctx.")?.trim());
        (!path.is_empty()).then_some(path)
    };
    let head = target_of(".substring(0, ");
    let tail = script.split_once(".substring(").and_then(|_| {
        // `substring(p+1, l)` -- the local's name varies, so anchor on the
        // `+1` that every one of these writes.
        let (before, _) = script.split_once("+1, ")?;
        let assignment = before.rsplit(['\n', ';', '{', '}']).next()?;
        let path = assignment.split('=').next()?.trim();
        let path = clean_path(path.strip_prefix("ctx.")?.trim());
        (!path.is_empty()).then_some(path)
    });
    if head.is_none() && tail.is_none() {
        return None;
    }

    // The sentinel arm, and whether the source survives being read.
    let sentinel = script
        .split_once("if (ctx.")
        .and_then(|(_, rest)| rest.split_once("=="))
        .and_then(|(_, rest)| {
            let literal = rest.trim().strip_prefix(['\'', '"'])?;
            Some(literal.split(['\'', '"']).next()?.to_owned())
        })
        .filter(|value| !value.is_empty());
    let last = source.rsplit('.').next()?;
    let remove_source = script.contains(&format!("remove('{last}')"))
        || script.contains(&format!("remove(\"{last}\")"));

    Some(SplitAtDelimiter::new(
        source,
        delimiter,
        head,
        tail,
        sentinel,
        remove_source,
    ))
}

/// Cut the source and write each half the script names.
pub fn split_at_delimiter(event: &mut Event, pattern: &SplitAtDelimiter) -> bool {
    let Some(value) = event.get_string(&pattern.source) else {
        return false;
    };

    // The sentinel means no data: the source goes and nothing is written.
    if pattern.sentinel.as_deref() == Some(value.as_str()) {
        event.remove(&pattern.source);
        return true;
    }

    // FIRST occurrence, the way `indexOf` reads it.
    let Some(at) = value.find(&pattern.delimiter) else {
        // `indexOf` returns -1 and the vendor's `substring` throws on it.
        return false;
    };
    if let Some(target) = &pattern.head {
        let _ = event.set(target, json!(&value[..at]));
    }
    if let Some(target) = &pattern.tail {
        let _ = event.set(target, json!(&value[at + pattern.delimiter.len()..]));
    }
    if pattern.remove_source {
        event.remove(&pattern.source);
    }
    true
}

/// A message normalised to carry a known prefix before it is dissected.
///
/// envoyproxy's access log arrives two ways: bare, starting `[`, or already
/// carrying `ACCESS `. One dissect pattern reads both, so the script adds the
/// prefix to the bare form and passes the other through. Everything
/// downstream reads the prefixed copy, so leaving this unbound costs the whole
/// source -- all 7 of its events and 137 fields.
///
/// A value that is NEITHER is not handled here. The vendor throws there, which
/// takes the processor's `on_failure` path, and an arm that quietly wrote
/// nothing would claim a script it did not apply.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnsurePrefix {
    source: String,
    target: String,
    /// The first character that means the prefix is missing.
    marker: char,
    prefix: String,
}

impl EnsurePrefix {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(source: String, target: String, marker: char, prefix: String) -> Self {
        Self {
            source,
            target,
            marker,
            prefix,
        }
    }
}

/// `if (ctx.a.charAt(0) == (char)("[")) { ctx.b = "P " + ctx.a; } else if ...`
fn parse_ensure_prefix(script: &str) -> Option<EnsurePrefix> {
    // The marker, and the field it is tested on.
    let (head, rest) = script.split_once(".charAt(0)")?;
    let source = clean_path(head.rsplit("ctx.").next()?.trim());
    let marker = rest
        .split_once("(char)")?
        .1
        .trim()
        .trim_start_matches('(')
        .trim()
        .trim_matches(['\'', '"'])
        .chars()
        .next()?;

    // The write that ADDS the prefix, anchored on the CONCATENATION rather
    // than on an `=`: the test above it is `==`, and splitting on the first
    // `=` lands in the middle of that.
    let (head, _) = rest.split_once("+ ctx.")?;
    let head = head.trim_end();
    let quote = head.chars().last()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let (before, prefix) = head[..head.len() - quote.len_utf8()].rsplit_once(quote)?;
    let prefix = prefix.to_owned();
    let target = clean_path(
        before
            .rsplit("ctx.")
            .next()?
            .trim()
            .trim_end_matches('=')
            .trim(),
    );
    if target.is_empty() || prefix.is_empty() || source.is_empty() {
        return None;
    }

    // The pass-through arm must test for that same prefix, or this is a
    // different script that happens to start the same way.
    if !script.contains(".substring(0, ") || !script.contains(&prefix) {
        return None;
    }
    Some(EnsurePrefix::new(source, target, marker, prefix))
}

/// Write the prefixed copy; leave a value that is neither form alone.
pub fn ensure_prefix(event: &mut Event, pattern: &EnsurePrefix) -> bool {
    let Some(value) = event.get_string(&pattern.source) else {
        return false;
    };
    if value.starts_with(pattern.marker) {
        let _ = event.set(&pattern.target, json!(format!("{}{value}", pattern.prefix)));
        return true;
    }
    if value.starts_with(&pattern.prefix) {
        let _ = event.set(&pattern.target, json!(value));
        return true;
    }
    // The vendor throws here. Claiming it would count a script this arm did
    // not apply, which is exactly what the never_ran ratchet exists to catch.
    false
}

/// Named keys of a map dropped when what they hold is an EMPTY map.
///
/// The tidy-up after a dot expansion. sysdig expands `proc.pid.ts` into a
/// nested `proc.pid.ts`, renames the leaf to `proc.pid_ts`, and is left with
/// `proc.pid` holding `{}` -- a container Elasticsearch does not emit. The
/// script names the keys to check rather than sweeping the map, so an empty
/// map the vendor means to keep is untouched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveEmptyChildMaps {
    /// The map the keys sit in.
    container: String,
    /// The keys to drop, in the order the script checks them.
    keys: Vec<String>,
}

impl RemoveEmptyChildMaps {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(container: String, keys: Vec<String>) -> Self {
        Self { container, keys }
    }

    /// The generated call site that rebuilds this pattern.
    #[cfg(feature = "codegen")]
    pub(crate) fn direct_call(&self) -> String {
        let keys: Vec<String> = self
            .keys
            .iter()
            .map(|key| format!("{}.into()", rust_str(key)))
            .collect();
        format!(
            "remove_empty_child_maps(event, &RemoveEmptyChildMaps::new({}.into(), vec![{}]));",
            rust_str(&self.container),
            keys.join(", "),
        )
    }
}

/// `if (p.containsKey('k') && p.k instanceof Map && p.k.size() == 0) p.remove('k');`
fn parse_remove_empty_child_maps(script: &str) -> Option<RemoveEmptyChildMaps> {
    // `def proc = ctx.<container>;` -- the local every check reads through.
    let (head, tail) = script.split_once(" = ctx.")?;
    let local = head.rsplit(['\n', ';', '{', '}']).next()?.trim();
    let local = local.strip_prefix("def ").unwrap_or(local).trim();
    if local.is_empty() || local.contains(char::is_whitespace) {
        return None;
    }
    let container = clean_path(tail.split([';', '\n']).next()?.trim());
    if container.is_empty() {
        return None;
    }

    // Every key checked for an empty map AND removed. Both halves, or the
    // script is doing something else with it.
    let mut keys = Vec::new();
    for piece in script.split(&format!("{local}.containsKey(")).skip(1) {
        let key = piece
            .split_once(')')?
            .0
            .trim()
            .trim_matches(['\'', '"'])
            .to_owned();
        let (guard, body) = piece.split_once(") {")?;
        if key.is_empty()
            || !guard.contains("instanceof Map")
            || !guard.contains(".size() == 0")
            || !body.contains(&format!("{local}.remove("))
        {
            return None;
        }
        keys.push(key);
    }
    (!keys.is_empty()).then(|| RemoveEmptyChildMaps::new(container, keys))
}

/// Drop each named key that holds an empty map; leave everything else.
pub fn remove_empty_child_maps(event: &mut Event, pattern: &RemoveEmptyChildMaps) -> bool {
    for key in &pattern.keys {
        let path = format!("{}.{key}", pattern.container);
        // An empty map ONLY. A missing key, a populated map and a scalar are
        // all left where they are.
        if event
            .get(&path)
            .and_then(Value::as_object)
            .is_some_and(serde_json::Map::is_empty)
        {
            event.remove(&path);
        }
    }
    true
}

/// A vendor flag folded to a real boolean by comparing its string spelling.
///
/// Distinct from [`KnownPattern::TruthyAssignments`], which is m365's
/// `isTruthy(...)` helper and leaves a value it cannot resolve ALONE. This is
/// written inline and always assigns: a spelling not in the truthy set becomes
/// `false`, not "unchanged", so the two cannot share a runner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoerceBoolean {
    /// Each `(source, target)` the script folds, in the order it writes them.
    fields: Vec<(String, String)>,
    /// The lower-cased spellings that mean true. Anything else is false.
    truthy: Vec<String>,
}

impl CoerceBoolean {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(fields: Vec<(String, String)>, truthy: Vec<String>) -> Self {
        Self { fields, truthy }
    }

    /// The generated call site that rebuilds this pattern.
    #[cfg(feature = "codegen")]
    pub(crate) fn direct_call(&self) -> String {
        let fields: Vec<String> = self
            .fields
            .iter()
            .map(|(source, target)| {
                format!(
                    "({}.to_owned(), {}.to_owned())",
                    rust_str(source),
                    rust_str(target)
                )
            })
            .collect();
        let truthy: Vec<String> = self
            .truthy
            .iter()
            .map(|spelling| format!("{}.to_owned()", rust_str(spelling)))
            .collect();
        format!(
            "coerce_boolean(event, &CoerceBoolean::new(vec![{}], vec![{}]));",
            fields.join(", "),
            truthy.join(", "),
        )
    }
}

/// `def v = ctx.a.toString().toLowerCase(); ctx.a = (v == "true" || v == "1");`
///
/// Every field in the script must agree on the truthy set. Two different sets
/// in one script is a different pattern -- probably two unrelated coercions --
/// and claiming it would apply one field's spellings to another's.
fn parse_coerce_boolean(script: &str) -> Option<CoerceBoolean> {
    const FOLD: &str = ".toString().toLowerCase()";

    let chunks: Vec<&str> = script.split(FOLD).collect();
    if chunks.len() < 2 {
        return None;
    }

    let mut fields = Vec::with_capacity(chunks.len() - 1);
    let mut truthy: Option<Vec<String>> = None;
    for pair in chunks.windows(2) {
        let source = painless_path(pair[0])?;
        // Past the fold's own statement, to the assignment that consumes it.
        let (_, assignment) = pair[1].split_once(';')?;
        let (lhs, rhs) = assignment.split_once('=')?;
        let target = painless_path(lhs)?;

        // An OR of equality tests and nothing else -- a negation or an AND
        // means the flag is not simply "is it one of these spellings".
        let expression = rhs.split(';').next()?;
        if expression.contains("&&") || expression.contains("!=") {
            return None;
        }
        let mut spellings = Vec::new();
        for piece in expression.split("||") {
            let literal = piece
                .split_once("==")?
                .1
                .trim()
                .trim_matches(['(', ')', ' ']);
            spellings.push(literal.strip_prefix('"')?.strip_suffix('"')?.to_owned());
        }

        match &truthy {
            Some(agreed) if *agreed != spellings => return None,
            Some(_) => {}
            None => truthy = Some(spellings),
        }
        fields.push((source, target));
    }
    Some(CoerceBoolean::new(fields, truthy?))
}

/// Fold each present field to a boolean; an absent one is left absent.
pub fn coerce_boolean(event: &mut Event, pattern: &CoerceBoolean) -> bool {
    for (source, target) in &pattern.fields {
        let Some(value) = event.get_as_string(source) else {
            continue;
        };
        let folded = value.to_lowercase();
        let _ = event.set(target, json!(pattern.truthy.contains(&folded)));
    }
    true
}

/// An ECS `geo_point` built from a `GeoJSON` coordinate array.
///
/// `GeoJSON` orders a position `[longitude, latitude]`, which is the reverse of
/// how every human writes one, so the INDICES are read off the script rather
/// than assumed -- a vendor that writes `['lat': c[1], 'lon': c[0]]` and one
/// that writes them the other way round must both come out right.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeoPointFromCoordinates {
    coordinates: String,
    target: String,
    lon_index: usize,
    lat_index: usize,
    /// `lon` written before `lat`. Under `preserve_order` the key order is
    /// part of the document Elasticsearch renders, so it is not cosmetic.
    lon_first: bool,
}

impl GeoPointFromCoordinates {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        coordinates: impl Into<String>,
        target: impl Into<String>,
        lon_index: usize,
        lat_index: usize,
        lon_first: bool,
    ) -> Self {
        Self {
            coordinates: coordinates.into(),
            target: target.into(),
            lon_index,
            lat_index,
            lon_first,
        }
    }
}

/// `ctx.t = ['lon': geom.coordinates[0], 'lat': geom.coordinates[1]];`
fn parse_geo_point_from_coordinates(script: &str) -> Option<GeoPointFromCoordinates> {
    /// The array and the index one entry of the map literal reads.
    fn entry<'a>(statement: &'a str, key: &str) -> Option<(&'a str, usize)> {
        let value = statement.split_once(key)?.1;
        let value = value.split([',', ']']).next()?.trim();
        let (base, index) = value.rsplit_once('[')?;
        Some((base, index.trim_end_matches(']').trim().parse().ok()?))
    }

    let lon_at = script.find("'lon':")?;
    let lat_at = script.find("'lat':")?;
    let statement = &script[lon_at.min(lat_at)..];

    // The assignment this map literal is the value of.
    let target = painless_path(script[..lon_at.min(lat_at)].rsplit_once('=')?.0)?;

    let (lon_base, lon_index) = entry(statement, "'lon':")?;
    let (lat_base, lat_index) = entry(statement, "'lat':")?;
    // Two different arrays is a different pattern -- this one is one position.
    if lon_base != lat_base || lon_index == lat_index {
        return None;
    }

    let coordinates = resolve_subject(script, lon_base.trim(), 3)?;
    Some(GeoPointFromCoordinates::new(
        coordinates,
        target,
        lon_index,
        lat_index,
        lon_at < lat_at,
    ))
}

/// Write the point, but only where both positions are actually NUMBERS.
///
/// That check is the type guard the script spells as `geomType == "Point"`: a
/// Polygon's `coordinates[0]` is itself an array, so a non-numeric position is
/// exactly the case the vendor's guard excludes.
pub fn geo_point_from_coordinates(event: &mut Event, pattern: &GeoPointFromCoordinates) -> bool {
    let Some(Value::Array(coordinates)) = event.get(&pattern.coordinates) else {
        return true;
    };
    let (Some(lon), Some(lat)) = (
        coordinates.get(pattern.lon_index),
        coordinates.get(pattern.lat_index),
    ) else {
        return true;
    };
    if !lon.is_number() || !lat.is_number() {
        return true;
    }
    let (lon, lat) = (lon.clone(), lat.clone());

    let mut point = Map::new();
    if pattern.lon_first {
        point.insert("lon".to_owned(), lon);
        point.insert("lat".to_owned(), lat);
    } else {
        point.insert("lat".to_owned(), lat);
        point.insert("lon".to_owned(), lon);
    }
    let _ = event.set(&pattern.target, Value::Object(point));
    true
}

/// A `GeoJSON` geometry rendered as WKT text, with everything else the same
/// script writes.
///
/// gdacs enriches a feature with a second geometry -- the area the event
/// affects -- and stores it as WKT rather than as `GeoJSON`, then overwrites two
/// fields from the enrichment's own copies. The centroid point and those copies
/// are carried HERE rather than left to `GeoPointFromCoordinates` and
/// `BranchCopies`, because all three are ONE script and the dispatch runs only
/// the first matcher that claims it. The point matcher answers true for every
/// script its trigger fires on, so anything after it never ran: gdacs lost the
/// WKT text and both copies to that.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WktGeometry {
    /// The `GeoJSON` geometry the text is rendered from.
    geometry: String,
    /// Where the rendered text is written.
    target: String,
    /// Where the geometry's own type is written. gdacs keeps it as scratch and
    /// removes it downstream; a vendor that keeps no copy still binds here.
    type_target: Option<String>,
    /// The centroid the same script writes, when it writes one.
    point: Option<GeoPointFromCoordinates>,
    /// The copies the same type guard encloses.
    branches: Vec<BranchCopy>,
}

/// One `GeoJSON` position, as WKT's `<x> <y>`.
///
/// The numbers are spelled the way Painless's `toString` spells them, which is
/// what the script appends: an integer keeps its integer text and a double keeps
/// its decimal point, so a whole-number coordinate reads `163.0` and not `163`.
/// The two engines still part company below `1e-3`, where Java switches to
/// `1.0E-4` and this does not -- no coordinate in the corpus is that small, so
/// the divergence is recorded rather than engineered around.
fn wkt_position(position: &Value, out: &mut String) -> Option<()> {
    let position = position.as_array()?;
    let (x, y) = (position.first()?, position.get(1)?);
    // Painless raises on a null here rather than appending four letters, so a
    // position carrying one declines the whole render.
    if x.is_null() || y.is_null() {
        return None;
    }
    out.push_str(&crate::painless_helpers::painless_to_string(x));
    out.push(' ');
    out.push_str(&crate::painless_helpers::painless_to_string(y));
    Some(())
}

/// A coordinate list at `depth` nestings of parentheses: 1 is a line or one
/// ring, 2 a polygon's rings, 3 a multipolygon's polygons.
///
/// WKT separates positions with `, ` and wraps each level in its own
/// parentheses, and that nesting is the whole difference between the four
/// spellings the vendor's helper writes.
fn wkt_coordinates(coordinates: &Value, depth: u8, out: &mut String) -> Option<()> {
    let items = coordinates.as_array()?;
    out.push('(');
    for (index, item) in items.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        if depth <= 1 {
            wkt_position(item, out)?;
        } else {
            wkt_coordinates(item, depth - 1, out)?;
        }
    }
    out.push(')');
    Some(())
}

/// The geometry as WKT, or `None` for a type this does not spell.
///
/// The four it does spell are the four the vendor's own guard admits, so a
/// geometry that declines here is one the guard excludes.
fn geojson_to_wkt(geometry: &Value) -> Option<String> {
    let (name, depth) = match geometry.get("type")?.as_str()? {
        "LineString" => ("LINESTRING ", 1),
        "Polygon" => ("POLYGON ", 2),
        // A multi-line's rings nest exactly as a polygon's do; only the name
        // differs, which is why the vendor's helper shares its body.
        "MultiLineString" => ("MULTILINESTRING ", 2),
        "MultiPolygon" => ("MULTIPOLYGON ", 3),
        _ => return None,
    };
    let mut out = String::from(name);
    wkt_coordinates(geometry.get("coordinates")?, depth, &mut out)?;
    Some(out)
}

/// `ctx.<target> = <helper>(<local>);`, where `<helper>` renders WKT.
///
/// The helper's NAME is read off the script rather than assumed, so a second
/// vendor writing the same rendering binds here too. What identifies it is the
/// `POLYGON` literal it appends: the declaration that literal sits inside is the
/// one whose result is stored.
fn parse_wkt_geometry(script: &str) -> Option<WktGeometry> {
    let polygon_at = script.find("\"POLYGON \"")?;
    let declaration = script[..polygon_at].rfind("(def ")?;
    let name = script[..declaration]
        .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()
        .filter(|name| !name.is_empty())?;

    // Where the rendered text lands, and the geometry handed to the helper.
    let call = format!("= {name}(");
    let at = script.find(&call)?;
    let target = painless_path(&script[..at])?;
    let subject = script[at + call.len()..].split_once(')')?.0.trim();
    let geometry = resolve_subject(script, subject, 3)?;

    // `String <local> = <subject>.type;` and then `ctx.<target> = <local>;`.
    let type_target = script
        .split_once(&format!(" = {subject}.type;"))
        .and_then(|(head, _)| head.rsplit(char::is_whitespace).next())
        .filter(|local| !local.is_empty())
        .and_then(|local| {
            let at = script.find(&format!(" = {local};"))?;
            painless_path(&script[..at])
        });

    Some(WktGeometry {
        geometry,
        target,
        type_target,
        point: parse_geo_point_from_coordinates(script),
        branches: parse_branch_copies(script).unwrap_or_default(),
    })
}

/// Write the centroid, the rendered geometry and the guarded copies -- the
/// three things the one script does.
///
/// A geometry the renderer declines writes NOTHING, the scratch type included:
/// that is the vendor's type guard, which admits exactly the four spellings the
/// renderer knows.
fn wkt_geometry(event: &mut Event, pattern: &WktGeometry) -> bool {
    if let Some(point) = &pattern.point {
        geo_point_from_coordinates(event, point);
    }

    let rendered = event.get(&pattern.geometry).and_then(|geometry| {
        let kind = geometry.get("type")?.as_str()?.to_owned();
        Some((geojson_to_wkt(geometry)?, kind))
    });
    if let Some((wkt, kind)) = rendered {
        let _ = event.set(&pattern.target, wkt);
        if let Some(target) = &pattern.type_target {
            let _ = event.set(target, kind);
        }
    }

    run_branch_copies(event, &pattern.branches)
}

/// A number rendered as an octal string, which is how a file mode reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OctalString {
    source: String,
    target: String,
}

impl OctalString {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
        }
    }
}

/// One step of a string-op chain, its literal arguments already resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringOp {
    Lower,
    Upper,
    Trim,
    Replace { from: String, to: String },
}

impl StringOp {
    fn apply(&self, text: &str) -> String {
        match self {
            Self::Lower => text.to_lowercase(),
            Self::Upper => text.to_uppercase(),
            Self::Trim => text.trim().to_string(),
            Self::Replace { from, to } => text.replace(from.as_str(), to),
        }
    }

    /// Parse one `<op>(<args>)` call, declining anything off the allowlist.
    ///
    /// The allowlist is the point: `replaceAll` takes a regex rather than a
    /// literal, so accepting it by pattern would quietly change the semantics.
    fn parse(name: &str, arguments: &str) -> Option<Self> {
        match name {
            "toLowerCase" | "toUpperCase" | "trim" if arguments.trim().is_empty() => {
                Some(match name {
                    "toLowerCase" => Self::Lower,
                    "toUpperCase" => Self::Upper,
                    _ => Self::Trim,
                })
            }
            "replace" => {
                let (from, to) = two_string_literals(arguments)?;
                Some(Self::Replace { from, to })
            }
            _ => None,
        }
    }
}

/// A field read into a local, rewritten by a chain of string ops, written back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringOps {
    source: String,
    target: String,
    ops: Vec<StringOp>,
}

impl StringOps {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(source: impl Into<String>, target: impl Into<String>, ops: Vec<StringOp>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            ops,
        }
    }
}

/// The same ops written as ONE chained expression, with no local at all.
///
/// ```painless
/// ctx.source.mac = ctx.gigamon.ami.src_mac.replace(":", "-").toUpperCase();
/// ```
///
/// `gigamon` writes both its MAC normalisers this way, and between them they
/// are `source.mac` wrong on 49 of its 78 events and `destination.mac` on 53.
/// [`parse_string_ops`] reads the local-variable spelling and cannot see this
/// one, so a second reader over the same `StringOps` is the whole fix.
///
/// The chain is split on `).` rather than `.`, since a `replace(".", "_")`
/// carries a dot inside its own arguments. One op off the allowlist rejects the
/// whole chain, exactly as the other reader does.
fn parse_chained_string_ops(script: &str) -> Option<StringOps> {
    use crate::painless_params::clean_path;

    let statement = script.trim().trim_end_matches(';').trim();
    if statement.contains(';') || statement.contains('\n') {
        return None;
    }
    let (target, expression) = statement.split_once('=')?;
    let target = clean_path(target.trim().strip_prefix("ctx.")?);
    let expression = expression.trim().strip_prefix("ctx.")?;
    if target.is_empty() {
        return None;
    }

    // The source is everything up to the first call, and the calls follow it.
    let (source, chain) = expression.split_once(".replace(").map_or_else(
        || {
            expression
                .split_once(".toUpperCase()")
                .map(|(s, _)| (s, ""))
        },
        |(source, rest)| Some((source, rest)),
    )?;
    let source = clean_path(source.trim());
    if source.is_empty() || source.contains('(') {
        return None;
    }

    // Rebuild the call list, putting back the `replace(` the split consumed.
    let calls = if chain.is_empty() {
        "toUpperCase()".to_owned()
    } else {
        format!("replace({chain}")
    };
    let mut ops = Vec::new();
    for call in split_calls(&calls) {
        let (name, arguments) = call.split_once('(')?;
        ops.push(StringOp::parse(name.trim(), arguments.strip_suffix(')')?)?);
    }
    (!ops.is_empty()).then(|| StringOps::new(source, target, ops))
}

/// Cut a chain into its calls at each `).`, which an argument cannot contain.
fn split_calls(chain: &str) -> Vec<String> {
    let mut calls = Vec::new();
    let mut rest = chain;
    while let Some(at) = rest.find(").") {
        calls.push(rest[..=at].to_owned());
        rest = &rest[at + 2..];
    }
    if !rest.trim().is_empty() {
        calls.push(rest.to_owned());
    }
    calls
}

/// `String v = ctx.<source>; v = v.<op>(..); ..; ctx.<target> = v;`
///
/// Every statement has to fit, and one op off the allowlist rejects the whole
/// chain. A partial parse is worse than none: the pattern binds, the ladder stops,
/// and the ops it could not read are dropped in silence -- which is what the
/// bare `.replace(` trigger below did to `ti_opencti`'s indicator type.
fn parse_string_ops(script: &str) -> Option<StringOps> {
    use crate::painless_params::clean_path;

    let statements: Vec<&str> = script
        .split(['\n', ';'])
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    let [declaration, steps @ .., write_back] = statements.as_slice() else {
        return None;
    };
    if steps.is_empty() {
        return None;
    }

    let (local, source) = declaration
        .strip_prefix("String ")
        .or_else(|| declaration.strip_prefix("def "))?
        .split_once('=')?;
    let (local, source) = (
        local.trim(),
        clean_path(source.trim().strip_prefix("ctx.")?),
    );

    let target = write_back
        .strip_prefix("ctx.")?
        .split_once('=')
        .filter(|(_, bound)| bound.trim() == local)
        .map(|(path, _)| clean_path(path.trim()))?;

    let mut ops = Vec::with_capacity(steps.len());
    for step in steps {
        let call = step
            .strip_prefix(local)?
            .trim_start()
            .strip_prefix('=')?
            .trim()
            .strip_prefix(local)?
            .strip_prefix('.')?;
        let (name, arguments) = call.split_once('(')?;
        ops.push(StringOp::parse(name, arguments.strip_suffix(')')?)?);
    }

    (!source.is_empty() && !target.is_empty()).then(|| StringOps::new(source, target, ops))
}

/// Run the chain, leaving the event alone where the source is not a string.
pub fn string_ops(event: &mut Event, pattern: &StringOps) -> bool {
    let Some(text) = event.get_str(&pattern.source) else {
        return true;
    };
    let value = pattern
        .ops
        .iter()
        .fold(text.to_string(), |text, op| op.apply(&text));
    let _ = event.set(&pattern.target, value);
    true
}

/// `int t = (int)ctx.<source>; ctx.<target> = Integer.toOctalString(t);`
fn parse_octal_string(script: &str) -> Option<OctalString> {
    use crate::painless_params::clean_path;

    let (head, tail) = script.split_once("Integer.toOctalString(")?;
    let local = tail.split(')').next()?.trim();
    let bound = script.split_once(&format!(" {local} = "))?.1;
    let source = painless_path(bound.split(';').next()?)?;
    let target = painless_path(head)?;
    (!source.is_empty() && !target.is_empty())
        .then(|| OctalString::new(source, clean_path(&target)))
}

/// Write the source as octal, the way Painless's 32-bit `(int)` cast renders.
pub fn octal_string(event: &mut Event, pattern: &OctalString) -> bool {
    let Some(value) = event.get_as_i64(&pattern.source) else {
        return true;
    };
    // `(int)` truncates to 32 bits before `toOctalString`, which then reads
    // the result unsigned -- a wider or signed rendering diverges on any
    // negative the vendor happens to send.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let truncated = value as i32 as u32;
    let _ = event.set(&pattern.target, json!(format!("{truncated:o}")));
    true
}

/// A value copied only when it is one of a literal set.
///
/// tychon reads a vendor `os.family`, lower-cases it, and writes ECS
/// `host.os.type` only for the six values ECS allows -- the membership test IS
/// the validation, so copying unconditionally would put a vendor string in a
/// field with a closed vocabulary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllowedValueCopy {
    source: String,
    lower: bool,
    allowed: Vec<String>,
    target: String,
}

impl AllowedValueCopy {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        source: impl Into<String>,
        lower: bool,
        allowed: Vec<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            lower,
            allowed,
            target: target.into(),
        }
    }
}

/// Read the source, the fold, the allowed set and the target off the script.
fn parse_allowed_value_copy(script: &str) -> Option<AllowedValueCopy> {
    use crate::painless_params::clean_path;

    let (head, tail) = script.split_once(" = ctx.")?;
    let local = identifier_before(head)?;
    let bound = tail.split(';').next()?;
    let lower = bound.contains(".toLowerCase()");
    let source = bound.trim_end_matches(".toLowerCase()").trim();

    let list = script.split_once('[')?.1.split_once("].contains(")?;
    let allowed: Vec<String> = quoted_all(list.0);
    if allowed.is_empty() || !list.1.starts_with(local) {
        return None;
    }

    let target = painless_path(script.rsplit_once(&format!(" = {local};"))?.0)?;
    (!source.is_empty()).then(|| AllowedValueCopy::new(clean_path(source), lower, allowed, target))
}

/// Every single-quoted literal in a fragment.
fn quoted_all(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = text;
    while let Some((_, after)) = rest.split_once('\'') {
        let Some((literal, tail)) = after.split_once('\'') else {
            break;
        };
        found.push(literal.to_owned());
        rest = tail;
    }
    found
}

/// Copy the source onto the target when the allowed set holds its value.
pub fn allowed_value_copy(event: &mut Event, pattern: &AllowedValueCopy) -> bool {
    let Some(raw) = event.get_as_string(&pattern.source) else {
        return true;
    };
    let value = if pattern.lower {
        raw.to_lowercase()
    } else {
        raw
    };
    if pattern.allowed.contains(&value) {
        let _ = event.set(&pattern.target, json!(value));
    }
    true
}

/// Prune the whole document, or one subtree where the script names a root.
pub fn drop_empty(event: &mut Event, policy: &DropPolicy, root: Option<&str>) -> bool {
    match (root, policy.shallow) {
        (None, _) => drop_empty_recursive(event, policy),
        (Some(path), false) => drop_subtree(event, policy, path),
        (Some(path), true) => drop_shallow(event, policy, path),
    }
    true
}

/// Drop the root's direct values the policy calls empty, without descending.
fn drop_shallow(event: &mut Event, policy: &DropPolicy, root: &str) {
    let Some(Value::Object(map)) = crate::painless_params::pointer_mut(event, root) else {
        return;
    };
    let doomed: Vec<String> = map
        .iter()
        .filter(|(_, value)| match value {
            Value::Null => policy.nulls,
            Value::String(text) if text.is_empty() => policy.empty_strings,
            Value::String(text) => policy.sentinels.iter().any(|s| s == text),
            _ => false,
        })
        .map(|(key, _)| key.clone())
        .collect();
    for key in doomed {
        map.shift_remove(&key);
    }
}

pub(crate) fn drop_subtree(event: &mut Event, policy: &DropPolicy, root: &str) {
    let Some(value) = crate::painless_params::pointer_mut(event, root) else {
        return;
    };
    let mut shrunk = Vec::new();
    drop_value(
        value,
        policy,
        &mut Marks::new(root.to_string()),
        &mut shrunk,
    );
    for (path, entries) in shrunk {
        event.record_map_capacity(path, entries);
    }
}

/// Where the prune currently is, and whether a list stands between it and the
/// subtree root.
struct Marks {
    path: String,
    /// A list PRUNE renumbers everything after the entry it drops, so a path
    /// through one no longer names the same value once the walk unwinds. The
    /// capacity a render could not trust is not recorded at all, which leaves
    /// it reading the surviving entry count as it did before.
    through_list: bool,
}

impl Marks {
    fn new(path: String) -> Self {
        Self {
            path,
            through_list: false,
        }
    }

    /// Replace whatever sits past `mark` with one more dotted segment.
    fn descend(&mut self, mark: usize, segment: &str) {
        self.path.truncate(mark);
        if !self.path.is_empty() {
            self.path.push('.');
        }
        self.path.push_str(segment);
    }
}

fn drop_value(
    value: &mut Value,
    policy: &DropPolicy,
    marks: &mut Marks,
    shrunk: &mut Vec<(String, usize)>,
) -> bool {
    match value {
        Value::Null => policy.nulls,
        Value::String(s) if s.is_empty() => policy.empty_strings,
        Value::String(s) if policy.sentinels.iter().any(|v| v == s) => true,
        Value::Object(map) => {
            let built = map.len();
            let mark = marks.path.len();
            let keys_to_remove: Vec<String> = map
                .iter_mut()
                .filter_map(|(k, v)| {
                    marks.descend(mark, k);
                    drop_value(v, policy, marks, shrunk).then(|| k.clone())
                })
                .collect();
            marks.path.truncate(mark);
            for key in &keys_to_remove {
                map.shift_remove(key);
            }
            // Java's HashMap keeps its table across a remove, so a prune that
            // crosses a table boundary changes the order `toString` walks.
            if !keys_to_remove.is_empty()
                && !marks.through_list
                && crate::painless_helpers::java_table_size(built)
                    != crate::painless_helpers::java_table_size(map.len())
            {
                shrunk.push((marks.path.clone(), built));
            }
            policy.empty_collections && map.is_empty()
        }
        Value::Array(arr) => {
            let mark = marks.path.len();
            let outer = marks.through_list;
            if policy.prune_lists {
                marks.through_list = true;
                let mut index = 0usize;
                arr.retain_mut(|v| {
                    marks.descend(mark, &index.to_string());
                    index += 1;
                    !drop_value(v, policy, marks, shrunk)
                });
            } else {
                for (index, item) in arr.iter_mut().enumerate() {
                    marks.descend(mark, &index.to_string());
                    drop_value(item, policy, marks, shrunk);
                }
            }
            marks.path.truncate(mark);
            marks.through_list = outer;
            policy.empty_collections && arr.is_empty()
        }
        _ => false,
    }
}

/// Convert a Painless `keys_to_snake_case` operation.
///
/// Converts camelCase JSON object keys to `snake_case` recursively.
/// Common in Okta and other pipelines for normalising field names.
pub fn keys_to_snake_case(value: &mut Value, rule: SnakeRule) {
    match value {
        Value::Object(map) => {
            let entries: Vec<(String, Value)> = map
                .iter()
                .map(|(k, v)| {
                    let snake = to_snake_case(k, rule);
                    let mut v = v.clone();
                    keys_to_snake_case(&mut v, rule);
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
                keys_to_snake_case(v, rule);
            }
        }
        _ => {}
    }
}

/// The `convertToSnakeCase` helper the integrations copy between packages.
///
/// It is copied, not shared, so the copies have DIVERGED and the two
/// differences both change the output. `sentinel_one`'s `unified_alert` and
/// `entityanalytics_entra_id` break the word wherever the previous character
/// was not itself uppercase -- digits and dots included, so `cve2021Id` becomes
/// `cve2021_id` -- and DROP a key holding an `@`, which is how Microsoft's
/// `@odata.*` metadata stays out of the document. `jupiter_one`'s breaks only
/// after a lowercase character and keeps every key, so its literal `tag.`-dotted
/// keys stay `tag.account_name` where the other rule writes `tag._account_name`.
/// [`snake_case_apply`] reads which is which off the script.
///
/// Returns a new value; the script assigns the result rather than mutating.
#[must_use]
pub fn camel_map_to_snake(value: &Value, rule: SnakeRule, drop_at_keys: bool) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = serde_json::Map::with_capacity(map.len());
            for (key, inner) in map {
                if drop_at_keys && key.contains('@') {
                    continue;
                }
                out.insert(
                    to_snake_case(key, rule),
                    camel_map_to_snake(inner, rule, drop_at_keys),
                );
            }
            Value::Object(out)
        }
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|item| camel_map_to_snake(item, rule, drop_at_keys))
                .collect(),
        ),
        other => other.clone(),
    }
}

/// Map one field's value through a table written as an if/else-if chain.
///
/// The pipelines write a small lookup this way rather than as params:
///
/// ```text
/// String osType = ctx.zscaler_zia.firewall.device.os.type;
/// if (osType == 'iOS') { ctx.host.os.put('type', 'ios'); }
/// else if (osType == 'Android OS') { ctx.host.os.put('type', 'android'); }
/// ```
///
/// Only branches testing the bound local against a literal are read, so the
/// null-guard preamble those scripts open with is skipped. Both `.put(k, v)`
/// and a plain assignment are recognised as the write.
///
/// One arm: the subject value that selects it, and the literal it writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueArm {
    when: String,
    target: String,
    value: String,
}

impl ValueArm {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        when: impl Into<String>,
        target: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            when: when.into(),
            target: target.into(),
            value: value.into(),
        }
    }
}

/// The whole chain as a table, read off the text once.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralValueMap {
    source: String,
    arms: Vec<ValueArm>,
}

impl LiteralValueMap {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(source: impl Into<String>, arms: Vec<ValueArm>) -> Self {
        Self {
            source: source.into(),
            arms,
        }
    }
}

/// Read the chain into a table, declining a script that yields no arm.
///
/// The arm order is the script's, because the first match wins. Branches that
/// do not test the bound local against a literal are skipped rather than
/// failing the parse -- that is what lets the null-guard preamble through --
/// so "no arm parsed" is the only structural rejection, and it is the one that
/// matters: it drops the recursive map-walkers that merely spell `else if (`
/// and which this pattern claimed for 91 call sites without ever applying.
fn parse_literal_value_map(script: &str) -> Option<LiteralValueMap> {
    let (local, source) = local_and_ctx_path(script)?;

    let mut arms = Vec::new();
    for block in script.split("if (").skip(1) {
        let Some((guard, body)) = block.split_once(") {") else {
            continue;
        };
        let Some(literal) = equality_literal(guard, &local) else {
            continue;
        };
        let Some((target, value)) = branch_write(body) else {
            continue;
        };
        arms.push(ValueArm::new(literal, target, value));
    }

    (!arms.is_empty()).then(|| LiteralValueMap::new(source, arms))
}

/// Write the arm the subject selects, if any.
pub fn literal_value_map(event: &mut Event, pattern: &LiteralValueMap) -> bool {
    // The field is absent, which every one of these scripts is gated on.
    let Some(subject) = event.get_as_string(&pattern.source) else {
        return false;
    };
    let Some(arm) = pattern.arms.iter().find(|arm| arm.when == subject) else {
        return false;
    };
    let _ = event.set(&arm.target, arm.value.clone());
    true
}

/// Every VALUE of a map gathered into one deduped list, lists flattened.
///
/// Sibling of [`KnownPattern::CollectMapValues`], which walks `.keySet()` and
/// reads a named leaf off each entry. This one walks `.values()` and takes the
/// value itself, stepping into a value that is a list -- `ti_abusech` gathers
/// `threat.indicator.file.hash` (md5, sha256, ssdeep, ...) into `related.hash`,
/// and `ssdeep` may itself be a list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FlattenMapInto {
    source: String,
    target: String,
}

fn parse_flatten_map_into(script: &str) -> Option<FlattenMapInto> {
    // `for (def x : <map>.values())`
    let receiver = identifier_before(script.split_once(".values()")?.0)?;
    let source = ctx_path_bound_to(script, receiver)
        .or_else(|| painless_path(script.split_once(".values()")?.0))?;

    // `ctx.<target>.add(<item>)`
    let target = painless_path(script.split_once(".add(")?.0)?;
    (source != target).then_some(FlattenMapInto { source, target })
}

fn run_flatten_map_into(event: &mut Event, pattern: &FlattenMapInto) -> bool {
    let Some(Value::Object(map)) = event.get(&pattern.source).cloned() else {
        // Gated on the map, which the processor's own `if` also checks.
        return true;
    };

    // The script appends to whatever is already there, and `preserve_order`
    // means `values()` walks the document's own order.
    let mut collected: Vec<Value> = match event.get(&pattern.target) {
        Some(Value::Array(items)) => items.clone(),
        _ => Vec::new(),
    };
    let push = |value: &Value, into: &mut Vec<Value>| {
        if !value.is_null() && !into.contains(value) {
            into.push(value.clone());
        }
    };
    for value in map.values() {
        match value {
            Value::Array(items) => {
                for item in items {
                    push(item, &mut collected);
                }
            }
            other => push(other, &mut collected),
        }
    }

    let _ = event.set(&pattern.target, Value::Array(collected));
    true
}

/// Render whole numbers as strings, in place, where they are not already one.
///
/// `ti_anomali` does it to `id` and `update_id` because they arrive from JSON
/// as a Double: `Long.toString((long) ctx.json.id)` is what stops a big id
/// rendering in exponent notation. The `instanceof String` guard means a value
/// already stringified is left alone.
fn parse_stringify_longs(script: &str) -> Option<Vec<String>> {
    let mut fields = Vec::new();
    for statement in script.split([';', '\n']) {
        let Some((lhs, rhs)) = statement.split_once('=') else {
            continue;
        };
        let Some(argument) = rhs.trim().strip_prefix("Long.toString(") else {
            continue;
        };
        // IN PLACE only. A call whose result goes to a LOCAL is building a
        // lookup KEY, not converting the field -- cyberark_epm makes one out
        // of `logon_status_id`, and stringifying that field in place is a type
        // change the vendor never makes.
        let (Some(target), Some(source)) = (painless_path(lhs), painless_path(argument)) else {
            continue;
        };
        if target == source && !fields.contains(&target) {
            fields.push(target);
        }
    }
    (!fields.is_empty()).then_some(fields)
}

fn run_stringify_longs(event: &mut Event, fields: &[String]) -> bool {
    for field in fields {
        // Already a string, or absent: the script's own guard leaves it.
        let Some(number) = event.get(field).and_then(Value::as_i64) else {
            continue;
        };
        let _ = event.set(field, json!(number.to_string()));
    }
    true
}

/// One target filled from a DIFFERENT source per label.
///
/// A ladder over a label field where each arm names its own source and carries
/// its own `!= null` guard: `ti_anomali` picks the indicator's name from
/// `srcip`, `domain`, `url`, `email` or `md5` according to the indicator type
/// it just derived. Distinct from `LiteralValueMap`, whose arms write a
/// LITERAL -- here every arm writes a copy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CopyByLabel {
    /// The field whose value picks the arm.
    subject: String,
    /// Labels and the source each of them copies from, in script order.
    arms: Vec<(Vec<String>, String)>,
    target: String,
}

fn parse_copy_by_label(script: &str) -> Option<CopyByLabel> {
    let (local, subject) = local_and_ctx_path(script)?;

    let mut arms = Vec::new();
    let mut target: Option<String> = None;
    for piece in script.split(" else if (") {
        let (head, body) = piece.split_once(") {")?;
        // The ladder's own guard, not the copy's -- the first arm's piece also
        // carries the binding that precedes it.
        let guard = head.rsplit("if (").next()?;

        let labels: Option<Vec<String>> = guard
            .split("||")
            .map(|clause| {
                let rest = clause.trim().strip_prefix(&local)?.trim_start();
                quoted_first(rest.strip_prefix("==")?)
            })
            .collect();
        let labels = labels?;

        // `if (ctx.<source> != null) ctx.<target> = ctx.<source>;`
        let statement = body.split(';').next()?;
        let at = last_assignment(statement)?;
        let path = painless_path(&statement[..at])?;
        if target.get_or_insert_with(|| path.clone()) != &path {
            return None;
        }
        arms.push((labels, painless_path(&statement[at..])?));
    }

    // One arm is a guarded copy, not a ladder.
    if arms.len() < 2 {
        return None;
    }
    Some(CopyByLabel {
        subject,
        arms,
        target: target?,
    })
}

fn run_copy_by_label(event: &mut Event, pattern: &CopyByLabel) -> bool {
    let Some(label) = event.get_as_string(&pattern.subject) else {
        // Absent, which the processor's own `if` gates on.
        return false;
    };
    for (labels, source) in &pattern.arms {
        if !labels.contains(&label) {
            continue;
        }
        // The arm's own `!= null`: an absent or explicitly null source leaves
        // the target alone rather than clearing it.
        if let Some(value) = event.get(source).filter(|v| !v.is_null()).cloned() {
            let _ = event.set(&pattern.target, value);
        }
        return true;
    }
    true
}

/// A number banded into a label.
///
/// Distinct from [`RangeLadder`] by SPELLING, not by meaning: the guards are
/// written subject-first (`value > 0 && value < 30`) instead of literal-first,
/// the out-of-range band is an `||`, and the label may land in a local that one
/// closing `.put()` writes rather than in the arm itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BandLadder {
    /// Where the number is read from.
    subject: String,
    /// Bands in the script's own order; the first that holds wins.
    arms: Vec<(Band, BandLabel)>,
    /// The label when no band holds -- a local's initialiser, or a trailing
    /// bare `else`. `None` where the script writes nothing, which is what
    /// `ti_anomali`'s threatstream ladder does between its bands.
    default: Option<BandLabel>,
    /// The label for an ABSENT or null subject, where the script spells that
    /// arm (`if (value == null)`). Distinct from `default`: one is "no value",
    /// the other "a value in no band".
    absent: Option<BandLabel>,
    target: String,
}

/// What an arm writes, in the type the vendor wrote it.
///
/// Most ladders name the band -- "Low", "High". `infoblox_threat_defense`'s
/// severity ladder writes the ECS NUMBER instead (`ctx.event.severity = 21`),
/// and a string there scores wrong against Elasticsearch's long.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BandLabel {
    Text(String),
    Number(i64),
}

impl BandLabel {
    fn value(&self) -> Value {
        match self {
            Self::Text(text) => json!(text),
            Self::Number(number) => json!(number),
        }
    }
}

/// The literal an arm writes: a quoted string, or a bare whole number.
///
/// The number is read only where the fragment is NOTHING else, so a copy
/// (`= ctx.x`) or an expression declines the whole ladder rather than writing
/// a literal the vendor never wrote.
fn band_label(text: &str) -> Option<BandLabel> {
    quoted_first(text)
        .map(BandLabel::Text)
        .or_else(|| whole_number(text).map(BandLabel::Number))
}

/// One arm's guard: comparisons over the subject, joined one way.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Band {
    bounds: Vec<Bound>,
    /// `&&` when true, `||` when false. A guard mixing the two is declined.
    all: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bound {
    op: Cmp,
    /// Whole numbers only, so the pattern derives `Eq` and never rounds a
    /// boundary the vendor wrote. Every threshold in the catalogue is one.
    value: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cmp {
    Lt,
    Le,
    Gt,
    Ge,
    /// `ctx.confidence == 0`. A band of one, which `ti_opencti`'s confidence
    /// ladder spells between its null arm and its ranges.
    Eq,
}

impl Band {
    fn holds(&self, value: f64) -> bool {
        let hit = |bound: &Bound| {
            // Thresholds are vendor-written band edges -- 0, 30, 70, 100 --
            // so the i64 is always exact in an f64.
            #[allow(clippy::cast_precision_loss)]
            let limit = bound.value as f64;
            match bound.op {
                Cmp::Lt => value < limit,
                Cmp::Le => value <= limit,
                Cmp::Gt => value > limit,
                Cmp::Ge => value >= limit,
                Cmp::Eq => (value - limit).abs() < f64::EPSILON,
            }
        };
        if self.all {
            self.bounds.iter().all(hit)
        } else {
            self.bounds.iter().any(hit)
        }
    }
}

fn parse_band_ladder(script: &str) -> Option<BandLadder> {
    let (local, subject) = local_and_ctx_path(script).or_else(|| inline_ctx_subject(script))?;

    let mut arms = Vec::new();
    let mut written: Option<String> = None;
    let mut label_local: Option<String> = None;
    let mut absent: Option<BandLabel> = None;

    for block in script.split("if (").skip(1) {
        let (guard, body) = block.split_once(") {")?;
        // A guard that never names the subject belongs to something else --
        // `ti_anomali`'s intelligence ladder opens with two `ctx.x == null`
        // map creations. Skipping them is what lets the ladder be read at all.
        if !mentions(guard, &local) {
            continue;
        }
        // `if (value == null)` is the NO-VALUE arm, not a band.
        if let Some(rest) = guard.trim().strip_prefix(&local)
            && rest.trim().trim_start_matches('=').trim() == "null"
        {
            absent = band_label(body.split(';').next()?.split_once('=')?.1);
            continue;
        }
        let band = parse_band(guard, &local)?;

        // `ctx["x"] = "Label";` or `label = "Label";` -- the second form needs
        // the closing `.put()` to say where it lands.
        let (lhs, rhs) = body.split(';').next()?.split_once('=')?;
        let label = band_label(rhs)?;
        // Every arm has to agree on where it writes, or this is two patterns.
        if let Some(path) = painless_path(lhs) {
            if written.get_or_insert_with(|| path.clone()) != &path {
                return None;
            }
        } else {
            let name = lhs.trim().to_string();
            if label_local.get_or_insert_with(|| name.clone()) != &name {
                return None;
            }
        }
        arms.push((band, label));
    }

    // Two arms is the least that is a ladder rather than a single guard.
    if arms.len() < 2 {
        return None;
    }

    let target = match &label_local {
        // `ctx.threat.indicator.put("confidence", confidence)`
        Some(_) => {
            let (head, rest) = script.rsplit_once(".put(")?;
            format!("{}.{}", painless_path(head)?, quoted_first(rest)?)
        }
        None => written?,
    };

    // The no-band-holds label: a local's initialiser, or a trailing bare
    // `else` whose write lands on the SAME target -- an `else` writing
    // somewhere else is a different statement, not this ladder's fallback.
    let default = label_local
        .and_then(|name| {
            script
                .split_once(&format!(" {name} = "))
                .and_then(|(_, rest)| band_label(rest.split(';').next()?))
        })
        .or_else(|| {
            let (_, tail) = script.rsplit_once(" else {")?;
            let (lhs, rhs) = tail.split(';').next()?.split_once('=')?;
            (painless_path(lhs)? == target).then(|| band_label(rhs))?
        });

    // A NUMERIC ladder has to be the whole script, bar the map it creates
    // first. `ScoreSeverityBands` writes `event.risk_score` beside the same
    // bands, and claiming that script here would drop the field. Label ladders
    // are not audited, so their reading is unchanged.
    let numeric = arms
        .iter()
        .map(|(_, label)| label)
        .chain(default.iter())
        .chain(absent.iter())
        .any(|label| matches!(label, BandLabel::Number(_)));
    if numeric && !ladder_is_the_whole_script(script, &target) {
        return None;
    }

    Some(BandLadder {
        subject,
        arms,
        default,
        absent,
        target,
    })
}

/// Whether the ladder's target is the only field the script writes.
///
/// A container the ladder creates to write into (`ctx.event = ctx.event ?:
/// [:]`) does not count against it -- that is the ladder's own prelude, not
/// other work.
fn ladder_is_the_whole_script(script: &str, target: &str) -> bool {
    crate::painless_params::ctx_writes(script)
        .iter()
        .all(|(path, rhs)| path == target || creates_container(rhs))
}

/// `ctx.<path> = ctx.<path> ?: [:]` and its spellings.
fn creates_container(rhs: &str) -> bool {
    let rhs = rhs.trim().trim_end_matches(';').trim();
    rhs.ends_with("[:]") || rhs.ends_with("new HashMap()") || rhs.ends_with("new TreeMap()")
}

/// Whether `token` appears in `text` as a whole identifier.
///
/// A substring test would read `ctx.myvalue` as naming the local `value`, and
/// then skip an arm that is really part of the ladder.
fn mentions(text: &str, token: &str) -> bool {
    let identifier = |c: char| c.is_alphanumeric() || c == '_';
    text.match_indices(token).any(|(at, _)| {
        let before = text[..at]
            .chars()
            .next_back()
            .is_none_or(|c| !identifier(c));
        let after = text[at + token.len()..]
            .chars()
            .next()
            .is_none_or(|c| !identifier(c));
        before && after
    })
}

/// The first `def x = ctx.<path>` binding, as (local, path).
///
/// Splits on STATEMENTS, not lines: these scripts ship as a YAML folded
/// scalar, so the whole preamble arrives on one line and a line-based read
/// takes the rest of the script as the path. `String x = ctx.y` reads the same
/// way, since both spellings are a two-word declaration.
pub(crate) fn local_and_ctx_path(script: &str) -> Option<(String, String)> {
    use crate::painless_params::clean_path;

    script.split([';', '\n']).find_map(|statement| {
        let (declaration, value) = statement.trim().split_once(" = ")?;
        let path = value.trim().strip_prefix("ctx.")?;
        // A brace closing the PREVIOUS block sits on this statement once the
        // script is folded (`} def value = ctx.x`), and it is punctuation
        // rather than part of the declaration.
        let declaration = declaration.trim_matches(['{', '}', ' ']);
        let name = declaration.rsplit(' ').next()?;
        (declaration.split(' ').count() == 2 && !name.is_empty())
            .then(|| (name.to_string(), clean_path(path.trim())))
    })
}

/// The ctx path a ladder tests when it binds no local, taken from its guards.
///
/// `local_and_ctx_path` reads `long severity = ctx.x;` and most ladders write
/// one. `ti_opencti`'s confidence ladder does not: it names `ctx.confidence` in
/// every guard directly, so with no local there is nothing for the reader to
/// key on and the whole ladder declines.
///
/// The path has to appear in at least TWO guards. One guard naming a ctx path
/// is an ordinary conditional, not a ladder over that path.
fn inline_ctx_subject(script: &str) -> Option<(String, String)> {
    use crate::painless_params::clean_path;

    let mut counts: Vec<(String, usize)> = Vec::new();
    for block in script.split("if (").skip(1) {
        let Some((guard, _)) = block.split_once(") {") else {
            continue;
        };
        let mut seen: Vec<&str> = Vec::new();
        let mut rest = guard;
        while let Some(at) = rest.find("ctx.") {
            let tail = &rest[at..];
            let end = tail
                .find(|c: char| !c.is_alphanumeric() && !"._?".contains(c))
                .unwrap_or(tail.len());
            let token = tail[..end].trim_end_matches('.');
            if !token.is_empty() && !seen.contains(&token) {
                seen.push(token);
            }
            rest = &tail[end.max(1)..];
        }
        // Once per guard, so a path named twice in one band does not outvote a
        // path named once in each of two.
        for token in seen {
            match counts.iter_mut().find(|(held, _)| held == token) {
                Some((_, count)) => *count += 1,
                None => counts.push((token.to_owned(), 1)),
            }
        }
    }
    let (token, count) = counts.into_iter().max_by_key(|(_, count)| *count)?;
    (count >= 2).then(|| {
        let path = clean_path(token.trim_start_matches("ctx."));
        (token, path)
    })
}

/// One guard, as comparisons over `local` joined by `&&` or `||`.
fn parse_band(guard: &str, local: &str) -> Option<Band> {
    let all = !guard.contains("||");
    if !all && guard.contains("&&") {
        // A mixed guard is a different pattern; reading it as one or the other
        // would take the wrong arm.
        return None;
    }
    let bounds: Option<Vec<Bound>> = guard
        .split(if all { "&&" } else { "||" })
        .map(|clause| parse_bound(clause, local))
        .collect();
    let bounds = bounds?;
    (!bounds.is_empty()).then_some(Band { bounds, all })
}

impl Cmp {
    /// The same comparison read from the other side, for a clause the vendor
    /// wrote literal-first (`100.0 < value` is `value > 100.0`).
    const fn mirrored(self) -> Self {
        match self {
            Self::Lt => Self::Gt,
            Self::Le => Self::Ge,
            Self::Gt => Self::Lt,
            Self::Ge => Self::Le,
            // Equality reads the same from either side.
            Self::Eq => Self::Eq,
        }
    }
}

fn parse_bound(clause: &str, local: &str) -> Option<Bound> {
    // Two-character operators first, or `>=` reads as `>` and the band shifts
    // by one.
    const OPERATORS: [(&str, Cmp); 5] = [
        ("<=", Cmp::Le),
        (">=", Cmp::Ge),
        ("==", Cmp::Eq),
        ("<", Cmp::Lt),
        (">", Cmp::Gt),
    ];

    let clause = clause.trim();
    let Some(rest) = clause.strip_prefix(local) else {
        // Literal-first: `100.0 < value`. ti_anomali's intelligence ladder
        // spells one bound each way in the same script.
        let (head, tail) = OPERATORS.into_iter().find_map(|(token, op)| {
            let (head, tail) = clause.split_once(token)?;
            (tail.trim() == local).then_some((head, op))
        })?;
        return Some(Bound {
            op: tail.mirrored(),
            value: whole_number(head)?,
        });
    };
    let rest = rest.trim_start();
    let (op, number) = OPERATORS
        .into_iter()
        .find_map(|(token, op)| Some((op, rest.strip_prefix(token)?)))?;
    Some(Bound {
        op,
        value: whole_number(number)?,
    })
}

/// A threshold written `70`, `70.0` or `70L`. A genuinely fractional one
/// declines the whole pattern rather than rounding a boundary the vendor wrote.
fn whole_number(text: &str) -> Option<i64> {
    let text = text.trim().trim_end_matches(['L', 'l', 'f', 'F', 'd', 'D']);
    let Some((whole, fraction)) = text.split_once('.') else {
        return text.parse().ok();
    };
    if !fraction.bytes().all(|b| b == b'0') {
        return None;
    }
    whole.parse().ok()
}

fn run_band_ladder(event: &mut Event, pattern: &BandLadder) -> bool {
    // A grok leaves a numeric field as a string and the `convert` may not have
    // run yet, so read it either way.
    let Some(value) = event
        .get_f64(&pattern.subject)
        .or_else(|| event.get_as_string(&pattern.subject)?.trim().parse().ok())
    else {
        // The script's own `== null` arm, where it spells one. Without one the
        // processor's `if` gates on the field and this declines.
        return match &pattern.absent {
            Some(label) => {
                let _ = event.set(&pattern.target, label.value());
                true
            }
            None => false,
        };
    };

    let label = pattern
        .arms
        .iter()
        .find(|(band, _)| band.holds(value))
        .map(|(_, label)| label.clone())
        .or_else(|| pattern.default.clone());

    if let Some(label) = label {
        let _ = event.set(&pattern.target, label.value());
    }
    true
}

/// An expiry timestamp: a base plus a duration whose LAST CHARACTER is a unit.
///
/// Every threat-intel package computes one. The unit switch is the pattern --
/// three adders selected by one character, with a days default and an
/// `error.message` for a unit the vendor does not know -- and the field names
/// around it are incidental: `ti_abusech` writes six streams' worth against
/// `threat.indicator.last_seen`, `ti_anomali` one against `json.added_at`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct IocExpiry {
    /// Where the duration string is read from.
    duration: String,
    /// The literal used when that field is absent, where the script names one.
    /// `ti_anomali` names none and its processor `if:` guarantees the field.
    default_duration: Option<String>,
    /// Base timestamps in the order the script tries them.
    bases: Vec<String>,
    /// Days added when the unit is not one the script knows.
    default_days: i64,
    /// Seconds taken back off the result. The `labels.interval` streams expire
    /// an indicator just BEFORE the next interval so the transform's retention
    /// window cannot miss it.
    settle_seconds: i64,
    /// The message that same branch appends to `error.message`.
    invalid_message: Option<String>,
    target: String,
}

fn parse_ioc_expiry(script: &str) -> Option<IocExpiry> {
    let at = last_assignment(script)?;
    let target = painless_path(&script[..at])?;

    // The base is whatever the adder is called ON, which is the one reading
    // that survives both spellings: `ZonedDateTime.parse(ctx.x)` where the
    // vendor holds a string, a bare `= ctx.x` where it holds a date already.
    let receiver = identifier_before(script.split_once(".plusDays(")?.0)?;
    let mut bases: Vec<String> = Vec::new();
    for segment in script.split(&format!("{receiver} = ")).skip(1) {
        // A script binds the same local in both arms of an if/else, and the
        // order it does so is its preference order.
        let expression = segment.split([';', '\n']).next().unwrap_or_default();
        if let Some(path) = painless_path(expression)
            && !bases.contains(&path)
        {
            bases.push(path);
        }
    }
    if bases.is_empty() {
        return None;
    }

    // The duration is whatever `.length()` is called on -- both spellings take
    // the last character off it, one with `substring` and one with `charAt`.
    let local = identifier_before(script.split_once(".length()")?.0)?;
    let binding = script.split_once(&format!(" {local} = "))?.1;
    let expression = binding.split_once(';').map_or(binding, |(head, _)| head);

    Some(IocExpiry {
        duration: painless_path(expression)?,
        // The ternary's else arm, which is the version-upgrade default.
        default_duration: expression
            .rsplit_once(':')
            .and_then(|(_, tail)| quoted_first(tail)),
        bases,
        // The one `plusDays` whose argument is a literal rather than the
        // parsed duration: the arm for a unit the script does not know.
        default_days: script.split("plusDays(").skip(1).find_map(|segment| {
            segment
                .split(')')
                .next()?
                .trim()
                .trim_end_matches(['L', 'l'])
                .parse::<i64>()
                .ok()
        })?,
        settle_seconds: subtracted(script, ".minusMinutes(") * 60
            + subtracted(script, ".minusSeconds("),
        invalid_message: quoted_after(script, "message.add(").into_iter().next(),
        target,
    })
}

/// The same expiry window, from an EPOCH base and with a decayed flag.
///
/// `ti_misp`'s spelling of what [`IocExpiry`] does, and it declines there for
/// a precise reason: `parse_ioc_expiry` reads its bases off what the adder's
/// receiver is assigned, and this script binds that receiver to LOCALS
/// (`_tmp_timestamp`, `_tmp_last_seen`) rather than to a `ctx` path, so the
/// base list comes back empty. Three further differences make it its own
/// pattern rather than a widening: the first base is epoch SECONDS through
/// `Instant.ofEpochMilli`, the two bases are combined by taking the LATER
/// rather than the first present, and the script also writes a BOOLEAN saying
/// whether the expiry has already passed at ingest time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DecayWindow {
    /// Where the duration string is read from.
    duration: String,
    /// The epoch-SECONDS base.
    seconds: String,
    /// The ISO base that wins when it is the later of the two.
    later: String,
    /// Days added when the unit is not one the script knows.
    default_days: i64,
    /// The message that same branch appends to `error.message`.
    invalid_message: Option<String>,
    /// Where the expiry goes.
    target: String,
    /// The flag, and the ingest time it is compared against.
    flag: String,
    ingested: String,
}

fn parse_decay_window(script: &str) -> Option<DecayWindow> {
    use crate::painless_params::clean_path;

    let duration = script.split_once("= ctx.")?.1.split_once(';')?.0;
    let seconds = script.split_once(" ts = ctx.")?.1.split_once(';')?.0;
    let later = script
        .split_once("ZonedDateTime.parse(ctx.")?
        .1
        .split_once(')')?
        .0;

    // The local the adders write into, which is what names both remaining
    // statements: the expiry assignment and the flag that reads it.
    let receiver = identifier_before(script.split_once(".plusDays(")?.0)?;
    let assigned = identifier_before(script.split_once(&format!(" = {receiver}.plusDays("))?.0)?;
    let target = painless_path(script.split_once(&format!(" = {assigned};"))?.0)?;

    let (head, tail) = script.split_once(&format!(" = {assigned}.isBefore("))?;
    let flag = painless_path(head)?;
    let ingested = tail
        .split_once("ZonedDateTime.parse(ctx.")?
        .1
        .split_once(')')?
        .0;

    let ok = |raw: &str| {
        !raw.is_empty()
            && raw
                .chars()
                .all(|c| c.is_alphanumeric() || "._?".contains(c))
    };
    if !ok(duration) || !ok(seconds) || !ok(later) || !ok(ingested) {
        return None;
    }

    Some(DecayWindow {
        duration: clean_path(duration),
        seconds: clean_path(seconds),
        later: clean_path(later),
        // The one `plusDays` whose argument is a literal: the arm for a unit
        // the script does not know.
        default_days: script.split("plusDays(").skip(1).find_map(|segment| {
            segment
                .split(')')
                .next()?
                .trim()
                .trim_end_matches(['L', 'l'])
                .parse::<i64>()
                .ok()
        })?,
        invalid_message: quoted_after(script, "message.add(").into_iter().next(),
        target,
        flag,
        ingested: clean_path(ingested),
    })
}

/// Expire the indicator, and say whether it has already expired.
fn run_decay_window(event: &mut Event, pattern: &DecayWindow) -> bool {
    let Some(duration) = event
        .get_as_string(&pattern.duration)
        .filter(|configured| !configured.is_empty())
    else {
        return true;
    };
    let Some(seconds) = event.get_as_i64(&pattern.seconds) else {
        return true;
    };
    let Some(mut base) = crate::date_formats::epoch_seconds_to_iso8601(seconds) else {
        return true;
    };

    // `isBefore` picks the LATER of the two, which is a different rule from
    // IocExpiry's first-present ladder: a re-sighting moves the window out.
    if let Some(seen) = event.get_as_string(&pattern.later)
        && crate::date_formats::iso8601_is_before(&base, &seen) == Some(true)
    {
        base = seen;
    }

    let mut value = duration.chars();
    let unit = value.next_back().unwrap_or_default();
    let (unit, count) = if matches!(unit, 'd' | 'h' | 'm') {
        let Ok(count) = value.as_str().parse::<i64>() else {
            // `Long.parseLong` throws, so the document fails and the vendor
            // writes nothing here either.
            return true;
        };
        (unit, count)
    } else {
        if let Some(message) = &pattern.invalid_message {
            let _ = event.append("error.message", json!(message));
        }
        ('d', pattern.default_days)
    };

    let Some(expiry) = crate::date_formats::iso8601_plus(&base, unit, count, 0) else {
        return true;
    };
    let _ = event.set(&pattern.target, json!(expiry.clone()));
    if let Some(ingested) = event.get_as_string(&pattern.ingested)
        && let Some(decayed) = crate::date_formats::iso8601_is_before(&expiry, &ingested)
    {
        let _ = event.set(&pattern.flag, json!(decayed));
    }
    true
}

/// The identifier a fragment ends with, which is the receiver of the call that
/// follows it.
fn identifier_before(fragment: &str) -> Option<&str> {
    fragment
        .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()
        .filter(|name| !name.is_empty())
}

/// What one `minusX(` takes off, whether the count is written there or held in
/// a local the script bound to a literal.
fn subtracted(script: &str, method: &str) -> i64 {
    let Some(argument) = script
        .split_once(method)
        .and_then(|(_, rest)| rest.split(')').next())
        .map(str::trim)
    else {
        return 0;
    };
    let literal = |text: &str| text.trim().trim_end_matches(['L', 'l']).parse::<i64>().ok();
    literal(argument)
        .or_else(|| {
            let binding = script.split_once(&format!(" {argument} = "))?.1;
            literal(binding.split([';', '\n']).next()?)
        })
        .unwrap_or_default()
}

fn run_ioc_expiry(event: &mut Event, pattern: &IocExpiry) -> bool {
    let Some(duration) = event
        .get_as_string(&pattern.duration)
        .filter(|configured| !configured.is_empty())
        .or_else(|| pattern.default_duration.clone())
    else {
        // No duration and no literal default. `ti_anomali` wraps its whole body
        // in `if (dur instanceof String)`, so that writes nothing.
        return true;
    };
    let Some(base) = pattern
        .bases
        .iter()
        .find_map(|path| event.get_as_string(path))
    else {
        return true;
    };

    let mut value = duration.chars();
    let unit = value.next_back().unwrap_or_default();
    let (unit, count) = if matches!(unit, 'd' | 'h' | 'm') {
        let Ok(count) = value.as_str().parse::<i64>() else {
            // `Long.parseLong` throws, so the document fails and the vendor
            // writes nothing here either.
            return true;
        };
        (unit, count)
    } else {
        if let Some(message) = &pattern.invalid_message {
            let _ = event.append("error.message", json!(message));
        }
        ('d', pattern.default_days)
    };

    if let Some(expiry) =
        crate::date_formats::iso8601_plus(&base, unit, count, pattern.settle_seconds)
    {
        let _ = event.set(&pattern.target, json!(expiry));
    }
    true
}

/// The literal a guard compares `local` to, whichever quote it used.
fn equality_literal(guard: &str, local: &str) -> Option<String> {
    let rest = guard.trim().strip_prefix(local)?.trim_start();
    let rest = rest.strip_prefix("==")?.trim();
    let quote = rest.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    let rest = &rest[quote.len_utf8()..];
    let end = rest.find(quote)?;
    // A compound guard is a different pattern, not this one.
    rest[end + quote.len_utf8()..]
        .trim()
        .is_empty()
        .then(|| rest[..end].to_string())
}

/// The field a branch writes and the literal it writes there.
///
/// `ctx.host.os.put('type', 'ios')` and `ctx.host.os.type = 'ios'` are the
/// same write spelled two ways.
fn branch_write(body: &str) -> Option<(String, String)> {
    let statement = body.split("ctx.").nth(1)?;
    if let Some((path, arguments)) = statement.split_once(".put(") {
        let (key, rest) = leading_literal(arguments)?;
        let (value, _) = leading_literal(rest.trim_start().strip_prefix(',')?)?;
        return Some((format!("{}.{key}", clean_path(path)), value));
    }
    let (path, value) = statement.split_once(" = ")?;
    let (value, _) = leading_literal(value)?;
    Some((clean_path(path), value))
}

/// A leading single- or double-quoted literal, and what follows it.
fn leading_literal(text: &str) -> Option<(String, &str)> {
    let text = text.trim_start();
    let quote = text.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    let rest = &text[quote.len_utf8()..];
    let end = rest.find(quote)?;
    Some((rest[..end].to_string(), &rest[end + quote.len_utf8()..]))
}

/// Write the basename of one or more path fields.
///
/// The pattern is a helper that finds the last separator and returns what
/// follows it, then one paragraph per field:
///
/// ```text
/// def getProcessName(def path) {
///   def idx = path.lastIndexOf("\");
///   if (idx > -1) { return path.substring(idx+1); }
///   return "";
/// }
/// def cmd = ctx.process?.executable;
/// if (cmd != null && cmd != "" && ctx.process?.name == null) {
///   def name = getProcessName(cmd);
///   if (name != "") { ctx.process.name = name; }
/// }
/// ```
///
/// The separator, the source and the target are all read off the script. The
/// helper's own name is not: the call is found by its ARGUMENT being a local
/// bound to a `ctx.` path, so a pipeline spelling it `basename` matches too.
///
/// Returns false when nothing parses, which lets a script that merely spells
/// `lastIndexOf` fall through to the matchers below.
/// Every basename cut one script makes, and the separator they share.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasenameCuts {
    separator: char,
    /// Source field to target field, one per local the helper is called with.
    cuts: Vec<(String, String)>,
}

/// Read the cuts out of the script text, or decline.
fn parse_basename_cuts(script: &str) -> Option<BasenameCuts> {
    let separator = last_index_of_separator(script)?;

    // `def cmd = ctx.process?.executable;` -- the locals the helper is called
    // with, and the field each one reads.
    let mut locals: Vec<(String, String)> = Vec::new();
    for line in script.lines() {
        let line = line.trim().trim_end_matches(';');
        let Some(rest) = line.strip_prefix("def ") else {
            continue;
        };
        let Some((name, value)) = rest.split_once(" = ") else {
            continue;
        };
        if let Some(path) = value.trim().strip_prefix("ctx.") {
            locals.push((name.trim().to_string(), clean_path(path)));
        }
    }

    // The call is `(<local>)` with nothing else in the parentheses. The
    // helper's DEFINITION takes `def path`, which is not one of the locals,
    // so it is skipped for free -- and so is `(cmd != null && ...)`, which
    // names the local but is not a call.
    let mut cuts = Vec::new();
    for (local, source) in &locals {
        let Some(at) = script.find(&format!("({local})")) else {
            continue;
        };
        let Some(target) = script[at..]
            .split("ctx.")
            .nth(1)
            .and_then(|after| after.split_once(" = "))
            .map(|(path, _)| clean_path(path))
        else {
            continue;
        };
        cuts.push((source.clone(), target));
    }

    (!cuts.is_empty()).then_some(BasenameCuts { separator, cuts })
}

/// Write each cut, leaving a source with no separator alone.
pub fn basename_cuts(event: &mut Event, pattern: &BasenameCuts) -> bool {
    for (source, target) in &pattern.cuts {
        let Some(text) = event.get_str(source) else {
            continue;
        };
        // Elastic writes nothing when the path holds no separator, and
        // nothing when the basename is empty -- a trailing separator.
        if let Some((_, base)) = text.rsplit_once(pattern.separator)
            && !base.is_empty()
        {
            let base = base.to_string();
            let _ = event.set(target, base);
        }
    }
    true
}

/// The separator a `lastIndexOf` in this script looks for.
fn last_index_of_separator(script: &str) -> Option<char> {
    let after = script.split("lastIndexOf(\"").nth(1)?;
    after.chars().next().filter(|c| *c != '"')
}

/// Fan a DNS answer set out into the ECS lists it feeds.
///
/// An address record is a resolved ip and a related ip; a CNAME is a related
/// host; an MX is a preference and a host, so the host is the SECOND
/// space-separated token. Each list is appended to without duplicates.
fn try_related_from_dns_answers(event: &mut Event) -> bool {
    let Some(Value::Array(answers)) = event.get("dns.answers").cloned() else {
        return false;
    };

    let mut ips: Vec<Value> = Vec::new();
    let mut hosts: Vec<Value> = Vec::new();
    for answer in &answers {
        let (Some(kind), Some(data)) = (
            answer.pointer("/type").and_then(Value::as_str),
            answer.pointer("/data").and_then(Value::as_str),
        ) else {
            continue;
        };
        match kind {
            "A" | "AAAA" => ips.push(Value::from(data)),
            "CNAME" => hosts.push(Value::from(data)),
            "MX" => {
                if let Some((_, host)) = data.split_once(' ') {
                    hosts.push(Value::from(host));
                }
            }
            _ => {}
        }
    }

    for ip in ips {
        let _ = event.append_unique("related.ip", ip.clone());
        let _ = event.append_unique("dns.resolved_ip", ip);
    }
    for host in hosts {
        let _ = event.append_unique("related.hosts", host);
    }
    true
}

/// Sort each member of a list into the ECS path its own text earns.
///
/// gcp's audit pipeline classifies principals by prefix and substring: one
/// opening `serviceAccount:` is a service, one holding `@` is a user, one
/// holding `/instances` is a host, and the rest fall to a catch-all. The
/// ladders, the tests and the destinations all come off the script.
///
/// One pass per `if (ctx.<path> instanceof List)` block, so the actor list
/// and the target list are read separately with their own ladders.
fn try_classify_members(event: &mut Event, script: &str) -> bool {
    let mut classified = false;
    for (at, _) in script.match_indices("instanceof List") {
        // The list is named just before the test; the ladder is the braced
        // body just after the `) {` that closes it.
        let Some(source) = painless_path(&script[..at]) else {
            continue;
        };
        let Some(after) = script[at..].split_once(") {").map(|(_, tail)| tail) else {
            continue;
        };
        let Some((body, _)) = crate::painless_params::balanced(&format!("{{{after}"), '{', '}')
            .map(|(body, rest)| (body.to_string(), rest.to_string()))
        else {
            continue;
        };
        let Some(Value::Array(members)) = event.get(&source).cloned() else {
            continue;
        };

        for member in &members {
            let Some(text) = member.as_str() else {
                continue;
            };
            if let Some(target) = classify(&body, text) {
                let _ = event.append_unique(&target, Value::from(text));
                classified = true;
            }
        }
    }
    classified
}

/// The destination the first holding arm of a ladder names.
fn classify(ladder: &str, member: &str) -> Option<String> {
    for arm in ladder.split("if (").skip(1) {
        let Some((test, tail)) = arm.split_once(") {") else {
            continue;
        };
        if !string_predicate(test, member) {
            continue;
        }
        return destination(tail);
    }
    // The trailing `else { addNestedValue(ctx, "entity.id", actor); }`.
    let (_, last) = ladder.rsplit_once("else {")?;
    destination(last)
}

/// The path an arm's own block names, bounded by that block's braces so a
/// later arm's destination cannot be read instead.
fn destination(tail: &str) -> Option<String> {
    let braced = format!("{{{tail}");
    let (block, _) = crate::painless_params::balanced(&braced, '{', '}')?;
    let (_, arguments) = block.split_once("addNestedValue(")?;
    quoted_first(arguments)
}

/// One arm's test, evaluated against the member rather than the event.
///
/// The grammar is what these ladders use: `||`, `&&`, `!`, `startsWith` and
/// `contains`, each over a quoted literal.
fn string_predicate(test: &str, member: &str) -> bool {
    test.split("||").any(|conjunction| {
        conjunction
            .split("&&")
            .all(|term| string_term(term.trim(), member))
    })
}

fn string_term(term: &str, member: &str) -> bool {
    // A grouping paren is not part of the term, and there may be several.
    let term = term.trim().trim_start_matches('(').trim();
    if let Some(inner) = term.strip_prefix('!') {
        return !string_term(inner, member);
    }
    let Some((call, argument)) = term.split_once('(') else {
        return false;
    };
    let Some(wanted) = quoted_first(argument) else {
        return false;
    };
    if call.ends_with(".startsWith") {
        member.starts_with(&wanted)
    } else if call.ends_with(".contains") {
        member.contains(&wanted)
    } else {
        false
    }
}

/// Keep a rendered copy of a nested object beside the object itself.
///
/// aws's cloudtrail writes `requestParameters`, `responseElements` and
/// `additionalEventData` twice: once as Java's `toString` under
/// `aws.cloudtrail.<name>`, and once whole under `aws.cloudtrail.flattened
/// .<name>` when the deployment asked to keep the duplicate.
///
/// Two scripts run back to back and both come through here. The first only
/// decides the flag, which is a plain read of `_conf.retain`; the second does
/// the copying, and the pairs it copies are read off its own text.
fn try_flattened_duplicates(event: &mut Event, script: &str) -> bool {
    // `ctx._conf.keep_flattened_duplicates = ctx._conf.retain == null || ...`
    if script.contains("keep_flattened_duplicates = ") {
        let retain = event.get_str("_conf.retain").map(str::to_string);
        let keep = retain.is_none_or(|value| {
            ["all", "flattened", "minimal"]
                .iter()
                .any(|wanted| value.contains(wanted))
        });
        let _ = event.set("_conf.keep_flattened_duplicates", keep);
        return true;
    }

    let keep = event.get("_conf.keep_flattened_duplicates") == Some(&Value::Bool(true));
    // `ctx.aws.cloudtrail.request_parameters = ctx.json.requestParameters.toString();`
    // Only the chunks a `.toString();` FOLLOWS are assignments; `split` hands
    // back a final chunk with no separator after it, and reading that one as
    // an assignment wrote the rendered copy over the flattened one.
    let chunks: Vec<&str> = script.split(".toString();").collect();
    for statement in chunks.iter().rev().skip(1).rev() {
        let Some((assignment, source_expression)) = statement.rsplit_once(" = ") else {
            continue;
        };
        let (Some(target), Some(source)) =
            (painless_path(assignment), painless_path(source_expression))
        else {
            continue;
        };
        let Some(value) = event.get(&source).cloned() else {
            continue;
        };

        // Rendered at the path it was pruned at: an empty-value prune that
        // crossed a bucket-table boundary left the map iterating through the
        // table it was BUILT with, which reorders the members.
        let rendered =
            crate::painless_helpers::java_to_string_sized(&value, &mut source.clone(), &|at| {
                event.map_capacity(at)
            });
        // Elasticsearch's keyword ceiling. Over it the rendered copy is kept
        // and the flattened one is not.
        let short_enough = rendered.len() < 32766;
        let _ = event.set(&target, rendered);
        if keep && short_enough {
            let Some((prefix, name)) = target.rsplit_once('.') else {
                continue;
            };
            let _ = event.set(&format!("{prefix}.flattened.{name}"), value);
        }
    }
    true
}

/// Collect every value a script names into one sorted, unique list.
///
/// gcp's audit pipeline gathers principals, resource names and role bindings
/// into a `TreeSet` and writes it to `related.entity`. The paths are read off
/// the script's own `addValue(...)` calls rather than transcribed, so a
/// vendor adding one is picked up by regenerating.
///
/// Three argument patterns appear, and all three resolve to a value on the
/// event: a `ctx.` path, a local bound to one, and a member of a local -- the
/// loops walk a list and add `i.principalSubject` from each element. A
/// `TreeSet` is sorted and unique, and nothing empty goes in.
fn try_collect_entities(event: &mut Event, script: &str) -> bool {
    // The path immediately before the assignment, not the first `ctx.` in the
    // script -- these open by reading half a dozen other fields.
    let Some(target) = script
        .find(" = entities")
        .map(|at| &script[..at])
        .and_then(painless_path)
    else {
        return false;
    };

    let mut entities: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for call in script.split("addValue(").skip(1) {
        let Some((arguments, _)) = call.split_once(");") else {
            continue;
        };
        let Some((_, expression)) = arguments.split_once(',') else {
            continue;
        };
        collect_entity_values(event, script, expression.trim(), &mut entities);
    }

    if !entities.is_empty() {
        let list: Vec<Value> = entities.into_iter().map(Value::from).collect();
        let _ = event.set(&target, Value::Array(list));
    }
    true
}

/// Every non-empty string an `addValue` argument resolves to.
fn collect_entity_values(
    event: &Event,
    script: &str,
    expression: &str,
    into: &mut std::collections::BTreeSet<String>,
) {
    let mut push = |value: Option<&Value>| {
        if let Some(text) = value.and_then(Value::as_str)
            && !text.is_empty()
        {
            into.insert(text.to_string());
        }
    };

    if let Some(path) = expression.strip_prefix("ctx.") {
        push(event.get(&clean_path(path)));
        return;
    }

    // `authInfo.principalEmail` and `i.principalSubject`: a local bound to a
    // ctx path, or a loop variable over the list one names.
    let Some((local, member)) = expression.split_once('.') else {
        return;
    };
    let member = clean_path(member);
    if let Some(path) = ctx_path_bound_to(script, local) {
        push(event.get(&format!("{path}.{member}")));
        return;
    }
    let Some(source) = loop_source(script, local) else {
        return;
    };
    let Some(Value::Array(items)) = event.get(&source) else {
        return;
    };
    for item in items {
        push(item.pointer(&format!("/{}", member.replace('.', "/"))));
    }
}

/// The list a `for (def x: <list>)` walks, as a ctx path.
fn loop_source(script: &str, local: &str) -> Option<String> {
    for opener in [
        format!("for (def {local}: "),
        format!("for (def {local} : "),
    ] {
        if let Some(at) = script.find(&opener) {
            let rest = &script[at + opener.len()..];
            let end = rest.find(')')?;
            return rest[..end].trim().strip_prefix("ctx.").map(clean_path);
        }
    }
    None
}

/// Parse Google's DNS `RData` into `dns.answers`.
///
/// One answer per line, five tab-separated columns:
///
/// ```text
/// elastic.co.\t300\tIN\ta\t127.0.0.1
/// ```
///
/// A trailing `...` line means the vendor truncated the list and is dropped.
/// Trailing dots come off the name and the data, and the type is uppercased.
/// A line with fewer than five columns is where Painless throws on the index,
/// so the whole answer set is abandoned rather than half-built.
fn try_dns_rdata_answers(event: &mut Event, script: &str) -> bool {
    let Some(source) = ctx_path_bound_to(script, "rdata") else {
        return false;
    };
    let Some(rdata) = event.get_str(&source) else {
        return false;
    };

    let lines: Vec<&str> = rdata.split('\n').collect();
    let kept = lines.len() - usize::from(rdata.ends_with("..."));
    let mut answers = Vec::with_capacity(kept);
    for line in lines.iter().take(kept) {
        let columns: Vec<&str> = line.split('\t').collect();
        let [name, ttl, class, kind, data] = columns[..] else {
            return true;
        };
        let Ok(ttl) = ttl.parse::<i64>() else {
            return true;
        };
        answers.push(json!({
            "name": name.trim_end_matches('.'),
            "ttl": ttl,
            "class": class,
            "type": kind.to_uppercase(),
            "data": data.trim_end_matches('.'),
        }));
    }

    let _ = event.set("dns.answers", Value::Array(answers));
    true
}

/// Google Public DNS's structured `RData` into `dns.answers`.
///
/// Each member keeps its class and type, parses the ttl STRING to a number,
/// takes `rvalue` as the data with ONE trailing dot stripped, and names the
/// answer from `domainName` only when an rvalue is there. A ttl that will
/// not parse throws in Painless and `ignore_failure` leaves the event
/// untouched, so nothing is written on the first bad one.
fn try_structured_rdata_answers(event: &mut Event) -> bool {
    let Some(Value::Array(members)) = event.get("json.jsonPayload.structuredRdata") else {
        return true;
    };

    let mut answers = Vec::with_capacity(members.len());
    for member in members {
        let mut answer = Map::new();
        if let Some(class) = member.get("class") {
            answer.insert("class".into(), class.clone());
        }
        if let Some(kind) = member.get("type") {
            answer.insert("type".into(), kind.clone());
        }
        if let Some(ttl) = member.get("ttl").and_then(Value::as_str) {
            let Ok(ttl) = ttl.parse::<i64>() else {
                return true;
            };
            answer.insert("ttl".into(), Value::from(ttl));
        }
        if let Some(rvalue) = member.get("rvalue").and_then(Value::as_str) {
            let data = rvalue.strip_suffix('.').unwrap_or(rvalue);
            answer.insert("data".into(), Value::from(data));
            if let Some(name) = member.get("domainName") {
                answer.insert("name".into(), name.clone());
            }
        }
        answers.push(Value::Object(answer));
    }

    let _ = event.set("dns.answers", Value::Array(answers));
    true
}

/// Append one DNS answer per resolved address, typed by its family.
///
/// sysmon's `QueryResults` lists the CNAME chain and the addresses separately,
/// so the addresses reach `dns.resolved_ip` with no record type on them. The
/// pipeline synthesises one: a colon in the address means `AAAA`, anything
/// else `A`. By the time this runs the `::ffff:` wrapping has already been
/// stripped by a `gsub`, so a v4-mapped address is correctly an `A`.
///
/// A null entry is dropped from `dns.resolved_ip` rather than typed -- the
/// `convert` to an ip upstream leaves one behind for an address it rejected.
fn try_answers_from_resolved_ip(event: &mut Event) -> bool {
    let Some(Value::Array(resolved)) = event.get("dns.resolved_ip").cloned() else {
        return false;
    };

    let mut kept = Vec::with_capacity(resolved.len());
    let mut answers = match event.get("dns.answers") {
        Some(Value::Array(existing)) => existing.clone(),
        _ => Vec::new(),
    };

    for ip in resolved {
        let Some(text) = ip.as_str() else {
            continue;
        };
        let kind = if text.contains(':') { "AAAA" } else { "A" };
        answers.push(serde_json::json!({"type": kind, "data": text}));
        kept.push(ip);
    }

    let _ = event.set("dns.resolved_ip", Value::Array(kept));
    let _ = event.set("dns.answers", Value::Array(answers));
    true
}

/// The `ctx.<target> = <fn>(ctx.<source>)` line the converter is applied by,
/// plus the two ways the copies of the helper disagree.
///
/// The spellings differ in whether the target is the source: `entra_id`
/// rewrites its own object in place, `sentinel_one` and `jupiter_one` write the
/// converted `json` somewhere new. Both are one assignment, so one reader
/// covers them. `jupiter_one` then REMOVES the source, and reading that off the
/// script is what keeps its whole `json` tree out of the document.
fn snake_case_apply(script: &str) -> Option<KnownPattern> {
    // Which word-break rule the copied helper implements. `lastCharWasUpperCase`
    // is cleared by any non-uppercase character, a dot included; the other guard
    // asks about the previous character directly and breaks only after a
    // lowercase one. See [`camel_map_to_snake`].
    // The helper must actually detect CASE. One source converts kebab-case
    // with a bare `str.replace("-", "_")` and names it `convertToSnakeCase`
    // too, so keying on the call alone claims a script this runner cannot
    // apply -- bound, never run, and invisible everywhere but the reach count.
    if !script.contains("Character.isUpperCase(") {
        return None;
    }

    let rule = if script.contains("Character.isLowerCase(str.charAt(i - 1))") {
        SnakeRule::OnWordBreak
    } else {
        SnakeRule::AfterNonUpper
    };
    let drop_at_keys = script.contains(".contains(\"@\")") || script.contains(".contains('@')");

    for line in script.lines().rev() {
        let line = line.trim().trim_end_matches(';');

        // Two spellings of the same hoist. An ASSIGNMENT replaces the target;
        // a `putAll` MERGES into whatever the script's own `?: [:]` left
        // there. qualys_gav writes the second, and reading only the first left
        // its whole payload under `json.*` -- 525 extra fields and 606
        // missing on every event.
        // The assignment form is tried FIRST so this is strictly additive:
        // every script that bound before binds the same way, and `putAll` only
        // catches what used to fall through.
        let (target, rhs, merge) = match line.split_once(" = ") {
            Some((target, rhs)) => (target, rhs, false),
            None => match line.split_once(".putAll(") {
                Some((target, rhs)) => (target, rhs, true),
                None => continue,
            },
        };

        let target = target.trim();
        if !target.starts_with("ctx.") {
            continue;
        }
        // `convertToSnakeCase(ctx.json)` -- the helper's name is not fixed, so
        // the pattern of the call is what identifies it.
        let Some((_, argument)) = rhs.trim().split_once("SnakeCase(") else {
            continue;
        };
        let argument = argument.trim_end_matches(')').trim();
        if !argument.starts_with("ctx.") {
            continue;
        }
        return Some(KnownPattern::CamelToSnake {
            target: target[4..].to_string(),
            source: argument[4..].to_string(),
            rule,
            drop_at_keys,
            merge,
            removes: ctx_removes(script),
        });
    }
    None
}

/// Every top-level `ctx.remove('<field>')` the script makes, in order.
fn ctx_removes(script: &str) -> Vec<String> {
    script
        .match_indices("ctx.remove(")
        .filter_map(|(at, marker)| {
            let rest = &script[at + marker.len()..];
            let (name, _) = rest.split_once(')')?;
            let name = name.trim().trim_matches(['\'', '"']);
            (!name.is_empty()).then(|| clean_path(name))
        })
        .collect()
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
fn sum_of_directions(script: &str) -> Vec<&'static str> {
    // EVERY unit the script names, not the first. fortinet_fortiproxy and
    // arista_ngfw both sum bytes and packets in ONE script, and returning on
    // the first left `network.packets` unwritten on every event of each.
    ["bytes", "packets"]
        .into_iter()
        .filter(|unit| {
            script.contains(&format!("ctx.network.{unit}"))
                && script.contains(&format!("ctx.source.{unit}"))
                && script.contains(&format!("ctx.destination.{unit}"))
        })
        .collect()
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

/// The same sum for every unit a script names.
///
/// Each unit is skipped independently: a script that sums both bytes and
/// packets still writes the bytes total when only the packet operands are
/// missing.
pub fn sum_directions(event: &mut Event, units: &[&str]) -> bool {
    for unit in units {
        try_sum_directions(event, unit);
    }
    true
}

/// `ctx.<target> = ctx.<left> <+|-> ctx.<right>`, whatever the three are
/// called, with an optional literal divisor on either operand.
///
/// The directional-bytes matcher above only knows `source`/`destination` into
/// `network`, and fortinet sums `rcvddelta` and `sentdelta` into `deltabytes`.
/// Reading all three names out of the script covers both and whatever comes
/// next.
///
/// The divisor and the subtraction are endace, which shifts an epoch by half a
/// view window at both ends -- `ctx._conf.event.start = ctx._conf.event.start -
/// ctx._conf.timedelta/2`. Reading the divided operand as a bare path made
/// `_conf.timedelta/2` the field name, which no event carries, so the matcher
/// claimed the script and wrote nothing.
///
/// Skips when either side is absent or non-numeric: Elastic's script would
/// throw, and its `if` gates on both being a Number. The arithmetic saturates,
/// since both operands came off the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombineFields {
    target: String,
    op: CombineOp,
    left: CombineOperand,
    right: CombineOperand,
}

/// Which way round the two operands go.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombineOp {
    Add,
    Subtract,
}

/// One operand: a field path, and the literal it is divided by, if any.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CombineOperand {
    path: String,
    divisor: Option<i64>,
}

impl CombineOperand {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(path: impl Into<String>, divisor: Option<i64>) -> Self {
        Self {
            path: path.into(),
            divisor,
        }
    }
}

impl CombineFields {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        target: impl Into<String>,
        op: CombineOp,
        left: CombineOperand,
        right: CombineOperand,
    ) -> Self {
        Self {
            target: target.into(),
            op,
            left,
            right,
        }
    }
}

/// One operand of the combination: `ctx.<path>`, optionally `/ <literal>`.
///
/// Declines anything else, which is what keeps the matcher off a script it
/// cannot read: a method call, a literal, or a second operator in the same
/// operand all fail the path test rather than becoming a field name.
fn combine_operand(text: &str) -> Option<CombineOperand> {
    use crate::painless_params::clean_path;

    let text = text.trim().strip_prefix("ctx.")?;
    let (path, divisor) = match text.split_once('/') {
        Some((path, divisor)) => {
            let literal = divisor
                .trim()
                .trim_end_matches(['L', 'l', 'd', 'D', 'f', 'F'])
                .trim();
            (path, Some(literal.parse::<i64>().ok().filter(|n| *n != 0)?))
        }
        None => (text, None),
    };

    let path = clean_path(path.trim());
    (!path.is_empty()
        && path
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@')))
    .then_some(CombineOperand { path, divisor })
}

fn parse_combine_fields(script: &str) -> Option<CombineFields> {
    use crate::painless_params::clean_path;

    // The assignment is the one whose RHS holds the arithmetic: taking the
    // script's FIRST `=` reads a preamble that creates the target container,
    // and the combination is never found.
    let (at, separator, op) = [(" + ", CombineOp::Add), (" - ", CombineOp::Subtract)]
        .into_iter()
        .filter_map(|(separator, op)| {
            script
                .find(&format!("{separator}ctx."))
                .map(|at| (at, separator, op))
        })
        .min_by_key(|(at, _, _)| *at)?;
    let assign = script[..at].rfind(" = ")?;
    let target = script[..assign].trim().rsplit("ctx.").next()?;

    let rhs = &script[assign + " = ".len()..];
    let end = rhs.find([';', '\n']).unwrap_or(rhs.len());
    let rhs = rhs[..end].trim();
    let rhs = rhs
        .strip_prefix('(')
        .and_then(|inner| inner.strip_suffix(')'))
        .unwrap_or(rhs);
    let (left, right) = rhs.split_once(separator)?;

    // A target the script assigns TWICE needs a matcher that reads both
    // writes: rubrik computes a free size in one branch of an `if` and zeroes
    // it in the other, and claiming that here would drop the zero.
    if script.matches(&format!("ctx.{target} = ")).count() != 1 {
        return None;
    }

    Some(CombineFields {
        target: clean_path(target),
        op,
        left: combine_operand(left)?,
        right: combine_operand(right)?,
    })
}

/// Combine two fields into a third, leaving the target alone where either is
/// absent.
pub fn combine_fields(event: &mut Event, pattern: &CombineFields) -> bool {
    let (Some(a), Some(b)) = (
        event.get(&pattern.left.path).cloned(),
        event.get(&pattern.right.path).cloned(),
    ) else {
        return true;
    };
    // Painless combines two integers as an integer and anything else as a
    // double, so a percentage stays fractional rather than truncating to zero.
    // An integer divisor truncates for the same reason.
    let total = if let (Some(x), Some(y)) = (a.as_i64(), b.as_i64()) {
        let x = pattern.left.divisor.map_or(x, |n| x / n);
        let y = pattern.right.divisor.map_or(y, |n| y / n);
        match pattern.op {
            CombineOp::Add => json!(x.saturating_add(y)),
            CombineOp::Subtract => json!(x.saturating_sub(y)),
        }
    } else {
        let (Some(x), Some(y)) = (a.as_f64(), b.as_f64()) else {
            return true;
        };
        #[allow(clippy::cast_precision_loss)]
        let divide = |value: f64, divisor: Option<i64>| divisor.map_or(value, |n| value / n as f64);
        let (x, y) = (
            divide(x, pattern.left.divisor),
            divide(y, pattern.right.divisor),
        );
        match pattern.op {
            CombineOp::Add => json!(x + y),
            CombineOp::Subtract => json!(x - y),
        }
    };
    let _ = event.set(&pattern.target, total);
    true
}

/// `event.duration = <field> * 1_000_000_000`, seconds to nanoseconds.
///
/// Returns false when the field name cannot be read out of the SCRIPT: that is
/// a pattern this code does not actually understand, and counting it as handled
/// would inflate the coverage figure. A field the script names but the EVENT
/// lacks is a different thing -- the script would have done nothing either.
/// A map's VALUES collected into a deduplicated list, written back in place.
///
/// netskope stores a single-valued field as a numbered map and flattens it
/// this way. The dedupe is the point: without it the same mime type or address
/// lands three or four times and no event can match.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DedupeMapValues {
    path: String,
}

impl DedupeMapValues {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

/// `def p = ctx.<path>; ... for (e in p.entrySet()) { l.add(e.getValue()); }
/// ctx.<path> = new ArrayList(new HashSet(l));`
fn parse_dedupe_map_values(script: &str) -> Option<DedupeMapValues> {
    if !script.contains("new HashSet(") || !script.contains(".getValue()") {
        return None;
    }
    let path = painless_path(script.split_once(".entrySet()")?.0)?;
    // The result must go back where it came from; a different target is a
    // different pattern.
    script
        .contains(&format!("ctx.{path} ="))
        .then_some(DedupeMapValues { path })
}

/// Collect, dedupe, and order the way Java's `HashSet` iterates.
pub fn dedupe_map_values(event: &mut Event, pattern: &DedupeMapValues) -> bool {
    let Some(Value::Object(map)) = event.get(&pattern.path) else {
        return true;
    };

    // First occurrence wins, which is what adding to a Set does.
    let mut unique: Vec<Value> = Vec::with_capacity(map.len());
    for value in map.values() {
        if !unique.contains(value) {
            unique.push(value.clone());
        }
    }

    // `new ArrayList(new HashSet(..))` reads the table in BUCKET order, not
    // insertion order, and a stable sort keeps insertion order within a
    // bucket -- which is how Java chains them.
    let table = crate::painless_helpers::java_table_size(unique.len());
    unique.sort_by_key(|value| {
        let key = crate::painless_helpers::java_to_string(value);
        crate::painless_helpers::java_bucket(&key, table)
    });

    let _ = event.set(&pattern.path, Value::Array(unique));
    true
}

/// The same dedupe, then the local parts of the addresses it produced.
///
/// netskope derives `related.user` from every address and `user.name` only
/// when exactly ONE survived -- a second address makes the name ambiguous, so
/// the vendor leaves it unset rather than guessing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailsToRelatedUsers {
    source: String,
    related: String,
    name: String,
}

impl EmailsToRelatedUsers {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        source: impl Into<String>,
        related: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            related: related.into(),
            name: name.into(),
        }
    }
}

/// `... ctx.<related> = related_users; if (n == 1) ctx.<name> = ..[0]`
fn parse_emails_to_related_users(script: &str) -> Option<EmailsToRelatedUsers> {
    if !script.contains("splitOnToken('@')") && !script.contains("splitOnToken(\"@\")") {
        return None;
    }
    // `clean_path`, not `painless_path`: the `ctx.` is already consumed by the
    // split, and `painless_path` needs to see one.
    let source = clean_path(script.split_once(" = ctx.")?.1.split(';').next()?.trim());

    // The accumulator is the list the SPLIT feeds, not the first `.add(` in
    // the script -- that one is the dedupe's own working list.
    let accumulator = script
        .split_once("splitOnToken")?
        .0
        .rsplit_once(".add(")?
        .0
        .rsplit(['\n', ';', '{', ' ', '('])
        .next()?
        .trim();
    let related = painless_path(script.split_once(&format!("= {accumulator};"))?.0)?;
    let name = painless_path(script.rsplit_once(&format!("= {accumulator}["))?.0)?;

    (!source.is_empty() && !related.is_empty() && !name.is_empty()).then_some(
        EmailsToRelatedUsers {
            source,
            related,
            name,
        },
    )
}

fn run_emails_to_related_users(event: &mut Event, pattern: &EmailsToRelatedUsers) -> bool {
    // The map form is flattened first, exactly as the script's own opening
    // guard does before it reads the list.
    if matches!(event.get(&pattern.source), Some(Value::Object(_))) {
        dedupe_map_values(event, &DedupeMapValues::new(pattern.source.clone()));
    }
    let Some(Value::Array(addresses)) = event.get(&pattern.source) else {
        return true;
    };

    // Only an address with an `@` contributes, and the local part is what the
    // vendor keeps.
    let users: Vec<Value> = addresses
        .iter()
        .filter_map(Value::as_str)
        .filter_map(|address| address.split_once('@'))
        .map(|(local, _)| Value::from(local))
        .collect();

    let single = (users.len() == 1).then(|| users[0].clone());
    let _ = event.set(&pattern.related, Value::Array(users));
    // A second address makes the name ambiguous, so the vendor leaves it.
    if let Some(name) = single {
        let _ = event.set(&pattern.name, name);
    }
    true
}

/// Every key of every object in a LIST, snake-cased in place.
///
/// A THIRD snake rule, and not either [`SnakeRule`]: `ti_recordedfuture`
/// rewrites keys with a regex substitution rather than by inspecting each
/// character, so the helpers behind `CamelToSnake` cannot express it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnakeCaseListElements {
    list: String,
}

impl SnakeCaseListElements {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(list: impl Into<String>) -> Self {
        Self { list: list.into() }
    }
}

/// One key under `/_?([a-z])([A-Z]+)/` -> `$1_$2`, then lower-cased.
///
/// The `[A-Z]+` run is greedy, so `fooBAR` becomes `foo_bar` rather than
/// `foo_b_a_r`. A run with no lower-case character before it does not match at
/// all, which is why `HTTPStatus` only loses its case.
fn regex_snake_key(key: &str) -> String {
    // `${1}` rather than `$1`: `$1_` would parse as a capture NAMED `1_`.
    crate::cached_regex!("_?([a-z])([A-Z]+)")
        .replace_all(key, "${1}_${2}")
        .to_lowercase()
}

/// Rewrite every key, recursing into maps and into maps inside lists.
fn snake_case_keys(value: &Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, value)| (regex_snake_key(key), snake_case_keys(value)))
                .collect(),
        ),
        // The vendor recurses into a list's MAP members only; anything else it
        // leaves exactly as it found it.
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|item| {
                    if item.is_object() {
                        snake_case_keys(item)
                    } else {
                        item.clone()
                    }
                })
                .collect(),
        ),
        other => other.clone(),
    }
}

/// `for (e in ctx.<list>) { out.add(keysToSnakeCase(e)); } ctx.<list> = out;`
fn parse_snake_case_list_elements(script: &str) -> Option<SnakeCaseListElements> {
    // The RULE is the trigger, not the helper's name: it is what makes this a
    // different conversion from the two `SnakeRule` variants.
    if !script.contains("_?([a-z])([A-Z]+)") || !script.contains("$1_$2") {
        return None;
    }
    let list = clean_path(script.split_once(" in ctx.")?.1.split(')').next()?.trim());

    // The loop must write the rebuilt list back over the one it read.
    if !script.contains(&format!("ctx.{list} =")) {
        return None;
    }
    (!list.is_empty()).then_some(SnakeCaseListElements { list })
}

/// Rewrite the list's objects in place.
pub fn snake_case_list_elements(event: &mut Event, pattern: &SnakeCaseListElements) -> bool {
    let Some(value @ Value::Array(_)) = event.get(&pattern.list) else {
        return true;
    };
    let converted = snake_case_keys(value);
    let _ = event.set(&pattern.list, converted);
    true
}

/// One member TOTALLED across a list of objects.
///
/// `ti_recordedfuture` reports evidence as a list and the ECS sighting count
/// is the sum of one member over it. Distinct from [`KnownPattern::CombineFields`],
/// which adds up named fields at fixed paths rather than walking a list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SumMemberOverList {
    list: String,
    member: String,
    target: String,
}

impl SumMemberOverList {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        list: impl Into<String>,
        member: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            list: list.into(),
            member: member.into(),
            target: target.into(),
        }
    }
}

/// `def t = 0; for (e in ctx.<list>) { t += e['<member>']; } ctx.<target> = t;`
fn parse_sum_member_over_list(script: &str) -> Option<SumMemberOverList> {
    let (head, rest) = script.split_once(" in ctx.")?;
    let variable = head.rsplit(['(', ' ']).next()?.trim();
    let list = crate::painless_params::clean_path(rest.split(')').next()?.trim());

    // The accumulation names the member, and only a bracketed literal is read
    // -- anything computed is a different pattern.
    let member = rest
        .split_once(&format!("{variable}['"))
        .or_else(|| rest.split_once(&format!("{variable}[\"")))?
        .1
        .split(['\'', '"'])
        .next()?;

    // The accumulator is whatever `+=` adds into, and the target is where that
    // same name is finally written.
    let accumulator = rest
        .split_once("+=")?
        .0
        .rsplit(['\n', ';', '{'])
        .next()?
        .trim();
    let target = painless_path(rest.split_once(&format!("= {accumulator};"))?.0)?;

    (!list.is_empty() && !member.is_empty() && !target.is_empty()).then_some(SumMemberOverList {
        list,
        member: member.to_owned(),
        target,
    })
}

/// Total the member across the list, skipping the entries that lack it.
pub fn sum_member_over_list(event: &mut Event, pattern: &SumMemberOverList) -> bool {
    let Some(Value::Array(items)) = event.get(&pattern.list) else {
        return true;
    };
    // Integer while every addend is one, as Painless's `int +=` stays, and a
    // double only once something fractional arrives.
    let mut whole: i64 = 0;
    let mut fraction = 0.0_f64;
    let mut fractional = false;
    for item in items {
        let Some(value) = item.get(&pattern.member) else {
            continue;
        };
        if let Some(n) = value.as_i64() {
            whole += n;
        } else if let Some(n) = value.as_f64() {
            fraction += n;
            fractional = true;
        }
    }

    #[allow(clippy::cast_precision_loss)]
    let total = if fractional {
        json!(whole as f64 + fraction)
    } else {
        json!(whole)
    };
    let _ = event.set(&pattern.target, total);
    true
}

/// The NAME of a map's first key whose value is not null.
///
/// `jamf_protect` reports which telemetry event fired by which member of one
/// object is populated, so the ECS `event.action` is a key name rather than
/// any value in the document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstPresentKeyName {
    source: String,
    target: String,
}

impl FirstPresentKeyName {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
        }
    }
}

/// `for (def k : <map>.keySet()) { if (<map>[k] != null) { ctx.<t> = k; break; } }`
fn parse_first_present_key_name(script: &str) -> Option<FirstPresentKeyName> {
    let (head, rest) = script.split_once(".keySet()")?;
    // The map is usually a local bound to the ctx path, so follow it.
    let subject = head.rsplit(['(', ' ', ':']).next()?.trim();
    let source = resolve_subject(script, subject, 3)?;

    // The loop variable is what gets written, so the assignment naming it is
    // the target. Reading any assignment would take the `ctx.event = new
    // HashMap()` line above it instead.
    let variable = head.rsplit_once(" : ")?.0.rsplit([' ', '(']).next()?.trim();
    if variable.is_empty() {
        return None;
    }
    let marker = format!("= {variable};");
    let target = painless_path(rest.split_once(&marker)?.0)?;

    (!source.is_empty() && !target.is_empty()).then_some(FirstPresentKeyName { source, target })
}

/// Write the first key that carries a value, in the document's own order.
pub fn first_present_key_name(event: &mut Event, pattern: &FirstPresentKeyName) -> bool {
    let Some(Value::Object(map)) = event.get(&pattern.source) else {
        return true;
    };
    // Insertion order, which `preserve_order` gives us and which is the order
    // Elasticsearch's own iteration would see.
    let Some(key) = map
        .iter()
        .find(|(_, value)| !value.is_null())
        .map(|(key, _)| key.clone())
    else {
        return true;
    };
    let _ = event.set(&pattern.target, json!(key));
    true
}

/// FRACTIONAL seconds scaled to nanoseconds and cast to a Java `int`.
///
/// Distinct from [`KnownPattern::DurationToNanos`], which parses a whole
/// number of seconds and saturates at `i64`. stormshield parses a float and
/// the vendor casts with `(int)`, so the result saturates at `i32` -- two of
/// its events expect exactly 2,147,483,647, which is that cast and not an
/// overflow to correct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloatSecondsToNanos {
    source: String,
    target: String,
    /// The script removes the seconds field once it has been converted.
    remove_source: bool,
}

impl FloatSecondsToNanos {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(source: impl Into<String>, target: impl Into<String>, remove_source: bool) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            remove_source,
        }
    }
}

/// `def d = Float.parseFloat(ctx.<source>); d *= 1000000000; ctx.<target> = (int)d;`
fn parse_float_seconds_to_nanos(script: &str) -> Option<FloatSecondsToNanos> {
    let source = script
        .split_once("Float.parseFloat(ctx.")?
        .1
        .split(')')
        .next()?;
    let source = crate::painless_params::clean_path(source.trim());

    // The cast names the target, and is what makes this an i32 result.
    let target = script.split_once("= (int)")?.0.rsplit("ctx.").next()?;
    let target = crate::painless_params::clean_path(target.trim().trim_start_matches('{').trim());

    let remove_source = script.contains(".remove(\"") || script.contains(".remove('");
    (!source.is_empty() && !target.is_empty()).then_some(FloatSecondsToNanos {
        source,
        target,
        remove_source,
    })
}

/// Scale the seconds, then narrow the way Java's `(int)` narrows.
pub fn float_seconds_to_nanos(event: &mut Event, pattern: &FloatSecondsToNanos) -> bool {
    let seconds = event.get(&pattern.source).and_then(|value| match value {
        Value::String(text) => text.parse::<f64>().ok(),
        other => other.as_f64(),
    });
    let Some(seconds) = seconds else {
        return true;
    };

    // Java narrows a float to int by SATURATING, not by wrapping, so a long
    // session lands on i32::MAX rather than a negative duration.
    let nanos = seconds * 1e9;
    let narrowed = if nanos >= f64::from(i32::MAX) {
        i64::from(i32::MAX)
    } else if nanos <= f64::from(i32::MIN) {
        i64::from(i32::MIN)
    } else {
        nanos as i64
    };

    let _ = event.set(&pattern.target, json!(narrowed));
    if pattern.remove_source {
        event.remove(&pattern.source);
    }
    true
}

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
    let Some(values_path) = crate::painless_params::ctx_path_between(&body, "(ctx, ctx.", "[i]")
    else {
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

/// The quoted strings of a literal list.
pub(crate) fn quoted_members(literals: &str) -> Vec<String> {
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

/// `ctx.<target> = <first> + ctx.<source>.substring(1).toLowerCase()`.
///
/// A vendor severity arrives shouted and ECS wants it title-cased, so the
/// script keeps the leading character and lowers the rest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TitleCase {
    source: String,
    target: String,
}

fn parse_title_case(script: &str) -> Option<TitleCase> {
    let head = script.split_once(".substring(1).toLowerCase()")?.0;

    // The nearest `ctx.` path is the one being lowered; the assignment before
    // it names where the result lands.
    let source = painless_path(head)?;
    let at = last_assignment(head)?;
    let target = painless_path(&head[..at])?;
    (!source.is_empty() && !target.is_empty()).then_some(TitleCase { source, target })
}

/// An empty string writes nothing, as the script's own `!= ""` guard does.
fn run_title_case(event: &mut Event, pattern: &TitleCase) -> bool {
    let titled = {
        let Some(raw) = event.get_str(&pattern.source) else {
            return true;
        };
        if raw.is_empty() {
            return true;
        }
        let mut out = String::with_capacity(raw.len());
        let mut chars = raw.chars();
        if let Some(first) = chars.next() {
            out.push(first);
        }
        out.extend(chars.flat_map(char::to_lowercase));
        out
    };
    let _ = event.set(&pattern.target, json!(titled));
    true
}

/// `for (def d: ctx.<map>.entrySet())` scanning for one key, with a literal
/// fall-through when it is absent.
///
/// crowdstrike reconstructs a quarantine flag this way. The script is the only
/// writer of both the found value and the default, so a miss costs the field on
/// true and false alike rather than looking like a drop-empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NamedMapEntry {
    map: String,
    key: String,
    target: String,
    fallback: Value,
}

fn parse_named_map_entry(script: &str) -> Option<NamedMapEntry> {
    use crate::painless_params::clean_path;

    let (local, rest) = script.split_once("for (def ")?.1.split_once(':')?;
    let local = local.trim();
    let map = clean_path(
        rest.split_once(".entrySet()")?
            .0
            .trim()
            .strip_prefix("ctx.")?,
    );
    if map.is_empty() {
        return None;
    }

    let key = script
        .split_once(&format!("{local}.getKey() =="))?
        .1
        .split(')')
        .next()?
        .trim()
        .trim_matches(['\'', '"'])
        .to_string();
    if key.is_empty() {
        return None;
    }

    let target = painless_path(script.split_once(&format!("= {local}.getValue()"))?.0)?;

    // The last statement is the fall-through, and it must write the same field.
    let (lhs, rhs) = script.trim_end().trim_end_matches(';').rsplit_once('=')?;
    if painless_path(lhs)? != target {
        return None;
    }

    Some(NamedMapEntry {
        map,
        key,
        target,
        fallback: painless_literal(rhs)?,
    })
}

/// A map the event does not carry is the processor's own `instanceof Map`
/// guard, so nothing is written.
fn run_named_map_entry(event: &mut Event, pattern: &NamedMapEntry) -> bool {
    let found = {
        let Some(map) = event.get(&pattern.map).and_then(Value::as_object) else {
            return true;
        };
        map.get(&pattern.key).cloned()
    };
    let _ = event.set(
        &pattern.target,
        found.unwrap_or_else(|| pattern.fallback.clone()),
    );
    true
}

/// A numeric band and the literal it names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RangeArm {
    low: i64,
    high: i64,
    /// The last band closes with `<=` where the rest use `<`.
    high_inclusive: bool,
    value: String,
}

/// An `if (0 <= n && n < 20) { ctx.t = "info" } else if ...` ladder.
///
/// A score becomes the name of its band. crowdstrike's alert severity is the
/// pattern, and its name then feeds a second script that scores it back, so a
/// miss here costs both fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RangeLadder {
    source: String,
    target: String,
    arms: Vec<RangeArm>,
}

/// Parse a numeric-band ladder, or `None` if the script is a different pattern.
fn parse_range_ladder(script: &str) -> Option<RangeLadder> {
    use crate::painless_params::clean_path;

    // A ladder is the whole of its script; one that loops is doing other work.
    if script.contains("for (") {
        return None;
    }

    // `long severity = ctx.crowdstrike.alert.severity;`
    let (declaration, tail) = script.split_once(" = ctx.")?;
    let local = declaration.split_whitespace().next_back()?;
    let source = clean_path(tail.split([';', '\n']).next()?.trim());
    if source.is_empty()
        || !source
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_'))
    {
        return None;
    }

    let mut target: Option<String> = None;
    let mut arms = Vec::new();
    for block in script.split("if (").skip(1) {
        let (guard, body) = block.split_once(") {")?;

        // `0 <= severity && severity < 20`
        let (lower, upper) = guard.split_once("&&")?;
        let (low_text, low_subject) = lower.split_once("<=")?;
        if low_subject.trim() != local {
            return None;
        }
        let low = low_text.trim().parse::<i64>().ok()?;

        let upper = upper.trim().strip_prefix(local)?.trim();
        let (high_inclusive, high_text) = match upper.strip_prefix("<=") {
            Some(text) => (true, text),
            None => (false, upper.strip_prefix('<')?),
        };
        let high = high_text.trim().parse::<i64>().ok()?;

        // `ctx.<target> = "<value>";`
        let (lhs, rhs) = body.split(';').next()?.split_once('=')?;
        let path = painless_path(lhs)?;
        if target.get_or_insert_with(|| path.clone()) != &path {
            return None;
        }
        let value = rhs.trim().trim_matches('"').to_string();
        if value.is_empty() {
            return None;
        }

        arms.push(RangeArm {
            low,
            high,
            high_inclusive,
            value,
        });
    }

    if arms.is_empty() {
        return None;
    }
    Some(RangeLadder {
        source,
        target: target?,
        arms,
    })
}

/// A value outside every band writes nothing, as the script's own fall-through
/// does.
fn run_range_ladder(event: &mut Event, pattern: &RangeLadder) -> bool {
    let Some(value) = event.get_as_i64(&pattern.source) else {
        return true;
    };
    for arm in &pattern.arms {
        let within_upper = if arm.high_inclusive {
            value <= arm.high
        } else {
            value < arm.high
        };
        if value >= arm.low && within_upper {
            let _ = event.set(&pattern.target, json!(arm.value));
            return true;
        }
    }
    true
}

/// `long <local> = ctx.<source>; ctx.event = ctx.event ?: [:]; ctx.event.<risk> = (double) <local>; if (<local> < <n1>) { ctx.event.<severity> = <v1>; } else if (<local> < <n2>) { ... } else { ctx.event.<severity> = <vN>; }`
///
/// crowdstrike's automated-lead risk score: unlike [`RangeLadder`], each arm
/// names only a CEILING, not a band, and the final `else` has none at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreSeverityBands {
    source: String,
    risk_score_target: String,
    severity_target: String,
    /// `(ceiling, severity)` ascending; the LAST entry's ceiling is `None`.
    bands: Vec<(Option<i64>, i64)>,
}

fn parse_score_severity_bands(script: &str) -> Option<ScoreSeverityBands> {
    // `long score = ctx.crowdstrike.alert.score;`
    let (declaration, tail) = script.split_once(" = ctx.")?;
    let local = declaration.split_whitespace().next_back()?;
    let source = clean_path(tail.split([';', '\n']).next()?.trim());
    if source.is_empty() {
        return None;
    }

    // `ctx.event.risk_score = (double) score;`
    let risk_score_target = ctx_assignment_target_before(script, &format!(" = (double) {local};"))?;

    let mut bands = Vec::new();
    let mut severity_target: Option<String> = None;
    for block in script.split("if (").skip(1) {
        let (guard, body) = block.split_once(") {")?;
        let ceiling: i64 = guard
            .trim()
            .strip_prefix(local)?
            .trim()
            .strip_prefix('<')?
            .trim()
            .parse()
            .ok()?;

        let (lhs, rhs) = body.split(';').next()?.split_once('=')?;
        let path = painless_path(lhs)?;
        if severity_target.get_or_insert_with(|| path.clone()) != &path {
            return None;
        }
        bands.push((Some(ceiling), rhs.trim().parse::<i64>().ok()?));
    }

    // The ladder's unconditional tail, one level below every guarded arm.
    let (_, else_body) = script.rsplit_once("} else {")?;
    let (lhs, rhs) = else_body.split(';').next()?.split_once('=')?;
    if painless_path(lhs)?.as_str() != severity_target.as_deref()? {
        return None;
    }
    bands.push((None, rhs.trim().parse::<i64>().ok()?));

    (bands.len() > 2).then_some(ScoreSeverityBands {
        source,
        risk_score_target,
        severity_target: severity_target?,
        bands,
    })
}

fn run_score_severity_bands(event: &mut Event, pattern: &ScoreSeverityBands) -> bool {
    let Some(score) = event.get_as_i64(&pattern.source) else {
        return true;
    };
    #[allow(clippy::cast_precision_loss)]
    let _ = event.set(&pattern.risk_score_target, json!(score as f64));
    for (ceiling, severity) in &pattern.bands {
        if ceiling.is_none_or(|c| score < c) {
            let _ = event.set(&pattern.severity_target, json!(severity));
            break;
        }
    }
    true
}

/// `def <local> = ctx.<list>[<index>]; if (<local>.<member> != null && <local>.<member> != '') { ctx.<container> = ctx.<container> ?: [:]; ctx.<target> = <local>.<member>; }`, repeated per member.
///
/// crowdstrike copies a few fields off the FIRST threatgraph indicator this
/// way; the null-check is [`crate::painless_helpers::painless_is_empty_value`]
/// applied to the indexed item directly, so the guards themselves need no
/// parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexedFieldCopies {
    list: String,
    index: usize,
    /// (the indexed item's member, the `ctx.` path it lands on).
    copies: Vec<(String, String)>,
}

fn parse_indexed_field_copies(script: &str) -> Option<IndexedFieldCopies> {
    let (declaration, rest) = script.split_once(" = ctx.")?;
    let local = declaration.split_whitespace().next_back()?;
    let (list, rest) = rest.split_once('[')?;
    let list = clean_path(list.trim());
    let index: usize = rest.split(']').next()?.trim().parse().ok()?;

    let prefix = format!("{local}.");
    let copies: Vec<(String, String)> = crate::painless_params::ctx_writes(script)
        .into_iter()
        .filter_map(|(path, rhs)| {
            let member = rhs.trim().strip_prefix(&prefix)?;
            (!member.is_empty() && member.chars().all(|c| c.is_alphanumeric() || c == '_'))
                .then(|| (member.to_string(), path))
        })
        .collect();

    (!copies.is_empty()).then_some(IndexedFieldCopies {
        list,
        index,
        copies,
    })
}

fn run_indexed_field_copies(event: &mut Event, pattern: &IndexedFieldCopies) -> bool {
    let Some(item) = event
        .get(&pattern.list)
        .and_then(Value::as_array)
        .and_then(|items| items.get(pattern.index))
        .cloned()
    else {
        return true;
    };
    for (member, target) in &pattern.copies {
        if let Some(value) = item
            .get(member)
            .filter(|v| !crate::painless_helpers::painless_is_empty_value(v))
            .cloned()
        {
            let _ = event.set(target, value);
        }
    }
    true
}

/// `def <local> = new HashMap(); <local>.put('<k1>', ctx.<s1>); <local>.put('<k2>', ctx.<s2>); ... ctx.<target> = <local>;`
///
/// crowdstrike combines a latitude and a longitude this way; nothing about it
/// is specific to geo, so any script assembling a local map from `ctx.`
/// fields and assigning the WHOLE map onto one target fits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldsIntoMap {
    /// (the map key, the source `ctx.` path), in script order.
    members: Vec<(String, String)>,
    target: String,
}

fn parse_fields_into_map(script: &str) -> Option<FieldsIntoMap> {
    let (before, _) = script.split_once(" = new HashMap();")?;
    let local = before.rsplit_once("def ")?.1.trim();

    let members: Vec<(String, String)> = parse_put_calls(script)
        .into_iter()
        .filter(|call| call.receiver == local)
        .filter_map(|call| Some((call.key, clean_path(call.value.strip_prefix("ctx.")?))))
        .collect();
    if members.len() < 2 {
        return None;
    }

    let target = ctx_assignment_target_before(script, &format!(" = {local};"))?;
    Some(FieldsIntoMap { members, target })
}

/// Any member absent leaves the whole map unwritten -- the vendor guards the
/// SCRIPT on every source being non-null, so this only runs when they are.
fn run_fields_into_map(event: &mut Event, pattern: &FieldsIntoMap) -> bool {
    let mut map = Map::new();
    for (key, source) in &pattern.members {
        let Some(value) = event.get(source).cloned() else {
            return true;
        };
        map.insert(key.clone(), value);
    }
    let _ = event.set(&pattern.target, Value::Object(map));
    true
}

/// crowdstrike `identity_protection_timeline`, tagged
/// `map_entity_identity_fields_by_type`: `entity.secondary_display_name`
/// splits at its first backslash into a domain for a USER entity, or lands
/// whole on `host.hostname` for an ENDPOINT entity with no backslash;
/// `entity.primary_display_name` then lands on `user.full_name` or
/// `host.name`, independent of the backslash branch above.
fn try_crowdstrike_timeline_entity_identity(event: &mut Event) -> bool {
    let entity_type = event
        .get_str("crowdstrike.idp.timeline.entity.type")
        .unwrap_or_default()
        .to_string();

    if let Some(secondary) = event
        .get_str("crowdstrike.idp.timeline.entity.secondary_display_name")
        .filter(|s| !s.is_empty())
        .map(str::to_string)
    {
        if let Some((domain, _)) = secondary.split_once('\\') {
            if !domain.is_empty() && entity_type == "USER" {
                let _ = event.set("user.domain", json!(domain));
            }
        } else if entity_type == "ENDPOINT" {
            let _ = event.set("host.hostname", json!(secondary));
        }
    }

    if let Some(primary) = event
        .get("crowdstrike.idp.timeline.entity.primary_display_name")
        .cloned()
    {
        match entity_type.as_str() {
            "ENDPOINT" => {
                let _ = event.set("host.name", primary);
            }
            "USER" => {
                let _ = event.set("user.full_name", primary);
            }
            _ => {}
        }
    }
    true
}

/// crowdstrike `identity_protection_timeline`, tagged
/// `map_entity_accounts_by_type`: the FIRST Active Directory account
/// descriptor names `host.*`, `user.*` or `entity.*` depending on the
/// entity's own type, and an ENDPOINT's SAM name loses a trailing `$` and
/// never overwrites a `host.name` some earlier processor already set.
fn try_crowdstrike_timeline_entity_accounts(event: &mut Event) -> bool {
    let entity_type = event
        .get_str("crowdstrike.idp.timeline.entity.type")
        .unwrap_or_default()
        .to_string();
    let Some(Value::Object(account)) = event
        .get("crowdstrike.idp.timeline.entity.accounts")
        .and_then(Value::as_array)
        .and_then(|accounts| accounts.first())
        .cloned()
    else {
        return true;
    };
    let sam = account
        .get("sam_account_name")
        .and_then(Value::as_str)
        .map(str::to_string);
    let sid = account.get("object_sid").cloned();

    match entity_type.as_str() {
        "ENDPOINT" => {
            if let Some(sam) = &sam
                && !event.has_value("host.name")
            {
                let _ = event.set("host.name", json!(sam.strip_suffix('$').unwrap_or(sam)));
            }
            if let Some(sid) = sid {
                let _ = event.set("host.id", sid);
            }
        }
        "USER" => {
            if let Some(sam) = &sam {
                let _ = event.set("user.name", json!(sam));
                let _ = event.set("user.target.name", json!(sam));
            }
            if let Some(sid) = sid {
                let _ = event.set("user.id", sid);
            }
        }
        _ => {
            if let Some(sam) = &sam {
                let _ = event.set("entity.target.name", json!(sam));
            }
            if let Some(sid) = sid {
                let _ = event.set("entity.id", sid);
            }
        }
    }
    true
}

/// Every `ctx.<target> = ctx.<array>[0];` a script writes.
///
/// crowdstrike's correlation-detection alerts take the first of several
/// source/destination/user lists this way -- unconditionally for a target
/// nothing else could have set yet, guarded where an earlier processor might
/// already have written one. `checkpoint_harmony_endpoint` spells a third guard
/// that UNWRAPS rather than selects: a value grok left as a one-element list
/// becomes the scalar Elasticsearch stores.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstElement {
    /// EVERY take the script writes, in the order it writes them.
    ///
    /// One script is not one take. `checkpoint_harmony_endpoint` unwraps
    /// `host.os.name` and `host.os.version` in the same script, and reading
    /// only the first `[0];` left `host.os.version` a one-element list on all
    /// 19 of the source's events.
    takes: Vec<FirstOf>,
}

/// One `ctx.<target> = ctx.<array>[0];` and the guard it sits under.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstOf {
    array: String,
    target: String,
    /// Guarded on `ctx.<target> == null || ctx.<target> == ''`.
    only_if_unset: bool,
    /// Guarded on `ctx.<array>.size() == 1`, so a longer list keeps every
    /// element instead of losing all but the first.
    only_if_single: bool,
}

fn parse_first_element(script: &str) -> Option<FirstElement> {
    let mut takes = Vec::new();
    let mut from = 0;
    while let Some(rel) = script[from..].find("[0];") {
        let at = from + rel;
        from = at + "[0];".len();

        // Anchored on THIS statement's own `ctx.`, so an earlier statement's
        // `=` (the `ctx.<container> = ctx.<container> ?: [:];` init) is not
        // mistaken for the assignment.
        let Some(rhs_at) = script[..at].rfind("ctx.") else {
            // Skipped, never failed: declining the whole script would drop the
            // takes that DO parse, and those are what the claim is for.
            continue;
        };
        let array = clean_path(script[rhs_at + "ctx.".len()..at].trim());
        let Some(lhs) = script[..rhs_at].strip_suffix(" = ") else {
            continue;
        };
        let Some(target_at) = lhs.rfind("ctx.") else {
            continue;
        };
        let target = clean_path(lhs[target_at + "ctx.".len()..].trim());

        let only_if_unset =
            script.contains(&format!("if (ctx.{target} == null || ctx.{target} == '')"));
        let only_if_single = script.contains(&format!("ctx.{array}.size() == 1)"));
        takes.push(FirstOf {
            array,
            target,
            only_if_unset,
            only_if_single,
        });
    }
    if takes.is_empty() {
        return None;
    }
    Some(FirstElement { takes })
}

fn run_first_element(event: &mut Event, pattern: &FirstElement) -> bool {
    for take in &pattern.takes {
        if take.only_if_unset && event.has_value(&take.target) {
            continue;
        }
        let first = match event.get(&take.array) {
            // The script's own `size() == 1` guard. Painless leaves a longer
            // list alone, and taking its first element would drop the rest.
            Some(Value::Array(items)) if !(take.only_if_single && items.len() != 1) => {
                match items.first() {
                    Some(first) => first.clone(),
                    None => continue,
                }
            }
            _ => continue,
        };
        let _ = event.set(&take.target, first);
    }
    true
}

/// One arm of an equality ladder: the literal(s) tested, and what it assigns.
///
/// More than one literal is an OR'd condition -- `u == 'LOW' || u ==
/// 'NEUTRAL'` -- both winning the same arm rather than only the first the
/// script names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LadderArm {
    literals: Vec<String>,
    /// EVERY assignment in the arm, in the order the script writes them.
    ///
    /// One arm does not mean one field: gdacs grades a disaster's alert level
    /// into `event.severity` AND `event.risk_score` together, and reading only
    /// the first left the second wrong on every event. The value is whatever
    /// `painless_literal` reads -- a quoted string in most vendored ladders,
    /// but a severity NUMBER arrives bare (`ctx.event.severity = 21;`), which
    /// a `String` field cannot hold without quoting it into the wrong JSON
    /// type.
    writes: Vec<(String, Value)>,
    /// Fields the arm REMOVES once it has written.
    ///
    /// stormshield reads `ipv` into `network.type` and drops it, and keeping
    /// it left a field Elasticsearch does not emit on 13 of its 44 events.
    removes: Vec<String>,
}

/// The fields an arm's body removes: `ctx.<parent>.remove("<key>")`.
fn arm_removes(body: &str) -> Vec<String> {
    use crate::painless_params::clean_path;

    let parts: Vec<&str> = body.split(".remove(").collect();
    let mut removes = Vec::new();
    // Each pair is the text BEFORE a `.remove(` and the text after it, so the
    // parent is the last `ctx.` path on the left and the key the literal on
    // the right.
    for pair in parts.windows(2) {
        let Some(parent) = painless_path(pair[0]) else {
            continue;
        };
        let Some(key) = pair[1]
            .trim_start()
            .strip_prefix(['"', '\''])
            .and_then(|rest| rest.split(['"', '\'']).next())
        else {
            continue;
        };
        if !parent.is_empty() && !key.is_empty() {
            removes.push(clean_path(&format!("{parent}.{key}")));
        }
    }
    removes
}

/// An `if (x == 'a') { ctx.t = 'A' } else if (x == 'b') { ... }` ladder.
///
/// The subject is read once -- bound to a local (`def x = ctx.a.b;`), bound
/// through a SECOND local (`def u = level.toUpperCase();` where `level`
/// itself reads `ctx.a.b`), or compared inline -- and every arm assigns a
/// literal to a ctx path. Fortinet's 11-arm IANA-number-to-transport table is
/// the pattern; writing the table out by hand is how a mapping silently goes
/// stale.
///
/// Owned rather than borrowed from the script: the parse runs once per call
/// site via [`crate::painless_plan::PainlessPlan`], so the arms are allocated
/// once per process, not once per event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Ladder {
    subject: String,
    /// The local the subject was bound through carried `.toLowerCase()`, so
    /// every comparison folds case -- inspector maps `HIGH` and `high` alike.
    fold_case: bool,
    arms: Vec<LadderArm>,
}

/// Parse an equality ladder, or `None` if the script is a different pattern.
fn parse_ladder(script: &str) -> Option<Ladder> {
    use crate::painless_params::clean_path;

    // A ladder is the WHOLE of its script. One that loops is doing something
    // else entirely -- powershell's entropy pass walks the script block
    // character by character -- and claiming it runs the ladder instead of the
    // real work.
    if script.contains("for (") {
        return None;
    }

    // The subject is whatever the FIRST ARM compares against -- a leading
    // type guard such as `if (level instanceof String) { ... }` has no `==`
    // of its own, so it is passed over rather than mistaken for the ladder.
    let lhs = script.split("if (").skip(1).find_map(|segment| {
        let (cond, _) = segment.split_once(')')?;
        let (lhs, _) = cond.split_once("==")?;
        Some(lhs.trim())
    })?;
    let mut through_put = false;

    let mut arms = Vec::new();
    for segment in script.split("if (").skip(1) {
        let Some((cond, body)) = segment.split_once(')') else {
            continue;
        };
        // Every literal an OR'd condition compares the subject against --
        // `u == 'LOW' || u == 'NEUTRAL'` is one arm with two literals, and a
        // plain `x == 'a'` is the same pattern with one.
        let literals: Vec<String> = cond
            .split("||")
            .filter_map(|piece| {
                piece
                    .split_once("==")
                    .and_then(|(_, rhs)| quoted_first(rhs))
            })
            .collect();
        if literals.is_empty() {
            continue;
        }
        // `ctx.<parent>.put('<key>', '<value>')` writes the same thing an
        // assignment does, and inspector's severity map is written that way.
        let arm = if let Some((subject, arguments)) = body.split_once(".put(") {
            let (Some(parent), Some((key, value))) = (
                subject
                    .trim()
                    .trim_start_matches('{')
                    .trim()
                    .strip_prefix("ctx."),
                quoted_first(arguments).zip(
                    arguments
                        .split_once(',')
                        .and_then(|(_, rest)| quoted_first(rest)),
                ),
            ) else {
                continue;
            };
            through_put = true;
            LadderArm {
                literals,
                writes: vec![(
                    format!("{}.{key}", clean_path(parent.trim())),
                    Value::String(value),
                )],
                removes: arm_removes(body),
            }
        } else {
            // Every assignment in the arm, not just the first -- a graded
            // severity writes a score alongside it.
            let mut writes = Vec::new();
            for statement in body.split(';') {
                let Some((lhs, rhs)) = statement.split_once('=') else {
                    continue;
                };
                let Some(target) = lhs
                    .trim()
                    .trim_start_matches('{')
                    .trim()
                    .strip_prefix("ctx.")
                else {
                    continue;
                };
                let Some(value) = painless_literal(rhs.trim()) else {
                    continue;
                };
                writes.push((clean_path(target.trim()), value));
            }
            if writes.is_empty() {
                continue;
            }
            LadderArm {
                literals,
                writes,
                removes: arm_removes(body),
            }
        };
        arms.push(arm);
    }
    if arms.len() < 2 {
        return None;
    }
    // A `.put()` arm is also how a script BUILDS a map, so the pattern only
    // claims one where every `if` in it is an arm -- nothing else going on.
    if through_put && arms.len() != script.matches("if (").count() {
        return None;
    }

    let (subject, fold_case) = match ladder_subject(script, lhs) {
        Some(resolved) => resolved,
        None => (clean_path(lhs.strip_prefix("ctx.")?), false),
    };

    Some(Ladder {
        subject,
        fold_case,
        arms,
    })
}

/// The ctx path a subject local ultimately reads, plus whether any hop folds
/// case with `.toLowerCase()` / `.toUpperCase()`.
///
/// `def u = level.toUpperCase();` binds `u` to ANOTHER local rather than to
/// `ctx.` directly, and `crowdstrike`'s severity ladder is written exactly
/// this way -- `level` carries the ctx path two lines up, `u` carries the
/// fold. One extra hop covers every vendored script seen so far, so the
/// lookup does not recurse further.
fn ladder_subject(script: &str, name: &str) -> Option<(String, bool)> {
    if let Some(path) = ctx_path_bound_to(script, name) {
        let (path, declared_fold) = strip_case_fold(&path);
        return Some((path, declared_fold || self_folds(script, name)));
    }

    let needle = format!(" {name} = ");
    let at = script.find(&needle)?;
    let rest = &script[at + needle.len()..];
    let end = rest.find([';', '\n']).unwrap_or(rest.len());
    let (other, own_fold) = strip_case_fold(rest[..end].trim());
    if other.is_empty() || !other.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let (path, deep_fold) = ctx_path_bound_to(script, &other).map(|p| strip_case_fold(&p))?;
    Some((path, own_fold || deep_fold))
}

/// Does the script narrow the local ONTO ITSELF -- `level = level.toLowerCase()`?
///
/// `ctx_path_bound_to` reads the DECLARATION only, so a fold written as its own
/// later statement is invisible to it. gdacs writes exactly that, and without
/// this the ladder compares an unfolded `"Red"` against the script's own
/// lower-case `"red"` and no arm can ever match.
fn self_folds(script: &str, name: &str) -> bool {
    [".toLowerCase()", ".toUpperCase()"]
        .iter()
        .any(|fold| script.contains(&format!("{name} = {name}{fold}")))
}

/// Split a trailing `.toLowerCase()` / `.toUpperCase()` off a path, reporting
/// whether one was there.
fn strip_case_fold(path: &str) -> (String, bool) {
    match path
        .strip_suffix(".toLowerCase()")
        .or_else(|| path.strip_suffix(".toUpperCase()"))
    {
        Some(bare) => (bare.to_string(), true),
        None => (path.to_string(), false),
    }
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
/// the same pattern over another vendor's version field lands the same way.
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
/// skipping -- 1,483 of 1,487 scripts, this one pattern.
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

/// Every value under a map whose KEY ends with one of a set of suffixes,
/// unioned with what the target already holds and sorted.
///
/// ```painless
/// HashSet set = null;
/// for (key in ctx.netflow.keySet()) {
///     if (key.endsWith("_ipv4_address") || key.endsWith("_ipv6_address")) {
///         if (set == null) { set = new HashSet(); }
///         set.add(ctx.netflow[key]);
///     }
/// }
/// if (set != null) {
///     if (ctx.related?.ip != null) { for (ip in ctx.related.ip) { set.add(ip); } }
///     ArrayList list = new ArrayList(set);
///     Collections.sort(list);
///     ctx.related.ip = list;
/// }
/// ```
///
/// The suffix is the whole selector: netflow names its addresses per role
/// (`source_`, `destination_`, `post_nat_`...), so no field list would cover a
/// template the exporter is free to extend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct KeysBySuffix {
    map: String,
    target: String,
    suffixes: Vec<String>,
}

/// Read the scanned map, the suffixes selecting its keys, and the target the
/// sorted union lands on.
fn parse_keys_by_suffix(script: &str) -> Option<KeysBySuffix> {
    let map = script
        .split_once(".keySet()")
        .and_then(|(head, _)| painless_path(head))?;

    let (_, tail) = script.rsplit_once("Collections.sort(")?;
    let at = last_assignment(tail)?;
    let target = painless_path(&tail[..at])?;

    let mut suffixes = Vec::new();
    for (at, opener) in script.match_indices(".endsWith(") {
        let rest = &script[at + opener.len()..];
        let quote = rest.chars().next().filter(|q| *q == '\'' || *q == '"')?;
        let literal = rest[quote.len_utf8()..].split_once(quote)?.0;
        suffixes.push(literal.to_string());
    }

    (!suffixes.is_empty()).then_some(KeysBySuffix {
        map,
        target,
        suffixes,
    })
}

/// Union into the target, sorted as STRINGS.
///
/// `Collections.sort` on a list of address strings is lexicographic, which is
/// why `2a02:cf40::1` sorts between `0.0.0.0` and `81.2.69.144` rather than
/// after both.
fn run_keys_by_suffix(event: &mut Event, pattern: &KeysBySuffix) -> bool {
    let mut matched = false;
    let mut collected: Vec<String> = Vec::new();
    if let Some(Value::Object(entries)) = event.get(&pattern.map) {
        for (key, value) in entries {
            if !pattern.suffixes.iter().any(|suffix| key.ends_with(suffix)) {
                continue;
            }
            matched = true;
            if let Some(text) = value.as_str() {
                collected.push(text.to_owned());
            }
        }
    }
    // `set == null` leaves the target alone, so a map with no matching key is
    // not the same as one whose values are all empty.
    if !matched {
        return true;
    }

    if let Some(Value::Array(held)) = event.get(&pattern.target) {
        collected.extend(held.iter().filter_map(Value::as_str).map(str::to_owned));
    }
    collected.sort_unstable();
    collected.dedup();
    let _ = event.set(
        &pattern.target,
        Value::Array(collected.into_iter().map(Value::String).collect()),
    );
    true
}

/// `ctx.<path> = ctx.<path>.replace(<from>, <to>)`, guarded on the field.
///
/// azure's platform logs carry `properties` as a stringified object in
/// Python's repr -- single quotes -- so the pipeline rewrites the quotes before
/// handing it to a `json` processor. Painless's
/// `replace(CharSequence, CharSequence)` is a LITERAL replace of every
/// occurrence, not a regex.
/// `ctx.<target> = ctx.<source>.replace('<from>', '<to>')`, guard optional.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuardedReplace {
    source: String,
    target: String,
    from: String,
    to: String,
}

impl GuardedReplace {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(
        source: impl Into<String>,
        target: impl Into<String>,
        from: impl Into<String>,
        to: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            from: from.into(),
            to: to.into(),
        }
    }
}

fn parse_guarded_replace(script: &str) -> Option<GuardedReplace> {
    use crate::painless_params::clean_path;

    let (head, arguments) = script.split_once(".replace(")?;
    // The last bare `=` is the assignment: the guard above it is `!= null`.
    let assign = head.rfind('=').filter(|at| {
        !matches!(
            head[..*at].chars().next_back(),
            Some('!' | '=' | '<' | '>' | '+')
        )
    })?;
    let (target_at, source_at) = (head[..assign].rfind("ctx.")?, head[assign..].rfind("ctx.")?);
    let target = clean_path(head[target_at + "ctx.".len()..assign].trim());
    let source = clean_path(head[assign + source_at + "ctx.".len()..].trim());
    // A guard sitting above the assignment holds a `ctx.` of its own, and
    // taking THAT as the target invented `winlog.event_data.SubcategoryGuid ==
    // null) {` as a field. A target is a plain dotted path or it is not ours.
    if !target
        .chars()
        .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@'))
    {
        return None;
    }
    let (from, to) = two_string_literals(arguments)?;
    Some(GuardedReplace::new(source, target, from, to))
}

/// Rewrite one substring of a field, leaving a non-string source alone.
pub fn guarded_replace(event: &mut Event, pattern: &GuardedReplace) -> bool {
    let Some(text) = event.get_str(&pattern.source) else {
        return true;
    };
    let replaced = text.replace(pattern.from.as_str(), &pattern.to);
    let _ = event.set(&pattern.target, replaced);
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

/// The ctx path a `<type> name = ctx.a.b;` binding reads, if there is one.
///
/// The type is whatever the script declared -- `def`, `String`, `HashMap` --
/// and naming them one at a time missed `HashMap authInfo = ...`, so any
/// single leading word counts. A `?:` default is cut off: the path is what
/// comes before it.
pub(crate) fn ctx_path_bound_to(script: &str, name: &str) -> Option<String> {
    use crate::painless_params::clean_path;

    let needle = format!(" {name} = ctx.");
    let at = script.find(&needle)?;
    // One word before the name, which is what a declaration looks like.
    let declaration = script[..at].rsplit(['\n', ';', '{', '}']).next()?.trim();
    if declaration.is_empty() || declaration.contains(' ') {
        return None;
    }

    let rest = &script[at + needle.len()..];
    let end = rest.find([';', '\n']).unwrap_or(rest.len());
    let path = rest[..end].split(" ?:").next().unwrap_or_default();
    Some(clean_path(path.trim()))
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
/// The two-patterns-one-field ending is the part the plain ladder cannot express,
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
        // same pattern and must not be mistaken for one.
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
    let Some((local, first)) = local_and_ctx_path(script) else {
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
            map.shift_remove(&key);
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
pub(crate) fn painless_literal(text: &str) -> Option<Value> {
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
fn try_ladder(event: &mut Event, ladder: &Ladder) -> bool {
    let subject = event
        .get_str(&ladder.subject)
        .map(String::from)
        .or_else(|| event.get_i64(&ladder.subject).map(|n| n.to_string()));

    let Some(subject) = subject else {
        return true;
    };
    let matches = |arm: &&LadderArm| {
        arm.literals.iter().any(|literal| {
            if ladder.fold_case {
                literal.eq_ignore_ascii_case(&subject)
            } else {
                literal == &subject
            }
        })
    };
    if let Some(arm) = ladder.arms.iter().find(matches) {
        for (target, value) in &arm.writes {
            let _ = event.set(target, value.clone());
        }
        // After the writes, as the script orders them: an arm that reads a
        // field into ECS and drops it would otherwise leave the original.
        for path in &arm.removes {
            event.remove(path);
        }
    }
    true
}

/// The source and destination arrays of an append-if-absent script.
///
/// The pattern is `for (x in ctx.A) { if (!ctx.B.contains(x)) ctx.B.add(x) }`,
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

/// `def m = new HashMap(); if (f.containsKey('A')) { m.put('a',
/// f.get('A')); } ...; out.add(m);` over a list, rebuilt under an explicit
/// rename table in place of the vendor's key.
///
/// `identity_protection_assessment`'s `assessmentFactors` is written this way
/// -- an explicit table rather than a blanket case-fold, so a key the script
/// does not list is dropped from the rebuild rather than carried over as-is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListRenameTable {
    source: String,
    target: String,
    /// `(vendor key, renamed key)`, in the order the script tests them.
    renames: Vec<(String, String)>,
}

/// Read the parent local's `remove`/`put` pair and the `containsKey`/`put`
/// renames between them.
fn parse_list_rename_table(script: &str) -> Option<ListRenameTable> {
    let (declaration, _) = script.split_once(" = ctx.")?;
    let parent_local = declaration.rsplit("def ").next()?.trim();
    let parent = ctx_path_bound_to(script, parent_local)?;

    let source_key = script
        .split_once(&format!("{parent_local}.remove('"))?
        .1
        .split('\'')
        .next()?;
    // The FINAL write, not the first -- `.put()` is also how each rebuilt
    // entry is filled in, and only the parent local's own call names the
    // list's new home.
    let target_key = script
        .rsplit(&format!("{parent_local}.put('"))
        .next()?
        .split('\'')
        .next()?;

    let mut renames = Vec::new();
    for segment in script.split(".containsKey('").skip(1) {
        let (old_key, rest) = segment.split_once('\'')?;
        let new_key = rest.split_once(".put('")?.1.split('\'').next()?;
        renames.push((old_key.to_string(), new_key.to_string()));
    }

    (!renames.is_empty()).then(|| ListRenameTable {
        source: format!("{parent}.{source_key}"),
        target: format!("{parent}.{target_key}"),
        renames,
    })
}

/// Rebuild each map in the list under the rename table.
///
/// The vendor script pops the source key UNCONDITIONALLY, before it checks
/// the value is even a list, so a present-but-wrong-typed field is gone
/// afterwards too -- the same `Event::remove` call gives that order for free,
/// and the pattern match declines onto a no-op for anything but an array.
fn run_list_rename_table(event: &mut Event, pattern: &ListRenameTable) -> bool {
    let Some(Value::Array(items)) = event.remove(&pattern.source) else {
        return true;
    };

    let mut rebuilt = Vec::with_capacity(items.len());
    for item in items {
        let Some(entries) = item.as_object() else {
            continue;
        };
        let mut out = Map::new();
        for (old_key, new_key) in &pattern.renames {
            if let Some(value) = entries.get(old_key) {
                out.insert(new_key.clone(), value.clone());
            }
        }
        rebuilt.push(Value::Object(out));
    }

    let _ = event.set(&pattern.target, Value::Array(rebuilt));
    true
}

/// The evidence list a `for (evidence in ctx.<path>)` loop walks.
fn evidence_loop_path(script: &str) -> Option<String> {
    use crate::painless_params::clean_path;

    let at = script.find("for (evidence in ctx.")?;
    let tail = &script[at + "for (evidence in ctx.".len()..];
    let path = clean_path(tail.split(')').next()?);
    (!path.is_empty()).then_some(path)
}

/// Read `ctx.<t> = <v>.substring(0, <v>.toLowerCase().lastIndexOf('<n>'))`.
///
/// m365's device events name an API call `ReadProcessMemoryApiCall` and the
/// pipeline wants the half before the marker. The search is case-INSENSITIVE
/// and the cut is on the ORIGINAL text, so the case of what survives is the
/// vendor's.
fn parse_substring_before_last(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let source = script
        .split_once("= ctx.")
        .and_then(|(_, rest)| rest.split_once(';'))
        .map(|(path, _)| clean_path(path))?;
    let needle = script
        .split_once("lastIndexOf(")
        .and_then(|(_, rest)| quoted_first(rest))?;
    let target = script
        .lines()
        .rev()
        .map(str::trim)
        .find(|line| line.starts_with("ctx.") && line.contains(" = "))
        .and_then(|line| line.split_once(" = "))
        .map(|(target, _)| clean_path(&target["ctx.".len()..]))?;

    (!source.is_empty() && !target.is_empty() && !needle.is_empty()).then_some(
        KnownPattern::SubstringBeforeLast {
            source,
            target,
            needle,
        },
    )
}

/// Cut the source at the last case-insensitive occurrence of the marker.
fn run_substring_before_last(event: &mut Event, source: &str, target: &str, needle: &str) -> bool {
    let Some(text) = event.get_str(source).map(str::to_string) else {
        return true;
    };
    // `substring(0, -1)` throws in Java, so a marker that is not there writes
    // nothing and the rename behind this finds no field.
    let Some(at) = text.to_lowercase().rfind(&needle.to_lowercase()) else {
        return true;
    };
    let _ = event.set(target, json!(&text[..at]));
    true
}

/// A duration in SECONDS becomes nanoseconds, and closes the span it opens.
///
/// proofpoint writes `ctx.event.duration = (int) (secs * 1000000000)` and then
/// `ctx.event.end = start.plus(duration, ChronoUnit.NANOS)`. The cast is the
/// script's own and it is to a 32-bit int, so a duration past ~2.1 seconds
/// wraps there -- reproduced, because the vendor's arithmetic is what decides
/// the value Elasticsearch stores.
fn run_seconds_to_span(event: &mut Event, source: &str) -> bool {
    let Some(seconds) = event.get_f64(source) else {
        return true;
    };
    #[allow(clippy::cast_possible_truncation)]
    let nanos = (seconds * 1_000_000_000.0) as i32;
    let _ = event.set("event.duration", json!(nanos));

    let Some(start) = event.get_str("event.start").map(str::to_string) else {
        return true;
    };
    let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(&start) else {
        return true;
    };
    let Some(end) = parsed.checked_add_signed(chrono::TimeDelta::nanoseconds(i64::from(nanos)))
    else {
        return true;
    };
    let _ = event.set("event.end", json!(render_java_instant(&end.to_utc())));
    true
}

/// A `ZonedDateTime` as Java prints it: no fraction at all when there is none,
/// otherwise three, six or nine digits -- never a partial group.
fn render_java_instant(instant: &chrono::DateTime<chrono::Utc>) -> String {
    use chrono::Timelike;

    let nanos = instant.nanosecond();
    let digits = if nanos == 0 {
        0
    } else if nanos.is_multiple_of(1_000_000) {
        3
    } else if nanos.is_multiple_of(1_000) {
        6
    } else {
        9
    };
    match digits {
        0 => instant.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        3 => instant.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string(),
        6 => instant.format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string(),
        _ => instant.format("%Y-%m-%dT%H:%M:%S%.9fZ").to_string(),
    }
}

/// Every key of one map capitalised, with one prefix that capitalises whole.
///
/// The defender exports disagree with themselves about casing -- one endpoint
/// ships `CveId`, another `cveId` -- so the pipeline folds both to
/// `PascalCase` before the renames. `osPlatform` becomes `OSPlatform` not
/// `OsPlatform`, which is what the prefix exception is for; the script names
/// both halves of it and neither is assumed here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PascalKeys {
    path: String,
    /// The lowercase prefix that gets replaced whole rather than capitalised,
    /// and only when an uppercase letter follows it.
    prefix: String,
    replacement: String,
}

/// Read the prefix exception and the map being rewritten.
fn parse_pascal_keys(script: &str) -> Option<PascalKeys> {
    use crate::painless_params::clean_path;

    let at = script.find(".entrySet()")?;
    let start = script[..at].rfind("ctx.")?;
    let path = clean_path(&script[start + "ctx.".len()..at]);
    if path.is_empty() {
        return None;
    }

    // `key.startsWith("os")` and `newKey = "OS" + key.substring(2)`.
    let (prefix, replacement) = match script.split_once(".startsWith(") {
        Some((_, tail)) => {
            let prefix = quoted_first(tail)?;
            let replacement = script
                .split_once(" = ")
                .and_then(|(_, rest)| rest.split_once("\" +"))
                .and_then(|(head, _)| head.rsplit('"').next())
                .map(str::to_string)?;
            (prefix, replacement)
        }
        None => (String::new(), String::new()),
    };

    Some(PascalKeys {
        path,
        prefix,
        replacement,
    })
}

/// Capitalise every key, honouring the prefix exception.
fn run_pascal_keys(event: &mut Event, pattern: &PascalKeys) -> bool {
    let Some(Value::Object(entries)) = event.get(&pattern.path).cloned() else {
        return true;
    };

    let mut rebuilt = Map::new();
    for (key, value) in entries {
        let tail = (!pattern.prefix.is_empty())
            .then(|| key.strip_prefix(pattern.prefix.as_str()))
            .flatten()
            .filter(|tail| tail.chars().next().is_some_and(char::is_uppercase));
        let renamed = if let Some(tail) = tail {
            format!("{}{tail}", pattern.replacement)
        } else {
            let mut chars = key.chars();
            chars.next().map_or_else(
                || key.clone(),
                |first| first.to_uppercase().chain(chars).collect(),
            )
        };
        rebuilt.insert(renamed, value);
    }
    let _ = event.set(&pattern.path, Value::Object(rebuilt));
    true
}

/// A numeric field's bits decoded into a list of names.
///
/// `aws/vpcflow` and `aws/firewall_logs` both spell out the six TCP flags this
/// way. The masks and names are read off the script rather than assumed to be
/// TCP's, because nothing in the pattern says they must be.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitFlagNames {
    source: String,
    target: String,
    /// Mask and the name it sets, in the order the script tests them --
    /// which is the order the list comes out in.
    flags: Vec<(u64, String)>,
}

/// Read `def flags = Integer.parseUnsignedInt(ctx.<s>)` and the `if ((flags &
/// 0xNN) != 0) { ctx.<t>.add('name'); }` ladder under it.
fn parse_bit_flag_names(script: &str) -> Option<BitFlagNames> {
    use crate::painless_params::clean_path;

    let at = script.find("parseUnsignedInt(ctx.")?;
    let source = script[at + "parseUnsignedInt(ctx.".len()..]
        .split(')')
        .next()?;

    let mut target = None;
    let mut flags = Vec::new();
    for arm in script.split("& 0x").skip(1) {
        let (mask, rest) = arm.split_once(')')?;
        let Ok(mask) = u64::from_str_radix(mask.trim(), 16) else {
            continue;
        };
        // `ctx.<target>.add('name')`
        let Some(add_at) = rest.find(".add(") else {
            continue;
        };
        let Some(ctx_at) = rest[..add_at].rfind("ctx.") else {
            continue;
        };
        let path = clean_path(&rest[ctx_at + "ctx.".len()..add_at]);
        let name = quoted_first(&rest[add_at..])?;
        if target.get_or_insert_with(|| path.clone()) != &path {
            return None;
        }
        flags.push((mask, name));
    }

    (!flags.is_empty()).then(|| BitFlagNames {
        source: clean_path(source),
        target: target.unwrap_or_default(),
        flags,
    })
}

/// Decode the flags, appending to whatever the list already holds.
///
/// The script creates the list when it is absent and adds to it otherwise, so
/// a second decode over the same field extends rather than replaces.
fn run_bit_flag_names(event: &mut Event, decode: &BitFlagNames) -> bool {
    let Some(flags) = event
        .get_str(&decode.source)
        .and_then(|text| text.trim().parse::<u64>().ok())
        .or_else(|| {
            event
                .get_i64(&decode.source)
                .and_then(|n| u64::try_from(n).ok())
        })
    else {
        return true;
    };

    let mut names = match event.get(&decode.target) {
        Some(Value::Array(existing)) => existing.clone(),
        _ => Vec::new(),
    };
    for (mask, name) in &decode.flags {
        if flags & mask != 0 {
            names.push(Value::String(name.clone()));
        }
    }
    let _ = event.set(&decode.target, Value::Array(names));
    true
}

/// A byte's bits named off a fixed array, tested from the high bit down.
///
/// netflow's TCP flags: `flags[N-i]` at loop index `i` counting from `N` down
/// to 1, so the top bit (bit `N-1`) is `flags[0]` and bit 0 is `flags[N-1]`.
/// Nothing is written when no bit is set, matching the script's own
/// `if (flagsSeen.length > 0)` guard -- unlike [`BitFlagNames`], whose
/// vpcflow script creates its target array up front and so writes it even
/// when empty.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitNameArray {
    source: String,
    target: String,
    /// High bit first: `names[0]` fires on the top bit, `names.last()` on bit 0.
    names: Vec<String>,
}

/// Read netflow's TCP-flags decode -- a fixed name array walked bit by bit
/// from the top down. Every literal between the extracted fields is matched
/// exactly, so a script that only shares the `new String[]{` opening falls
/// through instead of losing data silently.
fn parse_bit_name_array(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    let tail = script.strip_prefix("String[] flags = new String[]{")?;
    let (names_raw, tail) = tail.split_once("};")?;
    let names: Vec<String> = names_raw
        .split(',')
        .map(|n| n.trim().trim_matches('"').to_string())
        .collect();
    if names.is_empty() || names.len() > 32 || names.iter().any(String::is_empty) {
        return None;
    }
    let len = names.len();

    let tail = tail
        .trim_start()
        .strip_prefix("ArrayList flagsSeen = new ArrayList();")?;
    let tail = tail.trim_start().strip_prefix("int tcp_flags = ctx.")?;
    let (source_raw, tail) = tail.split_once(';')?;
    let source = clean_path(source_raw);

    let tail = tail
        .trim_start()
        .strip_prefix(&format!("for (int i = {len}; i > 0; --i) {{"))?;
    let tail = tail
        .trim_start()
        .strip_prefix("int value = (tcp_flags & (1 << (i-1))) & 0x0ff;")?;
    let tail = tail.trim_start().strip_prefix("if (value != 0) {")?;
    let tail = tail
        .trim_start()
        .strip_prefix(&format!("flagsSeen.add(flags[{len}-i]);"))?;
    let tail = tail.trim_start().strip_prefix('}')?; // closes the if
    let tail = tail.trim_start().strip_prefix('}')?; // closes the for

    let tail = tail
        .trim_start()
        .strip_prefix("if (flagsSeen.length > 0) {")?;
    let tail = tail.trim_start().strip_prefix("ctx.")?;
    let (target_raw, tail) = tail.split_once('=')?;
    let target = clean_path(target_raw.trim());
    let tail = tail.trim_start().strip_prefix("flagsSeen;")?;
    let tail = tail.trim_start().strip_prefix('}')?;

    // Nothing left but trailing whitespace, or this is a longer script that
    // only opens the same way.
    if !tail.trim().is_empty() || source.is_empty() || target.is_empty() {
        return None;
    }

    Some(KnownPattern::BitNameArray(Box::new(BitNameArray {
        source,
        target,
        names,
    })))
}

/// Decode the bits high to low against the name array, writing nothing when
/// none are set -- the script's own `if (flagsSeen.length > 0)` guard.
fn run_bit_name_array(event: &mut Event, pattern: &BitNameArray) -> bool {
    let Some(bits) = event.get_as_i64(&pattern.source) else {
        return true;
    };
    let len = pattern.names.len();
    let mut seen = Vec::with_capacity(len);
    for (index, name) in pattern.names.iter().enumerate() {
        let bit = (len - 1 - index) as u32;
        if bits & (1i64 << bit) != 0 {
            seen.push(Value::String(name.clone()));
        }
    }
    if !seen.is_empty() {
        let _ = event.set(&pattern.target, Value::Array(seen));
    }
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
/// Everything the split needs is a property of the script alone, so it is
/// resolved once into here. Reading it per event meant seven allocations
/// before the payload was even looked at.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplitKv {
    source: String,
    target: String,
    /// Between one pair and the next.
    field_sep: char,
    /// Between a key and its value. Spelled as a string because the vendor
    /// writes it quoted and it is compared with `split_once`.
    pair_sep: String,
}

fn parse_split_unquoted_kv(script: &str) -> Option<SplitKv> {
    use crate::painless_params::clean_path;

    let mut calls = script.split("splitUnquoted(").skip(1);
    // The definition, then the call that splits the whole payload into tokens.
    let _definition = calls.next()?;
    let fields = calls.next()?;
    let source = fields
        .strip_prefix("ctx.")
        .and_then(|rest| rest.split(',').next())?;

    // The per-token split is whichever helper takes the loop variable. Newer
    // pipelines call `splitOnceByToken`, older ones `splitUnquoted` again.
    let pairs = ["splitOnceByToken(", "splitUnquoted("]
        .iter()
        .find_map(|helper| script.split(helper).find(|s| s.starts_with("arr[")))?;

    let field_sep = quoted_first(fields)?.chars().next()?;
    let pair_sep = quoted_first(pairs)?;
    let target = crate::painless_params::ctx_path_before(script, " = map")?;

    Some(SplitKv {
        source: clean_path(source),
        target: clean_path(&target),
        field_sep,
        pair_sep,
    })
}

/// Split the payload and write the pairs, with nothing re-read off the script.
///
/// A fragment without the pair separator is skipped, which is what the
/// vendor's `kv.length == 2` guard does.
fn run_split_unquoted_kv(event: &mut Event, split: &SplitKv) -> bool {
    // A missing source is not a failure -- the processor's `if` guards it.
    let Some(payload) = event.get_str(&split.source).map(str::to_string) else {
        return true;
    };

    let mut map = Map::new();
    for token in split_unquoted(&payload, split.field_sep) {
        let Some((key, value)) = token.split_once(split.pair_sep.as_str()) else {
            continue;
        };
        map.insert(
            key.trim().to_string(),
            json!(value.trim().trim_matches('"')),
        );
    }
    let _ = event.set(&split.target, Value::Object(map));
    true
}

/// Split on `separator`, ignoring any occurrence inside double quotes.
///
/// Borrows from `input`. Returning owned tokens cost one `String` per field,
/// and a fortigate line carries thirty of them -- the caller keeps only the
/// halves either side of the pair separator, so nothing needed copying.
fn split_unquoted(input: &str, separator: char) -> Vec<&str> {
    let mut buffer = [0u8; 4];
    let separator_str: &str = separator.encode_utf8(&mut buffer);
    let mut out = Vec::new();
    let mut start = 0;
    let mut in_quotes = false;

    for (i, c) in input.char_indices() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c == separator && !in_quotes {
            let token = input[start..i].trim();
            if !token.is_empty() {
                out.push(token);
            }
            start = i + c.len_utf8();
        }
    }
    let last = input[start..].trim();
    if !last.is_empty() && last != separator_str {
        out.push(last);
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
    (0..bytes.len())
        .find(|at| bytes[*at] == b'=' && assigns_at(bytes, *at))
        .map(|at| at + 1)
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
/// A field split or flattened into the list at another path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppendEach {
    source: String,
    target: String,
    /// The two object keys the map branch joins, when the script has them.
    map_keys: Vec<String>,
    separator: String,
}

/// Read the paths, keys and separator the append uses, or decline.
fn parse_append_each(script: &str) -> Option<AppendEach> {
    use crate::painless_params::ctx_path_before;

    Some(AppendEach {
        target: ctx_path_before(script, ".add(")?,
        source: append_source_path(script)?,
        map_keys: quoted_after(script, "tag["),
        separator: quoted_after(script, ".splitOnToken(")
            .into_iter()
            .next()
            .unwrap_or_else(|| ",".to_string()),
    })
}

fn try_append_each(event: &mut Event, pattern: &AppendEach) -> bool {
    use crate::painless_params::clean_path;

    let AppendEach {
        source,
        target,
        map_keys,
        separator,
    } = pattern;

    // A missing source is not a failure -- the processor's `if` guards it.
    let entries: Vec<Value> = match event.get(source) {
        Some(Value::String(s)) => s
            .split(separator.as_str())
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

    let path = clean_path(target);
    let mut existing = match event.get(&path) {
        Some(Value::Array(arr)) => arr.clone(),
        _ => Vec::new(),
    };
    existing.extend(entries);
    let _ = event.set(&path, Value::Array(existing));
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

/// What the loop writes once a list member's key holds one of the literals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberWrite {
    /// `ctx.<target> = true;` inside the loop and `= false;` after it, so the
    /// answer is whether ANY member matched and never which one.
    Flag(String),
    /// The FIRST matching member's `<member>` copied to `<target>`, and
    /// nothing written when no member matches.
    Copy { member: String, target: String },
}

/// `for (def <item>: ctx.<list>) { if (<item>.<key> == '<v1>' || <item>.<key> == '<v2>' ...) { <write> return; } }`
///
/// One list, one member compared against string literals, one write on a hit.
/// crowdstrike derives `has_script_or_module_ioc` from `ioc_context` as the
/// flag; `rapid7_insightvm` takes `vulnerability.scanner.name` off the
/// `unique_identifiers` entry whose `source` is the agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListMemberSelect {
    list: String,
    key: String,
    values: Vec<String>,
    write: MemberWrite,
}

/// Either spelling of the write, over the one selection.
fn parse_list_member_select(script: &str) -> Option<ListMemberSelect> {
    parse_list_member_flag(script).or_else(|| parse_list_member_copy(script))
}

/// The loop variable here has no space before its colon (`c:` not `c :`), unlike [`for_binding`].
fn parse_list_member_flag(script: &str) -> Option<ListMemberSelect> {
    let (_, rest) = script.split_once("for (def ")?;
    let (item, rest) = rest.split_once(':')?;
    let item = item.trim();
    let list = clean_path(
        rest.trim_start()
            .strip_prefix("ctx.")?
            .split(')')
            .next()?
            .trim(),
    );

    // Every arm compares the SAME member; a mixed member name is not this pattern.
    let (_, after_if) = script.split_once(&format!("if ({item}."))?;
    let (member, _) = after_if.split_once(" == ")?;
    let member = member.trim();
    if member.is_empty() || member.contains([' ', '(', ')']) {
        return None;
    }

    let values = quoted_after(script, &format!("{item}.{member} == "));
    if values.is_empty() {
        return None;
    }

    let true_target = ctx_assignment_target_before(script, " = true;")?;
    let false_target = ctx_assignment_target_before(script, " = false;")?;
    (true_target == false_target).then_some(ListMemberSelect {
        list,
        key: member.to_string(),
        values,
        write: MemberWrite::Flag(true_target),
    })
}

/// The copy spelling, read STRUCTURALLY rather than by `contains`.
///
/// The loop stops on its hit, so this reproduces exactly one write and any
/// other statement in the body is work it would silently drop. The whole
/// script therefore has to be the loop: only map-ensure scaffolding before it,
/// nothing after it, and a body that is one guard around one write and the
/// `return` that ends the walk.
fn parse_list_member_copy(script: &str) -> Option<ListMemberSelect> {
    let (item, list, body, head, tail) = for_loop_parts(script)?;
    if !tail.trim().is_empty() || !only_map_ensures(head) {
        return None;
    }

    let guard = body.trim().strip_prefix("if (")?;
    let (cond, rest) = split_at_close_paren(guard)?;
    let inner = rest
        .trim()
        .strip_prefix('{')?
        .trim_end()
        .strip_suffix('}')?;

    let (key, values) = parse_member_equality(cond, &item)?;

    // `<write>; return;` and nothing else -- `break` reads the same, because
    // the loop is the last statement either way.
    let statements: Vec<&str> = inner
        .split(';')
        .map(str::trim)
        .filter(|statement| !statement.is_empty())
        .collect();
    let [write, "return" | "break"] = statements.as_slice() else {
        return None;
    };
    let (member, target) = parse_member_copy_write(write, &item)?;

    Some(ListMemberSelect {
        list,
        key,
        values,
        write: MemberWrite::Copy { member, target },
    })
}

/// The loop variable, the list it walks, its body, and the text either side.
///
/// Takes the colon with or without a space before it, unlike [`for_binding`],
/// and hands back the BODY so a matcher can refuse a loop that does more than
/// the one thing it reproduces.
fn for_loop_parts(script: &str) -> Option<(String, String, &str, &str, &str)> {
    let (head, rest) = script.split_once("for (def ")?;
    let (item, rest) = rest.split_once(':')?;
    let item = item.trim();
    if item.is_empty() || !item.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    let (walked, rest) = rest.split_once(')')?;
    let list = clean_path(walked.trim().strip_prefix("ctx.")?.trim());
    if list.is_empty() {
        return None;
    }

    let rest = rest.trim_start();
    let close = matching_brace(rest)?;
    Some((
        item.to_string(),
        list,
        &rest[1..close],
        head,
        &rest[close + 1..],
    ))
}

/// The offset of the `}` closing the `{` that `text` opens with.
///
/// Quoted text is stepped over: a brace inside a string literal would
/// otherwise end the body early and leave the rest of the script unread.
fn matching_brace(text: &str) -> Option<usize> {
    if !text.starts_with('{') {
        return None;
    }
    let mut depth = 0usize;
    let mut quote: Option<char> = None;
    for (at, c) in text.char_indices() {
        match (quote, c) {
            (Some(open), c) if c == open => quote = None,
            (None, '\'' | '"') => quote = Some(c),
            (None, '{') => depth += 1,
            (None, '}') => {
                depth -= 1;
                if depth == 0 {
                    return Some(at);
                }
            }
            _ => {}
        }
    }
    None
}

/// The condition and the rest, split at the `)` that closes it.
///
/// Quoted text is stepped over, so a literal carrying a bracket cannot end the
/// condition early.
fn split_at_close_paren(text: &str) -> Option<(&str, &str)> {
    let mut depth = 1usize;
    let mut quote: Option<char> = None;
    for (at, c) in text.char_indices() {
        match (quote, c) {
            (Some(open), c) if c == open => quote = None,
            (None, '\'' | '"') => quote = Some(c),
            (None, '(') => depth += 1,
            (None, ')') => {
                depth -= 1;
                if depth == 0 {
                    return Some((&text[..at], &text[at + 1..]));
                }
            }
            _ => {}
        }
    }
    None
}

/// `<item>.<key> == '<v1>' || <item>.<key> == '<v2>' ...`, one key throughout.
fn parse_member_equality(cond: &str, item: &str) -> Option<(String, Vec<String>)> {
    let mut key: Option<String> = None;
    let mut values = Vec::new();
    for clause in cond.split("||") {
        let (lhs, rhs) = clause.split_once("==")?;
        let named = lhs.trim().strip_prefix(item)?.strip_prefix('.')?.trim();
        if named.is_empty()
            || !named
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
        {
            return None;
        }
        if key.get_or_insert_with(|| named.to_string()) != named {
            return None;
        }
        let literal = quoted_first(rhs)?;
        // A quote that opens and never closes would land here as the rest of
        // the clause, which is not a literal the vendor wrote.
        if rhs.trim() != format!("'{literal}'") && rhs.trim() != format!("\"{literal}\"") {
            return None;
        }
        values.push(literal);
    }
    Some((key?, values))
}

/// `ctx.<a>.<b>.put('<leaf>', <item>.<member>)` or `ctx.<target> = <item>.<member>`.
fn parse_member_copy_write(statement: &str, item: &str) -> Option<(String, String)> {
    let member_of = |value: &str| {
        let member = value.trim().strip_prefix(item)?.strip_prefix('.')?.trim();
        (!member.is_empty()
            && member
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '.'))
        .then(|| member.to_string())
    };

    if let Some((receiver, args)) = statement.split_once(".put(") {
        let container = painless_path(receiver)?;
        if !receiver.trim_start().starts_with("ctx") {
            return None;
        }
        let args = args.trim_end().strip_suffix(')')?;
        let (leaf, value) = args.split_once(',')?;
        let leaf = quoted_first(leaf)?;
        if leaf.is_empty() {
            return None;
        }
        return Some((member_of(value)?, format!("{container}.{leaf}")));
    }

    let (lhs, rhs) = statement.split_once('=')?;
    if !lhs.trim_start().starts_with("ctx") {
        return None;
    }
    Some((member_of(rhs)?, painless_path(lhs)?))
}

/// Every statement is `ctx.<p> = ctx.<p> ?: [:];`, which creates the target's
/// parent maps and changes nothing else.
fn only_map_ensures(preamble: &str) -> bool {
    preamble
        .split(';')
        .map(str::trim)
        .filter(|statement| !statement.is_empty())
        .all(|statement| {
            let Some((target, rhs)) = statement.split_once('=') else {
                return false;
            };
            let Some((kept, empty)) = rhs.split_once("?:") else {
                return false;
            };
            let target = target.trim();
            target.starts_with("ctx.")
                && target == kept.trim()
                && matches!(empty.trim(), "[:]" | "new HashMap()")
        })
}

/// A list item's member, by its dotted name.
///
/// The whole name is tried first, because a vendor that flattened its payload
/// carries `a.b` as one key rather than as two levels.
fn item_member<'a>(item: &'a Value, path: &str) -> Option<&'a Value> {
    if let Some(value) = item.get(path) {
        return Some(value);
    }
    let mut current = item;
    for segment in path.split('.') {
        current = current.get(segment)?;
    }
    Some(current)
}

/// The loop exits on the first hit, so the flag is `list.any(key in values)` --
/// never which element -- and the copy takes that first element's member.
fn run_list_member_select(event: &mut Event, pattern: &ListMemberSelect) -> bool {
    let Some(Value::Array(items)) = event.get(&pattern.list) else {
        return true;
    };
    let matched = |item: &&Value| {
        item_member(item, &pattern.key)
            .and_then(Value::as_str)
            .is_some_and(|held| pattern.values.iter().any(|want| want == held))
    };
    match &pattern.write {
        MemberWrite::Flag(target) => {
            let hit = items.iter().any(|item| matched(&item));
            let _ = event.set(target, json!(hit));
        }
        MemberWrite::Copy { member, target } => {
            let hit = items
                .iter()
                .find(matched)
                .and_then(|item| item_member(item, member).cloned());
            if let Some(value) = hit {
                let _ = event.set(target, value);
            }
        }
    }
    true
}

/// `for (def item : ctx.<table>) { if (item.<key> == ctx.<subject>) { ... } }`
/// followed by a chain of fallback assignments to the same target.
///
/// Cisco IOS's timezone map is the pattern, and at 89 hits it was the single
/// largest unhandled script in the corpus. The table lives in `ctx`, not in
/// `params`, because the deployment supplies it -- so the mapping is data the
/// matcher READS, never a table transcribed into Rust.
/// The parts of a row lookup that the script text alone decides.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowLookup {
    item: String,
    table: String,
    key: String,
    subject: String,
    value_col: String,
    target: Option<String>,
}

/// Read the loop, its match condition and the column a hit yields.
///
/// Every rejection here is structural, so the pattern can be decided from the
/// text and no longer has to be claimed on a `for (def ` and a `" : ctx."`
/// appearing somewhere in the same script.
fn parse_row_lookup(script: &str) -> Option<RowLookup> {
    use crate::painless_params::clean_path;

    let (item, table) = for_binding(script)?;
    // `item.<key> == ctx.<subject>` names the column and the field to match.
    let (key, subject) = script
        .split_once(&format!("if ({item}."))
        .and_then(|(_, rest)| rest.split_once(')'))
        .and_then(|(cond, _)| cond.split_once("=="))?;
    let subject = subject.trim().strip_prefix("ctx.").map(clean_path)?;
    // A hit either assigns the column or, since the vendor wrapped this in a
    // function, RETURNS it. Reading only the assignment form left cisco_ios's
    // timezone chain unmatched from the first character.
    let value_col = script
        .split_once(&format!("= {item}."))
        .or_else(|| script.split_once(&format!("return {item}.")))
        .map(|(_, rest)| rest.trim_end_matches(';'))
        .and_then(|rest| rest.split([';', '\n']).next())?;
    // The return form assigns nothing on a hit, so there is no target to find
    // ahead of it; the fallback arms below name their own.
    let target = ctx_assignment_target_before(script, &format!("= {item}."));

    Some(RowLookup {
        item,
        table,
        key: key.trim().to_owned(),
        subject,
        value_col: value_col.trim().to_owned(),
        target,
    })
}

fn try_row_lookup_with_fallback(event: &mut Event, script: &str, pattern: &RowLookup) -> bool {
    let RowLookup {
        table,
        key,
        subject,
        value_col,
        target,
        ..
    } = pattern;
    let (key, value_col) = (key.as_str(), value_col.as_str());
    let target = target.clone();

    let wanted = event.get_as_string(subject);
    if let (Some(wanted), Some(Value::Array(rows))) = (&wanted, event.get(table)) {
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

    // Two spellings: `entry.getValue() == '<s>'` chains, and the security
    // pipeline's `[null, "", "-", ...].contains(entry.getValue())` list.
    let mut drops_null = false;
    let mut sentinels = quoted_after(script, "entry.getValue() == ");
    if sentinels.is_empty()
        && let Some(at) = script.find("].contains(entry.getValue())")
        && let Some(open) = script[..at].rfind('[')
    {
        let list = &script[open + 1..at];
        sentinels = quoted_members(list);
        drops_null = list.contains("null");
    }
    let drops_odd_keys = script.contains("entry.getKey()") && script.contains(r"\W+");
    if sentinels.is_empty() && !drops_odd_keys && !drops_null {
        return false;
    }

    if let Some(Value::Object(map)) = crate::painless_params::pointer_mut(event, &path) {
        map.retain(|k, v| {
            let sentinel = (drops_null && v.is_null())
                || v.as_str().is_some_and(|s| sentinels.iter().any(|x| x == s));
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
/// Read the two ends and the field the direction lands on.
///
/// The helper's own three ranges -- 10.0.0.0/8, 172.16.0.0/12 and
/// 192.168.0.0/16 -- are exactly the set `Ipv4Addr::is_private` answers for.
fn parse_private_cidr_direction(script: &str) -> Option<KnownPattern> {
    let mut ends = Vec::new();
    let mut cursor = 0usize;
    while let Some(rel) = script[cursor..].find("isPrivateCIDR(ctx.") {
        let at = cursor + rel + "isPrivateCIDR(ctx.".len();
        cursor = at;
        let path = clean_path(script[at..].split(')').next()?);
        if !path.is_empty() && !ends.contains(&path) {
            ends.push(path);
        }
    }
    let [source, destination] = ends.as_slice() else {
        return None;
    };

    let target = crate::painless_params::ctx_writes(script)
        .into_iter()
        .find(|(_, rhs)| rhs.trim().trim_matches(['\'', '"']) == "inbound")
        .map(|(path, _)| path)?;

    Some(KnownPattern::PrivateCidrDirection {
        source: source.clone(),
        destination: destination.clone(),
        target,
    })
}

/// Whether an address sits in one of the three private IPv4 ranges.
///
/// The script's `CIDR.contains` throws on anything that is not IPv4 and its
/// `catch` answers false, so an IPv6 address is not private rather than an
/// error.
fn is_private_v4(value: &str) -> bool {
    value
        .parse::<std::net::Ipv4Addr>()
        .is_ok_and(|ip| ip.is_private())
}

fn run_private_cidr_direction(
    event: &mut Event,
    source: &str,
    destination: &str,
    target: &str,
) -> bool {
    let (Some(from), Some(to)) = (
        event.get_as_string(source),
        event.get_as_string(destination),
    ) else {
        return true;
    };
    let direction = match (is_private_v4(&from), is_private_v4(&to)) {
        (false, true) => "inbound",
        (true, false) => "outbound",
        (true, true) => "internal",
        (false, false) => "external",
    };
    let _ = event.set(target, Value::String(direction.to_string()));
    true
}

/// Read every `if (ctx.<guard> == "<literal>") { ... }` branch and the guarded
/// copies inside it.
///
/// The branches of one chain test the same field against different literals, so
/// at most one can hold and `else if` needs no separate modelling.
fn parse_branch_copies(script: &str) -> Option<Vec<BranchCopy>> {
    let mut branches = Vec::new();
    let mut cursor = 0usize;
    while let Some(rel) = script[cursor..].find("== \"") {
        let at = cursor + rel;
        cursor = at + "== \"".len();

        let Some(guard) = guard_subject(script, &script[..at]) else {
            continue;
        };
        let Some(literal) = script[cursor..].split('"').next().map(str::to_string) else {
            continue;
        };
        let body = branch_body(&script[cursor + literal.len() + 1..]);
        let copies = guarded_copies(body);
        if !copies.is_empty() {
            branches.push(BranchCopy {
                guard,
                literal,
                copies,
            });
        }
    }
    (!branches.is_empty()).then_some(branches)
}

/// What the branch actually compares, resolved through any locals.
///
/// Taking the last `ctx.` path before the `==` is right only when the script
/// compares a field inline. gdacs reads the geometry into a local and tests a
/// member of it --
///
/// ```painless
/// def polyGeom = ctx.polygon_geometry;
/// String polyType = polyGeom.type;
/// if (polyType == "Polygon") { ... }
/// ```
///
/// -- where the nearest `ctx.` path is `polygon_geometry`, an OBJECT. Guarding
/// on that compares a map against `"Polygon"` and no branch can ever hold, so
/// every copy inside was silently dead.
fn guard_subject(script: &str, head: &str) -> Option<String> {
    // The token immediately before the comparison, not the last path anywhere.
    let subject = head
        .trim_end()
        .trim_end_matches('=')
        .trim_end()
        .rsplit(['(', ' ', '!', '&', '|'])
        .next()?
        .trim();
    if subject.is_empty() {
        return None;
    }
    resolve_subject(script, subject, 3)
}

/// Follow an expression back to the ctx path it reads, through local bindings.
///
/// gdacs takes two hops -- `polyType` is `polyGeom.type` and `polyGeom` is
/// `ctx.polygon_geometry` -- and the member access has to survive both, or the
/// guard tests the container instead of the field. The depth bound is what
/// stops a self-referential binding looping.
fn resolve_subject(script: &str, expression: &str, depth: u8) -> Option<String> {
    if expression.starts_with("ctx.") || expression.starts_with("ctx[") {
        return painless_path(expression);
    }
    let depth = depth.checked_sub(1)?;

    let (name, suffix) = expression
        .split_once('.')
        .map_or((expression, ""), |(head, tail)| (head, tail));
    let bound = local_expression(script, name)?;
    let base = resolve_subject(script, bound.trim(), depth)?;
    Some(if suffix.is_empty() {
        base
    } else {
        crate::painless_params::clean_path(&format!("{base}.{suffix}"))
    })
}

/// Whatever a local was last declared to hold, as written.
fn local_expression<'a>(script: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!(" {name} = ");
    let at = script.find(&needle)?;
    let rest = &script[at + needle.len()..];
    let end = rest.find([';', '\n']).unwrap_or(rest.len());
    Some(&rest[..end])
}

/// The text of the `{ ... }` block a branch opens.
fn branch_body(rest: &str) -> &str {
    let Some(open) = rest.find('{') else {
        return "";
    };
    let mut depth = 0usize;
    for (at, c) in rest[open..].char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &rest[open + 1..open + at];
                }
            }
            _ => {}
        }
    }
    ""
}

/// `if (ctx.<src> != null) { ctx.<dst> = ctx.<src>; }` pairs, as (dst, src).
/// Split at the first `=` that ASSIGNS, skipping the comparison operators.
///
/// A guard that says more than `!= null` -- gdacs writes
/// `if (ctx.a != null && ctx.a != "")` -- put the `=` of the second `!=`
/// first, so the target came out as the guard's own left-hand side with its
/// namespace lost: `gdacs.polygon_label` was read as `polygon_label`, and the
/// copy then wrote a field Elasticsearch never carries.
fn split_assignment_once(text: &str) -> Option<(&str, &str)> {
    let bytes = text.as_bytes();
    for (at, _) in text.match_indices('=') {
        let previous = at.checked_sub(1).map(|i| bytes[i]);
        let next = bytes.get(at + 1).copied();
        if matches!(previous, Some(b'!' | b'=' | b'<' | b'>')) || next == Some(b'=') {
            continue;
        }
        return Some((&text[..at], &text[at + 1..]));
    }
    None
}

fn guarded_copies(body: &str) -> Vec<(String, String)> {
    let mut copies = Vec::new();
    for part in body.split("!= null").skip(1) {
        let Some((target_expr, value_expr)) = split_assignment_once(part) else {
            continue;
        };
        // The value ends at its own statement: a body of several copies runs
        // straight into the next guard otherwise, and the path read is the
        // FOLLOWING field's.
        let Some(value_expr) = value_expr.split(';').next() else {
            continue;
        };
        let (Some(target), Some(value)) = (painless_path(target_expr), painless_path(value_expr))
        else {
            continue;
        };
        copies.push((target, value));
    }
    copies
}

fn run_branch_copies(event: &mut Event, branches: &[BranchCopy]) -> bool {
    for branch in branches {
        if event.get_as_string(&branch.guard).as_deref() != Some(branch.literal.as_str()) {
            continue;
        }
        for (target, source) in &branch.copies {
            if let Some(value) = event.get(source).cloned()
                && !value.is_null()
            {
                let _ = event.set(target, value);
            }
        }
    }
    true
}

fn try_guarded_copy(event: &mut Event, script: &str, literals: &Program) -> bool {
    // Every guarded copy in the script, not just the first. Windows'
    // `security_standard` is four hundred lines of them -- one per winlog
    // field, each in its own `if (... != null) { ... }` with a null-guard
    // preamble -- and taking only the first claimed the script and lost the
    // rest.
    if literals.run(event) {
        return true;
    }

    let Some(copy) = parse_single_copy(script) else {
        return false;
    };

    if let Some(v) = event.get(&copy.source).cloned()
        && !v.is_null()
    {
        let _ = event.set(&copy.target, v);
    }
    true
}

/// One `if (ctx.<source> != null) { ctx.<target> = ctx.<source>; }`.
struct SingleCopy {
    source: String,
    target: String,
}

/// Read the one copy the fallback path handles, or decline.
///
/// Split out so the ladder arm can ask the same question the runner would,
/// rather than claiming a script and discovering per event that it cannot read
/// it.
fn parse_single_copy(script: &str) -> Option<SingleCopy> {
    let (cond, rest) = script.split_once("!= null")?;
    let source = painless_path(cond)?;
    // The body starts where the CONDITION ends. Splitting at the first `=`
    // after `!= null` cut a two-clause guard in half: aws/ec2_metrics gates on
    // `&& ctx.host?.cpu?.usage == null` and then divides the source in place,
    // so the guard's own field was read as the target and the RAW percentage
    // was copied onto it -- 42 where the agent had already written 0.421.
    let body = condition_body(cond, rest);
    let (target_expr, value_expr) = body.split_once('=')?;
    let (target, value) = (painless_path(target_expr)?, painless_path(value_expr)?);
    (value == source).then_some(SingleCopy { source, target })
}

/// What follows a guard's closing parenthesis.
///
/// `head` is the text before the `!= null` and says how deep the parentheses
/// are there -- one for the `if (` of a plain guard, more where the guard
/// itself calls something. `rest` is walked until they balance, and what is
/// left is the body. A guard that never closes (or was never open) leaves
/// `rest` as it stands, which is what the reading used to do everywhere.
fn condition_body<'a>(head: &str, rest: &'a str) -> &'a str {
    let Some(mut depth) = head
        .matches('(')
        .count()
        .checked_sub(head.matches(')').count())
        .filter(|open| *open > 0)
    else {
        return rest;
    };
    for (index, c) in rest.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return &rest[index + c.len_utf8()..];
                }
            }
            _ => {}
        }
    }
    rest
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

/// `ctx.<target> = ctx.<source> * <literal>`, resolved once from the script.
///
/// Source and target are read separately because they are usually different --
/// zscaler scales `zscaler_zia.dns.duration.milliseconds` INTO `event.duration`
/// rather than in place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaleField {
    source: String,
    target: String,
    factor: Factor,
}

impl ScaleField {
    /// Build one from resolved parts, for a caller that already knows them.
    #[must_use]
    pub fn new(source: impl Into<String>, target: impl Into<String>, factor: Factor) -> Self {
        Self {
            source: source.into(),
            target: target.into(),
            factor,
        }
    }
}

/// A multiply's literal, carrying the Painless TYPE it was written with.
///
/// The type decides the PRODUCT's: `n * 1000000000L` is a long and `n * 1e9`
/// is a double, and Elasticsearch writes the difference --
/// `symantec_endpoint`'s scan duration comes back as `600000000000.0`, not
/// `600000000000`, so a long here fails the comparison on every scan event.
#[derive(Debug, Clone, Copy)]
pub enum Factor {
    Long(i64),
    Double(f64),
}

impl PartialEq for Factor {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Long(one), Self::Long(two)) => one == two,
            // Bit equality, so the pattern stays `Eq` and the lock can order it.
            (Self::Double(one), Self::Double(two)) => one.to_bits() == two.to_bits(),
            _ => false,
        }
    }
}

impl Eq for Factor {}

/// Read the multiply and the assignment that owns it.
///
/// The assignment is found by walking BACK from the multiply to the last real
/// `=`, because the script is not reliably one statement per line -- the
/// vendor folds it with YAML's `>-` and it arrives as a single line. Taking
/// the FIRST `=` instead found the `==` of an `if (ctx.event == null)`
/// preamble, so the target read as `event` and nothing was written: 25 zscaler
/// events with no `event.duration` at all.
fn parse_scale_field(script: &str) -> Option<ScaleField> {
    let (mut head, factor) = script.rsplit_once('*')?;
    let mut factor = literal_factor(factor)?;

    // A CHAIN of literal factors is ONE factor: endace writes `* 60 * 1000`
    // and zoom `* 60L * 1000000000L`, and reading only the last one scaled
    // both by a fraction of what the pipeline meant. The walk stops at the
    // first operand that is not a literal, which is the source path.
    while let Some((rest, next)) = head.rsplit_once('*') {
        let Some(next) = literal_factor(next) else {
            break;
        };
        factor = multiply_factors(factor, next);
        head = rest;
    }

    let at = last_assignment(head)?;
    let target = painless_path(&head[..at])?;
    let source = painless_path(&head[at + 1..])?;

    // Source and target may be the SAME field: scaling in place is the older
    // spelling and four vendored scripts still use it.
    (!target.is_empty() && !source.is_empty()).then_some(ScaleField {
        source,
        target,
        factor,
    })
}

/// One multiply's right-hand operand, when it is a numeric literal.
///
/// `L` is Java's long suffix, which checkpoint writes on its 1e9 constant;
/// `symantec_endpoint` writes the same magnitude as `1e9` and gets a double.
fn literal_factor(text: &str) -> Option<Factor> {
    let literal = text.trim().trim_end_matches([';', ')', ' ']).trim();
    match literal.trim_end_matches(['L', 'l']).trim().parse::<i64>() {
        Ok(long) => Some(Factor::Long(long)),
        Err(_) => literal.parse::<f64>().ok().map(Factor::Double),
    }
}

/// Fold two literal factors, a double on either side making the product one.
#[allow(clippy::cast_precision_loss)]
fn multiply_factors(one: Factor, two: Factor) -> Factor {
    match (one, two) {
        (Factor::Long(a), Factor::Long(b)) => Factor::Long(a.saturating_mul(b)),
        (Factor::Double(a), Factor::Double(b)) => Factor::Double(a * b),
        (Factor::Long(a), Factor::Double(b)) | (Factor::Double(b), Factor::Long(a)) => {
            Factor::Double(a as f64 * b)
        }
    }
}

/// The byte offset of the last `=` that ASSIGNS, rather than compares.
///
/// `==`, `!=`, `<=`, `>=` and the compound arithmetic forms are all reads.
pub(crate) fn last_assignment(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    text.char_indices()
        .rev()
        .find_map(|(at, c)| (c == '=' && assigns_at(bytes, at)).then_some(at))
}

/// Whether the `=` at `at` ASSIGNS rather than compares or accumulates.
///
/// `==`, `!=`, `<=` and `>=` compare. `+=` and its arithmetic siblings assign
/// too, but not to the path on their left: reading `ctx.a += ".0"` as an
/// assignment yields the write target `a +`, which `fortinet`'s tls version
/// ships and which one of the three scanners that shared this rule used to
/// return.
pub(crate) fn assigns_at(bytes: &[u8], at: usize) -> bool {
    let before = at.checked_sub(1).map(|i| bytes[i]);
    let compares = matches!(
        before,
        Some(b'=' | b'!' | b'<' | b'>' | b'+' | b'-' | b'*' | b'/')
    ) || bytes.get(at + 1) == Some(&b'=');
    !compares
}

/// Multiply the source into the target.
pub fn scale_field(event: &mut Event, pattern: &ScaleField) -> bool {
    if let Some(n) = event.get_as_i64(&pattern.source) {
        let scaled = match pattern.factor {
            Factor::Long(factor) => json!(n.saturating_mul(factor)),
            // Painless widens the long to a double BEFORE multiplying, so the
            // rounding is the same one Elasticsearch published.
            #[allow(clippy::cast_precision_loss)]
            Factor::Double(factor) => json!(n as f64 * factor),
        };
        let _ = event.set(&pattern.target, scaled);
        return true;
    }

    // A FRACTIONAL source, which the integer read above declines. gitlab times
    // a request in seconds -- 0.03275 -- and reading it as an integer scaled
    // nothing at all, leaving the raw value where 32,750,000 belonged.
    let fractional = event.get(&pattern.source).and_then(|value| match value {
        Value::String(text) => text.parse::<f64>().ok(),
        other => other.as_f64(),
    });
    if let Some(n) = fractional {
        #[allow(clippy::cast_precision_loss)]
        let factor = match pattern.factor {
            Factor::Long(factor) => factor as f64,
            Factor::Double(factor) => factor,
        };
        let _ = event.set(&pattern.target, json!(n * factor));
    }
    true
}

/// A recursive walk normalising every suspected timestamp under one subtree to
/// milliseconds, keyed on the FIELD NAME rather than on the value.
///
/// `amazon_security_lake` and `aws_securityhub` ship the same script over their
/// own subtree, so both the root and the suffixes come off the text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MillisecondLadder {
    root: String,
    suffixes: Vec<String>,
}

/// Read the walk's root and the name suffixes it converts.
///
/// Every rung of the vendor's magnitude ladder has to be present, because the
/// arithmetic below is hard-coded: a script laddering to different thresholds
/// must fall through rather than have this one's conversion applied to it.
fn parse_millisecond_ladder(script: &str) -> Option<MillisecondLadder> {
    const RUNGS: [&str; 7] = [
        "1e19",
        "1e16",
        "1e13",
        "1e10",
        "/ 1000000",
        "/ 1000",
        "* 1000",
    ];
    if !RUNGS.iter().all(|rung| script.contains(rung)) {
        return None;
    }

    let mut suffixes: Vec<String> = Vec::new();
    for tail in script.split(".endsWith(").skip(1) {
        let rest = tail.trim_start();
        let Some(quote) = rest.chars().next().filter(|c| matches!(c, '\'' | '"')) else {
            continue;
        };
        let inner = &rest[quote.len_utf8()..];
        let Some(end) = inner.find(quote) else {
            continue;
        };
        let literal = &inner[..end];
        if !literal.is_empty() && !suffixes.iter().any(|s| s == literal) {
            suffixes.push(literal.to_string());
        }
    }
    if suffixes.is_empty() {
        return None;
    }

    // The walk's entry point is the LAST `(ctx.`; the helpers above it name no
    // path at all.
    let at = script.rfind("(ctx.")?;
    let root = script[at + "(ctx.".len()..].split(')').next()?;
    if root.is_empty()
        || !root
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '?'))
    {
        return None;
    }

    Some(MillisecondLadder {
        root: crate::painless_params::clean_path(root),
        suffixes,
    })
}

/// Normalise every suspected timestamp under the walk's root.
fn run_millisecond_ladder(event: &mut Event, pattern: &MillisecondLadder) -> bool {
    if let Some(root) = crate::painless_params::pointer_mut(event, &pattern.root) {
        scale_suspected_timestamps(root, &pattern.suffixes);
    }
    true
}

/// The vendor's `processFields` recurses into Maps ONLY, which is why a
/// `created_time` inside a list keeps whatever unit it arrived in.
fn scale_suspected_timestamps(value: &mut Value, suffixes: &[String]) {
    let Value::Object(entries) = value else {
        return;
    };
    for (name, member) in entries {
        if member.is_object() {
            scale_suspected_timestamps(member, suffixes);
        } else if suffixes
            .iter()
            .any(|suffix| name.ends_with(suffix.as_str()))
            && let Some(millis) = as_milliseconds(member)
        {
            *member = Value::from(millis);
        }
    }
}

/// One number read as milliseconds from its own magnitude.
///
/// The vendor's `1e19` throw is unreachable: `(long)1e19` saturates to
/// `Long.MAX_VALUE` in Java, so only that exact value trips it.
fn as_milliseconds(value: &Value) -> Option<i64> {
    // Painless truncates through `Number.longValue()`, so a fractional value
    // converts as its whole part.
    #[allow(clippy::cast_possible_truncation)]
    let number = value
        .as_i64()
        .or_else(|| value.as_f64().map(|n| n as i64))?;

    Some(if number >= 10_000_000_000_000_000 {
        number / 1_000_000
    } else if number >= 10_000_000_000_000 {
        number / 1_000
    } else if number >= 10_000_000_000 {
        number
    } else {
        // Java's long arithmetic wraps rather than trapping.
        number.wrapping_mul(1_000)
    })
}

/// Named epoch fields rescaled to MILLISECONDS by their own magnitude.
///
/// Above `1e18` the value is nanoseconds and divides by `1e6`; below `1e10` it
/// is seconds and multiplies by `1e3`; between the two it is already
/// milliseconds. Every cloudflare stream runs this ahead of a `UNIX_MS` date
/// processor, so an unclaimed script leaves epoch seconds to be read as
/// milliseconds -- and a nanosecond value out of range of any date at all,
/// which fails the date processor and raises an error Elasticsearch does not.
///
/// Distinct from [`MillisecondLadder`], which walks a whole SUBTREE choosing
/// fields by name suffix over a four-rung ladder. This one converts the fields
/// the script names, on the vendor's two rungs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EpochToMillis {
    /// Each field, and whether a zero is written as null instead of rescaled.
    /// spectrum's connect and disconnect times use zero for "never happened".
    fields: Vec<(String, bool)>,
    /// Whether a STRING is read through `Long.parseLong` first. Only the
    /// logpull spelling does; the rest guard on `instanceof Number` and leave
    /// a string for the date processor's own `ISO8601` rung.
    parse_strings: bool,
}

/// The helper the multi-field spelling declares before its guarded blocks.
const CONVERT_TO_MILLIS: &str = "def convertToMillis(long timestamp) { \
     if (timestamp > (long)(1e18)) { return timestamp/(long)(1e6) } \
     else if (timestamp < (long)(1e10)) { return timestamp*(long)(1e3) } \
     return timestamp }";

/// A script with every run of whitespace collapsed to one space and `?.`
/// written as `.`, so one template can carry both path spellings.
fn collapse_script(script: &str) -> String {
    let mut out = String::with_capacity(script.len());
    for word in script.split_whitespace() {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(word);
    }
    out.replace("?.", ".")
}

/// A dotted path, ending where `terminator` begins, or `None` where what is
/// there is not a path at all.
fn epoch_path<'a>(text: &'a str, terminator: &str) -> Option<&'a str> {
    let path = text.split(terminator).next()?;
    let readable = !path.is_empty()
        && path
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_'));
    readable.then_some(path)
}

/// The whole single-field script this pattern would have to be, for `path`.
///
/// The parse works by REBUILDING the text and comparing it, rather than
/// reading statement by statement: a vendor edit to a threshold or a factor
/// then declines here instead of having this runner's hard-coded arithmetic
/// applied to a ladder that no longer means it.
fn epoch_inline_script(path: &str, wrapped: bool) -> String {
    let ladder = format!(
        "if (t > (long)(1e18)) {{ ctx.{path} = t/(long)(1e6) }} \
         else if (t < (long)(1e10)) {{ ctx.{path} = t*(long)(1e3) }}"
    );
    if wrapped {
        format!(
            "try {{ long t; if (ctx.{path} instanceof String) {{ \
             t = Long.parseLong(ctx.{path}); }} else if (ctx.{path} instanceof Number) {{ \
             t = (long)(ctx.{path}); }} else {{ return; }} {ladder} }} catch (Exception e) {{}}"
        )
    } else {
        format!("long t = (long)(ctx.{path}); {ladder}")
    }
}

/// One guarded block of the multi-field spelling, and the text after it.
fn epoch_block(text: &str) -> Option<((String, bool), &str)> {
    let path = epoch_path(text.strip_prefix("if (ctx.")?, " ")?;
    let rest = text.strip_prefix(&format!(
        "if (ctx.{path} != null && ctx.{path} instanceof Number) {{ "
    ))?;

    let plain = format!("ctx.{path} = convertToMillis(ctx.{path}); }}");
    if let Some(tail) = rest.strip_prefix(&plain) {
        return Some(((path.to_owned(), false), tail));
    }

    let zeroed = format!(
        "if (ctx.{path} == 0) {{ ctx.{path} = null; }} \
         else {{ ctx.{path} = convertToMillis(ctx.{path}); }} }}"
    );
    let tail = rest.strip_prefix(&zeroed)?;
    Some(((path.to_owned(), true), tail))
}

/// Read the fields the vendor's three spellings of this conversion name.
fn parse_epoch_to_millis(script: &str) -> Option<EpochToMillis> {
    let text = collapse_script(script);

    if let Some(rest) = text.strip_prefix("long t = (long)(ctx.") {
        let path = epoch_path(rest, ")")?;
        return (text == epoch_inline_script(path, false)).then(|| EpochToMillis {
            fields: vec![(path.to_owned(), false)],
            parse_strings: false,
        });
    }

    // The same conversion wrapped in a try/catch, which reaches a value that
    // arrived as a string because no `convert` processor runs ahead of it.
    if let Some(rest) = text.strip_prefix("try { long t; if (ctx.") {
        let path = epoch_path(rest, " ")?;
        return (text == epoch_inline_script(path, true)).then(|| EpochToMillis {
            fields: vec![(path.to_owned(), false)],
            parse_strings: true,
        });
    }

    // The multi-field spelling: one helper, then a guarded block per field.
    // Every block has to parse, so a block this cannot read declines the whole
    // script rather than converting the fields either side of it.
    let mut rest = text.strip_prefix(CONVERT_TO_MILLIS)?.trim_start();
    let mut fields = Vec::new();
    while !rest.is_empty() {
        let (field, tail) = epoch_block(rest)?;
        fields.push(field);
        rest = tail.trim_start();
    }
    (!fields.is_empty()).then_some(EpochToMillis {
        fields,
        parse_strings: false,
    })
}

/// Rescale each named field to epoch milliseconds.
fn run_epoch_to_millis(event: &mut Event, pattern: &EpochToMillis) -> bool {
    for (field, null_on_zero) in &pattern.fields {
        // Painless truncates a number through `(long)`, and a string reaches
        // the cast only in the spelling that parses one.
        #[allow(clippy::cast_possible_truncation)]
        let number = match event.get(field) {
            Some(Value::String(text)) if pattern.parse_strings => text.parse::<i64>().ok(),
            Some(Value::String(_)) | None => None,
            Some(other) => other.as_i64().or_else(|| other.as_f64().map(|n| n as i64)),
        };
        let Some(number) = number else {
            continue;
        };
        if *null_on_zero && number == 0 {
            let _ = event.set(field, Value::Null);
            continue;
        }
        let millis = if number > 1_000_000_000_000_000_000 {
            number / 1_000_000
        } else if number < 10_000_000_000 {
            // Java's long arithmetic wraps rather than trapping.
            number.wrapping_mul(1_000)
        } else {
            number
        };
        let _ = event.set(field, Value::from(millis));
    }
    true
}

/// `double <v> = ((Number) ctx.<source>).doubleValue(); ctx.<target> = (long)
/// Math.round(<v> * <factor>);`
///
/// `identity_protection_assessment`'s 0-1 risk score becomes ECS's 0-100
/// scale this way, and `ScaleField`'s plain integer multiply cannot read it --
/// the cast keeps a fractional score alive long enough for `Math.round` to
/// act on, where a plain `(long)` truncation of the product would drop the
/// top of every band instead of rounding into it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RoundedScale {
    source: String,
    target: String,
    factor: i64,
}

/// Read the cast source, the rounded target, and the whole-number factor
/// between them.
fn parse_rounded_scale(script: &str) -> Option<RoundedScale> {
    let source = script
        .split_once("(Number)")
        .and_then(|(_, rest)| rest.split_once(".doubleValue()"))
        .and_then(|(path, _)| painless_path(path))?;

    let (head, tail) = script.split_once("Math.round(")?;
    let at = last_assignment(head)?;
    let target = painless_path(&head[..at])?;

    let factor = tail
        .split_once(')')?
        .0
        .split_once('*')?
        .1
        .trim()
        .parse::<f64>()
        .ok()?;
    if factor.fract() != 0.0 {
        return None;
    }
    #[allow(clippy::cast_possible_truncation)]
    let factor = factor as i64;

    Some(RoundedScale {
        source,
        target,
        factor,
    })
}

/// Round rather than truncate: `Math.round` keeps `0.999 * 100` at the top of
/// its band where a plain `(long)` cast on the product would drop it.
fn run_rounded_scale(event: &mut Event, pattern: &RoundedScale) -> bool {
    let Some(v) = event.get_f64(&pattern.source) else {
        return true;
    };
    #[allow(clippy::cast_precision_loss)]
    let scaled = v * pattern.factor as f64;
    #[allow(clippy::cast_possible_truncation)]
    let rounded = scaled.round() as i64;
    let _ = event.set(&pattern.target, json!(rounded));
    true
}

/// `ctx.<target> = ChronoUnit.NANOS.between(<start>, <end>)`, resolved once.
///
/// Both ends are LOCALS, each bound earlier to `ZonedDateTime.parse(ctx.<path>)`,
/// so the paths are recovered from those declarations rather than the call.
///
/// The span itself is bound to a local in half the vendored spellings, and
/// copied onto its field afterwards under a sign guard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NanosBetween {
    start: String,
    end: String,
    target: String,
    /// The script's own `if (<local> >= 0)`, which suppresses a reversed span.
    non_negative: bool,
}

/// The `ctx.` path a local takes its instant from.
fn parsed_instant_source(script: &str, name: &str) -> Option<String> {
    let needle = format!(" {name} = ");
    let at = script.find(&needle)? + needle.len();
    let rest = &script[at..];
    let end = rest.find([';', '\n']).unwrap_or(rest.len());
    painless_path(&rest[..end])
}

/// The local a statement declares, or None when it assigns a `ctx.` path.
fn assigned_local(fragment: &str) -> Option<&str> {
    let statement = fragment
        .trim_end()
        .rsplit([';', '\n', '{', '}'])
        .next()?
        .trim();
    if statement.starts_with("ctx") {
        return None;
    }
    statement.split_whitespace().next_back()
}

/// The `ctx.` path a local is later copied onto: `ctx.<target> = <local>;`.
fn local_copied_to(script: &str, local: &str) -> Option<String> {
    let needle = format!("= {local}");
    let mut from = 0;
    while let Some(at) = script[from..].find(&needle) {
        let absolute = from + at;
        let after = script.as_bytes().get(absolute + needle.len()).copied();
        // A longer name that merely starts with this one is a different local.
        if !matches!(after, Some(c) if c.is_ascii_alphanumeric() || c == b'_') {
            return painless_path(&script[..absolute]);
        }
        from = absolute + needle.len();
    }
    None
}

fn parse_nanos_between(script: &str) -> Option<NanosBetween> {
    let (head, args) = script.split_once("ChronoUnit.NANOS.between(")?;
    let (call, tail) = args.split_once(')')?;
    let (first, second) = call.split_once(',')?;

    let at = last_assignment(head)?;

    // `painless_path` alone takes the last `ctx.` path ANYWHERE before the
    // assignment, which on the local form is the `end` declaration rather than
    // the field being written.
    let (target, non_negative) = match assigned_local(&head[..at]) {
        Some(local) => (
            local_copied_to(tail, local)?,
            tail.contains(&format!("{local} >= 0")),
        ),
        None => (painless_path(&head[..at])?, false),
    };

    Some(NanosBetween {
        start: parsed_instant_source(head, first.trim())?,
        end: parsed_instant_source(head, second.trim())?,
        target,
        non_negative,
    })
}

/// Nanoseconds since the epoch, for a field holding an ISO-8601 instant.
fn instant_nanos(event: &Event, path: &str) -> Option<i64> {
    chrono::DateTime::parse_from_rfc3339(event.get_str(path)?)
        .ok()?
        .timestamp_nanos_opt()
}

/// The `ctx.` path the script's FIRST local is bound to.
fn first_ctx_binding(script: &str) -> Option<String> {
    let rest = &script[script.find("= ctx.")?..];
    let end = rest.find([';', '\n']).unwrap_or(rest.len());
    painless_path(&rest[..end])
}

/// A ladder of `if (<subject>.contains('<needle>')) { ctx.<target> = <value>; }`
///
/// One value, and a type TAG that says which field it belongs on: defender's
/// machine actions carry a hash and name its algorithm separately.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContainsLadder {
    subject: String,
    value: String,
    lower: bool,
    /// `(the literal the subject must contain, the field the value lands on)`.
    arms: Vec<(String, String)>,
}

/// The expression a `<type> <local> = ...;` declaration binds.
fn declared_expression<'a>(script: &'a str, local: &str) -> Option<&'a str> {
    let needle = format!(" {local} = ");
    let tail = &script[script.find(&needle)? + needle.len()..];
    Some(&tail[..tail.find([';', '\n']).unwrap_or(tail.len())])
}

/// A `ctx.` path with the vendor's no-op string calls taken off the end.
fn ctx_path_of(expression: &str) -> Option<String> {
    let mut path = expression.trim();
    loop {
        let before = path;
        for call in [".toLowerCase()", ".toUpperCase()", ".toString()", ".trim()"] {
            path = path.trim_end_matches(call);
        }
        if path == before {
            break;
        }
    }
    painless_path(path)
}

/// Every `.contains(` in the script must be an arm, so a script that merely
/// spells the call -- `ctx.event.category.contains('network')` -- is refused.
fn parse_contains_ladder(script: &str) -> Option<ContainsLadder> {
    let mut arms = Vec::new();
    let (mut subject_local, mut value_local) = (String::new(), String::new());

    for (at, _) in script.match_indices(".contains(") {
        let local = script[..at]
            .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
            .next()
            .unwrap_or_default();
        let after = &script[at + ".contains(".len()..];
        let quote = after.chars().next().filter(|c| *c == '\'' || *c == '"')?;
        let needle = after[quote.len_utf8()..].split(quote).next()?;

        let body = after.split("} else").next().unwrap_or(after);
        let (target, tail) = body.split_once("ctx.")?.1.split_once(" = ")?;
        let source = tail.split(';').next()?.trim();

        if arms.is_empty() {
            subject_local = local.to_string();
            value_local = source.to_string();
        }
        if local != subject_local || source != value_local {
            return None;
        }
        arms.push((
            needle.to_string(),
            crate::painless_params::clean_path(target),
        ));
    }

    let subject_expression = declared_expression(script, &subject_local)?;
    (arms.len() > 1).then_some(())?;
    Some(ContainsLadder {
        subject: ctx_path_of(subject_expression)?,
        value: ctx_path_of(declared_expression(script, &value_local)?)?,
        lower: subject_expression.contains(".toLowerCase()"),
        arms,
    })
}

fn run_contains_ladder(event: &mut Event, pattern: &ContainsLadder) -> bool {
    let Some(subject) = event.get_str(&pattern.subject).map(str::to_string) else {
        return true;
    };
    let subject = if pattern.lower {
        subject.to_lowercase()
    } else {
        subject
    };
    let Some(value) = event.get(&pattern.value).cloned() else {
        return true;
    };

    for (needle, target) in &pattern.arms {
        if subject.contains(needle) {
            let _ = event.set(target, value);
            break;
        }
    }
    true
}

/// How one item of a list is cut for a column.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ItemSlice {
    /// `item.substring(0, item.lastIndexOf('<sep>'))`
    BeforeLast(String),
    /// `item.substring(item.indexOf('<open>')+1, item.indexOf('<close>'))`
    Between(String, String),
}

impl ItemSlice {
    fn cut<'a>(&self, item: &'a str) -> Option<&'a str> {
        match self {
            Self::BeforeLast(sep) => item.rsplit_once(sep.as_str()).map(|(head, _)| head),
            Self::Between(open, close) => item
                .split_once(open.as_str())
                .and_then(|(_, rest)| rest.split_once(close.as_str()))
                .map(|(inside, _)| inside),
        }
    }
}

/// One list cut two ways: `m365_defender` splits `Valid Accounts (T1078)`
/// into the technique's name and its id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliceEachItem {
    source: String,
    /// `(the field the column lands on, how each item is cut)`.
    columns: Vec<(String, ItemSlice)>,
}

/// The call's arguments, split at the comma OUTSIDE any quoted literal --
/// `indexOf('(')` puts a bracket inside quotes and depth-counting trips on it.
fn split_call_arguments(args: &str) -> Option<(&str, &str)> {
    let mut quote = None;
    for (at, c) in args.char_indices() {
        match (quote, c) {
            (Some(open), c) if c == open => quote = None,
            (None, '\'' | '"') => quote = Some(c),
            (None, ',') => return Some((args[..at].trim(), args[at + 1..].trim())),
            _ => {}
        }
    }
    None
}

/// The quoted literal a one-argument call is passed.
fn call_literal(expression: &str, call: &str) -> Option<String> {
    let rest = expression.split_once(call)?.1;
    let quote = rest.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    Some(rest[quote.len_utf8()..].split(quote).next()?.to_string())
}

fn parse_item_slice(expression: &str, local: &str) -> Option<ItemSlice> {
    let args = expression.strip_prefix(&format!("{local}.substring("))?;
    let args = args.strip_suffix(')')?;
    let (start, end) = split_call_arguments(args)?;

    if start == "0" {
        return Some(ItemSlice::BeforeLast(call_literal(end, ".lastIndexOf(")?));
    }
    let open = call_literal(start.strip_suffix("+1")?, ".indexOf(")?;
    Some(ItemSlice::Between(open, call_literal(end, ".indexOf(")?))
}

fn parse_slice_each_item(script: &str) -> Option<SliceEachItem> {
    let (head, tail) = script.split_once(" in ctx.")?;
    let local = head.rsplit_once("for (")?.1.trim();
    let source = crate::painless_params::clean_path(tail.split(')').next()?);

    // `<bucket>.add(<local>.substring(...));` then `ctx.<target> = <bucket>;`.
    let mut columns = Vec::new();
    for (at, _) in script.match_indices(".add(") {
        let bucket = script[..at]
            .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
            .next()
            .unwrap_or_default();
        let call = script[at + ".add(".len()..].split(";\n").next()?.trim();
        let slice = parse_item_slice(call.strip_suffix(')')?, local)?;
        let target = ctx_path_before_assignment(script, bucket)?;
        columns.push((target, slice));
    }

    (columns.len() > 1).then_some(SliceEachItem { source, columns })
}

/// The `ctx.` path a bucket is assigned to, from `ctx.<path> = <bucket>;`.
fn ctx_path_before_assignment(script: &str, bucket: &str) -> Option<String> {
    let at = script.find(&format!(" = {bucket};"))?;
    painless_path(&script[..at])
}

/// A cut that misses on ANY item leaves every column unwritten: Painless
/// throws on `substring(0, -1)` and the whole processor fails there.
fn run_slice_each_item(event: &mut Event, pattern: &SliceEachItem) -> bool {
    let Some(Value::Array(items)) = event.get(&pattern.source) else {
        return true;
    };
    let items: Vec<String> = items
        .iter()
        .filter_map(|item| item.as_str().map(str::to_string))
        .collect();

    let cut_all = |slice: &ItemSlice| -> Option<Value> {
        items
            .iter()
            .map(|item| slice.cut(item).map(|cut| Value::String(cut.to_string())))
            .collect::<Option<Vec<Value>>>()
            .map(Value::Array)
    };

    let Some(written) = pattern
        .columns
        .iter()
        .map(|(target, slice)| cut_all(slice).map(|column| (target, column)))
        .collect::<Option<Vec<_>>>()
    else {
        return true;
    };
    for (target, column) in written {
        let _ = event.set(target, column);
    }
    true
}

/// Parallel columns collected off a list of objects, written as one map.
///
/// entra id builds a manager's direct reports this way: one column per member
/// a report carries, and a column that stays EMPTY is left out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectColumns {
    source: String,
    /// `(the key the column is written under, the member it reads)`.
    columns: Vec<(String, String)>,
    target: String,
    /// The key the map is put under, and the wrapper it sits inside.
    name: String,
    inner: Option<String>,
}

fn parse_collect_columns(script: &str) -> Option<CollectColumns> {
    let (head, tail) = script.split_once(" : ctx.")?;
    let local = head.rsplit_once("for (def ")?.1.trim();
    let source = crate::painless_params::clean_path(tail.split(')').next()?);

    // `<bucket>.add(<local>.<member>)` names the member a bucket collects.
    let mut buckets: Vec<(&str, &str)> = Vec::new();
    let mut rest = script;
    while let Some(at) = rest.find(".add(") {
        let bucket = rest[..at]
            .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
            .next()
            .unwrap_or_default();
        rest = &rest[at + ".add(".len()..];
        if let Some(member) = rest
            .split(')')
            .next()
            .and_then(|arg| arg.trim().strip_prefix(&format!("{local}.")))
        {
            buckets.push((bucket, member));
        }
    }

    // `<map>.put("<key>", <bucket>)` names the key each bucket goes under.
    let mut columns = Vec::new();
    for piece in script.split(".put(\"").skip(1) {
        let Some((key, after)) = piece.split_once('"') else {
            continue;
        };
        let value = after.trim_start_matches([',', ' ']).split(')').next();
        if let Some((_, member)) = buckets
            .iter()
            .find(|(bucket, _)| value.is_some_and(|v| v.trim() == *bucket))
        {
            columns.push((key.to_string(), (*member).to_string()));
        }
    }

    let at = script.rfind(".put(\"")?;
    let (name, after) = script[at + ".put(\"".len()..].split_once('"')?;
    let target = painless_path(&script[..at])?;
    let inner = after
        .split_once("[\"")
        .and_then(|(_, rest)| rest.split('"').next())
        .map(str::to_string);

    (columns.len() > 1).then_some(CollectColumns {
        source,
        columns,
        target,
        name: name.to_string(),
        inner,
    })
}

fn run_collect_columns(event: &mut Event, pattern: &CollectColumns) -> bool {
    let Some(Value::Array(items)) = event.get(&pattern.source) else {
        return true;
    };
    let items = items.clone();

    let mut built = serde_json::Map::new();
    for (key, member) in &pattern.columns {
        let values: Vec<Value> = items
            .iter()
            .filter_map(|item| item.get(member).filter(|held| !held.is_null()).cloned())
            .collect();
        if !values.is_empty() {
            built.insert(key.clone(), Value::Array(values));
        }
    }
    if built.is_empty() {
        return true;
    }

    let value = match &pattern.inner {
        Some(inner) => {
            let mut wrapper = serde_json::Map::new();
            wrapper.insert(inner.clone(), Value::Object(built));
            Value::Object(wrapper)
        }
        None => Value::Object(built),
    };
    let _ = event.set(&format!("{}.{}", pattern.target, pattern.name), value);
    true
}

/// `def i = ctx.<source>.lastIndexOf("<sep>"); if (i > -1) { ctx.<target> =
/// ctx.<source>.substring(i+1); }` -- a file extension, and its separator.
///
/// Source, target and separator all come off the script. It is guarded by the
/// parse rather than the trigger: the basename helper spells `lastIndexOf` on
/// a PARAMETER, so no `ctx.` path precedes it and it falls through.
fn parse_suffix_after_separator(script: &str) -> Option<(String, String, String)> {
    let (binding, _) = script.split_once(".lastIndexOf(")?;
    let separator = quoted_after(script, ".lastIndexOf(").into_iter().next()?;
    let source = painless_path(binding)?;
    let local = binding.rsplit_once(" = ")?.0.rsplit(' ').next()?.trim();

    // The cut is matched WHOLE. Reading the source and the target off separate
    // statements claimed panw's url and file scripts and cost 176 events.
    let cut = format!("ctx.{source}.substring({local}+1)");
    let at = script.find(&cut)?;
    let target = painless_path(&script[..last_assignment(&script[..at])?])?;
    (source != target && !separator.is_empty()).then_some((source, target, separator))
}

/// The same cut written INLINE -- the path repeated rather than bound to a
/// local, and landing back on the field it read.
///
/// ```painless
/// ctx.process.name = ctx.process.name.substring(ctx.process.name.lastIndexOf('\\') + 1);
/// ```
///
/// Neither matcher beside this one reaches it: [`parse_suffix_after_separator`]
/// wants a local and rejects a target equal to its source, and the helper form
/// wants a `def`. Carbon Black spells every basename this way.
fn parse_inline_suffix_cut(script: &str) -> Option<(String, String, String)> {
    // ONE statement, so a longer script that merely contains this text is not
    // claimed on the strength of it.
    let statement = script.trim().trim_end_matches([';', '\n']).trim();
    if statement.contains(';') {
        return None;
    }
    let (assigned, cut) = statement.split_once(" = ")?;
    let target = painless_path(assigned)?;
    let (subject, argument) = cut.split_once(".substring(")?;
    let source = painless_path(subject)?;

    // The cut has to start one past THAT field's own last separator.
    let (indexed, offset) = argument.split_once(".lastIndexOf(")?;
    if painless_path(indexed)? != source {
        return None;
    }
    let separator = painless_unescape(&quoted_first(offset)?);
    let rest = offset.split_once(')')?.1;
    if separator.is_empty() || rest.trim().trim_end_matches(')').trim() != "+ 1" {
        return None;
    }
    Some((source, target, separator))
}

/// `int i = ctx.<s>.lastIndexOf('<sep>'); if (i != -1) { ctx.<t> =
/// ctx.<s>.substring(i + 1); } else { ctx.<t> = ctx.<s>; }`
///
/// The index lives in a LOCAL, which is what separates this from the inline
/// spellings. The guard on it compares that local against -1, and the guard
/// evaluator cannot read a local, so without this parse the script binds a
/// pattern whose guard never holds and the else arm writes the whole path as the
/// basename -- 99 `jamf_protect_telemetry` call sites, and no corpus capture
/// to catch it.
fn parse_local_index_basename(script: &str) -> Option<(String, String, String)> {
    let (declaration, rest) = script.split_once(".lastIndexOf(")?;
    // `int <local> = ctx.<source>` -- the name sits before the assignment, and
    // the path after it, so the split has to come first or the last
    // whitespace-separated token is the path rather than the local.
    let (named, assigned) = declaration.rsplit_once(" = ")?;
    let local = named.rsplit(char::is_whitespace).next()?.trim();
    let source = painless_path(assigned)?;

    let separator = painless_unescape(&quoted_first(rest)?);
    if separator.is_empty() {
        return None;
    }

    // The cut has to read the SAME field and start one past that local.
    let (before_cut, after_cut) = rest.split_once(".substring(")?;
    if painless_path(before_cut)? != source {
        return None;
    }
    if after_cut.split_once(')')?.0.trim() != format!("{local} + 1") {
        return None;
    }

    let target = painless_path(before_cut.rsplit_once('=')?.0)?;
    Some((source, target, separator))
}

/// A Painless string literal's escapes, resolved.
///
/// A Windows path separator reaches us as `'\\'` and is ONE backslash; cutting
/// on the two characters matches nothing.
fn painless_unescape(literal: &str) -> String {
    let mut out = String::with_capacity(literal.len());
    let mut chars = literal.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some(escaped) => out.push(escaped),
            None => out.push('\\'),
        }
    }
    out
}

/// The text after the source's LAST separator, where there is one.
fn run_suffix_after_separator(
    event: &mut Event,
    source: &str,
    target: &str,
    sep: &str,
    whole_when_absent: bool,
) -> bool {
    let Some(text) = event.get_str(source) else {
        return true;
    };
    match text.rsplit_once(sep) {
        Some((_, suffix)) if !suffix.is_empty() => {
            let suffix = suffix.to_string();
            let _ = event.set(target, json!(suffix));
        }
        // No separator: only the spelling with an else arm writes anything.
        _ if whole_when_absent => {
            let whole = text.to_string();
            let _ = event.set(target, json!(whole));
        }
        _ => {}
    }
    true
}

/// The first bracketed list of quoted strings, as its items.
fn first_string_list(script: &str) -> Option<Vec<String>> {
    let mut rest = script;
    while let Some(open) = rest.find('[') {
        let body = &rest[open + 1..];
        let close = body.find(']')?;
        let items: Vec<String> = body[..close]
            .split(',')
            .filter_map(|item| {
                let item = item.trim();
                let quote = item.chars().next().filter(|c| *c == '"' || *c == '\'')?;
                Some(item.trim_matches(quote).to_string())
            })
            .collect();
        if !items.is_empty() {
            return Some(items);
        }
        rest = &body[close..];
    }
    None
}

/// Which of a fixed set of keys a map marks present, collected as a list.
///
/// `m365_defender` reads DNS header flags this way: seven ECS names, and the ones
/// whose value in `additional_fields` is the STRING `"true"` become the list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagsPresent {
    source: String,
    target: String,
    keys: Vec<String>,
    wanted: String,
}

fn parse_flags_present(script: &str) -> Option<FlagsPresent> {
    let at = last_assignment(script)?;
    Some(FlagsPresent {
        source: first_ctx_binding(script)?,
        target: painless_path(&script[..at])?,
        keys: first_string_list(script)?,
        wanted: quoted_after(script, "] == ").into_iter().next()?,
    })
}

/// The list is written even when EMPTY: the vendor's assignment is unconditional
/// and the pipeline's own cleanup is what removes it again.
fn run_flags_present(event: &mut Event, pattern: &FlagsPresent) -> bool {
    let Some(map) = event.get_object(&pattern.source) else {
        return true;
    };
    let flags: Vec<&String> = pattern
        .keys
        .iter()
        .filter(|key| map.get(*key).and_then(Value::as_str) == Some(pattern.wanted.as_str()))
        .collect();
    let _ = event.set(&pattern.target, json!(flags));
    true
}

/// One column of a zip: the key it writes, the list it reads, and its cast.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ZipColumn {
    key: String,
    source: String,
    to_long: bool,
}

/// Parallel lists zipped into a list of objects, one object per index.
///
/// `m365_defender` pairs the DNS answers with their TTLs this way. Lists of
/// different lengths are the script's own error case, and it names the message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZipLists {
    columns: Vec<ZipColumn>,
    target: String,
    mismatch: Option<String>,
}

fn parse_zip_lists(script: &str) -> Option<ZipLists> {
    let body = script.split_once(".add([")?.1.split_once("])")?.0;
    let mut columns = Vec::new();
    for part in body.split(',') {
        let (key, expr) = part.split_once(':')?;
        let local = expr.trim().trim_start_matches("(long)").trim();
        columns.push(ZipColumn {
            key: key.trim().trim_matches(['"', '\'']).to_string(),
            source: ctx_path_bound_to(script, local.split('[').next()?.trim())?,
            to_long: expr.contains("(long)"),
        });
    }
    let at = last_assignment(script)?;
    (columns.len() > 1).then_some(ZipLists {
        columns,
        target: painless_path(&script[..at])?,
        mismatch: quoted_after(script, "message.add(").into_iter().next(),
    })
}

#[allow(clippy::cast_possible_truncation)]
fn run_zip_lists(event: &mut Event, pattern: &ZipLists) -> bool {
    let mut lists = Vec::with_capacity(pattern.columns.len());
    for column in &pattern.columns {
        let Some(Value::Array(items)) = event.get(&column.source) else {
            return true;
        };
        lists.push(items.clone());
    }
    if lists.iter().any(Vec::is_empty) {
        return true;
    }

    let rows = lists[0].len();
    if lists.iter().any(|items| items.len() != rows) {
        if let Some(message) = &pattern.mismatch {
            let _ = event.append("error.message", json!(message));
        }
        // A column shorter than the first is an index Painless cannot reach, so
        // the script throws there and writes nothing at all.
        if lists.iter().any(|items| items.len() < rows) {
            return true;
        }
    }

    let mut out = Vec::with_capacity(rows);
    for i in 0..rows {
        let row: serde_json::Map<String, Value> = pattern
            .columns
            .iter()
            .zip(&lists)
            .map(|(column, items)| {
                let held = &items[i];
                let value = match held.as_f64() {
                    Some(n) if column.to_long => json!(n as i64),
                    _ => held.clone(),
                };
                (column.key.clone(), value)
            })
            .collect();
        out.push(Value::Object(row));
    }
    let _ = event.set(&pattern.target, Value::Array(out));
    true
}

/// The executable a command line starts with, as the vendor spells it.
///
/// First whitespace-separated token, its last `/` segment when it is a posix
/// path, and every double quote stripped. A BACKSLASH path is left whole --
/// the script only splits on `/`, so `C:\Windows\notepad.exe` stays as it is.
fn command_line_executable(command: &str) -> Option<String> {
    let first = command.trim().split(' ').next()?;
    let first = first.rsplit('/').next()?;
    let name = first.replace('"', "");
    (!name.is_empty()).then_some(name)
}

/// `process.name` gathered from the names already there plus the executable of
/// every `process.command_line`.
///
/// The vendor collects into a `HashSet` and writes a SCALAR when exactly one name
/// survives and a list otherwise, so the FIELD'S SHAPE depends on the data. A
/// list keeps insertion order here; no corpus event reaches that arm, and Java's
/// own order is a hash order nothing outside the JVM can reproduce.
fn run_process_name_from_command_line(event: &mut Event) -> bool {
    let mut names: Vec<String> = Vec::new();
    let add = |name: String, names: &mut Vec<String>| {
        if !names.contains(&name) {
            names.push(name);
        }
    };

    for existing in string_values(event.get("process.name")) {
        add(existing, &mut names);
    }
    for command in string_values(event.get("process.command_line")) {
        if let Some(executable) = command_line_executable(&command) {
            add(executable, &mut names);
        }
    }

    // An empty set writes `[]`, which the pipeline's own drop-empty pass then
    // removes -- so there is nothing to write.
    match names.len() {
        0 => {}
        1 => {
            let _ = event.set("process.name", json!(names.remove(0)));
        }
        _ => {
            let _ = event.set("process.name", json!(names));
        }
    }
    true
}

/// A field the vendor reads as "String or List of String", flattened.
fn string_values(held: Option<&Value>) -> Vec<String> {
    match held {
        Some(Value::String(text)) => vec![text.clone()],
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(|item| item.as_str().map(str::to_string))
            .collect(),
        _ => Vec::new(),
    }
}

/// The span between two instants, in nanoseconds.
///
/// Either end missing or unparseable leaves the target alone: Painless throws
/// there and the processor carries `ignore_failure`, so the vendor writes
/// nothing either.
fn run_nanos_between(event: &mut Event, pattern: &NanosBetween) -> bool {
    let (Some(start), Some(end)) = (
        instant_nanos(event, &pattern.start),
        instant_nanos(event, &pattern.end),
    ) else {
        return true;
    };
    let span = end - start;
    if pattern.non_negative && span < 0 {
        return true;
    }
    let _ = event.set(&pattern.target, json!(span));
    true
}

/// The lone document a list-or-object field carries, lifted to a sibling.
///
/// The mirror of [`parse_wrap_map_in_list`]: where that levels a lone object UP
/// into a list, this takes a one-element list DOWN to the object, so every
/// processor after it can name one path. `carbonblack_edr` opens with it, and
/// missing it left `json.docs` where the next forty renames all read
/// `json.doc` -- the whole of its 99 events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FirstOrSelf {
    /// The list-or-object field.
    source: String,
    /// Where the single document lands.
    target: String,
    /// The field the script drops once the document has been lifted.
    remove: Option<String>,
}

fn parse_first_or_self(script: &str) -> Option<FirstOrSelf> {
    use crate::painless_params::clean_path;

    let local = script
        .split_once(" instanceof List")?
        .0
        .rsplit(|c: char| c.is_whitespace() || c == '(')
        .next()?;
    if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    // Both arms must be there. A script that only handles the list is some
    // other pattern, and running this one on it would invent the object arm.
    if !script.contains(&format!("{local} instanceof Map")) {
        return None;
    }

    let source = ctx_path_bound_to(script, local)?;
    let head = script.split_once(&format!("= {local}[0]"))?.0;
    let at = head.rfind("ctx.")? + "ctx.".len();
    let target = clean_path(&desubscript(head[at..].trim().trim_end_matches(" =")));
    if source.is_empty() || target.is_empty() {
        return None;
    }

    // `ctx.<parent>.remove('<leaf>')` -- the source under its two halves.
    let remove = script.split_once(".remove(").and_then(|(head, tail)| {
        let leaf = tail.split_once(')')?.0.trim();
        if !leaf.starts_with(['"', '\'']) {
            return None;
        }
        let at = head.rfind("ctx.")? + "ctx.".len();
        let parent = clean_path(&desubscript(&head[at..]));
        Some(format!("{parent}.{}", leaf.trim_matches(['"', '\''])))
    });

    Some(FirstOrSelf {
        source,
        target,
        remove,
    })
}

fn run_first_or_self(event: &mut Event, pattern: &FirstOrSelf) -> bool {
    let lifted = match event.get(&pattern.source) {
        Some(Value::Array(items)) => items.first().cloned(),
        Some(map @ Value::Object(_)) => Some(map.clone()),
        // An empty list or a scalar is the script's own `throw`, which fails
        // the pipeline rather than writing anything. Nothing is removed on
        // that path either, because the throw comes first.
        _ => None,
    };
    let Some(lifted) = lifted else {
        return true;
    };
    let _ = event.set(&pattern.target, lifted);
    if let Some(path) = &pattern.remove {
        event.remove(path);
    }
    true
}

/// Read `def <p> = ctx.<path>; if (<p> instanceof Map) { ctx.<path> = [ <p> ]; }`.
///
/// An XML-shaped payload serialises a repeated element as a LIST when there are
/// several and as a bare object when there is one, so the pipeline levels it
/// before the `foreach` that walks it. Skipping this cost cyberarkpas the whole
/// property: the `foreach` read a map, `{{{_ingest._value.Name}}}` rendered
/// empty, and the set wrote a field with no name and no value.
fn parse_wrap_map_in_list(script: &str) -> Option<String> {
    use crate::painless_params::clean_path;

    let local = script
        .split_once(" instanceof Map")?
        .0
        .rsplit(|c: char| c.is_whitespace() || c == '(')
        .next()?;
    if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    // The wrap itself. `[ <local> ]` and nothing else -- a list built from
    // anything more is some other pattern's business.
    let (head, tail) = script.split_once(" = [")?;
    if tail.split_once(']')?.0.trim() != local {
        return None;
    }
    let assigned = clean_path(head.rsplit_once("ctx.")?.1);

    // The local must be BOUND to the same path, or the script is levelling one
    // field by testing another.
    let bound = clean_path(
        script
            .split_once(&format!("{local} = ctx."))?
            .1
            .split([';', '\n'])
            .next()?,
    );
    (bound == assigned && !assigned.is_empty()).then_some(assigned)
}

/// Read `ctx.<target> = ctx.<source> / <n>;`, with an optional absent-guard.
///
/// `aws/ec2_metrics` and `aws/rds` turn a `CloudWatch` percentage into a
/// fraction in place, and only when the agent has not already written the
/// fraction itself -- that guard is the whole point there, so it is carried
/// rather than assumed. cyberarkpas's monitor writes the same fraction to a
/// DIFFERENT field, spells the divisor `100.0`, and puts no space either side
/// of the slash; each of those alone was enough to miss it, which is why the
/// statement is split apart rather than pattern-matched whole.
fn parse_guarded_divide(script: &str) -> Option<KnownPattern> {
    use crate::painless_params::clean_path;

    // Statements, not lines: a folded YAML scalar puts the whole script on one.
    let statement = script
        .split([';', '\n'])
        .map(str::trim)
        .find(|s| s.starts_with("ctx.") && s.contains('/') && s.contains(" = "))?;
    let (lhs, rhs) = statement.split_once(" = ")?;
    let target = clean_path(lhs.trim().strip_prefix("ctx.")?);

    let (value, divisor) = rhs.trim().rsplit_once('/')?;
    // `100` and `100.0` are the same divisor, and Painless allows a type
    // suffix on either. Read as an INTEGER -- `KnownPattern` derives `Eq`, and no
    // vendor divides by a fraction.
    let literal = divisor
        .trim()
        .trim_end_matches(['L', 'l', 'd', 'D', 'f', 'F']);
    let literal = match literal.split_once('.') {
        Some((whole, fraction)) if fraction.chars().all(|c| c == '0') => whole,
        Some(_) => return None,
        None => literal,
    };
    let divisor = literal.parse::<i64>().ok().filter(|n| *n != 0)?;
    let source = clean_path(value.trim().strip_prefix("ctx.")?);
    // The dividend has to be a plain field path. Read loosely it took endace's
    // `ctx._conf.event.start - ctx._conf.timedelta` whole, claimed the script,
    // and then wrote nothing, because no event carries a field of that name.
    if target.is_empty()
        || source.is_empty()
        || !source
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@'))
    {
        return None;
    }

    // Only a `&&`-joined clause is a GUARD. A bare `if (ctx.host == null)
    // ctx.host = [:];` is map creation, and reading it as a guard made
    // cyberarkpas skip the divide on every event that had a host at all.
    let absent = script.split_once("== null").and_then(|(head, _)| {
        let clause = head.rsplit_once("&&")?.1;
        let at = clause.rfind("ctx.")?;
        let path = clean_path(&clause[at + "ctx.".len()..]);
        (!path.is_empty()
            && path
                .chars()
                .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_')))
        .then_some(path)
    });

    Some(KnownPattern::GuardedDivide {
        target,
        source,
        absent,
        divisor,
    })
}

/// Divide one field by a literal into another, unless the guard is already set.
fn run_guarded_divide(
    event: &mut Event,
    target: &str,
    source: &str,
    absent: Option<&String>,
    divisor: i64,
) -> bool {
    if absent.is_some_and(|path| event.has_value(path)) {
        return true;
    }
    if let Some(value) = event.get_f64(source) {
        #[allow(clippy::cast_precision_loss)]
        let _ = event.set(target, json!(value / divisor as f64));
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

/// Which halves of a syslog PRI one script decomposes, and where it reads it.
///
/// Only the halves the SCRIPT writes get written. A pipeline that sets
/// `severity.code` with a `set` processor and only the facility here (cisco
/// nexus) must not gain a severity from us, and none of the vendor scripts
/// derive the `name` at all -- inventing one is an extra field, not a bonus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyslogPriorityScript {
    /// The vendor field the script reads, where it is not the ECS one.
    source: Option<String>,
    facility: bool,
    severity: bool,
    names: bool,
}

impl SyslogPriorityScript {
    /// Build one from resolved parts, for a caller that already knows them.
    ///
    /// `source` is the vendor field the PRI comes from, or `None` for the ECS
    /// `log.syslog.priority`.
    #[must_use]
    pub fn new(source: Option<String>, facility: bool, severity: bool, names: bool) -> Self {
        Self {
            source,
            facility,
            severity,
            names,
        }
    }
}

/// Read the four constants out of the script once, at the call site.
fn parse_syslog_priority(script: &str) -> SyslogPriorityScript {
    SyslogPriorityScript {
        source: priority_source(script).map(str::to_owned),
        facility: writes_syslog_half(script, "facility"),
        severity: writes_syslog_half(script, "severity"),
        names: script.contains("name"),
    }
}

/// Decompose a syslog PRI into ECS `log.syslog.{facility,severity}.{code,name}`.
///
/// The PRI is read from wherever the script found it: `log.syslog.priority`
/// for the generic pipelines, or a vendor field such as
/// `cisco_nexus.log.priority_number`.
pub fn syslog_priority(event: &mut Event, pattern: &SyslogPriorityScript) -> bool {
    let pri = pattern
        .source
        .as_deref()
        .and_then(|field| read_u16(event, field))
        .or_else(|| read_u16(event, "log.syslog.priority"));

    let Some(pri) = pri else {
        return true;
    };

    let (facility, severity) = crate::syslog_pri::decompose(pri);
    let names = pattern.names;

    if pattern.facility {
        let _ = event.set("log.syslog.facility.code", json!(facility));
        if let Some(name) = names
            .then(|| crate::syslog_pri::facility_name(facility))
            .flatten()
        {
            let _ = event.set("log.syslog.facility.name", json!(name));
        }
    }
    if pattern.severity {
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

/// One test a classify arm makes against the subject string.
#[derive(Debug, Clone, PartialEq, Eq)]
enum StringTest {
    Equals(String),
    StartsWith(String),
    EndsWith(String),
    Contains(String),
}

impl StringTest {
    /// The test, or `None` where the term is a pattern this ladder is not.
    fn parse(term: &str, subject: &str) -> Option<Self> {
        let rest = term.trim().strip_prefix(subject)?;
        if let Some(literal) = rest.trim_start().strip_prefix("==") {
            return quoted_first(literal).map(Self::Equals);
        }
        for (call, build) in [
            (".startsWith(", Self::StartsWith as fn(String) -> Self),
            (".endsWith(", Self::EndsWith),
            (".contains(", Self::Contains),
        ] {
            if let Some(argument) = rest.strip_prefix(call) {
                return quoted_first(argument).map(build);
            }
        }
        None
    }

    fn holds(&self, subject: &str) -> bool {
        match self {
            Self::Equals(literal) => subject == literal,
            Self::StartsWith(prefix) => subject.starts_with(prefix.as_str()),
            Self::EndsWith(suffix) => subject.ends_with(suffix.as_str()),
            Self::Contains(needle) => subject.contains(needle.as_str()),
        }
    }
}

/// One arm of a classify ladder: what the subject must satisfy, and the string
/// locals the arm assigns.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ClassifyArm {
    /// `||` of `&&`, as the vendor writes it: the arm holds when any inner
    /// group holds entirely.
    tests: Vec<Vec<StringTest>>,
    writes: Vec<(String, String)>,
}

impl ClassifyArm {
    fn holds(&self, subject: &str) -> bool {
        self.tests
            .iter()
            .any(|group| group.iter().all(|test| test.holds(subject)))
    }
}

/// One string field classified by an else-if ladder of prefix, suffix and
/// equality tests, the answers held in locals and written out at the end.
///
/// ```painless
/// String m = ctx.message;
/// String a = null;
/// String c = null;
/// if (m == 'Created an API key') { a = 'api_key_created'; }
/// else if (m.startsWith('Deleted Check ')) { a = 'check_deleted'; c = 'check'; }
/// ...
/// if (a != null) {
///   if (ctx.event == null) { ctx.event = new HashMap(); }
///   ctx.event.action = a;
/// }
/// if (c != null) {
///   if (ctx._tmp == null) { ctx._tmp = new HashMap(); }
///   ctx._tmp.cat = c;
/// }
/// ```
///
/// kolide's audit descriptions are classified this way. Without the pattern the
/// `!= null` catch-all claims the script and writes nothing at all -- neither
/// the action nor the `_tmp.cat` that gates every grok block behind it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ClassifyLadder {
    /// The `ctx.` path the subject is read from.
    subject: String,
    arms: Vec<ClassifyArm>,
    /// `(the local, the ctx path its value lands on)`.
    outputs: Vec<(String, String)>,
}

/// Read the subject, the ladder and the trailing writes off the script.
fn parse_classify_ladder(script: &str) -> Option<ClassifyLadder> {
    use crate::painless_params::{clean_path, skip_trivia};

    let (local, subject) = string_bindings(script).into_iter().next()?;
    let mut rest = skip_trivia(&script[script.find("if (")?..]);

    let mut arms = Vec::new();
    loop {
        let (test, block, after) = split_if(rest)?;
        let mut tests = Vec::new();
        for group in test.split("||") {
            tests.push(
                group
                    .split("&&")
                    .map(|term| StringTest::parse(term, &local))
                    .collect::<Option<Vec<_>>>()?,
            );
        }

        let mut writes = Vec::new();
        for statement in block.split(';') {
            let statement = skip_trivia(statement);
            if statement.is_empty() {
                continue;
            }
            let (name, literal) = statement.split_once('=')?;
            writes.push((name.trim().to_string(), quoted_first(literal)?));
        }
        if writes.is_empty() {
            return None;
        }
        arms.push(ClassifyArm { tests, writes });

        let after = skip_trivia(after);
        let Some(tail) = after.strip_prefix("else") else {
            rest = after;
            break;
        };
        rest = skip_trivia(tail);
    }
    // A two-arm chain is an ordinary either/or; the pattern is a classification
    // TABLE, and demanding three keeps it off the smaller scripts.
    if arms.len() < 3 {
        return None;
    }

    let mut outputs = Vec::new();
    while let Some((test, block, after)) = split_if(rest) {
        if let Some(name) = test.trim().strip_suffix("!= null").map(str::trim)
            && let Some(target) = block
                .split_once(&format!("= {name};"))
                .and_then(|(head, _)| head.rsplit_once("ctx."))
                .map(|(_, path)| clean_path(path.trim()))
        {
            outputs.push((name.to_string(), target));
        }
        rest = skip_trivia(after);
    }
    (!outputs.is_empty()).then_some(ClassifyLadder {
        subject: clean_path(&subject),
        arms,
        outputs,
    })
}

/// The test, the block and the tail of the `if (...) { ... }` at the head of
/// `text`, or `None` where `text` does not open with one.
fn split_if(text: &str) -> Option<(&str, &str, &str)> {
    use crate::painless_params::{balanced, skip_trivia};

    let after = text.strip_prefix("if")?;
    let (guard, after) = balanced(skip_trivia(after), '(', ')')?;
    let (block, after) = balanced(skip_trivia(after), '{', '}')?;
    Some((guard, block, after))
}

/// Classify the subject, then write each answer to the path it names.
fn run_classify_ladder(event: &mut Event, pattern: &ClassifyLadder) -> bool {
    // The processor's own `if` gates on the subject, so an absent one is a
    // no-op rather than a failure.
    let Some(subject) = event.get_string(&pattern.subject) else {
        return true;
    };
    let Some(arm) = pattern.arms.iter().find(|arm| arm.holds(&subject)) else {
        return true;
    };

    for (name, target) in &pattern.outputs {
        if let Some((_, literal)) = arm.writes.iter().find(|(local, _)| local == name) {
            let _ = event.set(target, Value::String(literal.clone()));
        }
    }
    true
}

/// A nested map emptied into an ancestor, and the routing keys beside it
/// dropped.
///
/// ```painless
/// Map data = (Map) ctx.json.remove('data');
/// for (def entry : data.entrySet()) {
///   ctx.json[entry.getKey()] = entry.getValue();
/// }
/// ctx.json.remove('type');
/// ```
///
/// kolide's Log Pipeline deliveries wrap the record in a `{type, timestamp,
/// data}` envelope, and every field the rest of the pipeline reads is inside
/// `data`.
///
/// The source is bound either by the `remove` above, which also deletes it, or
/// by a plain read that leaves it where it is -- cursor's S3 arm lifts
/// `json.metadata.context` into `json` and drops the whole envelope later, so
/// consuming the map here would take the rest of it with it. That form also
/// guards each write with `containsKey`, which is the vendor saying an
/// envelope field already set wins over the nested copy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MergeMapUp {
    parent: String,
    /// The full path of the map lifted, resolved here so the hot path
    /// allocates nothing.
    source: String,
    /// Whether the binding consumed the map. A plain read leaves it.
    take: bool,
    /// What a key already on the parent does to the one arriving.
    collision: Collision,
    drops: Vec<String>,
}

/// What happens when a lifted key is already on the parent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Collision {
    /// No guard: the arriving value wins.
    Overwrite,
    /// `if (!ctx.<parent>.containsKey(k))` -- the one already there wins.
    KeepExisting,
    /// `ctx.<parent>['data_' + k] = ...` -- BOTH are kept, the arriving one
    /// under a prefix. axonius lifts its whole payload out of `event.data`
    /// this way, in nine of its streams.
    Prefix(String),
}

/// `key=value key="value with spaces"` scanned a character at a time.
///
/// `watchguard_firebox` writes its own KV parser in Painless rather than using
/// the kv processor, because its values carry spaces and colons inside quotes
/// and the vendor wants the quotes off. It is a state machine, and the only
/// faithful way to read it is to run the same one: `msg` and `proxy_act` alone
/// are 410 wrong fields across 29 events it would otherwise unlock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct QuotedKvScan {
    /// The string scanned, e.g. `_temp`.
    source: String,
    /// Where each pair lands, e.g. `watchguard_firebox.log`.
    target: String,
}

fn parse_quoted_kv_scan(script: &str) -> Option<QuotedKvScan> {
    use crate::painless_params::{clean_path, subject_path};

    // The three locals the scanner needs, all of them named in one line.
    if !script.contains("kvStart") || !script.contains("kvSplit") || !script.contains("charAt(") {
        return None;
    }
    let source = script.split_once("length();")?.0.rsplit_once("ctx")?.1;
    // The subscript is read from `ctx["_temp"].length()`, so the member dot of
    // the CALL comes back on the end of the path.
    let source = clean_path(subject_path(&format!("ctx{source}")).strip_prefix("ctx.")?);
    let source = source.trim_matches('.').to_owned();
    let target = script.split_once(".put(key, value)")?.0;
    let target = clean_path(target.rsplit_once("ctx.")?.1.trim());

    // A dot on either end is not a path, and it is the failure this guard
    // missed: `_temp.` resolved to nothing, the scan wrote nothing, and the
    // pattern still reported the script handled.
    let named = |path: &str| {
        !path.is_empty()
            && !path.contains(['(', ')', '[', ']', ' ', '"'])
            && !path.starts_with('.')
            && !path.ends_with('.')
    };
    (named(&source) && named(&target)).then_some(QuotedKvScan { source, target })
}

/// The vendor's loop, character for character.
///
/// Ported rather than reinterpreted: a quote OPENS unless the one already open
/// is followed by an end, a space or a colon, which is what lets a value hold
/// `10:20` and `a "quoted" word` alike. Bounds are checked where Painless
/// would throw, because a panic here takes the pod.
fn run_quoted_kv_scan(event: &mut Event, pattern: &QuotedKvScan) -> bool {
    // DECLINE rather than claim it: the caller gates this script on the source
    // being a string, so a source that reads as absent means the PATH is
    // wrong, and answering true there is how a no-op passed for a parse.
    let Some(text) = event.get_string(&pattern.source) else {
        return false;
    };
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let (mut kv_start, mut kv_split, mut in_quote) = (0usize, 0usize, false);

    for i in 0..n {
        let c = chars[i];
        let c2 = if i + 1 < n { chars[i + 1] } else { '\0' };

        if c == '"' {
            in_quote = !(in_quote && (c2 == '\0' || c2 == ' ' || c2 == ':'));
        }
        if in_quote {
            continue;
        }
        if c == '=' {
            kv_split = i;
        }
        if c == '"' || c == ' ' || c2 == '\0' {
            let end = if i + 1 == n { i + 1 } else { i };
            if i != kv_start && kv_start <= kv_split && kv_split < end {
                let key: String = chars[kv_start..kv_split].iter().collect();
                let value: String = chars[kv_split + 1..end].iter().collect();
                let value = value.trim_start_matches('"').trim_end_matches('"');
                if !key.is_empty() {
                    let _ = event.set(&format!("{}.{key}", pattern.target), json!(value));
                }
            }
            kv_start = i + 1;
            kv_split = i + 1;
        }
    }
    true
}

/// A list of `{name, value}` objects fanned out into a map keyed by `name`.
///
/// How Google Workspace ships every event's payload, and five of its streams
/// carry the same script:
///
/// ```painless
/// for (int i = 0; i < ctx.json.events.parameters.length; ++i) {
///   if (ctx["json"]["events"]["parameters"][i]["value"] != null) {
///     ctx.google_workspace.drive[ctx["json"]["events"]["parameters"][i]["name"]]
///       = ctx["json"]["events"]["parameters"][i]["value"];
///   }
///   // then again for "multiValue" and "boolValue"
/// }
/// ```
///
/// Read as its own pattern rather than through general loop support, which the
/// corpus of scripts does not justify: 579 enhanced loops decompose to six
/// with a common body, where this one body appears in five streams and holds
/// whole blocks of `google_workspace.drive.*`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParameterFanOut {
    /// The list walked, e.g. `json.events.parameters`.
    list: String,
    /// Where the named keys land, e.g. `google_workspace.drive`.
    target: String,
    /// The value keys tried, in the order the script writes them, so a later
    /// one overwrites an earlier the same way.
    values: Vec<String>,
}

fn parse_parameter_fan_out(script: &str) -> Option<ParameterFanOut> {
    use crate::painless_params::clean_path;

    // The loop counts an index over a list's length, which is what separates
    // this from every enhanced-for pattern.
    let head = script.split_once("for (int ")?.1;
    let (counter, head) = head.split_once(" = 0;")?;
    let counter = counter.trim();
    let list = head.split_once(".length")?.0;
    let list = clean_path(list.rsplit_once("ctx.")?.1.trim());

    // The write's subject: `ctx.<target>[ctx[...][i]["name"]] = ...`. Anchored
    // on `[ctx[`, which is where the subject ends -- the first `[` in the
    // script is inside the guard above it.
    let keyed = format!("[{counter}][\"name\"]]");
    let before = script.split_once(&keyed)?.0;
    let target = clean_path(before.rsplit_once("[ctx[")?.0.rsplit_once("ctx.")?.1.trim());

    // Every `[i]["<key>"] != null` the body guards on, in order. `name` is the
    // key, never a value.
    let guard = format!("[{counter}][\"");
    let mut values = Vec::new();
    for (at, _) in script.match_indices(&guard) {
        let rest = &script[at + guard.len()..];
        let Some((key, tail)) = rest.split_once('"') else {
            continue;
        };
        if key == "name" || values.iter().any(|seen| seen == key) {
            continue;
        }
        if tail.trim_start().starts_with("] != null") {
            values.push(key.to_string());
        }
    }

    let named = |path: &str| !path.is_empty() && !path.contains(['(', ')', '[', ']', ' ', '"']);
    if values.is_empty() || !named(&list) || !named(&target) {
        return None;
    }
    Some(ParameterFanOut {
        list,
        target,
        values,
    })
}

fn run_parameter_fan_out(event: &mut Event, pattern: &ParameterFanOut) -> bool {
    let Some(Value::Array(items)) = event.get(&pattern.list) else {
        return true;
    };
    let items = items.clone();
    for item in items {
        let Some(entry) = item.as_object() else {
            continue;
        };
        let Some(name) = entry.get("name").and_then(Value::as_str) else {
            continue;
        };
        for key in &pattern.values {
            match entry.get(key) {
                Some(Value::Null) | None => {}
                Some(value) => {
                    let _ = event.set(&format!("{}.{name}", pattern.target), value.clone());
                }
            }
        }
    }
    true
}

/// The same merge with no loop at all: `ctx.<target>.putAll(ctx.<source>)`.
///
/// Four sites, and tanium's `threat_response` is one of them -- its whole
/// `Match Details` payload lands through this single call, so leaving it
/// unread cost 1,968 fields on 54 events. Count is not value: the loop forms
/// are a hundred times more common and worth a fraction of this.
fn parse_put_all(script: &str) -> Option<MergeMapUp> {
    use crate::painless_params::{clean_path, subject_path};

    // A loop spells the merge out and is the other function's business.
    if script.contains("for (") {
        return None;
    }
    let (head, tail) = script.split_once(".putAll(")?;
    // The subject is the LAST statement's, not the script's. tanium allocates
    // the target on the line above, and reading from the start took both.
    let head = head.rsplit([';', '\n', '{', '}']).next()?.trim();
    let parent = clean_path(subject_path(head).strip_prefix("ctx.")?);
    let argument = tail.split_once(')')?.0.trim();
    let source = clean_path(subject_path(argument).strip_prefix("ctx.")?);

    // Merging a map into itself, or into something that is not a path, is not
    // this pattern whatever the text says.
    let named = |path: &str| !path.is_empty() && !path.contains(['(', '[', ']', '\'', '"']);
    if !named(&parent) || !named(&source) || parent == source {
        return None;
    }
    Some(MergeMapUp {
        parent,
        source,
        take: false,
        collision: Collision::Overwrite,
        drops: Vec::new(),
    })
}

fn parse_merge_map_up(script: &str) -> Option<MergeMapUp> {
    use crate::painless_params::{clean_path, ctx_path_before};

    // The loop names both locals: the entry it binds, and the map it walks.
    // Two spellings of one operation -- `m.entrySet()` reaching members
    // through `getKey`, and `new ArrayList(m.keySet())` subscripting instead.
    let (entry, after) = script.split_once("for (")?.1.split_once(':')?;
    let entry = entry.trim().rsplit(' ').next()?;
    let keyed = after.contains(".keySet()");
    let local = if keyed {
        after
            .split_once(".keySet()")?
            .0
            .trim()
            .trim_start_matches("new ArrayList(")
            .trim()
    } else {
        after.split_once(".entrySet()")?.0.trim()
    };
    if entry.is_empty() || local.is_empty() {
        return None;
    }

    // The parent is read from the WRITE rather than from the binding, because
    // that is where the merge actually lands -- the two differ whenever the
    // map is nested more than one level down.
    let write = if keyed {
        format!("[{entry}] = {local}[{entry}]")
    } else {
        format!("[{entry}.getKey()] = {entry}.getValue()")
    };
    let parent = ctx_path_before(script, &write)?;
    if parent.is_empty() {
        return None;
    }

    // Three ways the loop names the map, and nothing else: the path itself, a
    // local bound to a `remove` that also consumes it, or a local bound to a
    // plain read that leaves it. A local the script BUILT is not a map the
    // event carries, so anything else declines here.
    let (source, take) = if let Some(path) = local.strip_prefix("ctx.") {
        (clean_path(path), false)
    } else {
        let binding = script.split_once(&format!("{local} = "))?.1.trim();
        if let Some((head, tail)) = binding.split_once(".remove(") {
            let holder = clean_path(head.rsplit_once("ctx.")?.1.trim());
            let key = quoted_first(tail)?;
            if holder.is_empty() || key.contains('.') {
                return None;
            }
            (format!("{holder}.{key}"), true)
        } else {
            let read = binding.split([';', '\n']).next()?.trim();
            (clean_path(read.strip_prefix("ctx.")?.trim()), false)
        }
    };
    // Merging a map into itself is not this pattern whatever the text says.
    if source.is_empty() || source == parent || source.contains(['(', ')', ' ']) {
        return None;
    }

    let collision = if script.contains(&format!("!ctx.{parent}.containsKey({entry}.getKey())")) {
        Collision::KeepExisting
    } else if let Some(prefix) = prefixed_on_collision(script, &parent, entry) {
        Collision::Prefix(prefix)
    } else {
        Collision::Overwrite
    };

    let dropper = format!("ctx.{parent}.remove(");
    let drops = script
        .match_indices(&dropper)
        .filter_map(|(at, _)| quoted_first(&script[at + dropper.len()..]))
        .map(|dropped| format!("{parent}.{dropped}"))
        .filter(|path| *path != source)
        .collect();

    Some(MergeMapUp {
        parent,
        source,
        take,
        collision,
        drops,
    })
}

/// The prefix a colliding key is stored under, if the script keeps both.
///
/// `if (ctx.<parent>.containsKey(k)) { ctx.<parent>['data_' + k] = ... }` is
/// axonius's spelling, and the prefix is the quoted half of that expression.
fn prefixed_on_collision(script: &str, parent: &str, entry: &str) -> Option<String> {
    let guard = format!("ctx.{parent}.containsKey({entry})");
    let after = script.split_once(&guard)?.1;
    let marker = format!("' + {entry}]");
    let head = after.split_once(&marker)?.0;
    let prefix = head.rsplit_once('\'')?.1;
    (!prefix.is_empty() && !prefix.contains(['.', ' '])).then(|| prefix.to_string())
}

fn run_merge_map_up(event: &mut Event, pattern: &MergeMapUp) -> bool {
    // The processor's own `instanceof Map` guard, so anything else is a no-op.
    let entries = if pattern.take {
        match event.remove(&pattern.source) {
            Some(Value::Object(entries)) => entries,
            _ => return true,
        }
    } else {
        match event.get(&pattern.source) {
            Some(Value::Object(entries)) => entries.clone(),
            _ => return true,
        }
    };
    if let Some(Value::Object(parent)) = crate::painless_params::pointer_mut(event, &pattern.parent)
    {
        for (key, value) in entries {
            match &pattern.collision {
                // The one already there wins, so the arriving value is dropped.
                Collision::KeepExisting if parent.contains_key(&key) => {}
                Collision::Prefix(prefix) if parent.contains_key(&key) => {
                    parent.insert(format!("{prefix}{key}"), value);
                }
                _ => {
                    parent.insert(key, value);
                }
            }
        }
    }
    for path in &pattern.drops {
        event.remove(path);
    }
    true
}

/// A whole map moved beneath a NEW parent, and the source removed.
///
/// ```painless
/// def dict = ['result': new HashMap()];
/// for (entry in ctx['json'].entrySet()) {
///   dict['result'][entry.getKey()] = entry.getValue();
/// }
/// ctx['osquery'] = dict;
/// ctx.remove('json');
/// ```
///
/// The loop copies every member, so the whole map lands under
/// `<target>.<member>` and the source goes. It is a rename spelled out
/// entry by entry, and osquery's result stream is gated on it: unclaimed, all
/// 2,213 of its events keep the raw `json` tree and reach none of the
/// forty-odd processors that read `osquery.result.*`.
///
/// [`MergeMapUp`] is the opposite move -- a map's members lifted INTO an
/// existing parent rather than nested under a new one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NestUnder {
    /// The map that moves.
    source: String,
    /// Where it lands, parent and member already joined.
    target: String,
    /// What the script removes afterwards.
    removes: Vec<String>,
}

/// A field replaced by WHETHER it equals a literal.
///
/// `def value = ctx.a.b; ctx.a['b'] = value == 'yes';` -- cyberarkpas turns
/// its `Rfc5424` string into the boolean Elasticsearch stores.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EqualsLiteralFlag {
    source: String,
    target: String,
    literal: String,
}

fn parse_equals_literal_flag(script: &str) -> Option<EqualsLiteralFlag> {
    use crate::painless_params::clean_path;

    // Two statements and a trailing newline; anything longer is a different
    // script that happens to compare against a literal somewhere.
    if script.split(';').filter(|s| !s.trim().is_empty()).count() > 2 {
        return None;
    }

    let (head, tail) = script.split_once(" == ")?;
    let literal = tail
        .split(';')
        .next()?
        .trim()
        .trim_matches(['\'', '"'])
        .to_string();

    let (assigned, local) = head.rsplit_once(" = ")?;
    let target = desubscript(assigned.rsplit(';').next()?.trim());
    let target = clean_path(target.strip_prefix("ctx.")?);
    let source = resolve_local_path(script, local.trim())?;

    Some(EqualsLiteralFlag {
        source,
        target,
        literal,
    })
}

fn run_equals_literal_flag(event: &mut Event, pattern: &EqualsLiteralFlag) -> bool {
    // Painless throws writing through an absent parent, so a missing container
    // means the script never got to write at all.
    let parent = pattern.target.rsplit_once('.').map_or("", |(head, _)| head);
    if !parent.is_empty() && !event.has(parent) {
        return true;
    }

    // An absent source reads as null, and `null == 'yes'` is false.
    let matched = event.get_str(&pattern.source) == Some(pattern.literal.as_str());
    let _ = event.set(&pattern.target, Value::Bool(matched));
    true
}

/// A value looked up in a LIST of records, one member of the match written on.
///
/// sophos spells it twice against its config: the log's `IST` through
/// `_conf.tz_map` to `Asia/Kolkata`, and the device serial through
/// `_conf.mappings` to a hostname, that one falling back to `_conf.default`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RecordLookup {
    /// The list of records.
    table: String,
    /// The field whose value is matched.
    subject: String,
    /// The record member compared against it.
    key: String,
    /// The record member taken from the match.
    value: String,
    /// Where the result lands.
    target: String,
    /// The field holding the value used when nothing matches.
    default: Option<String>,
}

/// Follow `def a = ctx['x']; def b = a.y;` back to the dotted `ctx` path.
///
/// These scripts bind a subtree to a local before looping over it, so the
/// table's real path is two or three `def`s away from the `for`.
fn resolve_local_path(script: &str, expr: &str) -> Option<String> {
    use crate::painless_params::clean_path;

    let mut expr = desubscript(expr.trim());
    // Bounded: a `def` that refers to itself would otherwise spin.
    for _ in 0..8 {
        if let Some(rest) = expr.strip_prefix("ctx.") {
            return Some(clean_path(rest));
        }
        let (head, tail) = expr
            .split_once('.')
            .map_or((expr.as_str(), ""), |(head, tail)| (head, tail));
        let binding = format!("def {head} = ");
        let start = script.find(&binding)? + binding.len();
        let bound = &script[start..];
        let bound = bound[..bound.find([';', '\n'])?].trim();
        expr = desubscript(&if tail.is_empty() {
            bound.to_string()
        } else {
            format!("{bound}.{tail}")
        });
    }
    None
}

/// `a['b']` and `a["b"]` are `a.b`, and the pipelines use every spelling.
fn desubscript(expr: &str) -> String {
    let mut out = String::with_capacity(expr.len());
    let mut rest = expr;
    while let Some((head, tail)) = rest.split_once('[') {
        let Some((key, after)) = tail.split_once(']') else {
            break;
        };
        out.push_str(head);
        out.push('.');
        out.push_str(key.trim().trim_matches(['\'', '"']));
        rest = after;
    }
    out.push_str(rest);
    out
}

/// `for (def i : <list>) { if (i.<key> == <subject>) { <target> = i.<value>; } }`
fn parse_record_lookup(script: &str) -> Option<RecordLookup> {
    use crate::painless_params::{balanced, clean_path};

    let (head, rest) = script.split_once(" : ")?;
    let item = head.rsplit_once("def ")?.1.trim();
    let (table, after) = rest.split_once(')')?;
    let table = resolve_local_path(script, table.trim())?;
    let (body, tail) = balanced(after.trim_start(), '{', '}')?;

    // The comparison names the member and the value it is matched against.
    let (guard, assignment) = body.split_once(&format!("{item}."))?;
    if !guard.contains("if") {
        return None;
    }
    let (key, after_key) = assignment.split_once("==")?;
    let subject = resolve_local_path(script, after_key.split(')').next()?.trim())?;

    // The taken member, and where the script puts it.
    let (lhs, rhs) = assignment.split_once(&format!("= {item}."))?;
    let value = rhs
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .next()
        .filter(|value| !value.is_empty())?;
    let written = lhs.rsplit(['\n', '{', '}', ';']).next()?.trim();

    // The write is either straight onto `ctx`, or onto a local the script
    // seeds with a default and assigns after the loop.
    let (target, default) = match written.strip_prefix("ctx.") {
        Some(path) => (clean_path(path), None),
        None => (
            crate::painless_params::ctx_path_before(tail, &format!("= {written};"))?,
            resolve_local_path(script, written),
        ),
    };

    Some(RecordLookup {
        table,
        subject,
        key: clean_path(key.trim()),
        value: value.to_string(),
        target,
        default,
    })
}

fn run_record_lookup(event: &mut Event, pattern: &RecordLookup) -> bool {
    let Some(Value::Array(records)) = event.get(&pattern.table).cloned() else {
        // `if (mappings == null) return;` -- the script's own guard.
        return true;
    };
    let Some(subject) = event.get(&pattern.subject).cloned() else {
        return true;
    };

    let mut chosen = pattern
        .default
        .as_ref()
        .and_then(|path| event.get(path).cloned());
    for record in &records {
        if record.get(&pattern.key) == Some(&subject) {
            chosen = record.get(&pattern.value).cloned();
            break;
        }
    }
    if let Some(value) = chosen {
        let _ = event.set(&pattern.target, value);
    }
    true
}

/// One list's members collected into deduped arrays.
///
/// `cisco_secure_endpoint` walks `computer.network_addresses` three times: the
/// addresses into `host.ip` and `related.ip`, the MACs into `host.mac` and its
/// own `related.mac`, each dash-separated and upper-cased on the way.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CollectFromList {
    /// The list the entries come from.
    source: String,
    columns: Vec<CollectedColumn>,
    /// `if (<collected>.size() > 0) { ctx.<target> = <collected>[0]; }` -- the
    /// first of a collected column, kept beside the whole list. gdacs writes
    /// its country lists and then names the leading country.
    firsts: Vec<(String, String)>,
}

/// One member of the list's entries, and the array its values land in.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CollectedColumn {
    member: String,
    target: String,
    /// `.replace(from, to)`, applied before the fold.
    replace: Option<(String, String)>,
    upper: bool,
}

/// A lookup table written as a LOCAL map literal, not shipped in `params`.
///
/// gdacs names its disaster codes this way, and fourteen sites over five files
/// do the same:
///
/// ```painless
/// def typeMap = ['EQ': 'Earthquake', 'TC': 'Tropical Cyclone', ...];
/// def code = ctx.gdacs?.event_type;
/// if (code != null && typeMap.containsKey(code)) {
///   ctx.gdacs.event_type_name = typeMap[code];
/// }
/// ```
///
/// The table is a literal the call site fully determines, so it is read once
/// per site rather than per event, like every other params block here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LocalMapLookup {
    source: String,
    target: String,
    table: std::collections::BTreeMap<String, String>,
}

fn parse_local_map_lookup(script: &str) -> Option<LocalMapLookup> {
    use crate::painless_params::{clean_path, subject_path};

    // `def <name> = [ '<k>': '<v>', ... ];`
    let (head, rest) = script.split_once(" = [")?;
    let name = head.rsplit([' ', '\n']).next()?.trim();
    // A map literal holds no nested brackets, so the first `]` closes it.
    let inside = rest.split_once(']')?.0;
    let mut table = std::collections::BTreeMap::new();
    for pair in inside.split(',') {
        let (key, value) = pair.split_once(':')?;
        let key = key.trim().trim_matches(['\'', '"']).to_string();
        let value = value.trim().trim_matches(['\'', '"']).to_string();
        if key.is_empty() || value.is_empty() {
            return None;
        }
        table.insert(key, value);
    }
    if table.is_empty() || name.is_empty() {
        return None;
    }

    // The key local, and the field it reads.
    let (bound, after) = rest.split_once(&format!("{name}.containsKey("))?;
    let key = after.split(')').next()?.trim();
    let source = clean_path(
        subject_path(
            bound
                .rsplit(&format!("{key} = "))
                .next()?
                .split(';')
                .next()?
                .trim(),
        )
        .strip_prefix("ctx.")?,
    );

    // `ctx.<target> = <name>[<key>]`
    let (assignment, _) = script.split_once(&format!("= {name}[{key}]"))?;
    let target = clean_path(assignment.rsplit_once("ctx.")?.1.trim());

    let named = |path: &str| !path.is_empty() && !path.contains(['(', ')', '[', ']', ' ']);
    (named(&source) && named(&target)).then_some(LocalMapLookup {
        source,
        target,
        table,
    })
}

fn run_local_map_lookup(event: &mut Event, pattern: &LocalMapLookup) -> bool {
    // The script's own `containsKey` guard: an unlisted code writes nothing.
    if let Some(code) = event.get_as_string(&pattern.source)
        && let Some(name) = pattern.table.get(&code)
    {
        let _ = event.set(&pattern.target, json!(name));
    }
    true
}

/// The loop's variable, the `ctx.` list it walks, and the text after its head.
///
/// Two spellings, and reading only the first left gdacs at 0 of 42 events:
/// `for (v in ctx.<path>)`, and `for (def v : <list>)` where the list is
/// either a `ctx.` path or a local bound to one above the loop.
fn loop_head(script: &str) -> Option<(String, String, &str)> {
    use crate::painless_params::clean_path;

    if let Some((head, rest)) = script.split_once(" in ctx") {
        let var = head.rsplit_once('(')?.1.trim().to_string();
        let (source, after) = rest.split_once(')')?;
        return Some((
            var,
            clean_path(source.trim().trim_start_matches('.')),
            after,
        ));
    }

    let (head, rest) = script.split_once(" : ")?;
    let var = head.rsplit(['(', ' ']).next()?.trim().to_string();
    let (walked, after) = rest.split_once(')')?;
    let walked = walked.trim();
    let source = match walked
        .strip_prefix("ctx.")
        .or_else(|| walked.strip_prefix("ctx?."))
    {
        Some(path) => clean_path(path),
        // A local: the binding above the loop names the list.
        None => bound_list(script, walked)?,
    };
    Some((var, source, after))
}

/// The `ctx.` list a local walked by a loop was bound to.
fn bound_list(script: &str, local: &str) -> Option<String> {
    use crate::painless_params::{clean_path, subject_path};

    if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    for statement in script.split([';', '\n']) {
        let Some((head, tail)) = statement.split_once('=') else {
            continue;
        };
        if head.trim().rsplit([' ', '\t']).next() != Some(local) {
            continue;
        }
        let path = subject_path(tail.trim());
        let Some(path) = path.strip_prefix("ctx.") else {
            continue;
        };
        if !path.contains([' ', '(', '[']) {
            return Some(clean_path(path));
        }
    }
    None
}

/// The `ctx.` path a local accumulator is handed to, if it is handed to one.
///
/// `ctx.<target> = <local>;` after the loop. The name must match whole, or
/// `names` would answer for `country_names`.
fn assigned_to_ctx(script: &str, local: &str) -> Option<String> {
    use crate::painless_params::clean_path;

    if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }
    for statement in script.split([';', '\n']) {
        let Some((head, tail)) = statement.split_once('=') else {
            continue;
        };
        if tail.trim() != local {
            continue;
        }
        let Some(path) = head.trim().strip_prefix("ctx.") else {
            continue;
        };
        if !path.contains([' ', '(', '[']) {
            return Some(clean_path(path));
        }
    }
    None
}

/// `for (v in ctx.<list>) { if (v.<member> != null && !v.<member>.isEmpty())
/// { ... ctx.<target>.add(...) } }`
fn parse_collect_from_list(script: &str) -> Option<CollectFromList> {
    use crate::painless_params::{balanced, clean_path};

    let (var, source, after) = loop_head(script)?;
    let var = var.as_str();
    if var.is_empty() || source.is_empty() {
        return None;
    }
    let (body, _) = balanced(after.trim_start(), '{', '}')?;

    let mut columns = Vec::new();
    for arm in body.split(&format!("if ({var}.")).skip(1) {
        let member = arm
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .next()
            .filter(|member| !member.is_empty())?;
        let (head, _) = arm.split_once(".add(")?;
        let added_to = head.rsplit(['\n', ';', '{', '}']).next()?.trim();
        // Either named on the document, or a LOCAL handed to one after the
        // loop: gdacs builds three accumulators and assigns them at the end.
        let target = match added_to.rsplit_once("ctx.") {
            Some((_, path)) => clean_path(path.trim()),
            None => assigned_to_ctx(script, added_to)?,
        };
        if target.is_empty() {
            return None;
        }
        let replace = arm.split_once(".replace(").and_then(|(_, tail)| {
            let (from, to) = tail.split_once(')')?.0.split_once(',')?;
            Some((
                from.trim().trim_matches(['\'', '"']).to_string(),
                to.trim().trim_matches(['\'', '"']).to_string(),
            ))
        });
        columns.push(CollectedColumn {
            member: member.to_string(),
            target,
            replace,
            upper: arm.contains(".toUpperCase()"),
        });
    }
    // `if (<local>.size() > 0) { ctx.<target> = <local>[0]; }` after the loop.
    let mut firsts = Vec::new();
    for arm in script.split(".size() > 0)").skip(1) {
        let Some((assignment, _)) = arm.split_once("[0]") else {
            continue;
        };
        let Some((head, local)) = assignment.rsplit_once('=') else {
            continue;
        };
        let Some(target) = head
            .trim()
            .rsplit_once("ctx.")
            .map(|(_, p)| clean_path(p.trim()))
        else {
            continue;
        };
        // The local has to be one this loop collected, or the first member
        // belongs to a list nothing here built.
        let local = local.trim();
        if let Some(collected) = assigned_to_ctx(script, local)
            && columns.iter().any(|column| column.target == collected)
            && !target.is_empty()
        {
            firsts.push((target, collected));
        }
    }

    (!columns.is_empty()).then_some(CollectFromList {
        source,
        columns,
        firsts,
    })
}

fn run_collect_from_list(event: &mut Event, pattern: &CollectFromList) -> bool {
    let Some(Value::Array(entries)) = event.get(&pattern.source).cloned() else {
        // Every one of these scripts is gated on the list being present.
        return true;
    };

    for column in &pattern.columns {
        let mut collected = match event.get(&column.target) {
            Some(Value::Array(existing)) => existing.clone(),
            _ => Vec::new(),
        };
        for entry in &entries {
            let Some(raw) = entry.get(&column.member).and_then(Value::as_str) else {
                continue;
            };
            if raw.is_empty() {
                continue;
            }
            let mut value = raw.to_string();
            if let Some((from, to)) = &column.replace {
                value = value.replace(from.as_str(), to);
            }
            if column.upper {
                value = value.to_uppercase();
            }
            // cisco's `related.mac` arm tests the RAW value for membership and
            // adds the folded one. The fold is injective on a MAC, so deduping
            // on the value that lands is the same answer and reads honestly.
            let value = Value::String(value);
            if !collected.contains(&value) {
                collected.push(value);
            }
        }
        // The script builds the array before the loop, so a list that
        // contributes nothing still leaves an empty one behind.
        let _ = event.set(&column.target, Value::Array(collected));
    }

    // The script's own `size() > 0` guard: an empty collection names nothing.
    for (target, collected) in &pattern.firsts {
        if let Some(Value::Array(items)) = event.get(collected)
            && let Some(first) = items.first().cloned()
        {
            let _ = event.set(target, first);
        }
    }
    true
}

fn parse_nest_under(script: &str) -> Option<NestUnder> {
    use crate::painless_params::{clean_path, subject_path};

    // `def dict = ['result': new HashMap()];` names the local and the member.
    let (head, rest) = script.split_once(": new HashMap()]")?;
    let (head, member) = head.rsplit_once("['")?;
    let local = head.rsplit_once(" = ")?.0.rsplit(' ').next()?.trim();
    let member = member.trim_end_matches('\'').trim();
    if local.is_empty() || member.is_empty() {
        return None;
    }

    // The loop has to copy EVERY entry across, or this is a filter and not a
    // move.
    let (loop_head, loop_body) = rest.split_once(".entrySet()")?;
    let source = clean_path(subject_path(loop_head.rsplit_once("ctx")?.1).trim_start_matches('.'));
    if source.is_empty()
        || !loop_body.contains("entry.getKey()")
        || !loop_body.contains("entry.getValue()")
    {
        return None;
    }

    // Where the local lands.
    let (head, _) = rest.split_once(&format!("= {local};"))?;
    let parent = clean_path(
        subject_path(head.rsplit_once("ctx")?.1)
            .trim_start_matches('.')
            .trim(),
    );
    if parent.is_empty() {
        return None;
    }

    Some(NestUnder {
        source,
        target: format!("{parent}.{member}"),
        removes: ctx_removes(script),
    })
}

fn run_nest_under(event: &mut Event, pattern: &NestUnder) -> bool {
    let Some(moved) = event.get(&pattern.source).cloned() else {
        return true;
    };
    let _ = event.set(&pattern.target, moved);
    for path in &pattern.removes {
        event.remove(path);
    }
    true
}

/// The value the document's POSITION in a list decides -- first, last, or
/// somewhere between.
///
/// ```painless
/// def evs = ctx.json.events;
/// def ts = ctx.json.timestamp;
/// int idx = -1;
/// for (int i = 0; i < evs.size(); i++) {
///   if (evs[i] instanceof Map && evs[i].timestamp == ts) { idx = i; break; }
/// }
/// if (idx != -1) {
///   if (ctx.event == null) { ctx.event = [:]; }
///   if (idx == evs.size() - 1) { ctx.event.type = ['end']; }
///   else if (idx == 0) { ctx.event.type = ['start']; }
///   else { ctx.event.type = ['info']; }
/// }
/// ```
///
/// kolide's auth stream repeats the whole session in `json.events` on every
/// document, so this is what says which of them this one is. The LAST test wins
/// a single-entry list, which is the case the ordering exists for: index 0 is
/// also index `size - 1` there, and the session ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PositionInList {
    /// The list this document is looked for in.
    list: String,
    /// The document's own copy of what the entries are matched on.
    key: String,
    /// The member of each entry compared against `key`.
    member: String,
    /// Where the decided value lands.
    target: String,
    /// Written for the last entry, the first entry, and anything between.
    last: Value,
    first: Value,
    middle: Value,
}

fn parse_position_in_list(script: &str) -> Option<PositionInList> {
    use crate::painless_params::{balanced, clean_path, literal_writes, skip_trivia};

    // `def evs = ctx.json.events;` and then `def ts = ctx.json.timestamp;`.
    let (head, rest) = script.split_once(" = ctx.")?;
    let items = head.rsplit(' ').next()?.trim().to_string();
    let (walked, rest) = rest.split_once(';')?;
    let (head, rest) = rest.split_once(" = ctx.")?;
    let wanted = head.rsplit(' ').next()?.trim().to_string();
    let (key, rest) = rest.split_once(';')?;
    if items.is_empty() || wanted.is_empty() {
        return None;
    }

    // The member each entry is matched on, and the local it is matched against.
    let (_, test) = rest.split_once(&format!("{items}[i]."))?;
    let (member, against) = test.split_once("==")?;
    let matched = against.split([')', '&', '|', ';']).next()?.trim();
    if matched != wanted || member.trim().is_empty() {
        return None;
    }

    // The three-armed ladder over the index, in the order the script tests it.
    let at = script.find(&format!("{items}.size() - 1"))?;
    let opens = script[..at].rfind("if")? + "if".len();
    let (_, after) = balanced(skip_trivia(&script[opens..]), '(', ')')?;
    let (last, after) = balanced(skip_trivia(after), '{', '}')?;
    let after = skip_trivia(after).strip_prefix("else")?;
    let after = skip_trivia(after).strip_prefix("if")?;
    let (_, after) = balanced(skip_trivia(after), '(', ')')?;
    let (first, after) = balanced(skip_trivia(after), '{', '}')?;
    let after = skip_trivia(after).strip_prefix("else")?;
    let (middle, _) = balanced(skip_trivia(after), '{', '}')?;

    // One write per arm, all three onto the same field, or this is a ladder
    // that means something else.
    let [last, first, middle] = [last, first, middle].map(literal_writes);
    let ([(target, last)], [(first_at, first)], [(middle_at, middle)]) =
        (last.as_slice(), first.as_slice(), middle.as_slice())
    else {
        return None;
    };
    (target == first_at && target == middle_at).then(|| PositionInList {
        list: clean_path(walked.trim()),
        key: clean_path(key.trim()),
        member: member.trim().to_string(),
        target: target.clone(),
        last: last.clone(),
        first: first.clone(),
        middle: middle.clone(),
    })
}

fn run_position_in_list(event: &mut Event, pattern: &PositionInList) -> bool {
    let found = {
        let (Some(items), Some(wanted)) = (
            event.get(&pattern.list).and_then(Value::as_array),
            event.get(&pattern.key),
        ) else {
            return true;
        };
        items
            .iter()
            .position(|item| item.is_object() && item.get(&pattern.member) == Some(wanted))
            .map(|at| (at, items.len()))
    };
    let Some((at, len)) = found else {
        return true;
    };

    let value = if at + 1 == len {
        &pattern.last
    } else if at == 0 {
        &pattern.first
    } else {
        &pattern.middle
    };
    let value = value.clone();
    let _ = event.set(&pattern.target, value);
    true
}

/// A dotted name split into its FIRST label and the rest of it.
///
/// ```painless
/// def domain = '';
/// def nameArray = ctx.json.dnsName.toString().splitOnToken('.');
/// if (nameArray?.length != null && nameArray.length > 0) {
///   for (int i = 1; i < nameArray.length; i++) {
///     domain += nameArray[i] + (i < nameArray.length - 1 ? '.' : '');
///   }
///   ctx.host.name = nameArray[0];
///   ctx.host.domain = domain;
/// }
/// ```
///
/// A one-label name leaves the tail EMPTY rather than unset, which is what the
/// vendor's accumulator starts at and what the drop-empty after it then takes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SplitFirstLabel {
    source: String,
    separator: String,
    head: String,
    tail: String,
}

fn parse_split_first_label(script: &str) -> Option<SplitFirstLabel> {
    use crate::painless_params::{clean_path, ctx_path_before};

    let (bound, rest) = script.split_once(".splitOnToken(")?;
    let separator = quoted_first(rest)?;
    let source = clean_path(
        bound
            .rsplit_once("ctx.")?
            .1
            .trim()
            .trim_end_matches(".toString()"),
    );
    let parts = bound.rsplit_once(" = ")?.0.rsplit(' ').next()?.trim();
    if source.is_empty() || separator.is_empty() || parts.is_empty() {
        return None;
    }

    // The first element lands on one field; the accumulator the loop built
    // lands on the other.
    let head = ctx_path_before(script, &format!(" = {parts}[0]"))?;
    let accumulator = script
        .split_once(&format!("+= {parts}["))?
        .0
        .rsplit(['\n', ';', '{'])
        .next()?
        .trim()
        .to_string();
    if accumulator.is_empty() {
        return None;
    }
    let tail = ctx_path_before(script, &format!(" = {accumulator};"))?;
    (!head.is_empty() && !tail.is_empty() && head != tail).then_some(SplitFirstLabel {
        source,
        separator,
        head,
        tail,
    })
}

fn run_split_first_label(event: &mut Event, pattern: &SplitFirstLabel) -> bool {
    let Some(name) = event.get_str(&pattern.source).map(str::to_owned) else {
        return true;
    };
    let mut labels = name.split(pattern.separator.as_str());
    let Some(first) = labels.next() else {
        return true;
    };
    let rest = labels.collect::<Vec<_>>().join(&pattern.separator);
    let _ = event.set(&pattern.head, Value::String(first.to_string()));
    let _ = event.set(&pattern.tail, Value::String(rest));
    true
}

/// One entry of a map retyped from its digit spelling to a boolean, in place.
///
/// ```painless
/// def obj = ctx.json.DETECTION_LIST;
/// if (obj.containsKey("IS_IGNORED") && obj.get("IS_IGNORED").equals('0')) {
///   obj.remove("IS_IGNORED");
///   obj.put("IS_IGNORED", false);
/// } else if (obj.containsKey("IS_IGNORED") && obj.get("IS_IGNORED").equals('1')) {
///   obj.remove("IS_IGNORED");
///   obj.put("IS_IGNORED", true);
/// }
/// ```
///
/// Qualys ships its flags as `"0"` and `"1"`, and the `convert` to boolean
/// after this one throws on both -- so leaving the script unclaimed does not
/// cost one field, it fails the processor and stamps `pipeline_error` over the
/// whole document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapEntryToBoolean {
    /// The map holding the entry, as a `ctx.` path.
    parent: String,
    /// The entry retyped.
    key: String,
    /// Each spelling and the boolean it means.
    values: Vec<(String, bool)>,
}

fn parse_map_entry_to_boolean(script: &str) -> Option<MapEntryToBoolean> {
    use crate::painless_params::clean_path;

    let (head, rest) = script.split_once(" = ctx.")?;
    let local = head.rsplit(['\n', ';', ' ']).next()?.trim();
    let parent = clean_path(rest.split([';', '\n']).next()?.trim());
    if local.is_empty() || parent.is_empty() || parent.contains(['(', ')', ' ']) {
        return None;
    }

    let test = format!("{local}.get(");
    let write = format!("{local}.put(");
    let mut key = String::new();
    let mut values = Vec::new();
    for (arm, _) in script.match_indices(&test) {
        let arm = &script[arm + test.len()..];
        let (Some(named), Some(spelling)) = (
            quoted_first(arm),
            arm.split_once(".equals(")
                .and_then(|(_, tail)| quoted_first(tail)),
        ) else {
            continue;
        };
        // The write has to name the SAME entry, and carry a bare boolean.
        let put = arm.split_once(&write)?.1;
        if quoted_first(put)? != named {
            return None;
        }
        let written = put.split_once(',')?.1.trim();
        let flag = if written.starts_with("true") {
            true
        } else if written.starts_with("false") {
            false
        } else {
            return None;
        };
        if !key.is_empty() && key != named {
            return None;
        }
        key = named;
        values.push((spelling, flag));
    }
    (!key.is_empty() && !values.is_empty()).then_some(MapEntryToBoolean {
        parent,
        key,
        values,
    })
}

fn run_map_entry_to_boolean(event: &mut Event, pattern: &MapEntryToBoolean) -> bool {
    let path = format!("{}.{}", pattern.parent, pattern.key);
    let Some(Value::String(held)) = event.get(&path) else {
        return true;
    };
    let Some((_, flag)) = pattern
        .values
        .iter()
        .find(|(spelling, _)| spelling == held)
        .map(|(s, f)| (s, *f))
    else {
        return true;
    };
    let _ = event.set(&path, Value::Bool(flag));
    true
}

/// A list of hashes split into typed fields by the LENGTH of each one.
///
/// ```painless
/// void mapHashField(def ctx, def hashes, def key) {
///     for (hash in hashes) {
///         if (hash.length() == 32) {ctx.json[key + '_md5'] = hash;}
///         if (hash.length() == 64) {ctx.json[key + '_sha256'] = hash;}
///     }
/// }
/// if (ctx.json?.process_hash instanceof List) {
///     mapHashField(ctx, ctx.json?.process_hash, 'process_hash');
/// }
/// ```
///
/// Carbon Black ships every hash as an untyped list and leaves the algorithm
/// to be inferred from the width. The renames after it read the typed keys,
/// so an unclaimed script here costs `process.hash.*`,
/// `process.parent.hash.*` and everything `related.hash` collects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct HashesByLength {
    /// Where the typed keys land.
    parent: String,
    /// Each list's `ctx.` path, and the key its typed fields are named for.
    lists: Vec<(String, String)>,
    /// Hex width to key suffix, in the order the script tests them.
    widths: Vec<(usize, String)>,
}

fn parse_hashes_by_length(script: &str) -> Option<HashesByLength> {
    use crate::painless_params::{balanced, clean_path, skip_trivia, subject_path};

    // The helper's declaration names all three of the locals its body spells.
    let declaration = script.split_once("void ")?.1;
    let open = declaration.find('(')?;
    let name = declaration[..open].trim();
    let (arguments, after) = balanced(&declaration[open..], '(', ')')?;
    let parameters: Vec<&str> = arguments
        .split(',')
        .filter_map(|argument| argument.trim().rsplit(' ').next())
        .collect();
    let [_, hashes, key] = parameters[..] else {
        return None;
    };
    let (body, _) = balanced(skip_trivia(after), '{', '}')?;

    let item = body.split_once(" in ")?.0.trim().rsplit(' ').next()?;
    if !body.contains(&format!("{item} in {hashes}")) {
        return None;
    }

    let test = format!("{item}.length() == ");
    let mut widths = Vec::new();
    let mut parent = String::new();
    for segment in body.split(&test).skip(1) {
        let (digits, tail) = segment.split_once(')')?;
        let width: usize = digits.trim().parse().ok()?;
        let (holder, suffix) = tail.split_once(&format!("[{key} + "))?;
        widths.push((width, quoted_first(suffix)?));
        // The holder is written either dotted or subscripted with a literal,
        // and `ctx['json']` names the same map as `ctx.json`.
        parent = clean_path(subject_path(holder).rsplit_once("ctx.")?.1.trim());
    }
    if widths.is_empty() || parent.is_empty() {
        return None;
    }

    // Every call site, each naming one list and the key it is typed under.
    let call = format!("{name}(");
    let lists: Vec<(String, String)> = script
        .match_indices(&call)
        .skip(1)
        .filter_map(|(at, matched)| {
            let arguments = &script[at + matched.len()..];
            let (_, tail) = arguments.split_once("ctx.")?;
            let path = clean_path(tail.split(',').next()?.trim());
            let typed_as = quoted_first(arguments.split(')').next()?)?;
            (!path.is_empty()).then_some((path, typed_as))
        })
        .collect();
    (!lists.is_empty()).then_some(HashesByLength {
        parent,
        lists,
        widths,
    })
}

fn run_hashes_by_length(event: &mut Event, pattern: &HashesByLength) -> bool {
    for (path, key) in &pattern.lists {
        // Anything but a list is the script's own `instanceof List` guard.
        let Some(Value::Array(hashes)) = event.get(path) else {
            continue;
        };
        let typed: Vec<(String, String)> = hashes
            .iter()
            .filter_map(Value::as_str)
            .filter_map(|hash| {
                let (_, suffix) = pattern
                    .widths
                    .iter()
                    .find(|(width, _)| hash.chars().count() == *width)?;
                Some((
                    format!("{}.{key}{suffix}", pattern.parent),
                    hash.to_string(),
                ))
            })
            .collect();
        for (target, hash) in typed {
            let _ = event.set(&target, Value::String(hash));
        }
    }
    true
}

/// The one key of an envelope that is not envelope names the event, and its
/// value is the payload.
///
/// ```painless
/// def doc = ctx.json;
/// Set reserved = new HashSet(['metadata', 'team_id', 'ip_address', 'user_email']);
/// String eventKey = null;
/// for (def k : doc.keySet()) {
///   if (!reserved.contains(k)) { eventKey = k; break; }
/// }
/// if (eventKey == null) { return; }
/// def payload = doc.remove(eventKey);
/// doc.event_type = eventKey;
/// if (payload instanceof Map) { doc.event_data = payload; }
/// else if (payload != null) { def wrap = new HashMap(); wrap.put('value', payload); doc.event_data = wrap; }
/// else { doc.event_data = new HashMap(); }
/// ```
///
/// cursor's S3 deliveries carry the event type as the KEY rather than as a
/// field. Everything after this reads `json.event_type`, so leaving the script
/// unclaimed cost that whole arm of the source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UnreservedKeyPayload {
    /// The envelope searched.
    doc: String,
    /// Keys that are envelope rather than payload.
    reserved: Vec<String>,
    /// Where the found key's NAME lands, relative to the envelope.
    name_field: String,
    /// Where its VALUE lands, relative to the envelope.
    payload_field: String,
    /// The key a payload that is not a map is wrapped under.
    wrap_key: String,
}

fn parse_unreserved_key_payload(script: &str) -> Option<UnreservedKeyPayload> {
    use crate::painless_params::{balanced, clean_path, skip_trivia};

    // The envelope, and the local the rest of the script spells it as.
    let (head, tail) = script.split_once(" = ctx.")?;
    let doc_local = head.rsplit(['\n', ';', ' ']).next()?.trim();
    let doc = clean_path(tail.split([';', '\n']).next()?.trim());
    if doc_local.is_empty() || doc.is_empty() || doc.contains(['(', ')', ' ']) {
        return None;
    }

    // The reserved set, and the scan that skips it.
    let set = script.split_once("new HashSet(")?.1;
    let (list, _) = balanced(skip_trivia(set), '[', ']')?;
    let reserved: Vec<String> = list.split(',').filter_map(quoted_first).collect();
    if reserved.is_empty() {
        return None;
    }
    let loop_var = script
        .split_once(&format!("{doc_local}.keySet()"))?
        .1
        .split_once(".contains(")?
        .1
        .split_once(')')?
        .0
        .trim();
    if loop_var.is_empty() {
        return None;
    }
    // The loop variable is not what the rest of the script spells: the body
    // captures it into a local declared above the loop, and that is the name
    // the write below is keyed by.
    let key_local = script
        .split_once(&format!("= {loop_var};"))?
        .0
        .trim_end()
        .rsplit(['\n', ';', '{', ' '])
        .next()?
        .trim()
        .to_string();
    if key_local.is_empty() || key_local == loop_var {
        return None;
    }

    // The payload local, and where the key's name and value are written.
    let payload_local = script
        .split_once(&format!("{doc_local}.remove("))?
        .0
        .rsplit_once(" = ")?
        .0
        .rsplit(' ')
        .next()?
        .to_string();
    let name_field = script
        .match_indices(&format!("{doc_local}."))
        .find_map(|(at, matched)| {
            let (field, value) = script[at + matched.len()..].split_once('=')?;
            let field = field.trim();
            let value = value.split([';', '\n']).next()?.trim();
            (value == key_local && !field.contains([' ', '(', '.'])).then(|| field.to_string())
        })?;
    let payload_field = script
        .split_once(&format!("{payload_local} instanceof Map"))?
        .1
        .split_once(&format!("{doc_local}."))?
        .1
        .split_once('=')?
        .0
        .trim()
        .to_string();
    if payload_field.is_empty() || payload_field.contains([' ', '(', '.']) {
        return None;
    }

    Some(UnreservedKeyPayload {
        doc,
        reserved,
        name_field,
        payload_field,
        wrap_key: quoted_first(script.split_once(".put(")?.1)?,
    })
}

fn run_unreserved_key_payload(event: &mut Event, pattern: &UnreservedKeyPayload) -> bool {
    let Some(Value::Object(envelope)) = event.get(&pattern.doc) else {
        return true;
    };
    // Insertion order is the script's own iteration order, which is why the
    // whole document is read under `preserve_order`.
    let Some(key) = envelope
        .keys()
        .find(|key| !pattern.reserved.iter().any(|held| held == *key))
        .cloned()
    else {
        // The script's own `if (eventKey == null) { return; }`.
        return true;
    };

    let payload = event.remove(&format!("{}.{key}", pattern.doc));
    let payload = match payload {
        Some(map @ Value::Object(_)) => map,
        Some(Value::Null) | None => Value::Object(Map::new()),
        Some(scalar) => {
            let mut wrap = Map::new();
            wrap.insert(pattern.wrap_key.clone(), scalar);
            Value::Object(wrap)
        }
    };
    let _ = event.set(
        &format!("{}.{}", pattern.doc, pattern.name_field),
        Value::String(key),
    );
    let _ = event.set(
        &format!("{}.{}", pattern.doc, pattern.payload_field),
        payload,
    );
    true
}

/// Named top-level keys moved under another object.
///
/// ```painless
/// if (ctx.containsKey('id')) { ctx.json.id = ctx.remove('id'); }
/// ```
///
/// kolide's webhook envelope arrives beside the ECS fields and collides with
/// them, so the whole of it is moved out of the way before anything else runs;
/// leaving it in place strands `data` where no later processor looks.
fn parse_move_keys(script: &str) -> Option<Vec<(String, String)>> {
    const MOVE: &str = " = ctx.remove(";

    let mut moves = Vec::new();
    for (at, _) in script.match_indices(MOVE) {
        let (_, target) = script[..at].rsplit_once("ctx.")?;
        if !target
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '_')
        {
            return None;
        }
        let source = quoted_first(&script[at + MOVE.len()..])?;
        moves.push((source, target.to_string()));
    }
    // Every removal has to be one of these moves, so a longer script that
    // merely spells one is left to the matcher that reads the rest of it.
    (!moves.is_empty() && moves.len() == script.matches("ctx.remove(").count()).then_some(moves)
}

fn run_move_keys(event: &mut Event, moves: &[(String, String)]) -> bool {
    for (source, target) in moves {
        if let Some(held) = event.remove(source) {
            let _ = event.set(target, held);
        }
    }
    true
}

/// `def <name> = <receiver>.splitOnToken('<separator>')`, as its three parts.
fn split_binding(statement: &str) -> Option<(&str, &str, String)> {
    let (name, value) = statement.strip_prefix("def ")?.split_once(" = ")?;
    let (receiver, arguments) = value.split_once(".splitOnToken(")?;
    Some((name.trim(), receiver.trim(), quoted_first(arguments)?))
}

/// A delimited string cut into a LIST OF RECORDS.
///
/// The outer separator cuts items and the inner one cuts each item into
/// positional members, whose NAMES come off the script because the vendor
/// writes one ternary per position. falco's `container.mounts` is the pattern:
/// `<source>:<dest>:<mode>:<rdrw>:<propagation>` per mount, space separated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SplitIntoRecords {
    source: String,
    target: String,
    outer: String,
    inner: String,
    /// Member name and the position in the item it reads, in the order the
    /// script assigns them. A position the item does not reach is written
    /// null, which is what `parts.length > N ? parts[N] : null` says.
    ///
    /// Painless `[:]` is a `HashMap`, so what Elasticsearch iterates is the
    /// hash's order rather than the writer's. Nothing here can settle which,
    /// because `compat.py` writes its captures with `sort_keys=True` and the
    /// comparison holds two maps equal whatever their key order -- so the
    /// order the vendor WROTE is what is kept.
    members: Vec<(String, usize)>,
    /// The `else` arm writes an explicit null rather than leaving the target
    /// absent. Elasticsearch stores that null and `has_value` fails it, so an
    /// absent field and a null one are not the same document.
    null_when_absent: bool,
}

/// Read both splits, the record's members and the target off the script.
fn parse_split_into_records(script: &str) -> Option<SplitIntoRecords> {
    let statements: Vec<&str> = script.split([';', '\n']).map(str::trim).collect();

    // The two locals the loop fills: the record, and the list it is added to.
    let record = statements
        .iter()
        .find_map(|statement| statement.strip_prefix("def ")?.strip_suffix(" = [:]"))?;
    let list = statements
        .iter()
        .find_map(|statement| statement.strip_prefix("def ")?.strip_suffix(" = []"))?;

    let splits: Vec<(&str, &str, String)> = statements
        .iter()
        .filter_map(|statement| split_binding(statement))
        .collect();
    if splits.len() != 2 {
        return None;
    }

    // Which split is the OUTER one is decided by what it READS -- the document
    // for the outer, a local for the inner -- not by which comes first.
    let outer_at = (0..2).find(|at| resolve_subject(script, splits[*at].1, 3).is_some())?;
    let inner_at = 1 - outer_at;
    let source = resolve_subject(script, splits[outer_at].1, 3)?;
    // The inner split has to cut an ITEM of the outer one. Two splits over
    // unrelated values would build a record from parts that never shared an
    // item.
    if !local_expression(script, splits[inner_at].1)?
        .trim_start()
        .starts_with(splits[outer_at].0)
    {
        return None;
    }

    let subscript = format!("{}[", splits[inner_at].0);
    let prefix = format!("{record}.");
    let mut members = Vec::new();
    for statement in &statements {
        let Some(rest) = statement.strip_prefix(prefix.as_str()) else {
            continue;
        };
        let (name, value) = rest.split_once(" = ")?;
        // A member read some other way is a script this cannot reproduce, and
        // half a record is worse than none of it.
        let index = value
            .split_once(subscript.as_str())?
            .1
            .split(']')
            .next()?
            .trim()
            .parse::<usize>()
            .ok()?;
        members.push((name.trim().to_string(), index));
    }
    if members.is_empty() {
        return None;
    }

    let store = format!(" = {list}");
    let target = statements
        .iter()
        .find(|statement| statement.ends_with(store.as_str()))
        .and_then(|statement| painless_path(statement))?;
    let null_when_absent = statements.iter().any(|statement| {
        statement.ends_with(" = null") && painless_path(statement).as_deref() == Some(&target)
    });

    Some(SplitIntoRecords {
        source,
        target,
        outer: splits[outer_at].2.clone(),
        inner: splits[inner_at].2.clone(),
        members,
        null_when_absent,
    })
}

/// Cut the source twice and write the records, or the else arm's null.
fn run_split_into_records(event: &mut Event, pattern: &SplitIntoRecords) -> bool {
    let Some(text) = event.get_string(&pattern.source) else {
        if pattern.null_when_absent {
            let _ = event.set(&pattern.target, Value::Null);
        }
        return true;
    };
    let records: Vec<Value> = text
        .split(pattern.outer.as_str())
        .map(|item| {
            let parts: Vec<&str> = item.split(pattern.inner.as_str()).collect();
            let record: Map<String, Value> = pattern
                .members
                .iter()
                .map(|(name, at)| {
                    (
                        name.clone(),
                        parts.get(*at).map_or(Value::Null, |part| json!(part)),
                    )
                })
                .collect();
            Value::Object(record)
        })
        .collect();
    let _ = event.set(&pattern.target, Value::Array(records));
    true
}

/// The first list member a regex finds a match in, and WHAT it matched.
///
/// `matcher.find()` is a substring search and `group()` is the matched text
/// rather than the member: falco's tag `mitre_T1555_credential_access` yields
/// `T1555`. The walk stops at the first member that matches, because the
/// script breaks out of its loop.
#[derive(Debug, Clone)]
pub(crate) struct FirstMatchInList {
    list: String,
    target: String,
    /// The regex as the script spells it. `Regex` carries no equality of its
    /// own, so this is what the two impls below compare.
    pattern: String,
    matcher: regex::Regex,
    /// The script wraps the match in a one-element list.
    wrap_in_list: bool,
}

impl PartialEq for FirstMatchInList {
    fn eq(&self, other: &Self) -> bool {
        self.list == other.list
            && self.target == other.target
            && self.pattern == other.pattern
            && self.wrap_in_list == other.wrap_in_list
    }
}

impl Eq for FirstMatchInList {}

/// `def <rx> = /<pattern>/;` walked over a list, the first match written out.
///
/// The regex is COMPILED here, once per call site: Rust's `regex` has no
/// lookaround or backreferences where Java's does, so a pattern that will not
/// build declines the script instead of panicking on the first event to reach
/// it.
fn parse_first_match_in_list(script: &str) -> Option<FirstMatchInList> {
    let statements: Vec<&str> = script.split([';', '\n']).map(str::trim).collect();

    let pattern = statements.iter().find_map(|statement| {
        let value = statement.strip_prefix("def ")?.split_once(" = ")?.1.trim();
        value.strip_prefix('/')?.strip_suffix('/')
    })?;
    let matcher = regex::Regex::new(pattern).ok()?;

    // What `matcher(...)` was handed, and the list that local indexes.
    //
    // The `?.` is dropped before the read because `painless_path` finds a root
    // spelled `ctx.` or `ctx[` and falco writes `ctx?.falco?.tags`, which is
    // neither -- it skips a `?` inside a path but not at the root.
    let subject = script.split_once(".matcher(")?.1.split_once(')')?.0.trim();
    let indexed = local_expression(script, subject)?.trim();
    let list = painless_path(&indexed.rsplit_once('[')?.0.replace("?.", "."))?;

    let assignment = statements
        .iter()
        .find(|statement| statement.contains(".group()"))?;
    let (head, value) = split_assignment_once(assignment)?;

    Some(FirstMatchInList {
        list,
        target: painless_path(head)?,
        pattern: pattern.to_string(),
        matcher,
        wrap_in_list: value.trim().starts_with('['),
    })
}

/// Write what the first matching member matched, or nothing at all.
fn run_first_match_in_list(event: &mut Event, pattern: &FirstMatchInList) -> bool {
    let Some(Value::Array(items)) = event.get(&pattern.list) else {
        return true;
    };
    let Some(found) = items
        .iter()
        .filter_map(Value::as_str)
        .find_map(|item| pattern.matcher.find(item))
        .map(|found| found.as_str().to_string())
    else {
        return true;
    };
    let value = if pattern.wrap_in_list {
        Value::Array(vec![json!(found)])
    } else {
        json!(found)
    };
    let _ = event.set(&pattern.target, value);
    true
}

/// An integer field divided by a literal and written back, through a local.
///
/// [`parse_guarded_divide`] reads the divide written inline in the assignment;
/// this reads the one written through a local, which is the form a script must
/// use when it needs the quotient twice. falco's `evt.time.iso8601` is
/// nanoseconds, and the quotient is both that field and the `@timestamp`
/// beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LongDivide {
    source: String,
    target: String,
    divisor: i64,
}

/// `<type> <local> = <expr> / <literal>; ctx.<target> = <local>;`
fn parse_long_divide(script: &str) -> Option<LongDivide> {
    for statement in script.split([';', '\n']).map(str::trim) {
        let Some((head, value)) = split_assignment_once(statement) else {
            continue;
        };
        // A bare local on the right, which is what makes this the two-statement
        // form rather than the inline one.
        let local = value.trim();
        if local.is_empty()
            || !local.chars().all(|c| c.is_alphanumeric() || c == '_')
            || local.starts_with(|c: char| c.is_ascii_digit())
        {
            continue;
        }
        let (Some(target), Some(bound)) = (painless_path(head), local_expression(script, local))
        else {
            continue;
        };
        let Some((expression, divisor)) = bound.rsplit_once('/') else {
            continue;
        };
        // A plain integer divisor only. A fractional one gives a fractional
        // result, which is a different type from the one this writes.
        let Some(divisor) = divisor
            .trim()
            .trim_end_matches(['L', 'l'])
            .parse::<i64>()
            .ok()
            .filter(|literal| *literal != 0)
        else {
            continue;
        };
        // The divide has to sit under the guard that says the value IS an
        // integer, or the runner's own type test is a guess rather than the
        // script's.
        let expression = expression.trim();
        if !script.contains(&format!("{expression} instanceof Long")) {
            continue;
        }
        let Some(source) = resolve_subject(script, expression, 3) else {
            continue;
        };
        return Some(LongDivide {
            source,
            target,
            divisor,
        });
    }
    None
}

/// Divide and write back, but only where the value really is an integer.
///
/// `as_i64` IS the script's `instanceof Long` guard. A string there takes the
/// date-parsing branch, which is deliberately not carried -- writing nothing
/// beats guessing at a Java `Date`'s serialisation.
fn run_long_divide(event: &mut Event, pattern: &LongDivide) -> bool {
    let Some(value) = event.get(&pattern.source).and_then(Value::as_i64) else {
        return true;
    };
    let _ = event.set(&pattern.target, json!(value / pattern.divisor));
    true
}

/// A list led by one field, then every part of a second field's split.
///
/// falco's `process.args` is the executable's path followed by its argument
/// string cut on spaces. Both fields are required, which is the script's own
/// two-clause guard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrependSplit {
    head: String,
    list: String,
    separator: String,
    target: String,
}

/// `<out>.add(ctx.<head>); for (...) { <out>.add(<items>[i]); } ctx.<t> = <out>;`
fn parse_prepend_split(script: &str) -> Option<PrependSplit> {
    let statements: Vec<&str> = script.split([';', '\n']).map(str::trim).collect();

    let out = statements
        .iter()
        .find_map(|statement| statement.strip_prefix("def ")?.strip_suffix(" = []"))?;
    let splits: Vec<(&str, &str, String)> = statements
        .iter()
        .filter_map(|statement| split_binding(statement))
        .collect();
    let [(items, receiver, separator)] = splits.as_slice() else {
        return None;
    };

    // The two appends are not interchangeable: the FIRST is the scalar that
    // leads and the second walks the split.
    let append = format!("{out}.add(");
    let appended: Vec<&str> = statements
        .iter()
        .filter_map(|statement| statement.strip_prefix(append.as_str())?.strip_suffix(')'))
        .collect();
    let [first, rest] = appended.as_slice() else {
        return None;
    };
    if !rest.trim().starts_with(items) {
        return None;
    }

    let store = format!(" = {out}");
    Some(PrependSplit {
        head: resolve_subject(script, first.trim(), 3)?,
        list: resolve_subject(script, receiver, 3)?,
        separator: separator.clone(),
        target: statements
            .iter()
            .find(|statement| statement.ends_with(store.as_str()))
            .and_then(|statement| painless_path(statement))?,
    })
}

/// Build the list, and only where BOTH fields are there to build it from.
fn run_prepend_split(event: &mut Event, pattern: &PrependSplit) -> bool {
    let (Some(first), Some(text)) = (
        event.get(&pattern.head).cloned(),
        event.get_string(&pattern.list),
    ) else {
        return true;
    };
    let mut parts = Vec::with_capacity(text.matches(pattern.separator.as_str()).count() + 2);
    parts.push(first);
    parts.extend(text.split(pattern.separator.as_str()).map(|part| json!(part)));
    let _ = event.set(&pattern.target, Value::Array(parts));
    true
}

/// Check if a Painless script source matches a known pattern.
///
/// Returns true if the script was handled, false if it should fall through
/// to the generic `painless_exec` stub.
///
/// The dispatch is two halves. [`known_patterns`] reads the script TEXT and
/// names the matchers it triggers -- a decision that never changes for a given
/// script, which is why [`crate::painless_plan::PainlessPlan`] makes it once
/// per call site. [`run_known_pattern`] then runs one matcher against one event.
/// This entry point does both per call, for callers without a plan.
pub fn try_known_painless(event: &mut Event, script: &str) -> bool {
    let normalised = normalise(script);
    known_patterns(&normalised)
        .iter()
        .any(|pattern| run_known_pattern(event, &normalised, pattern))
}

/// A matcher branch of the text-only dispatch, with whatever the trigger's own
/// parse already recovered from the script.
///
/// The variants up to `KeysToSnakeCase` recognise what a script DOES and work
/// for any source that writes the pattern; the rest are keyed on a vendor's
/// FIELD NAMES and recognise whose script it is, ending in the two catch-alls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KnownPattern {
    /// A scalar written from arithmetic over other fields.
    ScalarExpression(Box<crate::painless_expr::ScalarExpression>),
    /// A member copied out of whichever sibling key the payload carries.
    MemberFromVariantKey(Box<MemberFromVariantKey>),
    /// Members renamed inside every item of a list.
    ListItemRenames(Box<crate::painless_lists::ListItemRenames>),
    /// ECS fields written from the members of a list's items, under nested
    /// guards.
    ItemWrites(Box<crate::painless_item_writes::ItemWrites>),
    /// A duration, and the window it puts around a timestamp.
    DurationWindow(Box<DurationWindow>),
    /// Totals summed from two sides, an absent side counting as zero.
    SumTotals(Box<crate::painless_totals::SumTotals>),
    DropEmpty {
        policy: DropPolicy,
        root: Option<String>,
    },
    SplitCommandLine(crate::painless_windows::ArgvScript),
    Basename(Box<BasenameCuts>),
    FileInfo(String),
    HashLowercase(String),
    PrefixTail {
        source: String,
        prefix: String,
        strip_comma: bool,
        target: String,
    },
    PrependToArray {
        scalar: String,
        array: String,
        target: String,
    },
    ResourcesRenameDedup(String),
    SecurityhubResource(String),
    SecurityhubResources(String),
    InspectorResources {
        multi: bool,
    },
    MailRelated(Box<MailRelatedScript>),
    GcpRelatedEntity,
    RemoveListValue {
        field: String,
        value: String,
    },
    RemoveMapValue {
        field: String,
        value: String,
    },
    DatePlusDays {
        source: String,
        target: String,
        days: i64,
    },
    NestedKeyNames {
        source: String,
        inner: String,
        target: String,
    },
    FirehoseDataset,
    CtxTableLookup {
        table: String,
        key: String,
        target: String,
    },
    KvIntoNamespace(String),
    KvIntoFields(String),
    /// A value copied only when a literal set holds it.
    AllowedValueCopy(AllowedValueCopy),
    /// A number written back as an octal string.
    OctalString(OctalString),
    /// A value appended to a list the script first ensures exists.
    EnsureAppend(EnsureAppend),
    /// A composite key joined from whichever named fields are present.
    JoinPresentFields(JoinPresentFields),
    /// A vendor flag folded to a boolean by its string spelling.
    CoerceBoolean(CoerceBoolean),
    RemoveEmptyChildMaps(RemoveEmptyChildMaps),
    EnsurePrefix(EnsurePrefix),
    SplitAtDelimiter(SplitAtDelimiter),
    MailtoUriFields(MailtoUriFields),
    MoveMapEntry(MoveMapEntry),
    RenameMapKeys(RenameMapKeys),
    ParametersIntoMap(ParametersIntoMap),
    UnwrapSuffixedKeys(UnwrapSuffixedKeys),
    /// An ECS `geo_point` built from a `GeoJSON` coordinate array.
    GeoPointFromCoordinates(GeoPointFromCoordinates),
    /// A `GeoJSON` geometry rendered as WKT, with the point and the copies the
    /// same script writes.
    WktGeometry(Box<WktGeometry>),
    /// A numbered column map collapsed into a list, in key order.
    CsvMapToArray {
        source: String,
        target: String,
    },
    /// Tag names scrubbed into a list, the prefixed ones into a marking map.
    TagsAndMarking(Box<TagsAndMarking>),
    /// The expiry window from an epoch base, plus the already-expired flag.
    DecayWindow(Box<DecayWindow>),
    /// `Key: value` columns into a map, their keys into a fingerprint.
    CsvColonPairs {
        list: String,
        map_target: String,
        fingerprint_target: String,
        aliases: Vec<(String, String)>,
    },
    DedupeUnwrap(String),
    DropLastChar(String),
    /// Drop the first `count` characters of a string field, in place.
    DropLeadingChars {
        field: String,
        count: usize,
    },
    /// Named epoch fields rescaled to milliseconds by their own magnitude.
    EpochToMillis(Box<EpochToMillis>),
    M365ProcessEvidence(String),
    M365IdentityEvidence(String),
    Route53Answers,
    ReverseLookupAddress,
    TruthyAssignments(Vec<(String, String)>),
    LongOperationSession {
        first: String,
        last: String,
    },
    ScriptBlockEntropy(String),
    SplitOnPipe(Vec<String>),
    ZipAttachments(Box<AttachmentZip>),
    ZipColumns(Box<ColumnZip>),
    MaxByContains(Box<MaxByContains>),
    CheckpointPackets,
    ConsoleLoginEventData,
    BitFlagNames(Box<BitFlagNames>),
    BitNameArray(Box<BitNameArray>),
    PascalKeys(Box<PascalKeys>),
    SecondsToSpan(String),
    SubstringBeforeLast {
        source: String,
        target: String,
        needle: String,
    },
    TlsVersionSplit {
        source: String,
    },
    ConcatParts(ConcatScript),
    /// A string joined from named fields and literal separators, in ONE
    /// assignment.
    ConcatAssignment(ConcatAssignment),
    TrimListInPlace(String),
    StartsWithAppend {
        source: String,
        prefix: String,
        target: String,
        value: String,
    },
    CategoryTypeLadder(Vec<CategoryArm>),
    KeysStripWhitespace(String),
    /// One map's keys rebuilt by the replacements the script spells out.
    RewriteKeys(Box<RewriteKeys>),
    SnakeKeyMapCopy {
        source: String,
        target: String,
    },
    StripAnglePairs {
        scalars: Vec<String>,
        lists: Vec<String>,
    },
    NameValueFold {
        source: String,
        target: String,
        key_member: String,
        value_member: String,
    },
    OutcomeFromTags {
        action_field: String,
        tags: String,
        target: String,
    },
    RenameCommonAuth(Vec<String>),
    ProcessCreated(Vec<String>),
    ShareFilePath(Vec<String>),
    ObjectDn,
    CollectColumns(Box<CollectColumns>),
    SliceEachItem(Box<SliceEachItem>),
    ContainsLadder(Box<ContainsLadder>),
    RangeLadder(Box<RangeLadder>),
    ScoreSeverityBands(Box<ScoreSeverityBands>),
    IndexedFieldCopies(Box<IndexedFieldCopies>),
    FieldsIntoMap(Box<FieldsIntoMap>),
    CrowdstrikeTimelineEntityIdentity,
    CrowdstrikeTimelineEntityAccounts,
    FirstElement(Box<FirstElement>),
    NamedMapEntry(Box<NamedMapEntry>),
    TitleCase(Box<TitleCase>),
    SuffixAfterSeparator {
        source: String,
        target: String,
        separator: String,
        /// Whether a source with no separator in it writes the whole value.
        ///
        /// The inline spellings write nothing, because the vendor guards the
        /// cut and has no else arm. The local-index spelling has one.
        whole_when_absent: bool,
    },
    CopyTargetUser(Vec<String>),
    CopySubjectUser(Vec<String>),
    CopyMemberName(Vec<String>),
    CopyComputerObject(Vec<String>),
    CopyUserToBase {
        codes: Vec<String>,
        base: String,
        sid_field: String,
    },
    SplitPipeFields(Vec<String>),
    SplitTokenField(Box<SplitToken>),
    TrimThenSplit(Box<TrimThenSplit>),
    DecodeBase64 {
        source: String,
        target: String,
    },
    TokenCount {
        source: String,
        separator: String,
        target: String,
    },
    WrapValueInList {
        source: String,
        target: String,
        /// The script drops the source's own key once it is wrapped, which is
        /// what MOVES a value into its plural sibling rather than copying it.
        remove_source: bool,
    },
    LastElement {
        array: String,
        target: String,
    },
    ClassifyMembers,
    FlattenedDuplicates,
    CollectEntities,
    DnsRdataAnswers,
    StructuredRdataAnswers,
    RelatedFromDnsAnswers,
    AnswersFromResolvedIp,
    CamelToSnake {
        target: String,
        source: String,
        rule: SnakeRule,
        drop_at_keys: bool,
        /// The script wrote `putAll`, so the converted keys join whatever the
        /// target already holds rather than replacing it.
        merge: bool,
        removes: Vec<String>,
    },
    SplitTrimCollect,
    SumDirections(Vec<&'static str>),
    CombineFields(Box<CombineFields>),
    DurationToNanos,
    DedupeMapValues(Box<DedupeMapValues>),
    EmailsToRelatedUsers(Box<EmailsToRelatedUsers>),
    SnakeCaseListElements(Box<SnakeCaseListElements>),
    SumMemberOverList(Box<SumMemberOverList>),
    FirstPresentKeyName(Box<FirstPresentKeyName>),
    FloatSecondsToNanos(Box<FloatSecondsToNanos>),
    FlowDuration,
    ParallelDispatch,
    ConcatMessage,
    SwapSubtrees,
    CollectingLadder,
    CaseInsensitiveLadder,
    EqualityLadder(Ladder),
    SentinelRemovalLiteral,
    ListMemberSelect(Box<ListMemberSelect>),
    RowLookupWithFallback(Box<RowLookup>),
    SchemelessUrl,
    VersionSplit,
    SyslogPriority(SyslogPriorityScript),
    AppendUnique {
        from: &'static str,
        into: &'static str,
    },
    SplitUnquotedKv(Box<SplitKv>),
    ArrayToIndexedObject,
    ListRenameTable(Box<ListRenameTable>),
    KeyValuePairs,
    JoinOptional,
    AppendEach(Box<AppendEach>),
    LiteralValueMap(Box<LiteralValueMap>),
    KeysToSnakeCase(Option<String>, SnakeRule),
    CommandLine {
        parent: bool,
    },
    ProcessStartTime,
    EmailSplit(Box<EmailSplit>),
    RiskBehaviors,
    AzureCategoryEventType,
    AzureEventCategory,
    ReplaceDotsInKeys,
    OktaTargetRename,
    KeysBySuffix(Box<KeysBySuffix>),
    CollectMapValues,
    StringOps(Box<StringOps>),
    GuardedReplace(Box<GuardedReplace>),
    ScaleField(Box<ScaleField>),
    MillisecondLadder(Box<MillisecondLadder>),
    RoundedScale(Box<RoundedScale>),
    NanosBetween(Box<NanosBetween>),
    ProcessNameFromCommandLine,
    FlagsPresent(Box<FlagsPresent>),
    ZipLists(Box<ZipLists>),
    FlattenMapInto(Box<FlattenMapInto>),
    StringifyLongs(Vec<String>),
    CopyByLabel(Box<CopyByLabel>),
    BandLadder(Box<BandLadder>),
    IocExpiry(Box<IocExpiry>),
    WrapMapInList(String),
    FirstOrSelf(Box<FirstOrSelf>),
    GuardedDivide {
        target: String,
        /// The dividend. USUALLY the target -- aws divides in place -- but
        /// cyberarkpas reads a vendor field and writes an ECS one.
        source: String,
        absent: Option<String>,
        divisor: i64,
    },
    /// Both catch-alls carry the guarded-literal tail already parsed, so the
    /// four-hundred-line bodies are read once per call site, not per event.
    GuardedCopy(Program),
    PlainAssignments(Program),
    ClassifyLadder(Box<ClassifyLadder>),
    MoveKeys(Vec<(String, String)>),
    MergeMapUp(Box<MergeMapUp>),
    ParameterFanOut(Box<ParameterFanOut>),
    QuotedKvScan(Box<QuotedKvScan>),
    LocalMapLookup(Box<LocalMapLookup>),
    MapEntryToBoolean(Box<MapEntryToBoolean>),
    NestUnder(Box<NestUnder>),
    CollectFromList(Box<CollectFromList>),
    RecordLookup(Box<RecordLookup>),
    EqualsLiteralFlag(Box<EqualsLiteralFlag>),
    PositionInList(Box<PositionInList>),
    SplitFirstLabel(Box<SplitFirstLabel>),
    HashesByLength(Box<HashesByLength>),
    UnreservedKeyPayload(Box<UnreservedKeyPayload>),
    SuffixesByPrefix(Box<SuffixesByPrefix>),
    BranchCopies(Vec<BranchCopy>),
    /// A field unwrapped from a surrounding pair of characters, in place.
    StripSurroundingPair(Box<StripSurroundingPair>),
    /// A value cut at the Nth separator counted from its END, the prefix kept.
    NthSeparatorPrefix(Box<crate::painless_nth_separator::NthSeparatorPrefix>),
    PrivateCidrDirection {
        source: String,
        destination: String,
        target: String,
    },
    /// A delimited string cut twice, into a list of positional records.
    SplitIntoRecords(Box<SplitIntoRecords>),
    /// A list led by one field, then the parts of a second field's split.
    PrependSplit(Box<PrependSplit>),
    /// What a regex matched in the first list member it matched at all.
    FirstMatchInList(Box<FirstMatchInList>),
    /// An integer divided by a literal and written back, through a local.
    LongDivide(Box<LongDivide>),
}

/// Copies that apply only where `guard` holds `literal`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BranchCopy {
    guard: String,
    literal: String,
    copies: Vec<(String, String)>,
}

/// The matcher branches this script's text triggers, in dispatch order.
///
/// A branch whose matcher is guarded by its own parse -- one the old inline
/// dispatch spelled `trigger && try_x(event, ...)` -- FALLS THROUGH when the
/// matcher declines, so every trigger after it that also holds is included,
/// up to and including the first `return try_x(...)` branch, after which
/// nothing could ever run. The runner walks the list until a matcher returns
/// true, which reproduces the old chain exactly minus the per-event scans.
///
/// The trigger order is load-bearing; each comment that says why a branch sits
/// where it does travelled here with it.
#[allow(clippy::too_many_lines)] // A transliteration of the dispatch ladder; splitting it would hide the order.
pub(crate) fn known_patterns(normalised: &str) -> Vec<KnownPattern> {
    let mut patterns = Vec::new();

    // Pattern: seconds to nanoseconds, closing the span it opens. Ahead of the
    // scale-by-literal fallback, which reads the same multiply and stops
    // there, leaving `event.end` unwritten.
    if normalised.contains("ChronoUnit.NANOS")
        && let Some(source) = normalised
            .split_once("(int) (ctx.")
            .and_then(|(_, rest)| rest.split_once(" *"))
            .map(|(path, _)| crate::painless_params::clean_path(path))
    {
        patterns.push(KnownPattern::SecondsToSpan(source));
        return patterns;
    }

    // Pattern: the span between two parsed instants, in nanoseconds. The
    // defender and crowdstrike pipelines derive `event.duration` this way.
    if normalised.contains("ChronoUnit.NANOS.between(")
        && let Some(pattern) = parse_nanos_between(normalised)
    {
        patterns.push(KnownPattern::NanosBetween(Box::new(pattern)));
        return patterns;
    }

    // Pattern: every number under one subtree whose FIELD NAME reads as a
    // timestamp, normalised to milliseconds by magnitude. Leads with the other
    // rescale patterns because the `GuardedDivide` and `ScaleField` catch-alls
    // read this ladder's `/ 1000` and stop there, rescaling one field.
    if normalised.contains("instanceof Number")
        && normalised.contains(".endsWith(")
        && let Some(pattern) = parse_millisecond_ladder(normalised)
    {
        patterns.push(KnownPattern::MillisecondLadder(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a lone object wrapped in a one-element list so the `foreach`
    // after it has something to walk. Ahead of every other `instanceof Map`
    // branch because it is the SHORTEST of them and the rest would read its
    // single assignment as their own.
    if normalised.contains("instanceof Map")
        && let Some(path) = parse_wrap_map_in_list(normalised)
    {
        patterns.push(KnownPattern::WrapMapInList(path));
        return patterns;
    }

    // Pattern: the reverse -- a one-element list taken down to the object it
    // holds, so the renames after it can name one path.
    if normalised.contains("instanceof Map")
        && normalised.contains("instanceof List")
        && let Some(pattern) = parse_first_or_self(normalised)
    {
        patterns.push(KnownPattern::FirstOrSelf(Box::new(pattern)));
        return patterns;
    }

    // Pattern: the keys of a fixed list that a map marks `"true"`.
    if normalised.contains("instanceof Map")
        && normalised.contains("] == \"true\"")
        && let Some(pattern) = parse_flags_present(normalised)
    {
        patterns.push(KnownPattern::FlagsPresent(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a whole map moved beneath a NEW parent. Ahead of the merge
    // patterns, whose `entrySet()` and `getKey()` triggers this also spells and
    // which would lift its members to the wrong level.
    if normalised.contains(": new HashMap()]")
        && normalised.contains(".entrySet()")
        && let Some(pattern) = parse_nest_under(normalised)
    {
        patterns.push(KnownPattern::NestUnder(Box::new(pattern)));
        return patterns;
    }

    // Pattern: one list's members collected into deduped arrays. Ahead of the
    // copy patterns, whose `.add(` this also spells and which cannot walk a list,
    // so cisco_secure_endpoint's `host.ip`, `host.mac` and both `related`
    // arrays were never written on any of its 408 events.
    // `.isEmpty()` is one of the two guards a vendor writes -- gdacs checks
    // `!= null` alone -- so the trigger is the loop and the append, and the
    // PARSE decides.
    // Ahead of the collect below: that reads a member name off the loop body
    // and takes `email.contains('@')` for one, claiming netskope's script and
    // writing `related.user` from a member called `contains`.
    if normalised.contains("splitOnToken(")
        && let Some(pattern) = parse_emails_to_related_users(normalised)
    {
        patterns.push(KnownPattern::EmailsToRelatedUsers(Box::new(pattern)));
        return patterns;
    }

    if (normalised.contains(" in ctx") || normalised.contains(" : "))
        && normalised.contains(".add(")
        && let Some(pattern) = parse_collect_from_list(normalised)
    {
        patterns.push(KnownPattern::CollectFromList(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a value looked up in a LIST of records. sophos runs its whole
    // config through this -- the log's timezone abbreviation and the device
    // serial both -- and losing the timezone made its date processor fail,
    // whose `on_failure` then REMOVES `event.timezone` altogether.
    if normalised.contains("for (def ")
        && let Some(pattern) = parse_record_lookup(normalised)
    {
        patterns.push(KnownPattern::RecordLookup(Box::new(pattern)));
        return patterns;
    }

    // Pattern: the value this document's POSITION in a list decides. Early,
    // beside the other `instanceof Map` walks, so nothing downstream reads its
    // three-armed index ladder as a value table over a field.
    if normalised.contains("instanceof Map")
        && normalised.contains(".size() - 1")
        && let Some(pattern) = parse_position_in_list(normalised)
    {
        patterns.push(KnownPattern::PositionInList(Box::new(pattern)));
        return patterns;
    }

    // Pattern: one list cut two ways, a column per cut.
    if normalised.contains(".substring(")
        && normalised.contains(".add(")
        && let Some(pattern) = parse_slice_each_item(normalised)
    {
        patterns.push(KnownPattern::SliceEachItem(Box::new(pattern)));
        return patterns;
    }

    // Pattern: the tail of every list member whose named field carries a
    // prefix. Behind the cut above, whose `.substring(` and `.add(` triggers
    // this also spells and which declines on it.
    if normalised.contains(".substring(")
        && normalised.contains(".add(")
        && let Some(pattern) = parse_suffixes_by_prefix(normalised)
    {
        patterns.push(KnownPattern::SuffixesByPrefix(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a type tag choosing which field one value lands on.
    if normalised.contains(".contains('")
        && let Some(pattern) = parse_contains_ladder(normalised)
    {
        patterns.push(KnownPattern::ContainsLadder(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a shouted vendor value title-cased onto its ECS field.
    if normalised.contains(".substring(1).toLowerCase()")
        && let Some(pattern) = parse_title_case(normalised)
    {
        patterns.push(KnownPattern::TitleCase(Box::new(pattern)));
        return patterns;
    }

    // Pattern: one named entry lifted out of a map, or a literal in its place.
    if normalised.contains(".entrySet()")
        && normalised.contains(".getKey()")
        && let Some(pattern) = parse_named_map_entry(normalised)
    {
        patterns.push(KnownPattern::NamedMapEntry(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a dotted name split into its first label and the rest of it.
    if normalised.contains(".splitOnToken(")
        && normalised.contains("[0]")
        && let Some(pattern) = parse_split_first_label(normalised)
    {
        patterns.push(KnownPattern::SplitFirstLabel(Box::new(pattern)));
        return patterns;
    }

    // Pattern: one entry of a map retyped from its digit spelling to a boolean.
    if normalised.contains(".containsKey(")
        && normalised.contains(".equals(")
        && let Some(pattern) = parse_map_entry_to_boolean(normalised)
    {
        patterns.push(KnownPattern::MapEntryToBoolean(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a list of hashes split into typed fields by the LENGTH of each.
    if normalised.contains(".length() == ")
        && normalised.contains("instanceof List")
        && let Some(pattern) = parse_hashes_by_length(normalised)
    {
        patterns.push(KnownPattern::HashesByLength(Box::new(pattern)));
        return patterns;
    }

    // Pattern: the one key of an envelope that is not envelope names the
    // event, and its value is the payload.
    if normalised.contains(".keySet()")
        && normalised.contains("new HashSet(")
        && let Some(pattern) = parse_unreserved_key_payload(normalised)
    {
        patterns.push(KnownPattern::UnreservedKeyPayload(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a nested map emptied into an ancestor, and the routing keys
    // beside it dropped. The trigger is the PARSE, with only a cheap reject in
    // front: gating on `.getValue()` gated on one of the two spellings, and
    // the nine axonius streams write the other.
    if (normalised.contains(".entrySet()") || normalised.contains(".keySet()"))
        && let Some(pattern) = parse_merge_map_up(normalised)
    {
        patterns.push(KnownPattern::MergeMapUp(Box::new(pattern)));
        return patterns;
    }
    if normalised.contains(".putAll(")
        && let Some(pattern) = parse_put_all(normalised)
    {
        patterns.push(KnownPattern::MergeMapUp(Box::new(pattern)));
        return patterns;
    }
    if normalised.contains("for (int ")
        && normalised.contains("[\"name\"]]")
        && let Some(pattern) = parse_parameter_fan_out(normalised)
    {
        patterns.push(KnownPattern::ParameterFanOut(Box::new(pattern)));
        return patterns;
    }
    if normalised.contains("kvStart")
        && let Some(pattern) = parse_quoted_kv_scan(normalised)
    {
        patterns.push(KnownPattern::QuotedKvScan(Box::new(pattern)));
        return patterns;
    }
    // Pattern: a lookup table written inline rather than shipped in `params`.
    if normalised.contains(".containsKey(")
        && normalised.contains("': '")
        && let Some(pattern) = parse_local_map_lookup(normalised)
    {
        patterns.push(KnownPattern::LocalMapLookup(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a score named by the band it falls in.
    if normalised.contains("<=")
        && normalised.contains("&&")
        && let Some(pattern) = parse_range_ladder(normalised)
    {
        patterns.push(KnownPattern::RangeLadder(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a number banded into a label, in the spelling `RangeLadder`
    // above does NOT parse -- comparisons written subject-first
    // (`value > 0 && value < 30`) rather than literal-first, arms that may
    // land in a LOCAL one `.put()` writes at the end, and an `||` guard for
    // the out-of-range band. Every threat-intel package normalises its
    // vendor confidence this way.
    // Either joiner: a band may be `a && b`, an out-of-range `a || b`, or a
    // single open-ended bound in a ladder that has one of the other two.
    if (normalised.contains("&&") || normalised.contains("||"))
        && let Some(pattern) = parse_band_ladder(normalised)
    {
        patterns.push(KnownPattern::BandLadder(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a risk score cast to double, then a ceiling-only ladder over
    // the same source names its severity band. No `&&` in the guard, unlike
    // `RangeLadder` above, so the two triggers never both fire.
    if normalised.contains(" = (double) ")
        && normalised.contains("} else {")
        && let Some(pattern) = parse_score_severity_bands(normalised)
    {
        patterns.push(KnownPattern::ScoreSeverityBands(Box::new(pattern)));
        return patterns;
    }

    // Pattern: one column per member, gathered off a list of objects.
    if normalised.contains(" : ctx.")
        && normalised.contains(".isEmpty()")
        && let Some(pattern) = parse_collect_columns(normalised)
    {
        patterns.push(KnownPattern::CollectColumns(Box::new(pattern)));
        return patterns;
    }

    // Pattern: parallel lists zipped into a list of objects.
    if normalised.contains(".add([")
        && normalised.contains("new ArrayList()")
        && let Some(pattern) = parse_zip_lists(normalised)
    {
        patterns.push(KnownPattern::ZipLists(Box::new(pattern)));
        return patterns;
    }

    // Pattern: the executables of `process.command_line` folded into
    // `process.name`. The defender pipelines share one copy of this script.
    if normalised.contains("currentNames") && normalised.contains("ctx.process.command_line") {
        patterns.push(KnownPattern::ProcessNameFromCommandLine);
        return patterns;
    }

    // Pattern: a value appended to a list the script builds level by level.
    if normalised.contains(" ?: [];")
        && normalised.contains(".add(")
        && let Some(pattern) = parse_ensure_append(normalised)
    {
        patterns.push(KnownPattern::EnsureAppend(pattern));
        return patterns;
    }

    // Pattern: a composite key joined from whichever fields are present.
    if normalised.contains("String.join(")
        && normalised.contains(".add(")
        && let Some(pattern) = parse_join_present_fields(normalised)
    {
        patterns.push(KnownPattern::JoinPresentFields(pattern));
        return patterns;
    }

    // Pattern: keys carrying a suffix rewritten without it.
    if normalised.contains(".endsWith(")
        && normalised.contains(".keySet()")
        && normalised.contains(".length() - ")
        && let Some(pattern) = parse_unwrap_suffixed_keys(normalised)
    {
        patterns.push(KnownPattern::UnwrapSuffixedKeys(pattern));
        return patterns;
    }

    // Pattern: a list of {name, value} parameters fanned out into a map.
    if normalised.contains(".toLowerCase()")
        && normalised.contains("for (def ")
        && let Some(pattern) = parse_parameters_into_map(normalised)
    {
        patterns.push(KnownPattern::ParametersIntoMap(pattern));
        return patterns;
    }

    // Pattern: a duration and the window it puts around a timestamp.
    if normalised.contains("ChronoUnit.NANOS")
        && normalised.contains("ctx.event.duration = ")
        && let Some(pattern) = parse_duration_window(normalised)
    {
        patterns.push(KnownPattern::DurationWindow(Box::new(pattern)));
        return patterns;
    }

    // Pattern: members renamed inside every item of a list, atlassian's cloud
    // streams reshaped to match their self-hosted twins.
    if normalised.contains(".length; j++)")
        && normalised.contains(".put(")
        && normalised.contains(".remove(")
        && let Some(pattern) = crate::painless_lists::parse_list_item_renames(normalised)
    {
        patterns.push(KnownPattern::ListItemRenames(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a named member lifted out of whichever of several sibling keys
    // the payload carries.
    if normalised.contains(".keySet()")
        && normalised.contains("if (k ==")
        && let Some(pattern) = parse_member_from_variant_key(normalised)
    {
        patterns.push(KnownPattern::MemberFromVariantKey(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a whole subtree's keys rewritten at every depth and MOVED to a
    // new path. Ahead of both one-level readers, which would take the top level
    // and leave every nested map spelt the vendor's way.
    if normalised.contains("instanceof Map")
        && normalised.contains("instanceof List")
        && normalised.contains(".getKey()")
        && let Some(pattern) = parse_recursive_rewrite_keys(normalised)
    {
        patterns.push(KnownPattern::RewriteKeys(Box::new(pattern)));
        return patterns;
    }

    // Pattern: every key of a map rewritten by a CHAIN of transforms, written
    // as a `Collectors.toMap` one-liner. Ahead of the single-replacement arm
    // below, which reads only the first `.replace(` of such a chain.
    if normalised.contains("Collectors.toMap")
        && normalised.contains(".getKey()")
        && let Some(pattern) = parse_stream_rewrite_keys(normalised)
    {
        patterns.push(KnownPattern::RewriteKeys(Box::new(pattern)));
        return patterns;
    }

    // Pattern: the same fold written as an explicit loop into a local map.
    if normalised.contains(".entrySet()")
        && normalised.contains(".put(")
        && normalised.contains(".getKey()")
        && let Some(pattern) = parse_loop_rewrite_keys(normalised)
    {
        patterns.push(KnownPattern::RewriteKeys(Box::new(pattern)));
        return patterns;
    }

    // Pattern: every key of a map rewritten by one character replacement.
    if normalised.contains(".getKey().replace(")
        && let Some(pattern) = parse_rename_map_keys(normalised)
    {
        patterns.push(KnownPattern::RenameMapKeys(pattern));
        return patterns;
    }

    // Pattern: one entry moved out of a map by a key that holds a dot.
    if normalised.contains(".remove(")
        && (normalised.contains(" = obj;") || normalised.contains(", obj)"))
        && let Some(pattern) = parse_move_map_entry(normalised)
    {
        patterns.push(KnownPattern::MoveMapEntry(pattern));
        return patterns;
    }

    // Pattern: the ECS email block read out of a `mailto:` URI.
    if normalised.contains("attachment=")
        && normalised.contains("['from']")
        && let Some(pattern) = parse_mailto_uri_fields(normalised)
    {
        patterns.push(KnownPattern::MailtoUriFields(pattern));
        return patterns;
    }

    // Pattern: a field split at a delimiter, its halves written to targets.
    if normalised.contains(".indexOf")
        && normalised.contains(".substring(")
        && let Some(pattern) = parse_split_at_delimiter(normalised)
    {
        patterns.push(KnownPattern::SplitAtDelimiter(pattern));
        return patterns;
    }

    // Pattern: a message normalised to carry a known prefix.
    if normalised.contains(".charAt(0)")
        && normalised.contains(".substring(0, ")
        && let Some(pattern) = parse_ensure_prefix(normalised)
    {
        patterns.push(KnownPattern::EnsurePrefix(pattern));
        return patterns;
    }

    // Pattern: named keys dropped when what they hold is an empty map.
    if normalised.contains("instanceof Map")
        && normalised.contains(".size() == 0")
        && normalised.contains(".containsKey(")
        && let Some(pattern) = parse_remove_empty_child_maps(normalised)
    {
        patterns.push(KnownPattern::RemoveEmptyChildMaps(pattern));
        return patterns;
    }

    // Pattern: a vendor flag folded to a boolean by its string spelling.
    if normalised.contains(".toString().toLowerCase()")
        && let Some(pattern) = parse_coerce_boolean(normalised)
    {
        patterns.push(KnownPattern::CoerceBoolean(pattern));
        return patterns;
    }

    // Pattern: a number written back as octal, which is how a file mode reads.
    if normalised.contains("Integer.toOctalString(")
        && let Some(pattern) = parse_octal_string(normalised)
    {
        patterns.push(KnownPattern::OctalString(pattern));
        return patterns;
    }

    // Pattern: a copy gated on membership of a literal set, which is how a
    // vendor value reaches an ECS field with a closed vocabulary.
    if normalised.contains("].contains(")
        && let Some(pattern) = parse_allowed_value_copy(normalised)
    {
        patterns.push(KnownPattern::AllowedValueCopy(pattern));
        return patterns;
    }

    // Pattern: the same prune written as a walk over `keySet()`, which removes
    // the root's DIRECT values only -- descending would drop nested nulls the
    // script keeps. tychon ships it on every stream.
    if normalised.contains(".keySet())")
        && normalised.contains("for (key in keys)")
        && let Some(at) = normalised.find("ArrayList(ctx.")
        && let Some(root) = normalised[at + "ArrayList(ctx.".len()..]
            .split(".keySet()")
            .next()
        && !root.is_empty()
        && root
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '_')
    {
        let mut policy = DropPolicy::read(normalised);
        policy.shallow = true;
        patterns.push(KnownPattern::DropEmpty {
            policy,
            root: Some(root.to_string()),
        });
        return patterns;
    }

    // Pattern: drop null and empty values recursively. Matched on the PATTERN,
    // not the helper's name -- panw spells it `dropEmptyFields`, and keying
    // on `drop(ctx)` left every emptied object behind. What counts as empty
    // comes from the script's own predicate, which is not the same
    // everywhere; vpcflow runs it over ONE subtree rather than the document.
    if normalised.contains("removeIf")
        && normalised.contains("instanceof Map")
        && normalised.contains("instanceof List")
    {
        if normalised.contains("(ctx)") {
            patterns.push(KnownPattern::DropEmpty {
                policy: DropPolicy::read(normalised),
                root: None,
            });
            return patterns;
        }
        if let Some(at) = normalised.rfind("(ctx.")
            && let Some(root) = normalised[at + "(ctx.".len()..].split(')').next()
            && root
                .chars()
                .all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '?')
        {
            patterns.push(KnownPattern::DropEmpty {
                policy: DropPolicy::read(normalised),
                root: Some(crate::painless_params::clean_path(root)),
            });
            return patterns;
        }
    }

    // Pattern: Windows argument splitting, the Go implementation the sysmon,
    // powershell and m365_defender pipelines all carry. Ahead of everything
    // its 100 lines could otherwise trigger -- the named CommandLine matcher
    // included.
    if normalised.contains("commandLineToArgv(")
        && normalised.contains("readNextArg")
        && let Some(parsed) = crate::painless_windows::ArgvScript::parse(normalised)
    {
        patterns.push(KnownPattern::SplitCommandLine(parsed));
        return patterns;
    }

    // Pattern: the basename of one or more path fields -- everything after
    // the last separator. Guarded by the parse rather than by the trigger,
    // so a script that only looks similar falls through.
    if normalised.contains("lastIndexOf(") && normalised.contains(".substring(") {
        if let Some(pattern) = parse_basename_cuts(normalised) {
            patterns.push(KnownPattern::Basename(Box::new(pattern)));
        }
        // Pattern: the same cut, but written straight onto a ctx path and
        // landing on a DIFFERENT one -- `file.name` to `file.extension` -- or
        // written inline and landing back on the field it read.
        if let Some((source, target, separator)) =
            parse_suffix_after_separator(normalised).or_else(|| parse_inline_suffix_cut(normalised))
        {
            patterns.push(KnownPattern::SuffixAfterSeparator {
                source,
                target,
                separator,
                whole_when_absent: false,
            });
        } else if let Some((source, target, separator)) = parse_local_index_basename(normalised) {
            patterns.push(KnownPattern::SuffixAfterSeparator {
                source,
                target,
                separator,
                whole_when_absent: true,
            });
        }
    }

    // Pattern: sysmon's file split -- name and directory at the last
    // backslash, the extension off the whole path's last dot.
    if normalised.contains(".name = path.substring(idx+1)")
        && normalised.contains(".directory = path.substring(0, idx)")
        && let Some(source) = crate::painless_windows::file_info_source(normalised)
    {
        patterns.push(KnownPattern::FileInfo(source));
        return patterns;
    }

    // Pattern: sysmon's hash-map lowercasing, empty and all-zero hashes
    // dropped and `related` replaced with the hash list.
    if normalised.contains("hashIsEmpty(")
        && let Some(source) = crate::painless_windows::hash_lowercase_source(normalised)
    {
        patterns.push(KnownPattern::HashLowercase(source));
        return patterns;
    }

    // Pattern: the tail of one string field past another field's length,
    // optionally dropping one leading comma -- umbrella's identities dance.
    if normalised.contains(".substring(ctx.")
        && normalised.contains(".length())")
        && let Some(pattern) = parse_prefix_tail(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: a scalar field prepended to an array field into a target.
    if normalised.contains("new ArrayList()")
        && normalised.contains(".add(ctx.")
        && let Some(pattern) = parse_prepend_to_array(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: cloudtrail's resources -- ARN and accountId renamed per
    // element, then deduplicated by the arn_account_type composite.
    if normalised.contains("uniqueResources")
        && let Some(at) = normalised.find(" instanceof List")
        && let Some(source) = normalised[..at]
            .rfind("ctx.")
            .map(|s| &normalised[s + 4..at])
    {
        patterns.push(KnownPattern::ResourcesRenameDedup(
            crate::painless_params::clean_path(source),
        ));
        return patterns;
    }

    // Pattern: securityhub's single-resource entity extraction, and its
    // multi-resource sibling whose every write is an append. The data stream
    // is part of the path it reads, so take that from the binding rather than
    // naming one -- `securityhub_findings` and `..._full_posture` ship the
    // same pair of scripts over their own field.
    if normalised.contains("res.Details[res.Type]?.Name")
        && let Some(source) = ctx_path_bound_to(normalised, "resources")
    {
        if normalised.contains("resources.size() == 1") {
            patterns.push(KnownPattern::SecurityhubResource(source));
            return patterns;
        }
        if normalised.contains("ctx.resource.type.add(") {
            patterns.push(KnownPattern::SecurityhubResources(source));
            return patterns;
        }
    }

    // Pattern: m365's process and file fields off the alert evidence list,
    // and its identity sibling over the same list. The list is at
    // `json.evidence` on the alert stream and `json.alerts.evidence` on the
    // incident one, so the loop is what says which -- hard-coding the incident
    // spelling left every alert event without any of these fields.
    if normalised.contains("void maybeAddExecutable(")
        && let Some(path) = evidence_loop_path(normalised)
    {
        patterns.push(KnownPattern::M365ProcessEvidence(path));
        return patterns;
    }
    if normalised.contains("def processUserName = new HashSet()")
        && let Some(path) = evidence_loop_path(normalised)
    {
        patterns.push(KnownPattern::M365IdentityEvidence(path));
        return patterns;
    }

    // Pattern: route53's answers rebuilt into ECS, feeding related.* as they go.
    if normalised.contains("answer?.Rdata") && normalised.contains("new_answer") {
        patterns.push(KnownPattern::Route53Answers);
        return patterns;
    }

    // Pattern: the address a reverse-lookup question names, back out of its
    // `in-addr.arpa` / `ip6.arpa` labels and into `related.ip`.
    if normalised.contains(".in-addr.arpa") && normalised.contains(".ip6.arpa") {
        patterns.push(KnownPattern::ReverseLookupAddress);
        return patterns;
    }

    // Pattern: gcp's long-running operation, which opens and closes a session.
    if normalised.contains(".category.add('session')")
        && let Some(first) = ternary_default_path(normalised, "first")
        && let Some(last) = ternary_default_path(normalised, "last")
    {
        patterns.push(KnownPattern::LongOperationSession { first, last });
        return patterns;
    }

    // Pattern: powershell's script-block entropy and the spread around it.
    if normalised.contains("double surprisalVar")
        && let Some(source) = ctx_path_bound_to(normalised, "script")
    {
        patterns.push(KnownPattern::ScriptBlockEntropy(source));
        return patterns;
    }

    // Pattern: zscaler's pipe-delimited columns, split in place.
    if normalised.contains("void splitStr(Map m, String key)") {
        let fields = parse_split_on_pipe(normalised);
        if !fields.is_empty() {
            patterns.push(KnownPattern::SplitOnPipe(fields));
            return patterns;
        }
    }

    // Pattern: zscaler's parallel attachment columns zipped into one list.
    if normalised.contains("item.put('file', file)")
        && let Some(zip) = AttachmentZip::parse(normalised)
    {
        patterns.push(KnownPattern::ZipAttachments(Box::new(zip)));
        return patterns;
    }

    // Pattern: the highest score any of a field's values scores, each scored by
    // the substring it contains.
    if normalised.contains("if (cur > maxSev) maxSev = cur;")
        && let Some(pattern) = MaxByContains::parse(normalised)
    {
        patterns.push(KnownPattern::MaxByContains(Box::new(pattern)));
        return patterns;
    }

    // Pattern: the same zip with no wrapper, driven by one column's length and
    // skipping the vendor's placeholder names.
    if normalised.contains("out.add(item)")
        && let Some(zip) = ColumnZip::parse(normalised)
    {
        patterns.push(KnownPattern::ZipColumns(Box::new(zip)));
        return patterns;
    }

    // Pattern: m365's `isTruthy` helper, one target per vendor flag.
    if normalised.contains("def isTruthy(def val)") {
        let pairs = parse_truthy_assignments(normalised);
        if !pairs.is_empty() {
            patterns.push(KnownPattern::TruthyAssignments(pairs));
            return patterns;
        }
    }

    // Pattern: drop every ENTRY of a map holding one literal -- squid's `-`,
    // which it writes into a dozen grok captures instead of omitting them.
    // Ahead of the list version because the two share the `.removeIf(` opening
    // and only the subject tells them apart.
    if normalised.contains(".values().removeIf(")
        && !normalised.contains("instanceof Map")
        && let Some(pattern) = parse_remove_map_value(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: drop one literal out of a list -- m365's file.path, whose
    // append leaves a bare separator when neither half of its template is
    // there.
    if normalised.contains(".removeIf(")
        && !normalised.contains("instanceof Map")
        && let Some(pattern) = parse_remove_list_value(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: the same drop, spelled as a loop that removes by index while
    // walking forward instead of `.removeIf(`. coredns's own `int i=0` literal
    // rules out sysmon's V4MAPPED script, which opens its otherwise similar
    // loop with `def i = 0`.
    if normalised.contains("for (int i=0; i<ctx.")
        && normalised.contains(".remove(i)")
        && let Some(pattern) = parse_indexed_list_removal(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: gcp audit's related.entity, whose `isKubernetes` gate decides
    // which three of its sources are suppressed.
    if normalised.contains("boolean isKubernetes") && normalised.contains("ctx.related.entity") {
        patterns.push(KnownPattern::GcpRelatedEntity);
        return patterns;
    }

    // Pattern: mimecast's related.* collection -- display names and email
    // addresses off named paths, split at the `@`, sorted.
    if normalised.contains("splitmail(")
        && normalised.contains("related.hosts")
        && let Some(pattern) = parse_mail_related(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: inspector's twin of the above -- the same one-or-many split,
    // over its own lower-cased member names.
    if normalised.contains("ctx.aws.inspector.resources") && normalised.contains("ctx.resource.id")
    {
        patterns.push(KnownPattern::InspectorResources {
            multi: normalised.contains("ctx.resource.id.add("),
        });
        return patterns;
    }

    // Pattern: checkpoint's dropped-packet tuples into structured maps.
    if normalised.contains("packets_dropped") && normalised.contains(".splitOnToken('>')") {
        patterns.push(KnownPattern::CheckpointPackets);
        return patterns;
    }

    // Pattern: every member of a list trimmed where it sits -- cloudfront's
    // split x-forwarded-for, whose next processor greps each member anchored.
    if normalised.contains(".trim();")
        && normalised.contains("[i] =")
        && let Some(pattern) = parse_trim_list(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: a constant appended when a member of a list carries a prefix --
    // cloudfront's `localhost:8080`, which its grok cannot read as an address.
    if normalised.contains(".startsWith(")
        && normalised.contains(".add(")
        && normalised.contains("ctx[")
        && let Some(pattern) = parse_starts_with_append(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: elb's `tlsv12` -- the protocol and version out of one token.
    if normalised.contains("ctx.tls.version_protocol")
        && normalised.contains(".splitOnToken(\"v\")")
        && let Some(pattern) = parse_tls_version_split(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: sequential loop-over-category ladders assigning a LIST
    // literal -- defender's event.type.
    if normalised.contains(" in ctx.event.category)")
        && normalised.contains("break;")
        && let Some(pattern) = parse_category_type_ladder(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: whitespace stripped from every key of one map -- powershell's
    // spaced event_data names.
    if normalised.contains(".matcher(entry.getKey()")
        && normalised.contains("replaceAll(\"\")")
        && let Some(at) = normalised.find(".entrySet()")
        && let Some(start) = normalised[..at].rfind("ctx.")
    {
        patterns.push(KnownPattern::KeysStripWhitespace(
            crate::painless_params::clean_path(&normalised[start + 4..at]),
        ));
        return patterns;
    }

    // Pattern: one map rebuilt key by key through a helper the script defines
    // itself -- o365's CSV headings, qualys's spaced names, lambda's REPORT
    // metrics. Ahead of the snake-case copy below, which reads only the camel
    // break of the same helper and would drop the spaces, slashes, parentheses
    // and byte-order mark that sit beside it.
    if normalised.contains("] = item.getValue()")
        && normalised.contains(".entrySet()")
        && let Some(pattern) = parse_rewrite_keys(normalised)
    {
        patterns.push(KnownPattern::RewriteKeys(Box::new(pattern)));
        return patterns;
    }

    // Pattern: one map copied to another path with its keys snake_cased by a
    // helper the script defines itself -- lambda's REPORT metrics. The helper's
    // name is not fixed (`underscore` here), so the replacement it performs is
    // what identifies it.
    if normalised.contains("([a-z])([A-Z]+)")
        && normalised.contains(".getKey()")
        && let Some(pattern) = parse_snake_key_map_copy(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: strip a surrounding `<...>` pair from named fields and each
    // member of a list -- proofpoint's mail addresses.
    if normalised.contains(".startsWith(\"<\")")
        && normalised.contains(".endsWith(\">\")")
        && let Some(pattern) = parse_strip_angle_pairs(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: a list of {name, value} pairs folded into a map that REPLACES
    // the target -- proofpoint's audit labels.
    if normalised.contains(", new HashMap())")
        && normalised.contains("for (")
        && let Some(pattern) = parse_name_value_fold(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: the text before the last CASE-INSENSITIVE marker. The
    // `toLowerCase()` is what separates this from panw's url and file scripts,
    // which cut on a plain `lastIndexOf` and are several statements long.
    if normalised.contains(".toLowerCase().lastIndexOf(")
        && normalised.contains(".substring(0, ")
        && let Some(pattern) = parse_substring_before_last(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: every key of one map capitalised, ahead of PascalCase renames.
    if normalised.contains(".substring(0, 1).toUpperCase()")
        && normalised.contains(".entrySet()")
        && let Some(pattern) = parse_pascal_keys(normalised)
    {
        patterns.push(KnownPattern::PascalKeys(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a numeric field's bits decoded into a list of names.
    if normalised.contains("parseUnsignedInt(")
        && normalised.contains("& 0x")
        && let Some(pattern) = parse_bit_flag_names(normalised)
    {
        patterns.push(KnownPattern::BitFlagNames(Box::new(pattern)));
        return patterns;
    }

    // Pattern: the same bit decode, spelled as a fixed name array indexed off
    // the loop counter instead of one literal `.add()` per mask. netflow's
    // `new String[]{` opening does not appear in any other vendored pipeline.
    if normalised.contains("new String[]{")
        && let Some(pattern) = parse_bit_name_array(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: cloudtrail's ConsoleLogin extras.
    if normalised.contains("aed_map") && normalised.contains("'ConsoleLogin'") {
        patterns.push(KnownPattern::ConsoleLoginEventData);
        return patterns;
    }

    // Pattern: the same fold written as an indexed loop -- aws/waf's request
    // headers and the headers it inserts.
    if normalised.contains("= new HashMap()")
        && normalised.contains("] = ctx.")
        && let Some(pattern) = parse_indexed_name_value_fold(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: an outcome read off the tag whose name carries the action's
    // prefix -- proofpoint's audit outcome.
    if normalised.contains("+ '.'")
        && normalised.contains(".startsWith(action)")
        && let Some(pattern) = parse_outcome_from_tags(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: "Rename Common Auth Fields" -- process, source and client
    // fields out of event_data with the script's own conversions.
    if normalised.contains("WorkstationName")
        && normalised.contains("ClientAddress")
        && let Some(codes) = crate::painless_windows::event_code_list(normalised)
    {
        patterns.push(KnownPattern::RenameCommonAuth(codes));
        return patterns;
    }

    // Pattern: "Copy MemberName to User and User to Group" -- the split DN
    // to user.target, the Target fields to group.*. Ahead of its cousins,
    // whose triggers its text also spells.
    if normalised.contains("MemberNameParts")
        && let Some(codes) = crate::painless_windows::event_code_list(normalised)
    {
        patterns.push(KnownPattern::CopyMemberName(codes));
        return patterns;
    }

    // Pattern: "Copy Target User to Computer Object".
    if normalised.contains("computerObject")
        && let Some(codes) = crate::painless_windows::event_code_list(normalised)
    {
        patterns.push(KnownPattern::CopyComputerObject(codes));
        return patterns;
    }

    // Pattern: "Copy Target User to Target" and its Effective twin -- the
    // base and the SID field are the script's own.
    if normalised.contains("def userId = ctx.")
        && normalised.contains("ctx.user.put(")
        && let Some(codes) = crate::painless_windows::event_code_list(normalised)
        && let Some(base) = crate::painless_windows::copy_base(normalised)
        && let Some(sid_field) = crate::painless_windows::copy_sid_field(normalised)
    {
        patterns.push(KnownPattern::CopyUserToBase {
            codes,
            base,
            sid_field,
        });
        return patterns;
    }

    // Pattern: event 5136's `ObjectDN`, whose CN carries RFC 4514 escapes.
    if normalised.contains("ObjectDN")
        && normalised.contains("StringBuilder cn")
        && normalised.contains("objectClass")
    {
        patterns.push(KnownPattern::ObjectDn);
        return patterns;
    }

    // Pattern: the file-share events' path block. Ahead of the basename
    // matcher, whose `lastIndexOf`/`substring` pair this script also spells.
    if normalised.contains("RelativeTargetName")
        && normalised.contains("ShareLocalPath")
        && let Some(codes) = crate::painless_windows::contained_code_list(normalised)
    {
        patterns.push(KnownPattern::ShareFilePath(codes));
        return patterns;
    }

    // Pattern: event 4688's process block. Ahead of the append matcher, which
    // its closing `related.user.add` triggers -- that claimed the script and
    // left the whole process block unwritten.
    if normalised.contains("NewProcessId")
        && normalised.contains("ParentProcessName")
        && let Some(codes) = crate::painless_windows::event_code_list(normalised)
    {
        patterns.push(KnownPattern::ProcessCreated(codes));
        return patterns;
    }

    // Pattern: the security pipeline's "Copy Target User" -- SID, username
    // and domain to `user.*` or `user.target.*`, gated on the script's own
    // event-code list. Ahead of the email-split matcher, whose trigger its
    // `splitOnToken("@")` also spells.
    if normalised.contains("TargetUserSid")
        && normalised.contains("TargetDomainName")
        && normalised.contains("user.target")
        && let Some(codes) = crate::painless_windows::event_code_list(normalised)
    {
        patterns.push(KnownPattern::CopyTargetUser(codes));
        return patterns;
    }

    // Pattern: its sibling "Copy Subject User from Event Data", which
    // OVERWRITES `user.*`. The user_data variant is a different script and
    // is excluded by name.
    if normalised.contains("SubjectUserSid")
        && normalised.contains("SubjectDomainName")
        && normalised.contains("event_data")
        && !normalised.contains("user_data")
        && let Some(codes) = crate::painless_windows::event_code_list(normalised)
    {
        patterns.push(KnownPattern::CopySubjectUser(codes));
        return patterns;
    }

    // Pattern: zscaler's splitStr batch -- named map members split on `|` in
    // place. Ahead of the append-each matcher, whose `.add(` and
    // `instanceof Map` triggers the helper also spells.
    if normalised.contains("void splitStr(")
        && let Some(fields) = parse_split_pipe_fields(normalised)
    {
        patterns.push(KnownPattern::SplitPipeFields(fields));
        return patterns;
    }

    // Pattern: one field split on a token into a list, optionally parsed to
    // integers -- endpoint_dlp's dictionary counts. Same trigger overlap as
    // above.
    if normalised.contains(".splitOnToken(")
        && normalised.contains("new ArrayList()")
        && let Some(pattern) = parse_split_token_field(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: the same split written as a stream, dropping EVERY empty piece
    // and deleting the field when none survive -- ti_anomali's comma-fenced
    // `,10015,`, which the loop spelling above cannot express.
    if normalised.contains(".splitOnToken(")
        && normalised.contains("Collectors.toList()")
        && let Some(pattern) = parse_stream_split_filter(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: `ctx.<t> = ctx.<s>.decodeBase64();` -- zscaler web's URL and
    // referer.
    if normalised.contains(".decodeBase64()")
        && let Some(pattern) = parse_decode_base64(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: a split's token COUNT stored on the event -- vpcflow's format
    // dispatch, where every dissect gates on it.
    if normalised.contains(".splitOnToken(")
        && normalised.contains(").length")
        && let Some(pattern) = parse_token_count(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: one value wrapped in a one-element list -- mimecast's
    // attachments promotion, amazon_security_lake's singular `resource` moved
    // onto `resources`, and kolide's osquery_status address. No loop, or it is
    // the prepend pattern below.
    let wraps_a_new_list = (normalised.contains("= [];") || normalised.contains("new ArrayList()"))
        && normalised.contains(".add(ctx.");
    if (wraps_a_new_list || normalised.contains("= [ctx.") || normalised.contains("= [ ctx."))
        && !normalised.contains("for (")
        && let Some(pattern) = parse_wrap_value_in_list(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: the LAST element of an array assigned to a field --
    // mimecast's attachment extension off the split path.
    if normalised.contains(".length-1]")
        && let Some(pattern) = parse_last_element(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: classify each member of a list by string tests on the member.
    if normalised.contains("addNestedValue(") && normalised.contains("instanceof List") {
        patterns.push(KnownPattern::ClassifyMembers);
        return patterns;
    }

    // Pattern: keep a rendered copy of a nested object beside the object.
    if normalised.contains("keep_flattened_duplicates") {
        patterns.push(KnownPattern::FlattenedDuplicates);
        return patterns;
    }

    // Pattern: collect every non-empty value the script names into one sorted,
    // unique list.
    if normalised.contains("void addValue(") && normalised.contains("new TreeSet(") {
        patterns.push(KnownPattern::CollectEntities);
        return patterns;
    }

    // Pattern: DNS RData as tab-separated columns, one answer per line.
    if normalised.contains("answer_parts[") && normalised.contains("dns_answers.add(") {
        patterns.push(KnownPattern::DnsRdataAnswers);
        return patterns;
    }

    // Pattern: Google Public DNS's structured RData into `dns.answers`.
    if normalised.contains("structuredRdata") {
        patterns.push(KnownPattern::StructuredRdataAnswers);
        return patterns;
    }

    // Pattern: the ECS lists an answer set feeds, keyed on the record type.
    if normalised.contains("for (answer in ctx.dns.answers)") {
        patterns.push(KnownPattern::RelatedFromDnsAnswers);
        return patterns;
    }

    // Pattern: one synthesised DNS answer per resolved address, typed by
    // whether the address holds a colon.
    if normalised.contains("ctx.dns.answers.add(") && normalised.contains("ip.indexOf(\":\")") {
        patterns.push(KnownPattern::AnswersFromResolvedIp);
        return patterns;
    }

    // Pattern: the integrations' own recursive camelCase-to-snake_case pair,
    // applied to one object. Checked EARLY: the recursive arm spells `.add(`
    // and `instanceof Map`, which the append-each matcher below claims and
    // then does nothing with.
    if normalised.contains("Character.isUpperCase(")
        && normalised.contains("instanceof Map")
        && let Some(pattern) = snake_case_apply(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: split, trim and collect several optional fields into one list.
    // Checked early: the script also spells `.add(` and `.splitOnToken(`, which
    // a later matcher reads as a different pattern entirely.
    if normalised.contains("new HashSet(") && normalised.contains(".asList()") {
        patterns.push(KnownPattern::SplitTrimCollect);
        return patterns;
    }

    // Pattern: network.bytes / network.packets as the sum of both directions.
    let totals = sum_of_directions(normalised);
    if !totals.is_empty() {
        patterns.push(KnownPattern::SumDirections(totals));
        return patterns;
    }

    // Pattern: one ctx field combined from two others. Gated on the parse, so a
    // script that merely adds two fields somewhere no longer claims the pattern.
    if (normalised.contains(" + ctx.") || normalised.contains(" - ctx."))
        && let Some(pattern) = parse_combine_fields(normalised)
    {
        patterns.push(KnownPattern::CombineFields(Box::new(pattern)));
    }

    // Pattern: seconds to nanoseconds for event.duration.
    if normalised.contains("ctx.event.duration")
        && normalised.contains("Long.parseLong")
        && normalised.contains("1000000000")
    {
        patterns.push(KnownPattern::DurationToNanos);
        return patterns;
    }

    // Pattern: a map's values collected into a deduplicated list. Ahead of the
    // generic map walkers, which read the `entrySet()` loop as a fan-out.
    if normalised.contains("new HashSet(")
        && let Some(pattern) = parse_dedupe_map_values(normalised)
    {
        patterns.push(KnownPattern::DedupeMapValues(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a list's objects snake-cased in place by the vendor's own
    // regex rule. Ahead of the sum below, which also walks `ctx.<list>`.
    if normalised.contains("$1_$2")
        && let Some(pattern) = parse_snake_case_list_elements(normalised)
    {
        patterns.push(KnownPattern::SnakeCaseListElements(Box::new(pattern)));
        return patterns;
    }

    // Pattern: one member totalled across a list of objects.
    if normalised.contains(" in ctx.")
        && normalised.contains("+=")
        && let Some(pattern) = parse_sum_member_over_list(normalised)
    {
        patterns.push(KnownPattern::SumMemberOverList(Box::new(pattern)));
        return patterns;
    }

    // Pattern: the NAME of the first key carrying a value. Ahead of the map
    // walkers, which read the `keySet()` loop as a fan-out and would claim it
    // without ever writing the key name.
    if normalised.contains(".keySet()")
        && normalised.contains("break;")
        && let Some(pattern) = parse_first_present_key_name(normalised)
    {
        patterns.push(KnownPattern::FirstPresentKeyName(Box::new(pattern)));
        return patterns;
    }

    // Pattern: the same scaling from a FRACTIONAL count of seconds, narrowed
    // by the vendor's own `(int)` cast rather than saturating at i64.
    if normalised.contains("Float.parseFloat")
        && normalised.contains("1000000000")
        && normalised.contains("= (int)")
        && let Some(pattern) = parse_float_seconds_to_nanos(normalised)
    {
        patterns.push(KnownPattern::FloatSecondsToNanos(Box::new(pattern)));
        return patterns;
    }

    // Pattern: an `hh:mm:ss` duration scaled to nanoseconds, plus the span
    // around @timestamp where the script counts one end back from the other.
    //
    // The HELPER'S NAME is the trigger. Keying on the loop missed cyberarkpas,
    // which indexes with `charAt(i)` where cisco walks `.toCharArray()`, and
    // keying on `minusNanos(` missed it twice over -- it writes the duration
    // and no span at all, so that call was never going to be there.
    if normalised.contains("parse_hms(") && normalised.contains("1000000000") {
        patterns.push(KnownPattern::FlowDuration);
        return patterns;
    }

    // Pattern: two parallel arrays, one naming what the other holds.
    if normalised.contains("(ctx, ctx.") && normalised.contains("[i])") {
        patterns.push(KnownPattern::ParallelDispatch);
        return patterns;
    }

    // Pattern: a string built up piece by piece under per-field guards --
    // cloudfront's `url.full` out of the protocol, domain, path and query.
    if normalised.contains("def ")
        && normalised.contains(" += ")
        && normalised.contains("!= \"\"")
        && let Some(pattern) = parse_concat_parts(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: build a string out of ctx fields and literals.
    if normalised.contains("?: ''")
        && normalised.contains(".isEmpty()")
        && normalised.contains("\" + ")
    {
        patterns.push(KnownPattern::ConcatMessage);
    }

    // Pattern: swap two ctx subtrees, keeping named keys on one side.
    if normalised.contains("def tmp = ctx.") {
        patterns.push(KnownPattern::SwapSubtrees);
    }

    // Pattern: a ladder collecting into a list, written as scalar or array.
    if normalised.contains(".add(")
        && normalised.contains(".size()")
        && normalised.contains("else if (")
    {
        patterns.push(KnownPattern::CollectingLadder);
    }

    // Pattern: a case-insensitive ladder mapping one field onto a literal.
    // Tried before the `==` ladder, which cannot read either the multi-literal
    // arms or the numeric right-hand sides.
    if normalised.contains(".equalsIgnoreCase(") && normalised.contains("else if (") {
        patterns.push(KnownPattern::CaseInsensitiveLadder);
        return patterns;
    }

    // Pattern: an equality ladder mapping one field onto string literals.
    if normalised.contains("else if (")
        && let Some(ladder) = parse_ladder(normalised)
    {
        patterns.push(KnownPattern::EqualityLadder(ladder));
        return patterns;
    }

    // Pattern: strip sentinel values and junk keys out of a parsed map.
    if normalised.contains(".entrySet().removeIf(") && normalised.contains("entry.getValue()") {
        patterns.push(KnownPattern::SentinelRemovalLiteral);
        return patterns;
    }

    // Checked ahead of `RowLookupWithFallback`, whose `" : ctx."` trigger needs
    // a space before the colon this pattern's loop variable never has. Gated on
    // the parse alone, because each half already enforces its own write: the
    // flag needs the `= true;`/`= false;` pair and the copy needs a body that is
    // one guard around one statement.
    if normalised.contains("for (def ")
        && let Some(pattern) = parse_list_member_select(normalised)
    {
        patterns.push(KnownPattern::ListMemberSelect(Box::new(pattern)));
        return patterns;
    }

    // Pattern: look a value up in a ctx-held table of rows, else fall back.
    // Gated on the parse: on the two `contains` alone it claimed 47 call sites
    // whose loop and match condition it could not read, and applied to none of
    // them. No `return` here, so declining changes nothing at run time.
    if normalised.contains("for (def ")
        && normalised.contains(" : ctx.")
        && let Some(pattern) = parse_row_lookup(normalised)
    {
        patterns.push(KnownPattern::RowLookupWithFallback(Box::new(pattern)));
    }

    // Pattern: split a schemeless URL into its ECS components.
    if normalised.contains("domainPort") && normalised.contains("url.original") {
        patterns.push(KnownPattern::SchemelessUrl);
        return patterns;
    }

    // Pattern: split a version string at its first digit.
    if normalised.contains("matcher.start()") {
        patterns.push(KnownPattern::VersionSplit);
    }

    // Pattern: decompose a syslog PRI into ECS facility and severity.
    if normalised.contains("log.syslog") && normalised.contains("priority") {
        patterns.push(KnownPattern::SyslogPriority(parse_syslog_priority(
            normalised,
        )));
        return patterns;
    }

    // Pattern: append one array into another, skipping duplicates.
    if let Some((from, into)) = append_unique_fields(normalised) {
        patterns.push(KnownPattern::AppendUnique { from, into });
        return patterns;
    }

    // Pattern: quote-aware KV split of a whole vendor payload.
    if normalised.contains("splitUnquoted(")
        && let Some(pattern) = parse_split_unquoted_kv(normalised)
    {
        patterns.push(KnownPattern::SplitUnquotedKv(Box::new(pattern)));
        return patterns;
    }

    // Pattern: re-key an array of maps into an object indexed by position.
    if normalised.contains("new HashMap()") && normalised.contains("String.valueOf(") {
        patterns.push(KnownPattern::ArrayToIndexedObject);
        return patterns;
    }

    // Pattern: collapse an array of `{key, value}` maps into one object.
    if normalised.contains("[item.key] = item.value") {
        patterns.push(KnownPattern::KeyValuePairs);
        return patterns;
    }

    // Pattern: a base timestamp plus a duration whose LAST CHARACTER is the
    // unit. Ahead of the two matchers whose triggers this script's text
    // satisfies by coincidence: `JoinOptional` below counts two `String `
    // declarations and an `else if`, which the `ti_abusech` interval streams
    // spell exactly; `GuardedCopy` further down claims anything with
    // `!= null` and then walks THIS partially -- deciding the unit ladder on a
    // local it cannot resolve, taking the else arm, and writing the vendor's
    // "invalid duration" message onto every event while writing no expiry.
    // Pattern: ti_misp's spelling of the same window -- an epoch base, the
    // LATER of two, and a flag saying it has already passed. Ahead of
    // `IocExpiry`, which declines it (its bases are locals, not ctx paths) and
    // so let `GuardedCopy` further down claim it and write the vendor's
    // "invalid duration" message onto 15 events while writing no expiry.
    if normalised.contains("plusDays(")
        && normalised.contains("plusHours(")
        && normalised.contains("plusMinutes(")
        && normalised.contains("Instant.ofEpochMilli(")
        && normalised.contains(".isBefore(")
        && let Some(pattern) = parse_decay_window(normalised)
    {
        patterns.push(KnownPattern::DecayWindow(Box::new(pattern)));
        return patterns;
    }

    if normalised.contains("plusDays(")
        && normalised.contains("plusHours(")
        && normalised.contains("plusMinutes(")
        && let Some(pattern) = parse_ioc_expiry(normalised)
    {
        patterns.push(KnownPattern::IocExpiry(Box::new(pattern)));
        return patterns;
    }

    // Pattern: drop a field's last character, which is what strips the
    // trailing comma off mysql_enterprise's audit lines before the JSON parse.
    if normalised.contains(".substring(0, ctx.")
        && normalised.contains(".length() - 1)")
        && let Some(pattern) = parse_drop_last_char(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: ti_misp's tag list, scrubbed into names AND filtered into a
    // marking map. Ahead of whichever matcher was claiming it for the first
    // write alone, which left the marking missing on every tagged event.
    if normalised.contains("Collectors.toList()")
        && normalised.contains(".toUpperCase())")
        && let Some(pattern) = parse_tags_and_marking(normalised)
    {
        patterns.push(KnownPattern::TagsAndMarking(Box::new(pattern)));
        return patterns;
    }

    // Pattern: the numbered CSV column map collapsed into an ordered list.
    // Ahead of the colon-pair reader below, which is the NEXT script in the
    // same chain and shares its `_csv_array` anchor.
    if normalised.contains("new TreeMap()")
        && normalised.contains("columnArray.add(")
        && let Some(pattern) = parse_csv_map_to_array(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: `Key: value` columns into a map, their keys joined into the
    // fingerprint that identifies the log layout.
    if normalised.contains("String.join(\"|\", fingerprint)")
        && normalised.contains("m.group(1).toLowerCase()")
        && let Some(pattern) = parse_csv_colon_pairs(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: a list deduplicated and then unwrapped when one member is
    // left, which is how suricata collapses destination.domain.
    if normalised.contains(".stream().distinct().collect(Collectors.toList())")
        && normalised.contains(".length == 1")
        && let Some(pattern) = parse_dedupe_unwrap(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: the fortiproxy variant of the same loop. FIRST, because it
    // spells `inQuote` and `kvSplit` too and the plain reader would claim it
    // and then drop its N/A and non-word-key rules.
    if normalised.contains("wordPattern")
        && normalised.contains("kvSplit")
        && let Some(pattern) = parse_kv_into_fields(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: a hand-written quote-aware KV split into one namespace, which
    // is the whole of stormshield's parse.
    if normalised.contains("inQuote")
        && normalised.contains("kvSplit")
        && let Some(pattern) = parse_kv_into_namespace(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: a lookup whose table is on the DOCUMENT rather than in params,
    // which is why no params matcher can claim it.
    if normalised.contains("instanceof Map &&")
        && normalised.contains(".containsKey(")
        && let Some(pattern) = parse_ctx_table_lookup(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: awsfirehose naming the AWS log type its record carries. Its own
    // classifier, so the trigger is its own literals.
    if normalised.contains("aws-waf-logs-") && normalised.contains("tokens_result") {
        patterns.push(KnownPattern::FirehoseDataset);
        return patterns;
    }

    // Pattern: the sorted key names of the first nested map holding an inner
    // map, which awsfirehose fingerprints to key a document by its metrics.
    if normalised.contains("metricNames")
        && normalised.contains("Collections.sort(")
        && let Some(pattern) = parse_nested_key_names(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: the same expiry with no unit ladder at all -- one parse and one
    // literal number of days. ti_eset's apt stream dates its indicators a year
    // out from `@timestamp`, and `IocExpiry` above declines it for want of the
    // `plusHours` / `plusMinutes` arms it reads the unit from.
    if normalised.contains("ZonedDateTime.parse(")
        && normalised.contains(".plusDays(")
        && !normalised.contains("plusHours(")
        && let Some(pattern) = parse_date_plus_days(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: join two optional fields, each alone if the other is absent.
    if normalised.matches("String ").count() == 2 && normalised.contains("} else if (") {
        patterns.push(KnownPattern::JoinOptional);
        return patterns;
    }

    // Pattern: a list of maps rebuilt under an explicit rename table. Ahead
    // of the append-each matcher below, whose `.add(` and `instanceof Map`
    // triggers the closing `out.add(m)` and `!(f instanceof Map)` also spell.
    if normalised.contains(".containsKey('")
        && normalised.contains(".remove('")
        && let Some(pattern) = parse_list_rename_table(normalised)
    {
        patterns.push(KnownPattern::ListRenameTable(Box::new(pattern)));
        return patterns;
    }

    // The two below sit AHEAD of `AppendEach`, whose arm returns whether or not
    // its own parse succeeded -- a deliberate stop, so every split-then-append
    // script ends there. Both of these build a LOCAL list and store it, where
    // `AppendEach` appends to a ctx path, so the two never read the same script.

    // Pattern: a delimited string cut twice, into a list of positional records.
    // Ahead of the prepend below, whose `.splitOnToken(` and `.add(` triggers
    // this also spells.
    if normalised.contains("[:]")
        && normalised.contains(".splitOnToken(")
        && let Some(pattern) = parse_split_into_records(normalised)
    {
        patterns.push(KnownPattern::SplitIntoRecords(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a list led by one field, then the parts of a second field's
    // split. `PrependToArray` reads the same idea written `.add(ctx.<path>)`
    // over a list that is already one; this is the one built from a cut.
    if normalised.contains(".splitOnToken(")
        && normalised.contains(".add(")
        && let Some(pattern) = parse_prepend_split(normalised)
    {
        patterns.push(KnownPattern::PrependSplit(Box::new(pattern)));
        return patterns;
    }

    // Pattern: flatten a field into an array, either by splitting a delimited
    // string or by joining each map's two keys. The source has to come BEFORE
    // the append -- you split, THEN add -- or the pair is two unrelated
    // statements and this claims a script it cannot run: windows' connection
    // events append an address and separately split an executable, and reading
    // them as one pair put the path segments into `related.ip`.
    if let Some(add_at) = normalised.find(".add(")
        && [".splitOnToken(", "instanceof Map"]
            .iter()
            .any(|marker| normalised.find(marker).is_some_and(|at| at < add_at))
    {
        if let Some(pattern) = parse_append_each(normalised) {
            patterns.push(KnownPattern::AppendEach(Box::new(pattern)));
        }
        return patterns;
    }

    // Pattern: every value of a map gathered into one deduped list, a value
    // that is itself a list flattened into it.
    if normalised.contains(".values()")
        && normalised.contains("instanceof List")
        && normalised.contains(".add(")
        && let Some(pattern) = parse_flatten_map_into(normalised)
    {
        patterns.push(KnownPattern::FlattenMapInto(Box::new(pattern)));
        return patterns;
    }

    // Pattern: whole numbers rendered as strings in place. Ahead of the
    // ladders below, whose `instanceof String` guard and `!= null` triggers
    // this also spells.
    if normalised.contains("Long.toString(")
        && let Some(fields) = parse_stringify_longs(normalised)
    {
        patterns.push(KnownPattern::StringifyLongs(fields));
        return patterns;
    }

    // Pattern: the same if/else-if chain over one field, but every arm COPIES
    // a different source into one target rather than writing a literal. Ahead
    // of `LiteralValueMap` below, whose arms must write a literal, so it
    // declines this and leaves it to `GuardedCopy` -- which takes the FIRST
    // arm's source whatever the label says.
    if normalised.contains(" else if (")
        && let Some(pattern) = parse_copy_by_label(normalised)
    {
        patterns.push(KnownPattern::CopyByLabel(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a value map written out as an if/else-if chain over one field.
    // Now actually guarded by the parse, which the comment here used to claim
    // while the code matched on `contains` alone and took 91 sites it never
    // applied. No `return`, so declining changes nothing at run time.
    if normalised.contains("else if (")
        && let Some(pattern) = parse_literal_value_map(normalised)
    {
        patterns.push(KnownPattern::LiteralValueMap(Box::new(pattern)));
    }

    // Pattern: a field replaced by whether it equals a literal. Late, because
    // the parse is a two-statement script and every richer pattern above spells
    // a comparison somewhere too.
    if normalised.contains(" == '")
        && let Some(pattern) = parse_equals_literal_flag(normalised)
    {
        patterns.push(KnownPattern::EqualsLiteralFlag(Box::new(pattern)));
        return patterns;
    }

    // Pattern: keys_to_snake_case. The helper is COPIED between packages and
    // the copies disagree on an acronym, so the rule comes off the script:
    // a run counter with a `setCharAt` fix-up breaks before the run's last
    // character, where the plain copies break before every uppercase.
    if normalised.contains("keys_to_snake_case") || normalised.contains("keysToSnakeCase") {
        let rule = if normalised.contains("setCharAt(") {
            SnakeRule::AcronymRun
        } else {
            SnakeRule::BeforeEveryUpper
        };
        patterns.push(KnownPattern::KeysToSnakeCase(
            snake_case_target(normalised).or_else(|| extract_target_field(normalised)),
            rule,
        ));
        return patterns;
    }

    // From here down: the matchers keyed on a vendor's FIELD NAMES rather than
    // on a Painless construct, plus the two catch-alls.

    // Pattern: CommandLine → process fields
    if normalised.contains("CommandLine") && normalised.contains("process") {
        patterns.push(KnownPattern::CommandLine {
            parent: normalised.contains("ParentCommandLine"),
        });
        return patterns;
    }

    // Pattern: ProcessStartTime epoch → @timestamp or process.start
    if normalised.contains("ProcessStartTime") || normalised.contains("processStartTime") {
        patterns.push(KnownPattern::ProcessStartTime);
        return patterns;
    }

    // Pattern: an address split on `@`, each half written where the script
    // says. Used by okta, o365, azure and many others.
    if normalised.contains("splitOnToken")
        && normalised.contains('@')
        && let Some(pattern) = parse_email_split(normalised)
    {
        patterns.push(KnownPattern::EmailSplit(Box::new(pattern)));
        return patterns;
    }

    // Pattern: okta risk_behaviors extraction from flattened.behaviors
    // Extracts keys with value "POSITIVE" into an array
    if normalised.contains("POSITIVE") && normalised.contains("risk_behaviors") {
        patterns.push(KnownPattern::RiskBehaviors);
        return patterns;
    }

    // Pattern: Azure category → event type/category mapping via params lookup
    if normalised.contains("activitylogs")
        && normalised.contains("category")
        && normalised.contains("params.get")
    {
        patterns.push(KnownPattern::AzureCategoryEventType);
        return patterns;
    }

    // Pattern: Azure event_category assignment, in whichever module's subtree.
    if normalised.contains("event_category") && normalised.contains("eventCategory") {
        patterns.push(KnownPattern::AzureEventCategory);
        return patterns;
    }

    // Pattern: replace dots in map keys (Azure identity claims)
    // Matches: ctx.temp_claims[key.replace('.', '_')] = ...
    if normalised.contains("replace('.'") && normalised.contains("keySet()") {
        patterns.push(KnownPattern::ReplaceDotsInKeys);
        return patterns;
    }

    // Pattern: okta.target array key renames + user/group extraction
    // Renames alternateId→alternate_id, displayName→display_name in each element,
    // filters detailEntry, extracts first user/usergroup targets
    if normalised.contains("alternateId")
        && normalised.contains("alternate_id")
        && normalised.contains("okta")
    {
        patterns.push(KnownPattern::OktaTargetRename);
        return patterns;
    }

    // Pattern: a map's keys selected by suffix, their values sorted onto one
    // target. Ahead of the collect-by-key pattern below, whose trigger this also
    // satisfies and whose parse reads neither the suffix nor the sort.
    if normalised.contains(".keySet()")
        && normalised.contains(".endsWith(")
        && let Some(pattern) = parse_keys_by_suffix(normalised)
    {
        patterns.push(KnownPattern::KeysBySuffix(Box::new(pattern)));
        return patterns;
    }

    // Pattern: collect one nested key out of every entry of a map.
    if normalised.contains(".keySet()") && normalised.contains(".add(") {
        patterns.push(KnownPattern::CollectMapValues);
        return patterns;
    }

    // Pattern: a field read into a local, run through a chain of string ops and
    // written back. Ahead of the single-replace pattern below, whose parse reads
    // only the first `.replace(` and would drop the rest of the chain.
    if let Some(pattern) = parse_string_ops(normalised) {
        patterns.push(KnownPattern::StringOps(Box::new(pattern)));
        return patterns;
    }

    // The same ops with no local: one chained expression. Beside its twin, and
    // after it, because the local spelling is the more constrained of the two.
    if let Some(pattern) = parse_chained_string_ops(normalised) {
        patterns.push(KnownPattern::StringOps(Box::new(pattern)));
        return patterns;
    }

    // Pattern: rewrite one substring of a field in place. The parse decides what
    // BINDS; the arm stops the ladder either way, because letting an unreadable
    // script fall through cost juniper_srx 845 fields to worse matches below.
    if normalised.contains(".replace(") {
        if let Some(pattern) = parse_guarded_replace(normalised) {
            patterns.push(KnownPattern::GuardedReplace(Box::new(pattern)));
        }
        return patterns;
    }

    // Pattern: a cast double rounded and scaled into a long. Ahead of the
    // scale-by-literal catch-all below, which reads the same `*` but parses
    // its factor as an integer and would decline on this pattern's `100.0`.
    if normalised.contains("Math.round(")
        && normalised.contains(".doubleValue()")
        && let Some(pattern) = parse_rounded_scale(normalised)
    {
        patterns.push(KnownPattern::RoundedScale(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a local map assembled from `ctx.` fields via `.put()`, then
    // assigned whole onto one target.
    if normalised.contains(" = new HashMap();")
        && normalised.matches(".put('").count() >= 2
        && let Some(pattern) = parse_fields_into_map(normalised)
    {
        patterns.push(KnownPattern::FieldsIntoMap(Box::new(pattern)));
        return patterns;
    }

    // crowdstrike timeline: keyed on field names, ahead of `GuardedCopy`
    // below, whose `!= null` catch-all would otherwise claim either script
    // and write nothing -- neither guard is on a `ctx.` path.
    if normalised.contains("ctx.crowdstrike.idp.timeline.entity")
        && normalised.contains("secondary_display_name")
    {
        patterns.push(KnownPattern::CrowdstrikeTimelineEntityIdentity);
        return patterns;
    }
    if normalised.contains("ctx.crowdstrike.idp.timeline.entity")
        && normalised.contains("sam_account_name")
    {
        patterns.push(KnownPattern::CrowdstrikeTimelineEntityAccounts);
        return patterns;
    }

    // Pattern: the first element of a list, bare or guarded on the target
    // being unset.
    if normalised.contains("[0];")
        && let Some(pattern) = parse_first_element(normalised)
    {
        patterns.push(KnownPattern::FirstElement(Box::new(pattern)));
        return patterns;
    }

    // Pattern: members copied off one indexed list element, each under its own
    // guard. Ahead of `GuardedCopy` below, whose broader `!= null` catch-all
    // cannot see that the guard is on `<local>.<member>`, not a `ctx.` path,
    // and so claims the text and writes nothing.
    if normalised.contains(" = ctx.")
        && normalised.contains("!= null")
        && !normalised.contains("for (")
        && let Some(pattern) = parse_indexed_field_copies(normalised)
    {
        patterns.push(KnownPattern::IndexedFieldCopies(Box::new(pattern)));
        return patterns;
    }

    // Pattern: named top-level keys moved under another object.
    if normalised.contains(" = ctx.remove(")
        && let Some(moves) = parse_move_keys(normalised)
    {
        patterns.push(KnownPattern::MoveKeys(moves));
        return patterns;
    }

    // Pattern: one string field classified by an else-if ladder whose answers
    // are held in locals. Ahead of `GuardedCopy` below, whose `!= null`
    // catch-all the trailing writes also spell and which then declines,
    // leaving the script claimed and unrun.
    if normalised.contains("else if (")
        && let Some(pattern) = parse_classify_ladder(normalised)
    {
        patterns.push(KnownPattern::ClassifyLadder(Box::new(pattern)));
        return patterns;
    }

    // Pattern: ECS network.direction from whether each end is in a private
    // range.
    if normalised.contains("isPrivateCIDR")
        && let Some(pattern) = parse_private_cidr_direction(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: a `GeoJSON` geometry rendered as WKT text, with the centroid and
    // the guarded copies the same script writes. AHEAD of the geo_point arm
    // below, which claims a script on its own `'lon':` trigger and answers true
    // for every event, so no arm after it can ever run -- gdacs writes all
    // three in ONE script, and this parse carries the other two.
    if normalised.contains("\"POLYGON \"")
        && let Some(pattern) = parse_wkt_geometry(normalised)
    {
        patterns.push(KnownPattern::WktGeometry(Box::new(pattern)));
        return patterns;
    }

    // Pattern: an ECS geo_point from a `GeoJSON` position. The runner answers
    // true whatever the event holds, so this arm claims everything it triggers
    // on; a script that writes more than the point belongs to a matcher ABOVE
    // it, never to a second matcher below.
    if normalised.contains("'lon':")
        && let Some(pattern) = parse_geo_point_from_coordinates(normalised)
    {
        patterns.push(KnownPattern::GeoPointFromCoordinates(pattern));
    }

    // Pattern: copies selected by a field matching a literal -- arista's
    // interface aliases, keyed on the interface id. Ahead of `GuardedCopy`,
    // which claims the script on its first `!= null`, applies ONE of the
    // copies, and never reads the branch that was supposed to select it.
    if normalised.contains("== \"")
        && normalised.contains("!= null")
        && let Some(branches) = parse_branch_copies(normalised)
    {
        patterns.push(KnownPattern::BranchCopies(branches));
        return patterns;
    }

    // Pattern: a field unwrapped from a surrounding pair of characters. Ahead
    // of `GuardedCopy`, which claims it on the `!= null` in its outer guard and
    // then copies the field onto itself unchanged.
    if normalised.contains(".startsWith(")
        && normalised.contains(".endsWith(")
        && normalised.contains(".substring(")
        && let Some(pattern) = parse_strip_surrounding_pair(normalised)
    {
        patterns.push(KnownPattern::StripSurroundingPair(Box::new(pattern)));
        return patterns;
    }

    // The two catch-alls below are patterns a longer script also CONTAINS, so
    // they run only after every structural matcher has declined.

    // Pattern: divide a number by a literal into a field, under a guard. The
    // trigger is a bare slash because cyberarkpas writes `cpu_usage/100.0`
    // with no spaces; the parse is what actually decides, and it declines on
    // a slash that is part of a path or a literal.
    if normalised.contains('/')
        && !normalised.contains("params")
        && let Some(pattern) = parse_guarded_divide(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: one field scaled by a literal into another.
    if normalised.contains(" * ")
        && !normalised.contains("params")
        && let Some(pattern) = parse_scale_field(normalised)
    {
        patterns.push(KnownPattern::ScaleField(Box::new(pattern)));
        return patterns;
    }

    // Pattern: copy one field to another when the source is set.
    //
    // Gated on the two reads the runner actually has: a tree holding at least
    // one write, or the single-copy fallback.
    //
    // On `!= null` alone it claimed 112 sites it could do nothing with.
    if normalised.contains("!= null") && !normalised.contains("for (") {
        let literals = Program::parse(normalised);
        if literals.can_write() || parse_single_copy(normalised).is_some() {
            patterns.push(KnownPattern::GuardedCopy(literals));
            return patterns;
        }
    }

    // Pattern: a string joined from named fields and literal separators in ONE
    // assignment -- fortimanager's date, time and offset, cloudfront's date and
    // time.
    //
    // BEFORE `PlainAssignments`, which shadows it: that walk's right-hand-side
    // grammar has no concatenation, so the assignment reads as an EMPTY block,
    // `is_whole()` still answers true, and the ladder returns on a matcher that
    // then writes nothing -- which is why both scripts read `ran 0` in the
    // runtime reach dump. Every narrower arm above keeps first refusal.
    if normalised.contains('+')
        && let Some(pattern) = parse_concat_assignment(normalised)
    {
        patterns.push(KnownPattern::ConcatAssignment(pattern));
        return patterns;
    }

    // Pattern: nothing BUT statements the walk can run -- allocations, copies,
    // literals, and branches on comparisons it can decide. Running the
    // readable statements of ANY script was tried as a catch-all and rejected,
    // because a partial read writes a value where the vendor's whole script
    // would have written a different one. This is that walk with the hole
    // closed: a script qualifies only when nothing in it is skipped, so what
    // runs is the whole of what the vendor wrote. carbon_black's netconn
    // direction is the pattern.
    let program = Program::parse(normalised);
    if program.is_whole() {
        patterns.push(KnownPattern::PlainAssignments(program));
        return patterns;
    }

    // Pattern: totals summed from two sides through a `getOrZero` helper, so
    // an absent side counts as zero and a zero total is not written. Kept apart
    // from `sum_directions`, which requires both sides and is shared.
    if normalised.contains("getOrZero(")
        && let Some(pattern) = crate::painless_totals::parse_sum_totals(normalised)
    {
        patterns.push(KnownPattern::SumTotals(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a scalar written from arithmetic over other fields. LAST, and
    // deliberately: every arm above returns as soon as it claims a script, so a
    // general evaluator placed here can only take what nothing else took.
    //
    // A script reading `params` is declined -- this lane never sees the params
    // block, so claiming one would mark it handled and write nothing.
    if let Some(pattern) = crate::painless_expr::parse_scalar_expression(normalised)
        && !pattern.reads_params()
    {
        patterns.push(KnownPattern::ScalarExpression(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a string trimmed into one field and split into a list in
    // another. LAST, so it can only take what nothing above took; its own
    // parse is the trigger, and the `/<c>/.split(` literal is the cheap
    // reject.
    if normalised.contains("/.split(")
        && let Some(pattern) = parse_trim_then_split(normalised)
    {
        patterns.push(KnownPattern::TrimThenSplit(Box::new(pattern)));
        return patterns;
    }

    // Pattern: ECS fields written from the members of a list's items, under
    // nested guards -- the script all three Atlassian audit streams ship.
    // LAST, because nothing above claims it today and a general reader placed
    // here can only take what nothing else took. It reads a closed grammar
    // rather than one vendor's spelling, so the arm has to be somewhere it
    // cannot shadow a narrower matcher.
    if normalised.contains(".length;")
        && normalised.contains(".put(")
        && let Some(pattern) = crate::painless_item_writes::parse_item_writes(normalised)
    {
        patterns.push(KnownPattern::ItemWrites(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a value cut at the Nth separator counted from its END, the
    // prefix kept -- gigamon's DNS subdomain. LAST, because nothing above
    // claims it today (`binding: []` in the static census, `ran 0` in the
    // runtime reach dump) and a reader placed here can only take what nothing
    // else took. The trigger is the helper's own declaration, and the parse is
    // what decides.
    if normalised.contains(".charAt(")
        && normalised.contains(".substring(")
        && let Some(pattern) = crate::painless_nth_separator::parse_nth_separator_prefix(normalised)
    {
        patterns.push(KnownPattern::NthSeparatorPrefix(Box::new(pattern)));
        return patterns;
    }

    // The four below sit LAST, so each can only take a script nothing above
    // took and none can shadow a narrower arm.

    // Pattern: what a regex matched in the first list member it matched at all.
    if normalised.contains(".matcher(")
        && normalised.contains(".find()")
        && let Some(pattern) = parse_first_match_in_list(normalised)
    {
        patterns.push(KnownPattern::FirstMatchInList(Box::new(pattern)));
        return patterns;
    }

    // Pattern: an integer divided by a literal and written back through a
    // local. Gated on the `instanceof Long` the parse then ties to the divided
    // expression itself, so a script that divides something else under one is
    // declined.
    if normalised.contains("instanceof Long")
        && normalised.contains('/')
        && let Some(pattern) = parse_long_divide(normalised)
    {
        patterns.push(KnownPattern::LongDivide(Box::new(pattern)));
        return patterns;
    }

    // Pattern: a field cut back to everything past its first n characters, in
    // place -- the `?` off the front of cloudflare's `url.query`. The parse
    // decides; the literal is the cheap reject.
    if normalised.contains(".substring(")
        && let Some(pattern) = parse_drop_leading_chars(normalised)
    {
        patterns.push(pattern);
        return patterns;
    }

    // Pattern: named epoch fields rescaled to milliseconds by magnitude, which
    // every cloudflare stream runs ahead of a `UNIX_MS` date processor. All
    // ten spellings in the tree read `binding: []` before this arm, so it
    // shadows nothing.
    if normalised.contains("(long)(1e18)")
        && let Some(pattern) = parse_epoch_to_millis(normalised)
    {
        patterns.push(KnownPattern::EpochToMillis(Box::new(pattern)));
        return patterns;
    }

    patterns
}

/// A Rust string literal, escaped for the generated source.
#[cfg(feature = "codegen")]
fn rust_str(value: &str) -> String {
    format!("{value:?}")
}

#[cfg(feature = "codegen")]
impl KnownPattern {
    /// The runner call that reproduces this pattern, for a generator emitting it
    /// directly instead of the script.
    ///
    /// `None` means the pattern is not expressible yet and the call site keeps
    /// the ladder. This is an ALLOWLIST: a pattern is added here only once its
    /// runner takes extracted params and its payload can be written as
    /// literals, so a wrong emit is impossible rather than merely unlikely.
    #[allow(clippy::too_many_lines)] // One arm per emittable pattern; it grows with the allowlist.
    pub(crate) fn direct_call(&self) -> Option<String> {
        match self {
            Self::DropEmpty { policy, root } => {
                // Only the axes this script turns on, over `none()`. Listing
                // every field would make each new axis a regeneration of the
                // whole tree.
                let mut set: Vec<String> = [
                    ("nulls", policy.nulls),
                    ("empty_strings", policy.empty_strings),
                    ("empty_collections", policy.empty_collections),
                    ("prune_lists", policy.prune_lists),
                    ("shallow", policy.shallow),
                ]
                .into_iter()
                .filter(|(_, on)| *on)
                .map(|(name, _)| format!("{name}: true"))
                .collect();
                if !policy.sentinels.is_empty() {
                    let literals: Vec<String> = policy
                        .sentinels
                        .iter()
                        .map(|s| format!("{}.into()", rust_str(s)))
                        .collect();
                    set.push(format!("sentinels: vec![{}]", literals.join(", ")));
                }
                let root = root
                    .as_deref()
                    .map_or_else(|| "None".to_string(), |r| format!("Some({})", rust_str(r)));
                Some(format!(
                    "drop_empty(event, &DropPolicy {{ {}, ..DropPolicy::none() }}, {root});",
                    set.join(", "),
                ))
            }
            Self::AllowedValueCopy(pattern) => {
                let allowed: Vec<String> = pattern
                    .allowed
                    .iter()
                    .map(|value| format!("{}.into()", rust_str(value)))
                    .collect();
                Some(format!(
                    "allowed_value_copy(event, &AllowedValueCopy::new({}, {}, vec![{}], {}));",
                    rust_str(&pattern.source),
                    pattern.lower,
                    allowed.join(", "),
                    rust_str(&pattern.target),
                ))
            }
            Self::EnsureAppend(pattern) => Some(format!(
                "ensure_append(event, &EnsureAppend::new({}, {}));",
                rust_str(&pattern.source),
                rust_str(&pattern.target),
            )),
            Self::GeoPointFromCoordinates(pattern) => Some(format!(
                "geo_point_from_coordinates(event, &GeoPointFromCoordinates::new({}, {}, {}, {}, {}));",
                rust_str(&pattern.coordinates),
                rust_str(&pattern.target),
                pattern.lon_index,
                pattern.lat_index,
                pattern.lon_first,
            )),
            Self::RemoveEmptyChildMaps(pattern) => Some(pattern.direct_call()),
            Self::SplitAtDelimiter(pattern) => Some(pattern.direct_call()),
            Self::MoveMapEntry(pattern) => Some(pattern.direct_call()),
            Self::RenameMapKeys(pattern) => Some(pattern.direct_call()),
            Self::RewriteKeys(pattern) => Some(pattern.direct_call()),
            Self::ParametersIntoMap(pattern) => Some(pattern.direct_call()),
            Self::UnwrapSuffixedKeys(pattern) => Some(pattern.direct_call()),
            Self::MailtoUriFields(pattern) => Some(format!(
                "mailto_uri_fields(event, &MailtoUriFields::new({}.into(), {}.into()));",
                rust_str(&pattern.source),
                rust_str(&pattern.target),
            )),
            Self::EnsurePrefix(pattern) => Some(format!(
                "ensure_prefix(event, &EnsurePrefix::new({}.into(), {}.into(), '{}', {}.into()));",
                rust_str(&pattern.source),
                rust_str(&pattern.target),
                pattern.marker,
                rust_str(&pattern.prefix),
            )),
            Self::CoerceBoolean(pattern) => Some(pattern.direct_call()),
            Self::JoinPresentFields(pattern) => Some(format!(
                "join_present_fields(event, &JoinPresentFields::new(vec![{}], {}, {}));",
                pattern
                    .sources
                    .iter()
                    .map(|source| format!("{}.to_owned()", rust_str(source)))
                    .collect::<Vec<_>>()
                    .join(", "),
                rust_str(&pattern.separator),
                rust_str(&pattern.target),
            )),
            Self::OctalString(pattern) => Some(format!(
                "octal_string(event, &OctalString::new({}, {}));",
                rust_str(&pattern.source),
                rust_str(&pattern.target),
            )),
            Self::StringOps(pattern) => {
                let ops: Vec<String> = pattern
                    .ops
                    .iter()
                    .map(|op| match op {
                        StringOp::Lower => "StringOp::Lower".to_string(),
                        StringOp::Upper => "StringOp::Upper".to_string(),
                        StringOp::Trim => "StringOp::Trim".to_string(),
                        StringOp::Replace { from, to } => format!(
                            "StringOp::Replace {{ from: {}.into(), to: {}.into() }}",
                            rust_str(from),
                            rust_str(to),
                        ),
                    })
                    .collect();
                Some(format!(
                    "string_ops(event, &StringOps::new({}, {}, vec![{}]));",
                    rust_str(&pattern.source),
                    rust_str(&pattern.target),
                    ops.join(", "),
                ))
            }
            Self::GuardedReplace(pattern) => Some(format!(
                "guarded_replace(event, &GuardedReplace::new({}, {}, {}, {}));",
                rust_str(&pattern.source),
                rust_str(&pattern.target),
                rust_str(&pattern.from),
                rust_str(&pattern.to),
            )),
            Self::ScalarExpression(pattern) => Some(pattern.direct_call()),
            Self::MemberFromVariantKey(pattern) => Some(pattern.direct_call()),
            Self::ListItemRenames(pattern) => Some(pattern.direct_call()),
            Self::DurationWindow(pattern) => Some(pattern.direct_call()),
            Self::SumTotals(pattern) => Some(pattern.direct_call()),
            Self::KvIntoFields(target) => {
                Some(format!("kv_into_fields(event, {});", rust_str(target)))
            }
            Self::SumDirections(units) => {
                let list: Vec<String> = units.iter().map(|unit| rust_str(unit)).collect();
                Some(format!("sum_directions(event, &[{}]);", list.join(", ")))
            }
            Self::ScaleField(pattern) => {
                let factor = match pattern.factor {
                    Factor::Long(n) => format!("Factor::Long({n})"),
                    Factor::Double(n) => format!("Factor::Double({n:?})"),
                };
                Some(format!(
                    "scale_field(event, &ScaleField::new({}, {}, {factor}));",
                    rust_str(&pattern.source),
                    rust_str(&pattern.target),
                ))
            }
            Self::SyslogPriority(pattern) => {
                let source = pattern.source.as_deref().map_or_else(
                    || "None".to_string(),
                    |s| format!("Some({}.into())", rust_str(s)),
                );
                Some(format!(
                    "syslog_priority(event, &SyslogPriorityScript::new({source}, {}, {}, {}));",
                    pattern.facility, pattern.severity, pattern.names,
                ))
            }
            _ => None,
        }
    }
}

/// Run one matcher branch against one event.
///
/// Returns whether the script counts as HANDLED, with each branch's semantics
/// unchanged from the old inline dispatch: a guarded branch may decline, and
/// the caller then tries the next pattern in the list.
#[allow(clippy::too_many_lines)] // One delegation arm per pattern; it grows with the pattern list.
pub(crate) fn run_known_pattern(
    event: &mut Event,
    normalised: &str,
    pattern: &KnownPattern,
) -> bool {
    match pattern {
        KnownPattern::ScalarExpression(pattern) => {
            crate::painless_expr::scalar_expression(event, pattern)
        }
        KnownPattern::MemberFromVariantKey(pattern) => member_from_variant_key(event, pattern),
        KnownPattern::ListItemRenames(pattern) => {
            crate::painless_lists::list_item_renames(event, pattern)
        }
        KnownPattern::ItemWrites(pattern) => {
            crate::painless_item_writes::item_writes(event, pattern)
        }
        KnownPattern::DurationWindow(pattern) => duration_window(event, pattern),
        KnownPattern::SumTotals(pattern) => crate::painless_totals::sum_totals(event, pattern),
        KnownPattern::DropEmpty { policy, root } => drop_empty(event, policy, root.as_deref()),
        KnownPattern::SplitCommandLine(script) => {
            crate::painless_windows::run_argv_script(event, script)
        }
        KnownPattern::Basename(pattern) => basename_cuts(event, pattern),
        KnownPattern::FileInfo(source) => crate::painless_windows::run_file_info(event, source),
        KnownPattern::HashLowercase(source) => {
            crate::painless_windows::run_hash_lowercase(event, source)
        }
        KnownPattern::PrefixTail {
            source,
            prefix,
            strip_comma,
            target,
        } => try_prefix_tail(event, source, prefix, *strip_comma, target),
        KnownPattern::PrependToArray {
            scalar,
            array,
            target,
        } => try_prepend_to_array(event, scalar, array, target),
        KnownPattern::ResourcesRenameDedup(source) => run_resources_rename_dedup(event, source),
        KnownPattern::SecurityhubResource(source) => run_securityhub_resource(event, source),
        KnownPattern::CheckpointPackets => run_checkpoint_packets(event),
        KnownPattern::ConsoleLoginEventData => run_console_login_event_data(event),
        KnownPattern::BitFlagNames(decode) => run_bit_flag_names(event, decode),
        KnownPattern::BitNameArray(pattern) => run_bit_name_array(event, pattern),
        KnownPattern::PascalKeys(pattern) => run_pascal_keys(event, pattern),
        KnownPattern::SecondsToSpan(source) => run_seconds_to_span(event, source),
        KnownPattern::SubstringBeforeLast {
            source,
            target,
            needle,
        } => run_substring_before_last(event, source, target, needle),
        KnownPattern::TlsVersionSplit { source } => run_tls_version_split(event, source),
        KnownPattern::ConcatParts(script) => run_concat_parts(event, script),
        KnownPattern::ConcatAssignment(script) => run_concat_assignment(event, script),
        KnownPattern::TrimListInPlace(field) => run_trim_list(event, field),
        KnownPattern::StartsWithAppend {
            source,
            prefix,
            target,
            value,
        } => run_starts_with_append(event, source, prefix, target, value),
        KnownPattern::InspectorResources { multi } => run_inspector_resources(event, *multi),
        KnownPattern::MailRelated(script) => run_mail_related(event, script),
        KnownPattern::GcpRelatedEntity => run_gcp_related_entity(event),
        KnownPattern::RemoveListValue { field, value } => {
            run_remove_list_value(event, field, value)
        }
        KnownPattern::RemoveMapValue { field, value } => run_remove_map_value(event, field, value),
        KnownPattern::DatePlusDays {
            source,
            target,
            days,
        } => run_date_plus_days(event, source, target, *days),
        KnownPattern::NestedKeyNames {
            source,
            inner,
            target,
        } => run_nested_key_names(event, source, inner, target),
        KnownPattern::FirehoseDataset => run_firehose_dataset(event),
        KnownPattern::CtxTableLookup { table, key, target } => {
            run_ctx_table_lookup(event, table, key, target)
        }
        KnownPattern::KvIntoNamespace(target) => run_kv_into_namespace(event, target),
        KnownPattern::KvIntoFields(target) => kv_into_fields(event, target),
        KnownPattern::AllowedValueCopy(pattern) => allowed_value_copy(event, pattern),
        KnownPattern::OctalString(pattern) => octal_string(event, pattern),
        KnownPattern::EnsureAppend(pattern) => ensure_append(event, pattern),
        KnownPattern::JoinPresentFields(pattern) => join_present_fields(event, pattern),
        KnownPattern::CoerceBoolean(pattern) => coerce_boolean(event, pattern),
        KnownPattern::RemoveEmptyChildMaps(pattern) => remove_empty_child_maps(event, pattern),
        KnownPattern::EnsurePrefix(pattern) => ensure_prefix(event, pattern),
        KnownPattern::SplitAtDelimiter(pattern) => split_at_delimiter(event, pattern),
        KnownPattern::MailtoUriFields(pattern) => mailto_uri_fields(event, pattern),
        KnownPattern::MoveMapEntry(pattern) => move_map_entry(event, pattern),
        KnownPattern::RenameMapKeys(pattern) => rename_map_keys(event, pattern),
        KnownPattern::ParametersIntoMap(pattern) => parameters_into_map(event, pattern),
        KnownPattern::UnwrapSuffixedKeys(pattern) => unwrap_suffixed_keys(event, pattern),
        KnownPattern::GeoPointFromCoordinates(pattern) => {
            geo_point_from_coordinates(event, pattern)
        }
        KnownPattern::WktGeometry(pattern) => wkt_geometry(event, pattern),
        KnownPattern::CsvMapToArray { source, target } => {
            run_csv_map_to_array(event, source, target)
        }
        KnownPattern::TagsAndMarking(pattern) => run_tags_and_marking(event, pattern),
        KnownPattern::DecayWindow(pattern) => run_decay_window(event, pattern),
        KnownPattern::CsvColonPairs {
            list,
            map_target,
            fingerprint_target,
            aliases,
        } => run_csv_colon_pairs(event, list, map_target, fingerprint_target, aliases),
        KnownPattern::DedupeUnwrap(field) => run_dedupe_unwrap(event, field),
        KnownPattern::DropLastChar(field) => run_drop_last_char(event, field),
        KnownPattern::DropLeadingChars { field, count } => {
            run_drop_leading_chars(event, field, *count)
        }
        KnownPattern::EpochToMillis(pattern) => run_epoch_to_millis(event, pattern),
        KnownPattern::M365ProcessEvidence(path) => run_m365_process_evidence(event, path),
        KnownPattern::M365IdentityEvidence(path) => run_m365_identity_evidence(event, path),
        KnownPattern::Route53Answers => run_route53_answers(event),
        KnownPattern::ReverseLookupAddress => run_reverse_lookup_address(event),
        KnownPattern::TruthyAssignments(pairs) => run_truthy_assignments(event, pairs),
        KnownPattern::LongOperationSession { first, last } => {
            run_long_operation_session(event, first, last)
        }
        KnownPattern::ScriptBlockEntropy(source) => {
            crate::painless_windows::run_script_block_entropy(event, source)
        }
        KnownPattern::SplitOnPipe(fields) => run_split_on_pipe(event, fields),
        KnownPattern::ZipAttachments(zip) => run_zip_attachments(event, zip),
        KnownPattern::ZipColumns(zip) => run_zip_columns(event, zip),
        KnownPattern::MaxByContains(pattern) => run_max_by_contains(event, pattern),
        KnownPattern::SecurityhubResources(source) => {
            if let Some(Value::Array(resources)) = event.get(source).cloned()
                && resources.len() > 1
            {
                run_securityhub_multi(event, &resources)
            } else {
                true
            }
        }
        KnownPattern::CategoryTypeLadder(arms) => run_category_type_ladder(event, arms),
        KnownPattern::KeysStripWhitespace(source) => {
            if let Some(Value::Object(entries)) = event.get(source).cloned() {
                let mut rebuilt = Map::new();
                for (key, value) in entries {
                    let stripped: String = key.chars().filter(|c| !c.is_whitespace()).collect();
                    rebuilt.insert(stripped, value);
                }
                let _ = event.set(source, Value::Object(rebuilt));
            }
            true
        }
        KnownPattern::RewriteKeys(pattern) => rewrite_keys(event, pattern),
        KnownPattern::SnakeKeyMapCopy { source, target } => {
            if let Some(Value::Object(entries)) = event.get(source).cloned() {
                let mut rebuilt = Map::new();
                for (key, value) in entries {
                    // The helper breaks a word only where a lowercase run
                    // meets an uppercase one, so `memorySizeMB` becomes
                    // `memory_size_mb` and not `memory_size_m_b`.
                    rebuilt.insert(to_snake_case(&key, SnakeRule::OnWordBreak), value);
                }
                let _ = event.set(target, Value::Object(rebuilt));
            }
            true
        }
        KnownPattern::StripAnglePairs { scalars, lists } => {
            let strip = |text: &str| {
                text.strip_prefix('<')
                    .and_then(|t| t.strip_suffix('>'))
                    .map(str::to_string)
            };
            for path in scalars {
                if let Some(stripped) = event.get_str(path).and_then(strip) {
                    let _ = event.set(path, json!(stripped));
                }
            }
            for path in lists {
                if let Some(Value::Array(items)) = event.get(path).cloned() {
                    let rebuilt: Vec<Value> = items
                        .into_iter()
                        .map(|item| match item.as_str().and_then(strip) {
                            Some(stripped) => Value::String(stripped),
                            None => item,
                        })
                        .collect();
                    let _ = event.set(path, Value::Array(rebuilt));
                }
            }
            true
        }
        KnownPattern::NameValueFold {
            source,
            target,
            key_member,
            value_member,
        } => run_name_value_fold(event, source, target, key_member, value_member),
        KnownPattern::OutcomeFromTags {
            action_field,
            tags,
            target,
        } => run_outcome_from_tags(event, action_field, tags, target),
        KnownPattern::RenameCommonAuth(codes) => {
            crate::painless_windows::run_rename_common_auth(event, codes)
        }
        KnownPattern::ProcessCreated(codes) => {
            crate::painless_windows::run_process_created(event, codes)
        }
        KnownPattern::ShareFilePath(codes) => {
            crate::painless_windows::run_share_file_path(event, codes)
        }
        KnownPattern::ObjectDn => crate::painless_windows::run_object_dn(event),
        KnownPattern::CollectColumns(pattern) => run_collect_columns(event, pattern),
        KnownPattern::SliceEachItem(pattern) => run_slice_each_item(event, pattern),
        KnownPattern::ContainsLadder(pattern) => run_contains_ladder(event, pattern),
        KnownPattern::RangeLadder(pattern) => run_range_ladder(event, pattern),
        KnownPattern::ScoreSeverityBands(pattern) => run_score_severity_bands(event, pattern),
        KnownPattern::IndexedFieldCopies(pattern) => run_indexed_field_copies(event, pattern),
        KnownPattern::FieldsIntoMap(pattern) => run_fields_into_map(event, pattern),
        KnownPattern::CrowdstrikeTimelineEntityIdentity => {
            try_crowdstrike_timeline_entity_identity(event)
        }
        KnownPattern::CrowdstrikeTimelineEntityAccounts => {
            try_crowdstrike_timeline_entity_accounts(event)
        }
        KnownPattern::FirstElement(pattern) => run_first_element(event, pattern),
        KnownPattern::NamedMapEntry(pattern) => run_named_map_entry(event, pattern),
        KnownPattern::TitleCase(pattern) => run_title_case(event, pattern),
        KnownPattern::SuffixAfterSeparator {
            source,
            target,
            separator,
            whole_when_absent,
        } => run_suffix_after_separator(event, source, target, separator, *whole_when_absent),
        KnownPattern::CopyTargetUser(codes) => {
            crate::painless_windows::run_copy_target_user(event, codes)
        }
        KnownPattern::CopySubjectUser(codes) => {
            crate::painless_windows::run_copy_subject_user(event, codes)
        }
        KnownPattern::CopyMemberName(codes) => {
            crate::painless_windows::run_copy_member_name(event, codes)
        }
        KnownPattern::CopyComputerObject(codes) => {
            crate::painless_windows::run_copy_computer_object(event, codes)
        }
        KnownPattern::CopyUserToBase {
            codes,
            base,
            sid_field,
        } => crate::painless_windows::run_copy_user_to_base(event, codes, base, sid_field),
        KnownPattern::SplitPipeFields(fields) => run_split_pipe_fields(event, fields),
        KnownPattern::SplitTokenField(pattern) => run_split_token_field(event, pattern),
        KnownPattern::TrimThenSplit(pattern) => run_trim_then_split(event, pattern),
        KnownPattern::DecodeBase64 { source, target } => run_decode_base64(event, source, target),
        KnownPattern::TokenCount {
            source,
            separator,
            target,
        } => run_token_count(event, source, separator, target),
        KnownPattern::WrapValueInList {
            source,
            target,
            remove_source,
        } => {
            if let Some(value) = event.get(source).cloned() {
                let _ = event.set(target, Value::Array(vec![value]));
                if *remove_source {
                    event.remove(source);
                }
            }
            true
        }
        KnownPattern::LastElement { array, target } => {
            if let Some(Value::Array(items)) = event.get(array)
                && let Some(last) = items.last().cloned()
            {
                let _ = event.set(target, last);
            }
            true
        }
        KnownPattern::ClassifyMembers => try_classify_members(event, normalised),
        KnownPattern::FlattenedDuplicates => try_flattened_duplicates(event, normalised),
        KnownPattern::CollectEntities => try_collect_entities(event, normalised),
        KnownPattern::DnsRdataAnswers => try_dns_rdata_answers(event, normalised),
        KnownPattern::StructuredRdataAnswers => try_structured_rdata_answers(event),
        KnownPattern::RelatedFromDnsAnswers => try_related_from_dns_answers(event),
        KnownPattern::AnswersFromResolvedIp => try_answers_from_resolved_ip(event),
        KnownPattern::CamelToSnake {
            target,
            source,
            rule,
            drop_at_keys,
            merge,
            removes,
        } => {
            if let Some(value) = event.get(source) {
                let converted = camel_map_to_snake(value, *rule, *drop_at_keys);
                // `putAll` keeps what the target already holds; an arriving key
                // wins, which is what Java's Map::putAll does.
                match (merge, converted) {
                    (true, Value::Object(arriving)) => {
                        for (key, value) in arriving {
                            let _ = event.set(&format!("{target}.{key}"), value);
                        }
                    }
                    (_, converted) => {
                        let _ = event.set(target, converted);
                    }
                }
            }
            // Outside the null guard, exactly as the script writes it.
            for path in removes {
                event.remove(path);
            }
            true
        }
        KnownPattern::SplitTrimCollect => try_split_trim_collect(event, normalised),
        KnownPattern::SumDirections(totals) => sum_directions(event, totals),
        KnownPattern::CombineFields(pattern) => combine_fields(event, pattern),
        KnownPattern::DurationToNanos => try_duration_to_nanos(event, normalised),
        KnownPattern::FloatSecondsToNanos(pattern) => float_seconds_to_nanos(event, pattern),
        KnownPattern::FirstPresentKeyName(pattern) => first_present_key_name(event, pattern),
        KnownPattern::SumMemberOverList(pattern) => sum_member_over_list(event, pattern),
        KnownPattern::SnakeCaseListElements(pattern) => snake_case_list_elements(event, pattern),
        KnownPattern::DedupeMapValues(pattern) => dedupe_map_values(event, pattern),
        KnownPattern::EmailsToRelatedUsers(pattern) => run_emails_to_related_users(event, pattern),
        KnownPattern::FlowDuration => try_flow_duration(event, normalised),
        KnownPattern::ParallelDispatch => try_parallel_dispatch(event, normalised),
        KnownPattern::ConcatMessage => try_concat_message(event, normalised),
        KnownPattern::SwapSubtrees => try_swap_subtrees(event, normalised),
        KnownPattern::CollectingLadder => try_collecting_ladder(event, normalised),
        KnownPattern::CaseInsensitiveLadder => try_case_insensitive_ladder(event, normalised),
        KnownPattern::EqualityLadder(ladder) => try_ladder(event, ladder),
        KnownPattern::SentinelRemovalLiteral => try_sentinel_removal_literal(event, normalised),
        KnownPattern::ListMemberSelect(pattern) => run_list_member_select(event, pattern),
        KnownPattern::RowLookupWithFallback(pattern) => {
            try_row_lookup_with_fallback(event, normalised, pattern)
        }
        KnownPattern::SchemelessUrl => try_schemeless_url(event),
        KnownPattern::VersionSplit => try_version_split(event, normalised),
        KnownPattern::SyslogPriority(pattern) => syslog_priority(event, pattern),
        KnownPattern::AppendUnique { from, into } => try_append_unique(event, from, into),
        KnownPattern::SplitUnquotedKv(split) => run_split_unquoted_kv(event, split),
        KnownPattern::ArrayToIndexedObject => try_array_to_indexed_object(event, normalised),
        KnownPattern::ListRenameTable(pattern) => run_list_rename_table(event, pattern),
        KnownPattern::KeyValuePairs => try_key_value_pairs(event, normalised),
        KnownPattern::JoinOptional => try_join_optional(event, normalised),
        KnownPattern::AppendEach(pattern) => try_append_each(event, pattern),
        KnownPattern::LiteralValueMap(pattern) => literal_value_map(event, pattern),
        KnownPattern::KeysToSnakeCase(field, rule) => {
            // A script that names no container is NOT a licence to rewrite the
            // whole document. Converting every key destroyed the vendor's own
            // spellings elsewhere in the event: tanium's payload keys are
            // `Computer IP` and `Event Id`, and turning them into
            // `computer _i_p` left every later read of them empty -- 1,844 of
            // its 1,968 wrong fields, with no error to show for it.
            let Some(field) = field else {
                return false;
            };
            if let Some(mut val) = event.get(field).cloned() {
                keys_to_snake_case(&mut val, *rule);
                let _ = event.set(field, val);
            }
            true
        }
        KnownPattern::CommandLine { parent } => {
            if *parent {
                let _ = extract_process_fields(
                    event,
                    "crowdstrike.event.ParentCommandLine",
                    "process.parent",
                );
            } else {
                let _ = extract_process_fields(event, "crowdstrike.event.CommandLine", "process");
            }
            true
        }
        KnownPattern::ProcessStartTime => {
            let _ =
                epoch_to_timestamp(event, "crowdstrike.event.ProcessStartTime", "process.start");
            true
        }
        KnownPattern::EmailSplit(pattern) => run_email_split(event, pattern),
        KnownPattern::RiskBehaviors => try_risk_behaviors(event),
        KnownPattern::AzureCategoryEventType => try_azure_category_to_event_type(event),
        KnownPattern::AzureEventCategory => try_azure_event_category(event, normalised),
        KnownPattern::ReplaceDotsInKeys => try_replace_dots_in_keys(event, normalised),
        KnownPattern::OktaTargetRename => try_okta_target_rename(event),
        KnownPattern::KeysBySuffix(pattern) => run_keys_by_suffix(event, pattern),
        KnownPattern::CollectMapValues => try_collect_map_values(event, normalised),
        KnownPattern::StringOps(pattern) => string_ops(event, pattern),
        KnownPattern::GuardedReplace(pattern) => guarded_replace(event, pattern),
        KnownPattern::ScaleField(pattern) => scale_field(event, pattern),
        KnownPattern::MillisecondLadder(pattern) => run_millisecond_ladder(event, pattern),
        KnownPattern::RoundedScale(pattern) => run_rounded_scale(event, pattern),
        KnownPattern::NanosBetween(pattern) => run_nanos_between(event, pattern),
        KnownPattern::ProcessNameFromCommandLine => run_process_name_from_command_line(event),
        KnownPattern::FlagsPresent(pattern) => run_flags_present(event, pattern),
        KnownPattern::ZipLists(pattern) => run_zip_lists(event, pattern),
        KnownPattern::FirstOrSelf(pattern) => run_first_or_self(event, pattern),
        KnownPattern::WrapMapInList(path) => {
            if let Some(Value::Object(_)) = event.get(path) {
                let held = event.get(path).cloned().unwrap_or(Value::Null);
                let _ = event.set(path, Value::Array(vec![held]));
            }
            true
        }
        KnownPattern::GuardedDivide {
            target,
            source,
            absent,
            divisor,
        } => run_guarded_divide(event, target, source, absent.as_ref(), *divisor),
        KnownPattern::FlattenMapInto(pattern) => run_flatten_map_into(event, pattern),
        KnownPattern::StringifyLongs(fields) => run_stringify_longs(event, fields),
        KnownPattern::CopyByLabel(pattern) => run_copy_by_label(event, pattern),
        KnownPattern::BandLadder(pattern) => run_band_ladder(event, pattern),
        KnownPattern::StripSurroundingPair(pattern) => run_strip_surrounding_pair(event, pattern),
        KnownPattern::IocExpiry(pattern) => run_ioc_expiry(event, pattern),
        KnownPattern::GuardedCopy(literals) => try_guarded_copy(event, normalised, literals),
        KnownPattern::PlainAssignments(literals) => literals.run(event),
        KnownPattern::BranchCopies(branches) => run_branch_copies(event, branches),
        KnownPattern::PrivateCidrDirection {
            source,
            destination,
            target,
        } => run_private_cidr_direction(event, source, destination, target),
        KnownPattern::ClassifyLadder(pattern) => run_classify_ladder(event, pattern),
        KnownPattern::MoveKeys(moves) => run_move_keys(event, moves),
        KnownPattern::MergeMapUp(pattern) => run_merge_map_up(event, pattern),
        KnownPattern::ParameterFanOut(pattern) => run_parameter_fan_out(event, pattern),
        KnownPattern::QuotedKvScan(pattern) => run_quoted_kv_scan(event, pattern),
        KnownPattern::LocalMapLookup(pattern) => run_local_map_lookup(event, pattern),
        KnownPattern::UnreservedKeyPayload(pattern) => run_unreserved_key_payload(event, pattern),
        KnownPattern::HashesByLength(pattern) => run_hashes_by_length(event, pattern),
        KnownPattern::MapEntryToBoolean(pattern) => run_map_entry_to_boolean(event, pattern),
        KnownPattern::NestUnder(pattern) => run_nest_under(event, pattern),
        KnownPattern::CollectFromList(pattern) => run_collect_from_list(event, pattern),
        KnownPattern::RecordLookup(pattern) => run_record_lookup(event, pattern),
        KnownPattern::EqualsLiteralFlag(pattern) => run_equals_literal_flag(event, pattern),
        KnownPattern::PositionInList(pattern) => run_position_in_list(event, pattern),
        KnownPattern::SplitFirstLabel(pattern) => run_split_first_label(event, pattern),
        KnownPattern::SuffixesByPrefix(pattern) => run_suffixes_by_prefix(event, pattern),
        KnownPattern::NthSeparatorPrefix(pattern) => {
            crate::painless_nth_separator::nth_separator_prefix(event, pattern)
        }
        KnownPattern::SplitIntoRecords(pattern) => run_split_into_records(event, pattern),
        KnownPattern::PrependSplit(pattern) => run_prepend_split(event, pattern),
        KnownPattern::FirstMatchInList(pattern) => run_first_match_in_list(event, pattern),
        KnownPattern::LongDivide(pattern) => run_long_divide(event, pattern),
    }
}

/// An address split on `@`, and every path each half is written to.
///
/// ```painless
/// String[] splitmail = ctx.user.id.splitOnToken("@");
/// if (splitmail.length != 2) { return; }
/// ctx.user.email = ctx.user.id;
/// ctx.user.domain = splitmail[1];
/// ctx.user.name = splitmail[0];
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EmailSplit {
    /// The field holding the address.
    source: String,
    /// Paths taking the part before the `@`.
    names: Vec<String>,
    /// Paths taking the part after it.
    domains: Vec<String>,
    /// Paths taking the whole address back.
    emails: Vec<String>,
}

/// Read the split's source and targets off the script.
///
/// Four hard-coded `<prefix>.id` cases used to stand for this, and
/// `google_workspace` fits none of them: it reads `source.user.email` and
/// writes BOTH `user.*` and `source.user.*`, which cost it `user.name` on 437
/// of its 543 events and `related.hosts` with it.
fn parse_email_split(script: &str) -> Option<EmailSplit> {
    use crate::painless_params::{clean_path, ctx_path_before};

    let source = ctx_path_before(script, ".splitOnToken(")?;
    let mut split = EmailSplit {
        source,
        names: Vec::new(),
        domains: Vec::new(),
        emails: Vec::new(),
    };

    for statement in script.split(';') {
        let Some((lhs, rhs)) = statement.split_once('=') else {
            continue;
        };
        // The statement carries whatever block punctuation preceded it, so the
        // target is the last thing on the line rather than the whole left side.
        let Some(target) = lhs
            .rsplit(['\n', '{', '}'])
            .next()
            .map(str::trim)
            .and_then(|last| last.strip_prefix("ctx."))
            .map(clean_path)
        else {
            continue;
        };
        let rhs = rhs.trim();
        if rhs.ends_with("[0]") {
            split.names.push(target);
        } else if rhs.ends_with("[1]") {
            split.domains.push(target);
        } else if rhs
            .strip_prefix("ctx.")
            .map(clean_path)
            .is_some_and(|read| read == split.source)
        {
            split.emails.push(target);
        }
    }

    // A script that names neither half is one of the truncated forms, and both
    // land beside the address -- where every spelled-out variant puts them.
    if split.names.is_empty() && split.domains.is_empty() {
        let parent = split.source.rsplit_once('.').map_or("", |(head, _)| head);
        split.names.push(format!("{parent}.name"));
        split.domains.push(format!("{parent}.domain"));
    }
    Some(split)
}

fn run_email_split(event: &mut Event, split: &EmailSplit) -> bool {
    let Some(address) = event.get_string(&split.source) else {
        // The script's own guard: no address, nothing to split.
        return true;
    };
    let parts: Vec<&str> = address.split('@').collect();
    if parts.len() != 2 {
        // `if (splitmail.length != 2) { return; }`
        return true;
    }

    for path in &split.emails {
        let _ = event.set(path, json!(address));
    }
    for path in &split.names {
        let _ = event.set(path, json!(parts[0]));
    }
    for path in &split.domains {
        let _ = event.set(path, json!(parts[1]));
    }
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
/// The container a `keysToSnakeCase` script actually converts.
///
/// [`extract_target_field`] looks for a bare `ctx.` line and skips any holding
/// a `(`, which every line of a script that CALLS the helper does. The target
/// is on the left of the assignment whose right side is the call.
fn snake_case_target(script: &str) -> Option<String> {
    // Scanned over the WHOLE text rather than line by line: a stored script
    // carries its newlines escaped, so it arrives here as one line and any
    // per-line split lands in the middle of the helper's own body.
    //
    // The helper's name is not fixed and the vendors spell it both ways --
    // `keysToSnakeCase` and `keys_to_snake_case_recursive` -- so the call is
    // identified by the name CONTAINING it. The LAST assignment wins, the way
    // the script's own order does.
    // The target may be spelled with a quoted subscript --
    // `ctx.cloudflare_logpush['workers_trace']` -- so the path accepts one.
    let re = crate::cached_regex!(
        r#"ctx\.([A-Za-z0-9_.?\[\]'"]+?)\s*(?:=\s*|\.putAll\(\s*)([A-Za-z0-9_]+)\(\s*ctx\."#
    )
    .fast()?;

    let mut target = None;
    for caps in re.captures_iter(script) {
        let Some(name) = caps.get(2) else { continue };
        let name = name.as_str().to_ascii_lowercase();
        if name.contains("snakecase") || name.contains("snake_case") {
            let dotted = caps
                .get(1)?
                .as_str()
                .replace("['", ".")
                .replace("[\"", ".")
                .replace("']", "")
                .replace("\"]", "");
            let path = clean_path(&dotted);
            if !path.is_empty() {
                target = Some(path);
            }
        }
    }
    target
}

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
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "painless_common_tests.rs"]
mod tests;
