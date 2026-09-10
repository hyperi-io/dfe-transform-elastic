// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The free functions the transform modules call.
//!
//! One flat namespace, re-exported through [`crate::prelude`], so a transform
//! reads as a sequence of processor calls. They wrap the enrichment modules,
//! or stub what needs configuration this build may not have (`GeoIP`
//! databases, a Painless interpreter).

use std::collections::HashMap;
use std::sync::LazyLock;

use serde_json::{Map, Value};

use crate::enrichment::user_agent;
use crate::error::Result;
use crate::event::Event;

/// Result from a registered domain lookup.
pub struct RegisteredDomainResult {
    /// Absent when the name is ITSELF a public suffix -- `com` has a top-level
    /// domain and nothing registered under it.
    pub registered_domain: Option<String>,
    pub top_level_domain: String,
    pub subdomain: Option<String>,
}

/// Look up `GeoIP` data for an IP address.
///
/// Returns a flat map of field names to values (e.g., "`country_iso_code`" -> "AU").
/// Uses the global auto-initialised enricher (auto-detects MMDB files).
pub fn geoip_lookup(db_name: &str, ip: &str) -> Result<HashMap<String, Value>> {
    Ok(crate::enrichment::geoip_global::geoip_lookup(db_name, ip))
}

/// Parse a User-Agent string into structured components.
pub fn parse_user_agent(ua: &str) -> Result<user_agent::UserAgentResult> {
    Ok(user_agent::parse(ua))
}

/// Compute a Community ID v1 hash for network flow identification.
pub fn community_id_v1(
    src_ip: &str,
    dst_ip: &str,
    src_port: u16,
    dst_port: u16,
    protocol: &str,
) -> std::result::Result<String, String> {
    crate::enrichment::community_id::community_id_v1(
        src_ip, dst_ip, src_port, dst_port, protocol, 0,
    )
}

/// Split a fully qualified domain name into its registered parts.
///
/// An ICANN suffix is honoured whole, which is the only way `211.52.31.172
/// .in-addr.arpa` registers `172.in-addr.arpa` rather than the last two labels.
///
/// A PRIVATE entry of THREE labels or more is the registered domain itself, so
/// `ec2-instance-connect.us-east-1.amazonaws.com` registers
/// `us-east-1.amazonaws.com` under `amazonaws.com`. A two-label private entry
/// lands on the same split either way, so it is left to the ICANN path.
///
/// An unlisted top-level domain gets NOTHING. Elasticsearch's processor is a
/// list lookup and a name off the list has no answer, so `domain.tld` has no
/// registered domain -- which is what the list is a dependency for.
///
/// The list is lowercase and the lookup is case-INSENSITIVE, so a DNS query
/// name arrives as `B.ROOT-SERVERS.NET` and registers `root-servers.net`.
/// Matching the bytes as they came found no suffix at all and wrote nothing.
///
/// The SUBDOMAIN is cut from the name as it arrived, though, so an uppercase
/// name gets none: the registered domain came off the lowercase list and does
/// not end the original text. `B.ROOT-SERVERS.NET` registers with no
/// subdomain, which is what Elasticsearch writes.
pub fn registered_domain_lookup(domain: &str) -> Option<RegisteredDomainResult> {
    let original = domain;
    let lowered;
    let domain = if domain.bytes().any(|b| b.is_ascii_uppercase()) {
        lowered = domain.to_ascii_lowercase();
        lowered.as_str()
    } else {
        domain
    };

    let parts: Vec<&str> = domain.rsplitn(3, '.').collect();
    if parts.len() < 2 {
        return None;
    }
    if !psl::suffix(parts[0].as_bytes()).is_some_and(|suffix| suffix.is_known()) {
        return None;
    }

    let listed = psl::suffix(domain.as_bytes())
        .and_then(|suffix| Some((std::str::from_utf8(suffix.as_bytes()).ok()?, suffix.typ()?)))
        .filter(|(suffix, _)| suffix.len() < domain.len());

    if let Some((private, _)) =
        listed.filter(|(suffix, typ)| *typ == psl::Type::Private && suffix.split('.').count() > 2)
        && let Some((_, tld)) = private.split_once('.')
    {
        return Some(RegisteredDomainResult {
            registered_domain: Some(private.to_string()),
            top_level_domain: tld.to_string(),
            subdomain: subdomain_of(original, private),
        });
    }

    let icann = listed
        .filter(|(_, typ)| *typ == psl::Type::Icann)
        .map(|(suffix, _)| suffix);

    // Everything the suffix does not cover, split at its last label.
    let suffix = icann.unwrap_or(parts[0]);
    let head = &domain[..domain.len() - suffix.len() - 1];
    let label = head.rsplit_once('.').map_or(head, |(_, label)| label);
    if label.is_empty() {
        return None;
    }

    let registered = format!("{label}.{suffix}");
    Some(RegisteredDomainResult {
        subdomain: subdomain_of(original, &registered),
        registered_domain: Some(registered),
        top_level_domain: suffix.to_string(),
    })
}

/// What precedes the registered domain in the name AS IT ARRIVED.
///
/// Elasticsearch cuts the subdomain out of the original text, and the
/// registered domain came off a lowercase list, so a name that does not end
/// with it -- an uppercase DNS query, say -- gets no subdomain at all.
fn subdomain_of(domain: &str, registered: &str) -> Option<String> {
    domain
        .strip_suffix(registered)
        .and_then(|rest| rest.strip_suffix('.'))
        .filter(|rest| !rest.is_empty())
        .map(str::to_string)
}

// The unplanned script path reads only the Painless matchers and their
// counters, so it lives beside the planned one in `dfe-painless`. Re-exported
// here because the generated call sites and the prelude name it through this
// module.
pub use dfe_painless::plan::{painless_exec, painless_exec_params};

/// Run one processor over every element of an array field, the way Elastic's
/// `foreach` does: each element is exposed at `_ingest._value` for the body,
/// then collected back into the field.
///
/// The array is TAKEN, not cloned -- every element moves through
/// `_ingest._value` and back without an allocation, where the old inline
/// loop copied the whole array up front. The body only ever touches
/// `_ingest._value`; a generated body that names the field itself stays on
/// the cloning inline form, which the generator decides.
///
/// A body that fails part-way leaves what Elastic's shared-reference
/// iteration leaves: the processed prefix with its mutations, the failing
/// element -- still exposed at `_ingest._value`, as the inline loop left it
/// -- and the untouched rest, all back in the field.
///
/// # Errors
///
/// Whatever the body returned, after the field is restored.
pub fn foreach_array<F>(event: &mut Event, field: &str, mut body: F) -> Result<()>
where
    F: FnMut(&mut Event) -> Result<()>,
{
    let Some(items) = event.take_array(field) else {
        // A MAP is a foreach too: Elastic walks its entries, exposing the key
        // at `_ingest._key` and the value at `_ingest._value`, and rebuilds it
        // from whatever the body left in each. mimecast's attachment hashes
        // are a map, and skipping it lost every one of them.
        if event.get(field).is_some_and(Value::is_object) {
            return foreach_map(event, field, body);
        }
        return Ok(());
    };

    // A NESTED foreach borrows the same `_ingest._value` slot, so the
    // enclosing element is saved and put back afterwards -- Elasticsearch's
    // own processor restores the previous scope the same way. Without this
    // the inner loop replaced the outer's element and then wrote back INTO
    // the replacement.
    let enclosing = event.get("_ingest._value").cloned();

    // Probe the `_ingest._value` slot before consuming anything: the one way
    // the per-element set can fail is `_ingest` sitting there as a scalar,
    // and failing NOW lets the array go back untouched.
    if let Err(error) = event.set("_ingest._value", Value::Null) {
        event.set(field, Value::Array(items))?;
        return Err(error);
    }

    let mut out: Vec<Value> = Vec::with_capacity(items.len());
    let mut rest = items.into_iter();
    while let Some(item) = rest.next() {
        // Cannot fail: the probe above proved the path writable.
        event.set("_ingest._value", item)?;
        if let Err(error) = body(event) {
            if let Some(failed) = event.get("_ingest._value") {
                out.push(failed.clone());
            }
            out.extend(rest);
            event.set(field, Value::Array(out))?;
            return Err(error);
        }
        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
    }

    match enclosing {
        // Restore BEFORE the write-back: a nested loop's field lives inside
        // the restored element.
        Some(previous) => {
            event.set("_ingest._value", previous)?;
        }
        None => {
            event.remove("_ingest");
        }
    }
    event.set(field, Value::Array(out))?;
    Ok(())
}

/// [`foreach_array`] over a MAP: each entry's key and value are exposed, and
/// the map is rebuilt from what the body leaves in them.
///
/// A body that renames `_ingest._key` moves the entry, which is what
/// Elasticsearch's own processor does with the pair it puts back.
fn foreach_map<F>(event: &mut Event, field: &str, mut body: F) -> Result<()>
where
    F: FnMut(&mut Event) -> Result<()>,
{
    let Some(Value::Object(entries)) = event.get(field).cloned() else {
        return Ok(());
    };
    let enclosing_key = event.get("_ingest._key").cloned();
    let enclosing_value = event.get("_ingest._value").cloned();

    let mut out = serde_json::Map::with_capacity(entries.len());
    for (key, value) in entries {
        event.set("_ingest._key", Value::String(key))?;
        event.set("_ingest._value", value)?;
        body(event)?;
        let Some(Value::String(key)) = event.remove("_ingest._key") else {
            continue;
        };
        out.insert(key, event.remove("_ingest._value").unwrap_or(Value::Null));
    }

    for (path, previous) in [
        ("_ingest._key", enclosing_key),
        ("_ingest._value", enclosing_value),
    ] {
        match previous {
            Some(value) => event.set(path, value)?,
            None => {
                event.remove(path);
            }
        }
    }
    if event
        .get_object("_ingest")
        .is_some_and(serde_json::Map::is_empty)
    {
        event.remove("_ingest");
    }
    event.set(field, Value::Object(out))?;
    Ok(())
}

/// What a `json` processor gets when it reads `field`, resolved the way
/// Elasticsearch resolves it.
///
/// Elasticsearch reads the field as an `Object` and coerces it with
/// `toString()` before handing the text to Jackson, so what is STORED there
/// decides the outcome, and only a string is the ordinary case:
///
/// - a string is the text itself, parsed;
/// - a number, a boolean and null each render as exactly their own JSON
///   spelling, so the parse hands the same value straight back. Returned here
///   without the round trip, which is the identical value and no allocation;
/// - an OBJECT or an ARRAY renders through Java's `Map.toString` /
///   `List.toString` -- `{k=v, k2=v2}`, unquoted -- which is not JSON, so
///   Jackson throws and the processor FAILS. We decline it rather than
///   reproduce that text: the failure is the part parity rests on, and the
///   wording is the runtime's own.
///
/// Declining is the whole point of this function. Reading the field with
/// `get_string` answered `None` for a container and took the absent branch, so
/// every generated `json` call site returned `Ok(())` on an object with no
/// parse, no error, no counter and no `on_failure` -- silent data loss with
/// nothing left behind to find it by. `atlassian_cloud` is the worked case: its
/// `message` is an object, the pipeline renames it onto `event.original`, and
/// Elasticsearch fails the `json` processor and emits `error.message` where we
/// ran clean through.
///
/// `Ok(None)` is an ABSENT field. Elasticsearch throws there too, and we
/// deliberately do not -- treating "nothing to read" as a failure would fire
/// `on_failure` under every call site in the tree at once.
fn json_processor_value(event: &Event, field: &str) -> std::result::Result<Option<Value>, String> {
    match event.get(field) {
        None => Ok(None),
        Some(Value::String(text)) => parse_json_str(text).map(Some),
        Some(scalar @ (Value::Number(_) | Value::Bool(_) | Value::Null)) => Ok(Some(scalar.clone())),
        Some(Value::Object(_)) => {
            Err("failed to parse JSON: the field holds an object, not JSON text".to_string())
        }
        Some(Value::Array(_)) => {
            Err("failed to parse JSON: the field holds an array, not JSON text".to_string())
        }
    }
}

/// Parse one field's JSON string into `target`, the way Elastic's `json`
/// processor does.
///
/// An absent field is a no-op, exactly as the old inline `if let` was; a
/// present one that will not parse is the processor's failure, worded the
/// same way the inline `serde_json` block worded it so `on_failure` output
/// does not shift. [`json_processor_value`] carries what "will not parse"
/// covers, containers included.
///
/// # Errors
///
/// Returns [`crate::TransformError::ParseError`] naming the FIELD when the
/// text is not JSON or the field holds a container, or whatever `set` returns
/// for an unwritable target.
pub fn parse_json_field(event: &mut Event, field: &str, target: &str) -> Result<()> {
    let fail = |message: String| crate::TransformError::ParseError {
        path: field.into(),
        message,
    };
    let Some(parsed) = json_processor_value(event, field).map_err(fail)? else {
        return Ok(());
    };
    event.set(target, parsed)?;
    Ok(())
}

/// Parse a JSON field and add its members to the document ROOT.
///
/// Elastic's `add_to_root`. The parsed value must be an object -- anything
/// else throws, which is what hands the document to `on_failure`. `merge`
/// recursively merges an incoming object into an existing one of the same
/// name and throws where either side is not an object; `replace`, the
/// default, overwrites whatever was there.
///
/// Kibana's ECS log line is the whole reason: the message IS the document,
/// and without this its four packages parse nothing at all.
///
/// # Errors
///
/// Set a value at a path whose NAME is a mustache template.
///
/// A `set` inside a `foreach` names its target through `_ingest._value`, so the
/// path is only known per iteration. Writing the template literally produced a
/// field actually called `{{{_ingest._value.Name}}}`, and cyberarkpas routes
/// its whole `event.category`/`user.name` fan-out through one of these.
///
/// # Errors
///
/// Propagates whatever [`Event::set`] returns for the rendered path.
pub fn set_templated(event: &mut Event, template: &str, value: Value) -> Result<()> {
    let path = render_path(event, template);
    // An unresolved template names no field, and Elasticsearch's own `set`
    // skips rather than creating one under the empty name.
    if path.is_empty() {
        return Ok(());
    }
    event.set(&path, value)
}

/// Remove the field at a path whose NAME is a mustache template.
///
/// The mirror of [`set_templated`], and needed for the same reason: a `remove`
/// inside a `foreach` names its target through `_ingest._value`. sonicwall
/// defers every source key it has mapped into `_temp_.removes` and drops them
/// in one pass, and passing the template through literally removed nothing --
/// leaving the whole `sonicwall.firewall.*` block behind as extra fields.
pub fn remove_templated(event: &mut Event, template: &str) {
    let path = render_path(event, template);
    // An unresolved template names no field, so there is nothing to remove.
    if path.is_empty() {
        return;
    }
    event.remove(&path);
}

/// Substitute every `{{expr}}` / `{{{expr}}}` with the event's value for it.
fn render_path(event: &Event, template: &str) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some((head, tail)) = rest.split_once("{{") {
        out.push_str(head);
        let tail = tail.trim_start_matches('{');
        let Some((expr, after)) = tail.split_once("}}") else {
            out.push_str(tail);
            return out;
        };
        out.push_str(
            &event
                .get(expr.trim())
                .map_or_else(String::new, crate::painless_helpers::template_to_string),
        );
        rest = after.trim_start_matches('}');
    }
    out.push_str(rest);
    out
}

/// Returns a `ParseError` naming the field when the text is not JSON, when the
/// field holds a container rather than text (see [`json_processor_value`]), or
/// when what parses is not an object.
pub fn parse_json_field_to_root(event: &mut Event, field: &str, merge: bool) -> Result<()> {
    let fail = |message: String| crate::TransformError::ParseError {
        path: field.into(),
        message,
    };
    let Some(parsed) = json_processor_value(event, field).map_err(fail)? else {
        return Ok(());
    };
    let Value::Object(members) = parsed else {
        return Err(fail(
            "cannot add non-object root to the document".to_string(),
        ));
    };

    let Some(root) = event.as_value_mut().as_object_mut() else {
        return Err(fail("the document is not an object".to_string()));
    };
    for (key, value) in members {
        match (merge.then(|| root.get_mut(&key)).flatten(), value) {
            (Some(Value::Object(held)), Value::Object(incoming)) => {
                merge_object(held, incoming);
            }
            // Anything that is not two objects: the incoming value WINS, which
            // is what `add_to_root_conflict_strategy: merge` does -- it recurses
            // only where both sides are maps and returns the incoming otherwise.
            // This used to raise instead, and raising abandoned the loop, so
            // every key after the conflicting one was never inserted at all.
            // ti_ticura carries `tags` before `ticura` in the feed line, so one
            // array meeting another array cost it the whole `ticura` namespace,
            // the `drop` on a missing uuid then fired, and 7 of 13 events
            // vanished. Its own pipeline comment states the behaviour: the feed
            // line's `tags` "replaces ctx.tags and drops the
            // preserve_original_event tag", which is why the next processor
            // appends that tag back.
            (_, incoming) => {
                root.insert(key, incoming);
            }
        }
    }
    Ok(())
}

/// Merge `incoming` into `held`, recursing where both sides hold an object.
fn merge_object(held: &mut Map<String, Value>, incoming: Map<String, Value>) {
    for (key, value) in incoming {
        match (held.get_mut(&key), value) {
            (Some(Value::Object(nested)), Value::Object(value)) => merge_object(nested, value),
            (_, value) => {
                held.insert(key, value);
            }
        }
    }
}

/// Parse a JSON string into the document's own value type.
///
/// `serde_json`, not simd-json. simd-json is faster at building ITS tape;
/// getting a `serde_json::Value` out of it goes tape -> serde deserializer ->
/// `Value`, which is strictly more work than `serde_json` parsing straight
/// into the same tree, and the map here is an `IndexMap` because
/// `preserve_order` is not optional for parity. Measured in
/// `benches/json_parse.rs`: 6.7 vs 8.9 microseconds on an okta document and
/// 115 vs 271 nanoseconds on a small one, with half the allocations.
///
/// # Errors
///
/// Returns the parser's message, prefixed the way the generated modules
/// always worded it.
pub fn parse_json_str(text: &str) -> std::result::Result<Value, String> {
    serde_json::from_str::<Value>(text).map_err(|e| format!("failed to parse JSON: {e}"))
}

/// Two `ctx` paths compared inside an ingest `if` condition.
///
/// A conditional gets a read-only VIEW of the document, and every read of a
/// nested map or list builds a fresh wrapper that does not implement `equals`,
/// so two containers never compare equal there however identical their
/// contents. Absent and explicitly null are the same value to Painless, and
/// two of those DO compare equal.
///
/// gcp's firewall pipeline rests on it: it asks whether the source and
/// destination instances are the same to call the traffic internal, and by
/// then `vm_name` has been renamed onto `source.domain`, leaving two different
/// VMs holding identical `{project_id, region, zone}` maps.
#[must_use]
pub fn condition_eq(left: Option<&Value>, right: Option<&Value>) -> bool {
    fn present(v: Option<&Value>) -> Option<&Value> {
        v.filter(|v| !v.is_null())
    }
    match (present(left), present(right)) {
        (None, None) => true,
        (Some(a), Some(b))
            if !a.is_object() && !a.is_array() && !b.is_object() && !b.is_array() =>
        {
            a == b
        }
        _ => false,
    }
}

/// Turn keys whose NAME contains dots into the nested objects they describe.
///
/// `path` names the object to work on, empty for the document root, and
/// `field` the key to expand -- `*` for every dotted key it holds. A key that
/// is not there, or holds no dot, is left alone.
///
/// # Errors
///
/// Returns an error only if the event refuses a write.
pub fn dot_expand(event: &mut crate::Event, path: &str, field: &str) -> crate::Result<()> {
    // Decided on a BORROW, before anything is cloned. The rebuild below copies
    // the whole container, and for the root that is the entire document -- a
    // deep clone per event, on every source whose pipeline opens with a
    // `dot_expander`, whether or not a single key held a dot.
    let dotted = {
        let members = if path.is_empty() {
            event.as_value().as_object()
        } else {
            event.get(path).and_then(Value::as_object)
        };
        let Some(members) = members else {
            return Ok(());
        };
        members
            .keys()
            .any(|key| key.contains('.') && (field == "*" || key == field))
    };
    if !dotted {
        return Ok(());
    }

    let container = if path.is_empty() {
        event.as_value().clone()
    } else {
        match event.get(path) {
            Some(value) => value.clone(),
            None => return Ok(()),
        }
    };
    let Some(members) = container.as_object() else {
        return Ok(());
    };

    // Rebuilt whole rather than removed and re-set key by key: a dotted key
    // and the nested path that replaces it are spelled the same, so the two
    // operations would race over one name.
    let mut rebuilt = serde_json::Map::new();
    let mut expanded = false;
    for (key, value) in members {
        if key.contains('.') && (field == "*" || key == field) {
            expanded = true;
            let mut node = &mut rebuilt;
            let mut segments = key.split('.').peekable();
            while let Some(segment) = segments.next() {
                if segments.peek().is_none() {
                    node.insert(segment.to_string(), value.clone());
                    break;
                }
                node = node
                    .entry(segment.to_string())
                    .or_insert_with(|| Value::Object(serde_json::Map::new()))
                    .as_object_mut()
                    .ok_or_else(|| crate::TransformError::FieldNotFound { path: key.clone() })?;
            }
        } else if let (Some(Value::Object(into)), Value::Object(from)) =
            (rebuilt.get_mut(key), value)
        {
            // MERGE, never replace. A pipeline expands one dotted key per
            // processor, so by the second call the nested container already
            // exists -- and whether it is reached before or after the dotted
            // key decides everything. zeek writes `id.orig_h` BEFORE
            // `id.orig_p`, so expanding `orig_p` first put a nested `id`
            // AFTER the still-dotted `orig_h`; expanding `orig_h` then built
            // its own `id` and this branch overwrote it a moment later.
            // `source.address` vanished on ~30 of its streams that way, while
            // `id.resp_h` -- which sorts after the nested `id` -- survived.
            for (inner, value) in from {
                into.insert(inner.clone(), value.clone());
            }
        } else {
            rebuilt.insert(key.clone(), value.clone());
        }
    }

    if expanded {
        if path.is_empty() {
            *event.as_value_mut() = Value::Object(rebuilt);
        } else {
            event.set(path, Value::Object(rebuilt))?;
        }
    }
    Ok(())
}

/// Convert a value the way Elastic's `convert` processor does.
///
/// An ARRAY is converted element by element -- "if the field value is an
/// array, all members will be converted" -- where stringifying the array
/// whole gave `"[\"0\"]"` for a list that should have been `["0"]`, and the
/// pipeline's own sentinel pass then had nothing it recognised to remove.
///
/// `kind` is the processor's `type`: `integer`, `long`, `float`, `double`,
/// `string`, `boolean` or `ip`. `auto` picks its type at ingest time and a
/// compiled transform cannot do that, so the generator refuses it and it
/// never reaches here.
///
/// # Errors
///
/// Returns the message Elastic's processor throws with, for the caller to
/// wrap in a `ParseError` naming the field.
pub fn convert_value(value: &Value, kind: &str) -> std::result::Result<Value, String> {
    if let Value::Array(items) = value {
        return items
            .iter()
            .map(|item| convert_value(item, kind))
            .collect::<std::result::Result<Vec<_>, _>>()
            .map(Value::Array);
    }

    let cannot = |target: &str| format!("cannot convert '{value}' to {target}");
    match kind {
        "integer" | "long" => match value {
            Value::String(s) => {
                let s = s.trim();
                let parsed = s.strip_prefix("0x").map_or_else(
                    || s.parse::<i64>().ok(),
                    |hex| i64::from_str_radix(hex, 16).ok(),
                );
                parsed.map(Value::from).ok_or_else(|| cannot("integer"))
            }
            #[allow(clippy::cast_possible_truncation)]
            Value::Number(n) => Ok(Value::from(
                n.as_i64()
                    .unwrap_or_else(|| n.as_f64().unwrap_or(0.0) as i64),
            )),
            Value::Bool(b) => Ok(Value::from(i64::from(*b))),
            _ => Err(cannot("integer")),
        },
        // `float` is 32-bit in the convert processor and `double` is the
        // 64-bit one, so a rate converted as `float` keeps only the digits a
        // single carries. The route is the shortest decimal that names the
        // f32; widening the f32 back to binary lands further out than the
        // input was.
        "float" | "double" => {
            let narrow = kind == "float";
            let widened = |f: f64| {
                if narrow {
                    #[allow(clippy::cast_possible_truncation)]
                    let single = f as f32;
                    single.to_string().parse::<f64>().unwrap_or(f)
                } else {
                    f
                }
            };
            match value {
                Value::String(s) => s
                    .trim()
                    .parse::<f64>()
                    .ok()
                    .map(|f| Value::from(widened(f)))
                    .ok_or_else(|| cannot("float")),
                Value::Number(n) => Ok(Value::from(widened(n.as_f64().unwrap_or(0.0)))),
                Value::Bool(b) => Ok(Value::from(if *b { 1.0 } else { 0.0 })),
                _ => Err(cannot("float")),
            }
        }
        // Elastic's `string` convert is `String.valueOf(Object)`, so a
        // container renders as Java's `{k=v, k=v}` / `[a, b]` and never as its
        // JSON. qualys_was keeps a rendered copy of every finding's result list
        // in exactly that form.
        "string" => Ok(Value::from(crate::painless_helpers::painless_to_string(
            value,
        ))),
        // Elastic accepts only the exact strings, case insensitively, and
        // throws on anything else.
        "boolean" => match value {
            Value::Bool(_) => Ok(value.clone()),
            Value::String(s) if s.eq_ignore_ascii_case("true") => Ok(Value::from(true)),
            Value::String(s) if s.eq_ignore_ascii_case("false") => Ok(Value::from(false)),
            _ => Err(cannot("boolean")),
        },
        "ip" => match value.as_str() {
            Some(s) if s.trim().parse::<std::net::IpAddr>().is_ok() => Ok(Value::from(s.trim())),
            _ => Err(cannot("IP")),
        },
        other => Err(format!("unknown convert type '{other}'")),
    }
}

/// The bytes Elastic's `fingerprint` processor hashes for one value.
///
/// Every LEAF is preceded by a single NUL and written in the engine's own
/// binary form, little-endian throughout: a string as its UTF-8, an integer as
/// four bytes or eight when it will not fit, a double as eight, a boolean as 1
/// for true and 2 for false, and a null as the delimiter alone. A container
/// writes no marker of its own, so an empty one contributes nothing at all --
/// an empty map and an empty list both hash to the digest of no bytes.
///
/// A map's keys are walked SORTED, each written as a string leaf ahead of its
/// value, which is why two documents holding the same pairs in different order
/// fingerprint alike.
///
/// Read off Elasticsearch 9.2.2 through `_ingest/pipeline/_simulate` rather
/// than from its source: sixteen patterns, four methods and a salt, all
/// reproduced exactly.
fn fingerprint_bytes(value: &Value, out: &mut Vec<u8>) {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<&str> = map.keys().map(String::as_str).collect();
            keys.sort_unstable();
            for key in keys {
                out.push(0);
                out.extend_from_slice(key.as_bytes());
                if let Some(nested) = map.get(key) {
                    fingerprint_bytes(nested, out);
                }
            }
            return;
        }
        Value::Array(items) => {
            for item in items {
                fingerprint_bytes(item, out);
            }
            return;
        }
        _ => {}
    }

    out.push(0);
    match value {
        Value::Bool(set) => out.push(if *set { 1 } else { 2 }),
        Value::String(text) => out.extend_from_slice(text.as_bytes()),
        Value::Number(number) => {
            if let Some(whole) = number.as_i64() {
                match i32::try_from(whole) {
                    Ok(narrow) => out.extend_from_slice(&narrow.to_le_bytes()),
                    Err(_) => out.extend_from_slice(&whole.to_le_bytes()),
                }
            } else if let Some(real) = number.as_f64() {
                out.extend_from_slice(&real.to_le_bytes());
            }
        }
        // A null contributes the delimiter and nothing else.
        _ => {}
    }
}

/// The digest Elastic's `fingerprint` processor writes at its defaults.
///
/// Method `SHA-1`, no salt, and the result base64-encoded. `values` arrives in
/// the order the caller's FIELD NAMES sort, because the processor sorts them
/// and never hashes the names themselves.
#[must_use]
pub fn fingerprint_default(values: &[Value]) -> String {
    use sha1::{Digest, Sha1};

    encode_digest(&Sha1::digest(fingerprint_payload(values, "")))
}

/// The digest for a named `method`, with `salt` prepended to the payload.
///
/// The catalogue asks for `SHA-256` in 21 pipelines, `MurmurHash3` in three
/// and `MD5` in one.
///
/// # Errors
///
/// Names a method the runtime cannot produce, which is `MD5` -- no MD5
/// implementation is vendored, and one is not worth hand-rolling for the
/// single pipeline that asks.
pub fn fingerprint_with(
    values: &[Value],
    method: &str,
    salt: &str,
) -> std::result::Result<String, String> {
    use sha1::{Digest, Sha1};
    use sha2::{Sha256, Sha512};

    let payload = fingerprint_payload(values, salt);
    Ok(match method {
        "SHA-1" => encode_digest(&Sha1::digest(&payload)),
        "SHA-256" => encode_digest(&Sha256::digest(&payload)),
        "SHA-512" => encode_digest(&Sha512::digest(&payload)),
        "MurmurHash3" => {
            let (high, low) = murmur3_x64_128(&payload);
            let mut bytes = [0u8; 16];
            bytes[..8].copy_from_slice(&high.to_be_bytes());
            bytes[8..].copy_from_slice(&low.to_be_bytes());
            encode_digest(&bytes)
        }
        other => return Err(format!("unsupported fingerprint method '{other}'")),
    })
}

/// The bytes a fingerprint hashes: the salt, then every value in turn.
fn fingerprint_payload(values: &[Value], salt: &str) -> Vec<u8> {
    let mut payload = salt.as_bytes().to_vec();
    for value in values {
        fingerprint_bytes(value, &mut payload);
    }
    payload
}

fn encode_digest(digest: &[u8]) -> String {
    use base64::Engine as _;
    base64::engine::general_purpose::STANDARD.encode(digest)
}

/// `MurmurHash3` x64 128-bit at seed 0, returned as its two halves in the
/// order Elasticsearch writes them.
///
/// Hand-written rather than taken from `murmur3` 0.4.1, which is the only
/// version vendored and does not build on this edition -- it spells `panic!(e)`
/// and `9...15`. The algorithm is fixed and the engine's own digests pin it.
fn murmur3_x64_128(data: &[u8]) -> (u64, u64) {
    const C1: u64 = 0x87c3_7b91_1142_53d5;
    const C2: u64 = 0x4cf5_ad43_2745_937f;

    fn fmix64(mut k: u64) -> u64 {
        k ^= k >> 33;
        k = k.wrapping_mul(0xff51_afd7_ed55_8ccd);
        k ^= k >> 33;
        k = k.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
        k ^= k >> 33;
        k
    }

    let (mut h1, mut h2) = (0u64, 0u64);
    let (blocks, tail) = data.as_chunks::<16>();
    for block in blocks {
        let (first, second) = block.split_at(8);
        let mut k1 = u64::from_le_bytes(first.try_into().unwrap_or_default());
        let mut k2 = u64::from_le_bytes(second.try_into().unwrap_or_default());

        k1 = k1.wrapping_mul(C1).rotate_left(31).wrapping_mul(C2);
        h1 ^= k1;
        h1 = h1.rotate_left(27).wrapping_add(h2);
        h1 = h1.wrapping_mul(5).wrapping_add(0x52dc_e729);

        k2 = k2.wrapping_mul(C2).rotate_left(33).wrapping_mul(C1);
        h2 ^= k2;
        h2 = h2.rotate_left(31).wrapping_add(h1);
        h2 = h2.wrapping_mul(5).wrapping_add(0x3849_5ab5);
    }

    let (mut k1, mut k2) = (0u64, 0u64);
    for (index, byte) in tail.iter().enumerate() {
        if index < 8 {
            k1 |= u64::from(*byte) << (8 * index);
        } else {
            k2 |= u64::from(*byte) << (8 * (index - 8));
        }
    }
    if tail.len() > 8 {
        k2 = k2.wrapping_mul(C2).rotate_left(33).wrapping_mul(C1);
        h2 ^= k2;
    }
    if !tail.is_empty() {
        k1 = k1.wrapping_mul(C1).rotate_left(31).wrapping_mul(C2);
        h1 ^= k1;
    }

    let length = data.len() as u64;
    h1 ^= length;
    h2 ^= length;
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    h1 = fmix64(h1);
    h2 = fmix64(h2);
    h1 = h1.wrapping_add(h2);
    h2 = h2.wrapping_add(h1);
    (h1, h2)
}

/// Sort an array's elements, the way Elastic's `sort` processor does.
///
/// Elastic sorts by the elements' natural ordering, so the array has to be
/// all-numbers or all-strings; anything mixed throws in Java. Booleans sort
/// false before true, which is Java's `Boolean.compareTo`.
///
/// Returns `None` when the value is not an array, or holds something with no
/// natural ordering -- an object, a nested array, a null, or a mix of kinds.
/// That is the case Elastic throws on, so the caller raises.
#[must_use]
pub fn sort_values(value: &Value, descending: bool) -> Option<Vec<Value>> {
    let items = value.as_array()?;

    let mut sorted = items.clone();
    let all = |f: fn(&Value) -> bool| items.iter().all(f);
    if all(Value::is_string) {
        sorted.sort_by(|a, b| a.as_str().cmp(&b.as_str()));
    } else if all(Value::is_number) {
        sorted.sort_by(|a, b| {
            a.as_f64()
                .partial_cmp(&b.as_f64())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    } else if all(Value::is_boolean) {
        sorted.sort_by_key(Value::as_bool);
    } else if !items.is_empty() {
        return None;
    }

    if descending {
        sorted.reverse();
    }
    Some(sorted)
}

/// Percent-decode a string the way Elastic's `urldecode` processor does.
///
/// The processor calls Java's `URLDecoder.decode(value, "UTF-8")`, which is
/// the `application/x-www-form-urlencoded` reading rather than the RFC 3986
/// one: `+` becomes a SPACE. Zscaler's rule labels arrive that way.
///
/// Returns `None` on a malformed escape -- a `%` with fewer than two hex
/// digits after it -- which is where Java throws and the processor's
/// `on_failure` runs. Borrows when there is nothing to decode.
#[must_use]
pub fn url_decode(text: &str) -> Option<std::borrow::Cow<'_, str>> {
    if !text.contains('%') && !text.contains('+') {
        return Some(std::borrow::Cow::Borrowed(text));
    }

    let bytes = text.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' => {
                let hex = text.get(i + 1..i + 3)?;
                out.push(u8::from_str_radix(hex, 16).ok()?);
                i += 3;
            }
            byte => {
                out.push(byte);
                i += 1;
            }
        }
    }
    String::from_utf8(out).ok().map(std::borrow::Cow::Owned)
}

/// Join an array's elements into one separated string, the way Elastic's
/// `join` processor does.
///
/// Every element is stringified first -- Elastic calls `toString()` on each,
/// so a list of numbers joins as readily as a list of strings, and only a
/// nested object or array has no sensible rendering. Those are skipped rather
/// than written as their JSON, which is what `to_string` would give and is
/// never what the pipeline meant.
///
/// Returns `None` when the value is not an array, which is the case Elastic
/// throws on: writing a joined string for a scalar would invent one.
#[must_use]
pub fn join_values(value: &Value, separator: &str) -> Option<String> {
    let items = value.as_array()?;
    let mut out = String::new();
    // A `first` flag, not `out.is_empty()`: an empty string as the FIRST
    // element leaves `out` empty and would swallow the separator after it.
    let mut first = true;
    for item in items {
        let piece = match item {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null | Value::Array(_) | Value::Object(_) => continue,
        };
        if !first {
            out.push_str(separator);
        }
        first = false;
        out.push_str(&piece);
    }
    Some(out)
}

/// Close the gap between a delimiter and an opening quote, and nothing else.
///
/// Elasticsearch's CSV processor treats `, "a,b"` as a quoted field; a strict
/// reader treats the space as content, so the field is unquoted and its
/// embedded commas shift every column after it. Trimming the whole field would
/// fix that and also strip whitespace the vendor keeps -- panw's descriptions
/// carry it -- so only the run between a delimiter and a quote is removed.
///
/// Borrows unless there is something to remove.
#[must_use]
pub fn csv_close_quote_gap(line: &str, delimiter: char, quote: char) -> std::borrow::Cow<'_, str> {
    let mut out: Option<String> = None;
    let mut inside = false;
    let mut at_field_start = true;
    let mut chars = line.char_indices().peekable();

    while let Some((index, c)) = chars.next() {
        if c == quote {
            inside = !inside;
            at_field_start = false;
        } else if c == delimiter && !inside {
            at_field_start = true;
        } else if at_field_start && c.is_whitespace() && !inside {
            // Whitespace opening a field: drop it only if a quote follows the
            // whole run, which is what makes the field a quoted one.
            let run: String = std::iter::once(c)
                .chain(std::iter::from_fn(|| {
                    chars.next_if(|(_, n)| n.is_whitespace()).map(|(_, n)| n)
                }))
                .collect();
            if chars.peek().is_some_and(|(_, n)| *n == quote) {
                out.get_or_insert_with(|| line[..index].to_string());
                continue;
            }
            if let Some(kept) = out.as_mut() {
                kept.push_str(&run);
            }
            at_field_start = false;
            continue;
        } else {
            at_field_start = false;
        }
        if let Some(kept) = out.as_mut() {
            kept.push(c);
        }
    }

    out.map_or(std::borrow::Cow::Borrowed(line), std::borrow::Cow::Owned)
}

/// Convert a grok pattern string to a regex pattern string.
///
/// Expands `%{NAME:field}` to named capture groups with type-appropriate
/// sub-patterns. Returns `(regex_string, field_map)` where `field_map` maps
/// safe capture names back to original dotted field paths.
///
/// Phase 3 will replace grok with native dfe-parse parsers.
pub fn grok_to_regex(pattern: &str) -> String {
    grok_to_regex_with_map(pattern).0
}

/// Like `grok_to_regex` but also returns a map of `capture_name` → `original_field_path`.
///
/// This is needed because regex capture names can't contain dots, so
/// `user.name` becomes `user_name` in the regex. The map lets callers
/// restore the original dotted path when setting fields.
pub fn grok_to_regex_with_map(
    pattern: &str,
) -> (String, std::collections::HashMap<String, String>) {
    let (regex, field_map, _) = grok_to_regex_typed(pattern);
    (regex, field_map)
}

/// How Elastic types a grok capture, where it types it at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureType {
    Number,
    Boolean,
}

/// As [`grok_to_regex_with_map`], plus the type Elastic gives each capture
/// that names one.
///
/// A `%{NUMBER:bytes:long}` or `%{WORD:flag:boolean}` suffix is a type, not
/// part of the field name, and dropping it leaves every one of them a string.
#[must_use]
pub fn grok_to_regex_typed(
    pattern: &str,
) -> (
    String,
    std::collections::HashMap<String, String>,
    std::collections::HashMap<String, CaptureType>,
) {
    use std::fmt::Write as _;

    let mut result = String::with_capacity(pattern.len());
    let mut field_map = std::collections::HashMap::new();
    let mut capture_types = std::collections::HashMap::new();
    // Every group name emitted so far. One ledger for both kinds of capture,
    // because a `%{DATA:process.name}` and a literal `(?P<process_name>)` in
    // the same pattern collide just as surely as two of either.
    let mut used: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut chars = pattern.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '(' && starts_named_group(&chars) {
            // A named group written out in full, either by the vendor or by
            // the generator inlining a pattern definition. Java's engine takes
            // the same name in several alternation branches; Rust's rejects
            // the pattern outright, so each repeat is renamed and pointed at
            // the same destination -- only the branch that matched writes.
            let prefix: String = chars.by_ref().take_while(|c| *c != '<').collect();
            let name: String = chars.by_ref().take_while(|c| *c != '>').collect();
            let safe = unique_group_name(&sanitise_group_name(&name), &mut used);
            field_map.insert(safe.clone(), name);
            let _ = write!(result, "({prefix}<{safe}>");
            continue;
        }
        if c == '%' && chars.peek() == Some(&'{') {
            chars.next(); // consume '{'
            let mut name = String::new();
            let mut field = String::new();
            let mut in_field = false;

            for ch in chars.by_ref() {
                if ch == '}' {
                    break;
                } else if ch == ':' && !in_field {
                    in_field = true;
                } else if in_field {
                    field.push(ch);
                } else {
                    name.push(ch);
                }
            }

            let sub_pattern = grok_pattern_regex(&name);

            if field.is_empty() {
                // A few builtins carry their own destination, as Elastic's own
                // registry defines them -- used bare, they still capture.
                if let Some((safe, path, inner)) = grok_implicit_capture(&name) {
                    used.insert(safe.to_string());
                    field_map.insert(safe.to_string(), path.to_string());
                    capture_types.insert(safe.to_string(), CaptureType::Number);
                    let _ = write!(result, "{inner}");
                } else {
                    let _ = write!(result, "({sub_pattern})");
                }
            } else {
                // Strip Elastic type suffix (e.g., "source.ip:ip" → "source.ip")
                let mut parts = field.splitn(2, ':');
                let field_name = parts.next().unwrap_or(&field);
                // Java allows one field name in several alternation branches
                // and Rust's engine rejects a duplicate group name outright, so
                // the whole pattern fails to compile and matches nothing. Each
                // repeat gets a group of its own pointing at the same field;
                // only the branch that matched writes.
                let safe_field = unique_group_name(&sanitise_group_name(field_name), &mut used);

                if let Some(capture_type) = parts.next().and_then(capture_type_of) {
                    capture_types.insert(safe_field.clone(), capture_type);
                }
                field_map.insert(safe_field.clone(), field_name.to_string());
                let _ = write!(result, "(?P<{safe_field}>{sub_pattern})");
            }
        } else if c == '\\' {
            // Copy an escape whole, so the char it protects is never read as
            // syntax on the next turn of the loop.
            result.push(c);
            if let Some(escaped) = chars.next() {
                result.push(escaped);
                // `\p{L}` names a Unicode general category, and its brace is
                // part of the escape. Left to the arm below it became `\p\{L}`,
                // which is not a class at all -- hpe_aruba_cx is the first
                // vendor pattern to reach for one.
                if matches!(escaped, 'p' | 'P') && chars.peek() == Some(&'{') {
                    for ch in chars.by_ref() {
                        result.push(ch);
                        if ch == '}' {
                            break;
                        }
                    }
                }
            }
        } else if c == '{' && !opens_repetition(&chars) {
            // Elasticsearch groks with Oniguruma, which reads a brace that is
            // not a valid repetition as ordinary text. Rust's engine reads it
            // as a repetition with nothing to repeat and rejects the pattern.
            // Elastic's own SYSLOG_HEADER carries `(?:{DATA})?` -- a `%` short
            // of a pattern reference, and so a harmless optional literal there
            // and a dead grok here.
            result.push_str(r"\{");
        } else {
            result.push(c);
        }
    }

    resolve_capture_paths(&mut field_map, &mut capture_types);
    (result, field_map, capture_types)
}

/// Does an opening paren begin a named capture, rather than a look-behind?
///
/// `(?P<x>` and `(?<x>` are captures; `(?<=` and `(?<!` are assertions, and
/// renaming inside one would corrupt the pattern.
fn starts_named_group(chars: &std::iter::Peekable<std::str::Chars<'_>>) -> bool {
    let mut ahead = chars.clone();
    if ahead.next() != Some('?') {
        return false;
    }
    match ahead.next() {
        Some('P') => ahead.next() == Some('<'),
        Some('<') => !matches!(ahead.next(), Some('=' | '!') | None),
        _ => false,
    }
}

/// Does a brace begin a repetition -- `{2}`, `{2,}`, `{2,5}` -- or is it text?
fn opens_repetition(chars: &std::iter::Peekable<std::str::Chars<'_>>) -> bool {
    let inner: String = chars.clone().take(16).take_while(|c| *c != '}').collect();
    if inner.is_empty() || inner.len() == 16 {
        return false;
    }
    let (low, high) = inner.split_once(',').unwrap_or((inner.as_str(), ""));
    !low.is_empty()
        && low.bytes().all(|b| b.is_ascii_digit())
        && high.bytes().all(|b| b.is_ascii_digit())
}

/// Reduce a capture name to what a regex group name may hold.
///
/// Elastic's grok reads the name as `[pattern:]field[:type]` and the vendor
/// pipelines write dotted paths straight into it, so a name arrives carrying
/// dots and colons that Rust's engine will not accept. The real destination is
/// recovered from the name by [`resolve_capture_paths`]; this only has to
/// produce something the engine can compile.
fn sanitise_group_name(name: &str) -> String {
    let mut safe: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect();
    if safe.starts_with(|c: char| c.is_ascii_digit()) {
        safe.insert(0, 'g');
    }
    if safe.is_empty() {
        safe.push('g');
    }
    safe
}

/// Return `base`, or the first `base__N` nobody has taken.
fn unique_group_name(base: &str, used: &mut std::collections::HashSet<String>) -> String {
    if used.insert(base.to_string()) {
        return base.to_string();
    }
    for n in 2.. {
        let candidate = format!("{base}__{n}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
    }
    unreachable!("the counter is unbounded")
}

/// Numeric type suffixes Elastic's grok understands.
const NUMERIC_TYPES: [&str; 4] = ["int", "long", "float", "double"];

/// The [`CaptureType`] a grok type suffix names, when it names one at all.
///
/// The single point where a suffix string is read against both categories,
/// so [`grok_to_regex_typed`] and [`read_capture_name`] cannot drift apart on
/// which suffixes count.
fn capture_type_of(ty: &str) -> Option<CaptureType> {
    if NUMERIC_TYPES.contains(&ty) {
        Some(CaptureType::Number)
    } else if ty == "boolean" {
        Some(CaptureType::Boolean)
    } else {
        None
    }
}

/// Read a grok capture name as Elastic does: `[pattern:]field[:type]`.
///
/// The vendor pipelines write the internal form by hand -- `cisco_asa`'s repeat
/// counter is `(?<INT:_temp_.cisco.message_repeats:int>\d+)` -- so the field is
/// the middle part and the type the last, not the whole string.
///
/// Returns the destination path and the type Elastic gives it, if the suffix
/// names one it recognises.
#[must_use]
pub fn read_capture_name(name: &str) -> (&str, Option<CaptureType>) {
    let parts: Vec<&str> = name.split(':').collect();
    match parts.as_slice() {
        [_, field, ty] => (field, capture_type_of(ty)),
        [field, ty] => match capture_type_of(ty) {
            Some(capture_type) => (field, Some(capture_type)),
            None => (name, None),
        },
        _ => (name, None),
    }
}

/// Rewrite every destination in `field_map` to the path Elastic would write,
/// recording each capture's type. Idempotent, so it can run again once a
/// caller's own mapping has been substituted in.
pub fn resolve_capture_paths<S: std::hash::BuildHasher>(
    field_map: &mut std::collections::HashMap<String, String, S>,
    capture_types: &mut std::collections::HashMap<String, CaptureType, S>,
) {
    for (capture, path) in field_map.iter_mut() {
        let (field, capture_type) = read_capture_name(path);
        if let Some(capture_type) = capture_type {
            capture_types.insert(capture.clone(), capture_type);
        }
        if field.len() != path.len() {
            *path = field.to_string();
        }
    }
}

/// Builtins whose Elastic definition captures a field of its own.
///
/// `%{SYSLOG5424PRI}` is written without a field name throughout the vendor
/// pipelines because the destination is part of the pattern. Elastic's registry
/// defines it as `<%{NONNEGINT:syslog5424_pri}>`, and the pipelines that use it
/// then remove that field by name -- writing it to `log.syslog.priority`
/// instead left a field Elastic does not emit on every event.
///
/// Returns `(safe capture name, dotted path, the regex to emit)`.
fn grok_implicit_capture(name: &str) -> Option<(&'static str, &'static str, &'static str)> {
    match name {
        "SYSLOG5424PRI" => Some((
            "syslog5424_pri",
            "syslog5424_pri",
            r"<(?P<syslog5424_pri>\d{1,5})>",
        )),
        _ => None,
    }
}

/// A month name, abbreviated or spelled out.
///
/// `\w+` would do here too, but it also matches a bare word, so a pattern
/// meant to anchor on a date matches text that holds none.
const MONTH: &str = concat!(
    r"(?:Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?",
    r"|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)",
);

/// A clock time. Elastic's `SECOND` carries an optional fraction, so a
/// pattern anchored on `%{TIME}` has to accept `13:20:48.739`.
const TIME: &str = r"\d{1,2}:\d{2}(?::\d{2}(?:[.,]\d+)?)?";

/// `%{MONTHDAY}/%{MONTH}/%{YEAR}:%{TIME} %{INT}` -- the Apache common-log
/// date, which the AWS load-balancer and cloudfront pipelines grok.
const HTTPDATE: &str = concat!(
    r"\d{1,2}/",
    r"(?:Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?",
    r"|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)",
    r"/\d{4}:\d{1,2}:\d{2}(?::\d{2}(?:[.,]\d+)?)? [+-]?\d+",
);

/// `%{MONTH} +%{MONTHDAY} %{TIME}` -- the BSD syslog date, fraction and all.
const SYSLOG_TIMESTAMP: &str = concat!(
    r"(?:Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?",
    r"|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)",
    r" +\d{1,2} \d{1,2}:\d{2}(?::\d{2}(?:[.,]\d+)?)?",
);

/// Cisco's syslog date: the year may sit on either side of the time, and the
/// seconds may carry a fraction -- `Jan  6 2022 20:52:12.861`.
const CISCO_TIMESTAMP: &str = concat!(
    r"(?:Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?",
    r"|Aug(?:ust)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?)",
    r" +\d{1,2}(?: \d{4})? \d{2}:\d{2}:\d{2}(?:\.\d+)?(?: \d{4})?",
);

/// Elastic's own `IPV6`, verbatim from logstash-patterns-core's ecs-v1 set.
///
/// The `[0-9a-fA-F:]+` this replaces matched any run of hex and colons -- a
/// bare `2a02` included -- and `IP` carried no v6 branch at all, so a grok
/// reading a v6 address failed outright and took every capture in the pattern
/// with it. `cisco_ios`'s syslog header is exactly that pattern.
const IPV6: &str = r"((([0-9A-Fa-f]{1,4}:){7}([0-9A-Fa-f]{1,4}|:))|(([0-9A-Fa-f]{1,4}:){6}(:[0-9A-Fa-f]{1,4}|((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(([0-9A-Fa-f]{1,4}:){5}(((:[0-9A-Fa-f]{1,4}){1,2})|:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(([0-9A-Fa-f]{1,4}:){4}(((:[0-9A-Fa-f]{1,4}){1,3})|((:[0-9A-Fa-f]{1,4})?:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){3}(((:[0-9A-Fa-f]{1,4}){1,4})|((:[0-9A-Fa-f]{1,4}){0,2}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){2}(((:[0-9A-Fa-f]{1,4}){1,5})|((:[0-9A-Fa-f]{1,4}){0,3}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){1}(((:[0-9A-Fa-f]{1,4}){1,6})|((:[0-9A-Fa-f]{1,4}){0,4}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(:(((:[0-9A-Fa-f]{1,4}){1,7})|((:[0-9A-Fa-f]{1,4}){0,5}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:)))(%.+)?";

/// The simplified `IPV4`. Elastic's own guards it with `(?<![0-9])` and
/// `(?![0-9])` and range-checks every octet; ours does neither, so a greedy
/// `%{GREEDYDATA}` in front backs off to the shortest satisfying suffix and
/// `[AF_INET]175.16.199.1:34745` reads a `source.ip` of `5.16.199.1`.
/// `999.999.999.999` is accepted too.
///
/// Taking the guards is a throughput decision rather than a compile one:
/// `Pattern::compile` falls back to `fancy-regex` for lookaround, the way
/// `BASE16NUM` below does, and that engine measures 55x slower than `regex` on
/// a line the pattern does not match. `%{IP}` and `%{IPORHOST}` are spelt
/// across most of the generated tree, and a line a pattern does not match is
/// the common case.
const IPV4: &str = r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}";

/// A hostname as Elastic defines it, for the composites below.
const HOSTNAME: &str = r"[a-zA-Z0-9._-]+";

/// `%{IP}` is `(?:%{IPV6}|%{IPV4})` and `%{IPORHOST}` is `(?:%{IP}|%{HOSTNAME})`,
/// v6 first, exactly as Elastic orders them. Built once because `concat!` will
/// not expand a const, and this runs when a pattern compiles, never per event.
static IP: LazyLock<String> = LazyLock::new(|| format!("(?:{IPV6}|{IPV4})"));
static IPORHOST: LazyLock<String> = LazyLock::new(|| format!("(?:{IPV6}|{IPV4}|{HOSTNAME})"));

/// Map well-known grok pattern names to their regex equivalents.
fn grok_pattern_regex(name: &str) -> &'static str {
    match name {
        "USER" | "USERNAME" | "HOSTNAME" => HOSTNAME,
        "IP" => IP.as_str(),
        "IPV4" => IPV4,
        "IPV6" => IPV6,
        // Elastic's own, word boundaries and all. Without them a digit run can
        // be entered part-way or split in two, and both cost real fields:
        // `%{DATA}.%{NONNEGINT:observer.ingress.vlan.id}` over `igb1.12` read
        // the `1` out of `igb1` instead of the vlan, and pfsense's TCP grok
        // spelt `%{NONNEGINT:pfsense.tcp.seq:long}?:?%{NONNEGINT}` split
        // `1891286705` into a named `189128670` and an unnamed `5` where
        // Elasticsearch declines the optional capture and reads the whole run.
        // `PORT` keeps the bare form: Elastic defines it `(?:[\d]{1,5})`, with
        // no boundary of its own.
        "NONNEGINT" => r"\b(?:[0-9]+)\b",
        "POSINT" => r"\b(?:[1-9][0-9]*)\b",
        "PORT" => r"\d+",
        "INT" => r"[+-]?\d+",
        "NUMBER" | "BASE10NUM" => r"[+-]?(?:\d+\.?\d*|\.\d+)",
        // Elastic's own, look-behind and all, so it compiles on fancy-regex.
        // Without the guard `deadbeef` would match starting at `eadbeef`.
        "BASE16NUM" => r"(?<![0-9A-Fa-f])(?:[+-]?(?:0x)?(?:[0-9A-Fa-f]+))",
        "NOTSPACE" | "URI" | "URIPROTO" => r"\S+",
        // Elastic's own URI parts, verbatim. On the catch-all `URIPATHPARAM`
        // captured arbitrary text, so pfsense's haproxy line read the whole
        // request target AND the `HTTP/1.1` after it into `url.original`.
        "URIPATH" => r"(?:/[A-Za-z0-9$.+!*'(){},~:;=@#%&_\-]*)+",
        "URIPARAM" => r"\?[A-Za-z0-9$.+!*'|(){},~@#%&/=:;_?\-\[\]<>]*",
        "URIPATHPARAM" => {
            r"(?:/[A-Za-z0-9$.+!*'(){},~:;=@#%&_\-]*)+(?:\?[A-Za-z0-9$.+!*'|(){},~@#%&/=:;_?\-\[\]<>]*)?"
        }
        "GREEDYDATA" => r".*",
        "DATA" => r".*?",
        // Elastic's own is `\b\w+\b`. Measured over the whole corpus, taking
        // the boundaries buys ONE field and costs 7.5% on a matching grok and
        // 30% on a non-matching one, across 2,194 sites -- so the bare form
        // stands until that trade is taken deliberately.
        "WORD" => r"\w+",
        "MONTH" => MONTH,
        // Elastic's `MAC` is `(?:%{CISCOMAC}|%{WINDOWSMAC}|%{COMMONMAC})`, and
        // the Cisco form is the one this used to miss. `0200.0000.0000` failed
        // cisco_aironet's whole AWIPS grok, which took `cisco.ap_name`, both
        // MACs and the alarm fields with it, raised a `pipeline_error`
        // Elasticsearch does not, and left the `_temp_` scratch unpruned
        // because the prune sits after the step that failed -- 42 events and
        // 399 extra fields on one missing alternative.
        // Wrapped, because an unnamed `%{MAC}` substitutes BARE and a top-level
        // `|` would then split the whole enclosing pattern rather than itself.
        "MAC" => {
            r"(?:(?:[A-Fa-f0-9]{4}\.){2}[A-Fa-f0-9]{4}|(?:[0-9a-fA-F]{2}[:-]){5}[0-9a-fA-F]{2})"
        }
        "EMAILADDRESS" => r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}",
        // Elastic's own three. A Windows path admits SPACES between its
        // backslashes, which `[^\s]+` refused: sophos_central's
        // `C:\Program Files\Bad Vendor\Bad Program.exe` failed the whole
        // ips_threat grok and took every `raw_data` field with it. The atomic
        // group Elastic writes around the drive letter is dropped -- it changes
        // backtracking, not the language, and `(?>` would push every pattern
        // that uses PATH onto the backtracking engine.
        "UNIXPATH" => r"(?:/(?:[\w_%!$@:.,+~-]+|\\.)*)+",
        "WINPATH" => r"(?:[A-Za-z]+:|\\)(?:\\[^\\?*]*)+",
        "PATH" => r"(?:(?:/(?:[\w_%!$@:.,+~-]+|\\.)*)+|(?:[A-Za-z]+:|\\)(?:\\[^\\?*]*)+)",
        // Azure custom patterns (from pipeline pattern_definitions)
        "SUBID" => {
            r"(?:\{)?[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}(?:\})?"
        }
        // A path segment. `PROVIDER` on the catch-all `.+?` matched lazily,
        // so `/providers/Microsoft.aadiam` yielded "M".
        "GROUPID" | "PROVIDERNAME" | "PROVIDER" | "NAMESPACE" | "RULE" | "NAME" => r"[^/]+",
        "MONTHDAY" | "MONTHNUM" => r"\d{1,2}",
        // Elastic's own: the ZERO-PADDED month, two digits always. Grouped,
        // because an inlined alternation would reach past whatever sits either
        // side of it in the pattern.
        "MONTHNUM2" => r"(?:0[1-9]|1[0-2])",
        "YEAR" => r"\d{4}",
        // Elastic's own, and each part earns its pattern. HOUR takes one digit
        // or two, because an offset is written `-5:00` as often as `-05:00`.
        // SECOND carries an optional fraction, without which checkpoint's
        // `16:39:12.000Z` leaves `.000Z` for the next literal to fail on.
        "HOUR" | "ISO8601_HOUR" => r"(?:2[0-3]|[01]?\d)",
        "MINUTE" => r"[0-5]\d",
        "SECOND" => r"(?:[0-5]?\d|60)(?:[:.,]\d+)?",
        "ISO8601_TIMEZONE" => r"(?:Z|[+-](?:2[0-3]|[01]?\d)(?::?[0-5]\d))",
        // Whitespace, not "anything". The catch-all below made every pattern
        // containing %{SPACE} match arbitrary text -- 152 sites' worth.
        "SPACE" => r"\s*",
        "TIME" => TIME,
        "HTTPDATE" => HTTPDATE,
        "IPORHOST" | "SYSLOGHOST" => IPORHOST.as_str(),
        // Elastic accepts the abbreviation or the full name, either case.
        "DAY" => {
            r"(?i:Mon(?:day)?|Tue(?:sday)?|Wed(?:nesday)?|Thu(?:rsday)?|Fri(?:day)?|Sat(?:urday)?|Sun(?:day)?)"
        }
        "SYSLOGPRI" => r"<\d+>",
        // Elastic's own: a syslog program name is any printable character but
        // a `[`, which is what ends it before the pid.
        "PROG" => r"[\x21-\x5a\x5c\x5e-\x7e]+",
        // RFC 5424 structured data, one or more bracketed elements.
        "SYSLOG5424SD" => r"(?:\[.*?\]+)",
        "UUID" => r"[A-Fa-f0-9]{8}-(?:[A-Fa-f0-9]{4}-){3}[A-Fa-f0-9]{12}",
        // Elastic's own, and each is GROUPED: an inlined alternation would
        // otherwise reach past whatever sits either side of it in the pattern.
        "BASE16FLOAT" => {
            r"(?:\b(?<![0-9A-Fa-f.])(?:[+-]?(?:0x)?(?:(?:[0-9A-Fa-f]+(?:\.[0-9A-Fa-f]*)?)|(?:\.[0-9A-Fa-f]+)))\b)"
        }
        "JAVACLASS" => r"(?:(?:[a-zA-Z$_][a-zA-Z$_0-9]*\.)*[a-zA-Z$_][a-zA-Z$_0-9]*)",
        // `logstash-patterns-core` defines this as `(.*)` -- no dotall. Ours
        // carried a `(?s)`, which was compensating for a vendor `(?m)` that
        // used to arrive as a no-op: kafka's header grok stopped at the first
        // line, so the re-capture after it could swallow the rest and still
        // land on the right answer. With `(?m)` now respelt as the dotall it
        // is, the header takes the whole event and this one has to stop at the
        // newline, exactly as joni does, or a stack trace ends up in `message`.
        "JAVALOGMESSAGE" => r"(?:.*)",
        // The facility half of a syslog priority: a name or a number.
        "SYSLOGFACILITY" => r"(?:<\d+\.\d+>)",
        // Elastic's own, grouped: an inlined alternation would otherwise reach
        // past whatever sits either side of it in the pattern.
        "MONGO3_SEVERITY" => r"\w",
        "MONGO3_COMPONENT" => r"(?:\w+|-)",
        "SYSLOG5424PRI" => r"<\d{1,5}>",
        // Printable ASCII minus space, `=`, `]` and `"` -- RFC 5424's own
        // definition, which is what bounds a structured-data name.
        "SYSLOG5424PRINTASCII" => r"[!#-<>-\\\^-~]+",
        "SYSLOGTIMESTAMP" => SYSLOG_TIMESTAMP,
        "CISCOTIMESTAMP" => CISCO_TIMESTAMP,
        "CISCOMAC" => r"(?:[A-Fa-f0-9]{4}\.){2}[A-Fa-f0-9]{4}",
        // Elastic quotes with any of the three, and reading only the double
        // form left cisco_meraki's `ssid=''` unmatched -- which took the whole
        // key-value line with it, on every airmarshal event.
        "QS" | "QUOTEDSTRING" => r#"(?:"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'|`(?:[^`\\]|\\.)*`)"#,
        "LOGLEVEL" => r"(?i:emerg|alert|crit|err|warn|notice|info|debug|trace)\w*",
        // The fraction takes a comma as well as a dot: Elastic's SECOND does,
        // and hadoop's log4j writes `05:04:53,776` -- one missing comma
        // failed the whole emr grok on every event.
        "TIMESTAMP_ISO8601" => {
            r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:[.,]\d+)?(?:Z|[+-]\d{2}:?\d{2})?"
        }
        // Unknown name. `.+?` captures arbitrary text rather than failing, so
        // the field is populated with the wrong thing and nothing says so --
        // `unknown_grok_patterns` exists to make that visible.
        _ => ".+?",
    }
}

/// Pattern names still falling through to the catch-all, and how often.
///
/// The vendor-specific ones (`CISCO_*`, `NEXUS_*`, `IPV6PORTSEP`, ...) come
/// from `pattern_definitions` in the upstream ingest pipelines and have to be
/// carried across before they can be defined here.
#[must_use]
pub fn is_known_grok_pattern(name: &str) -> bool {
    grok_pattern_regex(name) != ".+?"
}

/// Check whether an IP address is in a private/internal range.
///
/// Recognises RFC 1918 (10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16),
/// loopback (127.0.0.0/8), and link-local (169.254.0.0/16).
pub fn is_internal_ip(ip: &str) -> bool {
    use std::net::IpAddr;
    match ip.parse::<IpAddr>() {
        Ok(IpAddr::V4(v4)) => v4.is_private() || v4.is_loopback() || v4.is_link_local(),
        Ok(IpAddr::V6(v6)) => v6.is_loopback(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod unicode_class_tests {
    /// The brace of `\p{L}` belongs to the escape, not to a repetition.
    /// Escaping it gave `\p\{L}`, which is not a class at all, and a pattern
    /// that will not compile matches nothing rather than failing loudly.
    /// `hpe_aruba_cx` is the first vendor pattern to reach for one.
    #[test]
    fn a_unicode_general_category_survives_expansion() {
        let site = r#"is (?P<aruba_status>(?:[\p{L},":;\s\-]*)). %{GREEDYDATA:event.reason}"#;
        let (expanded, _, _) = super::grok_to_regex_typed(site);
        assert!(expanded.contains(r"\p{L}"), "{expanded}");
        assert!(regex::Regex::new(&expanded).is_ok(), "{expanded}");
    }

    /// A brace that is NOT part of an escape is still ordinary text, which is
    /// what Elastic's own `(?:{DATA})?` relies on.
    #[test]
    fn a_bare_brace_is_still_escaped() {
        let (expanded, _, _) = super::grok_to_regex_typed("(?:{DATA})?");
        assert!(expanded.contains(r"\{DATA"), "{expanded}");
        assert!(regex::Regex::new(&expanded).is_ok(), "{expanded}");
    }
}

/// Lucene's inline elements. Every other element is block level.
///
/// This is the `InlineElment` macro from Lucene's
/// `analysis/common/src/java/org/apache/lucene/analysis/charfilter/HTMLStripCharFilter.jflex`
/// (the misspelling is upstream's), folded to lower case and sorted so the
/// lookup is a binary search. The macro's first alternative is the character
/// class `[aAbBiIqQsSuU]`, which is the six single-letter names here.
///
/// It is shorter than HTML's own inline list: `applet`, `button`, `del`,
/// `iframe`, `ins`, `map`, `object` and `param` are absent, so Lucene treats
/// those as block level and so do we.
const INLINE_ELEMENTS: [&str; 31] = [
    "a", "abbr", "acronym", "b", "basefont", "bdo", "big", "cite", "code", "dfn", "em", "font",
    "i", "img", "input", "kbd", "label", "q", "s", "samp", "select", "small", "span", "strike",
    "strong", "sub", "sup", "textarea", "tt", "u", "var",
];

/// The longest name in [`INLINE_ELEMENTS`] -- `basefont` and `textarea`.
const MAX_INLINE_NAME: usize = 8;

/// The longest reference [`decode_entity`] resolves: `#x10FFFF` and `#1114111`
/// are both 8 characters between the `&` and the `;`.
const MAX_ENTITY_BODY: usize = 8;

/// The text of an HTML fragment, with its markup removed.
///
/// Elastic's `html_strip` processor runs Lucene's `HTMLStripCharFilter`, and
/// that filter does NOT drop a tag silently: it emits a **newline** where a
/// block-level element opened or closed, and nothing where an inline one did.
/// Running the sentences together is the difference between
/// `... 22.04\n\n\n\nVulnerable software ...` and one unreadable line, and it
/// fails the field on every event that carries markup.
/// [`INLINE_ELEMENTS`] holds the list Lucene draws that line with.
///
/// Trimming is NOT part of this. `rapid7_insightvm` puts a `trim` processor
/// after each `html_strip` precisely because the leading `<p><p>` leaves two
/// newlines behind; `doppler` has no trim and keeps what the markup implied.
///
/// Entity decoding runs AFTER the tag scan, and the order is the point:
/// Lucene resolves an entity inline, so a decoded `<` is text and never opens
/// a tag. rapid7's GRUB remediation depends on it -- `password &lt;password&gt;`
/// has to survive as `password <password>` rather than lose the word between
/// the brackets. Decoding first would strip it.
///
/// One behaviour here is Elastic's rather than Lucene's, and it is reproduced:
/// the processor hands back a value holding no `<` or no `>` untouched, before
/// Lucene ever sees it, so `a &lt; b` with no markup keeps its entity.
///
/// What this does NOT reproduce:
/// - Lucene's full HTML4 entity table, all 253 names. Only the five predefined
///   XML names and numeric character references are resolved, which is every
///   entity the corpus carries. The table is Apache-2.0 source data, so
///   vendoring it into a BUSL file is a licensing call, not a coding one.
/// - A `>` inside a quoted attribute value, which Lucene's longest-match
///   grammar keeps inside the tag and this scan closes the tag on.
/// - Lucene DROPS a trailing `<name` that never closes; this keeps it as text,
///   which loses nothing.
#[must_use]
pub fn html_strip(text: &str) -> String {
    // Elasticsearch's own shortcut, and it is behaviour rather than an
    // optimisation: with no tag to strip the value is returned verbatim,
    // entities included.
    if !text.contains('<') || !text.contains('>') {
        return text.to_string();
    }

    let mut out = String::with_capacity(text.len());
    let mut cursor = 0;

    while let Some(offset) = text[cursor..].find('<') {
        let open = cursor + offset;
        out.push_str(&text[cursor..open]);
        cursor = strip_one_construct(text, open, &mut out);
    }
    out.push_str(&text[cursor..]);

    if out.contains('&') {
        out = decode_entities(&out);
    }
    out
}

/// Consume the construct opening at the `<` at `open`, appending whatever
/// Lucene emits in its place, and answer the byte index to resume at.
///
/// The answer is always greater than `open`, so the caller's scan advances.
fn strip_one_construct(text: &str, open: usize, out: &mut String) -> usize {
    let after = open + 1;
    let rest = &text[after..];

    // A comment carries no element, so no newline. Lucene swallows the rest of
    // the input when the comment never closes.
    if let Some(body) = rest.strip_prefix("!--") {
        return body
            .find("-->")
            .map_or_else(|| text.len(), |at| after + 3 + at + 3);
    }

    // A CDATA section is text, delimiters aside.
    if let Some(body) = rest.strip_prefix("![CDATA[") {
        let Some(at) = body.find("]]>") else {
            out.push_str(body);
            return text.len();
        };
        out.push_str(&body[..at]);
        return after + 8 + at + 3;
    }

    // A doctype or other declaration, and a processing instruction: dropped,
    // and neither is an element, so neither earns a newline.
    if rest.starts_with(['!', '?']) {
        let Some(at) = rest.find('>') else {
            out.push_str(&text[open..]);
            return text.len();
        };
        return after + at + 1;
    }

    let closing = rest.starts_with('/');
    let name_at = skip_whitespace(text, after + usize::from(closing));
    let Some((name, name_end)) = read_element_name(text, name_at) else {
        // Not a tag at all. Lucene pushes the `<` back as ordinary text and
        // carries on scanning, which is what keeps `1 < 2 > 3` intact.
        out.push('<');
        return after;
    };

    // `script` and `style` hold raw text rather than markup. Lucene drops the
    // content and emits one newline at the closing tag, so a page's JavaScript
    // never lands in a keyword field.
    if !closing && (name.eq_ignore_ascii_case("script") || name.eq_ignore_ascii_case("style")) {
        let Some(end) = skip_raw_text_element(text, name, name_end) else {
            // Unclosed: Lucene drops everything from the `<` onwards.
            return text.len();
        };
        out.push('\n');
        return end;
    }

    let Some(at) = text[name_end..].find('>') else {
        out.push_str(&text[open..]);
        return text.len();
    };
    if !is_inline_element(name) {
        out.push('\n');
    }
    name_end + at + 1
}

/// Whether Lucene calls `name` an inline element, matched without regard to
/// case the way its character classes are written.
fn is_inline_element(name: &str) -> bool {
    if name.len() > MAX_INLINE_NAME || !name.is_ascii() {
        return false;
    }
    let mut folded = [0_u8; MAX_INLINE_NAME];
    folded[..name.len()].copy_from_slice(name.as_bytes());
    folded[..name.len()].make_ascii_lowercase();
    let key = std::str::from_utf8(&folded[..name.len()]).unwrap_or_default();
    INLINE_ELEMENTS.binary_search(&key).is_ok()
}

/// The byte index of the first non-whitespace character at or after `from`.
fn skip_whitespace(text: &str, from: usize) -> usize {
    text[from..]
        .char_indices()
        .find(|(_, ch)| !ch.is_whitespace())
        .map_or_else(|| text.len(), |(idx, _)| from + idx)
}

/// The element name starting at `from`, with the byte index just past it.
///
/// Lucene spells the name with XML's production,
/// `[:_\p{ID_Start}] [-.:_\p{ID_Continue}]*`. `char::is_alphabetic` stands in
/// for `ID_Start` and `is_alphanumeric` for `ID_Continue`; they part company
/// only on characters no vendor spells a tag with.
fn read_element_name(text: &str, from: usize) -> Option<(&str, usize)> {
    let rest = text.get(from..)?;
    let mut chars = rest.char_indices();
    let (_, first) = chars.next()?;
    if !(first.is_alphabetic() || first == '_' || first == ':') {
        return None;
    }
    let end = chars
        .find(|(_, ch)| !(ch.is_alphanumeric() || matches!(ch, '-' | '.' | ':' | '_')))
        .map_or_else(|| rest.len(), |(idx, _)| idx);
    Some((&rest[..end], from + end))
}

/// The byte index past the `</script>` or `</style>` closing the element whose
/// name ends at `name_end`, or `None` when the text never closes it.
fn skip_raw_text_element(text: &str, name: &str, name_end: usize) -> Option<usize> {
    let mut from = name_end + text[name_end..].find('>')? + 1;
    while let Some(offset) = text[from..].find("</") {
        let at = from + offset;
        let after = skip_whitespace(text, at + 2);
        if let Some((found, found_end)) = read_element_name(text, after)
            && found.eq_ignore_ascii_case(name)
            && let Some(shut) = text[found_end..].find('>')
        {
            return Some(found_end + shut + 1);
        }
        from = at + 2;
    }
    None
}

/// Resolve every character reference in `text` in one left-to-right pass.
///
/// One pass is also what makes `&amp;lt;` come back as the text `&lt;` rather
/// than as `<`: the `&amp;` is consumed and the scan resumes past it.
fn decode_entities(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut cursor = 0;

    while let Some(offset) = text[cursor..].find('&') {
        let amp = cursor + offset;
        out.push_str(&text[cursor..amp]);
        let body_at = amp + 1;
        // A reference longer than the window is not one, so the search for the
        // terminator is bounded rather than running to the end of the field.
        let window_end = text.len().min(body_at + MAX_ENTITY_BODY + 1);
        let resolved = text.as_bytes()[body_at..window_end]
            .iter()
            .position(|byte| *byte == b';')
            .and_then(|at| decode_entity(&text[body_at..body_at + at]).map(|ch| (ch, at)));

        if let Some((ch, at)) = resolved {
            out.push(ch);
            cursor = body_at + at + 1;
        } else {
            out.push('&');
            cursor = body_at;
        }
    }
    out.push_str(&text[cursor..]);
    out
}

/// The character a `&<body>;` reference stands for, or `None` when `body` is
/// not a reference Lucene resolves -- which leaves it as the text it was.
fn decode_entity(body: &str) -> Option<char> {
    let Some(numeric) = body.strip_prefix('#') else {
        return named_entity(body);
    };
    // Lucene bounds the digits at what a code point can hold: 6 hexadecimal or
    // 7 decimal. Anything else is text -- an empty reference, a sign, or more
    // digits than a code point has room for.
    let point = match numeric.strip_prefix(['x', 'X']) {
        Some(hex) if is_digit_run(hex, 6, u8::is_ascii_hexdigit) => {
            u32::from_str_radix(hex, 16).ok()?
        }
        None if is_digit_run(numeric, 7, u8::is_ascii_digit) => numeric.parse().ok()?,
        _ => return None,
    };
    // A lone surrogate is not a character, and Lucene substitutes U+FFFD for
    // it rather than dropping the reference.
    if (0xD800..=0xDFFF).contains(&point) {
        return Some(char::REPLACEMENT_CHARACTER);
    }
    char::from_u32(point)
}

/// Whether `digits` is a run of one to `max` characters that all pass `class`.
fn is_digit_run(digits: &str, max: usize, class: fn(&u8) -> bool) -> bool {
    (1..=max).contains(&digits.len()) && digits.bytes().all(|byte| class(&byte))
}

/// The five predefined entity names, plus the upper-case spellings Lucene's
/// `upperCaseVariantsAccepted` table admits. `apos` has no such variant there.
fn named_entity(name: &str) -> Option<char> {
    Some(match name {
        "lt" | "LT" => '<',
        "gt" | "GT" => '>',
        "quot" | "QUOT" => '"',
        "apos" => '\'',
        "amp" | "AMP" => '&',
        _ => return None,
    })
}

#[cfg(test)]
mod html_strip_tests {
    use super::html_strip;

    /// Every expectation below the sorted check came back from Elasticsearch
    /// 9.2.2 in the compat corpus at
    /// `rapid7_insightvm/asset_vulnerability/test-asset-vulnerability` and
    /// `doppler/activity/test-doppler-activity`, so they are the engine's
    /// answer rather than a restatement of the code above.
    #[test]
    fn the_inline_table_is_sorted() {
        // `binary_search` answers nonsense on an unsorted table, and a wrong
        // answer here is a missing or a spurious newline, not a crash.
        assert!(super::INLINE_ELEMENTS.is_sorted());
        assert!(
            super::INLINE_ELEMENTS
                .iter()
                .all(|name| name.len() <= super::MAX_INLINE_NAME)
        );
    }

    /// The four tags between rapid7's two sentences leave four newlines, and
    /// the two that open and the two that close the document leave the ones
    /// the pipeline's own `trim` processor then takes off.
    #[test]
    fn a_block_element_leaves_a_newline_where_it_opened_and_closed() {
        let raw = "<p><p>Vulnerable OS: Ubuntu Linux 22.04<p></p></p><p>Vulnerable software \
                   installed: Azul Systems JRE 17.54.22 \
                   (/root/infaagent/jdk/lib/jrt-fs.jar)</p></p>";
        let stripped = html_strip(raw);

        assert_eq!(
            stripped,
            "\n\nVulnerable OS: Ubuntu Linux 22.04\n\n\n\nVulnerable software installed: Azul \
             Systems JRE 17.54.22 (/root/infaagent/jdk/lib/jrt-fs.jar)\n\n"
        );
        // What the corpus holds, once the pipeline's `trim` has run.
        assert_eq!(
            stripped.trim(),
            "Vulnerable OS: Ubuntu Linux 22.04\n\n\n\nVulnerable software installed: Azul Systems \
             JRE 17.54.22 (/root/infaagent/jdk/lib/jrt-fs.jar)"
        );
    }

    /// doppler's description runs `<a>` and `<br>` together. The anchor is
    /// inline and contributes nothing; `br` is not in Lucene's inline list, so
    /// it breaks the line. The hrefs are shortened -- nothing else is.
    #[test]
    fn an_inline_anchor_and_a_line_break_are_told_apart() {
        let raw = "Modified secrets in <a class=\"text-purple-500 hover:underline\" rel=\"noopener\" \
                   href=\"https://x/y\">example-config-1</a> project with <a \
                   href=\"https://x/z\">3 added</a>:<br>\u{2022} EXAMPLE_SECRET_1";

        assert_eq!(
            html_strip(raw),
            "Modified secrets in example-config-1 project with 3 added:\n\u{2022} EXAMPLE_SECRET_1"
        );
    }

    /// rapid7's GRUB remediation is why the entities are resolved AFTER the
    /// tag scan and not before: decoding first would turn `&lt;password&gt;`
    /// into a tag and lose the word between the brackets.
    #[test]
    fn an_entity_is_decoded_once_the_tag_scan_has_finished() {
        assert_eq!(
            html_strip("<pre>   password &lt;password&gt;</pre>"),
            "\n   password <password>\n"
        );
    }

    /// Three of rapid7's events carry `&#39;`, which the five predefined names
    /// do not cover.
    #[test]
    fn a_numeric_character_reference_is_decoded_too() {
        let raw = "<p><ul><li>Running CIFS service</li><li>Configuration item smb2-enabled set to \
                   &#39;true&#39; matched</li></ul></p>";

        assert_eq!(
            html_strip(raw).trim(),
            "Running CIFS service\n\nConfiguration item smb2-enabled set to 'true' matched"
        );
        assert_eq!(html_strip("<p>&#x41;&#66;</p>"), "\nAB\n");
        // A lone surrogate is not a character; Lucene writes U+FFFD for it.
        assert_eq!(html_strip("<p>&#xD800;</p>"), "\n\u{FFFD}\n");
        // Past the last code point it is text, not a reference.
        assert_eq!(html_strip("<p>&#x110000;</p>"), "\n&#x110000;\n");
    }

    /// A single pass is what keeps an escaped ampersand escaped: `&amp;lt;` is
    /// the TEXT `&lt;`, not a second reference to resolve.
    #[test]
    fn an_escaped_ampersand_does_not_decode_twice() {
        assert_eq!(html_strip("<p>&amp;lt;</p>"), "\n&lt;\n");
    }

    /// Elasticsearch's own processor returns a value holding no `<` or no `>`
    /// untouched, entities and all, before Lucene ever sees it.
    #[test]
    fn a_value_with_no_markup_keeps_its_entities() {
        assert_eq!(html_strip("a &lt; b"), "a &lt; b");
        assert_eq!(html_strip("2 > 1"), "2 > 1");
    }

    /// `<` opens a tag only when a name follows it. Lucene pushes the bracket
    /// back as text otherwise, so an inequality survives.
    #[test]
    fn a_bare_angle_bracket_is_still_text() {
        assert_eq!(html_strip("1 < 2 > 3"), "1 < 2 > 3");
    }

    /// A comment, a declaration and a CDATA section are not elements, so none
    /// of them earns a newline -- and a `>` inside a comment does not end it.
    #[test]
    fn the_non_element_constructs_leave_no_newline() {
        assert_eq!(html_strip("<p>a<!-- a > b -->c</p>"), "\nac\n");
        assert_eq!(html_strip("<!doctype html><p>a</p>"), "\na\n");
        assert_eq!(
            html_strip("<p>a<![CDATA[raw > text]]>b</p>"),
            "\naraw > textb\n"
        );
    }

    /// `script` holds raw text rather than markup, so Lucene drops the body
    /// and emits one newline at the closing tag. Emitting the JavaScript as
    /// text is the failure this stops.
    #[test]
    fn a_script_body_never_reaches_the_output() {
        assert_eq!(
            html_strip("<p>a</p><script>if (1 > 0) { x(); }</script><p>b</p>"),
            "\na\n\n\nb\n"
        );
        assert_eq!(
            html_strip("<p>a</p><style>i > b { top: 0 }</style>"),
            "\na\n\n"
        );
    }
}

/// Whether an address falls in any of the ranges `network_direction` names.
///
/// Elastic takes either a CIDR or one of its own range NAMES, and a pipeline
/// that lists several means the union of them. `unicast` and `global_unicast`
/// are almost everything, so a list carrying one calls nearly every address
/// internal -- which is what opencanary and stormshield mean by it.
#[must_use]
pub fn ip_in_networks(ip: &str, networks: &[&str]) -> bool {
    use std::net::IpAddr;

    let Ok(address) = ip.parse::<IpAddr>() else {
        return false;
    };
    networks.iter().any(|network| match *network {
        "loopback" => address.is_loopback(),
        "unspecified" => address.is_unspecified(),
        "multicast" => address.is_multicast(),
        "private" => match address {
            IpAddr::V4(v4) => v4.is_private(),
            // The v6 unique-local block, which `Ipv6Addr::is_unique_local` is
            // still unstable for.
            IpAddr::V6(v6) => (v6.segments()[0] & 0xfe00) == 0xfc00,
        },
        "link_local_unicast" => match address {
            IpAddr::V4(v4) => v4.is_link_local(),
            IpAddr::V6(v6) => (v6.segments()[0] & 0xffc0) == 0xfe80,
        },
        "link_local_multicast" => match address {
            IpAddr::V4(v4) => v4.is_multicast() && v4.octets()[..3] == [224, 0, 0],
            IpAddr::V6(v6) => v6.is_multicast() && (v6.segments()[0] & 0x000f) == 2,
        },
        "interface_local_multicast" => match address {
            IpAddr::V4(_) => false,
            IpAddr::V6(v6) => v6.is_multicast() && (v6.segments()[0] & 0x000f) == 1,
        },
        // `unicast` is anything that is not multicast; `global_unicast` also
        // excludes the addresses that never leave the host or the link.
        "unicast" => !address.is_multicast(),
        "global_unicast" => {
            !address.is_multicast() && !address.is_loopback() && !address.is_unspecified()
        }
        "public" => !ip_in_networks(ip, &["private", "loopback", "link_local_unicast"]),
        cidr => cidr_contains(cidr, address),
    })
}

/// Whether `address` falls in a `<network>/<bits>` block.
fn cidr_contains(cidr: &str, address: std::net::IpAddr) -> bool {
    use std::net::IpAddr;

    let Some((network, bits)) = cidr.split_once('/') else {
        return network_equals(cidr, address);
    };
    let Ok(bits) = bits.parse::<u32>() else {
        return false;
    };
    match (network.parse::<IpAddr>(), address) {
        (Ok(IpAddr::V4(network)), IpAddr::V4(address)) if bits <= 32 => {
            let mask = if bits == 0 {
                0
            } else {
                u32::MAX << (32 - bits)
            };
            u32::from(network) & mask == u32::from(address) & mask
        }
        (Ok(IpAddr::V6(network)), IpAddr::V6(address)) if bits <= 128 => {
            let mask = if bits == 0 {
                0
            } else {
                u128::MAX << (128 - bits)
            };
            u128::from(network) & mask == u128::from(address) & mask
        }
        _ => false,
    }
}

/// A bare address in the list, which Elastic reads as a single-host range.
fn network_equals(network: &str, address: std::net::IpAddr) -> bool {
    network.parse::<std::net::IpAddr>() == Ok(address)
}

/// Resolve a field PATH that carries mustache references, against the event.
///
/// A processor's `field` may name itself from the document -- `cisco_meraki`
/// renames `cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_mac`, so the
/// subtree it reads is whatever the event's subtype says. Emitting the template
/// verbatim gave a path nothing ever matched.
///
/// Returns `None` when a reference resolves to nothing, because the name it
/// would build has an empty segment and matches no field either.
#[must_use]
pub fn resolve_path(event: &Event, template: &str) -> Option<String> {
    if !template.contains("{{") {
        return Some(template.to_string());
    }

    let mut resolved = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(open) = rest.find("{{") {
        resolved.push_str(&rest[..open]);

        // Mustache spells an unescaped reference with three braces and an
        // escaped one with two; a field path wants the value either way.
        let after = &rest[open..];
        let inner = after.trim_start_matches('{');
        let close = "}".repeat(after.len() - inner.len());
        let (name, tail) = inner.split_once(&close)?;

        let value = event.get_as_string(name.trim())?;
        if value.is_empty() {
            return None;
        }
        resolved.push_str(&value);
        rest = tail;
    }
    resolved.push_str(rest);
    Some(resolved)
}

/// The pieces of a URI reference, borrowed from the string they came from.
struct UriRef<'a> {
    scheme: Option<&'a str>,
    user_info: Option<&'a str>,
    host: Option<&'a str>,
    port: Option<&'a str>,
    path: &'a str,
    query: Option<&'a str>,
    fragment: Option<&'a str>,
}

/// Split a URI reference into its parts, per RFC 3986's generic syntax.
///
/// A relative reference is the case that matters and the one a URL crate will
/// not take: fortinet's `url` field is a bare path far more often than a whole
/// URL, and rejecting those loses `url.path` on every one of them.
fn split_uri(uri: &str) -> UriRef<'_> {
    let (rest, fragment) = match uri.split_once('#') {
        Some((before, after)) => (before, Some(after)),
        None => (uri, None),
    };
    let (rest, query) = match rest.split_once('?') {
        Some((before, after)) => (before, Some(after)),
        None => (rest, None),
    };

    // A scheme runs to the first `:`, but only when nothing before it could
    // make that colon part of a path or an authority instead.
    let (rest, scheme) = match rest.find(':') {
        Some(colon) if is_scheme(&rest[..colon]) => (&rest[colon + 1..], Some(&rest[..colon])),
        _ => (rest, None),
    };

    let Some(after_slashes) = rest.strip_prefix("//") else {
        return UriRef {
            scheme,
            user_info: None,
            host: None,
            port: None,
            path: rest,
            query,
            fragment,
        };
    };

    let end = after_slashes.find('/').unwrap_or(after_slashes.len());
    let (authority, path) = after_slashes.split_at(end);

    let (user_info, host_port) = match authority.rsplit_once('@') {
        Some((user, host)) => (Some(user), host),
        None => (None, authority),
    };

    // Only a colon after the closing bracket separates an IPv6 host from its
    // port -- the address is full of them.
    let colon = match host_port.rfind(']') {
        Some(bracket) => host_port[bracket..].find(':').map(|at| bracket + at),
        None => host_port.rfind(':'),
    };
    let (host, port) = match colon {
        Some(at) => (&host_port[..at], Some(&host_port[at + 1..])),
        None => (host_port, None),
    };

    UriRef {
        scheme,
        user_info,
        host: Some(host),
        port,
        path,
        query,
        fragment,
    }
}

/// Whether `candidate` is a scheme: `ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )`.
fn is_scheme(candidate: &str) -> bool {
    let mut chars = candidate.chars();
    chars.next().is_some_and(char::is_alphabetic)
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
}

/// Percent-decode a string the way `java.net.URI` does: `%XX` only, no `+`
/// handling, and `None` on a malformed escape or invalid UTF-8 -- the cases
/// where Java rejects the whole URI.
fn percent_decode_strict(text: &str) -> Option<String> {
    if !text.contains('%') {
        return None;
    }
    let bytes = text.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = text.get(i + 1..i + 3)?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// The extension of a path, or `None` when its last segment has no dot.
///
/// Elasticsearch takes the extension from the segment after the LAST `/` and
/// gives a path with no slash at all none -- `elastic/elasticsearch#105689`, and
/// the corpus agrees: `/api/v2/cmdb/log.fortianalyzer/setting` and the bare
/// `subdomain.domain.tld` both come back without one, `/virus/eicar.com` gives
/// `com`, and `/config/` gives none.
fn path_extension(path: &str) -> Option<&str> {
    let segment = &path[path.rfind('/')?..];
    let dot = segment.rfind('.')?;
    let extension = &segment[dot + 1..];
    (!extension.is_empty()).then_some(extension)
}

/// Write one `kv` pair, APPENDING where the field already holds something.
///
/// Elastic's processor asks `hasField` first and appends when it does, so a key
/// that repeats in the line collects a list. Overwriting kept only the last:
/// checkpoint logs several `match_id` and `rule_action` per connection and we
/// reported one of each.
///
/// # Errors
///
/// Propagates a failure to set or append.
pub fn kv_put(event: &mut Event, path: &str, value: impl AsRef<str>) -> Result<()> {
    let value = value.as_ref();
    if event.get(path).is_some() {
        event.append(path, value)
    } else {
        event.set(path, value)
    }
}

/// Elastic's string processors: `lowercase`, `uppercase` and `trim`.
///
/// Each is an `AbstractStringProcessor` there, and every one of them walks a
/// LIST element by element -- the same rule [`gsub_field`] needed.
///
/// # Errors
///
/// Propagates a failure to set the target.
pub fn map_strings(
    event: &mut Event,
    field: &str,
    target: &str,
    each: impl Fn(&str) -> String,
) -> Result<()> {
    let mapped = match event.get(field) {
        Some(Value::String(text)) => Value::String(each(text)),
        Some(Value::Array(items)) => Value::Array(
            items
                .iter()
                .map(|item| match item {
                    Value::String(text) => Value::String(each(text)),
                    other => other.clone(),
                })
                .collect(),
        ),
        _ => return Ok(()),
    };
    event.set(target, mapped)
}

/// Elastic's `gsub` processor: replace every match in `field` into `target`.
///
/// A LIST is rewritten element by element, which is what Elasticsearch's own
/// processor does and what a string-only reader skipped: mimecast APPENDS the
/// sender address, so the field it then scrubs `<>` out of is a one-element
/// list, and the empty sender survived into `email.from.address`.
///
/// # Errors
///
/// Propagates a failure to set the target.
pub fn gsub_field(
    event: &mut Event,
    field: &str,
    target: &str,
    pattern: &crate::grok_cache::Pattern,
    replacement: &str,
) -> Result<()> {
    let rewrite = |text: &str| Value::String(pattern.replace_all(text, replacement).into_owned());
    let replaced = match event.get(field) {
        Some(Value::String(text)) => rewrite(text),
        Some(Value::Array(items)) => Value::Array(
            items
                .iter()
                .map(|item| match item {
                    Value::String(text) => rewrite(text),
                    other => other.clone(),
                })
                .collect(),
        ),
        _ => return Ok(()),
    };
    event.set(target, replaced)
}

/// Whether `java.net.URI` accepts every character of `text`.
///
/// Its legal set is RFC 2396's unreserved, reserved and escaped, widened to any
/// non-ASCII character that is neither a control nor a space. So a raw space, a
/// control character, one of the excluded ASCII punctuation marks, or a `%` not
/// followed by two hex digits throws `URISyntaxException` -- which is where
/// Elasticsearch drops to the `java.net.URL` fallback below.
fn java_uri_legal(text: &str) -> bool {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if !b.is_ascii() {
            // A multi-byte character: only the leading byte is inspected, and
            // every non-ASCII scalar but a control or space char is legal.
            i += 1;
            continue;
        }
        if b == b'%' {
            let hex = bytes.get(i + 1..i + 3);
            if !hex.is_some_and(|h| h.iter().all(u8::is_ascii_hexdigit)) {
                return false;
            }
            i += 3;
            continue;
        }
        if !(b.is_ascii_alphanumeric() || b"-_.!~*'();/?:@&=+$,[]#".contains(&b)) {
            return false;
        }
        i += 1;
    }
    text.chars().all(|c| !c.is_control() && !is_space_char(c))
}

/// Java's `Character.isSpaceChar`: the Unicode space separators.
fn is_space_char(c: char) -> bool {
    c == ' '
        || c == '\u{a0}'
        || c == '\u{1680}'
        || ('\u{2000}'..='\u{200a}').contains(&c)
        || matches!(
            c,
            '\u{2028}' | '\u{2029}' | '\u{202f}' | '\u{205f}' | '\u{3000}'
        )
}

/// Whether `java.net.URL` -- what Elasticsearch falls back to when
/// `java.net.URI` refuses the text -- would parse it.
///
/// It is far more forgiving about characters and far less about two things: it
/// needs a protocol the JDK ships a handler for, and it runs the port through a
/// bare `Integer.parseInt`, so a port like `80-` throws out of the constructor
/// and the whole processor fails. Both together are why an ALB log's
/// `http://host:80-/ ` yields NO `url.*` at all.
fn java_url_parses(uri: &UriRef<'_>) -> bool {
    let Some(scheme) = uri.scheme else {
        return false;
    };
    if !matches!(
        scheme.to_ascii_lowercase().as_str(),
        "http" | "https" | "file" | "ftp" | "jar" | "mailto"
    ) {
        return false;
    }
    uri.port
        .is_none_or(|port| port.is_empty() || port.bytes().all(|b| b.is_ascii_digit()))
}

/// Split `field` into ECS `url.*` components under `target`.
///
/// Elastic's `uri_parts` processor. Returns whether anything was written, so
/// the caller can run its `on_failure` block: a value that is not a string, an
/// empty one, or one neither `java.net.URI` nor `java.net.URL` will parse,
/// writes nothing.
///
/// # Errors
///
/// Returns [`crate::TransformError`] if a component cannot be set.
pub fn uri_parts(
    event: &mut Event,
    field: &str,
    target: &str,
    keep_original: bool,
    remove_if_successful: bool,
) -> Result<bool> {
    let Some(original) = event.get_string(field) else {
        return Ok(false);
    };
    if original.is_empty() {
        return Ok(false);
    }

    let uri = split_uri(&original);
    let uri_legal = java_uri_legal(&original);
    if !uri_legal && !java_url_parses(&uri) {
        return Ok(false);
    }
    let mut parts = serde_json::Map::new();

    if let Some(scheme) = uri.scheme {
        parts.insert("scheme".into(), Value::String(scheme.to_owned()));
    }
    if let Some(user_info) = uri.user_info {
        parts.insert("user_info".into(), Value::String(user_info.to_owned()));
        match user_info.split_once(':') {
            Some((username, password)) => {
                parts.insert("username".into(), Value::String(username.to_owned()));
                parts.insert("password".into(), Value::String(password.to_owned()));
            }
            None => {
                parts.insert("username".into(), Value::String(user_info.to_owned()));
            }
        }
    }
    if let Some(host) = uri.host.filter(|h| !h.is_empty()) {
        parts.insert("domain".into(), Value::String(host.to_owned()));
    }
    // A port that is not a number is left out rather than stored as text: the
    // ECS field is numeric and Elastic's processor drops it the same way.
    if let Some(port) = uri.port.and_then(|p| p.parse::<u32>().ok()) {
        parts.insert("port".into(), Value::Number(port.into()));
    }
    if uri.path.is_empty() {
        // Elasticsearch's processor goes through java.net.URI, whose getPath
        // returns the empty string for a URL with an authority and no path --
        // and the processor writes it. Verbatim in the umbrella corpus.
        if uri.host.is_some() {
            parts.insert("path".into(), Value::String(String::new()));
        }
    } else {
        parts.insert("path".into(), Value::String(uri.path.to_owned()));
        if let Some(extension) = path_extension(uri.path) {
            parts.insert("extension".into(), Value::String(extension.to_owned()));
        }
    }
    if let Some(query) = uri.query {
        // java.net.URI's getQuery DECODES percent escapes once, and that is
        // what Elasticsearch emits -- umbrella's double-encoded overwolf URLs
        // carry the proof. The URL fallback decodes nothing, so a text java.net
        // .URI refused keeps its query as written.
        let decoded = if uri_legal {
            percent_decode_strict(query).unwrap_or_else(|| query.to_owned())
        } else {
            query.to_owned()
        };
        parts.insert("query".into(), Value::String(decoded));
    }
    if let Some(fragment) = uri.fragment {
        parts.insert("fragment".into(), Value::String(fragment.to_owned()));
    }
    if keep_original {
        parts.insert("original".into(), Value::String(original.clone()));
    }

    // Elastic replaces the target wholesale with the parsed object, so a
    // scalar sitting there is gone before the parts land. Writing leaf by leaf
    // into a string instead fails on the FIRST leaf and loses every part with
    // it -- which is what happened to `url` on cisco_meraki's security events,
    // where the processor reads and writes the same field.
    if event.get(target).is_some_and(|v| !v.is_object()) {
        event.remove(target);
    }

    // Set leaf by leaf rather than replacing the target: the fortinet pipeline
    // writes `url.domain` from another field before and after this runs, and a
    // wholesale replace would discard it.
    for (key, value) in parts {
        event.set(&format!("{target}.{key}"), value)?;
    }

    if remove_if_successful && field != target {
        event.remove(field);
    }
    Ok(true)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use serde_json::json;

    // --- parse_json_field_to_root ---

    /// `add_to_root_conflict_strategy: merge` recurses only where BOTH sides
    /// are maps. An array meeting an array is not a conflict to refuse -- the
    /// incoming one wins, and the keys after it still have to be merged.
    #[test]
    fn a_root_merge_replaces_a_non_map_and_keeps_going() {
        let mut event = Event::new(json!({
            "tags": ["preserve_original_event"],
            "message": r#"{"tags":["Malware"],"ticura":{"indicator":{"uuid":"u-1"}}}"#,
        }));
        parse_json_field_to_root(&mut event, "message", true).expect("the merge succeeds");

        assert_eq!(event.get("tags"), Some(&json!(["Malware"])));
        // The key AFTER the conflicting one, which a mid-loop error lost.
        assert_eq!(event.get_str("ticura.indicator.uuid"), Some("u-1"));
    }

    /// Two maps at one key are the case that does recurse, so a key held on
    /// only one side survives from either.
    #[test]
    fn a_root_merge_recurses_where_both_sides_are_maps() {
        let mut event = Event::new(json!({
            "event": { "kind": "enrichment" },
            "message": r#"{"event":{"category":["threat"]}}"#,
        }));
        parse_json_field_to_root(&mut event, "message", true).expect("the merge succeeds");

        assert_eq!(event.get_str("event.kind"), Some("enrichment"));
        assert_eq!(event.get("event.category"), Some(&json!(["threat"])));
    }

    /// Without `merge` the strategy is `replace`, which is a whole-key putAll.
    #[test]
    fn a_root_replace_overwrites_the_whole_key() {
        let mut event = Event::new(json!({
            "event": { "kind": "enrichment" },
            "message": r#"{"event":{"category":["threat"]}}"#,
        }));
        parse_json_field_to_root(&mut event, "message", false).expect("the replace succeeds");

        assert_eq!(event.get_str("event.kind"), None);
        assert_eq!(event.get("event.category"), Some(&json!(["threat"])));
    }

    // --- fingerprint ---

    /// Every digest here came back from Elasticsearch 9.2.2 over
    /// `_ingest/pipeline/_simulate`, so the case list is the engine's answer
    /// and not a restatement of the code below it.
    #[test]
    fn the_fingerprint_matches_the_engine() {
        for (value, want) in [
            (
                json!({ "user": { "goog-gke-node": "" } }),
                "vP7l3YwKK9Mr+96HYpEj/Of6zVI=",
            ),
            (json!("abc"), "3TdC7BpNKltWOitirvf8Skb6bMo="),
            (json!(42), "/sRTDA6PwOI3ZlQA3Smv+XKzX/U="),
            (json!(42.5), "aeqOWcRKjs8fMmJx9bDV3Ii13O8="),
            (json!(true), "PylUZFNni4VZMcF0qX1sCJS49UY="),
            (json!(false), "msUh4y+OGUc7yRThr4rkI6bYwSI="),
            (
                json!(["a", "b", 1, 2.5, true]),
                "woDd6Sys2NjwMnAHPgls6Q9G5vw=",
            ),
            // Sorted keys, so a different insertion order is the same digest.
            (
                json!({ "z": 1, "a": 2, "m": 3 }),
                "iVtWh2nj2Y2tfu/JYNpbouUPATs=",
            ),
            (
                json!({ "a": { "b": { "c": "d" } } }),
                "hFYqlDQ7WMyVT3I/OBPd9TbFOik=",
            ),
            (json!({ "k": null }), "yW/bSvwAiTeESSxhjDcr21ekMsM="),
            (
                json!({ "a": null, "b": "c" }),
                "K3hlNi1Cczj7yiVlzt9Gjdx2hfA=",
            ),
            (json!(2_147_483_647), "u2sHUJ3ZGXJ0H+atgiAL4cKp9VI="),
            // Past an int, so eight bytes rather than four.
            (json!(2_147_483_648_i64), "7UBBR8UYA5FYijlBTjXWJjJ5tsw="),
            (json!(-2_147_483_648_i64), "+gVtPuvxhZqLcC0lINP4eZE86Nc="),
            (json!(1_234_567_890_123_i64), "rPjo3pVCgwPhGBtFWVE4OG1VNxE="),
            (
                json!({ "n": [{ "x": 1 }, { "y": "z" }] }),
                "71TTrLlh4rOvdr2I74vczknw9H4=",
            ),
            // A container writes no marker, so an empty one hashes as no bytes.
            (json!({}), "2jmj7l5rSw0yVb/vlWAYkK/YBwk="),
            (json!([]), "2jmj7l5rSw0yVb/vlWAYkK/YBwk="),
        ] {
            assert_eq!(
                fingerprint_default(std::slice::from_ref(&value)),
                want,
                "{value}"
            );
        }
    }

    /// Every method the catalogue asks for, over the same six payloads, and
    /// every digest read back off Elasticsearch 9.2.2.
    #[test]
    fn every_method_matches_the_engine() {
        let probes = [
            json!("abc"),
            json!(""),
            json!(42),
            json!({ "user": { "goog-gke-node": "" } }),
            json!(["a", "b", 1, 2.5, true]),
            json!("the quick brown fox jumps over the lazy dog, twice over"),
        ];
        for (method, wanted) in [
            (
                "SHA-1",
                [
                    "3TdC7BpNKltWOitirvf8Skb6bMo=",
                    "W6k8nbDP+T9StSHXQg5D9u2ieE8=",
                    "/sRTDA6PwOI3ZlQA3Smv+XKzX/U=",
                    "vP7l3YwKK9Mr+96HYpEj/Of6zVI=",
                    "woDd6Sys2NjwMnAHPgls6Q9G5vw=",
                    "6qWlPxLu0S4oZHkpS54dFw/JsrI=",
                ],
            ),
            (
                "SHA-256",
                [
                    "YJ9uNtJAVYUYjVz9dh9AfHzEan0/MUyIJwRp3eMV/NE=",
                    "bjQLnP+zepicpUTmu3gKLHiQHT+zNzh2hRGjBhevoB0=",
                    "8INMiB4uAAkzNgEyKXFeZ1EzBs5RBqnRqwEnMzUYU4A=",
                    "PFusDdBOVhwfy4tmRF8BhsAPYPiiNT/iUFXkyod+nUQ=",
                    "v6oE/OluPilqemIkARAcnDwLiGhjHTzLI1FFRRajc4U=",
                    "em2xIq3u1Py5SElyVe5rV/U5MeZ4nXWE+4dpKdcTVvo=",
                ],
            ),
            (
                "MurmurHash3",
                [
                    "fLPgJ47SvqixVKJpLWJP8Q==",
                    "RhCr5W7/XLVRYi2qePg1gw==",
                    "sRR5d4WUqD/w5xTps9osGA==",
                    "FjmoeN/lClaJjOJ9llCalg==",
                    "sDEqQ+MjnXsNTNY5jvWXeA==",
                    "TKbYelInHjaOI4MTotTy9w==",
                ],
            ),
        ] {
            for (value, want) in probes.iter().zip(wanted) {
                assert_eq!(
                    fingerprint_with(std::slice::from_ref(value), method, ""),
                    Ok(want.to_string()),
                    "{method} over {value}"
                );
            }
        }
    }

    /// The salt is prepended to the payload, whatever the method.
    #[test]
    fn a_salt_prefixes_the_payload() {
        assert_eq!(
            fingerprint_with(&[json!("abc")], "SHA-256", "pepper"),
            Ok("5mj1sW5EFQ22O56c7nfHHHJvLcf2YjtsmhatUq+Ntr0=".to_string())
        );
        assert_eq!(
            fingerprint_with(&[json!("abc")], "MurmurHash3", "pepper"),
            Ok("3g8ZwiWpayzFDS7mEm9BvQ==".to_string())
        );
    }

    /// A method with no implementation is named rather than silently wrong.
    #[test]
    fn an_unsupported_method_is_an_error() {
        assert!(fingerprint_with(&[json!("abc")], "MD5", "").is_err());
    }

    /// Two fields hash as one payload, so where they split does not vanish.
    #[test]
    fn two_fields_are_not_one_concatenated_string() {
        assert_eq!(
            fingerprint_default(&[json!("ab"), json!("c")]),
            "RJTTsHYwQbpBCX3TwVt+qKFyjSU="
        );
        assert_eq!(
            fingerprint_default(&[json!("a"), json!("bc")]),
            "fBBN16RIC/4tsZ7sbZHgFYHPJfM="
        );
        assert_ne!(
            fingerprint_default(&[json!("abc")]),
            fingerprint_default(&[json!("ab"), json!("c")])
        );
    }

    // --- condition_eq ---

    /// Two maps holding the same thing are still two wrappers, so a
    /// conditional reads them as different.
    #[test]
    fn two_equal_containers_are_not_equal_in_a_condition() {
        let same = json!({ "project_id": "p", "region": "r", "zone": "z" });
        assert!(!condition_eq(Some(&same), Some(&same.clone())));
        assert!(!condition_eq(Some(&json!([1, 2])), Some(&json!([1, 2]))));
    }

    /// Scalars are not wrapped, so they compare on value, and Painless reads
    /// an absent field and an explicit null as one.
    #[test]
    fn scalars_and_nulls_compare_on_value() {
        assert!(condition_eq(Some(&json!("a")), Some(&json!("a"))));
        assert!(!condition_eq(Some(&json!("a")), Some(&json!("b"))));
        assert!(condition_eq(None, Some(&Value::Null)));
        assert!(condition_eq(None, None));
        assert!(!condition_eq(Some(&json!("a")), None));
    }

    // --- dot_expander ---

    /// A dotted key becomes the nested path it spells, and the others are
    /// carried across untouched.
    #[test]
    fn a_dotted_key_expands_into_objects() {
        let mut event = Event::new(json!({
            "aws.cloudwatch.namespace": "AWS/EC2",
            "aws.ec2.metrics.CPUUtilization.avg": 42,
            "message": "left alone",
        }));
        dot_expand(&mut event, "", "*").unwrap();
        assert_eq!(event.get_str("aws.cloudwatch.namespace"), Some("AWS/EC2"));
        assert_eq!(
            event.get("aws.ec2.metrics.CPUUtilization.avg"),
            Some(&json!(42))
        );
        assert_eq!(event.get_str("message"), Some("left alone"));
        // Nested for real, not still a flat key.
        assert!(event.get("aws").is_some_and(|v| v.is_object()));
    }

    /// Naming ONE field expands that key and leaves the rest flat.
    #[test]
    fn a_named_field_expands_alone() {
        let mut event = Event::new(json!({"a.b": 1, "c.d": 2}));
        dot_expand(&mut event, "", "a.b").unwrap();
        assert_eq!(event.get("a.b"), Some(&json!(1)));
        assert!(event.get("a").is_some_and(|v| v.is_object()));
        assert!(event.as_value().get("c.d").is_some(), "c.d stays flat");
    }

    /// Expanding SIBLINGS one at a time must not lose the earlier one.
    ///
    /// A pipeline names one field per `dot_expander`, so the second call finds
    /// a nested container already there. zeek writes `id.orig_h` before
    /// `id.orig_p`, which put the nested `id` AFTER the still-dotted key --
    /// and rebuilding then overwrote what the expansion had just built.
    /// `source.address` disappeared on ~30 of its streams.
    #[test]
    fn expanding_siblings_in_turn_keeps_them_all() {
        let mut event = Event::new(json!({
            "id.orig_h": "192.168.86.167", "id.orig_p": 38339,
            "id.resp_h": "192.168.86.1", "id.resp_p": 53
        }));
        // The pipeline's own order: the ports and the responder expand around
        // the originator, which is the case that used to be lost.
        for field in ["id.orig_p", "id.orig_h", "id.resp_h", "id.resp_p"] {
            dot_expand(&mut event, "", field).unwrap();
        }

        assert_eq!(event.get_str("id.orig_h"), Some("192.168.86.167"));
        assert_eq!(event.get("id.orig_p"), Some(&json!(38339)));
        assert_eq!(event.get_str("id.resp_h"), Some("192.168.86.1"));
        assert_eq!(event.get("id.resp_p"), Some(&json!(53)));
        // One nested container, not four flat keys left behind.
        assert!(event.as_value().get("id.orig_h").is_none());
    }

    /// A container with no dotted key is left exactly as it was, without the
    /// deep clone the rebuild would otherwise cost on every event.
    #[test]
    fn a_container_with_no_dotted_key_is_untouched() {
        let before = json!({"message": "plain", "event": {"action": "x"}});
        let mut event = Event::new(before.clone());
        dot_expand(&mut event, "", "*").unwrap();
        assert_eq!(event.as_value(), &before);
    }

    /// Scoped to a path, and a path that is not there is not an error.
    #[test]
    fn a_scoped_expansion_only_touches_its_own_object() {
        let mut event = Event::new(json!({"json": {"a.b": 1}, "top.level": 2}));
        dot_expand(&mut event, "json", "*").unwrap();
        assert_eq!(event.get("json.a.b"), Some(&json!(1)));
        assert!(
            event.as_value().get("top.level").is_some(),
            "the root is out of scope"
        );

        dot_expand(&mut event, "absent", "*").unwrap();
    }

    // --- uri_parts: the java.net.URI behaviours the corpus pinned ---

    /// Verbatim from the umbrella corpus: the raw query is double-encoded
    /// and Elasticsearch emits it decoded ONCE.
    #[test]
    fn a_query_is_percent_decoded_once() {
        let mut event = Event::new(json!({ "src": "http://h/p?Extra=%255b%2522x%2522%255d" }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());
        assert_eq!(event.get("url.query"), Some(&json!("Extra=%5b%22x%22%5d")));
    }

    /// Verbatim from the cloudfront corpus: `.tld` is not on the Public Suffix
    /// List, and Elasticsearch writes nothing rather than inventing a
    /// registration under it.
    #[test]
    fn an_unlisted_top_level_domain_registers_nothing() {
        assert!(registered_domain_lookup("domain.tld").is_none());
        assert!(registered_domain_lookup("host.invalidtld").is_none());
        assert!(registered_domain_lookup("localhost").is_none());
    }

    /// A two-label PRIVATE entry lands on the same split whichever half of the
    /// list is read, so it stays on the ICANN path: the corpus has
    /// Elasticsearch registering `akamaized.net` under `net`.
    #[test]
    fn a_private_suffix_keeps_the_two_label_split() {
        let result = registered_domain_lookup("static-s-msn-com.akamaized.net").expect("known");
        assert_eq!(result.registered_domain.as_deref(), Some("akamaized.net"));
        assert_eq!(result.top_level_domain, "net");
        assert_eq!(result.subdomain.as_deref(), Some("static-s-msn-com"));
    }

    /// Verbatim from the `microsoft_dnsserver` audit corpus: a DNS query name
    /// arrives uppercase. The list is lowercase and the lookup ignores case,
    /// but the SUBDOMAIN is cut from the name as it arrived -- so the
    /// registered domain, which came off the list, does not end the original
    /// text and no subdomain is written at all.
    #[test]
    fn an_uppercase_name_registers_without_a_subdomain() {
        let result = registered_domain_lookup("B.ROOT-SERVERS.NET").expect("the suffix is listed");
        assert_eq!(
            result.registered_domain.as_deref(),
            Some("root-servers.net")
        );
        assert_eq!(result.top_level_domain, "net");
        assert_eq!(result.subdomain, None);

        // The same name in lowercase does get one.
        let lower = registered_domain_lookup("b.root-servers.net").expect("the suffix is listed");
        assert_eq!(lower.registered_domain.as_deref(), Some("root-servers.net"));
        assert_eq!(lower.subdomain.as_deref(), Some("b"));
    }

    /// A PRIVATE entry of three labels or more IS the registered domain, with
    /// its own first label peeled off as the top-level domain. Both cases are
    /// verbatim from the route53 corpus.
    #[test]
    fn a_long_private_suffix_is_the_registered_domain() {
        let short = registered_domain_lookup("ec2-instance-connect.us-east-1.amazonaws.com")
            .expect("known");
        assert_eq!(
            short.registered_domain.as_deref(),
            Some("us-east-1.amazonaws.com")
        );
        assert_eq!(short.top_level_domain, "amazonaws.com");
        assert_eq!(short.subdomain.as_deref(), Some("ec2-instance-connect"));

        let long = registered_domain_lookup(
            "amazonlinux-2-repos-us-east-1.s3.dualstack.us-east-1.amazonaws.com",
        )
        .expect("known");
        assert_eq!(
            long.registered_domain.as_deref(),
            Some("s3.dualstack.us-east-1.amazonaws.com")
        );
        assert_eq!(long.top_level_domain, "dualstack.us-east-1.amazonaws.com");
        assert_eq!(
            long.subdomain.as_deref(),
            Some("amazonlinux-2-repos-us-east-1")
        );
    }

    /// An ICANN suffix IS honoured whole -- reverse DNS is the case that needs
    /// it, and `co.uk` is the same pattern.
    #[test]
    fn an_icann_suffix_is_honoured_whole() {
        let reverse = registered_domain_lookup("211.52.31.172.in-addr.arpa").expect("known");
        assert_eq!(
            reverse.registered_domain.as_deref(),
            Some("172.in-addr.arpa")
        );
        assert_eq!(reverse.top_level_domain, "in-addr.arpa");
        assert_eq!(reverse.subdomain.as_deref(), Some("211.52.31"));

        let uk = registered_domain_lookup("a.example.co.uk").expect("known");
        assert_eq!(uk.registered_domain.as_deref(), Some("example.co.uk"));
        assert_eq!(uk.top_level_domain, "co.uk");
        assert_eq!(uk.subdomain.as_deref(), Some("a"));
    }

    #[test]
    fn a_two_label_name_has_no_subdomain() {
        let result = registered_domain_lookup("example.com").expect("a known suffix");
        assert_eq!(result.registered_domain.as_deref(), Some("example.com"));
        assert_eq!(result.top_level_domain, "com");
        assert_eq!(result.subdomain, None);
    }

    /// A malformed escape fails java.net.URI wholesale, so the raw text is
    /// what the java.net.URL fallback -- which decodes nothing -- keeps.
    #[test]
    fn a_query_that_will_not_decode_is_kept_raw() {
        let mut event = Event::new(json!({ "src": "http://h/p?bad=%ZZ" }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());
        assert_eq!(event.get("url.query"), Some(&json!("bad=%ZZ")));
    }

    /// Verbatim from the elb corpus: a space fails java.net.URI and the `80-`
    /// port fails the java.net.URL fallback's `Integer.parseInt`, so
    /// Elasticsearch writes NO url parts at all.
    #[test]
    fn a_url_neither_java_parser_takes_writes_nothing() {
        let mut event =
            Event::new(json!({ "src": "http://internal-service-alb.example.com:80-/ " }));
        assert!(!uri_parts(&mut event, "src", "url", true, false).unwrap());
        assert!(event.get("url.scheme").is_none());
        assert!(event.get("url.original").is_none());

        // The space alone still goes through the URL fallback.
        let mut spaced = Event::new(json!({ "src": "http://h/a b" }));
        assert!(uri_parts(&mut spaced, "src", "url", false, false).unwrap());
        assert_eq!(spaced.get("url.path"), Some(&json!("/a b")));
    }

    /// A URL with an authority and no path gets `path: ""`, which is what
    /// java.net.URI's getPath returns and the processor writes.
    #[test]
    fn a_pathless_url_with_a_host_writes_an_empty_path() {
        let mut event = Event::new(json!({ "src": "http://example.com?q=1" }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());
        assert_eq!(event.get("url.path"), Some(&json!("")));

        // A pure relative reference with no path still writes nothing.
        let mut event = Event::new(json!({ "src": "?q=1" }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());
        assert_eq!(event.get("url.path"), None);
    }

    // --- foreach_array ---

    /// The happy path: every element moves through `_ingest._value`, the
    /// mutated list lands back on the field, and `_ingest` is gone.
    #[test]
    fn foreach_array_rebuilds_the_field_from_the_mutated_elements() {
        let mut event = Event::new(json!({ "tags": ["a", "b"] }));
        foreach_array(&mut event, "tags", |event| {
            let text = event.get_str("_ingest._value").unwrap().to_uppercase();
            event.set("_ingest._value", text)
        })
        .unwrap();
        assert_eq!(event.as_value(), &json!({ "tags": ["A", "B"] }));
    }

    /// A MAP is a foreach too: the key and the value are both exposed, and the
    /// map is rebuilt from what the body leaves in them. mimecast's attachment
    /// hashes arrive that way, and skipping the map lost every one.
    #[test]
    fn foreach_array_walks_a_map_by_key_and_value() {
        let mut event = Event::new(json!({ "hash": { "sha1": "aa", "md5": "bb" } }));
        let mut seen: Vec<String> = Vec::new();
        foreach_array(&mut event, "hash", |event| {
            seen.push(event.get_str("_ingest._key").unwrap().to_string());
            let text = event.get_str("_ingest._value").unwrap().to_uppercase();
            event.set("_ingest._value", text)
        })
        .unwrap();

        assert_eq!(
            event.as_value(),
            &json!({ "hash": { "sha1": "AA", "md5": "BB" } })
        );
        assert_eq!(seen, ["sha1", "md5"]);
    }

    /// A value that is not an array is a no-op, exactly as the inline
    /// `if let Some(Value::Array(..))` was -- and it is NOT removed.
    #[test]
    fn foreach_array_leaves_a_non_array_alone() {
        let mut event = Event::new(json!({ "tags": "scalar" }));
        foreach_array(&mut event, "tags", |_| {
            panic!("the body must not run");
        })
        .unwrap();
        assert_eq!(event.as_value(), &json!({ "tags": "scalar" }));
    }

    /// A body that fails part-way leaves the processed prefix, the failing
    /// element -- still exposed at `_ingest._value` -- and the untouched
    /// rest, all back in the field.
    #[test]
    fn foreach_array_restores_the_field_when_the_body_fails() {
        let mut event = Event::new(json!({ "tags": ["a", "bad", "c"] }));
        let error = foreach_array(&mut event, "tags", |event| {
            let text = event.get_str("_ingest._value").unwrap().to_string();
            if text == "bad" {
                return Err(crate::TransformError::FieldNotFound { path: text });
            }
            event.set("_ingest._value", text.to_uppercase())
        })
        .unwrap_err();
        assert!(error.to_string().contains("bad"));
        assert_eq!(event.get("tags"), Some(&json!(["A", "bad", "c"])));
        assert_eq!(event.get("_ingest._value"), Some(&json!("bad")));
    }

    /// An empty array still clears `_ingest` and writes the field back,
    /// which is what the inline loop did.
    #[test]
    fn foreach_array_handles_an_empty_array() {
        let mut event = Event::new(json!({ "tags": [], "_ingest": { "old": 1 } }));
        foreach_array(&mut event, "tags", |_| Ok(())).unwrap();
        assert_eq!(event.as_value(), &json!({ "tags": [] }));
    }

    // --- parse_json_field ---

    /// The SIMD path and `serde_json` agree on what a document MEANS --
    /// nesting, numbers at both integer extremes, floats, escapes, unicode.
    #[test]
    fn simd_parse_agrees_with_serde_json() {
        for text in [
            r#"{"a": {"b": [1, 2.5, -3, 18446744073709551615, -9223372036854775808]}}"#,
            r#"{"s": "line\nbreak \"quoted\" é", "t": true, "n": null}"#,
            r#"[{"k": "v"}, [], {}, ""]"#,
            "42",
            r#""bare string""#,
        ] {
            let via_simd = parse_json_str(text).unwrap();
            let via_serde: Value = serde_json::from_str(text).unwrap();
            assert_eq!(via_simd, via_serde, "input: {text}");
        }
    }

    /// An absent field is a no-op, which is what the old inline `if let` did.
    #[test]
    fn parse_json_field_skips_an_absent_field() {
        let mut event = Event::new(json!({ "other": 1 }));
        parse_json_field(&mut event, "message", "json").unwrap();
        assert_eq!(event.as_value(), &json!({ "other": 1 }));
    }

    /// A field that will not parse is the processor's failure, naming the
    /// field and keeping the wording the generated `on_failure` blocks wrote.
    #[test]
    fn parse_json_field_fails_with_the_inline_blocks_wording() {
        let mut event = Event::new(json!({ "message": "{not json" }));
        let err = parse_json_field(&mut event, "message", "json").unwrap_err();
        let text = err.to_string();
        assert!(text.contains("message"), "{text}");
        assert!(text.contains("failed to parse JSON:"), "{text}");
    }

    /// The parsed object lands whole on the target.
    #[test]
    fn parse_json_field_writes_the_target() {
        let mut event = Event::new(json!({ "message": r#"{"a": [1, 2]}"# }));
        parse_json_field(&mut event, "message", "json").unwrap();
        assert_eq!(event.get("json"), Some(&json!({ "a": [1, 2] })));
    }

    /// A container is the processor's FAILURE, not an absent field.
    ///
    /// Elasticsearch coerces with `Object::toString` and Java spells a map
    /// `{format=simple, content=...}`, which Jackson will not parse. Reading
    /// the field with `get_string` answered `None` here and took the absent
    /// branch, so the processor silently succeeded on data it never parsed and
    /// `on_failure` never ran. `atlassian_cloud`'s `message` is exactly this.
    #[test]
    fn parse_json_field_declines_a_container() {
        for value in [json!({ "format": "simple" }), json!(["a", "b"])] {
            let mut event = Event::new(json!({ "message": value.clone() }));
            let err = parse_json_field(&mut event, "message", "json").unwrap_err();
            let text = err.to_string();
            assert!(text.contains("message"), "{value}: {text}");
            assert!(text.contains("not JSON text"), "{value}: {text}");
            // Nothing was written, and the field it could not read is intact.
            assert_eq!(event.get("json"), None, "{value}");
            assert_eq!(event.get("message"), Some(&value));
        }
    }

    /// A number, a boolean and null render as their own JSON spelling, so
    /// Elasticsearch's stringify-then-parse hands the same value back. We pass
    /// it through, which is that round trip without the allocation.
    #[test]
    fn parse_json_field_passes_a_scalar_through() {
        for value in [json!(42), json!(-1.5), json!(true), json!(null)] {
            let mut event = Event::new(json!({ "message": value.clone() }));
            parse_json_field(&mut event, "message", "json").unwrap();
            assert_eq!(event.get("json"), Some(&value));
        }
    }

    /// `add_to_root` reads the field the same way, so a container declines
    /// there too rather than merging an object it never parsed.
    #[test]
    fn parse_json_field_to_root_declines_a_container() {
        let mut event = Event::new(json!({ "message": { "a": 1 } }));
        let err = parse_json_field_to_root(&mut event, "message", false).unwrap_err();
        assert!(err.to_string().contains("not JSON text"), "{err}");
        assert_eq!(event.get("a"), None);
    }

    /// A scalar parses to something that is not an object, which is the
    /// `add_to_root` failure Elasticsearch already raises.
    #[test]
    fn parse_json_field_to_root_declines_a_scalar() {
        let mut event = Event::new(json!({ "message": 42 }));
        let err = parse_json_field_to_root(&mut event, "message", false).unwrap_err();
        assert!(err.to_string().contains("non-object root"), "{err}");
    }

    /// An absent field stays a no-op on the root path as well -- the branch a
    /// container used to share with it.
    #[test]
    fn parse_json_field_to_root_skips_an_absent_field() {
        let mut event = Event::new(json!({ "other": 1 }));
        parse_json_field_to_root(&mut event, "message", false).unwrap();
        assert_eq!(event.as_value(), &json!({ "other": 1 }));
    }

    /// Verbatim from `proofpoint_on_demand/message`, which converts a LIST of
    /// suborg recipients to string. Rendering the array whole gave the string
    /// `["0"]`, which the pipeline's own sentinel pass does not recognise.
    #[test]
    fn convert_walks_an_array_element_by_element() {
        assert_eq!(
            convert_value(&json!([0, 1]), "string").unwrap(),
            json!(["0", "1"])
        );
        assert_eq!(
            convert_value(&json!(["7", "8"]), "long").unwrap(),
            json!([7, 8])
        );
    }

    /// One bad member fails the whole conversion, which is where Elastic
    /// throws and the processor's `on_failure` runs.
    #[test]
    fn one_unconvertible_member_fails_the_array() {
        assert!(convert_value(&json!(["7", "not a number"]), "long").is_err());
    }

    /// A CONTAINER converts through Java's `String.valueOf`, so it renders
    /// `{k=v, k=v}` with the members in Java's hash order -- not its JSON.
    ///
    /// Verbatim from `qualys_was/vulnerability/test-verbose-findings`, whose
    /// `result_list_text` Elasticsearch wrote with `offset` ahead of `length`
    /// though the document carries them the other way round.
    #[test]
    fn a_container_converts_to_java_text_and_not_to_json() {
        assert_eq!(
            convert_value(
                &json!([{ "payloadResponse": { "length": 25, "offset": 271 } }]),
                "string"
            )
            .unwrap(),
            json!(["{payloadResponse={offset=271, length=25}}"])
        );
        assert_eq!(
            convert_value(&json!({ "a": ["x", 1], "b": null }), "string").unwrap(),
            json!("{a=[x, 1], b=null}")
        );
    }

    /// Verbatim from the compat corpus: `m365_defender/event/test-device`
    /// carries both the fingerprint's input and the digest Elasticsearch
    /// wrote, so this pins the encoding rather than describing it.
    #[test]
    fn the_default_fingerprint_is_base64_sha1_over_nul_delimited_values() {
        let value = json!("4248|2022-11-07T17:07:41.698Z|de6509d550e605faf3bbeac0905ab9590fe12345");
        assert_eq!(
            fingerprint_default(std::slice::from_ref(&value)),
            "utLjuzbrOqM8u+fh65n5nL10vuE="
        );
    }

    /// A string is hashed as its own text, quotes excluded -- and a NUMBER is
    /// hashed as its bytes, so the two never collide however alike they read.
    #[test]
    fn a_string_and_the_number_it_spells_differ() {
        assert_ne!(
            fingerprint_default(&[json!("4248")]),
            fingerprint_default(&[json!(4248)]),
        );
        assert_eq!(
            fingerprint_default(&[json!("42")]),
            "mV7xHXro5tXMlz9d1arNzS3QkcQ="
        );
        assert_eq!(
            fingerprint_default(&[json!(42)]),
            "/sRTDA6PwOI3ZlQA3Smv+XKzX/U="
        );
    }

    /// Verbatim from `tests/fixtures/cisco/umbrella`: a space before the quote
    /// of a field that itself holds commas.
    #[test]
    fn a_gap_before_a_quote_closes_so_the_field_reads_as_quoted() {
        let line = r#""2015-01-16 17:48:41","AD", "AD,ADSite,Network", "10.10.1.100""#;
        let closed = csv_close_quote_gap(line, ',', '"');
        assert_eq!(
            closed,
            r#""2015-01-16 17:48:41","AD","AD,ADSite,Network","10.10.1.100""#
        );
    }

    /// Whitespace that is part of a value is content, not a gap.
    #[test]
    fn whitespace_inside_a_field_is_left_alone() {
        for line in [
            r#""a"," spaced value ","b""#,
            "a, spaced value ,b",
            r#""a", b c,"d""#,
        ] {
            assert_eq!(csv_close_quote_gap(line, ',', '"'), line, "{line}");
        }
    }

    /// A line with nothing to close is returned borrowed.
    #[test]
    fn a_line_with_no_gap_is_not_reallocated() {
        let line = r#""a","b","c""#;
        assert!(matches!(
            csv_close_quote_gap(line, ',', '"'),
            std::borrow::Cow::Borrowed(_)
        ));
    }

    /// A delimiter inside a quoted field does not open a new one.
    #[test]
    fn a_quoted_delimiter_does_not_start_a_field() {
        let line = r#""a,  b", "c""#;
        assert_eq!(csv_close_quote_gap(line, ',', '"'), r#""a,  b","c""#);
    }

    // --- resolve_path ---

    // --- duplicate grok capture names ---

    /// Verbatim from `pipelines/cisco/ios/default.yml`. Java takes one field
    /// name in several alternation branches; Rust's engine rejects a duplicate
    /// group name outright, so the whole pattern failed to compile and the
    /// `BADAUTH` grok matched nothing on eighteen events.
    #[test]
    fn a_field_named_twice_compiles_and_both_branches_write_it() {
        let pattern = r"from %{DATA:source.address}(\(%{INT:source.port}\)|\:%{INT:source.port})";
        let (expanded, field_map, _) = grok_to_regex_typed(pattern);

        let re = regex::Regex::new(&expanded).expect("a repeated field name still compiles");
        assert_eq!(
            field_map.values().filter(|p| *p == "source.port").count(),
            2,
            "both groups must point at the same field"
        );

        for input in ["from 192.168.0.1(64999)", "from 192.168.0.1:64999"] {
            let caps = re.captures(input).unwrap_or_else(|| panic!("{input}"));
            let port = field_map
                .iter()
                .filter(|(_, path)| *path == "source.port")
                .find_map(|(group, _)| caps.name(group))
                .unwrap_or_else(|| panic!("{input}"));
            assert_eq!(port.as_str(), "64999", "{input}");
        }
    }

    /// Verbatim from `pipelines/cisco/meraki/events.yml`, where the subtree a
    /// rename reads is named by the event's own subtype.
    #[test]
    fn resolve_path_names_a_field_from_the_document() {
        let event = Event::new(json!({
            "cisco_meraki": { "event_subtype": "disassociation" },
        }));

        assert_eq!(
            resolve_path(
                &event,
                "cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_mac"
            )
            .as_deref(),
            Some("cisco_meraki.disassociation.client_mac")
        );
    }

    /// A reference that resolves to nothing would build a path with an empty
    /// segment, which matches no field -- so it is no path at all.
    #[test]
    fn resolve_path_refuses_an_unresolvable_reference() {
        let event = Event::new(json!({ "cisco_meraki": { "event_subtype": "" } }));
        assert_eq!(
            resolve_path(
                &event,
                "cisco_meraki.{{{cisco_meraki.event_subtype}}}.client_mac"
            ),
            None
        );

        let empty = Event::new(json!({}));
        assert_eq!(resolve_path(&empty, "a.{{{missing}}}.b"), None);
    }

    /// A path with no reference in it is itself, and costs nothing to ask for.
    #[test]
    fn resolve_path_passes_a_plain_path_through() {
        let event = Event::new(json!({}));
        assert_eq!(
            resolve_path(&event, "client.mac").as_deref(),
            Some("client.mac")
        );
    }

    /// Mustache spells an escaped reference with two braces and an unescaped
    /// one with three; a field path wants the value either way.
    #[test]
    fn resolve_path_reads_both_brace_forms() {
        let event = Event::new(json!({ "kind": "assoc" }));
        assert_eq!(
            resolve_path(&event, "a.{{kind}}.b").as_deref(),
            Some("a.assoc.b")
        );
        assert_eq!(
            resolve_path(&event, "a.{{{kind}}}.b").as_deref(),
            Some("a.assoc.b")
        );
    }

    // --- uri_parts ---

    /// Every one of these is a real fortinet input paired with what
    /// Elasticsearch 9.2.2 produced for it, taken from `testdata/compat`.
    #[test]
    fn uri_parts_matches_the_captured_elasticsearch_output() {
        let cases: &[(&str, Value)] = &[
            ("/config/", json!({ "path": "/config/" })),
            ("/", json!({ "path": "/" })),
            (
                "http://172.16.200.55/virus/eicar.com",
                json!({
                    "scheme": "http",
                    "domain": "172.16.200.55",
                    "path": "/virus/eicar.com",
                    "extension": "com",
                }),
            ),
            (
                "/ips/sig1.pdf",
                json!({ "path": "/ips/sig1.pdf", "extension": "pdf" }),
            ),
            (
                "/api/v2/monitor/system/usb-log?vdom=root",
                json!({ "path": "/api/v2/monitor/system/usb-log", "query": "vdom=root" }),
            ),
            // The dotted DIRECTORY must not become an extension.
            (
                "/api/v2/cmdb/log.fortianalyzer/setting?vdom=root",
                json!({ "path": "/api/v2/cmdb/log.fortianalyzer/setting", "query": "vdom=root" }),
            ),
            // Neither must a bare hostname, which java.net.URI reads as a
            // relative PATH. m365_defender and zscaler both ship these.
            (
                "subdomain.domain.tld",
                json!({ "path": "subdomain.domain.tld" }),
            ),
            ("url.com", json!({ "path": "url.com" })),
            (
                "https://172.16.200.88/dlp/files/fortiauto.pdf",
                json!({
                    "scheme": "https",
                    "domain": "172.16.200.88",
                    "path": "/dlp/files/fortiauto.pdf",
                    "extension": "pdf",
                }),
            ),
        ];

        for (input, expected) in cases {
            let mut event = Event::new(json!({ "src": input }));
            assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());
            assert_eq!(event.get("url"), Some(expected), "input: {input}");
        }
    }

    #[test]
    fn uri_parts_splits_an_authority_with_credentials_and_a_port() {
        let mut event =
            Event::new(json!({ "src": "https://bob:hunter2@example.com:8443/a?b=1#c" }));
        assert!(uri_parts(&mut event, "src", "url", true, false).unwrap());

        assert_eq!(event.get_str("url.scheme"), Some("https"));
        assert_eq!(event.get_str("url.domain"), Some("example.com"));
        assert_eq!(event.get("url.port"), Some(&json!(8443)));
        assert_eq!(event.get_str("url.user_info"), Some("bob:hunter2"));
        assert_eq!(event.get_str("url.username"), Some("bob"));
        assert_eq!(event.get_str("url.password"), Some("hunter2"));
        assert_eq!(event.get_str("url.path"), Some("/a"));
        assert_eq!(event.get_str("url.query"), Some("b=1"));
        assert_eq!(event.get_str("url.fragment"), Some("c"));
        assert_eq!(
            event.get_str("url.original"),
            Some("https://bob:hunter2@example.com:8443/a?b=1#c")
        );
    }

    /// An IPv6 authority is full of colons, so only the one after the closing
    /// bracket separates the port.
    #[test]
    fn uri_parts_keeps_an_ipv6_host_whole() {
        let mut event = Event::new(json!({ "src": "http://[2001:db8::1]:8080/x" }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());

        assert_eq!(event.get_str("url.domain"), Some("[2001:db8::1]"));
        assert_eq!(event.get("url.port"), Some(&json!(8080)));

        let mut event = Event::new(json!({ "src": "http://[2001:db8::1]/x" }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());
        assert_eq!(event.get_str("url.domain"), Some("[2001:db8::1]"));
        assert_eq!(event.get("url.port"), None);
    }

    /// The caller runs its `on_failure` on a false return, so a value that is
    /// not a usable string must say so rather than writing an empty subtree.
    #[test]
    fn uri_parts_reports_what_it_could_not_use() {
        let mut event = Event::new(json!({ "src": "", "num": 7 }));
        assert!(!uri_parts(&mut event, "src", "url", false, false).unwrap());
        assert!(!uri_parts(&mut event, "num", "url", false, false).unwrap());
        assert!(!uri_parts(&mut event, "absent", "url", false, false).unwrap());
        assert!(!event.has("url"));
    }

    /// The fortinet pipeline writes `url.domain` from the hostname field and
    /// then parses a path-only `url` over the top, so the parse must add to the
    /// subtree rather than replace it.
    #[test]
    fn uri_parts_adds_to_the_target_rather_than_replacing_it() {
        let mut event = Event::new(json!({ "src": "/config/", "url": { "domain": "elastic.co" } }));
        assert!(uri_parts(&mut event, "src", "url", false, false).unwrap());

        assert_eq!(event.get_str("url.domain"), Some("elastic.co"));
        assert_eq!(event.get_str("url.path"), Some("/config/"));
    }

    /// `cisco_meraki`'s security events parse `url` INTO `url`. A string sitting
    /// on the target has to go first -- writing `url.scheme` into a string
    /// fails on that first leaf and every other part is lost with it, which is
    /// how the event ended up with a scalar `url` and no parts at all.
    #[test]
    fn uri_parts_replaces_a_scalar_sitting_on_the_target() {
        let mut event = Event::new(json!({ "url": "http://www.eicar.org/download/eicar.com.txt" }));
        assert!(uri_parts(&mut event, "url", "url", true, false).unwrap());

        assert_eq!(
            event.get_str("url.original"),
            Some("http://www.eicar.org/download/eicar.com.txt")
        );
        assert_eq!(event.get_str("url.scheme"), Some("http"));
        assert_eq!(event.get_str("url.domain"), Some("www.eicar.org"));
        assert_eq!(event.get_str("url.path"), Some("/download/eicar.com.txt"));
        assert_eq!(event.get_str("url.extension"), Some("txt"));
    }

    // --- is_internal_ip ---

    #[test]
    fn internal_ip_rfc1918_class_a() {
        assert!(is_internal_ip("10.0.0.1"));
        assert!(is_internal_ip("10.255.255.255"));
    }

    #[test]
    fn internal_ip_rfc1918_class_b() {
        assert!(is_internal_ip("172.16.0.1"));
        assert!(is_internal_ip("172.31.255.255"));
        assert!(!is_internal_ip("172.32.0.1"));
    }

    #[test]
    fn internal_ip_rfc1918_class_c() {
        assert!(is_internal_ip("192.168.0.1"));
        assert!(is_internal_ip("192.168.255.255"));
    }

    #[test]
    fn internal_ip_loopback() {
        assert!(is_internal_ip("127.0.0.1"));
        assert!(is_internal_ip("::1"));
    }

    #[test]
    fn internal_ip_link_local() {
        assert!(is_internal_ip("169.254.0.1"));
    }

    #[test]
    fn internal_ip_public() {
        assert!(!is_internal_ip("8.8.8.8"));
        assert!(!is_internal_ip("175.16.199.1"));
        assert!(!is_internal_ip("1.1.1.1"));
    }

    #[test]
    fn internal_ip_invalid() {
        assert!(!is_internal_ip("not-an-ip"));
        assert!(!is_internal_ip(""));
    }

    // --- registered_domain_lookup ---

    #[test]
    fn registered_domain_simple() {
        let r = registered_domain_lookup("www.example.com").unwrap();
        assert_eq!(r.registered_domain.as_deref(), Some("example.com"));
        assert_eq!(r.top_level_domain, "com");
        assert_eq!(r.subdomain.as_deref(), Some("www"));
    }

    #[test]
    fn registered_domain_empty() {
        assert!(registered_domain_lookup("").is_none());
    }

    // --- grok_to_regex ---

    #[test]
    fn grok_simple_ip_field() {
        let regex = grok_to_regex("%{IP:source.ip}");
        assert!(regex.contains("(?P<source_ip>"));
    }

    #[test]
    fn grok_field_map_restores_dots() {
        let (_, map) = grok_to_regex_with_map("%{USER:user.name}");
        assert_eq!(map.get("user_name").unwrap(), "user.name");
    }

    #[test]
    fn grok_no_field() {
        let regex = grok_to_regex("%{NOTSPACE}");
        assert!(regex.contains(r"\S+"));
        assert!(!regex.contains("(?P<"));
    }

    #[test]
    fn grok_multiple_patterns() {
        let regex = grok_to_regex("%{IP:src}:%{POSINT:port}");
        assert!(regex.contains("(?P<src>"));
        assert!(regex.contains("(?P<port>"));
    }

    #[test]
    fn grok_unknown_pattern_fallback() {
        let regex = grok_to_regex("%{UNKNOWN_THING:field}");
        assert!(regex.contains(".+?")); // fallback
    }

    // --- parse_user_agent ---

    #[test]
    fn parse_ua_returns_ok() {
        let result = parse_user_agent("Mozilla/5.0 (Windows NT 10.0) Chrome/91.0");
        assert!(result.is_ok());
        let ua = result.unwrap();
        assert_eq!(ua.name.as_deref(), Some("Chrome"));
    }

    #[test]
    fn parse_ua_empty() {
        let result = parse_user_agent("");
        assert!(result.is_ok());
    }

    // `painless_exec`'s own tests live beside it in `dfe_painless::plan`, with
    // the counter lock they take.

    // --- community_id_v1 ---

    #[test]
    fn community_id_tcp() {
        let result = community_id_v1("1.2.3.4", "5.6.7.8", 1234, 80, "tcp");
        assert!(result.is_ok());
        let id = result.unwrap();
        assert!(id.starts_with("1:"));
    }

    #[test]
    fn community_id_invalid_ip() {
        let result = community_id_v1("not-an-ip", "5.6.7.8", 1234, 80, "tcp");
        assert!(result.is_err());
    }

    // --- geoip_lookup ---

    #[test]
    fn geoip_lookup_no_db() {
        // Reaches the process-global cache, so it takes the same lock the
        // enrichment tests do.
        let _guard = crate::enrichment::geoip_global::test_guard();
        // Without MMDB files loaded, should return empty map (not panic)
        let result = geoip_lookup("geoip_city", "8.8.8.8");
        assert!(result.is_ok());
    }

    // --- grok edge cases ---

    #[test]
    fn grok_type_suffix_stripped() {
        // Elastic grok uses %{IP:field:type} — the :type must be stripped
        let (regex, map) = grok_to_regex_with_map("%{IP:source.ip:ip}");
        assert!(regex.contains("(?P<source_ip>"));
        assert!(!regex.contains(":ip"));
        assert_eq!(map.get("source_ip").unwrap(), "source.ip");
    }

    #[test]
    fn grok_empty_pattern() {
        let regex = grok_to_regex("");
        assert_eq!(regex, "");
    }

    #[test]
    fn grok_literal_only() {
        let regex = grok_to_regex("hello world");
        assert_eq!(regex, "hello world");
    }

    #[test]
    fn grok_consecutive_patterns() {
        let regex = grok_to_regex("%{IP:src}%{POSINT:port}");
        assert!(regex.contains("(?P<src>"));
        assert!(regex.contains("(?P<port>"));
    }

    #[test]
    fn grok_long_type_suffix() {
        let (regex, map) = grok_to_regex_with_map("%{NUMBER:count:long}");
        assert!(regex.contains("(?P<count>"));
        assert!(!regex.contains(":long"));
        assert_eq!(map.get("count").unwrap(), "count");
    }

    // --- Boundary value tests ---

    #[test]
    fn internal_ip_boundary_first_last() {
        assert!(is_internal_ip("10.0.0.0"));
        assert!(is_internal_ip("10.255.255.255"));
        assert!(is_internal_ip("192.168.0.0"));
        assert!(is_internal_ip("192.168.255.255"));
        assert!(!is_internal_ip("0.0.0.0")); // not private
        assert!(!is_internal_ip("255.255.255.255")); // broadcast
    }

    #[test]
    fn community_id_boundary_ips() {
        // Loopback
        let result = community_id_v1("127.0.0.1", "127.0.0.1", 80, 80, "tcp");
        assert!(result.is_ok());
        // IPv6 loopback
        let result = community_id_v1("::1", "::1", 80, 80, "tcp");
        assert!(result.is_ok());
        // Zero port
        let result = community_id_v1("1.2.3.4", "5.6.7.8", 0, 0, "icmp");
        assert!(result.is_ok());
        // Max port
        let result = community_id_v1("1.2.3.4", "5.6.7.8", 65535, 65535, "tcp");
        assert!(result.is_ok());
    }

    #[test]
    fn community_id_unknown_protocol_errors() {
        let result = community_id_v1("1.2.3.4", "5.6.7.8", 80, 80, "unknown_protocol");
        // Unknown protocols are rejected, not defaulted — correct behaviour
        assert!(result.is_err());
    }

    #[test]
    fn registered_domain_deeply_nested() {
        let r = registered_domain_lookup("deep.sub.example.com").unwrap();
        assert_eq!(r.registered_domain.as_deref(), Some("example.com"));
        assert_eq!(r.subdomain.as_deref(), Some("deep.sub"));
    }

    #[test]
    fn grok_regex_actually_matches() {
        // Verify the compiled regex can actually match input
        let regex_str = grok_to_regex("^%{IP:src}:%{POSINT:port}$");
        let re = regex::Regex::new(&regex_str).expect("regex should compile");
        let caps = re.captures("10.0.0.1:8080").expect("should match");
        assert_eq!(caps.name("src").unwrap().as_str(), "10.0.0.1");
        assert_eq!(caps.name("port").unwrap().as_str(), "8080");
    }

    #[test]
    fn grok_regex_rejects_non_matching() {
        let regex_str = grok_to_regex("^%{IP:src}$");
        let re = regex::Regex::new(&regex_str).expect("regex should compile");
        assert!(re.captures("not-an-ip").is_none());
        assert!(re.captures("").is_none());
    }

    #[test]
    fn parse_ua_returns_other_for_unknown() {
        let result = parse_user_agent("SomeRandomBot/1.0").unwrap();
        assert_eq!(result.name.as_deref(), Some("Other"));
    }

    // --- Mutation resistance test ---
    // If you comment out the type-suffix stripping in grok_to_regex_with_map,
    // this test MUST fail (verifies the test catches the bug)
    #[test]
    fn grok_type_suffix_compiles_as_valid_regex() {
        let (regex, _) = grok_to_regex_with_map("%{IP:source.ip:ip}:%{POSINT:port:long}");
        // This MUST compile — if type suffix isn't stripped, it won't
        let re = regex::Regex::new(&regex);
        assert!(
            re.is_ok(),
            "grok regex with type suffixes should compile: {:?}",
            re.err()
        );
    }
}
