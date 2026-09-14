// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Grok and regex patterns, compiled once per process instead of per event.
//!
//! Every grok site in the transforms used to convert the pattern string and
//! call `Regex::new` inside the transform function, so a 20,000-event batch
//! rebuilt the same DFA 20,000 times. Compiling a regex is microseconds;
//! matching one is nanoseconds. That ratio is the whole reason this exists.
//!
//! The pattern set is FIXED at compile time -- every caller passes a string
//! literal -- so the map fills during the first batch and is read-only after
//! that. Entries are leaked deliberately: they live for the life of the
//! process anyway, and `&'static` lets callers hold the compiled form without
//! a guard or a refcount on the hot path.
//!
//! A pattern that will not compile is logged once and answered with a regex
//! that matches nothing, rather than panicking. A malformed pattern is a
//! defect in one processor; taking the pod down over it stalls a partition.

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::RwLock;

use regex::Regex;

// The regex dispatch moved to `dfe-core` so the Painless matchers could reach a
// cached regex without depending on grok compilation, which needs the pattern
// registry and the capture-type rules here. Re-exported at the original path.
pub use dfe_core::regex_cache::{Pattern, regex};

/// The compiled form of a grok pattern literal, looked up once per CALL SITE.
///
/// [`grok`]'s shared `RwLock` costs 29 ns a lookup on one thread and 1,522 ns
/// on eight, because every worker bounces the same reader-count atomic. A
/// per-site `OnceLock` in front of it holds at ~1 ns under the same load.
///
/// The map stays underneath, so sites sharing a pattern share one instance.
#[macro_export]
macro_rules! cached_grok {
    ($pattern:literal $(,)?) => {{
        static SITE: ::std::sync::OnceLock<&'static $crate::grok_cache::CompiledGrok> =
            ::std::sync::OnceLock::new();
        *SITE.get_or_init(|| $crate::grok_cache::grok($pattern))
    }};
}

/// As [`cached_grok`], plus capture names the pattern carries as raw groups.
///
/// A pipeline may define its own grok name and capture it into a dotted path.
/// Inlining the definition turns that into a plain `(?P<a_b>...)` group, which
/// the expander never sees as `%{NAME:a.b}` and so never maps -- the value
/// would land on `a_b` and every later processor would miss it. The pairs here
/// are that mapping, supplied by whoever built the pattern.
#[macro_export]
macro_rules! cached_grok_mapped {
    ($pattern:literal, [$(($capture:literal, $path:literal)),* $(,)?] $(,)?) => {{
        static SITE: ::std::sync::OnceLock<&'static $crate::grok_cache::CompiledGrok> =
            ::std::sync::OnceLock::new();
        *SITE.get_or_init(|| $crate::grok_cache::grok_mapped($pattern, &[$(($capture, $path)),*]))
    }};
}

/// A grok pattern whose whole pattern a native parser can handle.
///
/// Even against an already-compiled regex, `dfe-parse` is roughly 4-5x faster
/// on these -- 79.7ns to 16.3ns for a bare address, 77.2ns to 18.4ns for an
/// address and port. That margin is why the native path exists; it is not
/// worth the divergence for patterns where the margin is not there.
///
/// Only whole-pattern matches qualify. A pattern with literal text around the
/// captures stays on the regex path, because the regex engine is genuinely
/// good at that and hand-rolling it would be a source of bugs.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Native {
    /// `%{GREEDYDATA:field}` alone -- the input's FIRST LINE into one field.
    ///
    /// 194 call sites over 35 distinct patterns, the widest single form in the
    /// tree, running a regex to copy a string. Not the whole input:
    /// `line_anchored` makes this `(?m)^.*$` because joni anchors to lines, so
    /// the leftmost match is line one. It always matches -- `.*` accepts an
    /// empty line. An unnamed `%{GREEDYDATA}` has nothing to write and stays
    /// on the regex.
    FirstLine { field: String },
    /// `^%{IPV4:field}$`
    Ipv4 { field: String },
    /// `^%{IPV4:addr}:%{PORT:port}$`
    Ipv4Port {
        addr: String,
        port: String,
        /// Whether the port capture declared a numeric type, which the regex
        /// path honours -- the two must write the same type.
        port_numeric: bool,
    },
}

/// A grok pattern in its compiled form.
pub struct CompiledGrok {
    /// The expanded regex. Always present, and always correct -- the native
    /// path below is an optimisation over it, never a replacement.
    pub regex: Pattern,
    /// Capture name to original dotted field path. Regex capture names cannot
    /// contain dots, so `user.name` is captured as `user_name` and restored
    /// through this map.
    pub field_map: HashMap<String, String>,
    /// Capture names Elastic's `:long` / `:int` / `:float` / `:double` or
    /// `:boolean` suffix types explicitly. A capture absent from the map is
    /// untyped and lands as a string, same as the expectations would see it.
    pub capture_types: HashMap<String, crate::codegen_api::CaptureType>,
    /// A native parser for this pattern, when one covers it exactly.
    native: Option<Native>,
    /// Capture names whose sub-pattern cannot match the empty string, so an
    /// empty match reported for one is a group that never participated.
    ///
    /// Only ever populated for a pattern on the backtracking engine, and only
    /// read there. See [`never_empty_captures`].
    never_empty: std::collections::HashSet<String>,
}

static GROK: RwLock<Option<HashMap<String, &'static CompiledGrok>>> = RwLock::new(None);

/// The compiled form of a grok `pattern`, built once per distinct pattern.
///
/// # Panics
///
/// Does not panic on a malformed pattern -- see the module docs. The `expect`
/// on the fallback cannot fire: [`NEVER_MATCHES`] is a valid regex, pinned by
/// a test.
#[must_use]
pub fn grok(pattern: &str) -> &'static CompiledGrok {
    grok_mapped(pattern, &[])
}

/// As [`grok`], with `extra` capture-name to dotted-path pairs merged into the
/// field map for groups the expander cannot see. See [`cached_grok_mapped`].
///
/// # Panics
///
/// Does not panic -- see [`grok`].
#[must_use]
pub fn grok_mapped(pattern: &str, extra: &[(&str, &str)]) -> &'static CompiledGrok {
    // Keyed by the pattern AND its mapping. Two sources can write the same
    // pattern text and want different destinations -- `%{SYSLOG5424PRI}` goes
    // to `syslog5424_pri` or to `log.syslog.priority` depending on the
    // processor's `ecs_compatibility` -- and keying on the text alone hands the
    // second caller whatever the first compiled.
    let key = cache_key(pattern, extra);
    if let Some(hit) = GROK
        .read()
        .ok()
        .and_then(|g| g.as_ref().and_then(|map| map.get(&key)).copied())
    {
        return hit;
    }

    let (expanded, mut field_map, mut capture_types) =
        crate::codegen_api::grok_to_regex_typed(pattern);
    // The generator's mapping is keyed by the capture name AS WRITTEN, which a
    // rename has since replaced. Substituting by destination rather than by
    // key keeps every renamed twin pointed at the same field.
    let supplied: std::collections::HashMap<&str, &str> = extra.iter().copied().collect();
    for (capture, path) in extra {
        field_map.insert((*capture).to_string(), (*path).to_string());
    }
    for path in field_map.values_mut() {
        if let Some(real) = supplied.get(path.as_str()) {
            *path = (*real).to_string();
        }
    }
    crate::codegen_api::resolve_capture_paths(&mut field_map, &mut capture_types);
    let expanded = to_rust_dialect(&expanded);

    let regex = Pattern::compile(&expanded, pattern);
    let never_empty = match &regex {
        Pattern::Fast(_) => std::collections::HashSet::new(),
        Pattern::Backtracking(_) => never_empty_captures(&expanded),
    };
    let compiled: &'static CompiledGrok = Box::leak(Box::new(CompiledGrok {
        regex,
        field_map,
        capture_types,
        native: native_form(pattern),
        never_empty,
    }));

    if let Ok(mut guard) = GROK.write() {
        // Another thread may have inserted the same pattern first. Keep the
        // winner so every caller shares one instance; the loser is leaked and
        // that is a bounded, one-off cost per pattern.
        return guard
            .get_or_insert_with(HashMap::new)
            .entry(key)
            .or_insert(compiled);
    }
    compiled
}

/// The pattern and its mapping, as one cache key.
///
/// A NUL separates the parts because no grok pattern or field path holds one,
/// so no two different inputs can collide on the same key.
fn cache_key(pattern: &str, extra: &[(&str, &str)]) -> String {
    let mut key = String::with_capacity(pattern.len() + extra.len() * 32);
    key.push_str(pattern);
    for (capture, path) in extra {
        key.push('\0');
        key.push_str(capture);
        key.push('\0');
        key.push_str(path);
    }
    key
}

/// Recognise the whole-pattern patterns a native parser covers.
///
/// Deliberately literal: it matches the exact pattern strings the transforms
/// use rather than parsing grok generally. A near-miss must fall through to
/// the regex, never guess.
fn native_form(pattern: &str) -> Option<Native> {
    // A lone `%{GREEDYDATA:field}` reads the same anchored or not: `.*` cannot
    // cross a newline, so the leftmost match at position 0 is the first line
    // either way. The other forms below are anchored-only, because an
    // unanchored address would match one ANYWHERE in the input.
    if let Some(field) = capture_of(
        pattern.trim_start_matches('^').trim_end_matches('$'),
        "GREEDYDATA",
    ) {
        return Some(Native::FirstLine {
            field: field.to_string(),
        });
    }

    let body = pattern.strip_prefix('^')?.strip_suffix('$')?;

    // `^%{DATA:field}$` is NOT this form, though it reads like it. `%{DATA}`
    // is the lazy `.*?`, and on a `\r\n` line ending the two disagree: greedy
    // keeps the `\r` in the capture and lazy stops before it. 19 call sites
    // spell it, and they stay on the regex rather than take a rule inferred
    // from one input.
    if let Some(field) = capture_of(body, "IPV4") {
        return Some(Native::Ipv4 {
            field: field.to_string(),
        });
    }

    // Split on the literal colon BETWEEN the two captures, not on the colon
    // inside `%{IPV4:field}` -- `split_once(':')` finds the wrong one.
    let (left, right) = body.split_once("}:%{")?;
    let addr = capture_of(&format!("{left}}}"), "IPV4")?.to_string();
    let port_spec = format!("%{{{right}");
    let port = capture_of(&port_spec, "PORT")?.to_string();
    Some(Native::Ipv4Port {
        addr,
        port,
        port_numeric: port_spec.ends_with(":long}")
            || port_spec.ends_with(":int}")
            || port_spec.ends_with(":float}")
            || port_spec.ends_with(":double}"),
    })
}

/// The field name in `%{TYPE:field}`, when `text` is exactly that and nothing
/// else.
fn capture_of<'a>(text: &'a str, kind: &str) -> Option<&'a str> {
    let inner = text
        .strip_prefix("%{")?
        .strip_suffix('}')?
        .strip_prefix(kind)?
        .strip_prefix(':')?;
    // A second `%{` would mean there is more in the pattern than this capture.
    if inner.contains('%') || inner.contains('}') {
        return None;
    }
    // `%{IPV4:src:ip}` -- the regex expansion drops the type suffix, so this
    // must drop it too or the two paths write different field paths.
    Some(inner.split_once(':').map_or(inner, |(field, _)| field))
}

impl CompiledGrok {
    /// Extract this pattern's captures from `input` into `event`.
    ///
    /// Returns whether the pattern matched. Uses the native parser when the
    /// pattern has one and falls back to the regex otherwise, so a caller does
    /// not need to know or care which ran.
    ///
    /// # Errors
    ///
    /// Propagates a failure to set a field on the event.
    pub fn extract_into(
        &self,
        input: impl AsRef<str>,
        event: &mut crate::Event,
    ) -> crate::Result<bool> {
        let input = input.as_ref();
        if let Some(native) = &self.native {
            return Self::extract_native(native, input, event);
        }

        match &self.regex {
            Pattern::Fast(re) => {
                let Some(caps) = re.captures(input) else {
                    return Ok(false);
                };
                for name in re.capture_names().flatten() {
                    if let Some(m) = caps.name(name) {
                        self.write_capture(name, m.as_str(), event)?;
                    }
                }
            }
            Pattern::Backtracking(re) => {
                // A backtracking match can fail outright -- a step limit, say.
                // That is the same answer for the caller as no match.
                let Ok(Some(caps)) = re.captures(input) else {
                    return Ok(false);
                };
                for name in re.capture_names().flatten() {
                    if let Some(m) = caps.name(name) {
                        // The backtracking engine reports a group that never
                        // participated as an empty match, where Elasticsearch's
                        // grok and the fast engine report nothing at all. Only a
                        // capture that cannot match empty is corrected, so a
                        // `%{DATA:x}` that genuinely matched nothing still
                        // writes the empty string Elasticsearch writes.
                        if m.as_str().is_empty() && self.never_empty.contains(name) {
                            continue;
                        }
                        self.write_capture(name, m.as_str(), event)?;
                    }
                }
            }
        }
        Ok(true)
    }

    /// Where this pattern's leftmost match begins in `input`, if it matches.
    ///
    /// Always asks the regex, never the native parser: the native forms are
    /// anchored, so the two agree on whether there is a match, and the regex is
    /// the one that can say WHERE.
    #[must_use]
    pub fn match_start(&self, input: &str) -> Option<usize> {
        match &self.regex {
            Pattern::Fast(re) => re.find(input).map(|m| m.start()),
            Pattern::Backtracking(re) => re.find(input).ok().flatten().map(|m| m.start()),
        }
    }

    /// Write one capture to the field it names, typed as Elastic types it.
    ///
    /// `set_resolved`, not `set`: a vendor payload that already spells the
    /// target as one dotted key keeps it, and splitting the path leaves the
    /// capture in a nested twin the rest of the pipeline never reads.
    fn write_capture(
        &self,
        name: &str,
        value: &str,
        event: &mut crate::Event,
    ) -> crate::Result<()> {
        let path = self.field_map.get(name).map_or(name, String::as_str);
        match self.capture_types.get(name) {
            // A `:float` capture is numeric too, and an i64 parse alone left
            // every fractional value a string -- lambda's duration_ms among them.
            Some(crate::codegen_api::CaptureType::Number) => {
                if let Ok(n) = value.parse::<i64>() {
                    event.set_resolved(path, n)?;
                } else if let Ok(f) = value.parse::<f64>() {
                    event.set_resolved(path, f)?;
                } else {
                    event.set_resolved(path, value)?;
                }
            }
            // Grok does not invent a value it cannot read: only the exact
            // text "true" or "false" becomes a boolean, and anything else
            // stays the string it was captured as.
            Some(crate::codegen_api::CaptureType::Boolean) => match value {
                "true" => event.set_resolved(path, true)?,
                "false" => event.set_resolved(path, false)?,
                _ => event.set_resolved(path, value)?,
            },
            None => event.set_resolved(path, value)?,
        }
        Ok(())
    }

    fn extract_native(
        native: &Native,
        input: &str,
        event: &mut crate::Event,
    ) -> crate::Result<bool> {
        match native {
            // `line_anchored` has already made this `(?m)^.*$`, so the
            // leftmost match is the first line and `.*` never fails -- an
            // empty first line is an empty capture, not a miss. A `\r` stays
            // in the capture because Rust's `.` matches it, which is what the
            // regex path does too.
            Native::FirstLine { field } => {
                let first = input.split_once('\n').map_or(input, |(head, _)| head);
                event.set(field, first)?;
                Ok(true)
            }
            Native::Ipv4 { field } => match dfe_parse::ip::parse_ipv4(input) {
                // Anchored: a trailing remainder means the whole input was not
                // an address, which is what `^...$` demands.
                Ok(("", addr)) => {
                    event.set(field, addr)?;
                    Ok(true)
                }
                _ => Ok(false),
            },
            Native::Ipv4Port {
                addr,
                port,
                port_numeric,
            } => {
                let Ok((rest, parsed_addr)) = dfe_parse::ip::parse_ipv4(input) else {
                    return Ok(false);
                };
                let Some(rest) = rest.strip_prefix(':') else {
                    return Ok(false);
                };
                // Not `parse_port`, which rejects 0: firewall logs carry port
                // 0 for ICMP, and failing here would lose the address too.
                let Ok((tail, parsed_port)) = dfe_parse::numeric::parse_nonneg_int(rest) else {
                    return Ok(false);
                };
                if !tail.is_empty() || parsed_port > 65535 {
                    return Ok(false);
                }
                event.set(addr, parsed_addr)?;
                if *port_numeric {
                    // Bounded above by the 65535 check, so this always fits.
                    event.set(port, i64::try_from(parsed_port).unwrap_or_default())?;
                } else {
                    event.set(port, parsed_port.to_string())?;
                }
                Ok(true)
            }
        }
    }
}

/// Extract the captures of whichever of `patterns` Elastic would have used.
///
/// A grok processor's several patterns are ONE regex there -- `(?:P1)|(?:P2)|`
/// ... searched unanchored -- so the winner is the pattern whose match starts
/// EARLIEST, and only a tie is broken by list order. Trying them in sequence
/// instead hands the win to a pattern that matches further along the line: an
/// ALB access log's classic-ELB prefix matches from column 5, while the ALB
/// pattern that also names the load-balancer type matches from column 0.
///
/// Returns whether anything matched.
///
/// # Errors
///
/// Propagates a failure to set a field on the event.
pub fn extract_first_match(
    patterns: &[&CompiledGrok],
    input: &str,
    event: &mut crate::Event,
) -> crate::Result<bool> {
    let Some((_, winner)) = best_match(patterns, input) else {
        return Ok(false);
    };
    winner.extract_into(input, event)
}

/// The pattern Elastic's single alternation would have matched, and its index.
///
/// Earliest start wins; only a tie is broken by list order.
fn best_match<'p>(patterns: &[&'p CompiledGrok], input: &str) -> Option<(usize, &'p CompiledGrok)> {
    let mut best: Option<(usize, usize, &'p CompiledGrok)> = None;
    for (index, pattern) in patterns.iter().enumerate() {
        let Some(start) = pattern.match_start(input) else {
            continue;
        };
        if best.is_none_or(|(best_start, _, _)| start < best_start) {
            best = Some((start, index, pattern));
        }
        // Nothing later can start earlier than the front of the line, so the
        // common case still costs one probe.
        if start == 0 {
            break;
        }
    }
    best.map(|(_, index, pattern)| (index, pattern))
}

/// [`extract_first_match`], recording WHICH pattern won.
///
/// Elastic's `trace_match: true` puts the winning index in
/// `_ingest._grok_match_index`, and a pipeline that asks for it BRANCHES on it:
/// `cisco_secure_email_gateway`'s AMP stream splits the same field with two
/// different `kv` field separators depending on whether pattern 1 was the one
/// that matched. Without the index both `kv` processors run on every document.
///
/// The index is ingest metadata and never reaches the output document, so the
/// generated transform clears it before returning.
///
/// # Errors
///
/// Propagates a failure to set a field on the event.
pub fn extract_first_match_traced(
    patterns: &[&CompiledGrok],
    input: &str,
    event: &mut crate::Event,
) -> crate::Result<bool> {
    let Some((index, winner)) = best_match(patterns, input) else {
        return Ok(false);
    };
    event.set("_ingest._grok_match_index", index)?;
    winner.extract_into(input, event)
}

/// The names of the captures in `expanded` whose body cannot match the empty
/// string.
///
/// `fancy_regex`'s `optimize_nested_repeats` folds an optional group over a
/// one-or-more body -- `(?P<x>\w+)?` -- into `(?P<x>\w*)`, which matches the
/// same text and captures differently: the group now PARTICIPATES with a
/// zero-width match where Elasticsearch's grok leaves it unset. Reading a body
/// that cannot match empty is how a zero-width report is recognised as that
/// fold rather than a real capture.
///
/// Conservative by construction: a body that will not compile on its own -- a
/// backreference to a group outside it, say -- is treated as able to match
/// empty, which leaves the value written as it is today.
fn never_empty_captures(expanded: &str) -> std::collections::HashSet<String> {
    named_group_bodies(expanded)
        .into_iter()
        .filter(|(_, body)| !can_match_empty(body))
        .map(|(name, _)| name)
        .collect()
}

/// Whether `body` matches the empty string, on whichever engine compiles it.
fn can_match_empty(body: &str) -> bool {
    let anchored = format!("^(?:{body})$");
    if let Ok(re) = Regex::new(&anchored) {
        return re.is_match("");
    }
    match fancy_regex::Regex::new(&anchored) {
        Ok(re) => re.is_match("").unwrap_or(true),
        Err(_) => true,
    }
}

/// Every `(?P<name>body)` and `(?<name>body)` in a regex, name and body apart.
///
/// A look-behind is spelt `(?<=` / `(?<!` and is not a capture, and a `[...]`
/// class holds parens that open nothing, so both are stepped over rather than
/// read.
fn named_group_bodies(expanded: &str) -> Vec<(String, &str)> {
    let bytes = expanded.as_bytes();
    let mut out = Vec::new();
    let mut at = 0;
    let mut in_class = false;
    while at < bytes.len() {
        match bytes[at] {
            b'\\' => {
                at += 2;
                continue;
            }
            b'[' if !in_class => in_class = true,
            b']' if in_class => in_class = false,
            b'(' if !in_class => {
                if let Some((name, body_at)) = group_name_at(expanded, at)
                    && let Some(close) = closing_paren(expanded, body_at)
                {
                    out.push((name, &expanded[body_at..close]));
                }
            }
            _ => {}
        }
        at += 1;
    }
    out
}

/// The capture name a `(` at `at` opens, and where its body starts.
fn group_name_at(expanded: &str, at: usize) -> Option<(String, usize)> {
    let after = expanded.get(at + 1..)?;
    let after = after
        .strip_prefix("?P<")
        .or_else(|| after.strip_prefix("?<"))?;
    if after.starts_with('=') || after.starts_with('!') {
        return None;
    }
    let close = after.find('>')?;
    let name = after[..close].to_string();
    Some((name, expanded.len() - after.len() + close + 1))
}

/// The offset of the `)` that closes the group whose body starts at `from`.
fn closing_paren(expanded: &str, from: usize) -> Option<usize> {
    let bytes = expanded.as_bytes();
    let mut depth = 1usize;
    let mut at = from;
    let mut in_class = false;
    while at < bytes.len() {
        match bytes[at] {
            b'\\' => {
                at += 2;
                continue;
            }
            b'[' if !in_class => in_class = true,
            b']' if in_class => in_class = false,
            b'(' if !in_class => depth += 1,
            b')' if !in_class => {
                depth -= 1;
                if depth == 0 {
                    return Some(at);
                }
            }
            _ => {}
        }
        at += 1;
    }
    None
}

/// Rewrite an expanded grok pattern from Elasticsearch's dialect into Rust's.
///
/// Four rules, each with its own function and its own reason below. They are
/// collected here so `grok_compiles.rs` can compile the pattern that actually
/// ships: it used to compile the raw expansion, which is not what any call site
/// runs, so a rule that broke a pattern would have gone unseen.
#[must_use]
pub fn to_rust_dialect(expanded: &str) -> String {
    let literal_angles = ruby_literal_angles(expanded);
    let dotall = ruby_dotall_flag(&literal_angles);
    let tolerant = tolerate_trailing_terminator(&dotall);
    line_anchored(&tolerant).into_owned()
}

/// Anchor `^` and `$` to LINES, which is what Elasticsearch's grok does.
///
/// Its grok is joni, not `java.util.regex`, and joni follows Ruby: `^` matches
/// after any newline and `$` before one. Rust anchors to the haystack unless
/// `(?m)` says otherwise, so every anchored pattern failed outright on a
/// multi-line value where joni matches one line of it. `rapid7_insightvm`'s
/// `^%{GREEDYDATA}$` is the clearest case -- a catch-all whose whole job is to
/// make the processor always succeed, matching nothing.
fn line_anchored(expanded: &str) -> Cow<'_, str> {
    if expanded.starts_with("(?m)") {
        return Cow::Borrowed(expanded);
    }
    Cow::Owned(format!("(?m){expanded}"))
}

/// `\<` and `\>` are the LITERAL brackets, which is what Ruby's syntax makes
/// them.
///
/// Same root as the anchor rule above: Elasticsearch's grok is joni under
/// `Syntax.RUBY`, and Ruby has no word-start or word-end escape, so a backslash
/// before a non-special character stands for that character. Rust's `regex`
/// added `\<` and `\>` as word boundaries in 1.9, which silently turns the
/// vendor's literal bracket into a zero-width assertion -- the pattern then
/// compiles and matches NOTHING. rabbitmq's `ERL_PID` is written
/// `\<%{INT}+\.%{INT}+\.%{INT}+\>` to match an Erlang pid like `<0.222.0>`, and
/// it failed on all 157 of its events; `aws_mq` ships the same definition.
fn ruby_literal_angles(expanded: &str) -> Cow<'_, str> {
    if !expanded.contains("\\<") && !expanded.contains("\\>") {
        return Cow::Borrowed(expanded);
    }
    let mut out = String::with_capacity(expanded.len());
    let mut chars = expanded.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            // The bracket alone -- the backslash was never an escape here.
            Some(angle @ ('<' | '>')) => out.push(angle),
            // A doubled backslash is a literal backslash, and whatever follows
            // it is NOT escaped, so it must not be examined as if it were.
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    Cow::Owned(out)
}

/// `(?m)` in a grok pattern is DOT-MATCHES-NEWLINE, which Rust spells `(?s)`.
///
/// Same root as the two rules above: joni under `Syntax.RUBY`, where `^` and `$`
/// are line anchors with no flag at all and `m` is Ruby's `/m` -- the flag that
/// lets `.` cross a newline. Rust reads `(?m)` as the line anchors it already
/// gets from `line_anchored`, so a vendor `(?m)` was arriving as a no-op and
/// every `%{GREEDYDATA}` written to span lines stopped at the first one.
///
/// `oracle`'s audit trail is the whole cost of it. Its first pattern reaches
/// across four lines to capture a wrapped SQL statement and hand the rest to
/// `audit`; without the dotall it cannot, so the SECOND pattern matched instead,
/// `audit` was never set, and the key-value processor that fills `DBID`,
/// `SESSIONID` and `USERHOST` was skipped on 27 of 29 events.
///
/// Grok only. An ingest processor's own regex -- `gsub`, and Painless `=~` --
/// is `java.util.regex`, where `(?m)` really is the line anchors and `(?s)` is
/// the dotall, so those patterns are already right and must not be touched.
fn ruby_dotall_flag(expanded: &str) -> Cow<'_, str> {
    if !expanded.contains("(?m)") {
        return Cow::Borrowed(expanded);
    }
    let mut out = String::with_capacity(expanded.len());
    let mut rest = expanded;
    // A character class is the one place `(?m)` is four ordinary characters, so
    // the scan has to know where it is rather than replacing on sight.
    let mut in_class = false;
    while let Some(c) = rest.chars().next() {
        let width = c.len_utf8();
        match c {
            '\\' => {
                let escape_width = rest
                    .chars()
                    .nth(1)
                    .map_or(width, |next| width + next.len_utf8());
                out.push_str(&rest[..escape_width]);
                rest = &rest[escape_width..];
                continue;
            }
            '[' if !in_class => in_class = true,
            ']' if in_class => in_class = false,
            '(' if !in_class && rest.starts_with("(?m)") => {
                out.push_str("(?s)");
                rest = &rest[4..];
                continue;
            }
            _ => {}
        }
        out.push(c);
        rest = &rest[width..];
    }
    Cow::Owned(out)
}

/// Let a trailing `$` match before a final line terminator, as Java's does.
///
/// Rust's `$` is the end of the haystack and Java's is the end but for one
/// terminator, so a vendor line delivered with its newline still attached fails
/// every anchored pattern. One `cisco_nexus` event arrives that way.
fn tolerate_trailing_terminator(expanded: &str) -> Cow<'_, str> {
    let anchored = expanded.ends_with('$') && !expanded.ends_with("\\$");
    if !anchored {
        return Cow::Borrowed(expanded);
    }
    let head = &expanded[..expanded.len() - 1];
    Cow::Owned(format!("{head}(?:\\r\\n|\\n|\\r)?$"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// Verbatim from `pipelines/checkpoint/firewall/default.yml`: the offset
    /// normaliser writes group 1 and a literal zero, which Rust would read as
    /// group 10 and drop.
    #[test]
    fn a_group_reference_stops_where_the_groups_do() {
        let compiled = grok("([+-])([0-9]):?([0-9]{2})");
        assert_eq!(compiled.regex.replace_all("-5:00", "$10$2:$3"), "-05:00");
        assert_eq!(compiled.regex.replace_all("+5:30", "$10$2:$3"), "+05:30");

        // A reference that is not ambiguous is passed through untouched.
        let pair = grok("([+-][0-9]{2})([0-9]{2})");
        assert_eq!(pair.regex.replace_all("-0500", "$1:$2"), "-05:00");
    }

    /// Verbatim from `pipelines/checkpoint/firewall/default.yml`: the RFC5424
    /// timestamp carries its offset behind an extra `-`, which the pattern
    /// eats so the offset lands in its own field rather than in the timestamp.
    #[test]
    fn a_doubled_dash_leaves_the_offset_in_its_own_field() {
        let compiled = grok(
            "%{SYSLOG5424PRI}%{NONNEGINT:syslog5424_ver} +(?:(?:(?P<syslog5424_ts>\
             (?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?))\
             (?:-?%{ISO8601_TIMEZONE:_temp_.tz})?)|-) +(?:%{SYSLOG5424PRINTASCII:syslog5424_host}|-)",
        );

        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            compiled
                .extract_into("<85>1 2023-01-13T10:10:16--5:00 172.16.2.9", &mut event)
                .expect("extraction"),
        );
        assert_eq!(event.get_str("syslog5424_ts"), Some("2023-01-13T10:10:16"));
        assert_eq!(event.get_str("_temp_.tz"), Some("-5:00"));
    }

    /// Java's `$` matches before a final line terminator and Rust's does not,
    /// so a vendor line still carrying its newline failed every anchored
    /// pattern. One `cisco_nexus` event arrives that way.
    #[test]
    fn an_anchored_pattern_tolerates_a_trailing_newline() {
        let compiled = grok("^<%{NUMBER:pri:long}>%{GREEDYDATA:body}$");

        for input in [
            "<186>switchname: a duplicate host",
            "<186>switchname: a duplicate host\n",
            "<186>switchname: a duplicate host\r\n",
        ] {
            let mut event = crate::Event::new(serde_json::json!({}));
            assert!(
                compiled
                    .extract_into(input, &mut event)
                    .expect("extraction"),
                "{input:?}"
            );
            assert_eq!(event.get_i64("pri"), Some(186), "{input:?}");
        }
    }

    /// The terminator is optional, not required, and an escaped `$` is a
    /// literal dollar rather than an anchor.
    #[test]
    fn tolerating_a_terminator_leaves_other_patterns_alone() {
        assert_eq!(
            tolerate_trailing_terminator("^a$"),
            "^a(?:\\r\\n|\\n|\\r)?$"
        );
        assert_eq!(tolerate_trailing_terminator("^a\\$"), "^a\\$");
        assert_eq!(tolerate_trailing_terminator("^a"), "^a");
    }

    /// `%{NONNEGINT}` is `\b(?:[0-9]+)\b`, so it cannot enter a digit run
    /// part-way. pfsense reads its vlan out of the interface name with
    /// `%{DATA}.%{NONNEGINT:...}`, and without the boundaries the lazy `%{DATA}`
    /// stops at the first digit it can reach -- the `1` of `igb1`.
    #[test]
    fn nonnegint_takes_a_whole_digit_run_or_none() {
        let compiled = grok("%{DATA}.%{NONNEGINT:observer.ingress.vlan.id}");

        for (interface, vlan) in [
            ("igb1.12", Some("12")),
            ("vtnet0.27", Some("27")),
            // No dotted suffix, so Elasticsearch writes no vlan at all.
            ("em0", None),
            ("lan", None),
        ] {
            let mut event = crate::Event::new(serde_json::json!({}));
            compiled
                .extract_into(interface, &mut event)
                .expect("extraction");
            assert_eq!(
                event.get_str("observer.ingress.vlan.id"),
                vlan,
                "{interface}"
            );
        }
    }

    /// A capture that did not participate writes nothing, on either engine.
    ///
    /// `fancy_regex` folds `(?P<x>\w+)?` into `(?P<x>\w*)`, which reports a
    /// zero-width match where Elasticsearch's grok leaves the field unset --
    /// three of pfsense's filterlog fields are optional and usually empty.
    /// `%{BASE16NUM}`'s look-behind is what puts this pattern on that engine.
    #[test]
    fn an_optional_capture_that_did_not_participate_writes_nothing() {
        let compiled = grok(
            "%{BASE16NUM:pfsense.ip.tos},%{WORD:pfsense.ip.ecn}?,\
             %{NONNEGINT:pfsense.ip.ttl:long}",
        );
        assert!(
            compiled.regex.fast().is_none(),
            "the correction only applies on the backtracking engine, so the \
             pattern has to reach it for this test to mean anything"
        );

        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            compiled
                .extract_into("0x0,,63", &mut event)
                .expect("extraction"),
        );
        assert_eq!(event.get_str("pfsense.ip.tos"), Some("0x0"));
        assert_eq!(event.get_i64("pfsense.ip.ttl"), Some(63));
        assert!(!event.has("pfsense.ip.ecn"));

        // The same capture, participating, still writes what it read.
        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            compiled
                .extract_into("0x0,ce,63", &mut event)
                .expect("extraction"),
        );
        assert_eq!(event.get_str("pfsense.ip.ecn"), Some("ce"));
    }

    /// A capture that CAN match empty keeps its empty string, which is what
    /// Elasticsearch writes for one.
    #[test]
    fn a_capture_that_matched_nothing_still_writes_the_empty_string() {
        let compiled = grok("%{BASE16NUM:tos},%{DATA:rest}$");
        assert!(compiled.regex.fast().is_none());

        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            compiled
                .extract_into("0x0,", &mut event)
                .expect("extraction")
        );
        assert_eq!(event.get_str("rest"), Some(""));
    }

    /// Verbatim from `pipelines/pfsense/log/firewall.yml`: an optional sequence
    /// number in front of an unnamed one. The boundaries are what stop the
    /// named capture taking all but the last digit and leaving that digit to
    /// the unnamed one -- Elasticsearch declines the optional capture and reads
    /// the whole run into the unnamed group, so the field is never written.
    #[test]
    fn an_optional_number_does_not_split_the_run_behind_it() {
        let compiled = grok(
            "%{NONNEGINT:pfsense.tcp.length:long},%{WORD:pfsense.tcp.flags}?,\
             %{NONNEGINT:pfsense.tcp.seq:long}?:?%{NONNEGINT},",
        );

        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            compiled
                .extract_into("0,S,1891286705,", &mut event)
                .expect("extraction"),
        );
        assert_eq!(event.get_i64("pfsense.tcp.length"), Some(0));
        assert_eq!(event.get_str("pfsense.tcp.flags"), Some("S"));
        assert!(!event.has("pfsense.tcp.seq"));
    }

    /// Ruby has no word-start escape, so rabbitmq's `\<...\>` is a pair of
    /// literal brackets round an Erlang pid.
    #[test]
    fn an_escaped_angle_bracket_is_the_bracket_itself() {
        assert_eq!(ruby_literal_angles("\\<a\\>"), "<a>");
        // A doubled backslash is a literal backslash, and the bracket after it
        // was never escaped -- rewriting it would change what the regex means.
        assert_eq!(ruby_literal_angles("\\\\<a"), "\\\\<a");
        // Nothing to do, and nothing allocated.
        assert!(matches!(
            ruby_literal_angles("^%{WORD:a}$"),
            std::borrow::Cow::Borrowed(_)
        ));

        let compiled = grok(
            "%{TIMESTAMP_ISO8601:timestamp} \\[%{WORD:level}\\] \
             (?P<pid>(?:\\<%{INT}+\\.%{INT}+\\.%{INT}+\\>)) (?P<msg>(?:(.|\n)*))",
        );
        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            compiled
                .extract_into(
                    "2021-05-13 09:00:00.000000+00:00 [info] <0.222.0> Server startup complete",
                    &mut event,
                )
                .expect("the pid pattern compiles")
        );
        assert_eq!(event.get_str("pid"), Some("<0.222.0>"));
        assert_eq!(event.get_str("msg"), Some("Server startup complete"));
    }

    /// joni anchors to lines, so a catch-all matches the first line of a
    /// multi-line value rather than failing the whole processor.
    #[test]
    fn an_anchored_pattern_matches_a_line_of_a_multiline_value() {
        assert_eq!(line_anchored("^a$"), "(?m)^a$");
        // Already declared by the pipeline; not declared twice.
        assert_eq!(line_anchored("(?m)^a$"), "(?m)^a$");

        let compiled = grok("^%{GREEDYDATA:first}$");
        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            compiled
                .extract_into("Following entries\n\nttyS0\n", &mut event)
                .expect("the catch-all compiles")
        );
        assert_eq!(event.get_str("first"), Some("Following entries"));
    }

    /// joni reads `m` as Ruby does, so the flag the vendor writes to cross a
    /// newline has to be respelt before Rust reads it as the anchors it already
    /// has.
    #[test]
    fn the_vendors_m_flag_is_the_dotall_one() {
        assert_eq!(ruby_dotall_flag("a(?m).*"), "a(?s).*");
        assert_eq!(ruby_dotall_flag("a.*"), "a.*");
        // Four ordinary characters inside a class, and an escaped bracket does
        // not open one.
        assert_eq!(ruby_dotall_flag("[(?m)]"), "[(?m)]");
        assert_eq!(ruby_dotall_flag("\\[(?m)x"), "\\[(?s)x");

        // oracle's own first pattern, cut to the part that needs the flag: the
        // SQL statement wraps, and the key-values after it are what the source
        // is actually scored on.
        let compiled = grok(
            "LENGTH : '%{GREEDYDATA:length}'\\nACTION :\\[\\d+\\] (?m)%{GREEDYDATA:action}\
             (DATABASE USER):\\S+ '(?P<db_user>(?:[^']+))'\\n(?m)%{GREEDYDATA:audit}",
        );
        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            compiled
                .extract_into(
                    "LENGTH : '392'\nACTION :[151] 'select /*+ opt_param('a',\n   'false') */'\n\
                     DATABASE USER:[1] '/'\nDBID:[10] '2824230686'\nSESSIONID:[1] '0'",
                    &mut event,
                )
                .expect("the oracle pattern compiles")
        );
        assert_eq!(event.get_str("length"), Some("392"));
        assert_eq!(event.get_str("db_user"), Some("/"));
        assert_eq!(
            event.get_str("audit"),
            Some("DBID:[10] '2824230686'\nSESSIONID:[1] '0'")
        );
    }

    /// The other half of the same rule: kafka's header grok DOES take the whole
    /// multi-line event, and the re-capture after it is what puts `message`
    /// back to one line. Both patterns are needed to see the answer, which is
    /// why the dotall on `JAVALOGMESSAGE` hid the missing one for so long.
    #[test]
    fn a_stack_trace_stays_out_of_the_message() {
        let event_text = "[2020-01-20 01:32:00,705] [ERROR] [controller-event-thread] \
             [state.change.logger] - [Controller id=1 epoch=25] Controller 1 epoch 25 failed\n\
             kafka.common.StateChangeFailedException: Failed to elect leader\n    \
             at kafka.controller.PartitionStateMachine.doElect(PartitionStateMachine.scala:390)";

        let header = grok(
            "(?m)\\[%{TIMESTAMP_ISO8601:kafka.log.timestamp}\\] \\[%{LOGLEVEL:log.level} ?\\] \
             \\[%{NOTSPACE:kafka.log.thread}\\] \\[%{NOTSPACE:kafka.log.class}\\] \\- \
             %{GREEDYDATA:message}",
        );
        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            header
                .extract_into(event_text, &mut event)
                .expect("the header pattern compiles")
        );
        assert_eq!(
            event.get_str("kafka.log.class"),
            Some("state.change.logger")
        );
        // The vendor's `(?m)`: the header takes everything, trace included.
        assert!(
            event
                .get_str("message")
                .is_some_and(|m| m.contains("StateChangeFailedException"))
        );

        let component = grok("\\[(?P<component>(?:[^\\]]*))\\][,:.]? +%{JAVALOGMESSAGE:message}");
        let held = event.get_string("message").expect("the header set message");
        assert!(
            component
                .extract_into(&held, &mut event)
                .expect("the component pattern compiles")
        );
        assert_eq!(event.get_str("component"), Some("Controller id=1 epoch=25"));
        assert_eq!(
            event.get_str("message"),
            Some("Controller 1 epoch 25 failed")
        );
    }

    /// The pattern that made this necessary: an ALB access log begins with the
    /// load-balancer type, so the classic-ELB pattern matches from column 5
    /// while the ALB pattern matches from column 0 -- and Elastic, matching one
    /// alternation, takes the earlier start rather than the earlier pattern.
    #[test]
    fn the_earliest_match_wins_over_the_earliest_pattern() {
        let classic = grok("%{TIMESTAMP_ISO8601:ts} %{NOTSPACE:name}");
        let v2 = grok("%{WORD:kind} %{TIMESTAMP_ISO8601:ts} %{NOTSPACE:name}");
        let mut event = crate::Event::new(serde_json::json!({}));

        assert!(
            extract_first_match(
                &[classic, v2],
                "http 2018-07-02T22:23:00.186641Z app/my-loadbalancer",
                &mut event,
            )
            .expect("extraction")
        );
        assert_eq!(event.get_str("kind"), Some("http"));
    }

    /// Order still decides a tie, and a line no pattern matches says so.
    #[test]
    fn a_tie_goes_to_the_earlier_pattern() {
        let first = grok("%{WORD:first_hit}");
        let second = grok("%{NOTSPACE:second_hit}");
        let mut event = crate::Event::new(serde_json::json!({}));

        assert!(extract_first_match(&[first, second], "alpha", &mut event).expect("extraction"));
        assert_eq!(event.get_str("first_hit"), Some("alpha"));
        assert!(event.get_str("second_hit").is_none());

        let mut empty = crate::Event::new(serde_json::json!({}));
        assert!(!extract_first_match(&[grok("^%{IP:ip}$")], "not-an-ip", &mut empty).unwrap());
    }

    #[test]
    fn the_same_pattern_returns_the_same_instance() {
        let first = grok("%{USER:user.name}");
        let second = grok("%{USER:user.name}");
        assert!(
            std::ptr::eq(first, second),
            "a repeated pattern must be compiled once, not twice"
        );
    }

    #[test]
    fn distinct_patterns_do_not_share() {
        let a = grok("%{WORD:a.b}");
        let b = grok("%{WORD:c.d}");
        assert!(!std::ptr::eq(a, b));
    }

    #[test]
    fn the_field_map_restores_dotted_paths() {
        let compiled = grok("%{USER:user.name}");
        assert_eq!(
            compiled.field_map.get("user_name").map(String::as_str),
            Some("user.name"),
            "the dotted path must survive the capture-name rewrite"
        );
    }

    /// Elastic's `:long` is a type, not part of the field name. Dropping it
    /// left every numeric capture a string.
    #[test]
    fn a_typed_capture_lands_as_a_number() {
        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            grok("^%{NUMBER:network.bytes:long} %{WORD:event.action}$")
                .extract_into("2282 blocked", &mut event)
                .expect("extraction")
        );

        assert_eq!(event.get_i64("network.bytes"), Some(2282));
        assert_eq!(event.get_str("event.action"), Some("blocked"));
    }

    /// Elastic's `:boolean` is a type too, and dropping it the same way left
    /// `coredns.log.dnssec_ok` a string that `get_bool` never matched --
    /// coredns's own grok is `%{WORD:coredns.log.dnssec_ok:boolean}`. Grok
    /// does not invent a value it cannot read, so a word that is not "true"
    /// or "false" must stay the string it was captured as.
    #[test]
    fn a_boolean_typed_capture_lands_as_a_json_boolean() {
        let compiled = grok("^%{WORD:coredns.log.dnssec_ok:boolean} %{WORD:event.action}$");

        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            compiled
                .extract_into("true blocked", &mut event)
                .expect("extraction")
        );
        assert_eq!(event.get_bool("coredns.log.dnssec_ok"), Some(true));

        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            compiled
                .extract_into("false blocked", &mut event)
                .expect("extraction")
        );
        assert_eq!(event.get_bool("coredns.log.dnssec_ok"), Some(false));

        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            compiled
                .extract_into("maybe blocked", &mut event)
                .expect("extraction")
        );
        assert!(
            event.get_bool("coredns.log.dnssec_ok").is_none(),
            "a word that is not true/false must not become a boolean"
        );
        assert_eq!(event.get_str("coredns.log.dnssec_ok"), Some("maybe"));
    }

    /// `%{SYSLOG5424PRI}` is written without a field name because Elastic's
    /// own definition carries the destination -- `syslog5424_pri` in the
    /// legacy registry, which is the one that applies unless the processor
    /// asks for `ecs_compatibility: v1`. The generator supplies the ECS
    /// destination as an explicit mapping where it does.
    #[test]
    fn a_bare_pri_still_captures_its_priority() {
        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            grok("%{SYSLOG5424PRI}%{GREEDYDATA:rest}$")
                .extract_into("<188>date=2020-04-23", &mut event)
                .expect("extraction")
        );

        assert_eq!(event.get_i64("syslog5424_pri"), Some(188));

        let mut ecs = crate::Event::new(serde_json::json!({}));
        assert!(
            grok_mapped(
                "%{SYSLOG5424PRI}%{GREEDYDATA:rest}$",
                &[("syslog5424_pri", "log.syslog.priority")],
            )
            .extract_into("<188>date=2020-04-23", &mut ecs)
            .expect("extraction")
        );

        assert_eq!(ecs.get_i64("log.syslog.priority"), Some(188));
    }

    /// `cisco_ios` wraps its whole syslog preamble in an optional group, so both
    /// the priority and the hostname are captured from inside one.
    #[test]
    fn a_capture_inside_an_optional_group_is_written() {
        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            grok("^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?%{WORD:event.action}$")
                .extract_into("<190>3132517: blocked", &mut event)
                .expect("extraction")
        );

        assert_eq!(event.get_i64("log.syslog.priority"), Some(190));
        assert_eq!(event.get_str("event.action"), Some("blocked"));
    }

    /// The same group, not participating. An absent optional capture must
    /// leave the field unset rather than writing an empty string.
    #[test]
    fn an_optional_group_that_does_not_participate_writes_nothing() {
        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            grok("^(?:<%{NONNEGINT:log.syslog.priority:long}>)?%{WORD:event.action}$")
                .extract_into("blocked", &mut event)
                .expect("extraction")
        );

        assert!(!event.has("log.syslog.priority"));
        assert_eq!(event.get_str("event.action"), Some("blocked"));
    }

    /// The `cisco_ios` header pattern, exactly as the generator emits it, against a
    /// line from its own fixtures.
    #[test]
    fn the_cisco_ios_header_pattern_captures_its_preamble() {
        let mut event = crate::Event::new(serde_json::json!({}));
        let compiled = grok_mapped(
            r"^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?%{SYSLOGTIMESTAMP} %{IP} (?:(?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?)): )?(?:%{NUMBER:cisco.ios.sequence}: )?(?:(?P<cisco_ios_uptime>(?:(?:\d{1,4}:\d{2}:\d{2}|(?:(\d+)y)?(?:(\d+)w)?(?:(\d+)d)?(?:(\d+)h)?(?:(\d+)m)?(?:(\d+)s)?)))|(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\d{1,2}|[+-]\d{2}:\d{2})?)))?)): %{GREEDYDATA:_temp_.message}$",
            &[
                ("log_syslog_hostname", "log.syslog.hostname"),
                ("cisco_ios_uptime", "cisco.ios.uptime"),
                ("_temp__cisco_timestamp", "_temp_.cisco_timestamp"),
                ("_temp__tz", "_temp_.tz"),
            ],
        );

        let matched = compiled
            .extract_into(
                "<190>3132517: Jul 13 08:23:43 192.168.100.2 sw01: 3132779: Jul 14 2023 08:23:43.398 UTC: %FOO-6-BAR: Test header format",
                &mut event,
            )
            .expect("extraction");

        assert!(matched, "the header pattern did not match its own fixture");
        assert_eq!(event.get_i64("log.syslog.priority"), Some(190));
        assert_eq!(event.get_str("log.syslog.hostname"), Some("sw01"));
        assert_eq!(event.get_str("cisco.ios.sequence"), Some("3132779"));
    }

    /// `%{IP}` is `(?:%{IPV6}|%{IPV4})` in Elastic and was v4-only here, so
    /// every pattern reading an address matched half of them.
    #[test]
    fn ip_matches_v6_as_well_as_v4() {
        for address in [
            "2a02:cf40::",
            "::1",
            "fe80::16a2:a0ff:fe09:f6a8",
            "2001:db8:0:0:1:0:0:1",
            "81.2.69.192",
        ] {
            let mut event = crate::Event::new(serde_json::json!({}));
            assert!(
                grok("^%{IP:source.ip}$")
                    .extract_into(address, &mut event)
                    .expect("extraction"),
                "{address} did not match %{{IP}}"
            );
            assert_eq!(event.get_str("source.ip"), Some(address));
        }
    }

    /// The header pattern that reads the hostname slot with `%{IP}`, against
    /// the fixture line where that hostname IS a v6 address. Without the v6
    /// branch the whole pattern failed, the sequence after the address went
    /// with it, and `event.sequence` silently fell back to the message count.
    #[test]
    fn a_v6_hostname_does_not_swallow_the_sequence_after_it() {
        let mut event = crate::Event::new(serde_json::json!({}));
        let compiled = grok_mapped(
            r"^(?:<%{NONNEGINT:log.syslog.priority:long}>(?:%{NONNEGINT:cisco.ios.message_count})?(?:: )?)?(?:(?:%{IP}|(?P<log_syslog_hostname>(?:[0-9a-zA-Z][.0-9a-zA-Z_-]{0,253}[0-9a-zA-Z]?)))(?:: \*%{DATA}:|:?)? )?(?:%{NUMBER:cisco.ios.sequence}: )?(?:(?P<cisco_ios_uptime>(?:(?:\d{1,4}:\d{2}:\d{2}|(?:(\d+)y)?(?:(\d+)w)?(?:(\d+)d)?(?:(\d+)h)?(?:(\d+)m)?(?:(\d+)s)?)))|(?P<_temp__timestamp>(?:[*]?(?P<_temp__cisco_timestamp>(?:(%{CISCOTIMESTAMP})|(%{YEAR} %{MONTH} %{MONTHDAY} %{TIME})))(?: (?P<_temp__tz>(?:[a-zA-Z]{1,7}([+-]\d{1,2}|[+-]\d{2}:\d{2})?)))?))): %{GREEDYDATA:_temp_.message}$",
            &[
                ("log_syslog_hostname", "log.syslog.hostname"),
                ("cisco_ios_uptime", "cisco.ios.uptime"),
                ("_temp__timestamp", "_temp_.timestamp"),
                ("_temp__cisco_timestamp", "_temp_.cisco_timestamp"),
                ("_temp__tz", "_temp_.tz"),
            ],
        );

        let matched = compiled
            .extract_into(
                "<190>3132783: 2a02:cf40::: 3132779: Jul 14 2023 08:23:43.398 UTC: %FOO-6-BAR: Test header format",
                &mut event,
            )
            .expect("extraction");

        assert!(matched, "the header pattern did not match its own fixture");
        // Untyped in the pattern, so it lands as a string here; the pipeline's
        // own `convert` makes it a long further down.
        assert_eq!(event.get_str("cisco.ios.message_count"), Some("3132783"));
        assert_eq!(event.get_str("cisco.ios.sequence"), Some("3132779"));
        assert!(!event.has("cisco.ios.uptime"));
        assert_eq!(
            event.get_str("_temp_.cisco_timestamp"),
            Some("Jul 14 2023 08:23:43.398")
        );
    }

    /// Elastic's `QUOTEDSTRING` takes any of the three quote characters.
    /// Reading only the double form left `cisco_meraki`'s `ssid=''` unmatched,
    /// and the grok that failed carried the whole key-value line with it.
    #[test]
    fn a_quoted_string_takes_any_of_the_three_quotes() {
        for (input, expected) in [
            (r#"ssid="home""#, r#""home""#),
            ("ssid='home'", "'home'"),
            ("ssid=`home`", "`home`"),
            ("ssid=''", "''"),
        ] {
            let mut event = crate::Event::new(serde_json::json!({}));
            assert!(
                grok("^ssid=%{QS:network.name}$")
                    .extract_into(input, &mut event)
                    .expect("extraction"),
                "{input} did not match"
            );
            assert_eq!(event.get_str("network.name"), Some(expected), "{input}");
        }
    }

    #[test]
    fn a_plain_pattern_is_cached_too() {
        let first = regex(r"\d{6}$");
        let second = regex(r"\d{6}$");
        assert!(std::ptr::eq(first, second));
        assert!(first.is_match("abc123456"));
    }

    /// A malformed pattern must not take the process down -- one broken
    /// processor is not worth a stalled partition.
    #[test]
    fn a_malformed_pattern_matches_nothing_instead_of_panicking() {
        let compiled = regex("(unclosed");
        assert!(!compiled.is_match("unclosed"));
        assert!(!compiled.is_match(""));
    }

    // -- native path ------------------------------------------------------

    /// Run a pattern BOTH ways over the same input and require identical
    /// results. This is what makes the native path safe to enable: it is only
    /// ever an optimisation if it cannot disagree with the regex.
    fn assert_paths_agree(pattern: &str, input: &str) {
        let compiled = grok(pattern);
        assert!(
            compiled.native.is_some(),
            "{pattern} was expected to take the native path"
        );

        let mut native_event = crate::Event::new(serde_json::json!({}));
        let native_matched = compiled
            .extract_into(input, &mut native_event)
            .expect("native extraction");

        // The same work with the native path forced off.
        let regex_only = CompiledGrok {
            regex: compiled.regex.clone(),
            field_map: compiled.field_map.clone(),
            capture_types: compiled.capture_types.clone(),
            native: None,
            never_empty: compiled.never_empty.clone(),
        };
        let mut regex_event = crate::Event::new(serde_json::json!({}));
        let regex_matched = regex_only
            .extract_into(input, &mut regex_event)
            .expect("regex extraction");

        assert_eq!(
            native_matched, regex_matched,
            "{pattern} on {input:?}: native and regex disagree on whether it matched"
        );
        assert_eq!(
            native_event.as_value(),
            regex_event.as_value(),
            "{pattern} on {input:?}: native and regex produced different fields"
        );
    }

    /// The widest form in the tree, so the two paths have to agree on every
    /// awkward input rather than the happy one. The newline cases are the
    /// whole risk, and they are the opposite of the obvious reading:
    /// `line_anchored` makes this `(?m)^.*$`, so a multi-line input MATCHES
    /// and yields line one, an empty first line is an empty capture rather
    /// than a miss, and a `\r` stays in because Rust's `.` matches it.
    #[test]
    fn native_and_regex_agree_on_whole_input() {
        for input in [
            "a plain message",
            "",
            " leading and trailing ",
            "with:colons and %{braces}",
            "unicode -- \u{5bff}\u{53f8} and an emoji \u{1f600}",
            "tab\there",
            // The newline cases, which decide the fast path.
            "two\nlines",
            "trailing\n",
            "\nleading",
            "\n",
            "\r\n",
            "carriage\rreturn",
        ] {
            assert_paths_agree("^%{GREEDYDATA:event.original}$", input);
            // The unanchored spelling, which `.*` reads the same way: it
            // cannot cross a newline, so the leftmost match is line one.
            assert_paths_agree("%{GREEDYDATA:event.original}", input);
        }
    }

    /// `^%{DATA:field}$` looks like the same form and is not, so the reason it
    /// stays on the regex is pinned rather than left to be rediscovered.
    #[test]
    fn the_lazy_catch_all_is_not_the_first_line_form() {
        assert!(
            grok("^%{DATA:event.original}$").native.is_none(),
            "the lazy catch-all took a native path"
        );

        // Where they part: greedy keeps the carriage return, lazy stops before
        // it.
        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            grok("^%{DATA:event.original}$")
                .extract_into("\r\n", &mut event)
                .expect("the lazy catch-all compiles")
        );
        assert_eq!(event.get_str("event.original"), Some(""));

        let mut event = crate::Event::new(serde_json::json!({}));
        assert!(
            grok("^%{GREEDYDATA:event.original}$")
                .extract_into("\r\n", &mut event)
                .expect("the greedy catch-all compiles")
        );
        assert_eq!(event.get_str("event.original"), Some("\r"));
    }

    #[test]
    fn native_and_regex_agree_on_addresses() {
        for input in [
            "192.168.1.100",
            "10.0.0.1",
            "255.255.255.255",
            "0.0.0.0",
            // Non-matches matter as much as matches.
            "not-an-ip",
            "",
            "192.168.1",
            "192.168.1.100.5",
            " 192.168.1.1",
            "192.168.1.1 ",
            "::1",
        ] {
            assert_paths_agree("^%{IPV4:source.ip}$", input);
        }
    }

    #[test]
    fn native_and_regex_agree_on_address_and_port() {
        for input in [
            "10.0.0.7:443",
            "192.168.1.1:1",
            "8.8.8.8:65535",
            // Non-matches.
            "10.0.0.7:",
            "10.0.0.7",
            // Port 0 is valid input here -- see the parser comment.
            "10.0.0.7:0",
            "10.0.0.7:443:8080",
            "host:443",
            "",
        ] {
            assert_paths_agree("^%{IPV4:_temp.src_ip}:%{PORT:sport}$", input);
        }
    }

    /// Where the two paths deliberately disagree, pinned so it cannot drift.
    ///
    /// Our grok expansions do not range-check: `%{IPV4}` is `\d{1,3}` per
    /// octet and `%{PORT}` is bare digits, so both accept values that are not
    /// valid. `dfe-parse` validates. Elastic's own patterns validate too, so
    /// the native path is the more faithful of the two -- but it is still a
    /// behaviour change, and the fixture match rates are what show it is safe.
    #[test]
    fn the_native_path_rejects_values_the_loose_regex_accepts() {
        for (pattern, input) in [
            ("^%{IPV4:source.ip}$", "192.168.1.256"),
            ("^%{IPV4:_temp.src_ip}:%{PORT:sport}$", "10.0.0.7:65536"),
        ] {
            let compiled = grok(pattern);
            let mut event = crate::Event::new(serde_json::json!({}));

            assert!(
                !compiled
                    .extract_into(input, &mut event)
                    .expect("extraction"),
                "{input} is out of range and the native parser must reject it"
            );
            assert!(
                compiled.regex.is_match(input),
                "{input}: the regex still accepts it -- this is the divergence, not a stale test"
            );
        }
    }

    /// Grok's type suffix (`%{IPV4:src:ip}`) is stripped by the regex
    /// expansion, so the native path must strip it too or the two write
    /// different field paths for the same pattern.
    #[test]
    fn native_and_regex_agree_on_type_suffixed_captures() {
        assert_paths_agree("^%{IPV4:source.ip:ip}$", "192.168.1.100");
        assert_paths_agree(
            "^%{IPV4:_temp.src_ip:ip}:%{PORT:sport:long}$",
            "10.0.0.7:443",
        );
    }

    /// A pattern with literal text around the captures must NOT be claimed by
    /// the native path -- the regex engine is the right tool for those.
    #[test]
    fn patterns_with_literals_stay_on_the_regex() {
        for pattern in [
            "^src=%{IPV4:source.ip}$",
            "%{IPV4:source.ip}",
            "^%{IPV4:a} %{PORT:b}$",
            "^%{WORD:a}$",
            "^%{IPV4:a}:%{WORD:b}$",
        ] {
            assert!(
                grok(pattern).native.is_none(),
                "{pattern} must not be claimed by the native path"
            );
        }
    }

    /// The point of the whole module: matching must not rebuild the regex.
    #[test]
    fn repeated_lookups_are_the_same_compiled_regex() {
        let pattern = "%{IPV4:source.ip}";
        let addresses: Vec<&'static CompiledGrok> = (0..1000).map(|_| grok(pattern)).collect();
        let first = addresses[0];
        assert!(
            addresses.iter().all(|c| std::ptr::eq(*c, first)),
            "1000 lookups must all return the one compiled instance"
        );
    }
}
