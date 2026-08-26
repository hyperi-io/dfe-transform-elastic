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
use crate::painless_params::clean_path;

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

/// Read `<local> = ctx.<a>.substring(ctx.<b>.length())` and where the local
/// finally lands, as a [`KnownShape::PrefixTail`].
fn parse_prefix_tail(script: &str) -> Option<KnownShape> {
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

    Some(KnownShape::PrefixTail {
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
/// ctx.<target> = x;` as a [`KnownShape::PrependToArray`].
fn parse_prepend_to_array(script: &str) -> Option<KnownShape> {
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

    Some(KnownShape::PrependToArray {
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
    let (local, base) = local_bound_to_ctx(script)?;

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
fn parse_split_token_field(script: &str) -> Option<KnownShape> {
    use crate::painless_params::clean_path;

    let (local, source) = local_bound_to_ctx(script)?;
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
    // this shape, and claiming it would write a wrong array.
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

    Some(KnownShape::SplitTokenField {
        source,
        separator: separator.chars().next()?,
        parse_int: script.contains("Integer.parseInt("),
        target,
        head,
    })
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

/// Split one field on its token in place, optionally keeping only the pieces
/// that parse as 32-bit integers -- `Integer.parseInt`'s range, since Painless
/// skips the ones that throw.
fn run_split_token_field(
    event: &mut Event,
    source: &str,
    separator: char,
    parse_int: bool,
    target: &str,
    head: Option<char>,
) -> bool {
    if let Some(text) = event.get_str(source).map(str::to_string) {
        // Java's split drops trailing empty pieces.
        let mut pieces: Vec<&str> = text.split(separator).collect();
        while pieces.last() == Some(&"") {
            pieces.pop();
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
        let _ = event.set(target, Value::Array(values));
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
fn run_max_by_contains(event: &mut Event, shape: &MaxByContains) -> bool {
    let values = match event.get(&shape.source) {
        Some(Value::Array(values)) => values.clone(),
        Some(value @ Value::String(_)) => vec![value.clone()],
        _ => return true,
    };

    let mut highest = 0i64;
    for value in values.iter().filter_map(Value::as_str) {
        let folded = value.to_lowercase();
        if let Some((_, score)) = shape
            .arms
            .iter()
            .find(|(literals, _)| literals.iter().any(|lit| folded.contains(lit)))
            && *score > highest
        {
            highest = *score;
        }
    }

    if highest > 0 {
        let _ = event.set(&shape.target, json!(highest));
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
/// [`KnownShape::RemoveListValue`].
fn parse_remove_list_value(script: &str) -> Option<KnownShape> {
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
    (!field.is_empty()).then_some(KnownShape::RemoveListValue { field, value })
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
fn parse_mail_related(script: &str) -> Option<KnownShape> {
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

    (!addresses.is_empty() || !names.is_empty()).then_some(KnownShape::MailRelated(Box::new(
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
fn parse_category_type_ladder(script: &str) -> Option<KnownShape> {
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

    (!arms.is_empty()).then_some(KnownShape::CategoryTypeLadder(arms))
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
/// Both halves must be present: the loop alone could be any of a dozen shapes,
/// and the assignment alone says nothing about what is being copied.
fn parse_snake_key_map_copy(script: &str) -> Option<KnownShape> {
    use crate::painless_params::clean_path;

    let loop_at = script.find(" : ctx.")?;
    let (source, _) = script[loop_at + " : ctx.".len()..].split_once(".entrySet()")?;

    let assignment = script
        .lines()
        .rev()
        .map(|line| line.trim().trim_end_matches(';'))
        .find(|line| line.starts_with("ctx.") && line.contains(" = "))?;
    let (target, _) = assignment.split_once(" = ")?;

    Some(KnownShape::SnakeKeyMapCopy {
        source: clean_path(source),
        target: clean_path(&target["ctx.".len()..]),
    })
}

/// Read the angle-strip helper's call sites: `ctx.<p> = <name>(ctx.<p>);`
/// scalars, and the loop rebuilding a list through the same helper.
fn parse_strip_angle_pairs(script: &str) -> Option<KnownShape> {
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
        .then_some(KnownShape::StripAnglePairs { scalars, lists })
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
        if let Some(value) = resource.remove("ARN") {
            resource.insert("arn".into(), value);
        }
        if let Some(value) = resource.remove("accountId") {
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
/// ctx.<t>.put(<v>.<k>, <v>.<val>); }` as a [`KnownShape::NameValueFold`].
fn parse_name_value_fold(script: &str) -> Option<KnownShape> {
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

    Some(KnownShape::NameValueFold {
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
/// [`KnownShape::NameValueFold`]'s, so only the reading differs: the map is
/// subscripted rather than `put` to, and the element is reached by index
/// rather than by a loop variable.
fn parse_indexed_name_value_fold(script: &str) -> Option<KnownShape> {
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
        .then_some(KnownShape::NameValueFold {
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

/// Read the outcome-from-tags shape: the action plus a dot prefixes the tag
/// whose value decides success or failure.
fn parse_outcome_from_tags(script: &str) -> Option<KnownShape> {
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

    Some(KnownShape::OutcomeFromTags {
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
/// [`KnownShape::TokenCount`].
fn parse_token_count(script: &str) -> Option<KnownShape> {
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

    Some(KnownShape::TokenCount {
        source,
        separator,
        target,
    })
}

/// Read `def <l> = []; <l>.add(ctx.<s>); ctx.<t> = <l>;` as a
/// [`KnownShape::WrapValueInList`].
fn parse_wrap_value_in_list(script: &str) -> Option<KnownShape> {
    use crate::painless_params::clean_path;

    let add_at = script.find(".add(ctx.")?;
    let after = &script[add_at + ".add(ctx.".len()..];
    let (source, _) = after.split_once(')')?;

    let local = script[..add_at]
        .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()?;
    let store = format!(" = {local};");
    let store_at = script.rfind(&store)?;
    let before = &script[..store_at];
    let target = clean_path(&before[before.rfind("ctx.")? + 4..]);

    Some(KnownShape::WrapValueInList {
        source: clean_path(source),
        target,
    })
}

/// Read `ctx.<t> = ctx.<a>[ctx.<a>.length-1];` as a
/// [`KnownShape::LastElement`].
fn parse_last_element(script: &str) -> Option<KnownShape> {
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
    Some(KnownShape::LastElement { array, target })
}

/// Read `ctx.<a>[i] = ctx.<a>[i].trim()` as a [`KnownShape::TrimListInPlace`].
fn parse_trim_list(script: &str) -> Option<KnownShape> {
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
    (read == written && !read.is_empty()).then_some(KnownShape::TrimListInPlace(read))
}

/// Read cloudfront's localhost edge case as a [`KnownShape::StartsWithAppend`]:
/// a constant appended to a list when a member of another list has a prefix.
fn parse_starts_with_append(script: &str) -> Option<KnownShape> {
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

    (!source.is_empty() && !target.is_empty()).then_some(KnownShape::StartsWithAppend {
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

/// Read cloudfront's `url.full` assembly as a [`KnownShape::ConcatParts`].
///
/// `def full = ""` then a run of guarded `full += ...`, and the result assigned
/// to a ctx field when it came to something.
fn parse_concat_parts(script: &str) -> Option<KnownShape> {
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
    (!target.is_empty()).then_some(KnownShape::ConcatParts(ConcatScript { target, clauses }))
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

/// Read elb's `tlsv12` split as a [`KnownShape::TlsVersionSplit`].
///
/// s3access spells the same thing `ctx.<p>.toLowerCase().splitOnToken("v")`,
/// and taking everything before the split read `toLowerCase()` as a segment of
/// the path -- so the field resolved to nothing and eight events lost both
/// `tls.version` and `tls.version_protocol`. The run lowercases the protocol
/// half itself, so dropping the call changes nothing else.
fn parse_tls_version_split(script: &str) -> Option<KnownShape> {
    use crate::painless_params::clean_path;

    let at = script.find(".splitOnToken(")?;
    let before = &script[..at];
    let path = clean_path(&before[before.rfind("ctx.")? + 4..]);
    let source = path
        .split('.')
        .take_while(|segment| !segment.contains('('))
        .collect::<Vec<_>>()
        .join(".");
    (!source.is_empty()).then_some(KnownShape::TlsVersionSplit { source })
}

/// `ctx.<t> = ctx.<s>.decodeBase64();` as a [`KnownShape::DecodeBase64`].
fn parse_decode_base64(script: &str) -> Option<KnownShape> {
    use crate::painless_params::clean_path;

    let at = script.find(".decodeBase64()")?;
    let before = &script[..at];
    let source = clean_path(&before[before.rfind("ctx.")? + 4..]);
    let (lhs, _) = before.split_once('=')?;
    let target = clean_path(lhs.trim().strip_prefix("ctx.")?);
    (!target.is_empty() && !source.is_empty())
        .then_some(KnownShape::DecodeBase64 { source, target })
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
}

impl DropPolicy {
    /// The reading of a script's predicate, taken off its own text.
    #[must_use]
    pub fn read(script: &str) -> Self {
        Self {
            nulls: script.contains("== null"),
            empty_strings: script.contains("== ''") || script.contains("== \"\""),
            // `.isEmpty()` is the third spelling and aws/waf's only one, so
            // every empty list and map it ships survived: four per event.
            empty_collections: script.contains(".size() == 0")
                || script.contains(".length == 0")
                || script.contains(".isEmpty()"),
            // A list is pruned by a `removeIf` whose receiver is the list
            // itself. `map.values().removeIf` prunes the MAP, which is
            // cisco_asa's only one; gcp's recursive helper walks the map
            // through an iterator and spells `removeIf` once, for the list.
            prune_lists: script
                .split(".removeIf(")
                .take(script.matches(".removeIf(").count())
                .any(|head| !head.trim_end().ends_with("values()")),
            sentinels: predicate_sentinels(script),
        }
    }
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
    let inner = event.as_value_mut();
    drop_value(inner, policy);
}

fn drop_value(value: &mut Value, policy: &DropPolicy) -> bool {
    match value {
        Value::Null => policy.nulls,
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

/// The `convertToSnakeCase` helper the integrations copy verbatim between
/// packages -- `sentinel_one`'s `unified_alert` and `entityanalytics_entra_id`'s
/// user and device all carry the same twenty lines.
///
/// Two details separate it from [`keys_to_snake_case`], and both are visible
/// in the fixtures. A key holding an `@` is DROPPED rather than renamed, which
/// is how Microsoft's `@odata.*` metadata stays out of the document. And the
/// underscore goes in wherever the previous character was not itself
/// uppercase, digits included, so `cve2021Id` becomes `cve2021_id`.
///
/// Returns a new value; the script assigns the result rather than mutating.
#[must_use]
pub fn camel_map_to_snake(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = serde_json::Map::with_capacity(map.len());
            for (key, inner) in map {
                if key.contains('@') {
                    continue;
                }
                out.insert(
                    to_snake_case(key, SnakeRule::AfterNonUpper),
                    camel_map_to_snake(inner),
                );
            }
            Value::Object(out)
        }
        Value::Array(items) => Value::Array(items.iter().map(camel_map_to_snake).collect()),
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
/// Returns false when nothing parses.
fn try_literal_value_map(event: &mut Event, script: &str) -> bool {
    let Some((local, source)) = local_bound_to_ctx(script) else {
        return false;
    };
    let Some(subject) = event.get_as_string(&source) else {
        // The field is absent, which every one of these scripts is gated on.
        return false;
    };

    let mut matched = false;
    for block in script.split("if (").skip(1) {
        let Some((guard, body)) = block.split_once(") {") else {
            continue;
        };
        let Some(literal) = equality_literal(guard, &local) else {
            continue;
        };
        if literal != subject {
            continue;
        }
        let Some((target, value)) = branch_write(body) else {
            continue;
        };
        let _ = event.set(&target, value);
        matched = true;
        break;
    }
    matched
}

/// The first `String x = ctx.<path>;` binding, as (local, path).
fn local_bound_to_ctx(script: &str) -> Option<(String, String)> {
    for line in script.lines() {
        let line = line.trim().trim_end_matches(';');
        let Some((declaration, value)) = line.split_once(" = ") else {
            continue;
        };
        let Some(path) = value.trim().strip_prefix("ctx.") else {
            continue;
        };
        let name = declaration.rsplit(' ').next()?;
        if declaration.split(' ').count() != 2 || name.is_empty() {
            continue;
        }
        return Some((name.to_string(), clean_path(path)));
    }
    None
}

/// The literal a guard compares `local` to, whichever quote it used.
fn equality_literal(guard: &str, local: &str) -> Option<String> {
    let rest = guard.trim().strip_prefix(local)?.trim_start();
    let rest = rest.strip_prefix("==")?.trim();
    let quote = rest.chars().next().filter(|c| *c == '\'' || *c == '"')?;
    let rest = &rest[quote.len_utf8()..];
    let end = rest.find(quote)?;
    // A compound guard is a different shape, not this one.
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
/// The shape is a helper that finds the last separator and returns what
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
fn try_basename_after_separator(event: &mut Event, script: &str) -> bool {
    let Some(separator) = last_index_of_separator(script) else {
        return false;
    };

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
        let value = value.trim();
        if let Some(path) = value.strip_prefix("ctx.") {
            locals.push((name.trim().to_string(), clean_path(path)));
        }
    }

    let mut wrote = false;
    // The call is `(<local>)` with nothing else in the parentheses. The
    // helper's DEFINITION takes `def path`, which is not one of the locals,
    // so it is skipped for free -- and so is `(cmd != null && ...)`, which
    // names the local but is not a call.
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

        if let Some(text) = event.get_str(source) {
            // Elastic writes nothing when the path holds no separator, and
            // nothing when the basename is empty -- a trailing separator.
            if let Some((_, base)) = text.rsplit_once(separator)
                && !base.is_empty()
            {
                let base = base.to_string();
                let _ = event.set(&target, base);
            }
        }
        wrote = true;
    }

    wrote
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
    first_quoted(arguments)
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

/// The first single- or double-quoted literal in a fragment.
fn first_quoted(text: &str) -> Option<String> {
    let start = text.find(['\'', '"'])?;
    let quote = text[start..].chars().next()?;
    let rest = &text[start + quote.len_utf8()..];
    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
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
    let Some(wanted) = first_quoted(argument) else {
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

        let rendered = crate::painless_helpers::java_to_string(&value);
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
/// Three argument shapes appear, and all three resolve to a value on the
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

/// The `ctx.<target> = <fn>(ctx.<source>)` line the converter is applied by.
///
/// The two spellings differ only in whether the target is the source: `entra_id`
/// rewrites its own object in place, `sentinel_one` writes the converted `json`
/// somewhere new. Both are one assignment, so one reader covers them.
fn snake_case_apply(script: &str) -> Option<(String, String)> {
    for line in script.lines().rev() {
        let line = line.trim().trim_end_matches(';');
        let Some((target, rhs)) = line.split_once(" = ") else {
            continue;
        };
        let target = target.trim();
        if !target.starts_with("ctx.") {
            continue;
        }
        // `convertToSnakeCase(ctx.json)` -- the helper's name is not fixed, so
        // the shape of the call is what identifies it.
        let Some((_, argument)) = rhs.trim().split_once("SnakeCase(") else {
            continue;
        };
        let argument = argument.trim_end_matches(')').trim();
        if !argument.starts_with("ctx.") {
            continue;
        }
        return Some((target[4..].to_string(), argument[4..].to_string()));
    }
    None
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LadderArm {
    literal: String,
    target: String,
    value: String,
}

/// An `if (x == 'a') { ctx.t = 'A' } else if (x == 'b') { ... }` ladder.
///
/// The subject is read once -- either bound to a local (`def x = ctx.a.b;`)
/// or compared inline -- and every arm assigns a string literal to a ctx
/// path. Fortinet's 11-arm IANA-number-to-transport table is the shape;
/// writing the table out by hand is how a mapping silently goes stale.
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

/// Parse an equality ladder, or `None` if the script is a different shape.
fn parse_ladder(script: &str) -> Option<Ladder> {
    use crate::painless_params::clean_path;

    // A ladder is the WHOLE of its script. One that loops is doing something
    // else entirely -- powershell's entropy pass walks the script block
    // character by character -- and claiming it runs the ladder instead of the
    // real work.
    if script.contains("for (") {
        return None;
    }

    // The subject is whatever the FIRST `if (... == ...)` compares against.
    let first = script.find("if (")? + 4;
    let (lhs, _) = script[first..].split_once("==")?;
    let lhs = lhs.trim();
    let mut through_put = false;

    let mut arms = Vec::new();
    for segment in script.split("if (").skip(1) {
        let Some((cond, body)) = segment.split_once(')') else {
            continue;
        };
        let Some((_, rhs)) = cond.split_once("==") else {
            continue;
        };
        let Some(literal) = quoted_first(rhs) else {
            continue;
        };
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
                literal,
                target: format!("{}.{key}", clean_path(parent.trim())),
                value,
            }
        } else {
            let (Some(value), Some(assign)) = (quoted_first(body), body.find('=')) else {
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
            LadderArm {
                literal,
                target: clean_path(target.trim()),
                value,
            }
        };
        arms.push(arm);
    }
    if arms.len() < 2 {
        return None;
    }
    // A `.put()` arm is also how a script BUILDS a map, so the shape only
    // claims one where every `if` in it is an arm -- nothing else going on.
    if through_put && arms.len() != script.matches("if (").count() {
        return None;
    }

    // A local binding resolves back to the ctx path it was read from, with any
    // case fold the binding applied travelling with it.
    let bound = ctx_path_bound_to(script, lhs);
    let fold_case = bound
        .as_deref()
        .is_some_and(|path| path.ends_with(".toLowerCase()") || path.ends_with(".toUpperCase()"));
    let subject = match bound {
        Some(path) => path
            .strip_suffix(".toLowerCase()")
            .or_else(|| path.strip_suffix(".toUpperCase()"))
            .unwrap_or(&path)
            .to_string(),
        None => clean_path(lhs.strip_prefix("ctx.")?),
    };

    Some(Ladder {
        subject,
        fold_case,
        arms,
    })
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
    // A guard sitting above the assignment holds a `ctx.` of its own, and
    // taking THAT as the target invented `winlog.event_data.SubcategoryGuid ==
    // null) {` as a field. A target is a plain dotted path or it is not ours.
    if !target
        .chars()
        .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_' | '-' | '@'))
    {
        return false;
    }
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
fn try_ladder(event: &mut Event, ladder: &Ladder) -> bool {
    let subject = event
        .get_str(&ladder.subject)
        .map(String::from)
        .or_else(|| event.get_i64(&ladder.subject).map(|n| n.to_string()));

    let Some(subject) = subject else {
        return true;
    };
    let matches = |arm: &&LadderArm| {
        if ladder.fold_case {
            arm.literal.eq_ignore_ascii_case(&subject)
        } else {
            arm.literal == subject
        }
    };
    if let Some(arm) = ladder.arms.iter().find(matches) {
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
fn parse_substring_before_last(script: &str) -> Option<KnownShape> {
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
        KnownShape::SubstringBeforeLast {
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
fn run_pascal_keys(event: &mut Event, shape: &PascalKeys) -> bool {
    let Some(Value::Object(entries)) = event.get(&shape.path).cloned() else {
        return true;
    };

    let mut rebuilt = Map::new();
    for (key, value) in entries {
        let tail = (!shape.prefix.is_empty())
            .then(|| key.strip_prefix(shape.prefix.as_str()))
            .flatten()
            .filter(|tail| tail.chars().next().is_some_and(char::is_uppercase));
        let renamed = if let Some(tail) = tail {
            format!("{}{tail}", shape.replacement)
        } else {
            let mut chars = key.chars();
            chars.next().map_or_else(
                || key.clone(),
                |first| first.to_uppercase().chain(chars).collect(),
            )
        };
        rebuilt.insert(renamed, value);
    }
    let _ = event.set(&shape.path, Value::Object(rebuilt));
    true
}

/// A numeric field's bits decoded into a list of names.
///
/// `aws/vpcflow` and `aws/firewall_logs` both spell out the six TCP flags this
/// way. The masks and names are read off the script rather than assumed to be
/// TCP's, because nothing in the shape says they must be.
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
fn try_guarded_copy(event: &mut Event, script: &str) -> bool {
    // Every guarded copy in the script, not just the first. Windows'
    // `security_standard` is four hundred lines of them -- one per winlog
    // field, each in its own `if (... != null) { ... }` with a null-guard
    // preamble -- and taking only the first claimed the script and lost the
    // rest.
    if crate::painless_params::run_guarded_literals(event, script) {
        return true;
    }

    let Some((cond, rest)) = script.split_once("!= null") else {
        return false;
    };
    let Some(source) = painless_path(cond) else {
        return false;
    };
    // The body starts where the CONDITION ends. Splitting at the first `=`
    // after `!= null` cut a two-clause guard in half: aws/ec2_metrics gates on
    // `&& ctx.host?.cpu?.usage == null` and then divides the source in place,
    // so the guard's own field was read as the target and the RAW percentage
    // was copied onto it -- 42 where the agent had already written 0.421.
    let body = condition_body(cond, rest);
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
    factor: i64,
}

/// Read the multiply and the assignment that owns it.
///
/// The assignment is found by walking BACK from the multiply to the last real
/// `=`, because the script is not reliably one statement per line -- the
/// vendor folds it with YAML's `>-` and it arrives as a single line. Taking
/// the FIRST `=` instead found the `==` of an `if (ctx.event == null)`
/// preamble, so the target read as `event` and nothing was written: 25 zscaler
/// events with no `event.duration` at all.
fn parse_scale_field(script: &str) -> Option<ScaleField> {
    let (head, factor) = script.rsplit_once('*')?;
    // `L` is Java's long suffix, which checkpoint writes on its 1e9 constant.
    let factor = factor
        .trim()
        .trim_end_matches([';', ')', ' '])
        .trim_end_matches(['L', 'l'])
        .trim()
        .parse::<i64>()
        .ok()?;

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

/// The byte offset of the last `=` that ASSIGNS, rather than compares.
///
/// `==`, `!=`, `<=`, `>=` and the compound arithmetic forms are all reads.
pub(crate) fn last_assignment(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    text.char_indices().rev().find_map(|(at, c)| {
        if c != '=' {
            return None;
        }
        let before = at.checked_sub(1).map(|i| bytes[i]);
        let after = bytes.get(at + 1).copied();
        let compares = matches!(
            before,
            Some(b'=' | b'!' | b'<' | b'>' | b'+' | b'-' | b'*' | b'/')
        ) || after == Some(b'=');
        (!compares).then_some(at)
    })
}

/// Multiply the source into the target.
fn run_scale_field(event: &mut Event, shape: &ScaleField) -> bool {
    if let Some(n) = event.get_as_i64(&shape.source) {
        let _ = event.set(&shape.target, json!(n.saturating_mul(shape.factor)));
    }
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

    // Reading the target with `painless_path` alone takes the last `ctx.` path
    // ANYWHERE before the assignment, which on the local form is the `end`
    // declaration -- crowdstrike's alert span landed on `event.end`.
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

fn run_contains_ladder(event: &mut Event, shape: &ContainsLadder) -> bool {
    let Some(subject) = event.get_str(&shape.subject).map(str::to_string) else {
        return true;
    };
    let subject = if shape.lower {
        subject.to_lowercase()
    } else {
        subject
    };
    let Some(value) = event.get(&shape.value).cloned() else {
        return true;
    };

    for (needle, target) in &shape.arms {
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
fn run_slice_each_item(event: &mut Event, shape: &SliceEachItem) -> bool {
    let Some(Value::Array(items)) = event.get(&shape.source) else {
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

    let Some(written) = shape
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

fn run_collect_columns(event: &mut Event, shape: &CollectColumns) -> bool {
    let Some(Value::Array(items)) = event.get(&shape.source) else {
        return true;
    };
    let items = items.clone();

    let mut built = serde_json::Map::new();
    for (key, member) in &shape.columns {
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

    let value = match &shape.inner {
        Some(inner) => {
            let mut wrapper = serde_json::Map::new();
            wrapper.insert(inner.clone(), Value::Object(built));
            Value::Object(wrapper)
        }
        None => Value::Object(built),
    };
    let _ = event.set(&format!("{}.{}", shape.target, shape.name), value);
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

/// The text after the source's LAST separator, where there is one.
fn run_suffix_after_separator(event: &mut Event, source: &str, target: &str, sep: &str) -> bool {
    if let Some(text) = event.get_str(source)
        && let Some((_, suffix)) = text.rsplit_once(sep)
        && !suffix.is_empty()
    {
        let suffix = suffix.to_string();
        let _ = event.set(target, json!(suffix));
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
fn run_flags_present(event: &mut Event, shape: &FlagsPresent) -> bool {
    let Some(map) = event.get_object(&shape.source) else {
        return true;
    };
    let flags: Vec<&String> = shape
        .keys
        .iter()
        .filter(|key| map.get(*key).and_then(Value::as_str) == Some(shape.wanted.as_str()))
        .collect();
    let _ = event.set(&shape.target, json!(flags));
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
fn run_zip_lists(event: &mut Event, shape: &ZipLists) -> bool {
    let mut lists = Vec::with_capacity(shape.columns.len());
    for column in &shape.columns {
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
        if let Some(message) = &shape.mismatch {
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
        let row: serde_json::Map<String, Value> = shape
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
    let _ = event.set(&shape.target, Value::Array(out));
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
fn run_nanos_between(event: &mut Event, shape: &NanosBetween) -> bool {
    let (Some(start), Some(end)) = (
        instant_nanos(event, &shape.start),
        instant_nanos(event, &shape.end),
    ) else {
        return true;
    };
    let span = end - start;
    if shape.non_negative && span < 0 {
        return true;
    }
    let _ = event.set(&shape.target, json!(span));
    true
}

/// Read `if (ctx.<a> != null && ctx.<b> == null) { ctx.<a> = ctx.<a> / <n>; }`.
///
/// `aws/ec2_metrics` and `aws/rds` turn a `CloudWatch` percentage into a fraction
/// this way, and only when the agent has not already written the fraction
/// itself -- that second clause is the whole point of the script, so the
/// absent-field guard is carried rather than assumed.
fn parse_guarded_divide(script: &str) -> Option<KnownShape> {
    use crate::painless_params::clean_path;

    let line = script
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("ctx.") && line.contains('/'))?;
    let (lhs, rhs) = line.split_once(" = ")?;
    let target = clean_path(lhs.trim().strip_prefix("ctx.")?);

    let (value, divisor) = rhs.trim().trim_end_matches(';').rsplit_once('/')?;
    let divisor = divisor.trim().parse::<i64>().ok().filter(|n| *n != 0)?;
    if clean_path(value.trim().strip_prefix("ctx.")?) != target {
        return None;
    }

    let absent = script.split_once("== null").and_then(|(head, _)| {
        let at = head.rfind("ctx.")?;
        let path = clean_path(&head[at + "ctx.".len()..]);
        (!path.is_empty()
            && path
                .chars()
                .all(|c| c.is_alphanumeric() || matches!(c, '.' | '_')))
        .then_some(path)
    });

    Some(KnownShape::GuardedDivide {
        target,
        absent,
        divisor,
    })
}

/// Divide a field in place, unless the guarded field is already set.
fn run_guarded_divide(
    event: &mut Event,
    target: &str,
    absent: Option<&String>,
    divisor: i64,
) -> bool {
    if absent.is_some_and(|path| event.has_value(path)) {
        return true;
    }
    if let Some(value) = event.get_f64(target) {
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
///
/// The dispatch is two halves. [`known_shapes`] reads the script TEXT and
/// names the matchers it triggers -- a decision that never changes for a given
/// script, which is why [`crate::painless_plan::PainlessPlan`] makes it once
/// per call site. [`run_known_shape`] then runs one matcher against one event.
/// This entry point does both per call, for callers without a plan.
pub fn try_known_painless(event: &mut Event, script: &str) -> bool {
    let normalised = normalise(script);
    known_shapes(&normalised)
        .iter()
        .any(|shape| run_known_shape(event, &normalised, shape))
}

/// A matcher branch of the text-only dispatch, with whatever the trigger's own
/// parse already recovered from the script.
///
/// The variants up to `KeysToSnakeCase` recognise what a script DOES and work
/// for any source that writes the shape; the rest are keyed on a vendor's
/// FIELD NAMES and recognise whose script it is, ending in the two catch-alls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KnownShape {
    DropEmpty {
        policy: DropPolicy,
        root: Option<String>,
    },
    SplitCommandLine(crate::painless_windows::ArgvScript),
    Basename,
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
    TrimListInPlace(String),
    StartsWithAppend {
        source: String,
        prefix: String,
        target: String,
        value: String,
    },
    CategoryTypeLadder(Vec<CategoryArm>),
    KeysStripWhitespace(String),
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
    SuffixAfterSeparator {
        source: String,
        target: String,
        separator: String,
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
    SplitTokenField {
        source: String,
        separator: char,
        parse_int: bool,
        target: String,
        /// A second separator whose FIRST piece is what each member keeps.
        head: Option<char>,
    },
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
    },
    SplitTrimCollect,
    SumDirections(&'static str),
    SumOfFields,
    DurationToNanos,
    FlowDuration,
    ParallelDispatch,
    ConcatMessage,
    SwapSubtrees,
    CollectingLadder,
    CaseInsensitiveLadder,
    EqualityLadder(Ladder),
    SentinelRemovalLiteral,
    RowLookupWithFallback,
    SchemelessUrl,
    VersionSplit,
    SyslogPriority,
    AppendUnique {
        from: &'static str,
        into: &'static str,
    },
    SplitUnquotedKv(Box<SplitKv>),
    ArrayToIndexedObject,
    KeyValuePairs,
    JoinOptional,
    AppendEach,
    LiteralValueMap,
    KeysToSnakeCase(Option<String>),
    CommandLine {
        parent: bool,
    },
    ProcessStartTime,
    EmailSplit,
    RiskBehaviors,
    AzureCategoryEventType,
    AzureEventCategory,
    ReplaceDotsInKeys,
    OktaTargetRename,
    CollectMapValues,
    GuardedReplace,
    ScaleField(Box<ScaleField>),
    NanosBetween(Box<NanosBetween>),
    ProcessNameFromCommandLine,
    FlagsPresent(Box<FlagsPresent>),
    ZipLists(Box<ZipLists>),
    GuardedDivide {
        target: String,
        absent: Option<String>,
        divisor: i64,
    },
    GuardedCopy,
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
pub(crate) fn known_shapes(normalised: &str) -> Vec<KnownShape> {
    let mut shapes = Vec::new();

    // Pattern: seconds to nanoseconds, closing the span it opens. Ahead of the
    // scale-by-literal fallback, which reads the same multiply and stops
    // there, leaving `event.end` unwritten.
    if normalised.contains("ChronoUnit.NANOS")
        && let Some(source) = normalised
            .split_once("(int) (ctx.")
            .and_then(|(_, rest)| rest.split_once(" *"))
            .map(|(path, _)| crate::painless_params::clean_path(path))
    {
        shapes.push(KnownShape::SecondsToSpan(source));
        return shapes;
    }

    // Pattern: the span between two parsed instants, in nanoseconds. The
    // defender and crowdstrike pipelines derive `event.duration` this way.
    if normalised.contains("ChronoUnit.NANOS.between(")
        && let Some(shape) = parse_nanos_between(normalised)
    {
        shapes.push(KnownShape::NanosBetween(Box::new(shape)));
        return shapes;
    }

    // Pattern: the keys of a fixed list that a map marks `"true"`.
    if normalised.contains("instanceof Map")
        && normalised.contains("] == \"true\"")
        && let Some(shape) = parse_flags_present(normalised)
    {
        shapes.push(KnownShape::FlagsPresent(Box::new(shape)));
        return shapes;
    }

    // Pattern: one list cut two ways, a column per cut.
    if normalised.contains(".substring(")
        && normalised.contains(".add(")
        && let Some(shape) = parse_slice_each_item(normalised)
    {
        shapes.push(KnownShape::SliceEachItem(Box::new(shape)));
        return shapes;
    }

    // Pattern: a type tag choosing which field one value lands on.
    if normalised.contains(".contains('")
        && let Some(shape) = parse_contains_ladder(normalised)
    {
        shapes.push(KnownShape::ContainsLadder(Box::new(shape)));
        return shapes;
    }

    // Pattern: one column per member, gathered off a list of objects.
    if normalised.contains(" : ctx.")
        && normalised.contains(".isEmpty()")
        && let Some(shape) = parse_collect_columns(normalised)
    {
        shapes.push(KnownShape::CollectColumns(Box::new(shape)));
        return shapes;
    }

    // Pattern: parallel lists zipped into a list of objects.
    if normalised.contains(".add([")
        && normalised.contains("new ArrayList()")
        && let Some(shape) = parse_zip_lists(normalised)
    {
        shapes.push(KnownShape::ZipLists(Box::new(shape)));
        return shapes;
    }

    // Pattern: the executables of `process.command_line` folded into
    // `process.name`. The defender pipelines share one copy of this script.
    if normalised.contains("currentNames") && normalised.contains("ctx.process.command_line") {
        shapes.push(KnownShape::ProcessNameFromCommandLine);
        return shapes;
    }

    // Pattern: drop null and empty values recursively. Matched on the SHAPE,
    // not the helper's name -- panw spells it `dropEmptyFields`, and keying
    // on `drop(ctx)` left every emptied object behind. What counts as empty
    // comes from the script's own predicate, which is not the same
    // everywhere; vpcflow runs it over ONE subtree rather than the document.
    if normalised.contains("removeIf")
        && normalised.contains("instanceof Map")
        && normalised.contains("instanceof List")
    {
        if normalised.contains("(ctx)") {
            shapes.push(KnownShape::DropEmpty {
                policy: DropPolicy::read(normalised),
                root: None,
            });
            return shapes;
        }
        if let Some(at) = normalised.rfind("(ctx.")
            && let Some(root) = normalised[at + "(ctx.".len()..].split(')').next()
            && root
                .chars()
                .all(|c| c.is_alphanumeric() || c == '.' || c == '_' || c == '?')
        {
            shapes.push(KnownShape::DropEmpty {
                policy: DropPolicy::read(normalised),
                root: Some(crate::painless_params::clean_path(root)),
            });
            return shapes;
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
        shapes.push(KnownShape::SplitCommandLine(parsed));
        return shapes;
    }

    // Pattern: the basename of one or more path fields -- everything after
    // the last separator. Guarded by the parse rather than by the trigger,
    // so a script that only looks similar falls through.
    if normalised.contains("lastIndexOf(") && normalised.contains(".substring(") {
        shapes.push(KnownShape::Basename);
        // Pattern: the same cut, but written straight onto a ctx path and
        // landing on a DIFFERENT one -- `file.name` to `file.extension`.
        if let Some((source, target, separator)) = parse_suffix_after_separator(normalised) {
            shapes.push(KnownShape::SuffixAfterSeparator {
                source,
                target,
                separator,
            });
        }
    }

    // Pattern: sysmon's file split -- name and directory at the last
    // backslash, the extension off the whole path's last dot.
    if normalised.contains(".name = path.substring(idx+1)")
        && normalised.contains(".directory = path.substring(0, idx)")
        && let Some(source) = crate::painless_windows::file_info_source(normalised)
    {
        shapes.push(KnownShape::FileInfo(source));
        return shapes;
    }

    // Pattern: sysmon's hash-map lowercasing, empty and all-zero hashes
    // dropped and `related` replaced with the hash list.
    if normalised.contains("hashIsEmpty(")
        && let Some(source) = crate::painless_windows::hash_lowercase_source(normalised)
    {
        shapes.push(KnownShape::HashLowercase(source));
        return shapes;
    }

    // Pattern: the tail of one string field past another field's length,
    // optionally dropping one leading comma -- umbrella's identities dance.
    if normalised.contains(".substring(ctx.")
        && normalised.contains(".length())")
        && let Some(shape) = parse_prefix_tail(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: a scalar field prepended to an array field into a target.
    if normalised.contains("new ArrayList()")
        && normalised.contains(".add(ctx.")
        && let Some(shape) = parse_prepend_to_array(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: cloudtrail's resources -- ARN and accountId renamed per
    // element, then deduplicated by the arn_account_type composite.
    if normalised.contains("uniqueResources")
        && let Some(at) = normalised.find(" instanceof List")
        && let Some(source) = normalised[..at]
            .rfind("ctx.")
            .map(|s| &normalised[s + 4..at])
    {
        shapes.push(KnownShape::ResourcesRenameDedup(
            crate::painless_params::clean_path(source),
        ));
        return shapes;
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
            shapes.push(KnownShape::SecurityhubResource(source));
            return shapes;
        }
        if normalised.contains("ctx.resource.type.add(") {
            shapes.push(KnownShape::SecurityhubResources(source));
            return shapes;
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
        shapes.push(KnownShape::M365ProcessEvidence(path));
        return shapes;
    }
    if normalised.contains("def processUserName = new HashSet()")
        && let Some(path) = evidence_loop_path(normalised)
    {
        shapes.push(KnownShape::M365IdentityEvidence(path));
        return shapes;
    }

    // Pattern: route53's answers rebuilt into ECS, feeding related.* as they go.
    if normalised.contains("answer?.Rdata") && normalised.contains("new_answer") {
        shapes.push(KnownShape::Route53Answers);
        return shapes;
    }

    // Pattern: the address a reverse-lookup question names, back out of its
    // `in-addr.arpa` / `ip6.arpa` labels and into `related.ip`.
    if normalised.contains(".in-addr.arpa") && normalised.contains(".ip6.arpa") {
        shapes.push(KnownShape::ReverseLookupAddress);
        return shapes;
    }

    // Pattern: gcp's long-running operation, which opens and closes a session.
    if normalised.contains(".category.add('session')")
        && let Some(first) = ternary_default_path(normalised, "first")
        && let Some(last) = ternary_default_path(normalised, "last")
    {
        shapes.push(KnownShape::LongOperationSession { first, last });
        return shapes;
    }

    // Pattern: powershell's script-block entropy and the spread around it.
    if normalised.contains("double surprisalVar")
        && let Some(source) = ctx_path_bound_to(normalised, "script")
    {
        shapes.push(KnownShape::ScriptBlockEntropy(source));
        return shapes;
    }

    // Pattern: zscaler's pipe-delimited columns, split in place.
    if normalised.contains("void splitStr(Map m, String key)") {
        let fields = parse_split_on_pipe(normalised);
        if !fields.is_empty() {
            shapes.push(KnownShape::SplitOnPipe(fields));
            return shapes;
        }
    }

    // Pattern: zscaler's parallel attachment columns zipped into one list.
    if normalised.contains("item.put('file', file)")
        && let Some(zip) = AttachmentZip::parse(normalised)
    {
        shapes.push(KnownShape::ZipAttachments(Box::new(zip)));
        return shapes;
    }

    // Pattern: the highest score any of a field's values scores, each scored by
    // the substring it contains.
    if normalised.contains("if (cur > maxSev) maxSev = cur;")
        && let Some(shape) = MaxByContains::parse(normalised)
    {
        shapes.push(KnownShape::MaxByContains(Box::new(shape)));
        return shapes;
    }

    // Pattern: the same zip with no wrapper, driven by one column's length and
    // skipping the vendor's placeholder names.
    if normalised.contains("out.add(item)")
        && let Some(zip) = ColumnZip::parse(normalised)
    {
        shapes.push(KnownShape::ZipColumns(Box::new(zip)));
        return shapes;
    }

    // Pattern: m365's `isTruthy` helper, one target per vendor flag.
    if normalised.contains("def isTruthy(def val)") {
        let pairs = parse_truthy_assignments(normalised);
        if !pairs.is_empty() {
            shapes.push(KnownShape::TruthyAssignments(pairs));
            return shapes;
        }
    }

    // Pattern: drop one literal out of a list -- m365's file.path, whose
    // append leaves a bare separator when neither half of its template is
    // there.
    if normalised.contains(".removeIf(")
        && !normalised.contains("instanceof Map")
        && let Some(shape) = parse_remove_list_value(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: gcp audit's related.entity, whose `isKubernetes` gate decides
    // which three of its sources are suppressed.
    if normalised.contains("boolean isKubernetes") && normalised.contains("ctx.related.entity") {
        shapes.push(KnownShape::GcpRelatedEntity);
        return shapes;
    }

    // Pattern: mimecast's related.* collection -- display names and email
    // addresses off named paths, split at the `@`, sorted.
    if normalised.contains("splitmail(")
        && normalised.contains("related.hosts")
        && let Some(shape) = parse_mail_related(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: inspector's twin of the above -- the same one-or-many split,
    // over its own lower-cased member names.
    if normalised.contains("ctx.aws.inspector.resources") && normalised.contains("ctx.resource.id")
    {
        shapes.push(KnownShape::InspectorResources {
            multi: normalised.contains("ctx.resource.id.add("),
        });
        return shapes;
    }

    // Pattern: checkpoint's dropped-packet tuples into structured maps.
    if normalised.contains("packets_dropped") && normalised.contains(".splitOnToken('>')") {
        shapes.push(KnownShape::CheckpointPackets);
        return shapes;
    }

    // Pattern: every member of a list trimmed where it sits -- cloudfront's
    // split x-forwarded-for, whose next processor greps each member anchored.
    if normalised.contains(".trim();")
        && normalised.contains("[i] =")
        && let Some(shape) = parse_trim_list(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: a constant appended when a member of a list carries a prefix --
    // cloudfront's `localhost:8080`, which its grok cannot read as an address.
    if normalised.contains(".startsWith(")
        && normalised.contains(".add(")
        && normalised.contains("ctx[")
        && let Some(shape) = parse_starts_with_append(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: elb's `tlsv12` -- the protocol and version out of one token.
    if normalised.contains("ctx.tls.version_protocol")
        && normalised.contains(".splitOnToken(\"v\")")
        && let Some(shape) = parse_tls_version_split(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: sequential loop-over-category ladders assigning a LIST
    // literal -- defender's event.type.
    if normalised.contains(" in ctx.event.category)")
        && normalised.contains("break;")
        && let Some(shape) = parse_category_type_ladder(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: whitespace stripped from every key of one map -- powershell's
    // spaced event_data names.
    if normalised.contains(".matcher(entry.getKey()")
        && normalised.contains("replaceAll(\"\")")
        && let Some(at) = normalised.find(".entrySet()")
        && let Some(start) = normalised[..at].rfind("ctx.")
    {
        shapes.push(KnownShape::KeysStripWhitespace(
            crate::painless_params::clean_path(&normalised[start + 4..at]),
        ));
        return shapes;
    }

    // Pattern: one map copied to another path with its keys snake_cased by a
    // helper the script defines itself -- lambda's REPORT metrics. The helper's
    // name is not fixed (`underscore` here), so the replacement it performs is
    // what identifies it.
    if normalised.contains("([a-z])([A-Z]+)")
        && normalised.contains(".getKey()")
        && let Some(shape) = parse_snake_key_map_copy(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: strip a surrounding `<...>` pair from named fields and each
    // member of a list -- proofpoint's mail addresses.
    if normalised.contains(".startsWith(\"<\")")
        && normalised.contains(".endsWith(\">\")")
        && let Some(shape) = parse_strip_angle_pairs(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: a list of {name, value} pairs folded into a map that REPLACES
    // the target -- proofpoint's audit labels.
    if normalised.contains(", new HashMap())")
        && normalised.contains("for (")
        && let Some(shape) = parse_name_value_fold(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: the text before the last CASE-INSENSITIVE marker. The
    // `toLowerCase()` is what separates this from panw's url and file scripts,
    // which cut on a plain `lastIndexOf` and are several statements long.
    if normalised.contains(".toLowerCase().lastIndexOf(")
        && normalised.contains(".substring(0, ")
        && let Some(shape) = parse_substring_before_last(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: every key of one map capitalised, ahead of PascalCase renames.
    if normalised.contains(".substring(0, 1).toUpperCase()")
        && normalised.contains(".entrySet()")
        && let Some(shape) = parse_pascal_keys(normalised)
    {
        shapes.push(KnownShape::PascalKeys(Box::new(shape)));
        return shapes;
    }

    // Pattern: a numeric field's bits decoded into a list of names.
    if normalised.contains("parseUnsignedInt(")
        && normalised.contains("& 0x")
        && let Some(shape) = parse_bit_flag_names(normalised)
    {
        shapes.push(KnownShape::BitFlagNames(Box::new(shape)));
        return shapes;
    }

    // Pattern: cloudtrail's ConsoleLogin extras.
    if normalised.contains("aed_map") && normalised.contains("'ConsoleLogin'") {
        shapes.push(KnownShape::ConsoleLoginEventData);
        return shapes;
    }

    // Pattern: the same fold written as an indexed loop -- aws/waf's request
    // headers and the headers it inserts.
    if normalised.contains("= new HashMap()")
        && normalised.contains("] = ctx.")
        && let Some(shape) = parse_indexed_name_value_fold(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: an outcome read off the tag whose name carries the action's
    // prefix -- proofpoint's audit outcome.
    if normalised.contains("+ '.'")
        && normalised.contains(".startsWith(action)")
        && let Some(shape) = parse_outcome_from_tags(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: "Rename Common Auth Fields" -- process, source and client
    // fields out of event_data with the script's own conversions.
    if normalised.contains("WorkstationName")
        && normalised.contains("ClientAddress")
        && let Some(codes) = crate::painless_windows::event_code_list(normalised)
    {
        shapes.push(KnownShape::RenameCommonAuth(codes));
        return shapes;
    }

    // Pattern: "Copy MemberName to User and User to Group" -- the split DN
    // to user.target, the Target fields to group.*. Ahead of its cousins,
    // whose triggers its text also spells.
    if normalised.contains("MemberNameParts")
        && let Some(codes) = crate::painless_windows::event_code_list(normalised)
    {
        shapes.push(KnownShape::CopyMemberName(codes));
        return shapes;
    }

    // Pattern: "Copy Target User to Computer Object".
    if normalised.contains("computerObject")
        && let Some(codes) = crate::painless_windows::event_code_list(normalised)
    {
        shapes.push(KnownShape::CopyComputerObject(codes));
        return shapes;
    }

    // Pattern: "Copy Target User to Target" and its Effective twin -- the
    // base and the SID field are the script's own.
    if normalised.contains("def userId = ctx.")
        && normalised.contains("ctx.user.put(")
        && let Some(codes) = crate::painless_windows::event_code_list(normalised)
        && let Some(base) = crate::painless_windows::copy_base(normalised)
        && let Some(sid_field) = crate::painless_windows::copy_sid_field(normalised)
    {
        shapes.push(KnownShape::CopyUserToBase {
            codes,
            base,
            sid_field,
        });
        return shapes;
    }

    // Pattern: event 5136's `ObjectDN`, whose CN carries RFC 4514 escapes.
    if normalised.contains("ObjectDN")
        && normalised.contains("StringBuilder cn")
        && normalised.contains("objectClass")
    {
        shapes.push(KnownShape::ObjectDn);
        return shapes;
    }

    // Pattern: the file-share events' path block. Ahead of the basename
    // matcher, whose `lastIndexOf`/`substring` pair this script also spells.
    if normalised.contains("RelativeTargetName")
        && normalised.contains("ShareLocalPath")
        && let Some(codes) = crate::painless_windows::contained_code_list(normalised)
    {
        shapes.push(KnownShape::ShareFilePath(codes));
        return shapes;
    }

    // Pattern: event 4688's process block. Ahead of the append matcher, which
    // its closing `related.user.add` triggers -- that claimed the script and
    // left the whole process block unwritten.
    if normalised.contains("NewProcessId")
        && normalised.contains("ParentProcessName")
        && let Some(codes) = crate::painless_windows::event_code_list(normalised)
    {
        shapes.push(KnownShape::ProcessCreated(codes));
        return shapes;
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
        shapes.push(KnownShape::CopyTargetUser(codes));
        return shapes;
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
        shapes.push(KnownShape::CopySubjectUser(codes));
        return shapes;
    }

    // Pattern: zscaler's splitStr batch -- named map members split on `|` in
    // place. Ahead of the append-each matcher, whose `.add(` and
    // `instanceof Map` triggers the helper also spells.
    if normalised.contains("void splitStr(")
        && let Some(fields) = parse_split_pipe_fields(normalised)
    {
        shapes.push(KnownShape::SplitPipeFields(fields));
        return shapes;
    }

    // Pattern: one field split on a token into a list, optionally parsed to
    // integers -- endpoint_dlp's dictionary counts. Same trigger overlap as
    // above.
    if normalised.contains(".splitOnToken(")
        && normalised.contains("new ArrayList()")
        && let Some(shape) = parse_split_token_field(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: `ctx.<t> = ctx.<s>.decodeBase64();` -- zscaler web's URL and
    // referer.
    if normalised.contains(".decodeBase64()")
        && let Some(shape) = parse_decode_base64(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: a split's token COUNT stored on the event -- vpcflow's format
    // dispatch, where every dissect gates on it.
    if normalised.contains(".splitOnToken(")
        && normalised.contains(").length")
        && let Some(shape) = parse_token_count(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: one value wrapped in a one-element list -- mimecast's
    // attachments promotion. No loop, or it is the prepend shape below.
    if (normalised.contains("= [];") || normalised.contains("new ArrayList()"))
        && normalised.contains(".add(ctx.")
        && !normalised.contains("for (")
        && let Some(shape) = parse_wrap_value_in_list(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: the LAST element of an array assigned to a field --
    // mimecast's attachment extension off the split path.
    if normalised.contains(".length-1]")
        && let Some(shape) = parse_last_element(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: classify each member of a list by string tests on the member.
    if normalised.contains("addNestedValue(") && normalised.contains("instanceof List") {
        shapes.push(KnownShape::ClassifyMembers);
        return shapes;
    }

    // Pattern: keep a rendered copy of a nested object beside the object.
    if normalised.contains("keep_flattened_duplicates") {
        shapes.push(KnownShape::FlattenedDuplicates);
        return shapes;
    }

    // Pattern: collect every non-empty value the script names into one sorted,
    // unique list.
    if normalised.contains("void addValue(") && normalised.contains("new TreeSet(") {
        shapes.push(KnownShape::CollectEntities);
        return shapes;
    }

    // Pattern: DNS RData as tab-separated columns, one answer per line.
    if normalised.contains("answer_parts[") && normalised.contains("dns_answers.add(") {
        shapes.push(KnownShape::DnsRdataAnswers);
        return shapes;
    }

    // Pattern: Google Public DNS's structured RData into `dns.answers`.
    if normalised.contains("structuredRdata") {
        shapes.push(KnownShape::StructuredRdataAnswers);
        return shapes;
    }

    // Pattern: the ECS lists an answer set feeds, keyed on the record type.
    if normalised.contains("for (answer in ctx.dns.answers)") {
        shapes.push(KnownShape::RelatedFromDnsAnswers);
        return shapes;
    }

    // Pattern: one synthesised DNS answer per resolved address, typed by
    // whether the address holds a colon.
    if normalised.contains("ctx.dns.answers.add(") && normalised.contains("ip.indexOf(\":\")") {
        shapes.push(KnownShape::AnswersFromResolvedIp);
        return shapes;
    }

    // Pattern: the integrations' own recursive camelCase-to-snake_case pair,
    // applied to one object. Checked EARLY: the recursive arm spells `.add(`
    // and `instanceof Map`, which the append-each matcher below claims and
    // then does nothing with.
    if normalised.contains("Character.isUpperCase(")
        && normalised.contains("instanceof Map")
        && let Some((target, source)) = snake_case_apply(normalised)
    {
        shapes.push(KnownShape::CamelToSnake { target, source });
        return shapes;
    }

    // Pattern: split, trim and collect several optional fields into one list.
    // Checked early: the script also spells `.add(` and `.splitOnToken(`, which
    // a later matcher reads as a different shape entirely.
    if normalised.contains("new HashSet(") && normalised.contains(".asList()") {
        shapes.push(KnownShape::SplitTrimCollect);
        return shapes;
    }

    // Pattern: network.bytes / network.packets as the sum of both directions.
    if let Some(total) = sum_of_directions(normalised) {
        shapes.push(KnownShape::SumDirections(total));
        return shapes;
    }

    // Pattern: one ctx field as the sum of two others.
    if normalised.contains(" + ctx.") {
        shapes.push(KnownShape::SumOfFields);
    }

    // Pattern: seconds to nanoseconds for event.duration.
    if normalised.contains("ctx.event.duration")
        && normalised.contains("Long.parseLong")
        && normalised.contains("1000000000")
    {
        shapes.push(KnownShape::DurationToNanos);
        return shapes;
    }

    // Pattern: an `hh:mm:ss` flow duration becomes a span ending at @timestamp.
    if normalised.contains("minusNanos(") && normalised.contains(".toCharArray()") {
        shapes.push(KnownShape::FlowDuration);
        return shapes;
    }

    // Pattern: two parallel arrays, one naming what the other holds.
    if normalised.contains("(ctx, ctx.") && normalised.contains("[i])") {
        shapes.push(KnownShape::ParallelDispatch);
        return shapes;
    }

    // Pattern: a string built up piece by piece under per-field guards --
    // cloudfront's `url.full` out of the protocol, domain, path and query.
    if normalised.contains("def ")
        && normalised.contains(" += ")
        && normalised.contains("!= \"\"")
        && let Some(shape) = parse_concat_parts(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: build a string out of ctx fields and literals.
    if normalised.contains("?: ''")
        && normalised.contains(".isEmpty()")
        && normalised.contains("\" + ")
    {
        shapes.push(KnownShape::ConcatMessage);
    }

    // Pattern: swap two ctx subtrees, keeping named keys on one side.
    if normalised.contains("def tmp = ctx.") {
        shapes.push(KnownShape::SwapSubtrees);
    }

    // Pattern: a ladder collecting into a list, written as scalar or array.
    if normalised.contains(".add(")
        && normalised.contains(".size()")
        && normalised.contains("else if (")
    {
        shapes.push(KnownShape::CollectingLadder);
    }

    // Pattern: a case-insensitive ladder mapping one field onto a literal.
    // Tried before the `==` ladder, which cannot read either the multi-literal
    // arms or the numeric right-hand sides.
    if normalised.contains(".equalsIgnoreCase(") && normalised.contains("else if (") {
        shapes.push(KnownShape::CaseInsensitiveLadder);
        return shapes;
    }

    // Pattern: an equality ladder mapping one field onto string literals.
    if normalised.contains("else if (")
        && let Some(ladder) = parse_ladder(normalised)
    {
        shapes.push(KnownShape::EqualityLadder(ladder));
        return shapes;
    }

    // Pattern: strip sentinel values and junk keys out of a parsed map.
    if normalised.contains(".entrySet().removeIf(") && normalised.contains("entry.getValue()") {
        shapes.push(KnownShape::SentinelRemovalLiteral);
        return shapes;
    }

    // Pattern: look a value up in a ctx-held table of rows, else fall back.
    if normalised.contains("for (def ") && normalised.contains(" : ctx.") {
        shapes.push(KnownShape::RowLookupWithFallback);
    }

    // Pattern: split a schemeless URL into its ECS components.
    if normalised.contains("domainPort") && normalised.contains("url.original") {
        shapes.push(KnownShape::SchemelessUrl);
        return shapes;
    }

    // Pattern: split a version string at its first digit.
    if normalised.contains("matcher.start()") {
        shapes.push(KnownShape::VersionSplit);
    }

    // Pattern: decompose a syslog PRI into ECS facility and severity.
    if normalised.contains("log.syslog") && normalised.contains("priority") {
        shapes.push(KnownShape::SyslogPriority);
        return shapes;
    }

    // Pattern: append one array into another, skipping duplicates.
    if let Some((from, into)) = append_unique_fields(normalised) {
        shapes.push(KnownShape::AppendUnique { from, into });
        return shapes;
    }

    // Pattern: quote-aware KV split of a whole vendor payload.
    if normalised.contains("splitUnquoted(")
        && let Some(shape) = parse_split_unquoted_kv(normalised)
    {
        shapes.push(KnownShape::SplitUnquotedKv(Box::new(shape)));
        return shapes;
    }

    // Pattern: re-key an array of maps into an object indexed by position.
    if normalised.contains("new HashMap()") && normalised.contains("String.valueOf(") {
        shapes.push(KnownShape::ArrayToIndexedObject);
        return shapes;
    }

    // Pattern: collapse an array of `{key, value}` maps into one object.
    if normalised.contains("[item.key] = item.value") {
        shapes.push(KnownShape::KeyValuePairs);
        return shapes;
    }

    // Pattern: join two optional fields, each alone if the other is absent.
    if normalised.matches("String ").count() == 2 && normalised.contains("} else if (") {
        shapes.push(KnownShape::JoinOptional);
        return shapes;
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
        shapes.push(KnownShape::AppendEach);
        return shapes;
    }

    // Pattern: a value map written out as an if/else-if chain over one field.
    // Guarded by the parse, so a script that merely branches falls through.
    if normalised.contains("else if (") {
        shapes.push(KnownShape::LiteralValueMap);
    }

    // Pattern: keys_to_snake_case
    if normalised.contains("keys_to_snake_case") || normalised.contains("keysToSnakeCase") {
        shapes.push(KnownShape::KeysToSnakeCase(extract_target_field(
            normalised,
        )));
        return shapes;
    }

    // From here down: the matchers keyed on a vendor's FIELD NAMES rather than
    // on a Painless construct, plus the two catch-alls.

    // Pattern: CommandLine → process fields
    if normalised.contains("CommandLine") && normalised.contains("process") {
        shapes.push(KnownShape::CommandLine {
            parent: normalised.contains("ParentCommandLine"),
        });
        return shapes;
    }

    // Pattern: ProcessStartTime epoch → @timestamp or process.start
    if normalised.contains("ProcessStartTime") || normalised.contains("processStartTime") {
        shapes.push(KnownShape::ProcessStartTime);
        return shapes;
    }

    // Pattern: email split — splitOnToken("@") → user.email, user.domain, user.name
    // Used in Okta, O365, Azure, and many other sources
    if normalised.contains("splitOnToken") && normalised.contains('@') {
        shapes.push(KnownShape::EmailSplit);
        return shapes;
    }

    // Pattern: okta risk_behaviors extraction from flattened.behaviors
    // Extracts keys with value "POSITIVE" into an array
    if normalised.contains("POSITIVE") && normalised.contains("risk_behaviors") {
        shapes.push(KnownShape::RiskBehaviors);
        return shapes;
    }

    // Pattern: Azure category → event type/category mapping via params lookup
    if normalised.contains("activitylogs")
        && normalised.contains("category")
        && normalised.contains("params.get")
    {
        shapes.push(KnownShape::AzureCategoryEventType);
        return shapes;
    }

    // Pattern: Azure event_category assignment, in whichever module's subtree.
    if normalised.contains("event_category") && normalised.contains("eventCategory") {
        shapes.push(KnownShape::AzureEventCategory);
        return shapes;
    }

    // Pattern: replace dots in map keys (Azure identity claims)
    // Matches: ctx.temp_claims[key.replace('.', '_')] = ...
    if normalised.contains("replace('.'") && normalised.contains("keySet()") {
        shapes.push(KnownShape::ReplaceDotsInKeys);
        return shapes;
    }

    // Pattern: okta.target array key renames + user/group extraction
    // Renames alternateId→alternate_id, displayName→display_name in each element,
    // filters detailEntry, extracts first user/usergroup targets
    if normalised.contains("alternateId")
        && normalised.contains("alternate_id")
        && normalised.contains("okta")
    {
        shapes.push(KnownShape::OktaTargetRename);
        return shapes;
    }

    // Pattern: collect one nested key out of every entry of a map.
    if normalised.contains(".keySet()") && normalised.contains(".add(") {
        shapes.push(KnownShape::CollectMapValues);
        return shapes;
    }

    // Pattern: rewrite one substring of a field in place.
    if normalised.contains(".replace(") {
        shapes.push(KnownShape::GuardedReplace);
        return shapes;
    }

    // The two catch-alls below are shapes a longer script also CONTAINS, so
    // they run only after every structural matcher has declined.

    // Pattern: divide a number in place by a literal, under a guard.
    if normalised.contains(" / ")
        && !normalised.contains("params")
        && let Some(shape) = parse_guarded_divide(normalised)
    {
        shapes.push(shape);
        return shapes;
    }

    // Pattern: one field scaled by a literal into another.
    if normalised.contains(" * ")
        && !normalised.contains("params")
        && let Some(shape) = parse_scale_field(normalised)
    {
        shapes.push(KnownShape::ScaleField(Box::new(shape)));
        return shapes;
    }

    // Pattern: copy one field to another when the source is set.
    if normalised.contains("!= null") && !normalised.contains("for (") {
        shapes.push(KnownShape::GuardedCopy);
        return shapes;
    }

    // Running the statements a script writes that CAN be read, as a last
    // resort, was tried and is NOT here: it moved nothing and cost gcp eight
    // fields. A partial read writes a value where Elastic's whole script
    // would have written a different one, and the corpus says that is worse
    // than writing nothing.
    shapes
}

/// Run one matcher branch against one event.
///
/// Returns whether the script counts as HANDLED, with each branch's semantics
/// unchanged from the old inline dispatch: a guarded branch may decline, and
/// the caller then tries the next shape in the list.
#[allow(clippy::too_many_lines)] // One delegation arm per shape; it grows with the shape list.
pub(crate) fn run_known_shape(event: &mut Event, normalised: &str, shape: &KnownShape) -> bool {
    match shape {
        KnownShape::DropEmpty { policy, root } => {
            match root {
                None => drop_empty_recursive(event, policy),
                Some(path) => {
                    if let Some(subtree) = crate::painless_params::pointer_mut(event, path) {
                        drop_value(subtree, policy);
                    }
                }
            }
            true
        }
        KnownShape::SplitCommandLine(script) => {
            crate::painless_windows::run_argv_script(event, script)
        }
        KnownShape::Basename => try_basename_after_separator(event, normalised),
        KnownShape::FileInfo(source) => crate::painless_windows::run_file_info(event, source),
        KnownShape::HashLowercase(source) => {
            crate::painless_windows::run_hash_lowercase(event, source)
        }
        KnownShape::PrefixTail {
            source,
            prefix,
            strip_comma,
            target,
        } => try_prefix_tail(event, source, prefix, *strip_comma, target),
        KnownShape::PrependToArray {
            scalar,
            array,
            target,
        } => try_prepend_to_array(event, scalar, array, target),
        KnownShape::ResourcesRenameDedup(source) => run_resources_rename_dedup(event, source),
        KnownShape::SecurityhubResource(source) => run_securityhub_resource(event, source),
        KnownShape::CheckpointPackets => run_checkpoint_packets(event),
        KnownShape::ConsoleLoginEventData => run_console_login_event_data(event),
        KnownShape::BitFlagNames(decode) => run_bit_flag_names(event, decode),
        KnownShape::PascalKeys(shape) => run_pascal_keys(event, shape),
        KnownShape::SecondsToSpan(source) => run_seconds_to_span(event, source),
        KnownShape::SubstringBeforeLast {
            source,
            target,
            needle,
        } => run_substring_before_last(event, source, target, needle),
        KnownShape::TlsVersionSplit { source } => run_tls_version_split(event, source),
        KnownShape::ConcatParts(script) => run_concat_parts(event, script),
        KnownShape::TrimListInPlace(field) => run_trim_list(event, field),
        KnownShape::StartsWithAppend {
            source,
            prefix,
            target,
            value,
        } => run_starts_with_append(event, source, prefix, target, value),
        KnownShape::InspectorResources { multi } => run_inspector_resources(event, *multi),
        KnownShape::MailRelated(script) => run_mail_related(event, script),
        KnownShape::GcpRelatedEntity => run_gcp_related_entity(event),
        KnownShape::RemoveListValue { field, value } => run_remove_list_value(event, field, value),
        KnownShape::M365ProcessEvidence(path) => run_m365_process_evidence(event, path),
        KnownShape::M365IdentityEvidence(path) => run_m365_identity_evidence(event, path),
        KnownShape::Route53Answers => run_route53_answers(event),
        KnownShape::ReverseLookupAddress => run_reverse_lookup_address(event),
        KnownShape::TruthyAssignments(pairs) => run_truthy_assignments(event, pairs),
        KnownShape::LongOperationSession { first, last } => {
            run_long_operation_session(event, first, last)
        }
        KnownShape::ScriptBlockEntropy(source) => {
            crate::painless_windows::run_script_block_entropy(event, source)
        }
        KnownShape::SplitOnPipe(fields) => run_split_on_pipe(event, fields),
        KnownShape::ZipAttachments(zip) => run_zip_attachments(event, zip),
        KnownShape::ZipColumns(zip) => run_zip_columns(event, zip),
        KnownShape::MaxByContains(shape) => run_max_by_contains(event, shape),
        KnownShape::SecurityhubResources(source) => {
            if let Some(Value::Array(resources)) = event.get(source).cloned()
                && resources.len() > 1
            {
                run_securityhub_multi(event, &resources)
            } else {
                true
            }
        }
        KnownShape::CategoryTypeLadder(arms) => run_category_type_ladder(event, arms),
        KnownShape::KeysStripWhitespace(source) => {
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
        KnownShape::SnakeKeyMapCopy { source, target } => {
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
        KnownShape::StripAnglePairs { scalars, lists } => {
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
        KnownShape::NameValueFold {
            source,
            target,
            key_member,
            value_member,
        } => run_name_value_fold(event, source, target, key_member, value_member),
        KnownShape::OutcomeFromTags {
            action_field,
            tags,
            target,
        } => run_outcome_from_tags(event, action_field, tags, target),
        KnownShape::RenameCommonAuth(codes) => {
            crate::painless_windows::run_rename_common_auth(event, codes)
        }
        KnownShape::ProcessCreated(codes) => {
            crate::painless_windows::run_process_created(event, codes)
        }
        KnownShape::ShareFilePath(codes) => {
            crate::painless_windows::run_share_file_path(event, codes)
        }
        KnownShape::ObjectDn => crate::painless_windows::run_object_dn(event),
        KnownShape::CollectColumns(shape) => run_collect_columns(event, shape),
        KnownShape::SliceEachItem(shape) => run_slice_each_item(event, shape),
        KnownShape::ContainsLadder(shape) => run_contains_ladder(event, shape),
        KnownShape::SuffixAfterSeparator {
            source,
            target,
            separator,
        } => run_suffix_after_separator(event, source, target, separator),
        KnownShape::CopyTargetUser(codes) => {
            crate::painless_windows::run_copy_target_user(event, codes)
        }
        KnownShape::CopySubjectUser(codes) => {
            crate::painless_windows::run_copy_subject_user(event, codes)
        }
        KnownShape::CopyMemberName(codes) => {
            crate::painless_windows::run_copy_member_name(event, codes)
        }
        KnownShape::CopyComputerObject(codes) => {
            crate::painless_windows::run_copy_computer_object(event, codes)
        }
        KnownShape::CopyUserToBase {
            codes,
            base,
            sid_field,
        } => crate::painless_windows::run_copy_user_to_base(event, codes, base, sid_field),
        KnownShape::SplitPipeFields(fields) => run_split_pipe_fields(event, fields),
        KnownShape::SplitTokenField {
            source,
            separator,
            parse_int,
            target,
            head,
        } => run_split_token_field(event, source, *separator, *parse_int, target, *head),
        KnownShape::DecodeBase64 { source, target } => run_decode_base64(event, source, target),
        KnownShape::TokenCount {
            source,
            separator,
            target,
        } => run_token_count(event, source, separator, target),
        KnownShape::WrapValueInList { source, target } => {
            if let Some(value) = event.get(source).cloned() {
                let _ = event.set(target, Value::Array(vec![value]));
            }
            true
        }
        KnownShape::LastElement { array, target } => {
            if let Some(Value::Array(items)) = event.get(array)
                && let Some(last) = items.last().cloned()
            {
                let _ = event.set(target, last);
            }
            true
        }
        KnownShape::ClassifyMembers => try_classify_members(event, normalised),
        KnownShape::FlattenedDuplicates => try_flattened_duplicates(event, normalised),
        KnownShape::CollectEntities => try_collect_entities(event, normalised),
        KnownShape::DnsRdataAnswers => try_dns_rdata_answers(event, normalised),
        KnownShape::StructuredRdataAnswers => try_structured_rdata_answers(event),
        KnownShape::RelatedFromDnsAnswers => try_related_from_dns_answers(event),
        KnownShape::AnswersFromResolvedIp => try_answers_from_resolved_ip(event),
        KnownShape::CamelToSnake { target, source } => {
            if let Some(value) = event.get(source) {
                let converted = camel_map_to_snake(value);
                let _ = event.set(target, converted);
            }
            true
        }
        KnownShape::SplitTrimCollect => try_split_trim_collect(event, normalised),
        KnownShape::SumDirections(total) => try_sum_directions(event, total),
        KnownShape::SumOfFields => try_sum_of_fields(event, normalised),
        KnownShape::DurationToNanos => try_duration_to_nanos(event, normalised),
        KnownShape::FlowDuration => try_flow_duration(event, normalised),
        KnownShape::ParallelDispatch => try_parallel_dispatch(event, normalised),
        KnownShape::ConcatMessage => try_concat_message(event, normalised),
        KnownShape::SwapSubtrees => try_swap_subtrees(event, normalised),
        KnownShape::CollectingLadder => try_collecting_ladder(event, normalised),
        KnownShape::CaseInsensitiveLadder => try_case_insensitive_ladder(event, normalised),
        KnownShape::EqualityLadder(ladder) => try_ladder(event, ladder),
        KnownShape::SentinelRemovalLiteral => try_sentinel_removal_literal(event, normalised),
        KnownShape::RowLookupWithFallback => try_row_lookup_with_fallback(event, normalised),
        KnownShape::SchemelessUrl => try_schemeless_url(event),
        KnownShape::VersionSplit => try_version_split(event, normalised),
        KnownShape::SyslogPriority => try_syslog_priority(event, normalised),
        KnownShape::AppendUnique { from, into } => try_append_unique(event, from, into),
        KnownShape::SplitUnquotedKv(split) => run_split_unquoted_kv(event, split),
        KnownShape::ArrayToIndexedObject => try_array_to_indexed_object(event, normalised),
        KnownShape::KeyValuePairs => try_key_value_pairs(event, normalised),
        KnownShape::JoinOptional => try_join_optional(event, normalised),
        KnownShape::AppendEach => try_append_each(event, normalised),
        KnownShape::LiteralValueMap => try_literal_value_map(event, normalised),
        KnownShape::KeysToSnakeCase(field) => {
            if let Some(field) = field {
                if let Some(val) = event.get(field).cloned() {
                    let mut val = val;
                    keys_to_snake_case(&mut val);
                    let _ = event.set(field, val);
                }
            } else {
                // Apply to entire event
                let inner = event.as_value_mut();
                keys_to_snake_case(inner);
            }
            true
        }
        KnownShape::CommandLine { parent } => {
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
        KnownShape::ProcessStartTime => {
            let _ =
                epoch_to_timestamp(event, "crowdstrike.event.ProcessStartTime", "process.start");
            true
        }
        KnownShape::EmailSplit => try_email_split(event, normalised),
        KnownShape::RiskBehaviors => try_risk_behaviors(event),
        KnownShape::AzureCategoryEventType => try_azure_category_to_event_type(event),
        KnownShape::AzureEventCategory => try_azure_event_category(event, normalised),
        KnownShape::ReplaceDotsInKeys => try_replace_dots_in_keys(event, normalised),
        KnownShape::OktaTargetRename => try_okta_target_rename(event),
        KnownShape::CollectMapValues => try_collect_map_values(event, normalised),
        KnownShape::GuardedReplace => try_guarded_replace(event, normalised),
        KnownShape::ScaleField(shape) => run_scale_field(event, shape),
        KnownShape::NanosBetween(shape) => run_nanos_between(event, shape),
        KnownShape::ProcessNameFromCommandLine => run_process_name_from_command_line(event),
        KnownShape::FlagsPresent(shape) => run_flags_present(event, shape),
        KnownShape::ZipLists(shape) => run_zip_lists(event, shape),
        KnownShape::GuardedDivide {
            target,
            absent,
            divisor,
        } => run_guarded_divide(event, target, absent.as_ref(), *divisor),
        KnownShape::GuardedCopy => try_guarded_copy(event, normalised),
    }
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

    /// Verbatim from `pipelines/zscaler_zia/email_dlp/default.yml`: the vendor
    /// ships its columns pipe-delimited in one string each.
    #[test]
    fn pipe_delimited_columns_split_in_place() {
        let script = "void splitStr(Map m, String key) {\n  if (m == null || key == null) return;\n  \
            def v = m.get(key);\n  if (!(v instanceof String)) return;\n  String s = (String) v;\n  \
            if (s.length() == 0) return;\n  List out = new ArrayList();\n  \
            out.add(s.substring(from, n));\n  m.put(key, out);\n}\n\
            def ed = ctx.zscaler_zia?.email_dlp;\nif (ed == null) return;\n\
            splitStr(ed, 'severity');\nif (ed.dlp instanceof Map) {\n  \
            splitStr(ed.dlp, 'dict_names');\n}\n";

        let mut event = Event::new(json!({ "zscaler_zia": { "email_dlp": {
            "severity": "a|b", "dlp": { "dict_names": "Credit Cards|SSN" }
        }}}));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get("zscaler_zia.email_dlp.severity"),
            Some(&json!(["a", "b"]))
        );
        assert_eq!(
            event.get("zscaler_zia.email_dlp.dlp.dict_names"),
            Some(&json!(["Credit Cards", "SSN"]))
        );
    }

    /// Verbatim from the same pipeline: names and counts zipped, and the
    /// vendor's `None` placeholder dropping the whole entry.
    #[test]
    fn named_counts_zip_and_skip_the_placeholder() {
        let script = "def dlp = ctx.zscaler_zia.email_dlp.dlp;\n\
            def names = dlp.dict_names instanceof List ? dlp.dict_names : null;\n\
            def counts = dlp.dict_counts instanceof List ? dlp.dict_counts : null;\n\
            if (names == null) return;\ndef out = new ArrayList();\n\
            for (int i = 0; i < names.size(); i++) {\n  def name = names.get(i);\n  \
            if (!(name instanceof String) || name == '' || name == 'None') continue;\n  \
            def item = new HashMap();\n  item.put('name', name);\n  \
            if (counts != null && i < counts.size()) item.put('count', counts.get(i));\n  \
            out.add(item);\n}\n\
            if (out.size() > 0) ctx.zscaler_zia.email_dlp.dlp.dictionaries = out;\n";

        let mut event = Event::new(json!({ "zscaler_zia": { "email_dlp": { "dlp": {
            "dict_names": ["Credit Cards", "None", "SSN"], "dict_counts": [7, 0, 3]
        }}}}));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get("zscaler_zia.email_dlp.dlp.dictionaries"),
            Some(&json!([
                { "name": "Credit Cards", "count": 7 },
                { "name": "SSN", "count": 3 }
            ]))
        );
    }

    /// Verbatim from the same pipeline: parallel columns zipped one entry per
    /// index, the md5 nested a level deeper than the rest.
    #[test]
    fn parallel_attachment_columns_zip_into_one_list() {
        let script = "def att = ctx.zscaler_zia.email_dlp.email.attachments;\n\
            def names = att.file_names instanceof List ? att.file_names : null;\n\
            def md5s = att.md5s instanceof List ? att.md5s : null;\n\
            def types = att.file_types instanceof List ? att.file_types : null;\n\
            int n = 0;\nif (names != null && names.size() > n) n = names.size();\n\
            if (n == 0) return;\nif (ctx.email == null) ctx.email = [:];\n\
            if (ctx.email.attachments == null) ctx.email.attachments = new ArrayList();\n\
            for (int i = 0; i < n; i++) {\n  def file = new HashMap();\n  \
            if (names != null && i < names.size()) file.put('name', names.get(i));\n  \
            if (md5s != null && i < md5s.size()) {\n    def hash = new HashMap();\n    \
            hash.put('md5', md5s.get(i));\n    file.put('hash', hash);\n  }\n  \
            if (types != null && i < types.size()) file.put('extension', types.get(i));\n  \
            def item = new HashMap();\n  item.put('file', file);\n  \
            ctx.email.attachments.add(item);\n}\n";

        let mut event = Event::new(json!({ "zscaler_zia": { "email_dlp": { "email": {
            "attachments": {
                "file_names": ["a.pdf"], "md5s": ["d41d8"], "file_types": ["pdf"]
            }
        }}}}));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get("email.attachments"),
            Some(&json!([{ "file": {
                "name": "a.pdf", "hash": { "md5": "d41d8" }, "extension": "pdf"
            }}]))
        );
    }

    /// Verbatim from `pipelines/aws/inspector/default.yml`: the ladder writes
    /// through `.put()` and compares a local the script LOWER-CASED, so the
    /// fold has to travel with the subject.
    #[test]
    fn a_case_folded_ladder_writes_through_put() {
        let script = "String severity = ctx.aws.inspector.severity.toLowerCase();\n\
            if (severity == 'untriaged') {\n  ctx.vulnerability.put('severity', 'Unknown');\n\
            } else if (severity == 'informational') {\n\
            ctx.vulnerability.put('severity', 'Low');\n\
            } else if (severity == 'high') {\n  ctx.vulnerability.put('severity', 'High');\n}";

        let mut event = Event::new(json!({ "aws": { "inspector": { "severity": "HIGH" }}}));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_str("vulnerability.severity"), Some("High"));

        // A value no arm names writes nothing.
        let mut other = Event::new(json!({ "aws": { "inspector": { "severity": "none" }}}));
        assert!(try_known_painless(&mut other, script));
        assert!(other.get("vulnerability.severity").is_none());
    }

    /// Verbatim from `pipelines/mimecast/message_release_logs/default.yml`:
    /// every display name and address the script names, split at the `@` and
    /// sorted, with the operator object flattened to its address first.
    #[test]
    fn the_mail_related_sets_collect_and_sort() {
        let script = "def splitmail(String email) {\n\
            String[] parts = email.splitOnToken(\"@\");\n\
            if (parts.length != 2) {\n  return null;\n}\n  return parts;\n}\n\
            def users = new HashSet();\ndef hosts = new HashSet();\n\
            if (ctx.mimecast?.fromEnv?.displayableName != null) {\n\
            users.add(ctx.mimecast.fromEnv.displayableName);\n}\n\
            if (ctx.mimecast?.operator instanceof Map) {\n\
            ctx.mimecast.operator = ctx.mimecast.operator.emailAddress;\n}\n\
            if (ctx.mimecast?.operator != null) {\n\
            def parts = splitmail(ctx.mimecast.operator);\n\
            if (parts != null) {\n  users.add(parts[0]);\n  hosts.add(parts[1]);\n}\n\
            users.add(ctx.mimecast.operator);\n}\n\
            for (def to: ctx.mimecast.to) {\n\
            if (to.displayableName != null) {\n  users.add(to.displayableName);\n}\n\
            if (to.emailAddress != null) {\n\
            def parts = splitmail(to.emailAddress);\n\
            if (parts != null) {\n  users.add(parts[0]);\n  hosts.add(parts[1]);\n}\n\
            users.add(to.emailAddress);\n}\n}\n\
            if (users.size() != 0 || hosts.size() != 0) {\n\
            if (ctx.related == null) {\n  ctx.related = new HashMap();\n}\n\
            if (users.size() != 0 && ctx.related.user == null) {\n\
            ctx.related.user = new ArrayList();\n\
            for (def u: users) {\n  ctx.related.user.add(u);\n}\n\
            Collections.sort(ctx.related.user);\n}\n\
            if (hosts.size() != 0 && ctx.related.hosts == null) {\n\
            ctx.related.hosts = new ArrayList();\n\
            for (def h: hosts) {\n  ctx.related.hosts.add(h);\n}\n\
            Collections.sort(ctx.related.hosts);\n}\n}";

        let mut event = Event::new(json!({
            "mimecast": {
                "fromEnv": { "displayableName": "FromName LastName" },
                "operator": { "emailAddress": "admin@domain.tld" },
                "to": [{ "displayableName": "ToName LastName", "emailAddress": "to_user@to_domain.tld" }],
            },
        }));
        assert!(try_known_painless(&mut event, script));

        assert_eq!(
            event.get("mimecast.operator"),
            Some(&json!("admin@domain.tld"))
        );
        assert_eq!(
            event.get("related.user"),
            Some(&json!([
                "FromName LastName",
                "ToName LastName",
                "admin",
                "admin@domain.tld",
                "to_user",
                "to_user@to_domain.tld"
            ]))
        );
        assert_eq!(
            event.get("related.hosts"),
            Some(&json!(["domain.tld", "to_domain.tld"]))
        );
    }

    /// Verbatim from `pipelines/aws/cloudfront_logs/default.yml`: the split
    /// x-forwarded-for is trimmed where it sits, because the grok that reads
    /// each member next is anchored and a leading space fails it.
    #[test]
    fn a_split_list_is_trimmed_in_place() {
        let script = "for (int i = 0; i < ctx._tmp.split_x_forwarded_for.length; i++) {\n\
            ctx._tmp.split_x_forwarded_for[i] = ctx._tmp.split_x_forwarded_for[i].trim();\n}";
        let mut event = Event::new(json!({
            "_tmp": { "split_x_forwarded_for": ["81.2.69.142", " 216.160.83.56"] },
        }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get("_tmp.split_x_forwarded_for"),
            Some(&json!(["81.2.69.142", "216.160.83.56"]))
        );
    }

    /// Its sibling: `localhost:8080` is not an address any grok reads, so the
    /// script appends the loopback for each member that carries it.
    #[test]
    fn a_prefixed_member_appends_the_scripts_constant() {
        let script = "if (ctx.get('network') == null) {\n  ctx['network'] = new HashMap();\n}\n\
            for (String item : ctx._tmp.split_x_forwarded_for ) {\n\
            if (item.startsWith('localhost')) {\n\
            if (ctx.network.forwarded_ip == null) {\n\
            ctx['network']['forwarded_ip'] = new ArrayList();\n}\n\
            ctx['network']['forwarded_ip'].add('127.0.0.1');\n}\n}";

        let mut event = Event::new(json!({
            "_tmp": { "split_x_forwarded_for": ["localhost:8080"] },
        }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get("network.forwarded_ip"),
            Some(&json!(["127.0.0.1"]))
        );

        // No member with the prefix leaves the list untouched.
        let mut plain = Event::new(json!({
            "_tmp": { "split_x_forwarded_for": ["81.2.69.142"] },
        }));
        assert!(try_known_painless(&mut plain, script));
        assert!(plain.get("network.forwarded_ip").is_none());
    }

    /// Verbatim from `pipelines/aws/cloudfront_logs/default.yml`: the URL is
    /// reassembled out of whichever parts the log line carried.
    #[test]
    fn the_url_is_concatenated_from_the_parts_that_are_there() {
        let script = "def full = \"\";\n\
            if(ctx.network?.protocol != null && ctx.network?.protocol != \"\") {\n\
            full += ctx.network.protocol+\"://\";\n}\n\
            if(ctx.destination?.domain != null && ctx.destination?.domain != \"\") {\n\
            full += ctx.destination.domain;\n}\n\
            if(ctx.url?.path != null && ctx.url?.path != \"\") {\n  full += ctx.url.path;\n}\n\
            if(ctx.url?.query != null && ctx.url?.query != \"\") {\n\
            full += \"?\"+ctx.url.query;\n}\n\
            if(full != \"\") {\n  ctx._tmp.url_full = full\n}";

        let mut event = Event::new(json!({
            "network": { "protocol": "https" },
            "destination": { "domain": "test.com" },
            "url": { "path": "/getApplications", "query": "source=global" },
        }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get("_tmp.url_full"),
            Some(&json!("https://test.com/getApplications?source=global"))
        );

        // A missing part takes its whole clause -- the `?` with the query.
        let mut partial = Event::new(json!({
            "network": { "protocol": "http" },
            "destination": { "domain": "www.example.com" },
            "url": { "path": "/" },
        }));
        assert!(try_known_painless(&mut partial, script));
        assert_eq!(
            partial.get("_tmp.url_full"),
            Some(&json!("http://www.example.com/"))
        );

        // Nothing to build from writes nothing.
        let mut empty = Event::new(json!({}));
        assert!(try_known_painless(&mut empty, script));
        assert!(empty.get("_tmp.url_full").is_none());
    }

    /// Verbatim from `pipelines/aws/elb_logs/default.yml`: the network load
    /// balancer writes `tlsv12`, which is TLS 1.2.
    #[test]
    fn the_tls_token_splits_into_protocol_and_version() {
        let script = "def parts = ctx.aws.elb.ssl_protocol.splitOnToken(\"v\");\n\
            if (parts.length != 2) {\n  return;\n}\n\
            if (parts[1].contains(\".\")) {\n  ctx.tls.version = parts[1];\n} else {\n\
            ctx.tls.version = parts[1].substring(0,1) + \".\" + parts[1].substring(1);\n}\n\
            ctx.tls.version_protocol = parts[0].toLowerCase();";

        let mut event = Event::new(json!({ "aws": { "elb": { "ssl_protocol": "tlsv12" }}}));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get("tls.version"), Some(&json!("1.2")));
        assert_eq!(event.get("tls.version_protocol"), Some(&json!("tls")));

        // A token that does not split in two is the script's own early return.
        let mut plain = Event::new(json!({ "aws": { "elb": { "ssl_protocol": "-" }}}));
        assert!(try_known_painless(&mut plain, script));
        assert!(plain.get("tls.version").is_none());
    }

    /// Verbatim from `pipelines/aws/ec2_metrics/default.yml`: a `CloudWatch`
    /// percentage becomes a fraction, but ONLY where the agent has not already
    /// written the fraction itself. Read as a guarded copy, the second clause
    /// of the guard was taken as the target and the raw 42 was written over
    /// the agent's 0.421.
    #[test]
    fn a_percentage_divides_in_place_unless_the_agent_beat_it_to_it() {
        let script = "if(ctx.aws?.ec2?.metrics?.CPUUtilization?.avg != null \
             && ctx.host?.cpu?.usage == null) {\n    \
             ctx.aws.ec2.metrics.CPUUtilization.avg = \
             ctx.aws.ec2.metrics.CPUUtilization.avg / 100;\n}\n";

        // Firehose ships the percentage and nothing else.
        let mut firehose = Event::new(json!({
            "aws": {"ec2": {"metrics": {"CPUUtilization": {"avg": 21.96}}}}
        }));
        assert!(try_known_painless(&mut firehose, script));
        assert_eq!(
            firehose.get("aws.ec2.metrics.CPUUtilization.avg"),
            Some(&json!(21.96 / 100.0))
        );

        // The agent has already computed it, so the metric is left alone.
        let mut agent = Event::new(json!({
            "aws": {"ec2": {"metrics": {"CPUUtilization": {"avg": 42}}}},
            "host": {"cpu": {"usage": 0.421}},
        }));
        assert!(try_known_painless(&mut agent, script));
        assert_eq!(
            agent.get("aws.ec2.metrics.CPUUtilization.avg"),
            Some(&json!(42))
        );
        assert_eq!(agent.get("host.cpu.usage"), Some(&json!(0.421)));
    }

    /// Verbatim from `pipelines/aws/cloudtrail/default.yml`. `MobileVersion`
    /// and `MFAUsed` are read as `!= 'No'`, so the string becomes a boolean;
    /// `LoginTo` is carried as it stands.
    #[test]
    fn console_login_extras_become_two_booleans_and_a_url() {
        let script = "if (ctx.json?.eventName != 'ConsoleLogin') {\n  return;\n}\n\
             Map aed_map = [:];\n\
             if (ctx.json?.additionalEventData?.MobileVersion != null) {\n  \
             aed_map.mobile_version = ctx.json.additionalEventData.MobileVersion != 'No';\n}\n\
             if (ctx.json?.additionalEventData?.LoginTo != null) {\n  \
             aed_map.login_to = ctx.json.additionalEventData.LoginTo;\n}\n\
             if (ctx.json?.additionalEventData?.MFAUsed != null) {\n  \
             aed_map.mfa_used = ctx.json.additionalEventData.MFAUsed != 'No';\n}\n\
             if (aed_map.size() > 0) {\n  \
             ctx.aws.cloudtrail.console_login = [:];\n  \
             ctx.aws.cloudtrail.console_login.additional_eventdata = aed_map;\n}";

        let mut event = Event::new(json!({"json": {
            "eventName": "ConsoleLogin",
            "additionalEventData": {
                "MobileVersion": "No",
                "LoginTo": "https://console.aws.amazon.com/s3/",
                "MFAUsed": "No",
            },
        }}));
        assert!(try_known_painless(&mut event, script));
        let base = "aws.cloudtrail.console_login.additional_eventdata";
        assert_eq!(
            event.get(&format!("{base}.mobile_version")),
            Some(&json!(false))
        );
        assert_eq!(event.get(&format!("{base}.mfa_used")), Some(&json!(false)));
        assert_eq!(
            event.get_str(&format!("{base}.login_to")),
            Some("https://console.aws.amazon.com/s3/")
        );

        // Anything that is not the literal `No` is true, and another event
        // name writes nothing at all.
        let mut yes = Event::new(json!({"json": {
            "eventName": "ConsoleLogin",
            "additionalEventData": {"MFAUsed": "Yes"},
        }}));
        assert!(try_known_painless(&mut yes, script));
        assert_eq!(yes.get(&format!("{base}.mfa_used")), Some(&json!(true)));

        let mut other = Event::new(json!({"json": {
            "eventName": "AssumeRole",
            "additionalEventData": {"MFAUsed": "Yes"},
        }}));
        assert!(try_known_painless(&mut other, script));
        assert!(!other.has("aws.cloudtrail.console_login"));
    }

    /// Verbatim from `pipelines/m365_defender/event/pipeline_device.yml`: the
    /// API name is whatever comes before the `ApiCall` marker, found without
    /// regard to case but cut out of the ORIGINAL text.
    #[test]
    fn the_text_before_a_case_insensitive_marker_is_taken() {
        let script = "String actiontype = ctx.m365_defender.event.action.type;\n\
             def idx = actiontype.toLowerCase().lastIndexOf('apicall');\n\
             ctx._temp_process_Ext_api_name = actiontype.substring(0, idx);\n";

        let mut event = Event::new(json!({
            "m365_defender": {"event": {"action": {"type": "ReadProcessMemoryApiCall"}}}
        }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get_str("_temp_process_Ext_api_name"),
            Some("ReadProcessMemory")
        );

        // No marker means `substring(0, -1)`, which throws in Java, so
        // nothing is written and the rename behind it finds no field.
        let mut plain = Event::new(json!({
            "m365_defender": {"event": {"action": {"type": "SomethingElse"}}}
        }));
        assert!(try_known_painless(&mut plain, script));
        assert!(!plain.has("_temp_process_Ext_api_name"));
    }

    /// Verbatim from `pipelines/proofpoint_on_demand/message/default.yml`. A
    /// fractional second becomes nanoseconds and closes the span the start
    /// opened, rendered the way Java prints a `ZonedDateTime` -- three, six or
    /// nine fractional digits, never a partial group.
    #[test]
    fn a_fractional_second_closes_the_span_it_opens() {
        let script = "ctx.event.duration = (int) (ctx.proofpoint_on_demand.message.filter\
             .duration_secs * 1000000000);\nif (ctx.event?.start != null) {\n  \
             ZonedDateTime start = ZonedDateTime.parse(ctx.event.start);\n  \
             ctx.event.end = start.plus(ctx.event.duration, ChronoUnit.NANOS);\n}\n";

        let mut event = Event::new(json!({
            "event": { "start": "2020-02-07T16:34:49.929Z" },
            "proofpoint_on_demand": {"message": {"filter": {"duration_secs": 0.286_712}}},
        }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get("event.duration"), Some(&json!(286_712_000)));
        assert_eq!(
            event.get_str("event.end"),
            Some("2020-02-07T16:34:50.215712Z")
        );

        // Landing on a whole second prints NO fraction, which is Java's own
        // rendering and not a rounding of ours.
        let mut whole = Event::new(json!({
            "event": { "start": "2020-02-07T16:34:49.929Z" },
            "proofpoint_on_demand": {"message": {"filter": {"duration_secs": 0.071}}},
        }));
        assert!(try_known_painless(&mut whole, script));
        assert_eq!(whole.get_str("event.end"), Some("2020-02-07T16:34:50Z"));

        // A whole number of milliseconds prints three digits, not six.
        let mut millis = Event::new(json!({
            "event": { "start": "2020-02-07T16:34:49.929Z" },
            "proofpoint_on_demand": {"message": {"filter": {"duration_secs": 0.5}}},
        }));
        assert!(try_known_painless(&mut millis, script));
        assert_eq!(
            millis.get_str("event.end"),
            Some("2020-02-07T16:34:50.429Z")
        );

        // No start means no span, and the duration is still written.
        let mut startless = Event::new(json!({
            "proofpoint_on_demand": {"message": {"filter": {"duration_secs": 1.5}}},
        }));
        assert!(try_known_painless(&mut startless, script));
        assert_eq!(startless.get("event.duration"), Some(&json!(1_500_000_000)));
        assert!(!startless.has("event.end"));
    }

    /// Verbatim from `pipelines/m365_defender/vulnerability/default.yml`. The
    /// defender exports disagree with themselves about casing, and every
    /// `camelCase` event produced nothing at all while this went unmatched --
    /// two of six in both `m365_defender` and `microsoft_defender_endpoint`.
    #[test]
    fn camel_keys_fold_to_pascal_with_the_os_exception() {
        let script = "Map normalized = new HashMap();\n\
             for (entry in ctx.json.entrySet()) {\n  \
             String key = entry.getKey();\n  String newKey;\n  \
             if (key.startsWith(\"os\") && key.length() > 2 \
             && Character.isUpperCase(key.charAt(2))) {\n    \
             newKey = \"OS\" + key.substring(2);\n  } else {\n    \
             newKey = key.substring(0, 1).toUpperCase() + key.substring(1);\n  }\n  \
             normalized.put(newKey, entry.getValue());\n}\nctx.json = normalized;\n";

        let mut event = Event::new(json!({"json": {
            "cveId": "CVE-2024-9143",
            "osPlatform": "Linux",
            "osVersion": "enterprise_linux_9.4",
            "deviceId": "cccccccccccccc",
            "isOnboarded": true,
        }}));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_str("json.CveId"), Some("CVE-2024-9143"));
        assert_eq!(event.get_str("json.OSPlatform"), Some("Linux"));
        assert_eq!(
            event.get_str("json.OSVersion"),
            Some("enterprise_linux_9.4")
        );
        assert_eq!(event.get_str("json.DeviceId"), Some("cccccccccccccc"));
        assert_eq!(event.get("json.IsOnboarded"), Some(&json!(true)));

        // A key that is already PascalCase survives it, which is what lets one
        // pipeline take both spellings.
        let mut pascal = Event::new(json!({"json": {
            "CveId": "CVE-2022-49226", "OSPlatform": "Linux", "Other": 1,
        }}));
        assert!(try_known_painless(&mut pascal, script));
        assert_eq!(pascal.get_str("json.CveId"), Some("CVE-2022-49226"));
        assert_eq!(pascal.get_str("json.OSPlatform"), Some("Linux"));
        assert_eq!(pascal.get("json.Other"), Some(&json!(1)));
    }

    /// Verbatim from `pipelines/aws/vpcflow/default.yml`: the six TCP flags,
    /// in the order the script tests them. `aws/firewall_logs` ships the same
    /// ladder with double quotes.
    #[test]
    fn a_flag_word_decodes_into_its_names() {
        let script = "if (ctx.aws.vpcflow.tcp_flags_array == null) {\n  \
             ArrayList al = new ArrayList();\n  \
             ctx.aws.vpcflow.put(\"tcp_flags_array\", al);\n}\n\n\
             def flags = Integer.parseUnsignedInt(ctx.aws.vpcflow.tcp_flags);\n\n\
             if ((flags & 0x01) != 0) {\n  ctx.aws.vpcflow.tcp_flags_array.add('fin');\n}\n\
             if ((flags & 0x02) != 0) {\n  ctx.aws.vpcflow.tcp_flags_array.add('syn');\n}\n\
             if ((flags & 0x04) != 0) {\n  ctx.aws.vpcflow.tcp_flags_array.add('rst');\n}\n\
             if ((flags & 0x08) != 0) {\n  ctx.aws.vpcflow.tcp_flags_array.add('psh');\n}\n\
             if ((flags & 0x10) != 0) {\n  ctx.aws.vpcflow.tcp_flags_array.add('ack');\n}\n\
             if ((flags & 0x20) != 0) {\n  ctx.aws.vpcflow.tcp_flags_array.add('urg');\n}\n";

        // SYN alone.
        let mut syn = Event::new(json!({"aws": {"vpcflow": {"tcp_flags": "2"}}}));
        assert!(try_known_painless(&mut syn, script));
        assert_eq!(
            syn.get("aws.vpcflow.tcp_flags_array"),
            Some(&json!(["syn"]))
        );

        // SYN + ACK, in the script's own test order rather than the input's.
        let mut synack = Event::new(json!({"aws": {"vpcflow": {"tcp_flags": "18"}}}));
        assert!(try_known_painless(&mut synack, script));
        assert_eq!(
            synack.get("aws.vpcflow.tcp_flags_array"),
            Some(&json!(["syn", "ack"]))
        );

        // No bits set writes an empty list, which is what the script's own
        // `new ArrayList()` leaves behind.
        let mut none = Event::new(json!({"aws": {"vpcflow": {"tcp_flags": "0"}}}));
        assert!(try_known_painless(&mut none, script));
        assert_eq!(none.get("aws.vpcflow.tcp_flags_array"), Some(&json!([])));
    }

    /// Verbatim from `pipelines/aws/waf/default.yml`: the same name/value fold
    /// proofpoint writes with `put`, spelled as an indexed loop over a
    /// subscripted map.
    #[test]
    fn an_indexed_name_value_loop_folds_into_a_map() {
        let script = "if (ctx.json.httpRequest.headers != null) {\n  \
             ctx.aws.waf.request = new HashMap();\n  \
             ctx.aws.waf.request.headers = new HashMap();\n  \
             for (def i = 0; i < ctx.json.httpRequest.headers.length; i++) {\n    \
             ctx.aws.waf.request.headers[ctx.json.httpRequest.headers[i].name] = \
             ctx.json.httpRequest.headers[i].value;\n  }\n}";

        let mut event = Event::new(json!({
            "json": { "httpRequest": { "headers": [
                {"name": "Host", "value": "localhost:1989"},
                {"name": "User-Agent", "value": "curl/7.61.1"},
                {"name": "Accept", "value": "*/*"},
            ]}}
        }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get_str("aws.waf.request.headers.Host"),
            Some("localhost:1989")
        );
        assert_eq!(
            event.get_str("aws.waf.request.headers.User-Agent"),
            Some("curl/7.61.1")
        );
        assert_eq!(event.get_str("aws.waf.request.headers.Accept"), Some("*/*"));
    }

    /// Verbatim from `pipelines/aws/s3access/default.yml`, which lowercases
    /// before the split. Taking everything before `.splitOnToken(` read
    /// `toLowerCase()` as a segment of the field's path.
    #[test]
    fn a_lowercased_tls_token_still_names_its_field() {
        let script = "def parts = ctx.aws.s3access.tls_version.toLowerCase().splitOnToken(\"v\");\n\
            if (parts.length != 2) {\n  return;\n}\n\
            ctx.tls.version = parts[1];\n\
            ctx.tls.version_protocol = parts[0]";

        let mut event = Event::new(json!({
            "aws": { "s3access": { "tls_version": "TLSv1.2" }},
            "tls": { "cipher": "ECDHE-RSA-AES128-GCM-SHA256" },
        }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_str("tls.version"), Some("1.2"));
        assert_eq!(event.get_str("tls.version_protocol"), Some("tls"));
    }

    /// Verbatim from `pipelines/cisco/umbrella/default.yml`: the identities
    /// dance's first half, the tail of one field past another's length.
    #[test]
    fn prefix_tail_reads_its_three_paths_and_strips_the_comma() {
        let script = "String identities_tail = ctx.cisco.umbrella.identities.substring(ctx.cisco.umbrella.identity.length());\n\
            if (identities_tail.startsWith(',')) {\n  identities_tail = identities_tail.substring(1);\n}\n\
            if (ctx.cisco.umbrella._tmp == null) {\n  ctx.cisco.umbrella._tmp = new HashMap();\n}\n\
            ctx.cisco.umbrella._tmp.identities_tail = identities_tail;";
        let mut event = Event::new(json!({
            "cisco": { "umbrella": {
                "identity": "Last, First (f.last@example.com)",
                "identities": "Last, First (f.last@example.com),HOSTNAME1",
            }},
        }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get("cisco.umbrella._tmp.identities_tail"),
            Some(&json!("HOSTNAME1"))
        );
    }

    /// The dance's second half: the scalar prepended to the split tail.
    #[test]
    fn prepend_to_array_rebuilds_the_list() {
        let script = "def identities = new ArrayList();\n\
            identities.add(ctx.cisco.umbrella.identity);\n\
            for (identity in ctx.cisco.umbrella._tmp.identities_tail) {\n  identities.add(identity);\n}\n\
            ctx.cisco.umbrella._tmp.identities = identities;";
        let mut event = Event::new(json!({
            "cisco": { "umbrella": {
                "identity": "Last, First (f.last@example.com)",
                "_tmp": { "identities_tail": ["HOSTNAME1", "HOSTNAME2"] },
            }},
        }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get("cisco.umbrella._tmp.identities"),
            Some(&json!([
                "Last, First (f.last@example.com)",
                "HOSTNAME1",
                "HOSTNAME2"
            ]))
        );
    }

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

    /// Verbatim from `pipelines/zscaler_zia/dns/default.yml`, folded onto one
    /// line the way YAML's `>-` delivers it. The target comes from the
    /// assignment that owns the multiply, not the first `=` -- that one is the
    /// `==` of the guard, and reading it wrote nothing at all.
    #[test]
    fn a_scaled_field_lands_on_its_own_target() {
        let script = "if (ctx.event == null) { ctx.put('event', new HashMap()); } \
             ctx.event.duration = ctx.zscaler_zia.dns.duration.milliseconds * 1000000;";

        let mut event = Event::new(json!({
            "zscaler_zia": {"dns": {"duration": {"milliseconds": 1000}}}
        }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_i64("event.duration"), Some(1_000_000_000));
        assert_eq!(
            event.get_i64("zscaler_zia.dns.duration.milliseconds"),
            Some(1000),
            "the source is read, not consumed"
        );

        // An absent source writes nothing rather than a zero.
        let mut empty = Event::new(json!({}));
        assert!(try_known_painless(&mut empty, script));
        assert!(!empty.has("event.duration"));
    }

    /// Verbatim from `pipelines/m365_defender/event/pipeline_alert.yml`.
    #[test]
    fn an_attack_technique_splits_into_a_name_and_an_id() {
        let script = "def subtechnique_name = new ArrayList();\n\
             def subtechnique_id = new ArrayList();\n\
             if (!(ctx.threat instanceof HashMap)) {\n  ctx.threat = new HashMap();\n}\n\
             for (item in ctx.m365_defender.event.attack_techniques) {\n\
             subtechnique_name.add(item.substring(0,item.lastIndexOf(' ')));\n\
             subtechnique_id.add(item.substring(item.indexOf('(')+1,item.indexOf(')')));\n}\n\
             ctx.threat.technique.subtechnique.id = subtechnique_id;\n\
             ctx.threat.technique.subtechnique.name = subtechnique_name;\n";

        let mut event = Event::new(json!({ "m365_defender": { "event": {
            "attack_techniques": ["Valid Accounts (T1078)", "Cloud Accounts (T1078.004)"]
        }}}));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get("threat.technique.subtechnique.name"),
            Some(&json!(["Valid Accounts", "Cloud Accounts"])),
            "the loop's own order -- the pipeline sorts both lists afterwards"
        );
        assert_eq!(
            event.get("threat.technique.subtechnique.id"),
            Some(&json!(["T1078", "T1078.004"]))
        );

        // An item with no bracket throws in Painless, so nothing is written.
        let mut ragged = Event::new(json!({ "m365_defender": { "event": {
            "attack_techniques": ["Valid Accounts (T1078)", "no id here"]
        }}}));
        assert!(try_known_painless(&mut ragged, script));
        assert!(!ragged.has("threat.technique.subtechnique.id"));
    }

    /// Verbatim from
    /// `pipelines/microsoft_defender_endpoint/machine_action/default.yml`.
    #[test]
    fn a_hash_lands_on_the_field_its_type_names() {
        let script = "ctx.file = ctx.file ?: [:]; ctx.file.hash = ctx.file.hash ?: [:]; \
             String fileType = ctx.mde.related_file_info.file_identifier_type.toLowerCase(); \
             String fileHash = ctx.mde.related_file_info.file_identifier; \
             if (fileType.contains('sha1')) {\n  ctx.file.hash.sha1 = fileHash;\n\
             } else if (fileType.contains('md5')) {\n  ctx.file.hash.md5 = fileHash;\n\
             } else if (fileType.contains('sha256')) {\n  ctx.file.hash.sha256 = fileHash;\n}\n";

        // `Sha1` is mixed case on the wire and the script lower-cases it.
        let mut event = Event::new(json!({ "mde": { "related_file_info": {
            "file_identifier": "aaf4c61d", "file_identifier_type": "Sha1"
        }}}));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_str("file.hash.sha1"), Some("aaf4c61d"));
        assert!(!event.has("file.hash.md5"));

        // A type no arm names writes nothing.
        let mut unknown = Event::new(json!({ "mde": { "related_file_info": {
            "file_identifier": "abc", "file_identifier_type": "Crc32"
        }}}));
        assert!(try_known_painless(&mut unknown, script));
        assert!(!unknown.has("file.hash"));
    }

    /// Verbatim from `pipelines/entityanalytics_entra_id/entity/user.yml`.
    #[test]
    fn direct_reports_become_one_column_per_member() {
        let script = "def ids = new ArrayList();\ndef names = new ArrayList();\n\
             def emails = new ArrayList();\n\
             for (def report : ctx.entityanalytics_entra_id.user.direct_reports) {\n\
             if (report == null) { continue; }\n  if (report.id != null) { ids.add(report.id); }\n\
             if (report.user_principal_name != null) { names.add(report.user_principal_name); }\n\
             if (report.mail != null) { emails.add(report.mail); }\n}\n\
             def userObj = new HashMap();\nif (!ids.isEmpty()) { userObj.put(\"id\", ids); }\n\
             if (!names.isEmpty()) { userObj.put(\"name\", names); }\n\
             if (!emails.isEmpty()) { userObj.put(\"email\", emails); }\n\
             if (!userObj.isEmpty()) {\n  ctx.user = ctx.user ?: new HashMap();\n\
             ctx.user.entity = ctx.user.entity ?: new HashMap();\n\
             ctx.user.entity.relationships = ctx.user.entity.relationships ?: new HashMap();\n\
             ctx.user.entity.relationships.put(\"supervises\", [\"user\": userObj]);\n}\n";

        let mut event = Event::new(json!({ "entityanalytics_entra_id": { "user": {
            "direct_reports": [
                {"id": "ee55", "mail": "one@example.com", "user_principal_name": "one@example.com"},
                {"id": "ff66", "mail": "two@example.com", "user_principal_name": "two@example.com"}
            ]
        }}}));
        assert!(try_known_painless(&mut event, script));

        assert_eq!(
            event.get("user.entity.relationships.supervises.user.id"),
            Some(&json!(["ee55", "ff66"]))
        );
        assert_eq!(
            event.get("user.entity.relationships.supervises.user.email"),
            Some(&json!(["one@example.com", "two@example.com"]))
        );

        // A member no report carries leaves its column out entirely.
        let mut sparse = Event::new(json!({ "entityanalytics_entra_id": { "user": {
            "direct_reports": [{"id": "ee55"}]
        }}}));
        assert!(try_known_painless(&mut sparse, script));
        assert_eq!(
            event.get("user.entity.relationships.supervises.user.id"),
            Some(&json!(["ee55", "ff66"]))
        );
        assert!(!sparse.has("user.entity.relationships.supervises.user.email"));
    }

    /// Verbatim from `pipelines/windows/forwarded/security_standard.yml`.
    #[test]
    fn a_file_extension_is_the_name_after_its_last_dot() {
        let script = "def extIdx = ctx.file.name.lastIndexOf(\".\");\n\
             if (extIdx > -1) {\n    ctx.file.extension = ctx.file.name.substring(extIdx+1);\n}";

        let mut event = Event::new(json!({ "file": { "name": "summary.docx" } }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_str("file.extension"), Some("docx"));

        // A name with no dot writes nothing.
        let mut bare = Event::new(json!({ "file": { "name": "summary" } }));
        assert!(try_known_painless(&mut bare, script));
        assert!(!bare.has("file.extension"));
    }

    /// Verbatim from `pipelines/m365_defender/event/pipeline_device.yml`.
    #[test]
    fn dns_header_flags_are_the_keys_the_map_marks_true() {
        let script = "def af = ctx.m365_defender.event.additional_fields;\n\
             List ecs_flags = [\"AA\", \"TC\", \"RD\", \"RA\", \"AD\", \"CD\", \"DO\"];\n\
             List flags = [];\n\
             if (af instanceof Map) {\n    for (def flag: ecs_flags) {\n\
             if (af[flag] != null && af[flag] == \"true\") {\n            flags.add(flag);\n\
             }\n    }\n}\n\
             if (!ctx.m365_defender.event.containsKey('dns')) {\n\
             ctx.m365_defender.event.dns = new HashMap();\n}\n\
             ctx.m365_defender.event.dns.header_flags = flags;\n";

        let mut event = Event::new(json!({ "m365_defender": { "event": { "additional_fields": {
            "AA": "false", "TC": "false", "RD": "true", "RA": "true"
        }}}}));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get("m365_defender.event.dns.header_flags"),
            Some(&json!(["RD", "RA"])),
            "the ECS order of the literal list, not the map's"
        );
    }

    /// Verbatim from `pipelines/m365_defender/event/pipeline_device.yml`.
    const ZIP_ANSWERS: &str = "def answers = ctx.m365_defender.event.dns.answers; \
         def ttls = ctx.m365_defender.event.dns.ttls; \
         if (answers.isEmpty() || ttls.isEmpty()) {\n  return;\n} \
         else if (answers.length != ttls.length) {\n  if (ctx.error == null) {\n\
         ctx.error = new HashMap();\n  }\n  if (ctx.error.message == null) {\n\
         ctx.error.message = new ArrayList();\n  }\n\
         ctx.error.message.add('DNS answers and TTLs have a different length');\n} \
         def lst = new ArrayList(); for (def i = 0; i < answers.length; i++) {\n\
         lst.add([\n    \"data\": answers[i],\n    \"ttl\": (long)ttls[i]\n  ])\n} \
         if (ctx.dns == null) {\n  ctx.dns = new HashMap();\n} ctx.dns.answers = lst;";

    #[test]
    fn dns_answers_zip_with_their_ttls() {
        let mut event = Event::new(json!({ "m365_defender": { "event": { "dns": {
            "answers": ["89.160.20.112", "google.com"], "ttls": [5.0, 5.0]
        }}}}));
        assert!(try_known_painless(&mut event, ZIP_ANSWERS));
        assert_eq!(
            event.get("dns.answers"),
            Some(&json!([
                {"data": "89.160.20.112", "ttl": 5},
                {"data": "google.com", "ttl": 5}
            ])),
            "the TTL carries the script's own cast to long"
        );

        // Lists of different lengths are the script's stated error, and the one
        // that runs short throws before anything is written.
        let mut ragged = Event::new(json!({ "m365_defender": { "event": { "dns": {
            "answers": ["a", "b"], "ttls": [5.0]
        }}}}));
        assert!(try_known_painless(&mut ragged, ZIP_ANSWERS));
        assert!(!ragged.has("dns.answers"));
        assert_eq!(
            ragged.get("error.message"),
            Some(&json!(["DNS answers and TTLs have a different length"])),
            "the vendor's error.message is a list it appends to"
        );
    }

    /// The shape of `pipelines/m365_defender/incident/default.yml`'s
    /// `set_process_name_from_command_line`, which four pipelines share.
    const PROCESS_NAME: &str = "ctx.process = ctx.process ?: [:];\n\
         ctx.process.name = ctx.process.name ?: [];\n\
         def currentNames = new HashSet();\n\
         if (ctx.process.command_line != null) { ... }\n\
         ctx.process.name = new ArrayList(currentNames);\n";

    #[test]
    fn a_process_name_comes_off_its_command_line() {
        // One name survives, so the field is a SCALAR.
        let mut event = Event::new(json!({ "process": { "command_line": ["\"MsSense.exe\""] } }));
        assert!(try_known_painless(&mut event, PROCESS_NAME));
        assert_eq!(event.get_str("process.name"), Some("MsSense.exe"));

        // A posix path keeps its last segment; a windows one is left whole,
        // because the vendor splits on `/` alone.
        let mut paths = Event::new(json!({ "process": { "command_line": [
            "/usr/bin/curl -s http://x", "C:\\Windows\\System32\\cmd.exe"
        ]}}));
        assert!(try_known_painless(&mut paths, PROCESS_NAME));
        assert_eq!(
            paths.get("process.name"),
            Some(&json!(["curl", "C:\\Windows\\System32\\cmd.exe"]))
        );

        // Nothing to gather writes nothing, not an empty list.
        let mut empty = Event::new(json!({ "process": { "pid": 4 } }));
        assert!(try_known_painless(&mut empty, PROCESS_NAME));
        assert!(!empty.has("process.name"));
    }

    /// Verbatim from `pipelines/microsoft_defender_endpoint/log/default.yml`.
    #[test]
    fn a_duration_is_the_span_between_two_parsed_instants() {
        let script = "Instant eventstart = ZonedDateTime.parse(ctx.event.start).toInstant(); \
             Instant eventend = ZonedDateTime.parse(ctx.event.end).toInstant(); \
             ctx.event['duration'] = ChronoUnit.NANOS.between(eventstart, eventend);\n";

        let mut event = Event::new(json!({ "event": {
            "start": "2020-07-06T05:23:56.7191052Z",
            "end": "2020-07-06T06:04:39.4188046Z"
        }}));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_i64("event.duration"), Some(2_442_699_699_400));

        // An end Painless cannot parse throws, and the vendor writes nothing.
        let mut unparseable = Event::new(json!({ "event": {
            "start": "2020-07-06T05:23:56.7191052Z", "end": "not a time"
        }}));
        assert!(try_known_painless(&mut unparseable, script));
        assert!(!unparseable.has("event.duration"));
    }

    /// Verbatim from `crowdstrike/data_stream/alert`, which binds the span to a
    /// local and copies it out under a sign guard.
    ///
    /// The target was read as `event.end` -- the last `ctx.` path before the
    /// assignment is the `end` declaration, not the field being written -- so
    /// the span overwrote the timestamp a `set` processor had just copied there
    /// and `event.duration` was never written at all.
    #[test]
    fn a_span_bound_to_a_local_lands_on_the_field_it_is_copied_to() {
        let script = "def start = ZonedDateTime.parse(ctx.event.start);\n\
             def end = ZonedDateTime.parse(ctx.event.end);\n\
             def duration = ChronoUnit.NANOS.between(start, end);\n\
             if (duration >= 0) {\n  ctx.event.duration = duration;\n}";

        let mut event = Event::new(json!({ "event": {
            "start": "2026-05-11T05:11:47.000Z",
            "end": "2026-05-11T05:13:20.000Z"
        }}));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_i64("event.duration"), Some(93_000_000_000));
        assert_eq!(event.get_str("event.end"), Some("2026-05-11T05:13:20.000Z"));

        // The script's own guard: a reversed span writes nothing.
        let mut reversed = Event::new(json!({ "event": {
            "start": "2026-05-11T05:13:20.000Z",
            "end": "2026-05-11T05:11:47.000Z"
        }}));
        assert!(try_known_painless(&mut reversed, script));
        assert!(!reversed.has("event.duration"));
        assert_eq!(
            reversed.get_str("event.end"),
            Some("2026-05-11T05:11:47.000Z")
        );
    }

    /// Verbatim from `pipelines/checkpoint/firewall/default.yml`, whose factor
    /// carries Java's long suffix.
    #[test]
    fn a_scale_factor_may_carry_the_java_long_suffix() {
        let script = "ctx.event.duration = ctx.event.duration * 1000000000L";

        let mut event = Event::new(json!({ "event": { "duration": 1931 } }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_i64("event.duration"), Some(1_931_000_000_000));
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
            nulls: true,
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
                nulls: true,
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

    /// The twenty-line `camelToSnake` / `convertToSnakeCase` pair, cut down to
    /// what the matcher keys on plus the apply line it reads the paths from.
    const CAMEL_TO_SNAKE: &str = "String camelToSnake(String str) {\n\
        def result = \"\";\n\
        if (Character.isUpperCase(c)) { result += \"_\"; }\n\
        return result;\n\
        }\n\
        def convertToSnakeCase(def obj) {\n\
        if (obj instanceof Map) {\n\
        if (!entry.getKey().contains(\"@\")) {\n\
        String newKey = camelToSnake(entry.getKey());\n\
        newObj[newKey] = convertToSnakeCase(entry.getValue());\n\
        }\n\
        } else if (obj instanceof List) {\n\
        for (item in obj) { newList.add(convertToSnakeCase(item)); }\n\
        }\n\
        }\n";

    #[test]
    fn camel_to_snake_writes_the_converted_object_to_its_target() {
        let script = format!(
            "{CAMEL_TO_SNAKE}ctx.sentinel_one = ctx.sentinel_one ?: [:];\n\
             ctx.sentinel_one.unified_alert = convertToSnakeCase(ctx.json);\n"
        );
        let mut event = Event::new(json!({
            "json": {"analystVerdict": "UNDEFINED", "detectionSource": {"vendorName": "S1"}}
        }));

        assert!(try_known_painless(&mut event, &script));
        assert_eq!(
            event.get_str("sentinel_one.unified_alert.analyst_verdict"),
            Some("UNDEFINED")
        );
        assert_eq!(
            event.get_str("sentinel_one.unified_alert.detection_source.vendor_name"),
            Some("S1"),
        );
        assert!(event.has("json"), "the source object is not consumed");
    }

    /// lambda's REPORT metrics: a map copied to a new path with its keys
    /// `snake_cased` by a helper the script names itself, so the shape of the
    /// replacement identifies it rather than the helper's name. `MB` is one
    /// word to the vendor's regex, not two letters.
    #[test]
    fn a_snake_cased_map_copy_lands_on_its_target() {
        let script = "String underscore(String s) {\n    \
             def regex = /_?([a-z])([A-Z]+)/;\n    \
             s = regex.matcher(s).replaceAll('$1_$2').toLowerCase();\n    \
             return s\n}\n\n\
             def out = [:];\n\
             for (def item : ctx.parsed.record.metrics.entrySet()) {\n    \
             out[underscore(item.getKey())] = item.getValue();\n}\n\
             ctx.aws.lambda.metrics = out\n";
        let mut event = Event::new(json!({
            "parsed": {"record": {"metrics": {
                "durationMs": 1234.567,
                "billedDurationMs": 1235,
                "memorySizeMB": 256,
                "maxMemoryUsedMB": 79,
            }}}
        }));

        assert!(try_known_painless(&mut event, script));
        assert_eq!(
            event.get_f64("aws.lambda.metrics.duration_ms"),
            Some(1234.567)
        );
        assert_eq!(
            event.get_i64("aws.lambda.metrics.billed_duration_ms"),
            Some(1235)
        );
        assert_eq!(
            event.get_i64("aws.lambda.metrics.memory_size_mb"),
            Some(256)
        );
        assert_eq!(
            event.get_i64("aws.lambda.metrics.max_memory_used_mb"),
            Some(79)
        );
    }

    /// `entra_id` rewrites its own object rather than writing somewhere new, and
    /// drops the `@odata.*` metadata on the way through.
    #[test]
    fn camel_to_snake_rewrites_in_place_and_drops_at_keys() {
        let script = format!(
            "{CAMEL_TO_SNAKE}if (ctx.entityanalytics_entra_id?.user != null) {{\n\
             ctx.entityanalytics_entra_id.user = \
             convertToSnakeCase(ctx.entityanalytics_entra_id.user);\n\
             }}\n"
        );
        let mut event = Event::new(json!({
            "entityanalytics_entra_id": {"user": {
                "accountEnabled": true,
                "@odata.type": "#microsoft.graph.user",
            }}
        }));

        assert!(try_known_painless(&mut event, &script));
        assert_eq!(
            event.get("entityanalytics_entra_id.user.account_enabled"),
            Some(&json!(true))
        );
        assert!(
            !event.has("entityanalytics_entra_id.user.accountEnabled"),
            "the camelCase key does not survive beside the snake_case one",
        );
        let user = event.get("entityanalytics_entra_id.user").unwrap();
        assert_eq!(
            user.as_object().unwrap().len(),
            1,
            "the @odata key is dropped, not renamed: {user}"
        );
    }

    /// The integrations' own rule breaks the word wherever the PREVIOUS
    /// character was not uppercase, which a digit satisfies.
    #[test]
    fn camel_to_snake_breaks_after_a_digit() {
        let converted = camel_map_to_snake(&json!({"cve2021Id": 1, "HTTPServer": 2}));
        assert!(converted.get("cve2021_id").is_some(), "{converted}");
        assert!(converted.get("httpserver").is_some(), "{converted}");
    }

    /// A value map written as an if/else-if chain with `.put()` as the write.
    /// Verbatim from `zscaler_zia/firewall`, whose device OS table is spelled
    /// this way rather than as params.
    #[test]
    fn a_literal_value_map_writes_through_put() {
        let script = "String osType = ctx.zscaler_zia.firewall.device.os.type;\n\
            if (ctx.host == null) {\n    Map map = new HashMap();\n    ctx.put('host', map);\n}\n\
            if (ctx.host?.os == null) {\n    Map map = new HashMap();\n    ctx.host.put('os', map);\n}\n\
            if (osType == 'iOS') {\n   ctx.host.os.put('type', 'ios');\n}\n\
            else if (osType == 'Android OS') {\n   ctx.host.os.put('type', 'android');\n}\n\
            else if (osType == 'Windows OS') {\n   ctx.host.os.put('type', 'windows');\n}\n";

        let mut event = Event::new(json!({
            "zscaler_zia": {"firewall": {"device": {"os": {"type": "iOS"}}}}
        }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_str("host.os.type"), Some("ios"));

        let mut event = Event::new(json!({
            "zscaler_zia": {"firewall": {"device": {"os": {"type": "Android OS"}}}}
        }));
        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get_str("host.os.type"), Some("android"));
    }

    /// A value the table has no arm for leaves the field alone.
    #[test]
    fn a_literal_value_map_writes_nothing_for_an_unlisted_value() {
        let script = "String osType = ctx.a.b;\n\
            if (osType == 'iOS') {\n   ctx.host.os.put('type', 'ios');\n}\n\
            else if (osType == 'MAC OS') {\n   ctx.host.os.put('type', 'macos');\n}\n";
        let mut event = Event::new(json!({"a": {"b": "Solaris"}}));
        try_known_painless(&mut event, script);
        assert_eq!(event.get("host.os.type"), None);
    }

    /// Cut from `pipelines/gcp/audit/default.yml`, tagged "Classify actor and
    /// target entities into type-specific fields".
    const CLASSIFY: &str = "void addNestedValue(def currentCtx, String path, def value) {\n\
        return;\n\
        }\n\
        if (ctx.actor?.entity?.id instanceof List) {\n\
        for (def actorId : ctx.actor.entity.id) {\n\
        String actor = actorId.toString();\n\
        if (actor.startsWith(\"serviceAccount:\") || actor.contains(\".gserviceaccount.com\")) {\n\
        addNestedValue(ctx, \"service.entity.id\", actor);\n\
        }\n\
        else if (actor.startsWith(\"user:\") || (actor.contains(\"@\") && \
        !actor.contains(\".gserviceaccount.com\"))) {\n\
        addNestedValue(ctx, \"user.entity.id\", actor);\n\
        }\n\
        else {\n\
        addNestedValue(ctx, \"entity.id\", actor);\n\
        }\n\
        }\n\
        }";

    #[test]
    fn each_member_lands_in_the_path_its_own_text_earns() {
        let mut event = Event::new(json!({"actor": {"entity": {"id": [
            "serviceAccount:svc@project.iam.gserviceaccount.com",
            "user@mycompany.com",
            "projects/foo/workloadIdentityPools/bar",
        ]}}}));

        assert!(try_known_painless(&mut event, CLASSIFY));
        assert_eq!(
            event.get("service.entity.id"),
            Some(&json!([
                "serviceAccount:svc@project.iam.gserviceaccount.com"
            ]))
        );
        assert_eq!(
            event.get("user.entity.id"),
            Some(&json!(["user@mycompany.com"])),
            "an `@` that is not a service account is a user"
        );
        assert_eq!(
            event.get("entity.id"),
            Some(&json!(["projects/foo/workloadIdentityPools/bar"])),
            "the trailing else is the catch-all"
        );
    }

    /// Cut from `pipelines/aws/cloudtrail/default.yml`, both scripts.
    const FLATTENED: &str = "ctx._conf.keep_flattened_duplicates = ctx._conf.retain == null ||\n\
        ctx._conf.retain.contains('all');";
    const DUPLICATE: &str = "if (ctx.json?.requestParameters != null) {\n\
        ctx.aws.cloudtrail.request_parameters = ctx.json.requestParameters.toString();\n\
        if (ctx._conf.keep_flattened_duplicates) {\n\
        ctx.aws.cloudtrail.flattened.request_parameters = ctx.json.requestParameters;\n\
        }\n\
        }";

    /// A Painless map renders as Java's `{k=v, k=v}`, not as JSON.
    #[test]
    fn a_rendered_object_is_kept_beside_the_object() {
        let mut event = Event::new(json!({"json": {"requestParameters": {
            "principal": "sns.amazonaws.com",
            "functionName": "cloudtrail-events-test",
        }}}));

        assert!(try_known_painless(&mut event, FLATTENED));
        assert_eq!(
            event.get("_conf.keep_flattened_duplicates"),
            Some(&json!(true)),
            "no `retain` means keep"
        );

        assert!(try_known_painless(&mut event, DUPLICATE));
        assert_eq!(
            event.get_str("aws.cloudtrail.request_parameters"),
            Some("{principal=sns.amazonaws.com, functionName=cloudtrail-events-test}")
        );
        assert_eq!(
            event.get_str("aws.cloudtrail.flattened.request_parameters.principal"),
            Some("sns.amazonaws.com")
        );
    }

    /// A `retain` the flag does not recognise means no flattened copy, and
    /// the rendered one is written either way.
    #[test]
    fn an_unrecognised_retain_keeps_no_duplicate() {
        let mut event = Event::new(json!({
            "_conf": {"retain": "keyword"},
            "json": {"requestParameters": {"principal": "sns.amazonaws.com"}},
        }));

        assert!(try_known_painless(&mut event, FLATTENED));
        assert!(try_known_painless(&mut event, DUPLICATE));
        assert_eq!(
            event.get_str("aws.cloudtrail.request_parameters"),
            Some("{principal=sns.amazonaws.com}")
        );
        assert_eq!(event.get("aws.cloudtrail.flattened"), None);
    }

    /// Cut from `pipelines/gcp/audit/default.yml` to the three argument
    /// shapes: a `ctx.` path, a member of a bound local, and a member of a
    /// loop variable over a list.
    const ENTITIES: &str = "void addValue(Set entities, def value) {\n\
        if (value != null && value != \"\") { entities.add(value); }\n\
        }\n\
        TreeSet entities = new TreeSet();\n\
        addValue(entities, ctx.json.protoPayload.resourceName);\n\
        HashMap authInfo = ctx.json?.protoPayload?.authenticationInfo ?: new HashMap();\n\
        addValue(entities, authInfo.principalEmail);\n\
        for (def i: ctx.json.protoPayload.delegates) {\n\
        addValue(entities, i.principalSubject);\n\
        }\n\
        if (entities.size() > 0) {\n\
        ctx.related = ctx.related ?: [:];\n\
        ctx.related.entity = entities;\n\
        }";

    #[test]
    fn entities_collect_sorted_unique_and_non_empty() {
        let mut event = Event::new(json!({"json": {"protoPayload": {
            "resourceName": "projects/elastic-beats",
            "authenticationInfo": {"principalEmail": "xxx@xxx.xxx"},
            "delegates": [
                {"principalSubject": "serviceAccount:a"},
                {"principalSubject": ""},
                {"principalSubject": "serviceAccount:a"},
            ],
        }}}));

        assert!(try_known_painless(&mut event, ENTITIES));
        assert_eq!(
            event.get("related.entity"),
            Some(&json!([
                "projects/elastic-beats",
                "serviceAccount:a",
                "xxx@xxx.xxx"
            ])),
            "sorted, unique, and nothing empty"
        );
    }

    /// Nothing to collect leaves the field alone rather than writing an empty
    /// list, which is what the script's own `entities.size() > 0` says.
    #[test]
    fn no_entities_writes_no_list() {
        let mut event = Event::new(json!({"json": {}}));
        assert!(try_known_painless(&mut event, ENTITIES));
        assert_eq!(event.get("related.entity"), None);
    }

    /// Verbatim from `pipelines/gcp/dns/default.yml`, cut to the lines the
    /// matcher keys on.
    const RDATA: &str = "def rdata = ctx.gcp.dns.rdata;\n\
        def dns_answers = [];\n\
        def answer_parts = /\\t/.split(rdata_answers[i]);\n\
        def name = answer_parts[0];\n\
        def ttl = Long.parseLong(answer_parts[1]);\n\
        dns_answers.add([\"name\": name]);\n\
        ctx.dns.answers = dns_answers;";

    #[test]
    fn dns_rdata_columns_become_answers() {
        let mut event = Event::new(json!({"gcp": {"dns": {"rdata": concat!(
            "elastic.co.\t300\tIN\ta\t127.0.0.1\n",
            "elastic.co.\t21600\tIN\tns\tns-1168.awsdns-18.org."
        )}}}));

        assert!(try_known_painless(&mut event, RDATA));
        assert_eq!(
            event.get("dns.answers"),
            Some(&json!([
                {"name": "elastic.co", "ttl": 300, "class": "IN", "type": "A", "data": "127.0.0.1"},
                {"name": "elastic.co", "ttl": 21600, "class": "IN", "type": "NS",
                 "data": "ns-1168.awsdns-18.org"},
            ]))
        );
    }

    /// A trailing `...` is the vendor saying the list was cut short, and is
    /// not an answer.
    #[test]
    fn a_truncated_rdata_list_drops_its_last_line() {
        let mut event = Event::new(json!({"gcp": {"dns": {"rdata":
            "elastic.co.\t300\tIN\ta\t127.0.0.1\n..."}}}));

        assert!(try_known_painless(&mut event, RDATA));
        let Some(Value::Array(answers)) = event.get("dns.answers") else {
            panic!("no answers")
        };
        assert_eq!(answers.len(), 1);
    }

    /// An address is a resolved ip, a CNAME is a host, and an MX's host is
    /// its SECOND token -- the first is the preference.
    #[test]
    fn an_answer_set_fans_out_into_the_ecs_lists() {
        let script = "for (answer in ctx.dns.answers) { ctx.related.ip.add(answer.data); }";
        let mut event = Event::new(json!({"dns": {"answers": [
            {"type": "A", "data": "127.0.0.1"},
            {"type": "CNAME", "data": "www.elastic.co"},
            {"type": "MX", "data": "1 aspmx.l.google.com"},
            {"type": "TXT", "data": "v=spf1"},
        ]}}));

        assert!(try_known_painless(&mut event, script));
        assert_eq!(event.get("dns.resolved_ip"), Some(&json!(["127.0.0.1"])));
        assert_eq!(event.get("related.ip"), Some(&json!(["127.0.0.1"])));
        assert_eq!(
            event.get("related.hosts"),
            Some(&json!(["www.elastic.co", "aspmx.l.google.com"]))
        );
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
