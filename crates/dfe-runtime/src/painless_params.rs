// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Painless script patterns whose behaviour lives in the processor's `params`.
//!
//! An Elastic `script` processor may carry a `params` block, and the recurring
//! shapes read their whole configuration from it -- the sentinel list to strip,
//! the field names to convert, the lookup table to merge. Matching on script
//! text alone cannot execute any of them, because the text says only *that* a
//! param is read, never what it holds.
//!
//! [`try_params_painless`] is tried before the text-only matchers in
//! [`crate::painless_common::try_known_painless`], so a shape with a params
//! block runs against the pipeline's real table rather than a transcribed copy.

use serde_json::{Map, Value, json};

use crate::event::Event;
use crate::painless_helpers::{filetime_to_unix_ms, remove_sentinel_values};

/// A processor's `params` block, parsed once per CALL SITE.
///
/// The block is emitted as a JSON string rather than a `json!` literal: the
/// macro expands once per key, and the o365 operation table alone is deep
/// enough to blow rustc's recursion limit. Parsing it behind a per-site
/// `OnceLock` costs one parse per process and nothing per event -- the same
/// shape [`crate::cached_grok`] uses, and for the same reason.
#[macro_export]
macro_rules! cached_params {
    ($json:literal $(,)?) => {{
        static SITE: ::std::sync::OnceLock<::serde_json::Value> = ::std::sync::OnceLock::new();
        SITE.get_or_init(|| {
            ::serde_json::from_str($json).expect("codegen emits a valid params block")
        })
    }};
}

/// The FILETIME threshold, as it is written in the vendor scripts.
const FILETIME_LITERAL: &str = "0x0100000000000000L";

/// Run a Painless script whose behaviour is carried by its `params` block.
///
/// Returns false when nothing matched, so the caller falls through to the
/// text-only matchers.
pub fn try_params_painless(event: &mut Event, script: &str, params: &Value) -> bool {
    let Some(params) = params.as_object() else {
        return false;
    };
    let normalised = crate::painless_common::normalise(script);
    match params_shape(&normalised) {
        Some(shape) => run_params_shape(event, &normalised, params, &shape),
        None => false,
    }
}

/// The matcher a params script's text routes to.
///
/// Every branch of the params dispatch is terminal -- the first trigger that
/// holds names the matcher, whatever that matcher then returns -- so the whole
/// decision is a property of the script TEXT and is made once per call site by
/// [`crate::painless_plan::PainlessPlan`] rather than once per event. A shape
/// whose trigger is itself a parse carries the parse's result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParamsShape {
    AwsEntity(Box<crate::painless_entity::EntityScript>),
    SysmonQueryResults,
    SysmonRegistry,
    MessageTable,
    FirstAsset,
    MatcherKv,
    LookupPut {
        source: String,
        target: String,
        removes: Vec<String>,
    },
    LookupWrapList {
        source: String,
        target: String,
    },
    ProtocolPrefix {
        list: String,
        fallback: String,
        subject: String,
        target: String,
    },
    MimecastLogType,
    SentinelRemoval,
    FiletimeFieldList,
    BitFlags,
    FirstContainedMember,
    RenameKeys,
    ValueMaps,
    RowColumns,
    KeyedMessageTable,
    ReversibleLookup,
    LookupMerge,
    LookupColumns,
    LookupNormalise,
    IndexedLookup,
    Scale,
    Replace,
    AddUniqueRow,
    FrameworkPreference,
}

/// The one matcher this script's text triggers, or `None`.
///
/// The trigger order is load-bearing and each comment says why a branch sits
/// where it does; a script can spell several triggers and the FIRST wins,
/// exactly as the old inline dispatch behaved.
pub(crate) fn params_shape(normalised: &str) -> Option<ParamsShape> {
    // Pattern: aws cloudtrail's entity classifier, per-service enrichment
    // into TreeSets then classification through the params tables. First,
    // because its 950 lines spell half the other triggers somewhere. The
    // trigger IS the parse: a restructured script declines here and falls
    // through the dispatch unclaimed.
    if normalised.contains("enrichCtx")
        && normalised.contains("related.entity")
        && let Some(parsed) = crate::painless_entity::EntityScript::parse(normalised)
    {
        return Some(ParamsShape::AwsEntity(Box::new(parsed)));
    }

    // Pattern: sysmon's semicolon-separated DNS QueryResults, where params is
    // the RR-number-to-name table. Checked first: the script also spells
    // `.put(` and `params`, which a later matcher reads as an indexed lookup.
    if normalised.contains("QueryResults") && normalised.contains("startsWith(\"type:\")") {
        return Some(ParamsShape::SysmonQueryResults);
    }

    // Pattern: sysmon's registry fields, the hive abbreviated through the
    // params table and the Details value typed by its own text.
    if normalised.contains("ctx.registry = new HashMap()") && normalised.contains("TargetObject") {
        return Some(ParamsShape::SysmonRegistry);
    }

    // Pattern: strip the vendor's sentinel values out of a map.
    if normalised.contains(".entrySet().removeIf(") && normalised.contains("entry.getValue()") {
        return Some(ParamsShape::SentinelRemoval);
    }

    // Pattern: mimecast's scored log-type classifier, keyed on its four
    // params tables.
    if normalised.contains("params.definite_positive") && normalised.contains("params.candidates") {
        return Some(ParamsShape::MimecastLogType);
    }

    // Pattern: convert every named field from Windows FILETIME to UNIX ms.
    if normalised.contains(FILETIME_LITERAL) && normalised.contains("for (def field : params.") {
        return Some(ParamsShape::FiletimeFieldList);
    }

    // Pattern: decode a bitfield into one boolean label per set bit.
    if normalised.contains("params.entrySet()") && normalised.contains("& flag") {
        return Some(ParamsShape::BitFlags);
    }

    // Pattern: the first member of a params list the subject contains.
    if normalised.contains("for (String ") && normalised.contains(".put(") {
        return Some(ParamsShape::FirstContainedMember);
    }

    // Pattern: rename an object's keys, recursively, through a name map.
    if normalised.contains("keyMap.containsKey(key)") {
        return Some(ParamsShape::RenameKeys);
    }

    // Pattern: several fields each normalised through their own value map.
    // Ahead of the reversible lookup, whose trigger this shape also matches.
    if normalised.contains(".map?.getOrDefault(") || normalised.contains("param.map.") {
        return Some(ParamsShape::ValueMaps);
    }

    // Pattern: params IS the table, and one row's columns are written straight
    // onto ctx, then refined by the event's outcome.
    if normalised.contains("params.get(ctx.") && normalised.contains(").get('") {
        return Some(ParamsShape::RowColumns);
    }

    // Pattern: fan a parsed key/value message out through a params table.
    if normalised.contains("appendOrCreate(") && normalised.contains("params.get(entry.getKey())") {
        return Some(ParamsShape::KeyedMessageTable);
    }

    // Pattern: the security pipeline's msobjs message-table decode, keyed on
    // its two auxiliary tables. Ahead of the indexed lookup, whose `.put(`
    // trigger its writes also spell.
    if normalised.contains("AccessMaskDescriptions") && normalised.contains("reversed_descriptions")
    {
        return Some(ParamsShape::MessageTable);
    }

    // Pattern: sentinel_one's first-asset extraction, the os list in params.
    if normalised.contains("agent_uuid") && normalised.contains("params.os_type") {
        return Some(ParamsShape::FirstAsset);
    }

    // Pattern: powershell's matcher-driven KV -- tab-prefixed keys, the
    // value everything up to the next key, multiline included.
    if normalised.contains("ctx.winlog?.event_data[params[")
        && normalised.contains("previousEnd")
    {
        return Some(ParamsShape::MatcherKv);
    }

    // Pattern: look one field up in the table and `.put` the row somewhere
    // ELSE -- the security pipeline's logon type, dnsserver's QTYPE with its
    // trailing `.remove`. Ahead of the normalise shape, which writes back to
    // the field it read.
    if normalised.contains("= params.get(ctx.")
        && let Some((source, target)) = parse_lookup_put(normalised)
    {
        return Some(ParamsShape::LookupPut {
            source,
            target,
            removes: parse_removes(normalised),
        });
    }

    // Pattern: the row wrapped in a one-element list -- dnsserver's winlog
    // keywords.
    if normalised.contains("= params.get(ctx.")
        && normalised.contains("new ArrayList()")
        && let Some((source, target)) = parse_lookup_wrap_list(normalised)
    {
        return Some(ParamsShape::LookupWrapList { source, target });
    }

    // Pattern: prefix a field with a validated scheme -- zscaler web's
    // `<protocol>://<url>` build, the fallback scheme from params.
    if normalised.contains("+ '://' +")
        && let Some(shape) = parse_protocol_prefix(normalised)
    {
        return Some(shape);
    }

    // Pattern: map a field through a params table in whichever direction it
    // was written -- name to number, or a number already there back to a name.
    if normalised.contains("params.entrySet()") && normalised.contains("entry.getKey()") {
        return Some(ParamsShape::ReversibleLookup);
    }

    // Pattern: look a field up in a static table and merge the row into ctx.
    if normalised.contains("params.get(") && normalised.contains("forEach((k, v) ->") {
        return Some(ParamsShape::LookupMerge);
    }

    // Pattern: look a row up in a nested table and fan its columns out,
    // appending the list-valued ones rather than replacing them.
    if normalised.contains("params.get(") && normalised.matches(".get(").count() >= 3 {
        return Some(ParamsShape::LookupColumns);
    }

    // Pattern: normalise a field through a params table, keeping the input
    // when the table has no row for it.
    if normalised.contains("params.get(") {
        return Some(ParamsShape::LookupNormalise);
    }

    // Pattern: index a params array by a numeric field.
    if normalised.contains(".put(") && normalised.contains("params") {
        return Some(ParamsShape::IndexedLookup);
    }

    // Pattern: scale a numeric field by a params constant.
    if normalised.contains("* params.") {
        return Some(ParamsShape::Scale);
    }

    // Pattern: strip a params-named marker out of a string field.
    if normalised.contains(".replace(params.") {
        return Some(ParamsShape::Replace);
    }

    // Pattern: union a table row's list columns into the ECS arrays.
    if normalised.contains("addUnique(") && normalised.contains("params[ctx.") {
        return Some(ParamsShape::AddUniqueRow);
    }

    // Pattern: pick one framework name from the prefixes of several ID lists.
    if normalised.contains("new HashSet()") && normalised.contains("params.framework_preference") {
        return Some(ParamsShape::FrameworkPreference);
    }

    // Pattern: the same normalise-through-a-table written with the bracket
    // form. LAST, so nothing that reads the brackets for its own shape --
    // `addUnique` over a row, for one -- is claimed by the general case.
    if normalised.contains("params[ctx.") {
        return Some(ParamsShape::LookupNormalise);
    }

    None
}

/// Run the matcher a shape names, against one event.
pub(crate) fn run_params_shape(
    event: &mut Event,
    normalised: &str,
    params: &Map<String, Value>,
    shape: &ParamsShape,
) -> bool {
    match shape {
        ParamsShape::AwsEntity(script) => {
            crate::painless_entity::run_entity_script(event, script, params)
        }
        ParamsShape::SysmonQueryResults => try_sysmon_query_results(event, normalised, params),
        ParamsShape::SysmonRegistry => crate::painless_windows::run_registry(event, params),
        ParamsShape::MessageTable => crate::painless_windows::run_message_table(event, params),
        ParamsShape::FirstAsset => try_first_asset(event, params),
        ParamsShape::MatcherKv => try_matcher_kv(event, params),
        ParamsShape::LookupPut {
            source,
            target,
            removes,
        } => {
            if let Some(row) = event
                .get_as_string(source)
                .and_then(|key| params.get(&key))
                .cloned()
            {
                let _ = event.set(target, row);
            }
            // The script removes OUTSIDE its null guard, so a key with no
            // table row still goes.
            for path in removes {
                event.remove(path);
            }
            true
        }
        ParamsShape::LookupWrapList { source, target } => {
            if let Some(row) = event
                .get_as_string(source)
                .and_then(|key| params.get(&key))
                .cloned()
            {
                let _ = event.set(target, Value::Array(vec![row]));
            }
            true
        }
        ParamsShape::MimecastLogType => try_mimecast_log_type(event, params),
        ParamsShape::ProtocolPrefix {
            list,
            fallback,
            subject,
            target,
        } => {
            let (Some(scheme), Some(tail)) = (
                event.get_str(subject).map(str::to_string),
                event.get_str(target).map(str::to_string),
            ) else {
                return true;
            };
            let valid = params
                .get(list)
                .and_then(Value::as_array)
                .is_some_and(|schemes| schemes.iter().any(|s| s.as_str() == Some(&scheme)));
            let scheme = if valid {
                scheme
            } else {
                match params.get(fallback).and_then(Value::as_str) {
                    Some(fallback) => fallback.to_string(),
                    None => return true,
                }
            };
            let _ = event.set(target, Value::String(format!("{scheme}://{tail}")));
            true
        }
        ParamsShape::SentinelRemoval => try_sentinel_removal(event, normalised, params),
        ParamsShape::FiletimeFieldList => try_filetime_field_list(event, normalised, params),
        ParamsShape::BitFlags => try_bit_flags(event, normalised, params),
        ParamsShape::FirstContainedMember => try_first_contained_member(event, normalised, params),
        ParamsShape::RenameKeys => try_rename_keys(event, normalised, params),
        ParamsShape::ValueMaps => try_value_maps(event, normalised, params),
        ParamsShape::RowColumns => try_row_columns(event, normalised, params),
        ParamsShape::KeyedMessageTable => try_keyed_message_table(event, normalised, params),
        ParamsShape::ReversibleLookup => try_reversible_lookup(event, normalised, params),
        ParamsShape::LookupMerge => try_lookup_merge(event, normalised, params),
        ParamsShape::LookupColumns => try_lookup_columns(event, normalised, params),
        ParamsShape::LookupNormalise => try_lookup_normalise(event, normalised, params),
        ParamsShape::IndexedLookup => try_indexed_lookup(event, normalised, params),
        ParamsShape::Scale => try_scale(event, normalised, params),
        ParamsShape::Replace => try_replace(event, normalised, params),
        ParamsShape::AddUniqueRow => try_add_unique_row(event, normalised, params),
        ParamsShape::FrameworkPreference => try_framework_preference(event, normalised, params),
    }
}

/// Read `def <t> = params.get(ctx.<source>);` and the `ctx.<base>.put("<leaf>",
/// <t>)` that stores it, as (source, target).
fn parse_lookup_put(script: &str) -> Option<(String, String)> {
    let at = script.find("= params.get(ctx.")?;
    let after = &script[at + "= params.get(ctx.".len()..];
    let (source, _) = after.split_once(')')?;
    let local = script[..at].split_whitespace().next_back()?;
    if local.is_empty() || !local.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return None;
    }

    let store = format!(", {local})");
    let store_at = script.find(&store)?;
    let before = &script[..store_at];
    let put_at = before.rfind(".put(\"")?;
    let leaf = before[put_at + ".put(\"".len()..].trim_end_matches('"');
    let base = clean_path(before[..put_at].rsplit("ctx.").next()?);
    Some((clean_path(source), format!("{base}.{leaf}")))
}

/// Every `ctx.<base>.remove("<key>")` in the script, as dotted paths.
fn parse_removes(script: &str) -> Vec<String> {
    let mut removes = Vec::new();
    for (at, _) in script.match_indices(".remove(\"") {
        let Some(key) = script[at + ".remove(\"".len()..].split('"').next() else {
            continue;
        };
        let before = &script[..at];
        let Some(ctx_at) = before.rfind("ctx.") else {
            continue;
        };
        let base = clean_path(&before[ctx_at + 4..]);
        if base
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '_')
        {
            removes.push(format!("{base}.{key}"));
        }
    }
    removes
}

/// Read the wrap-in-list shape: `def <t> = params.get(ctx.<source>); def
/// <list> = new ArrayList(); <list>.add(<t>); ctx.<target> = <list>;`.
fn parse_lookup_wrap_list(script: &str) -> Option<(String, String)> {
    let at = script.find("= params.get(ctx.")?;
    let after = &script[at + "= params.get(ctx.".len()..];
    let (source, _) = after.split_once(')')?;
    let local = script[..at].split_whitespace().next_back()?;

    let add = format!(".add({local})");
    let add_at = script.find(&add)?;
    let list = script[..add_at]
        .rsplit(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()?;
    if list.is_empty() {
        return None;
    }

    let store = format!(" = {list};");
    let store_at = script.rfind(&store)?;
    let before = &script[..store_at];
    let target = clean_path(before[before.rfind("ctx.")? + 4..].trim());
    Some((clean_path(source), target))
}

/// powershell's matcher-driven KV over `winlog.event_data.<params.field>`:
/// a key is a TAB, a word run and an equals sign, and its value is
/// everything -- newlines, equals signs and all -- up to the next key,
/// trimmed. No key found writes nothing, which is where the script's own
/// `group` would have thrown.
fn try_matcher_kv(event: &mut Event, params: &Map<String, Value>) -> bool {
    let Some(field) = params.get("field").and_then(Value::as_str) else {
        return true;
    };
    let Some(text) = event
        .get_str(&format!("winlog.event_data.{field}"))
        .map(str::to_string)
    else {
        return true;
    };

    let bytes = text.as_bytes();
    let mut keys: Vec<(usize, usize, String)> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\t' {
            let mut j = i + 1;
            while j < bytes.len() && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                j += 1;
            }
            if j > i + 1 && bytes.get(j) == Some(&b'=') {
                keys.push((i, j + 1, text[i + 1..j].to_string()));
                i = j + 1;
                continue;
            }
        }
        i += 1;
    }

    for (index, (_, end, key)) in keys.iter().enumerate() {
        let until = keys.get(index + 1).map_or(text.len(), |(start, ..)| *start);
        let value = text[*end..until].trim().to_string();
        let _ = event.set(&format!("winlog.event_data.{key}"), json!(value));
    }
    true
}

/// `sentinel_one`'s first-asset extraction, transliterated: the container
/// defaults come first exactly as the script writes them -- WHICH is why the
/// provider-details instance, machine-type and region fallbacks are dead,
/// their `== null` guards seeing the empty defaults -- then the first asset,
/// the first detection-time asset, its cloud and kubernetes blocks, and the
/// params-listed os match that RETURNS from the whole script.
#[allow(clippy::too_many_lines)] // A transliteration; splitting it would hide the script's order.
fn try_first_asset(event: &mut Event, params: &Map<String, Value>) -> bool {
    for (path, empty) in [
        ("related", json!({})),
        ("related.hosts", json!([])),
        ("related.user", json!([])),
        ("related.ip", json!([])),
        ("host", json!({})),
        ("host.ip", json!([])),
        ("host.os", json!({})),
        ("cloud", json!({})),
        ("cloud.instance", json!({})),
        ("cloud.machine", json!({})),
        ("cloud.account", json!({})),
        ("cloud.machine.type", json!([])),
        ("cloud.region", json!([])),
        ("cloud.instance.id", json!([])),
        ("cloud.account.id", json!([])),
        ("cloud.account.name", json!([])),
        ("cloud.project", json!({})),
        ("cloud.project.id", json!([])),
        ("container", json!({})),
        ("container.id", json!([])),
        ("container.name", json!([])),
        ("container.image", json!({})),
        ("container.image.name", json!([])),
        ("container.labels", json!([])),
        ("orchestrator", json!({})),
        ("orchestrator.cluster", json!({})),
        ("orchestrator.namespace", json!([])),
        ("orchestrator.resource", json!({})),
        ("orchestrator.resource.parent", json!({})),
        ("orchestrator.resource.parent.type", json!([])),
        ("orchestrator.resource.label", json!([])),
        ("orchestrator.resource.name", json!([])),
        ("observer", json!({})),
    ] {
        if !event.has_value(path) {
            let _ = event.set(path, empty);
        }
    }

    if let Some(Value::Array(assets)) = event.get("sentinel_one.unified_alert.assets").cloned()
        && let Some(asset) = assets.first()
    {
        let field = |k: &str| asset.get(k).cloned();
        if let Some(v) = field("agent_uuid") {
            let _ = event.set("observer.serial_number", v);
        }
        if let Some(v) = field("agent_version") {
            let _ = event.set("observer.version", v);
        }
        if let Some(v) = field("id") {
            let _ = event.append("related.hosts", v);
        }
        if let Some(v) = field("name") {
            let _ = event.append("related.hosts", v.clone());
            let _ = event.set("host.name", v);
        }
        if let Some(v) = field("subcategory") {
            let _ = event.set("host.type", v);
        }
        if let Some(v) = field("last_logged_in_user") {
            let _ = event.append("related.user", v);
        }
    }

    let Some(Value::Array(dt_assets)) = event
        .get("sentinel_one.unified_alert.detection_time.assets")
        .cloned()
    else {
        return true;
    };
    let Some(first) = dt_assets.first() else {
        return true;
    };

    if let Some(a) = first.get("asset") {
        for key in ["console_ip_address", "ip_v4", "ip_v6"] {
            if let Some(v) = a.get(key).cloned() {
                let _ = event.append("host.ip", v.clone());
                let _ = event.append("related.ip", v);
            }
        }
        if let Some(v) = a.get("last_logged_in_user").cloned() {
            let _ = event.append("related.user", v);
        }
        if let Some(v) = a.get("os_name").cloned() {
            let _ = event.set("host.os.name", v);
        }
        if let Some(v) = a.get("os_revision").cloned() {
            let _ = event.set("host.os.version", v);
        }
        if let Some(os_type) = a.get("os_type").and_then(Value::as_str) {
            let lowered = os_type.to_lowercase();
            if let Some(Value::Array(names)) = params.get("os_type") {
                for os in names.iter().filter_map(Value::as_str) {
                    if lowered.contains(os) {
                        let _ = event.set("host.os.type", json!(os));
                        return true;
                    }
                }
            }
        }
    }

    let cloud = first.get("cloud");
    if let Some(c) = cloud {
        for (key, target) in [
            ("account_id", "cloud.account.name"),
            ("cloud_provider", "cloud.provider"),
            ("instance_id", "cloud.instance.id"),
            ("instance_size", "cloud.machine.type"),
            ("location", "cloud.region"),
        ] {
            if let Some(v) = c.get(key).cloned() {
                let _ = event.set(target, v);
            }
        }
        if let Some(pd) = c.get("provider_details") {
            for (key, target, only_if_absent) in [
                ("account_id", "cloud.account.id", false),
                ("instance_id", "cloud.instance.id", true),
                ("instance_type", "cloud.machine.type", true),
                ("project_id", "cloud.project.id", false),
                ("region", "cloud.region", true),
                ("service_account", "cloud.account.id", false),
                ("subscription_id", "cloud.account.id", false),
            ] {
                if let Some(v) = pd.get(key).cloned()
                    && (!only_if_absent || !event.has_value(target))
                {
                    let _ = event.set(target, v);
                }
            }
        }
    }

    if let Some(k) = first.get("kubernetes") {
        for (key, target) in [
            ("cluster_name", "orchestrator.cluster.name"),
            ("container_id", "container.id"),
            ("container_image_name", "container.image.name"),
            ("container_name", "container.name"),
            ("controller_type", "orchestrator.resource.parent.type"),
            ("namespace_name", "orchestrator.namespace"),
            ("pod_labels", "orchestrator.resource.label"),
            ("pod_name", "orchestrator.resource.name"),
        ] {
            if let Some(v) = k.get(key).cloned() {
                let _ = event.set(target, v);
            }
        }
    }
    true
}

/// mimecast's scored log-type classifier, tables from params.
///
/// Keys lowercase into a Java `HashSet`; a `definite_positive` hit wins
/// outright, then candidate ELIMINATION through the `negative` table (a lone
/// survivor wins), then the `positive` table scores what remains and every
/// co-equal winner is listed. Iteration orders are Java's hash orders, which
/// [`crate::painless_helpers::java_bucket`] reproduces -- the corpus's own
/// single-winner strings depend on them.
fn try_mimecast_log_type(event: &mut Event, params: &Map<String, Value>) -> bool {
    use crate::painless_helpers::{java_bucket, java_table_size};

    let Some(Value::Object(mimecast)) = event.get("mimecast") else {
        return true;
    };
    let table = |name: &str| params.get(name).and_then(Value::as_object);
    let (Some(definite), Some(negative), Some(positive), Some(candidates)) = (
        table("definite_positive"),
        table("negative"),
        table("positive"),
        table("candidates"),
    ) else {
        return true;
    };

    // The event's keys, lowercased into a HashSet and walked in ITS hash
    // order -- insertion order is the mimecast map's own bucket walk.
    let mut keys: Vec<String> = {
        let map_table = java_table_size(mimecast.len());
        let mut walked: Vec<(usize, usize, String)> = mimecast
            .keys()
            .enumerate()
            .map(|(position, key)| (java_bucket(key, map_table), position, key.to_lowercase()))
            .collect();
        walked.sort_by_key(|(bucket, position, _)| (*bucket, *position));
        let mut seen = Vec::new();
        for (_, _, key) in walked {
            if !seen.contains(&key) {
                seen.push(key);
            }
        }
        seen
    };
    let set_table = java_table_size(keys.len());
    let mut ordered: Vec<(usize, usize, String)> = keys
        .drain(..)
        .enumerate()
        .map(|(position, key)| (java_bucket(&key, set_table), position, key))
        .collect();
    ordered.sort_by_key(|(bucket, position, _)| (*bucket, *position));
    let keys: Vec<String> = ordered.into_iter().map(|(_, _, key)| key).collect();

    for key in &keys {
        if let Some(kind) = definite.get(key).and_then(Value::as_str) {
            let _ = event.set("mimecast.log_type", json!(kind));
            return true;
        }
    }

    // Elimination: candidates in the params table's own hash order.
    let candidate_table = java_table_size(candidates.len());
    let mut score: Vec<(usize, usize, String, i64)> = candidates
        .keys()
        .enumerate()
        .map(|(position, name)| {
            (
                java_bucket(name, candidate_table),
                position,
                name.clone(),
                0,
            )
        })
        .collect();
    score.sort_by_key(|(bucket, position, ..)| (*bucket, *position));

    for key in &keys {
        if let Some(Value::Array(kinds)) = negative.get(key) {
            for kind in kinds.iter().filter_map(Value::as_str) {
                score.retain(|(_, _, name, _)| name != kind);
            }
        }
    }
    if score.len() == 1 {
        let _ = event.set("mimecast.log_type", json!(score[0].2));
        return true;
    }

    let mut max = 0i64;
    for key in &keys {
        if let Some(Value::Array(kinds)) = positive.get(key) {
            for kind in kinds.iter().filter_map(Value::as_str) {
                if let Some(entry) = score.iter_mut().find(|(_, _, name, _)| name == kind) {
                    entry.3 += 1;
                    max = max.max(entry.3);
                }
            }
        }
    }
    score.retain(|(_, _, _, points)| *points >= max);

    let winners: Vec<Value> = score
        .into_iter()
        .map(|(_, _, name, _)| json!(name))
        .collect();
    let _ = event.set("mimecast.log_type", Value::Array(winners));
    true
}

/// Read the scheme-prefix shape: `if (params.<list>.contains(ctx.<subject>))
/// { ctx.<target> = ctx.<subject> + '://' + ctx.<target>; } else {
/// ctx.<target> = params.<fallback> + '://' + ... }`.
fn parse_protocol_prefix(script: &str) -> Option<ParamsShape> {
    let at = script.find(".contains(ctx.")?;
    let before = &script[..at];
    let list = before[before.rfind("params.")? + 7..].to_string();
    let after = &script[at + ".contains(ctx.".len()..];
    let (subject, _) = after.split_once(')')?;

    // The concatenation's tail names the target; the else-arm's params read
    // names the fallback scheme.
    let concat_at = script.find("+ '://' + ctx.")?;
    let tail = &script[concat_at + "+ '://' + ctx.".len()..];
    let target = tail
        .split([';', '\n'])
        .next()?
        .trim()
        .trim_end_matches(';')
        .trim();
    let fallback_at = script.rfind("= params.")?;
    let fallback = script[fallback_at + "= params.".len()..]
        .split_whitespace()
        .next()?
        .to_string();

    if list.is_empty() || fallback.is_empty() {
        return None;
    }
    Some(ParamsShape::ProtocolPrefix {
        list,
        fallback,
        subject: clean_path(subject),
        target: clean_path(target),
    })
}

/// Split sysmon's `QueryResults` into `dns.answers`, `dns.resolved_ip` and
/// `related.hosts`.
///
/// Event 22 packs the whole DNS response into one semicolon-separated string:
///
/// ```text
/// type:  5 f2.taboola.map.fastly.net;::ffff:151.101.66.2;::ffff:151.101.2.2;
/// ```
///
/// An entry opening `type:` is a record whose middle token is the IANA RR
/// NUMBER -- `params` is the number-to-name table and is read rather than
/// transcribed, so the vendor adding a type is picked up by regenerating.
/// Anything else is a resolved address. A `type:` entry with a third token
/// names a host, which is where the CNAME chain in `related.hosts` comes
/// from; without one only the type is recorded.
///
/// The `::ffff:` unwrapping and the `convert` to an ip are separate processors
/// downstream, so this leaves the addresses exactly as the vendor wrote them.
fn try_sysmon_query_results(event: &mut Event, _script: &str, params: &Map<String, Value>) -> bool {
    let Some(results) = event.get_str("winlog.event_data.QueryResults") else {
        return false;
    };

    let mut answers: Vec<Value> = Vec::new();
    let mut ips: Vec<Value> = Vec::new();
    let mut hosts: Vec<Value> = Vec::new();

    for answer in results.split(';') {
        if answer.is_empty() {
            continue;
        }
        if !answer.starts_with("type:") {
            ips.push(Value::String(answer.to_string()));
            continue;
        }

        let parts: Vec<&str> = answer.split_whitespace().collect();
        if parts.len() < 2 {
            // The script throws here, and a throw runs the processor's
            // on_failure rather than writing a half-parsed answer.
            return false;
        }
        // An unknown number reads as null in Painless, and the null reaches
        // the document, so it does here too.
        let kind = params.get(parts[1]).cloned().unwrap_or(Value::Null);
        if parts.len() == 3 {
            answers.push(serde_json::json!({"type": kind, "data": parts[2]}));
            hosts.push(Value::String(parts[2].to_string()));
        } else {
            answers.push(serde_json::json!({"type": kind}));
        }
    }

    if !answers.is_empty() {
        let _ = event.set("dns.answers", Value::Array(answers));
    }
    if !ips.is_empty() {
        let _ = event.set("dns.resolved_ip", Value::Array(ips));
    }
    if !hosts.is_empty() {
        let _ = event.set("related.hosts", Value::Array(hosts));
    }
    true
}

/// Collect a set of framework names from several ID lists, then pick one by a
/// declared preference order.
///
/// `CrowdStrike`'s `threat.framework`: a tactic id starting `TA` means MITRE, one
/// starting `CS` means Falcon, and a tactic NAME in the params list means Falcon
/// too. Every name, prefix and list member is read out of the script and its
/// params rather than transcribed, so the vendor extending any of them is picked
/// up by regenerating.
fn try_framework_preference(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(Value::Array(preference)) = params.get("framework_preference") else {
        return false;
    };

    let mut found: Vec<Value> = Vec::new();
    for (var, block) in loop_blocks(script) {
        let Some(path) = crate::painless_common::ctx_path_bound_to(script, &var) else {
            continue;
        };
        for member in string_members(event, &path) {
            for (prefix, key) in prefix_rules(block) {
                if member.starts_with(&prefix)
                    && let Some(name) = params.get(&key)
                    && !found.contains(name)
                {
                    found.push(name.clone());
                }
            }
            for (list_key, key) in membership_rules(block) {
                let Some(Value::Array(list)) = params.get(&list_key) else {
                    continue;
                };
                let lowered = Value::String(member.to_lowercase());
                if list.contains(&lowered)
                    && let Some(name) = params.get(&key)
                    && !found.contains(name)
                {
                    found.push(name.clone());
                }
            }
        }
    }

    if found.is_empty() {
        return true;
    }
    let chosen = if found.len() == 1 {
        found[0].clone()
    } else {
        // The preference list decides; a framework absent from it falls back to
        // whichever was collected first, which is what the script's final line
        // does with an unordered set.
        preference
            .iter()
            .find(|wanted| found.contains(wanted))
            .unwrap_or(&found[0])
            .clone()
    };
    let _ = event.set("threat.framework", chosen);
    true
}

/// Each `for (String t: <var>) { ... }` in the script, as its variable and the
/// text of its body.
///
/// The body runs to the next loop header, which is enough: the rules inside one
/// are all that is read from it.
fn loop_blocks(script: &str) -> Vec<(String, &str)> {
    const HEADER: &str = "for (String t: ";

    let mut blocks = Vec::new();
    let parts: Vec<&str> = script.split(HEADER).collect();
    for part in parts.iter().skip(1) {
        let Some((var, body)) = part.split_once(')') else {
            continue;
        };
        blocks.push((var.trim().to_string(), body));
    }
    blocks
}

/// Every `t.startsWith('<prefix>')` paired with the `params.<key>` its arm adds.
fn prefix_rules(block: &str) -> Vec<(String, String)> {
    rules_in(block, ".startsWith(")
}

/// Every `params.<list>.contains(...)` paired with the `params.<key>` its arm
/// adds.
fn membership_rules(block: &str) -> Vec<(String, String)> {
    const MARKER: &str = ".contains(";

    let mut rules = Vec::new();
    let mut at = 0;
    while let Some(found) = block[at..].find(MARKER) {
        let call = at + found;
        at = call + MARKER.len();

        // The list is whatever `params.` name sits immediately before the call.
        let head = &block[..call];
        let Some(start) = head.rfind("params.") else {
            continue;
        };
        let list = &head[start + "params.".len()..];
        if list.is_empty() || !list.chars().all(|c| c.is_alphanumeric() || c == '_') {
            continue;
        }
        let Some(key) = added_params_key(&block[at..]) else {
            continue;
        };
        rules.push((list.to_string(), key));
    }
    rules
}

/// The `(<literal>, params.<key>)` pairs an arm opened by `marker` names.
fn rules_in(block: &str, marker: &str) -> Vec<(String, String)> {
    let mut rules = Vec::new();
    for segment in block.split(marker).skip(1) {
        let Some(literal) = quoted_after(segment, "") else {
            continue;
        };
        let Some(key) = added_params_key(segment) else {
            continue;
        };
        rules.push((literal, key));
    }
    rules
}

/// The params key of the FIRST `frameworks.add(params.<key>)` in `text`.
fn added_params_key(text: &str) -> Option<String> {
    let start = text.find(".add(params.")? + ".add(params.".len();
    let tail = &text[start..];
    let end = tail
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .unwrap_or(tail.len());
    Some(tail[..end].to_string())
}

/// The string members of a field, whether it holds one or a list of them.
fn string_members(event: &Event, path: &str) -> Vec<String> {
    match event.get(path) {
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(|v| v.as_str().map(str::to_owned))
            .collect(),
        Some(Value::String(one)) => vec![one.clone()],
        _ => Vec::new(),
    }
}

/// `def p = params[ctx.<subject>];` then, per column,
/// `ctx.<target> = addUnique(ctx.<target>, p.<column>);`
///
/// Elastic's `gen` emits this as an `ecs_category_type` sub-pipeline for every
/// package that maps a vendor event name onto ECS categorisation, so the table
/// is the params block and a vendor adding an event type is picked up by
/// regenerating rather than by editing Rust.
///
/// `addUnique` is a set union, and the ECS arrays it writes are compared as
/// sets -- `tests/compare-policy.yaml` names both -- so the Java `HashSet`
/// iteration order it comes out in is not reproduced.
///
/// A subject with no row throws in Painless, and the sub-pipeline's `on_failure`
/// turns that into `event.kind: pipeline_error`. Nothing here models the throw,
/// so an unknown event type leaves the event as it was.
fn try_add_unique_row(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(subject) = ctx_path_between(script, "params[ctx.", "]") else {
        return false;
    };
    let bindings = add_unique_bindings(script);
    if bindings.is_empty() {
        return false;
    }

    let Some(key) = event.get_as_string(&subject) else {
        return true;
    };
    let Some(Value::Object(row)) = params.get(&key) else {
        return true;
    };
    let row = row.clone();

    for (target, column) in bindings {
        let Some(Value::Array(additions)) = row.get(&column) else {
            continue;
        };
        let mut merged = match event.get(&target) {
            Some(Value::Array(existing)) => existing.clone(),
            Some(other) => vec![other.clone()],
            None => Vec::new(),
        };
        for addition in additions {
            if !merged.contains(addition) {
                merged.push(addition.clone());
            }
        }
        let _ = event.set(&target, Value::Array(merged));
    }
    true
}

/// Every `ctx.<target> = addUnique(ctx.<target>, p.<column>)` in the script, as
/// the pair it names.
fn add_unique_bindings(script: &str) -> Vec<(String, String)> {
    const CALL: &str = "= addUnique(";

    let mut pairs = Vec::new();
    let mut at = 0;
    while let Some(found) = script[at..].find(CALL) {
        let call = at + found;
        let head = &script[..call];
        at = call + CALL.len();

        // The last `ctx.` before the call is the assignment's left-hand side;
        // the one inside the call is the same path read back.
        let Some(start) = head.rfind("ctx.") else {
            continue;
        };
        let target = clean_path(&head[start + "ctx.".len()..]);

        let Some((_, column)) = script[at..].split_once(',') else {
            continue;
        };
        let Some((_, column)) = column.trim_start().split_once('.') else {
            continue;
        };
        let end = column
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(column.len());
        pairs.push((target, column[..end].to_string()));
    }
    pairs
}

/// `long value = ctx.<source>;` then, per params entry,
/// `if ((value & flag) != 0) { <target>[entry.getKey()] = true; }`
///
/// The params block IS the flag table, so a vendor adding a bit is picked up by
/// regenerating rather than by editing Rust. Only set bits are written; a clear
/// bit leaves the label absent rather than false, which is what the script does.
///
/// panw's decryption-log flags are the shape, and they gate two of its largest
/// remaining blockers -- `labels.nat_translated` and `labels.captive_portal`.
fn try_bit_flags(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(source) = ctx_path_between(script, "long value = ctx.", ";") else {
        return false;
    };
    // The map the labels land in, from `ctx['labels'] = labels;`.
    let target = quoted_after(script, "ctx[").unwrap_or_else(|| "labels".to_owned());

    let Some(bits) = event.get(&source).and_then(as_u64_flags) else {
        return false;
    };

    for (name, flag) in params {
        // Elastic's own script decodes a string flag, because Kibana has been
        // known to hand the params block back with the numbers stringified.
        let Some(flag) = as_u64_flags(flag) else {
            continue;
        };
        if flag != 0 && bits & flag != 0 {
            let path = format!("{target}.{name}");
            if event.set(&path, Value::Bool(true)).is_err() {
                return false;
            }
        }
    }
    true
}

/// A bitfield, however the pipeline happens to carry it.
///
/// A `0x`-prefixed string is what `Long.decode` in the vendor script exists to
/// handle, and a float is what a large flag becomes if it round-trips as JSON.
fn as_u64_flags(value: &Value) -> Option<u64> {
    match value {
        Value::Number(n) => n
            .as_u64()
            .or_else(|| n.as_i64().and_then(|i| u64::try_from(i).ok()))
            .or_else(|| n.as_f64().map(|f| f as u64)),
        Value::String(s) => {
            let s = s.trim();
            s.strip_prefix("0x")
                .or_else(|| s.strip_prefix("0X"))
                .map_or_else(|| s.parse().ok(), |hex| u64::from_str_radix(hex, 16).ok())
        }
        _ => None,
    }
}

/// `ctx.<path>.entrySet().removeIf(entry -> params.<name>.contains(entry.getValue()))`
///
/// The sentinel list is the params entry, so a vendor adding `"-"` to it is
/// picked up by regenerating rather than by editing Rust.
fn try_sentinel_removal(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(path) = ctx_path_before(script, ".entrySet().removeIf(") else {
        return false;
    };
    let Some(Value::Array(sentinels)) = params_ref(script, params, "params.") else {
        return false;
    };
    let sentinels = sentinels.clone();

    // A missing map is not a failure -- the `if` on the processor already
    // guards it, and Elastic's own script returns without touching ctx.
    if let Some(Value::Object(map)) = pointer_mut(event, &path) {
        remove_sentinel_values(map, &sentinels);
    }
    true
}

/// `for (def field : params.<name>) { ctx.<path>[field] = convertToUnix(...) }`
///
/// Only the numeric conversion is modelled: a value that is neither a number
/// nor a digit string is left alone, which is what the vendor script's
/// `instanceof` ladder does.
fn try_filetime_field_list(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(container) = ctx_path_before(script, "[field]") else {
        return false;
    };
    let Some(Value::Array(fields)) = params_ref(script, params, "for (def field : params.") else {
        return false;
    };

    for field in fields.clone() {
        let Some(name) = field.as_str() else { continue };
        let path = format!("{container}.{name}");
        let converted = match event.get(&path) {
            Some(Value::Number(n)) => n.as_i64().map(filetime_to_unix_ms),
            // The script skips a string holding a fractional value.
            Some(Value::String(s)) if !s.contains('.') => {
                s.parse::<i64>().ok().map(filetime_to_unix_ms)
            }
            _ => None,
        };
        if let Some(v) = converted {
            let _ = event.set(&path, v);
        }
    }
    true
}

/// `params.get(<key>)[.get(<key>)...]` then `forEach((k, v) -> ctx.<t>[k] = v)`
///
/// The table is the params block itself, keyed by one field's value per level.
/// `cisco_asa` uses one level for a message id and two for a message id and its
/// outcome, and both end the same way -- the row that survives the chain is
/// merged into `ctx.event`.
///
/// Following only the first level fanned the SECOND level's keys out as if
/// they were fields, which is how `event.denied.action` and its two siblings
/// appeared in place of one `event.action`.
fn try_lookup_merge(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    // The lambda is written both inline and as a braced block that branches on
    // the value's type, so the target is read from the `ctx.<path>[k] =`
    // assignment ANYWHERE after the `forEach` rather than from its head.
    let Some(body) = script.split_once("forEach(").map(|(_, tail)| tail) else {
        return false;
    };
    let Some(target) = ctx_path_between(body, "ctx.", "[k] = ") else {
        return false;
    };
    let keys = get_chain(script);
    if keys.is_empty() {
        return false;
    }

    // The literal writes the script makes on its own account. A script that
    // sets `event.kind`, `event.type` and an outcome before looking the row up
    // -- aws's cloudtrail categorisation is the shape -- was claimed here and
    // only its merge ran, so those three came out missing. The walk skips
    // anything it cannot read, so the lookup and the `forEach` pass it by.
    run_guarded_literals(event, script);

    let mut node = Some(&Value::Null);
    for (level, expr) in keys.iter().enumerate() {
        let Some(key) = resolve_key(event, script, expr) else {
            // The keyed field is absent, and every one of these scripts opens
            // by returning when that is so.
            return true;
        };
        node = if level == 0 {
            params.get(&key)
        } else {
            node.and_then(Value::as_object)
                .and_then(|map| map.get(&key))
        };
        if node.is_none() {
            return true;
        }
    }

    let Some(Value::Object(row)) = node else {
        return true;
    };
    for (k, v) in row.clone() {
        let _ = event.set(&format!("{target}.{k}"), v);
    }
    true
}

/// The arguments of a `params.get(a)[?].get(b)...` chain, outermost first.
///
/// Read from the LAST `params.get(`, because these scripts commonly look the
/// row up once to null-check it and again to use it, and only calls chained
/// directly onto the previous one are levels of the same table.
fn get_chain(script: &str) -> Vec<String> {
    let head = script.split("forEach").next().unwrap_or(script);
    let Some(start) = head.rfind("params.get(") else {
        return Vec::new();
    };
    let mut rest = &head[start + "params".len()..];
    let mut keys = Vec::new();
    loop {
        let Some(after) = rest
            .strip_prefix(".get(")
            .or_else(|| rest.strip_prefix("?.get("))
        else {
            return keys;
        };
        let mut depth = 1usize;
        let mut end = None;
        for (index, c) in after.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(index);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(end) = end else { return keys };
        keys.push(after[..end].trim().to_string());
        rest = &after[end + 1..];
    }
}

/// `def row = params.get('<table>').get(ctx.<subject>);` then a column each:
/// `def c = row.get('<key>'); for (def x : c) { ctx.<path>.add(x) }` or
/// `ctx.<path> = c;`, with `ctx.<path> = ctx.<subject>` when the row is absent.
///
/// Cisco Meraki's event map is the shape: one vendor subtype expands into an
/// ECS action plus additions to `event.type` and `event.category`. Appending
/// matters -- the pipeline has already put `info` in `event.type`, and a
/// replacing write drops it.
fn try_lookup_columns(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(Value::Object(table)) = params_ref(script, params, "params.get('") else {
        return false;
    };
    // The table is bound to a local first, and the row lookup goes through it.
    let Some(table_local) = script
        .split_once("def ")
        .and_then(|(_, rest)| rest.split_once(" = params.get("))
        .map(|(name, _)| name.trim().to_string())
    else {
        return false;
    };
    let Some(key_expr) = last_call_argument(script, &format!("{table_local}.get(")) else {
        return false;
    };
    let Some(key) = resolve_key(event, script, &key_expr) else {
        return true;
    };

    let Some(Value::Object(row)) = table.get(&key) else {
        // Every one of these scripts falls back to writing the raw subtype.
        if let Some(target) = fallback_target(script) {
            let _ = event.set(&target, Value::String(key));
        }
        return true;
    };
    let row = row.clone();

    for (local, column) in column_bindings(script) {
        let Some(value) = row.get(&column) else {
            continue;
        };
        match appended_target(script, &local) {
            Some(path) => {
                let mut existing = match event.get(&path) {
                    Some(Value::Array(a)) => a.clone(),
                    _ => Vec::new(),
                };
                match value {
                    Value::Array(items) => existing.extend(items.iter().cloned()),
                    other => existing.push(other.clone()),
                }
                let _ = event.set(&path, Value::Array(existing));
            }
            None => {
                if let Some(path) = assigned_target(script, &local) {
                    let _ = event.set(&path, value.clone());
                }
            }
        }
    }
    true
}

/// `def <local> = <row>.get('<column>')` pairs, minus the row lookup itself.
fn column_bindings(script: &str) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for segment in script.split("def ").skip(1) {
        let Some((local, rest)) = segment.split_once(" = ") else {
            continue;
        };
        if rest.contains("params") {
            continue;
        }
        let Some(column) = rest
            .split_once(".get('")
            .and_then(|(_, s)| s.split_once('\''))
        else {
            continue;
        };
        found.push((local.trim().to_string(), column.0.to_string()));
    }
    found
}

/// The `ctx.` path a `for (def x : <local>) { ctx.<path>.add(x) }` appends to.
fn appended_target(script: &str, local: &str) -> Option<String> {
    let needle = format!(" : {local})");
    let (_, tail) = script.split_once(&needle)?;
    let end = tail.find('}')?;
    ctx_path_before(&tail[..end], ".add(")
}

/// The `ctx.` path a bare `ctx.<path> = <local>;` assignment writes.
fn assigned_target(script: &str, local: &str) -> Option<String> {
    let needle = format!("= {local};");
    ctx_path_before(script, &needle)
}

/// The `ctx.` path the no-row branch writes the raw subject into.
fn fallback_target(script: &str) -> Option<String> {
    let (head, _) = script.split_once("== null")?;
    let (_, tail) = script[head.len()..].split_once('{')?;
    ctx_path_before(tail, " = ")
}

/// `def k = ctx.<path>.toLowerCase(); def v = params.get(k);`
/// `if (v != null) { ctx.<path> = v; return; } ctx.<path> = k;`
///
/// A vendor-vocabulary-to-ECS map: fortinet's `outgoing` is ECS `outbound`.
/// The fallback writes the LOOKUP KEY back, not the original, so a value the
/// table misses still comes out lower-cased -- and the pipeline's own
/// allow-list check downstream then sees the same string Elastic would.
fn try_lookup_normalise(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    // `params[key]` and `params.get(key)` are the same operation in Painless
    // and the integrations use both -- sysmon's DNS status table is written
    // with the brackets.
    let Some(key_expr) = last_call_argument(script, "params.get(")
        .or_else(|| last_bracket_subscript(script, "params["))
    else {
        return false;
    };
    // The field assigned from the lookup, falling back to the last assignment
    // where the lookup is bound to a local first.
    let writes = ctx_writes(script);
    let target = writes
        .iter()
        .find(|(_, rhs)| rhs.contains("params.get(") || rhs.contains("params["))
        .map(|(path, _)| path.clone())
        .or_else(|| writes.last().map(|(path, _)| path.clone()));
    let Some(target) = target else {
        return false;
    };
    let Some(key) = resolve_key(event, script, &key_expr) else {
        return true;
    };

    let value = params.get(&key).cloned().unwrap_or(Value::String(key));
    let _ = event.set(&target, value);

    // Whatever else the script writes on its own account, AFTER the lookup so
    // a `ctx.x = null` that clears the field the key came from is not read
    // before it is used. mimecast's siem_logs is that shape exactly.
    run_guarded_literals(event, script);
    true
}

/// The first member of a params list the subject contains, written to a field.
///
/// Microsoft's Defender pipelines classify by substring: an OS platform that
/// contains `linux` is Linux, a vulnerability id that contains `CVE` is a CVE.
/// The list is the params, and a trailing block of literal `contains` tests
/// catches what the list misses.
fn try_first_contained_member(
    event: &mut Event,
    script: &str,
    params: &Map<String, Value>,
) -> bool {
    let Some((subject, folded)) = subject_of_binding(script) else {
        return false;
    };
    let Some(text) = event.get_as_string(&subject) else {
        return false;
    };
    let text = if folded {
        text.to_lowercase()
    } else {
        text.to_uppercase()
    };

    let Some(members) = params_ref(script, params, "params.").and_then(Value::as_array) else {
        return false;
    };
    let Some(target) = put_target(script) else {
        return false;
    };

    for member in members {
        let Some(member) = member.as_str() else {
            continue;
        };
        if text.contains(member) {
            let _ = event.set(&target, member);
            return true;
        }
    }

    // The tail: `if (x.contains('centos') || ...) { ctx.a.put('b', 'linux'); }`
    // Its subject is the same local, already folded, so the literals are
    // tested against the text in hand rather than re-read from the event.
    for block in script.split("if (").skip(1) {
        // `) {` ends the header; the first `)` alone sits inside `contains(`.
        let Some((guard, body)) = block.split_once(") {") else {
            continue;
        };
        if !guard.contains(".contains(") {
            continue;
        }
        let matched = guard
            .split("||")
            .filter_map(|term| quoted_after(term, ".contains("))
            .any(|literal| text.contains(&literal));
        if matched && let Some(literal) = quoted_after(body, ", ") {
            let _ = event.set(&target, literal);
            break;
        }
    }
    true
}

/// The `ctx.` path a `String x = ctx.<path>[.toLowerCase()];` binds, and
/// whether it was folded DOWN rather than up.
fn subject_of_binding(script: &str) -> Option<(String, bool)> {
    let at = script.find("String ")?;
    let statement = script[at..].split(';').next()?;
    let (_, bound) = statement.split_once('=')?;
    let bound = bound.trim();
    let folded = bound.contains(".toLowerCase()");
    let path = bound
        .replace(".toLowerCase()", "")
        .replace(".toUpperCase()", "");
    Some((clean_path(path.trim().strip_prefix("ctx.")?), folded))
}

/// The field a `ctx.<path>.put('<key>', ...)` writes.
fn put_target(script: &str) -> Option<String> {
    let path = ctx_path_before(script, ".put(")?;
    let key = quoted_after(script, ".put(")?;
    Some(format!("{path}.{key}"))
}

/// Rename an object's keys, recursively, through a name map.
///
/// Windows hands its event data over in the vendor's own casing --
/// `AdditionalInfo`, `QNAME`, `XID` -- and the pipeline renames the lot in one
/// pass before anything else reads them. Everything downstream is keyed on the
/// renamed form, so a miss here costs far more than the keys themselves.
///
/// A key the map does not name keeps its own, and a value that is not an
/// object or a list is carried across untouched.
fn try_rename_keys(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some((path, _)) = ctx_writes(script)
        .into_iter()
        .find(|(_, rhs)| rhs.contains(", params)"))
    else {
        return false;
    };
    let Some(subject) = event.get(&path).cloned() else {
        // The processor's own `if` guards the object being absent.
        return false;
    };
    let _ = event.set(&path, renamed_keys(&subject, params));
    true
}

/// `value` with every key the map names replaced, at any depth.
fn renamed_keys(value: &Value, names: &Map<String, Value>) -> Value {
    match value {
        Value::Object(members) => Value::Object(
            members
                .iter()
                .map(|(key, member)| {
                    let key = names
                        .get(key)
                        .and_then(Value::as_str)
                        .unwrap_or(key)
                        .to_string();
                    (key, renamed_keys(member, names))
                })
                .collect(),
        ),
        Value::Array(items) => {
            Value::Array(items.iter().map(|item| renamed_keys(item, names)).collect())
        }
        other => other.clone(),
    }
}

/// Normalise several fields, each through a value map of its own.
///
/// The params are keyed by SOURCE PATH, and each entry carries the map and,
/// optionally, a different destination. Cisco's FTD uses it to turn the DNS
/// record type the device spells out -- "a host address" -- into the mnemonic
/// ECS wants, and the response code likewise.
///
/// A key is looked up folded to lower case, because that is what the script
/// does; a path that names no field is skipped, which is how the entry keyed
/// `ctx._temp_.cisco.message_id` behaves upstream as well. Its `ctx.` prefix
/// makes the script look for a field of that name, and there is none.
fn try_value_maps(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let folded = script.contains(".toLowerCase()");
    for (source, entry) in params {
        let Some(entry) = entry.as_object() else {
            continue;
        };
        let Some(table) = entry.get("map").and_then(Value::as_object) else {
            continue;
        };
        // A list contributes its first member, as the script reads it.
        let current = match event.get(source) {
            Some(Value::Array(items)) => items.first().and_then(scalar_text),
            Some(value) => scalar_text(value),
            None => None,
        };
        let Some(current) = current else {
            continue;
        };
        let key = if folded {
            current.to_lowercase()
        } else {
            current
        };
        if let Some(replacement) = table.get(&key) {
            let target = entry
                .get("target")
                .and_then(Value::as_str)
                .unwrap_or(source.as_str());
            let _ = event.set(target, replacement.clone());
        }
    }
    true
}

/// Write one params row's columns onto ctx, then refine them by outcome.
///
/// This is Elastic's ECS categorisation shape: the vendor action selects a row
/// giving `event.kind`, `event.category` and `event.type`, and a tail of
/// guarded statements then adds `allowed` or `denied` and rewrites the outcome
/// into an ECS one. It is `cisco_ftd`'s remaining 398 corpus events, and the
/// same shape appears wherever a package maps an action onto categorisation.
fn try_row_columns(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(key_path) = ctx_path_between(script, "params.get(ctx.", ")") else {
        return false;
    };
    let Some(key) = event.get_as_string(&key_path) else {
        return true;
    };
    let Some(row) = params.get(&key).and_then(Value::as_object).cloned() else {
        // Every one of these scripts returns early on a key it has no row for.
        return true;
    };

    let mut wrote = false;
    for (path, rhs) in ctx_writes(script) {
        if !rhs.contains("params.get(") {
            continue;
        }
        let Some(column) = rhs
            .rsplit_once(".get('")
            .and_then(|(_, s)| s.split_once('\''))
        else {
            continue;
        };
        if let Some(value) = row.get(column.0) {
            let _ = event.set(&path, value.clone());
            wrote = true;
        }
    }
    if !wrote {
        return false;
    }

    // Everything after the last table read is the refinement tail.
    if let Some(cut) = script.rfind("params.get(") {
        let tail = &script[cut..];
        if let Some((_, rest)) = tail.split_once(';') {
            run_guarded_literals(event, rest);
        }
    }
    true
}

/// Run a tail of `if (<test>) { ... }` blocks over literal appends and writes.
///
/// The grammar is deliberately tiny, because that is all these tails do once
/// the row is on the event: compare a field to a literal, and append or assign
/// another literal. Anything outside it is left alone rather than guessed at.
/// Returns whether anything was written, so a caller can tell a script it
/// read from one it walked past.
pub(crate) fn run_guarded_literals(event: &mut Event, body: &str) -> bool {
    let mut wrote = false;
    let mut rest = body;
    while let Some(offset) = rest.find(|c: char| !c.is_whitespace()) {
        rest = &rest[offset..];
        if let Some(after) = rest.strip_prefix("return") {
            let _ = after;
            return wrote;
        }
        if let Some(after) = rest.strip_prefix("if") {
            let Some((test, after)) = balanced(after.trim_start(), '(', ')') else {
                return wrote;
            };
            let Some((block, after)) = balanced(after.trim_start(), '{', '}') else {
                return wrote;
            };
            // An `else` arm when there is one. `else if` has no braces of its
            // own, so the whole tail becomes the alternative and the recursion
            // reads it as another `if`.
            let (alternative, after) = match after.trim_start().strip_prefix("else") {
                Some(tail) => match balanced(tail.trim_start(), '{', '}') {
                    Some((body, rest)) => (Some(body), rest),
                    None => (Some(tail), ""),
                },
                None => (None, after),
            };

            if guard_holds(event, test) {
                wrote |= run_guarded_literals(event, block);
            } else if let Some(body) = alternative {
                wrote |= run_guarded_literals(event, body);
            }
            rest = after;
            continue;
        }
        // A plain statement, up to its terminator.
        let end = rest.find(';').unwrap_or(rest.len());
        wrote |= run_literal_statement(event, &rest[..end]);
        rest = &rest[(end + 1).min(rest.len())..];
    }
    wrote
}

/// Split `text` at the region opened by `open` and closed by its match.
pub(crate) fn balanced(text: &str, open: char, close: char) -> Option<(&str, &str)> {
    let mut chars = text.char_indices();
    let (_, first) = chars.next()?;
    if first != open {
        return None;
    }
    let mut depth = 1usize;
    for (index, c) in chars {
        if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return Some((
                    &text[open.len_utf8()..index],
                    &text[index + close.len_utf8()..],
                ));
            }
        }
    }
    None
}

/// Rewrite Painless's map syntax into the dotted form paths are read in.
///
/// Only where it is subscripting with a literal: `ctx['@timestamp']` is a path
/// and `params[net.transport]` is a lookup, and the difference is the quotes.
fn subject_path(term: &str) -> String {
    // Null-safe navigation goes too: `ctx?.event` is `ctx.event`, and leaving
    // the `?` on defeats the `ctx.` prefix every reader below strips.
    term.replace("?.", ".")
        .replace("['", ".")
        .replace("[\"", ".")
        .replace("']", "")
        .replace("\"]", "")
}

/// Evaluate one `if` test: `||` of `&&` of comparisons against literals.
pub(crate) fn guard_holds(event: &Event, test: &str) -> bool {
    test.split("||").any(|conjunction| {
        conjunction
            .split("&&")
            .all(|term| term_holds(event, term.trim()))
    })
}

/// One comparison, with `!` handled by inverting what it wraps.
fn term_holds(event: &Event, term: &str) -> bool {
    if let Some(inner) = term.strip_prefix('!') {
        // `!x.contains(y)` -- a bare `!ctx.field` is not a shape these use.
        return !term_holds(event, inner.trim());
    }
    // `ctx['@timestamp']` and `ctx.event.action` name the same kind of thing.
    let term = &subject_path(term);
    if let Some((subject, literal)) = term.split_once(".contains(") {
        // The argument is a literal or another field -- `!ctx.related.user
        // .contains(ctx.winlog.event_data.SubjectUserName)` is the append-once
        // guard these scripts use.
        let Some(wanted) = quoted_after(literal, "").or_else(|| {
            let argument = literal.trim().trim_end_matches([')', ';']).trim();
            let path = argument.strip_prefix("ctx.")?;
            event.get_as_string(&clean_path(path))
        }) else {
            return false;
        };
        let Some(path) = subject.trim().strip_prefix("ctx.") else {
            return false;
        };
        return match event.get(&clean_path(path)) {
            Some(Value::Array(items)) => items.iter().any(|i| i.as_str() == Some(&wanted)),
            Some(Value::String(text)) => text.contains(&wanted),
            _ => false,
        };
    }
    for (operator, negated) in [("==", false), ("!=", true)] {
        let Some((subject, wanted)) = term.split_once(operator) else {
            continue;
        };
        let Some(path) = subject.trim().strip_prefix("ctx.") else {
            return false;
        };
        let held = event.get(&clean_path(path));
        let wanted = wanted.trim();
        let matched = if wanted == "null" {
            held.is_none_or(Value::is_null)
        } else {
            quoted_after(wanted, "")
                .is_some_and(|literal| held.and_then(Value::as_str) == Some(literal.as_str()))
        };
        return matched != negated;
    }
    false
}

/// One statement that writes a value the script already has to hand.
///
/// Four shapes, and the value is either a literal or another `ctx.` field:
/// `ctx.a.add(v)`, `ctx.a.put('k', v)`, `ctx.a = v`, and the `.put` and `.add`
/// forms with a copied source. Anything else is left alone.
fn run_literal_statement(event: &mut Event, statement: &str) -> bool {
    if let Some((subject, argument)) = statement.split_once(".add(") {
        let (Some(path), Some(value)) = (
            subject.trim().strip_prefix("ctx."),
            written_value(event, argument),
        ) else {
            return false;
        };
        append_or_create(event, &clean_path(path), value);
        return true;
    }
    // `ctx.user.put("name", ctx.winlog.event_data.SubjectUserName)`. The
    // null-guard blocks these scripts open with -- `ctx.put("user", hm)` --
    // fall out here: the subject is bare `ctx` and the value is a local.
    if let Some((subject, arguments)) = statement.split_once(".put(") {
        let Some(parent) = subject.trim().strip_prefix("ctx.") else {
            return false;
        };
        let (Some(key), Some((_, rest))) = (quoted_after(arguments, ""), arguments.split_once(','))
        else {
            return false;
        };
        let Some(value) = written_value(event, rest) else {
            return false;
        };
        let _ = event.set(&format!("{}.{key}", clean_path(parent)), value);
        return true;
    }
    let Some((subject, value)) = split_assignment(statement) else {
        return false;
    };
    let Some(path) = subject.trim().strip_prefix("ctx.") else {
        return false;
    };
    let Some(value) = written_value(event, value) else {
        return false;
    };
    let _ = event.set(&clean_path(path), value);
    true
}

/// The value a statement writes: a literal, or a `ctx.` field read off the
/// event.
fn written_value(event: &Event, text: &str) -> Option<Value> {
    let text = text.trim().trim_end_matches([')', ';']).trim();
    if let Some(path) = text.strip_prefix("ctx.") {
        // A source path and nothing else. `ctx.a + ctx.b` and a method call
        // on one are different shapes with their own matchers.
        if path
            .chars()
            .all(|c| c.is_alphanumeric() || "._?@['\"]".contains(c))
        {
            return event.get(&clean_path(path)).cloned();
        }
        return None;
    }
    literal_value(text)
}

/// A quoted string, or a bracketed list of them.
///
/// `ctx.event.type = ['info']` is as common as the scalar form and was read as
/// the bare string `info`, so the field came out a string where Elastic writes
/// a one-element array.
fn literal_value(text: &str) -> Option<Value> {
    let text = text.trim();
    // Painless reads a present-but-null field as null, and several pipelines
    // write one deliberately so their own drop-empty pass takes the field.
    if text == "null" {
        return Some(Value::Null);
    }
    let Some(inner) = text
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
    else {
        return quoted_after(text, "").map(Value::String);
    };
    let members: Vec<Value> = inner
        .split(',')
        .filter_map(|member| quoted_after(member, ""))
        .map(Value::String)
        .collect();
    (!members.is_empty()).then_some(Value::Array(members))
}

/// Fan an already-parsed key/value message out through a params table.
///
/// Cisco's FTD security events arrive as `Key: value, Key: value` pairs that an
/// earlier processor lifts into a map. Each vendor key has a row in params
/// naming the vendor field it becomes, the ECS fields it also feeds, and the
/// message ids it is evidence for -- and when the header carried no id, the id
/// with the most evidence is the message's. Nothing about which keys exist is
/// written here: it is all read from the params the pipeline ships.
///
/// This one script is the whole security-event family -- connection, file,
/// malware, intrusion and DNS -- 398 of `cisco_ftd`'s 432 corpus events.
fn try_keyed_message_table(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let script = expand_locals(script);
    let Some(source) = ctx_path_before(&script, ".entrySet()") else {
        return false;
    };
    let Some(message) = event.get(&source).and_then(Value::as_object).cloned() else {
        // The processor's `if` guards the map being absent, so nothing to read
        // means the path came out wrong rather than the event being quiet.
        return false;
    };

    // Which of the two maps a row lands in is decided by a list written into
    // the script, and each map's destination by the local it was assigned to.
    let writes = ctx_writes(&script);
    let Some(list_local) = identifier_before(&script, ".contains(param.target)") else {
        return false;
    };
    let listed = script_string_list(&script, &list_local);
    let Some(listed_local) = local_indexed_by(&script, "contains(param.target)){") else {
        return false;
    };
    let Some(other_local) = local_indexed_by(&script, "else{") else {
        return false;
    };
    let destination = |local: &str| {
        writes
            .iter()
            .find(|(_, rhs)| rhs == local)
            .map(|(path, _)| path.clone())
    };
    let (Some(listed_path), Some(other_path)) =
        (destination(&listed_local), destination(&other_local))
    else {
        return false;
    };

    let mut listed_map = Map::new();
    let mut other_map = Map::new();
    let mut counters: Vec<(String, usize)> = Vec::new();

    for (key, value) in &message {
        let Some(row) = params.get(key).and_then(Value::as_object) else {
            continue;
        };
        // Counted even for an empty value: the key's presence is the evidence.
        for id in row
            .get("id")
            .and_then(Value::as_array)
            .unwrap_or(&Vec::new())
        {
            let Some(id) = id.as_str() else { continue };
            match counters.iter_mut().find(|(name, _)| name == id) {
                Some((_, count)) => *count += 1,
                None => counters.push((id.to_string(), 1)),
            }
        }
        if is_painless_empty(value) {
            continue;
        }
        for field in row
            .get("ecs")
            .and_then(Value::as_array)
            .unwrap_or(&Vec::new())
        {
            if let Some(field) = field.as_str() {
                append_or_create(event, field, value.clone());
            }
        }
        let Some(target) = row.get("target").and_then(Value::as_str) else {
            continue;
        };
        if listed.iter().any(|name| name == target) {
            listed_map.insert(target.to_string(), value.clone());
        } else {
            other_map.insert(target.to_string(), value.clone());
        }
    }

    let _ = event.set(&listed_path, Value::Object(listed_map));
    let _ = event.set(&other_path, Value::Object(other_map));

    // The header's own id wins where there was one; the vote is the fallback.
    let Some((decided, _)) = writes.iter().find(|(_, rhs)| rhs.ends_with("getKey()")) else {
        return true;
    };
    if event
        .get_as_string(decided)
        .is_some_and(|current| !current.is_empty())
    {
        return true;
    }
    if let Some((best, _)) = counters.iter().max_by_key(|(_, count)| *count) {
        let _ = event.set(decided, best.clone());
    }
    true
}

/// Painless `isEmpty`: no members, or no characters.
fn is_painless_empty(value: &Value) -> bool {
    match value {
        Value::Array(items) => items.is_empty(),
        Value::String(text) => text.is_empty(),
        Value::Null => true,
        _ => false,
    }
}

/// Write `value` at `path`, growing a list where one is already there.
fn append_or_create(event: &mut Event, path: &str, value: Value) {
    let grown = match event.get(path) {
        None => value,
        Some(Value::Array(existing)) => {
            let mut items = existing.clone();
            items.push(value);
            Value::Array(items)
        }
        Some(existing) => Value::Array(vec![existing.clone(), value]),
    };
    let _ = event.set(path, grown);
}

/// The quoted strings of the `new ArrayList([...])` bound to `local`.
///
/// Named rather than positional: the helper the vendor defines above the loop
/// builds its own `new ArrayList([existing, value])`, and taking the first
/// literal in the script finds that one and reads an empty list out of it.
fn script_string_list(script: &str, local: &str) -> Vec<String> {
    let binding = format!("def {local} = new ArrayList([");
    let Some(start) = script.find(&binding) else {
        return Vec::new();
    };
    let tail = &script[start + binding.len()..];
    let end = tail.find("])").unwrap_or(tail.len());
    tail[..end]
        .split(',')
        .filter_map(|item| {
            let item = item.trim();
            item.strip_prefix('\'')
                .and_then(|item| item.strip_suffix('\''))
                .map(str::to_string)
        })
        .collect()
}

/// The identifier immediately preceding `marker`.
fn identifier_before(script: &str, marker: &str) -> Option<String> {
    let head = &script[..script.find(marker)?];
    let name: String = head
        .chars()
        .rev()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    (!name.is_empty()).then(|| name.chars().rev().collect())
}

/// The local a `<local>[param.target]` write names, just after `marker`.
fn local_indexed_by(script: &str, marker: &str) -> Option<String> {
    let tail = &script[script.find(marker)? + marker.len()..];
    let name = tail[..tail.find("[param.target]")?].trim();
    (!name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .then(|| name.to_string())
}

/// A table read in either direction, depending on which side the field holds.
///
/// Cisco's ASA and FTD pipelines map `network.transport` to its IANA protocol
/// number, and if the device wrote the NUMBER there instead, put the number in
/// `network.iana_number` and the name back in `network.transport`. It is worth
/// its own matcher because it is the difference between a connection event
/// having a protocol and not: it was wrong in 414 of 512 ASA events.
///
/// Everything is read out of the script -- which field, which two destinations,
/// and the table itself -- so the vendor renaming or extending any of them is
/// picked up by regenerating rather than by editing this.
fn try_reversible_lookup(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let script = expand_locals(script);
    let Some(source) = ctx_path_between(&script, "params[ctx.", "]") else {
        return false;
    };
    let Some(forward_local) = local_bound_to(&script, "params[") else {
        return false;
    };
    let writes = ctx_writes(&script);

    let Some(forward_target) = writes
        .iter()
        .find(|(_, rhs)| *rhs == forward_local)
        .map(|(path, _)| path.clone())
    else {
        return false;
    };

    let Some(key) = event.get_as_string(&source) else {
        // The processor's own `if` guards the field being absent, so reaching
        // here with nothing to read means the path came out wrong.
        return false;
    };

    if let Some(value) = params.get(&key) {
        let _ = event.set(&forward_target, value.clone());
        return true;
    }

    // The reverse table is built by value, so the field already holds a number
    // and the name has to be put back. Both destinations come from the script:
    // the one assigned the field itself, and the one assigned the lookup.
    let Some(name) = params
        .iter()
        .find(|(_, value)| scalar_text(value).as_deref() == Some(key.as_str()))
        .map(|(name, _)| name.clone())
    else {
        return true;
    };
    let echo = format!("ctx.{source}");
    for (path, rhs) in &writes {
        if *rhs == echo {
            // What the field already held, moved across untouched.
            let _ = event.set(path, key.clone());
        } else if *rhs != forward_local
            && !rhs.is_empty()
            && rhs.chars().all(|c| c.is_alphanumeric() || c == '_')
        {
            // The only other local in play is the reverse lookup's result.
            let _ = event.set(path, name.clone());
        }
    }
    true
}

/// Rewrite `def x = ctx.a.b;` bindings back into the paths they alias.
///
/// The vendor binds a subtree to a local and then writes through it, so every
/// destination in the script reads `net['iana_number']` rather than a `ctx.`
/// path. Substituting the binding puts them all back in one form.
fn expand_locals(script: &str) -> String {
    let mut out = script.to_string();
    for statement in script.split(';') {
        // A statement carries the previous block's closing brace, so the
        // binding is found from the LAST `def ` in it, not the start.
        let Some(start) = statement.rfind("def ") else {
            continue;
        };
        let Some((name, bound)) = statement[start + "def ".len()..].split_once('=') else {
            continue;
        };
        let (name, bound) = (name.trim(), bound.trim());
        if !bound.starts_with("ctx.")
            || bound.contains('(')
            || bound.contains('[')
            || name.contains(|c: char| !c.is_alphanumeric() && c != '_')
        {
            continue;
        }
        out = out
            .replace(&format!("{name}."), &format!("{bound}."))
            .replace(&format!("{name}["), &format!("{bound}["));
    }
    out
}

/// The local a `def x = <marker>...` statement binds.
fn local_bound_to(script: &str, marker: &str) -> Option<String> {
    let head = &script[..script.find(marker)?];
    let statement = head.rsplit(';').next()?;
    let rest = statement.trim().strip_prefix("def ")?;
    let name = rest.split('=').next()?.trim();
    (!name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .then(|| name.to_string())
}

/// Every `ctx.<path> = <expression>` in the script, as `(path, expression)`.
///
/// The path is normalised out of Painless's map syntax, so `ctx.network
/// ['iana_number']` and `ctx.network.iana_number` come back the same.
pub(crate) fn ctx_writes(script: &str) -> Vec<(String, String)> {
    let mut writes = Vec::new();
    for statement in script.split(';') {
        let Some((lhs, rhs)) = split_assignment(statement) else {
            continue;
        };
        let Some(start) = lhs.rfind("ctx.") else {
            continue;
        };
        let path = lhs[start + "ctx.".len()..]
            .replace("['", ".")
            .replace("[\"", ".")
            .replace("']", "")
            .replace("\"]", "");
        writes.push((clean_path(&path), rhs.trim().to_string()));
    }
    writes
}

/// Split a statement at its assignment, ignoring every comparison.
///
/// A guarded write reads `if (x != null) { ctx.a.b = c`, so the first `=` in
/// the text belongs to the comparison and splitting there loses the write.
fn split_assignment(statement: &str) -> Option<(&str, &str)> {
    let bytes = statement.as_bytes();
    for (i, byte) in bytes.iter().enumerate() {
        if *byte != b'=' {
            continue;
        }
        let before = if i == 0 { b' ' } else { bytes[i - 1] };
        let after = *bytes.get(i + 1).unwrap_or(&b' ');
        if matches!(before, b'=' | b'!' | b'<' | b'>') || after == b'=' {
            continue;
        }
        return Some((&statement[..i], &statement[i + 1..]));
    }
    None
}

/// A scalar's text, the way Painless would stringify it for a map key.
fn scalar_text(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

/// `ctx.<target>.put('<key>', params['<name>'][<numeric field>])`
///
/// The bounds check the vendor writes around it is the array's own length, so
/// an index outside it simply leaves the field unset.
fn try_indexed_lookup(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(container) = ctx_path_before(script, ".put(") else {
        return false;
    };
    let Some(key) = quoted_after(script, ".put(") else {
        return false;
    };
    let Some(Value::Array(table)) = params_indexed(script, params) else {
        return false;
    };
    // The index is the only ctx field the script reads, and the processor's own
    // `if` guards it being absent -- so a value we cannot read means the path
    // came out wrong, and saying "handled" would hide that.
    let Some(index_path) = ctx_path_before(script, ";") else {
        return false;
    };
    let Some(index) = event
        .get_as_string(&index_path)
        .and_then(|s| s.trim().parse::<usize>().ok())
    else {
        return false;
    };

    if let Some(value) = table.get(index).cloned() {
        let _ = event.set(&format!("{container}.{key}"), value);
    }
    true
}

/// `ctx.<field> = ctx.<field> * params.<name>`
fn try_scale(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some((head, _)) = script.split_once("* params.") else {
        return false;
    };
    let Some(path) = crate::painless_common::painless_path(head) else {
        return false;
    };
    let Some(factor) = params_ref(script, params, "* params.").and_then(Value::as_f64) else {
        return false;
    };
    let Some(current) = event.get(&path).and_then(Value::as_f64) else {
        return true;
    };

    let scaled = current * factor;
    // Whole results stay integers: a duration in nanoseconds is not a float.
    if scaled.fract() == 0.0 && scaled.abs() < 9.007_199_254_740_992e15 {
        #[allow(clippy::cast_possible_truncation)]
        let _ = event.set(&path, scaled as i64);
    } else {
        let _ = event.set(&path, scaled);
    }

    // The same script often derives an end instant from the scaled duration.
    #[allow(clippy::cast_possible_truncation)]
    if script.contains(".plusNanos(") {
        add_nanos(event, script, scaled as i64);
    }
    true
}

/// `ctx.<target> = ZonedDateTime.parse(<start>).plusNanos(nanos)`
///
/// The start instant is a ctx field bound to a local earlier in the script.
fn add_nanos(event: &mut Event, script: &str, nanos: i64) {
    use crate::painless_common::painless_path;

    use crate::painless_common::ctx_path_bound_to;

    let Some((head, _)) = script.split_once(".plusNanos(") else {
        return;
    };
    // `parse(<local>)`, where the local was bound to a ctx path earlier.
    let Some(source) = head
        .rsplit_once("parse(")
        .map(|(_, name)| name.trim_end_matches(')').trim())
        .and_then(|name| ctx_path_bound_to(script, name).or_else(|| painless_path(name)))
    else {
        return;
    };
    // The assignment target is whatever sits left of the `=` on this line.
    let Some(target) = head
        .rsplit_once('=')
        .and_then(|(lhs, _)| painless_path(lhs))
    else {
        return;
    };
    let Some(start) = event.get_as_string(&source) else {
        return;
    };
    let Ok(start) = chrono::DateTime::parse_from_rfc3339(&start) else {
        return;
    };

    let end = start + chrono::TimeDelta::nanoseconds(nanos);
    let _ = event.set(
        &target,
        Value::String(
            end.with_timezone(&chrono::Utc)
                .format("%Y-%m-%dT%H:%M:%S%.3f%:z")
                .to_string(),
        ),
    );
}

/// `ctx.<field> = ctx.<field>.replace(params.<name>, '<replacement>')`
fn try_replace(event: &mut Event, script: &str, params: &Map<String, Value>) -> bool {
    let Some(path) = ctx_path_before(script, ".replace(params.") else {
        return false;
    };
    let Some(needle) = params_ref(script, params, ".replace(params.").and_then(Value::as_str)
    else {
        return false;
    };
    let replacement = quoted_after(script, ",").unwrap_or_default();
    let Some(current) = event.get_str(&path) else {
        return true;
    };

    let replaced = current.replace(needle, &replacement);
    let _ = event.set(&path, replaced);
    true
}

/// The first single- or double-quoted string following `after`.
fn quoted_after(script: &str, after: &str) -> Option<String> {
    let tail = &script[script.find(after)? + after.len()..];
    let start = tail.find(['\'', '"'])?;
    let quote = tail.as_bytes()[start] as char;
    let end = tail[start + 1..].find(quote)?;
    Some(tail[start + 1..=start + end].to_string())
}

/// Resolve the expression inside `params.get(...)` to a table key.
///
/// It is either a `ctx.` path written inline or a `def` bound to one earlier in
/// the script, and either may be lower-cased before the lookup.
fn resolve_key(event: &Event, script: &str, expr: &str) -> Option<String> {
    let lower = expr.contains(".toLowerCase()");
    let expr = expr.replace(".toLowerCase()", "");
    let expr = expr.trim();

    let (path, lower) = if let Some(rest) = expr.strip_prefix("ctx.") {
        (rest.to_string(), lower)
    } else {
        let binding = format!("def {expr} = ctx.");
        let start = script.find(&binding)? + binding.len();
        let tail = &script[start..];
        let end = tail.find([';', '\n']).unwrap_or(tail.len());
        let bound = &tail[..end];
        (
            bound.replace(".toLowerCase()", "").trim().to_string(),
            lower || bound.contains(".toLowerCase()"),
        )
    };

    let value = event.get_as_string(&clean_path(&path))?;
    Some(if lower { value.to_lowercase() } else { value })
}

/// The params entry an indexed reference names, in either form Painless allows:
/// `params['LogLevel'][i]` or `params.LogLevel[i]`.
fn params_indexed<'a>(script: &str, params: &'a Map<String, Value>) -> Option<&'a Value> {
    if let Some(name) = quoted_after(script, "params[") {
        return params.get(&name);
    }
    params_ref(script, params, "params.")
}

/// The params entry a `params.<name>` reference names, given its lead-in text.
fn params_ref<'a>(script: &str, params: &'a Map<String, Value>, prefix: &str) -> Option<&'a Value> {
    let start = script.find(prefix)? + prefix.len();
    let tail = &script[start..];
    let end = tail
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .unwrap_or(tail.len());
    params.get(&tail[..end])
}

/// The dotted `ctx.` path that immediately precedes `marker`.
pub(crate) fn ctx_path_before(script: &str, marker: &str) -> Option<String> {
    let end = script.find(marker)?;
    let head = &script[..end];
    let start = head.rfind("ctx.")? + "ctx.".len();
    Some(clean_path(&head[start..]))
}

/// The dotted `ctx.` path written between two markers.
pub(crate) fn ctx_path_between(script: &str, open: &str, close: &str) -> Option<String> {
    let start = script.find(open)? + open.len();
    let tail = &script[start..];
    let end = tail.find(close)?;
    Some(clean_path(&tail[..end]))
}

/// The argument of the LAST call to `name(`, balanced across nested parens.
///
/// The table lookup is often written twice -- once to null-check, once to use
/// -- and it is the second that feeds the merge.
fn last_call_argument(script: &str, name: &str) -> Option<String> {
    let start = script.rfind(name)? + name.len();
    let mut depth = 1usize;
    for (i, c) in script[start..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(script[start..start + i].to_string());
                }
            }
            _ => {}
        }
    }
    None
}

/// The last `name[...]` subscript's contents, brackets balanced.
///
/// The bracket twin of [`last_call_argument`]: a Painless map reads the same
/// whether it is subscripted or `.get()`, and both spellings appear across the
/// integrations for the same job.
fn last_bracket_subscript(script: &str, name: &str) -> Option<String> {
    let start = script.rfind(name)? + name.len();
    let mut depth = 1usize;
    for (i, c) in script[start..].char_indices() {
        match c {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(script[start..start + i].to_string());
                }
            }
            _ => {}
        }
    }
    None
}

/// Strip Painless null-safe navigation from a field path.
pub(crate) fn clean_path(path: &str) -> String {
    path.trim().replace("?.", ".").replace('?', "")
}

/// A mutable reference to the value at a dotted path.
pub(crate) fn pointer_mut<'a>(event: &'a mut Event, path: &str) -> Option<&'a mut Value> {
    let mut pointer = String::with_capacity(path.len() + 1);
    for segment in path.split('.') {
        pointer.push('/');
        // JSON Pointer's own escapes, so a key holding one still resolves.
        for c in segment.chars() {
            match c {
                '~' => pointer.push_str("~0"),
                '/' => pointer.push_str("~1"),
                _ => pointer.push(c),
            }
        }
    }
    event.as_value_mut().pointer_mut(&pointer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Verbatim from `pipelines/crowdstrike/default.yml`, so a change upstream
    /// shows up here as a miss.
    const SENTINEL: &str = "ctx.crowdstrike.event.entrySet().removeIf(entry -> \
                            params.values.contains(entry.getValue()));";

    /// Verbatim from `pipelines/aws/cloudtrail/default.yml`. The table read is
    /// the LAST thing it does; three writes come first, and a matcher that
    /// took only the merge dropped all three.
    const CLOUDTRAIL_CATEGORY: &str = "ctx.event.kind = 'event';\n\
        ctx.event.type = ['info'];\n\
        if (ctx.aws?.cloudtrail?.error_code != null) {\n  \
        ctx.event.outcome = 'failure'\n} else {\n  \
        ctx.event.outcome = 'success'\n}\n\
        if (params.get(ctx.event.action) == null) {\n  return;\n}\n\
        def hm = new HashMap(params.get(ctx.event.action));\n\
        hm.forEach((k, v) -> ctx.event[k] = v);";

    #[test]
    fn a_lookup_merge_runs_what_comes_before_the_table_read() {
        let params = json!({"CreateUser": {"category": ["iam"], "type": ["creation"]}});
        let mut event = Event::new(json!({"event": {"action": "CreateUser"}}));

        assert!(try_params_painless(
            &mut event,
            CLOUDTRAIL_CATEGORY,
            &params
        ));
        assert_eq!(event.get_str("event.kind"), Some("event"));
        assert_eq!(event.get("event.type"), Some(&json!(["creation"])));
        assert_eq!(event.get_str("event.outcome"), Some("success"));
        assert_eq!(event.get("event.category"), Some(&json!(["iam"])));
    }

    /// The `else` arm is taken when the guard does not hold, and a list
    /// literal reaches the field as a list.
    #[test]
    fn an_else_arm_and_a_list_literal_both_land() {
        let params = json!({});
        let mut event = Event::new(json!({
            "event": {"action": "Unlisted"},
            "aws": {"cloudtrail": {"error_code": "AccessDenied"}},
        }));

        assert!(try_params_painless(
            &mut event,
            CLOUDTRAIL_CATEGORY,
            &params
        ));
        assert_eq!(event.get_str("event.outcome"), Some("failure"));
        assert_eq!(
            event.get("event.type"),
            Some(&json!(["info"])),
            "the table has no row, so the preamble's own list stands"
        );
    }

    /// Verbatim from `pipelines/okta/ecs_category_type.yml`. Elastic's `gen`
    /// emits this same script for every package that maps a vendor event name
    /// onto ECS categorisation.
    const ADD_UNIQUE: &str = "def addUnique(List dst, List src) {\n  src = src ?: [];\n  \
                              if (src.length == 0) {\n    return dst ?: [];\n  }\n  \
                              HashSet s = new HashSet(dst ?: []);\n  s.addAll(src);\n  \
                              return new ArrayList(s);\n}\n\
                              def p = params[ctx.okta.event_type];\n\
                              ctx.event.type = addUnique(ctx.event.type, p.type);\n\
                              ctx.event.category = addUnique(ctx.event.category, p.category);\n\
                              ctx.tags = addUnique(ctx.tags, p.tags);";

    /// Verbatim from `pipelines/microsoft_defender_endpoint/vulnerability`,
    /// tagged `script_map_host_os_type`.
    const CONTAINED: &str = "String os_platform = \
        ctx.microsoft_defender_endpoint.vulnerability.os_platform.toLowerCase();\n\
        for (String os: params.os_type) {\n  if (os_platform.contains(os)) {\n    \
        ctx.host.os.put('type', os);\n    return;\n  }\n}\n\
        if (os_platform.contains('centos') || os_platform.contains('ubuntu')) {\n  \
        ctx.host.os.put('type', 'linux');\n}\n";

    fn contained_params() -> Value {
        json!({ "os_type": ["linux", "macos", "windows"] })
    }

    /// The first member the subject contains wins, case-folded as the script
    /// folds it.
    #[test]
    fn the_first_contained_member_is_the_answer() {
        let mut event = Event::new(json!({
            "microsoft_defender_endpoint": { "vulnerability": { "os_platform": "Windows10" } },
        }));

        assert!(try_params_painless(
            &mut event,
            CONTAINED,
            &contained_params()
        ));

        assert_eq!(event.get("host.os.type"), Some(&json!("windows")));
    }

    /// A subject no member matches falls to the script's own literal tail.
    #[test]
    fn a_subject_no_member_matches_falls_to_the_tail() {
        let mut event = Event::new(json!({
            "microsoft_defender_endpoint": { "vulnerability": { "os_platform": "CentOS7" } },
        }));

        assert!(try_params_painless(
            &mut event,
            CONTAINED,
            &contained_params()
        ));

        assert_eq!(event.get("host.os.type"), Some(&json!("linux")));
    }

    /// And one neither reaches leaves the field alone.
    #[test]
    fn a_subject_nothing_matches_writes_nothing() {
        let mut event = Event::new(json!({
            "microsoft_defender_endpoint": { "vulnerability": { "os_platform": "Plan9" } },
        }));

        assert!(try_params_painless(
            &mut event,
            CONTAINED,
            &contained_params()
        ));

        assert_eq!(event.get("host.os.type"), None);
    }

    /// Verbatim from `pipelines/microsoft_dnsserver/analytical/default.yml`,
    /// cut to the branches that decide a key's fate.
    const RENAME_KEYS: &str = "def renameKeys(Map src, Map keyMap) {\n  \
        def dst = new HashMap();\n  for (def entry: src.entrySet()) {\n    \
        def key = entry.getKey();\n    def value = entry.getValue();\n    \
        if (value instanceof Map) {\n      if (keyMap.containsKey(key)) {\n        \
        dst[keyMap[key]] = renameKeys(value, keyMap);\n      } else {\n        \
        dst[key] = renameKeys(value, keyMap);\n      }\n    } else {\n      \
        if (keyMap.containsKey(key)) {\n        dst[keyMap[key]] = value;\n      } \
        else {\n        dst[key] = value;\n      }\n    }\n  }\n  return dst;\n}\n\
        ctx.microsoft_dnsserver.analytical = \
        renameKeys(ctx.microsoft_dnsserver.analytical, params)";

    fn rename_params() -> Value {
        json!({ "QNAME": "question_name", "XID": "xid", "SID": "sid" })
    }

    /// A named key is renamed at any depth; one the map does not name keeps
    /// its own, and the values are carried across untouched.
    #[test]
    fn named_keys_are_renamed_at_every_depth() {
        let mut event = Event::new(json!({
            "microsoft_dnsserver": { "analytical": {
                "QNAME": "google.es.",
                "XID": "7",
                "Untouched": "kept",
                "extended_data": { "SID": "S-1-5-21" },
                "listed": [{ "XID": "9" }, "plain"],
            } },
        }));

        assert!(try_params_painless(
            &mut event,
            RENAME_KEYS,
            &rename_params()
        ));

        let analytical = event.get("microsoft_dnsserver.analytical").unwrap();
        assert_eq!(analytical.get("question_name"), Some(&json!("google.es.")));
        assert_eq!(analytical.get("xid"), Some(&json!("7")));
        assert_eq!(analytical.get("Untouched"), Some(&json!("kept")));
        assert_eq!(analytical.get("QNAME"), None);
        assert_eq!(
            event.get("microsoft_dnsserver.analytical.extended_data.sid"),
            Some(&json!("S-1-5-21"))
        );
        assert_eq!(
            analytical.get("listed"),
            Some(&json!([{ "xid": "9" }, "plain"]))
        );
    }

    /// Verbatim from `pipelines/cisco_asa/default.yml`, tagged
    /// `script_ecs_outcome_categorization`: a message id, then its outcome.
    const TWO_LEVEL: &str =
        "params.get(ctx.event.code)?.get(ctx._temp_.outcome)?.forEach((k, v) -> ctx.event[k] = v);";

    fn two_level_params() -> Value {
        json!({
            "106100": {
                "denied": { "type": ["connection", "denied"], "outcome": "failure", "action": "firewall-rule" },
                "permitted": { "type": ["connection", "allowed"], "outcome": "success", "action": "firewall-rule" },
            },
        })
    }

    /// Only the row the outcome selects is merged. Following one level fanned
    /// the second level's keys out as fields of their own.
    #[test]
    fn a_two_level_table_merges_the_row_the_outcome_selects() {
        let mut event = Event::new(json!({
            "event": { "code": "106100" },
            "_temp_": { "outcome": "permitted" },
        }));

        assert!(try_params_painless(
            &mut event,
            TWO_LEVEL,
            &two_level_params()
        ));

        assert_eq!(event.get("event.outcome"), Some(&json!("success")));
        assert_eq!(event.get("event.action"), Some(&json!("firewall-rule")));
        assert_eq!(
            event.get("event.type"),
            Some(&json!(["connection", "allowed"]))
        );
        assert_eq!(event.get("event.denied"), None);
    }

    /// An outcome the table has no row for leaves the event alone.
    #[test]
    fn a_two_level_table_with_no_row_for_the_outcome_writes_nothing() {
        let mut event = Event::new(json!({
            "event": { "code": "106100" },
            "_temp_": { "outcome": "est-allowed" },
        }));

        assert!(try_params_painless(
            &mut event,
            TWO_LEVEL,
            &two_level_params()
        ));

        assert_eq!(event.get("event.action"), None);
    }

    /// Verbatim from `pipelines/cisco_ftd/default.yml`, tagged
    /// `script_a14307b2`, cut to its loop.
    const VALUE_MAPS: &str = "def getField(Map src, String[] path) {\n return null;\n}\n\
        def setField(Map dest, String[] path, def value) {\n return null;\n}\n\
        for (entry in params.entrySet()) {\n  def srcField = entry.getKey();\n  \
        def param = entry.getValue();\n  \
        def rawVal = getField(ctx, srcField.splitOnToken('.'));\n  \
        if (rawVal == null) continue;\n  String oldVal;\n  \
        if (rawVal instanceof AbstractList) {\n    if (rawVal.size() == 0) continue;\n    \
        oldVal = rawVal[0].toString();\n  } else {\n    oldVal = rawVal.toString();\n  }\n  \
        def newVal = param.map?.getOrDefault(oldVal.toLowerCase(), null);\n  \
        if (newVal != null) {\n    def dstField = param.getOrDefault('target', srcField);\n    \
        setField(ctx, dstField.splitOnToken('.'), newVal);\n  }\n}\n";

    fn value_map_params() -> Value {
        json!({
            "dns.question.type": { "map": { "a host address": "A", "ip6 address": "AAAA" } },
            "dns.response_code": { "map": { "no error": "NOERROR" } },
            "ctx._temp_.cisco.message_id": {
                "target": "event.action",
                "map": { "430002": "connection-started" },
            },
        })
    }

    /// The device spells the record type out; ECS wants the mnemonic. The key
    /// is folded to lower case, because the script folds it.
    #[test]
    fn each_field_is_normalised_through_its_own_map() {
        let mut event = Event::new(json!({
            "dns": { "question": { "type": "a host address" }, "response_code": "No error" },
        }));

        assert!(try_params_painless(
            &mut event,
            VALUE_MAPS,
            &value_map_params()
        ));

        assert_eq!(event.get("dns.question.type"), Some(&json!("A")));
        assert_eq!(event.get("dns.response_code"), Some(&json!("NOERROR")));
    }

    /// A params key written with a `ctx.` prefix names no field, so it matches
    /// nothing -- which is what it does upstream too, and it is the reason
    /// FTD's `message_id` never reaches `event.action` through this script.
    #[test]
    fn a_source_path_that_names_no_field_writes_nothing() {
        let mut event = Event::new(json!({ "_temp_": { "cisco": { "message_id": "430002" } } }));

        assert!(try_params_painless(
            &mut event,
            VALUE_MAPS,
            &value_map_params()
        ));

        assert_eq!(event.get("event.action"), None);
    }

    /// A value the map has no row for is left exactly as it was.
    #[test]
    fn a_value_absent_from_the_map_is_left_alone() {
        let mut event = Event::new(json!({ "dns": { "question": { "type": "mail exchange" } } }));

        assert!(try_params_painless(
            &mut event,
            VALUE_MAPS,
            &value_map_params()
        ));

        assert_eq!(
            event.get("dns.question.type"),
            Some(&json!("mail exchange"))
        );
    }

    /// Verbatim from `pipelines/cisco_ftd/default.yml`, tagged
    /// `script_categorize_event`, cut to two of its outcome branches.
    const CATEGORISE: &str = "if (ctx.event?.action == null || \
        !params.containsKey(ctx.event.action)) {\n  return;\n}\n\
        ctx.event.kind = params.get(ctx.event.action).get('kind');\n\
        ctx.event.category = params.get(ctx.event.action).get('category').clone();\n\
        ctx.event.type = params.get(ctx.event.action).get('type').clone();\n\
        if (ctx.event?.outcome == null) {\n  return;\n}\n\
        if (ctx.event.category.contains('network') || \
        ctx.event.category.contains('intrusion_detection')) {\n  \
        if (ctx.event.outcome == 'success') {\n    ctx.event.type.add('allowed');\n  }\n  \
        if (ctx.event.outcome == 'block') {\n    ctx.event.outcome = 'success';\n    \
        ctx.event.type.add('denied');\n  }\n}\n";

    fn categorise_params() -> Value {
        json!({
            "flow-expiration": {
                "kind": "event",
                "category": ["network"],
                "type": ["connection", "end"],
            },
            "intrusion-detected": {
                "kind": "alert",
                "category": ["intrusion_detection"],
                "type": ["info"],
            },
        })
    }

    /// The row's three columns land as they are when there is no outcome to
    /// refine them by.
    #[test]
    fn an_action_row_sets_the_ecs_categorisation() {
        let mut event = Event::new(json!({ "event": { "action": "flow-expiration" } }));

        assert!(try_params_painless(
            &mut event,
            CATEGORISE,
            &categorise_params()
        ));

        assert_eq!(event.get("event.kind"), Some(&json!("event")));
        assert_eq!(event.get("event.category"), Some(&json!(["network"])));
        assert_eq!(event.get("event.type"), Some(&json!(["connection", "end"])));
    }

    /// A vendor outcome is rewritten to the ECS one AND adds its own type.
    #[test]
    fn a_vendor_outcome_is_translated_and_adds_its_type() {
        let mut event = Event::new(json!({
            "event": { "action": "flow-expiration", "outcome": "block" },
        }));

        assert!(try_params_painless(
            &mut event,
            CATEGORISE,
            &categorise_params()
        ));

        assert_eq!(event.get("event.outcome"), Some(&json!("success")));
        assert_eq!(
            event.get("event.type"),
            Some(&json!(["connection", "end", "denied"]))
        );
    }

    /// An action with no row leaves the event exactly as it was.
    #[test]
    fn an_action_with_no_row_writes_nothing() {
        let mut event = Event::new(json!({ "event": { "action": "not-in-the-table" } }));

        assert!(try_params_painless(
            &mut event,
            CATEGORISE,
            &categorise_params()
        ));

        assert_eq!(event.get("event.kind"), None);
        assert_eq!(event.get("event.category"), None);
    }

    /// Verbatim from `pipelines/cisco_ftd/default.yml`, trimmed to the two
    /// helper definitions and the loop -- the 79-name list is replaced by two
    /// of its members, since the matcher reads the list rather than knowing it.
    const KEYED: &str = "boolean isEmpty(def value) {\n  return (value instanceof \
        AbstractList ? value.size() : value.length()) == 0;\n}\n\
        def appendOrCreate(Map dest, String[] path, def value) {\n return null;\n}\n\
        def msg = ctx._temp_.orig_security;\ndef counters = new HashMap();\n\
        def dest = new HashMap();\ndef dest_event = new HashMap();\n\
        def security_event_list = new ArrayList(['dst_ip', 'src_ip']);\n\
        ctx._temp_.cisco['security'] = dest;\n\
        ctx._temp_.cisco['security_event'] = dest_event;\n\
        for (entry in msg.entrySet()) {\n def param = params.get(entry.getKey());\n \
        if (param == null) {\n   continue;\n }\n \
        param.getOrDefault('id', []).forEach( id -> counters[id] = 1 + \
        counters.getOrDefault(id, 0) );\n if (!isEmpty(entry.getValue())) {\n  \
        param.getOrDefault('ecs', []).forEach( field -> appendOrCreate(ctx, \
        field.splitOnToken('.'), entry.getValue()) );\n  \
        if (security_event_list.contains(param.target)){\n    \
        dest_event[param.target] = entry.getValue();\n  }\n  else{\n    \
        dest[param.target] = entry.getValue();\n  }\n }\n}\n\
        if (ctx._temp_.cisco.message_id != \"\") return;\ndef best;\n\
        for (entry in counters.entrySet()) {\n if (best == null || \
        best.getValue() < entry.getValue()) best = entry;\n}\n\
        if (best != null) ctx._temp_.cisco.message_id = best.getKey();\n";

    fn keyed_params() -> Value {
        json!({
            "DstIP": { "target": "dst_ip", "id": ["430002"], "ecs": ["destination.address"] },
            "SrcIP": { "target": "src_ip", "id": ["430002"], "ecs": ["source.address"] },
            "AC_RuleName": { "target": "access_control_rule_name", "id": ["430002"] },
            "Protocol": { "target": "protocol", "ecs": ["network.transport"] },
        })
    }

    /// Each key lands in the map its target's membership decides, and feeds
    /// every ECS field its row names.
    #[test]
    fn a_keyed_message_fans_out_through_its_table() {
        let mut event = Event::new(json!({
            "_temp_": {
                "cisco": { "message_id": "430002" },
                "orig_security": {
                    "DstIP": "10.0.1.20",
                    "SrcIP": "10.0.100.30",
                    "AC_RuleName": "Rule-1",
                    "Protocol": "icmp",
                },
            },
        }));

        assert!(try_params_painless(&mut event, KEYED, &keyed_params()));

        assert_eq!(event.get("destination.address"), Some(&json!("10.0.1.20")));
        assert_eq!(event.get("source.address"), Some(&json!("10.0.100.30")));
        assert_eq!(event.get("network.transport"), Some(&json!("icmp")));
        assert_eq!(
            event.get("_temp_.cisco.security_event.dst_ip"),
            Some(&json!("10.0.1.20"))
        );
        // `access_control_rule_name` is not in this cut-down list, so it goes
        // to the other map -- which is the whole point of reading the list.
        assert_eq!(
            event.get("_temp_.cisco.security.access_control_rule_name"),
            Some(&json!("Rule-1"))
        );
    }

    /// With no id in the header, the id the most keys vote for becomes it.
    #[test]
    fn an_absent_message_id_is_decided_by_the_keys_present() {
        let mut event = Event::new(json!({
            "_temp_": {
                "cisco": { "message_id": "" },
                "orig_security": { "DstIP": "10.0.1.20", "Protocol": "icmp" },
            },
        }));

        assert!(try_params_painless(&mut event, KEYED, &keyed_params()));

        assert_eq!(event.get("_temp_.cisco.message_id"), Some(&json!("430002")));
    }

    /// An id the header already carried is never overwritten by the vote.
    #[test]
    fn a_message_id_already_set_survives_the_vote() {
        let mut event = Event::new(json!({
            "_temp_": {
                "cisco": { "message_id": "430003" },
                "orig_security": { "DstIP": "10.0.1.20" },
            },
        }));

        assert!(try_params_painless(&mut event, KEYED, &keyed_params()));

        assert_eq!(event.get("_temp_.cisco.message_id"), Some(&json!("430003")));
    }

    /// Verbatim from `pipelines/cisco_asa/default.yml`, where it is tagged
    /// `script_process_iana_number`. `cisco_ftd` ships the same script.
    const IANA: &str = "def net = ctx.network; def iana = params[net.transport]; \
                        if (iana != null) {\n  net['iana_number'] = iana;\n  return;\n} \
                        def reverse = new HashMap(); def[] arr = new def[] { null }; \
                        for (entry in params.entrySet()) {\n  arr[0] = entry.getValue();\n  \
                        reverse.put(String.format(\"%d\", arr), entry.getKey());\n} \
                        def trans = reverse[net.transport]; if (trans != null) {\n  \
                        net['iana_number'] = net.transport;\n  net['transport'] = trans;\n}\n";

    fn iana_params() -> Value {
        json!({ "icmp": 1, "tcp": 6, "udp": 17, "gre": 47 })
    }

    /// The ordinary direction: a transport NAME gets its protocol number.
    #[test]
    fn a_transport_name_gets_its_iana_number() {
        let mut event = Event::new(json!({ "network": { "transport": "tcp" } }));

        assert!(try_params_painless(&mut event, IANA, &iana_params()));

        assert_eq!(event.get("network.iana_number"), Some(&json!(6)));
        assert_eq!(event.get("network.transport"), Some(&json!("tcp")));
    }

    /// The device wrote the NUMBER into `transport`. Elastic moves it across
    /// and puts the name back, rather than leaving a number in a name field.
    #[test]
    fn a_transport_number_is_moved_and_the_name_restored() {
        let mut event = Event::new(json!({ "network": { "transport": "17" } }));

        assert!(try_params_painless(&mut event, IANA, &iana_params()));

        assert_eq!(event.get("network.iana_number"), Some(&json!("17")));
        assert_eq!(event.get("network.transport"), Some(&json!("udp")));
    }

    /// A transport the table does not carry is left exactly as it was --
    /// guessing a number for it would be worse than having none.
    #[test]
    fn an_unknown_transport_is_left_alone() {
        let mut event = Event::new(json!({ "network": { "transport": "sctp" } }));

        assert!(try_params_painless(&mut event, IANA, &iana_params()));

        assert_eq!(event.get("network.iana_number"), None);
        assert_eq!(event.get("network.transport"), Some(&json!("sctp")));
    }

    fn add_unique_params() -> Value {
        json!({
            "user.session.start": {
                "category": ["authentication"],
                "type": ["start"],
                "tags": ["identity"],
            },
        })
    }

    /// The row's columns must be UNIONED into what the pipeline already put
    /// there -- the append processors ahead of this one have usually written
    /// `event.type` already, and replacing it drops their work.
    #[test]
    fn add_unique_unions_the_row_into_the_existing_arrays() {
        let mut event = Event::new(json!({
            "okta": { "event_type": "user.session.start" },
            "event": { "type": ["info"], "category": ["session"] },
            "tags": ["forwarded"],
        }));

        assert!(try_params_painless(
            &mut event,
            ADD_UNIQUE,
            &add_unique_params()
        ));

        assert_eq!(event.get("event.type"), Some(&json!(["info", "start"])));
        assert_eq!(
            event.get("event.category"),
            Some(&json!(["session", "authentication"]))
        );
        assert_eq!(event.get("tags"), Some(&json!(["forwarded", "identity"])));
    }

    /// A member the row repeats must not appear twice -- the script is a set
    /// union, not an append.
    #[test]
    fn add_unique_does_not_duplicate_what_is_already_there() {
        let mut event = Event::new(json!({
            "okta": { "event_type": "user.session.start" },
            "event": { "type": ["start"] },
        }));

        assert!(try_params_painless(
            &mut event,
            ADD_UNIQUE,
            &add_unique_params()
        ));
        assert_eq!(event.get("event.type"), Some(&json!(["start"])));
    }

    /// An arrays-absent event still gets the row, since `addUnique` treats a
    /// null destination as empty.
    #[test]
    fn add_unique_creates_the_arrays_it_finds_missing() {
        let mut event = Event::new(json!({ "okta": { "event_type": "user.session.start" } }));

        assert!(try_params_painless(
            &mut event,
            ADD_UNIQUE,
            &add_unique_params()
        ));
        assert_eq!(
            event.get("event.category"),
            Some(&json!(["authentication"]))
        );
    }

    /// Verbatim from `pipelines/crowdstrike/default.yml`, trimmed to the loops
    /// that decide the answer.
    const FRAMEWORK: &str = "def tid = ctx.threat.tactic?.id;\n\
        def nid = ctx.threat.technique?.id;\n\
        def tname = ctx.threat.tactic?.name;\n\
        Set frameworks = new HashSet();\n\
        if (tid != null && !tid.isEmpty()) {\n  for (String t: tid) {\n    \
        if (t.startsWith(\"CS\")) {\n      frameworks.add(params.framework_cs);\n    }\n    \
        else if (t.startsWith(\"TA\")) {\n      frameworks.add(params.framework_ma);\n    }\n  }\n}\n\
        if (nid != null && !nid.isEmpty()) {\n  for (String t: nid) {\n    \
        if (t.startsWith(\"CS\")) {\n      frameworks.add(params.framework_cs);\n    }\n  }\n}\n\
        if (tname != null && !tname.isEmpty()) {\n  for (String t: tname) {\n    \
        if (params.falcon_tactic_names.contains(t.toLowerCase())) {\n      \
        frameworks.add(params.framework_cs);\n    }\n  }\n}\n\
        for (def preferred : params.framework_preference) {\n  \
        if (frameworks.contains(preferred)) {\n    ctx.threat.framework = preferred;\n    \
        return;\n  }\n}";

    fn framework_params() -> Value {
        json!({
            "framework_preference": ["MITRE ATT&CK", "CrowdStrike Falcon Detections Framework"],
            "framework_cs": "CrowdStrike Falcon Detections Framework",
            "framework_ma": "MITRE ATT&CK",
            "falcon_tactic_names": ["malware", "exploit", "falcon overwatch"],
        })
    }

    #[test]
    fn framework_reads_mitre_off_a_ta_prefixed_tactic() {
        let mut event = Event::new(json!({
            "threat": { "tactic": { "id": ["TA0002"] } },
        }));

        assert!(try_params_painless(
            &mut event,
            FRAMEWORK,
            &framework_params()
        ));
        assert_eq!(event.get_str("threat.framework"), Some("MITRE ATT&CK"));
    }

    /// A tactic NAME in the params list is Falcon's own framework, and the
    /// comparison is case-insensitive because the script lower-cases first.
    #[test]
    fn framework_reads_falcon_off_a_named_tactic() {
        let mut event = Event::new(json!({
            "threat": { "tactic": { "name": ["Falcon OverWatch"] } },
        }));

        assert!(try_params_painless(
            &mut event,
            FRAMEWORK,
            &framework_params()
        ));
        assert_eq!(
            event.get_str("threat.framework"),
            Some("CrowdStrike Falcon Detections Framework")
        );
    }

    /// Both frameworks present is what the preference list exists for, and
    /// MITRE is first in it.
    #[test]
    fn framework_follows_the_declared_preference_when_both_match() {
        let mut event = Event::new(json!({
            "threat": {
                "tactic": { "id": ["CS0001", "TA0002"] },
            },
        }));

        assert!(try_params_painless(
            &mut event,
            FRAMEWORK,
            &framework_params()
        ));
        assert_eq!(event.get_str("threat.framework"), Some("MITRE ATT&CK"));
    }

    /// Nothing matching writes nothing -- the script returns early rather than
    /// picking the first preference.
    #[test]
    fn framework_writes_nothing_when_no_rule_matches() {
        let mut event = Event::new(json!({
            "threat": { "tactic": { "id": ["XX0001"] } },
        }));

        assert!(try_params_painless(
            &mut event,
            FRAMEWORK,
            &framework_params()
        ));
        assert!(!event.has("threat.framework"));
    }

    /// An event type the table does not carry throws in Painless and the
    /// sub-pipeline's `on_failure` catches it. Nothing here models the throw,
    /// so the event must at least come through untouched rather than mangled.
    #[test]
    fn add_unique_leaves_an_unknown_event_type_alone() {
        let mut event = Event::new(json!({
            "okta": { "event_type": "user.session.nosuchthing" },
            "event": { "type": ["info"] },
        }));

        assert!(try_params_painless(
            &mut event,
            ADD_UNIQUE,
            &add_unique_params()
        ));
        assert_eq!(event.get("event.type"), Some(&json!(["info"])));
    }

    fn sentinel_params() -> Value {
        json!({ "values": [null, "", "-", "N/A", "NA", 0] })
    }

    #[test]
    fn strips_every_sentinel_the_params_name() {
        let mut event = Event::new(json!({
            "crowdstrike": { "event": {
                "keep": "value", "empty": "", "dash": "-", "na": "NA", "zero": 0, "nul": null,
            }},
        }));

        assert!(try_params_painless(
            &mut event,
            SENTINEL,
            &sentinel_params()
        ));

        let obj = event.get_object("crowdstrike.event").unwrap();
        assert_eq!(obj.len(), 1, "only `keep` survives: {obj:?}");
        assert_eq!(obj.get("keep"), Some(&json!("value")));
    }

    /// The metadata variant omits `0`, and a numeric zero must then survive.
    #[test]
    fn a_sentinel_absent_from_params_is_kept() {
        let script = SENTINEL.replace("crowdstrike.event", "crowdstrike.metadata");
        let mut event = Event::new(json!({
            "crowdstrike": { "metadata": { "zero": 0, "dash": "-" } },
        }));

        assert!(try_params_painless(
            &mut event,
            &script,
            &json!({ "values": [null, "", "-", "N/A", "NA"] }),
        ));

        let obj = event.get_object("crowdstrike.metadata").unwrap();
        assert_eq!(obj.get("zero"), Some(&json!(0)));
        assert!(!obj.contains_key("dash"));
    }

    #[test]
    fn a_missing_map_is_not_a_failure() {
        let mut event = Event::new(json!({ "other": 1 }));
        assert!(try_params_painless(
            &mut event,
            SENTINEL,
            &sentinel_params()
        ));
    }

    const FILETIME: &str = "def convertToUnix(def longValue) {\n\
                            if (longValue > 0x0100000000000000L) {\n\
                            return (longValue / 10000) - 11644473600000L;\n}\nreturn longValue;\n}\n\
                            for (def field : params.values) {\n\
                            def fieldValue = ctx.crowdstrike.event[field];\n\
                            ctx.crowdstrike.event[field] = convertToUnix(fieldValue);\n}";

    #[test]
    fn converts_every_filetime_field_the_params_name() {
        let mut event = Event::new(json!({
            "crowdstrike": { "event": {
                // 2020-01-01T00:00:00Z as a FILETIME, as a number and as a string.
                "StartTime": 132_223_104_000_000_000_i64,
                "EndTime": "132223104000000000",
                // Already UNIX seconds -- below the threshold, so untouched.
                "ContextTimeStamp": 1_577_836_800_i64,
                "Untouched": 132_223_104_000_000_000_i64,
            }},
        }));

        assert!(try_params_painless(
            &mut event,
            FILETIME,
            &json!({ "values": ["StartTime", "EndTime", "ContextTimeStamp"] }),
        ));

        assert_eq!(
            event.get_i64("crowdstrike.event.StartTime"),
            Some(1_577_836_800_000)
        );
        assert_eq!(
            event.get_i64("crowdstrike.event.EndTime"),
            Some(1_577_836_800_000)
        );
        assert_eq!(
            event.get_i64("crowdstrike.event.ContextTimeStamp"),
            Some(1_577_836_800)
        );
        assert_eq!(
            event.get_i64("crowdstrike.event.Untouched"),
            Some(132_223_104_000_000_000)
        );
    }

    /// Verbatim from `pipelines/azure/activitylogs/default.yml`.
    const LOOKUP: &str = "if (ctx?.azure?.activitylogs?.category == null) { return; } \
                          def category = ctx.azure.activitylogs.category.toLowerCase(); \
                          if (params.get(category) == null) { return; } \
                          def hm = new HashMap(params.get(category)); \
                          hm.forEach((k, v) -> ctx.event[k] = v);";

    fn lookup_params() -> Value {
        json!({
            "write": { "type": ["change"] },
            "read": { "type": ["access"] },
            "delete": { "type": ["deletion"] },
        })
    }

    #[test]
    fn merges_the_row_the_keyed_field_selects() {
        let mut event = Event::new(json!({
            "azure": { "activitylogs": { "category": "Write" } },
            "event": {},
        }));

        assert!(try_params_painless(&mut event, LOOKUP, &lookup_params()));
        assert_eq!(event.get("event.type"), Some(&json!(["change"])));
    }

    #[test]
    fn a_key_absent_from_the_table_changes_nothing() {
        let mut event = Event::new(json!({
            "azure": { "activitylogs": { "category": "Unmapped" } },
            "event": {},
        }));

        assert!(try_params_painless(&mut event, LOOKUP, &lookup_params()));
        assert_eq!(event.get_object("event").unwrap().len(), 0);
    }

    /// panw's decryption-log flags, as the generator emits them.
    const BIT_FLAGS: &str = r"def labels = ctx.labels; if (labels == null) {
  labels = new HashMap();
  ctx['labels'] = labels;
} long value = ctx._temp_.labels; for (entry in params.entrySet()) {
  def flag = entry.getValue();
  if (flag instanceof String) {
      flag = Long.decode(flag);
  }
  if ((value & flag) != 0) {
      labels[entry.getKey()] = true;
  }
}
";

    fn flag_params() -> Value {
        json!({
            "nat_translated": 0x0040_0000,
            "captive_portal": 0x0020_0000,
            "ssl_decrypted": 0x0100_0000,
        })
    }

    #[test]
    fn sets_a_label_for_every_bit_the_field_has_set() {
        let mut event = Event::new(json!({
            "_temp_": { "labels": 0x0060_0000 },
        }));

        assert!(try_params_painless(&mut event, BIT_FLAGS, &flag_params()));
        assert_eq!(event.get("labels.nat_translated"), Some(&json!(true)));
        assert_eq!(event.get("labels.captive_portal"), Some(&json!(true)));
    }

    /// The script only ever writes `true`, so a clear bit must leave the label
    /// ABSENT -- writing `false` would be an extra field Elastic never emits.
    #[test]
    fn a_clear_bit_leaves_its_label_absent() {
        let mut event = Event::new(json!({
            "_temp_": { "labels": 0x0040_0000 },
        }));

        assert!(try_params_painless(&mut event, BIT_FLAGS, &flag_params()));
        assert_eq!(event.get("labels.nat_translated"), Some(&json!(true)));
        assert_eq!(event.get("labels.captive_portal"), None);
        assert_eq!(event.get("labels.ssl_decrypted"), None);
    }

    /// `Long.decode` is in the vendor script because the flags have been known
    /// to arrive stringified, and a hex string must decode as hex.
    #[test]
    fn a_stringified_hex_flag_decodes() {
        let mut event = Event::new(json!({ "_temp_": { "labels": 0x0040_0000 } }));
        let params = json!({ "nat_translated": "0x00400000" });

        assert!(try_params_painless(&mut event, BIT_FLAGS, &params));
        assert_eq!(event.get("labels.nat_translated"), Some(&json!(true)));
    }

    #[test]
    fn a_bitfield_carried_as_a_string_still_decodes() {
        let mut event = Event::new(json!({ "_temp_": { "labels": "4194304" } }));

        assert!(try_params_painless(&mut event, BIT_FLAGS, &flag_params()));
        assert_eq!(event.get("labels.nat_translated"), Some(&json!(true)));
    }

    #[test]
    fn an_absent_bitfield_writes_no_labels() {
        let mut event = Event::new(json!({ "event": {} }));

        assert!(!try_params_painless(&mut event, BIT_FLAGS, &flag_params()));
        assert_eq!(event.get("labels"), None);
    }

    /// A level whose key names no field stops the descent, and nothing of the
    /// row above it is merged -- the second level's keys are not fields.
    #[test]
    fn a_two_level_table_stops_where_the_key_is_absent() {
        let script = "params.get(ctx.event.code).get(ctx._temp_.outcome)\
                      .forEach((k, v) -> ctx.event[k] = v);";
        let mut event = Event::new(json!({ "event": { "code": "750002" } }));

        assert!(try_params_painless(
            &mut event,
            script,
            &json!({ "750002": { "success": { "action": "started" } } }),
        ));
        assert_eq!(event.get("event.action"), None);
        assert_eq!(event.get("event.success"), None);
    }

    /// Verbatim from `pipelines/cisco/nexus/default.yml`.
    const INDEXED: &str = "def LogLevelValue = (int) ctx.event.severity;\n\
                           if (LogLevelValue >= 0 && LogLevelValue < params.LogLevel.length) {\n  \
                           ctx.log.put('level', params['LogLevel'][LogLevelValue]);\n}";

    #[test]
    fn indexes_the_params_array_by_the_numeric_field() {
        let mut event = Event::new(json!({ "event": { "severity": 3 }, "log": {} }));

        assert!(try_params_painless(
            &mut event,
            INDEXED,
            &json!({ "LogLevel": ["emergency", "alert", "critical", "error", "warning"] }),
        ));
        assert_eq!(event.get_str("log.level"), Some("error"));
    }

    /// The vendor's bounds check is the array's own length.
    #[test]
    fn an_index_past_the_end_sets_nothing() {
        let mut event = Event::new(json!({ "event": { "severity": 9 }, "log": {} }));

        assert!(try_params_painless(
            &mut event,
            INDEXED,
            &json!({ "LogLevel": ["emergency", "alert"] }),
        ));
        assert!(!event.has("log.level"));
    }

    /// Verbatim from `pipelines/azure/auditlogs/default.yml`.
    const SCALE: &str = "ctx.event.duration = ctx.event.duration * params.param_nano";

    #[test]
    fn scales_a_duration_by_the_params_constant() {
        let mut event = Event::new(json!({ "event": { "duration": 42 } }));

        assert!(try_params_painless(
            &mut event,
            SCALE,
            &json!({ "param_nano": 1_000_000_000_i64 }),
        ));
        assert_eq!(event.get_i64("event.duration"), Some(42_000_000_000));
    }

    /// Verbatim from `pipelines/azure/activitylogs/default.yml`.
    const REPLACE: &str = "ctx.message = ctx.message.replace(params.empty_field_name, '')";

    #[test]
    fn strips_the_marker_the_params_name() {
        let mut event = Event::new(json!({ "message": "a<EMPTY>b<EMPTY>" }));

        assert!(try_params_painless(
            &mut event,
            REPLACE,
            &json!({ "empty_field_name": "<EMPTY>" }),
        ));
        assert_eq!(event.get_str("message"), Some("ab"));
    }

    #[test]
    fn a_script_without_params_falls_through() {
        let mut event = Event::new(json!({}));
        assert!(!try_params_painless(&mut event, SENTINEL, &Value::Null));
    }
}
