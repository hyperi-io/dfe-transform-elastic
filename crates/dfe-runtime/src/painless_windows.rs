// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The Windows event-log script shapes: sysmon, powershell and the
//! `m365_defender` copies of the same helpers.
//!
//! Four shapes recur across those pipelines. `commandLineToArgv` is the Go
//! implementation of Windows argument splitting transliterated into Painless,
//! carried by eight pipelines with per-copy field pairs read off the tail.
//! The file-info split, the hash-map lowercasing and the registry parser are
//! sysmon's, the last driven by its params hive table.

use std::sync::LazyLock;

use serde_json::{Map, Value, json};

use crate::event::Event;

// ---------------------------------------------------------------------------
// commandLineToArgv
// ---------------------------------------------------------------------------

/// One `ctx.<base>.args = commandLineToArgv(<source>)` site in a script, with
/// what its surrounding statements also write.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArgvSite {
    /// The command line read, e.g. `process.command_line`.
    source: String,
    /// The base the args land under, e.g. `process`.
    base: String,
    /// Whether `<base>.args_count` is written too.
    count: bool,
    /// Whether `<base>.executable` is set from `args[0]` when non-empty.
    executable: bool,
}

/// The parse of a `commandLineToArgv` script: its sites, and whether this
/// copy skips empty arguments (the `m365_defender` variant does, sysmon's
/// keeps them).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArgvScript {
    sites: Vec<ArgvSite>,
    skip_empty: bool,
}

impl ArgvScript {
    /// Read the sites off the script's tail, or `None` when none parse.
    pub(crate) fn parse(script: &str) -> Option<Self> {
        let mut sites = Vec::new();
        for (at, _) in script.match_indices(".args = commandLineToArgv(") {
            let base = ctx_path_before(&script[..at])?;
            let after = &script[at + ".args = commandLineToArgv(".len()..];
            let (argument, _) = after.split_once(')')?;
            let source = match argument.trim().strip_prefix("ctx.") {
                Some(path) => clean(path),
                None => local_binding(script, argument.trim())?,
            };
            let count = script.contains(&format!("ctx.{base}.args_count = "));
            let executable = script
                .contains(&format!("ctx.{base}.executable = ctx.{base}.args[0]"));
            sites.push(ArgvSite {
                source,
                base,
                count,
                executable,
            });
        }
        if sites.is_empty() {
            return None;
        }
        Some(Self {
            sites,
            skip_empty: script.contains(".arg == ''"),
        })
    }
}

/// The dotted ctx path ending at `text`'s end.
fn ctx_path_before(text: &str) -> Option<String> {
    let at = text.rfind("ctx.")?;
    let path = &text[at + 4..];
    path.chars()
        .all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '?')
        .then(|| clean(path))
}

/// A path with Painless's null-safe `?` markers dropped.
fn clean(path: &str) -> String {
    path.replace('?', "")
}

/// The ctx path a `def <local> = ctx.<path>;` binds, `?`s dropped.
fn local_binding(script: &str, local: &str) -> Option<String> {
    for opener in [format!("def {local} = ctx."), format!("String {local} = ctx.")] {
        if let Some(at) = script.find(&opener) {
            let rest = &script[at + opener.len()..];
            let end = rest.find(';')?;
            return Some(clean(rest[..end].trim()));
        }
    }
    None
}

/// Split a command line the way Windows does -- the Go implementation the
/// pipelines transliterated, "Prior to 2008" double-double-quote rule and
/// all.
fn command_line_to_argv(cmd: &str, skip_empty: bool) -> Vec<String> {
    let chars: Vec<char> = cmd.chars().collect();
    let mut args = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == ' ' || chars[i] == '\t' {
            i += 1;
            continue;
        }
        let (arg, next) = read_next_arg(&chars, i);
        i = next;
        if skip_empty && arg.is_empty() {
            continue;
        }
        args.push(arg);
    }
    args
}

/// One argument from `chars[i..]`, and where the next one starts.
fn read_next_arg(chars: &[char], mut i: usize) -> (String, usize) {
    let mut out = String::new();
    let mut inquote = false;
    let mut nslash = 0usize;
    let backslashes = |out: &mut String, n: usize| {
        for _ in 0..n {
            out.push('\\');
        }
    };
    while i < chars.len() {
        let c = chars[i];
        if (c == ' ' || c == '\t') && !inquote {
            backslashes(&mut out, nslash);
            return (out, i + 1);
        }
        if c == '"' {
            backslashes(&mut out, nslash / 2);
            if nslash.is_multiple_of(2) {
                if inquote && chars.get(i + 1) == Some(&'"') {
                    out.push(c);
                    i += 1;
                }
                inquote = !inquote;
            } else {
                out.push(c);
            }
            nslash = 0;
            i += 1;
            continue;
        }
        if c == '\\' {
            nslash += 1;
            i += 1;
            continue;
        }
        backslashes(&mut out, nslash);
        nslash = 0;
        out.push(c);
        i += 1;
    }
    backslashes(&mut out, nslash);
    (out, i)
}

/// Run a parsed argv script: each site splits its command line into args,
/// with the count and the executable where that copy writes them.
pub(crate) fn run_argv_script(event: &mut Event, script: &ArgvScript) -> bool {
    for site in &script.sites {
        let Some(cmd) = event.get_str(&site.source).map(str::to_string) else {
            continue;
        };
        if cmd.is_empty() {
            continue;
        }
        let args = command_line_to_argv(&cmd, script.skip_empty);
        if site.count {
            let _ = event.set(
                &format!("{}.args_count", site.base),
                json!(i64::try_from(args.len()).unwrap_or(i64::MAX)),
            );
        }
        if site.executable
            && let Some(first) = args.first()
        {
            let _ = event.set(&format!("{}.executable", site.base), json!(first));
        }
        let _ = event.set(
            &format!("{}.args", site.base),
            Value::Array(args.into_iter().map(Value::String).collect()),
        );
    }
    true
}

// ---------------------------------------------------------------------------
// File info: name, directory and extension off one path field
// ---------------------------------------------------------------------------

/// Split `<source>` at its last backslash into `file.name` and
/// `file.directory`, with `file.extension` from the last dot of the WHOLE
/// path -- which is the script's own reading, dotted directories included.
pub(crate) fn run_file_info(event: &mut Event, source: &str) -> bool {
    let Some(path) = event.get_str(source).map(str::to_string) else {
        return true;
    };
    let Some(idx) = path.rfind('\\') else {
        return true;
    };
    let _ = event.set("file.name", json!(&path[idx + 1..]));
    let _ = event.set("file.directory", json!(&path[..idx]));
    if let Some(ext_idx) = path.rfind('.') {
        let _ = event.set("file.extension", json!(&path[ext_idx + 1..]));
    }
    true
}

/// The path a file-info script reads, off its `def path = ctx.<p>;` binding.
pub(crate) fn file_info_source(script: &str) -> Option<String> {
    local_binding(script, "path")
}

// ---------------------------------------------------------------------------
// Hash-map lowercasing
// ---------------------------------------------------------------------------

/// Lowercase every key and value of the map at `source`, dropping empty and
/// all-zero hashes, and REPLACE `related` with the collected hash list --
/// which is what sysmon's script does, existing keys and all.
pub(crate) fn run_hash_lowercase(event: &mut Event, source: &str) -> bool {
    let Some(Value::Object(entries)) = event.get(source).cloned() else {
        return true;
    };

    let mut hashes = Map::new();
    let mut related: Vec<Value> = Vec::new();
    for (key, value) in &entries {
        let value = crate::painless_helpers::painless_to_string(value).to_lowercase();
        if value.is_empty() || value.bytes().all(|b| b == b'0') {
            continue;
        }
        hashes.insert(key.to_lowercase(), Value::String(value.clone()));
        related.push(Value::String(value));
    }

    let _ = event.set(source, Value::Object(hashes));
    if !related.is_empty() {
        let _ = event.set("related", json!({ "hash": related }));
    }
    true
}

/// The map a hash-lowercase script walks, off its `.entrySet()` loop.
pub(crate) fn hash_lowercase_source(script: &str) -> Option<String> {
    let at = script.find(".entrySet())")?;
    ctx_path_before(&script[..at])
}

// ---------------------------------------------------------------------------
// Registry fields
// ---------------------------------------------------------------------------

static QWORD: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?i)^QWORD \((0x[0-9A-F]{8})-(0x[0-9A-F]{8})\)$")
        .expect("a literal pattern compiles")
});
static DWORD: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?i)^DWORD \((0x[0-9A-F]{8})\)$").expect("a literal pattern compiles")
});

/// Set `registry.*` from sysmon's `TargetObject` and `Details`, the hive
/// abbreviated through the params table.
///
/// The QWORD value is `(high << 8) + low`, exactly as the script computes it
/// -- the pipeline's own arithmetic is the parity target, not what the shift
/// arguably should have been.
pub(crate) fn run_registry(event: &mut Event, params: &Map<String, Value>) -> bool {
    let Some(path) = event
        .get_str("winlog.event_data.TargetObject")
        .map(str::to_string)
    else {
        return true;
    };

    let mut registry = Map::new();
    registry.insert("path".into(), Value::String(path.clone()));

    let tokens: Vec<&str> = path.split('\\').collect();
    if let Some(hive) = tokens
        .first()
        .and_then(|root| params.get(*root))
        .and_then(Value::as_str)
    {
        registry.insert("hive".into(), Value::String(hive.to_string()));
        if tokens.len() > 1 {
            registry.insert("key".into(), Value::String(tokens[1..].join("\\")));
        }
    }
    if let Some(value) = tokens.last() {
        registry.insert("value".into(), Value::String((*value).to_string()));
    }

    if let Some(data) = event
        .get_str("winlog.event_data.Details")
        .filter(|d| !d.is_empty())
        .map(str::to_string)
    {
        let parsed = |text: &str| i64::from_str_radix(&text[2..], 16).ok();
        let entry = if let Some(caps) = QWORD.captures(&data) {
            parsed(&caps[1])
                .zip(parsed(&caps[2]))
                .map(|(high, low)| ((high << 8) + low, "SZ_QWORD"))
                .map(|(value, kind)| (value.to_string(), kind))
        } else if let Some(caps) = DWORD.captures(&data) {
            parsed(&caps[1]).map(|value| (value.to_string(), "SZ_DWORD"))
        } else if data == "Binary Data" {
            Some((data.clone(), "REG_BINARY"))
        } else {
            Some((data.clone(), "REG_SZ"))
        };
        if let Some((value, kind)) = entry {
            registry.insert("data".into(), json!({ "strings": [value], "type": kind }));
        }
    }

    let _ = event.set("registry", Value::Object(registry));
    true
}

// ---------------------------------------------------------------------------
// The security pipeline's message-table decode
// ---------------------------------------------------------------------------

/// The msobjs.dll message-table script: `%%`-prefixed codes translated
/// through `params.descriptions`, the access mask OR-ed together and fanned
/// out through `params.AccessMaskDescriptions`' hex keys.
///
/// The tables come from params, so a vendor table update flows through
/// regeneration; the block structure is the script's and fixed.
#[allow(clippy::too_many_lines)] // One block per event_data field, as the script has them.
pub(crate) fn run_message_table(event: &mut Event, params: &Map<String, Value>) -> bool {
    let descriptions = params.get("descriptions").and_then(Value::as_object);
    let describe = |code: &str| -> Option<String> {
        descriptions
            .and_then(|table| table.get(code))
            .and_then(Value::as_str)
            .map(str::to_string)
    };

    // FailureReason: the description, or the bare code when the table has
    // no row -- always written when the field is present.
    if let Some(reason) = event
        .get_str("winlog.event_data.FailureReason")
        .map(str::to_string)
    {
        let code = reason.replace("%%", "");
        let desc = describe(&code).unwrap_or(code);
        let _ = event.set("winlog.logon.failure.reason", json!(desc));
    }

    // AuditPolicyChanges: a comma-separated string or an array, each code
    // described where the table can.
    if let Some(value) = event.get("winlog.event_data.AuditPolicyChanges").cloned() {
        let elems: Vec<String> = match &value {
            Value::String(s) => s.split(',').map(str::to_string).collect(),
            Value::Array(items) => items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect(),
            _ => Vec::new(),
        };
        let results: Vec<Value> = elems
            .iter()
            .map(|elem| {
                let code = elem.replace("%%", "").trim().to_string();
                Value::String(describe(&code).unwrap_or(code))
            })
            .collect();
        if !results.is_empty() {
            let _ = event.set(
                "winlog.event_data.AuditPolicyChangesDescription",
                Value::Array(results),
            );
        }
    }

    // AccessList: the field is REWRITTEN to the cleaned codes, and the
    // descriptions land beside it.
    if let Some(value) = event.get("winlog.event_data.AccessList").cloned() {
        let elems = whitespace_or_array(&value);
        let mut codes: Vec<Value> = Vec::new();
        let mut results: Vec<Value> = Vec::new();
        for elem in &elems {
            let code = elem.replace("%%", "").trim().to_string();
            if !code.is_empty() {
                codes.push(Value::String(code.clone()));
            }
            results.push(Value::String(describe(&code).unwrap_or(code)));
        }
        if !codes.is_empty() {
            let _ = event.set("winlog.event_data.AccessList", Value::Array(codes));
        }
        if !results.is_empty() {
            let _ = event.set(
                "winlog.event_data.AccessListDescription",
                Value::Array(results),
            );
        }
    }

    // Direction and LayerName: described only where the table has the code.
    for (source, target) in [
        ("winlog.event_data.Direction", "winlog.event_data.DirectionDescription"),
        ("winlog.event_data.LayerName", "winlog.event_data.LayerNameDescription"),
    ] {
        if let Some(text) = event.get_str(source).map(str::to_string) {
            let code = text.replace("%%", "").trim().to_string();
            if let Some(desc) = describe(&code) {
                let _ = event.set(target, json!(desc));
            }
        }
    }

    // AccessMask: each element described or kept, the numeric codes OR-ed
    // into one mask, and every set bit fanned out through the hex-keyed
    // AccessMaskDescriptions table.
    if let Some(value) = event.get("winlog.event_data.AccessMask").cloned() {
        let elems = whitespace_or_array(&value);
        let reversed = params.get("reversed_descriptions").and_then(Value::as_object);
        let mut list: Vec<Value> = Vec::new();
        let mut mask: i64 = 0;
        for elem in &elems {
            if elem.is_empty() {
                continue;
            }
            let mut code = elem.replace("%%", "").trim().to_string();
            if let Some(desc) = describe(&code) {
                list.push(Value::String(desc));
            } else {
                list.push(Value::String(code.clone()));
                if let Some(mapped) = reversed
                    .and_then(|table| table.get(&code))
                    .and_then(Value::as_array)
                    .and_then(|row| row.first())
                    .and_then(Value::as_str)
                {
                    code = mapped.to_string();
                }
            }
            if let Some(number) = java_long_decode(&code) {
                mask |= number;
            }
        }
        if !list.is_empty() {
            let _ = event.set("winlog.event_data.AccessMask", Value::Array(list));
        }

        let flags = params.get("AccessMaskDescriptions").and_then(Value::as_object);
        let mut descs: Vec<Value> = Vec::new();
        for bit in 0..32u32 {
            let flag = 1i64 << bit;
            if mask & flag == flag
                && let Some(desc) = flags
                    .and_then(|table| table.get(&format!("0x{flag:08X}")))
                    .and_then(Value::as_str)
            {
                descs.push(Value::String(desc.to_string()));
            }
        }
        if !descs.is_empty() {
            let _ = event.set(
                "winlog.event_data.AccessMaskDescription",
                Value::Array(descs),
            );
        }
    }

    true
}

/// A whitespace-split string, or the array's string members -- the two
/// shapes the script's own `split` helper accepts.
fn whitespace_or_array(value: &Value) -> Vec<String> {
    match value {
        Value::String(s) => s.split_whitespace().map(str::to_string).collect(),
        Value::Array(items) => items
            .iter()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect(),
        _ => Vec::new(),
    }
}

/// `Long.decode`: `0x`/`0X`/`#` hex, leading-`0` octal, else decimal.
fn java_long_decode(text: &str) -> Option<i64> {
    let (negative, rest) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let value = if let Some(hex) = rest.strip_prefix("0x").or_else(|| rest.strip_prefix("0X")) {
        i64::from_str_radix(hex, 16).ok()?
    } else if let Some(hex) = rest.strip_prefix('#') {
        i64::from_str_radix(hex, 16).ok()?
    } else if rest.len() > 1 && rest.starts_with('0') {
        i64::from_str_radix(&rest[1..], 8).ok()?
    } else {
        rest.parse::<i64>().ok()?
    };
    Some(if negative { -value } else { value })
}

// ---------------------------------------------------------------------------
// The security pipeline's user-copy scripts
// ---------------------------------------------------------------------------

/// The `!["4624", ...].contains(ctx.event.code)` gate's code list.
pub(crate) fn event_code_list(script: &str) -> Option<Vec<String>> {
    let at = script.find("![\"")?;
    let after = &script[at + 1..];
    let (list, rest) = after.split_once(']')?;
    if !rest.trim_start().starts_with(".contains(ctx.event.code)") {
        return None;
    }
    let codes: Vec<String> = list
        .split(',')
        .filter_map(|piece| {
            let piece = piece.trim().trim_start_matches('[');
            piece.strip_prefix('"')?.strip_suffix('"').map(str::to_string)
        })
        .collect();
    (!codes.is_empty()).then_some(codes)
}

/// Does the event's code pass a script's own gate?
fn code_gated(event: &Event, codes: &[String]) -> bool {
    event
        .get_str("event.code")
        .is_some_and(|code| codes.iter().any(|c| c == code))
}

/// The security pipeline's "Copy Target User": the target SID to `user.id`
/// or -- when a user is already named -- `user.target.id`, the username's
/// pre-`@` half likewise plus `related.user`, and the domain the same way.
pub(crate) fn run_copy_target_user(event: &mut Event, codes: &[String]) -> bool {
    if !code_gated(event, codes) {
        return true;
    }

    let target_id = event
        .get_str("winlog.event_data.TargetUserSid")
        .or_else(|| event.get_str("winlog.event_data.TargetSid"))
        .map(str::to_string);
    if let Some(id) = target_id {
        let field = if event.has_value("user.id") {
            "user.target.id"
        } else {
            "user.id"
        };
        let _ = event.set(field, json!(id));
    }

    if let Some(name) = event
        .get_str("winlog.event_data.TargetUserName")
        .map(str::to_string)
    {
        let first = name.split('@').next().unwrap_or(&name).to_string();
        let field = if event.has_value("user.name") {
            "user.target.name"
        } else {
            "user.name"
        };
        let _ = event.set(field, json!(first.clone()));
        let _ = event.append_unique("related.user", Value::String(first));
    }

    if let Some(domain) = event
        .get_str("winlog.event_data.TargetDomainName")
        .map(str::to_string)
    {
        let field = if event.has_value("user.domain") {
            "user.target.domain"
        } else {
            "user.domain"
        };
        let _ = event.set(field, json!(domain));
    }
    true
}

/// The security pipeline's "Copy Subject User from Event Data": the subject
/// SID, name and domain OVERWRITE `user.*`, the name also joining
/// `related.user`.
pub(crate) fn run_copy_subject_user(event: &mut Event, codes: &[String]) -> bool {
    if !code_gated(event, codes) {
        return true;
    }

    if let Some(id) = event
        .get_str("winlog.event_data.SubjectUserSid")
        .map(str::to_string)
    {
        let _ = event.set("user.id", json!(id));
    }
    if let Some(name) = event
        .get_str("winlog.event_data.SubjectUserName")
        .map(str::to_string)
    {
        let _ = event.set("user.name", json!(name.clone()));
        let _ = event.append_unique("related.user", Value::String(name));
    }
    if let Some(domain) = event
        .get_str("winlog.event_data.SubjectDomainName")
        .map(str::to_string)
    {
        let _ = event.set("user.domain", json!(domain));
    }
    true
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    /// The sysmon copy: guarded pairs for the process and its parent, empty
    /// arguments kept.
    const SYSMON_ARGV: &str = r#"
        def readNextArg(String cmd) { }
        def commandLineToArgv(String cmd) { }
        def cmd = ctx.process?.command_line;
        if (cmd != null && cmd != "") {
          ctx.process.args = commandLineToArgv(cmd);
          ctx.process.args_count = ctx.process.args.length;
        }
        def parentCmd = ctx.process?.parent?.command_line;
        if (parentCmd != null && parentCmd != "") {
          ctx.process.parent.args = commandLineToArgv(parentCmd);
          ctx.process.parent.args_count = ctx.process.parent.args.length;
        }
    "#;

    /// The `m365_defender` copy: unguarded, empties skipped, executable set.
    const M365_ARGV: &str = r"
        def readNextArg(String line, int offset) {
          if (next.arg == '') { }
        }
        def commandLineToArgv(String line) { }
        ctx.process.args = commandLineToArgv(ctx.process.command_line);
        ctx.process.args_count = ctx.process.args.length;
        if (ctx.process.args.length > 0) {
            ctx.process.executable = ctx.process.args[0];
        }
    ";

    #[test]
    fn the_sysmon_copy_parses_both_pairs() {
        let script = ArgvScript::parse(SYSMON_ARGV).unwrap();
        assert_eq!(script.sites.len(), 2);
        assert!(!script.skip_empty);
        assert_eq!(script.sites[0].source, "process.command_line");
        assert_eq!(script.sites[0].base, "process");
        assert!(script.sites[0].count);
        assert!(!script.sites[0].executable);
        assert_eq!(script.sites[1].source, "process.parent.command_line");
        assert_eq!(script.sites[1].base, "process.parent");
    }

    /// The daviddeley cases the Go implementation documents, plus the
    /// double-double-quote rule.
    #[test]
    fn argv_splits_like_windows() {
        let cases: &[(&str, &[&str])] = &[
            (r#"C:\a\b.exe -c "d e" f"#, &[r"C:\a\b.exe", "-c", "d e", "f"]),
            // The pre-2008 rule: `""` inside quotes is a literal quote AND
            // leaves the quoted run, so the `c` sits bare and the space after
            // it splits.
            (r#"a "b ""c"" d""#, &["a", r#"b "c"#, "d"]),
            (r#"a\\\"b"#, &[r#"a\"b"#]),
            (r#"a\\\\"b c" d"#, &[r"a\\b c", "d"]),
            ("  spaced\targ  ", &["spaced", "arg"]),
        ];
        for (input, expected) in cases {
            assert_eq!(
                command_line_to_argv(input, false),
                expected.to_vec(),
                "input: {input}"
            );
        }
    }

    #[test]
    fn the_m365_copy_sets_the_executable_and_skips_empties() {
        let script = ArgvScript::parse(M365_ARGV).unwrap();
        assert!(script.skip_empty);
        assert!(script.sites[0].executable);

        let mut event = Event::new(json!({
            "process": { "command_line": r#""C:\Program Files\a.exe" "" run"# }
        }));
        assert!(run_argv_script(&mut event, &script));
        assert_eq!(
            event.get("process.args"),
            Some(&json!([r"C:\Program Files\a.exe", "run"])),
            "the empty argument is skipped"
        );
        assert_eq!(event.get("process.args_count"), Some(&json!(2)));
        assert_eq!(
            event.get("process.executable"),
            Some(&json!(r"C:\Program Files\a.exe"))
        );
    }

    #[test]
    fn the_sysmon_copy_keeps_empties_and_skips_a_missing_parent() {
        let script = ArgvScript::parse(SYSMON_ARGV).unwrap();
        let mut event = Event::new(json!({
            "process": { "command_line": r#"a "" b"# }
        }));
        assert!(run_argv_script(&mut event, &script));
        assert_eq!(event.get("process.args"), Some(&json!(["a", "", "b"])));
        assert_eq!(event.get("process.args_count"), Some(&json!(3)));
        assert_eq!(event.get("process.parent.args"), None);
    }

    #[test]
    fn file_info_splits_name_directory_and_whole_path_extension() {
        let mut event = Event::new(json!({
            "file": { "path": r"C:\Users\x.y\Desktop\report" }
        }));
        assert!(run_file_info(&mut event, "file.path"));
        assert_eq!(event.get("file.name"), Some(&json!("report")));
        assert_eq!(event.get("file.directory"), Some(&json!(r"C:\Users\x.y\Desktop")));
        // The extension comes off the WHOLE path, dotted directory included.
        assert_eq!(event.get("file.extension"), Some(&json!(r"y\Desktop\report")));
    }

    #[test]
    fn file_info_leaves_a_slashless_path_alone() {
        let mut event = Event::new(json!({ "file": { "path": "pipe" } }));
        assert!(run_file_info(&mut event, "file.path"));
        assert_eq!(event.get("file.name"), None);
    }

    #[test]
    fn hash_lowercase_drops_empty_and_zero_hashes_and_replaces_related() {
        let mut event = Event::new(json!({
            "_temp": { "hashes": {
                "MD5": "ABC123",
                "SHA256": "0000",
                "IMPHASH": "",
                "SHA1": "DEF456",
            }},
            "related": { "user": ["kept?"] },
        }));
        assert!(run_hash_lowercase(&mut event, "_temp.hashes"));
        assert_eq!(
            event.get("_temp.hashes"),
            Some(&json!({ "md5": "abc123", "sha1": "def456" }))
        );
        // The script REPLACES related wholesale.
        assert_eq!(
            event.get("related"),
            Some(&json!({ "hash": ["abc123", "def456"] }))
        );
    }

    fn hive_params() -> Map<String, Value> {
        json!({ "HKLM": "HKLM", "HKEY_LOCAL_MACHINE": "HKLM" })
            .as_object()
            .cloned()
            .unwrap()
    }

    #[test]
    fn registry_splits_the_target_object_through_the_hive_table() {
        let mut event = Event::new(json!({
            "winlog": { "event_data": {
                "TargetObject": r"HKLM\System\CurrentControlSet\Services\Tcpip",
                "Details": "DWORD (0x000000FF)",
            }},
        }));
        assert!(run_registry(&mut event, &hive_params()));
        assert_eq!(
            event.get("registry"),
            Some(&json!({
                "path": r"HKLM\System\CurrentControlSet\Services\Tcpip",
                "hive": "HKLM",
                "key": r"System\CurrentControlSet\Services\Tcpip",
                "value": "Tcpip",
                "data": { "strings": ["255"], "type": "SZ_DWORD" },
            }))
        );
    }

    /// The QWORD value is the script's own `(high << 8) + low`.
    #[test]
    fn registry_reads_a_qword_with_the_scripts_own_arithmetic() {
        let mut event = Event::new(json!({
            "winlog": { "event_data": {
                "TargetObject": r"HKLM\X",
                "Details": "QWORD (0x00000001-0x00000002)",
            }},
        }));
        assert!(run_registry(&mut event, &hive_params()));
        assert_eq!(
            event.get("registry.data"),
            Some(&json!({ "strings": ["258"], "type": "SZ_QWORD" }))
        );
    }

    #[test]
    fn registry_keeps_plain_details_as_reg_sz() {
        let mut event = Event::new(json!({
            "winlog": { "event_data": {
                "TargetObject": r"UNKNOWN\X",
                "Details": "some text",
            }},
        }));
        assert!(run_registry(&mut event, &hive_params()));
        let registry = event.get("registry").unwrap();
        assert_eq!(registry.pointer("/hive"), None, "unknown root has no hive");
        assert_eq!(registry.pointer("/value"), Some(&json!("X")));
        assert_eq!(
            registry.pointer("/data"),
            Some(&json!({ "strings": ["some text"], "type": "REG_SZ" }))
        );
    }
}
