// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `ti_opencti`'s GraphQL edges, observable grouping and ECS fold, transcribed.
//!
//! `OpenCTI` answers over GraphQL, so every collection arrives as
//! `{edges: [{node: ...}]}` and most of this pipeline is unwrapping that:
//! lift the nodes out, group the observables by STIX entity type, then fold
//! each group into ONE object, because a mapping cannot hold a list of
//! objects with a leaf underneath it.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::{java_bucket, java_table_size, painless_to_string};

/// The observable edges, as the API returns them.
const EDGES: &str = "observables.edges";

/// Where the grouped observables land.
const OBSERVABLE: &str = "opencti.observable";

/// Where the external reference nodes land.
const EXTERNAL_REFERENCE: &str = "opencti.indicator.external_reference";

/// The ECS containers `merge_maps` folds, after the observables themselves.
const FOLDED: [&str; 6] = [
    EXTERNAL_REFERENCE,
    "threat.indicator.file",
    "threat.indicator.as",
    "threat.indicator.url",
    "threat.indicator.registry",
    "threat.indicator.x509",
];

/// The single-valued indicator fields the mapping wants as arrays.
const WRAPPED: [&str; 5] = [
    "threat.indicator.file.name",
    "threat.indicator.file.extension",
    "threat.indicator.email.address",
    "threat.indicator.ip",
    "threat.indicator.url",
];

/// The hash algorithms `related.hash` gathers, in the script's order.
const RELATED_ALGORITHMS: [&str; 3] = ["md5", "sha1", "sha256"];

/// The untagged startup-info folder in `ti_opencti/indicator`: a process
/// observable's `{key, value}` pairs folded into one object.
fn fold_process_startup_info(event: &mut Event, _params: &Value) {
    let Some(mut edges) = event.take_array(EDGES) else {
        return;
    };
    for edge in &mut edges {
        let Some(node) = edge.get_mut("node").and_then(Value::as_object_mut) else {
            continue;
        };
        let folded = {
            let Some(Value::Array(pairs)) = node.get("startup_info") else {
                continue;
            };
            let mut folded = Map::with_capacity(pairs.len());
            for pair in pairs {
                let key = painless_to_string(pair.get("key").unwrap_or(&Value::Null));
                folded.insert(key, pair.get("value").cloned().unwrap_or(Value::Null));
            }
            folded
        };
        node.insert("startup_info".to_string(), Value::Object(folded));
    }
    let _ = event.update(EDGES, Value::Array(edges));
}

/// The untagged hash folder in `ti_opencti/indicator`: an observable's STIX
/// `hashes` list taken to an ECS `hash` object.
fn hashes_to_ecs(event: &mut Event, _params: &Value) {
    let Some(mut edges) = event.take_array(EDGES) else {
        return;
    };
    for edge in &mut edges {
        let Some(node) = edge.get_mut("node").and_then(Value::as_object_mut) else {
            continue;
        };
        ecs_hashes(node);
        if let Some(Value::Array(libraries)) = node.get_mut("service_dlls") {
            for library in libraries {
                if let Some(fields) = library.as_object_mut() {
                    ecs_hashes(fields);
                }
            }
        }
    }
    let _ = event.update(EDGES, Value::Array(edges));
}

/// One object's `hashes` list keyed by ECS algorithm name.
///
/// STIX spells the SHA family with a hyphen and ECS does not, and SHA-3 takes
/// an underscore because the digest length is a separate ECS field name.
fn ecs_hashes(fields: &mut Map<String, Value>) {
    if !fields.contains_key("hashes") {
        return;
    }
    let folded = {
        // The vendor's `hashesToECS` raises on anything else, taking the rest
        // of the script with it.
        let Some(hashes) = fields.get("hashes").and_then(Value::as_array) else {
            return;
        };
        let mut folded = Map::with_capacity(hashes.len());
        for entry in hashes {
            let Some(algorithm) = entry.get("algorithm").and_then(Value::as_str) else {
                return;
            };
            let algorithm = algorithm
                .to_lowercase()
                .replace("sha-", "sha")
                .replace("sha3-", "sha3_");
            folded.insert(algorithm, entry.get("hash").cloned().unwrap_or(Value::Null));
        }
        folded
    };
    if !folded.is_empty() {
        fields.insert("hash".to_string(), Value::Object(folded));
    }
    fields.shift_remove("hashes");
}

/// The untagged reference mover in `ti_opencti/indicator`: each external
/// reference node lifted out of its edge, without the nulls `OpenCTI` fills
/// every unset member with.
fn move_external_references(event: &mut Event, _params: &Value) {
    let nodes: Vec<Value> = {
        let Some(edges) = event.get_array("externalReferences.edges") else {
            return;
        };
        edges
            .iter()
            .map(|edge| {
                let mut node = Map::new();
                if let Some(Value::Object(fields)) = edge.get("node") {
                    for (key, value) in fields {
                        if !value.is_null() {
                            node.insert(key.clone(), value.clone());
                        }
                    }
                }
                Value::Object(node)
            })
            .collect()
    };
    // The list is created inside the loop, so no edges means no list.
    if nodes.is_empty() {
        return;
    }
    if !event.has(EXTERNAL_REFERENCE) {
        let _ = event.set(EXTERNAL_REFERENCE, Value::Array(Vec::new()));
    }
    for node in nodes {
        let _ = event.append(EXTERNAL_REFERENCE, node);
    }
}

/// The untagged observable mover in `ti_opencti/indicator`: each observable
/// node filed under its own entity type.
///
/// The container is created whether or not anything lands in it, and the
/// prune that follows is what takes an empty one away again.
fn group_observables(event: &mut Event, _params: &Value) {
    if !event.has(OBSERVABLE) {
        let _ = event.set(OBSERVABLE, Value::Object(Map::new()));
    }
    let grouped: Vec<(String, Value)> = {
        let Some(edges) = event.get_array(EDGES) else {
            return;
        };
        edges
            .iter()
            .filter_map(|edge| {
                let node = edge.get("node")?;
                let entity = node.get("entity_type")?.as_str()?;
                // STIX calls a file a StixFile, and every other entity type is
                // hyphenated where ECS wants a separator a field name can hold.
                let entity = entity
                    .replace("StixFile", "file")
                    .to_lowercase()
                    .replace('-', "_");
                Some((entity, node.clone()))
            })
            .collect()
    };
    for (entity, node) in grouped {
        let _ = event.append(&format!("{OBSERVABLE}.{entity}"), node);
    }
}

/// The untagged prune in `ti_opencti/indicator`: the nulls `OpenCTI` sends for
/// every unset member, and any entity type left with nothing in it.
fn prune_observables(event: &mut Event, _params: &Value) {
    let pruned = {
        let Some(held) = event.get(OBSERVABLE) else {
            return;
        };
        let mut pruned = drop_nulls(held);
        if let Some(groups) = pruned.as_object_mut() {
            groups.retain(|_, group| !is_empty_container(group));
        }
        pruned
    };
    if pruned.as_object().is_some_and(Map::is_empty) {
        event.remove(OBSERVABLE);
        return;
    }
    let _ = event.update(OBSERVABLE, pruned);
}

/// The vendor's `dropNulls`: a null member goes, a container stays whether or
/// not anything survived inside it.
fn drop_nulls(value: &Value) -> Value {
    match value {
        Value::Object(fields) => {
            let mut out = Map::with_capacity(fields.len());
            for (key, held) in fields {
                if held.is_object() || held.is_array() {
                    out.insert(key.clone(), drop_nulls(held));
                } else if !held.is_null() {
                    out.insert(key.clone(), held.clone());
                }
            }
            Value::Object(out)
        }
        Value::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                if item.is_object() || item.is_array() {
                    out.push(drop_nulls(item));
                } else if !item.is_null() {
                    out.push(item.clone());
                }
            }
            Value::Array(out)
        }
        other => other.clone(),
    }
}

/// Whether a container holds nothing. Anything that is not one answers false,
/// which is what the vendor's `.size()` call raising on it leaves behind.
fn is_empty_container(value: &Value) -> bool {
    match value {
        Value::Array(items) => items.is_empty(),
        Value::Object(fields) => fields.is_empty(),
        _ => false,
    }
}

/// `merge_maps` in `ti_opencti/indicator`: every list of maps folded into one
/// map, so no ECS container is a list with leaves underneath it.
fn merge_lists_of_maps(event: &mut Event, _params: &Value) {
    let folded: Vec<(String, Value)> = {
        match event.get_object(OBSERVABLE) {
            Some(groups) => groups
                .iter()
                .filter_map(|(entity, group)| {
                    Some((
                        format!("{OBSERVABLE}.{entity}"),
                        fold_list(group.as_array()?),
                    ))
                })
                .collect(),
            None => Vec::new(),
        }
    };
    for (path, merged) in folded {
        let _ = event.update(&path, merged);
    }
    for path in FOLDED {
        let merged = {
            let Some(items) = event.get(path).and_then(Value::as_array) else {
                continue;
            };
            fold_list(items)
        };
        let _ = event.update(path, merged);
    }
}

/// The vendor's `mergeListOfMaps`: every map in the list merged onto one.
fn fold_list(items: &[Value]) -> Value {
    let mut merged = Map::new();
    for item in items {
        if let Some(fields) = item.as_object() {
            merge_into(&mut merged, fields);
        }
    }
    Value::Object(merged)
}

/// The vendor's `mergeMaps`: `from`'s members over `into`'s, with two nested
/// maps merged and two disagreeing values GATHERED rather than one winning.
fn merge_into(into: &mut Map<String, Value>, from: &Map<String, Value>) {
    for (key, value) in from {
        let combined = match into.get(key) {
            None => value.clone(),
            Some(held) if held == value => value.clone(),
            Some(Value::Object(nested)) if value.is_object() => {
                let mut nested = nested.clone();
                if let Some(right) = value.as_object() {
                    merge_into(&mut nested, right);
                }
                Value::Object(nested)
            }
            Some(held) => {
                let mut gathered: Vec<Value> = Vec::new();
                match held {
                    Value::Array(items) => {
                        for item in items {
                            add_unique(&mut gathered, item);
                        }
                    }
                    other => add_unique(&mut gathered, other),
                }
                match value {
                    Value::Array(items) => {
                        for item in items {
                            add_unique(&mut gathered, item);
                        }
                    }
                    other => add_unique(&mut gathered, other),
                }
                Value::Array(java_set_order(gathered))
            }
        };
        into.insert(key.clone(), combined);
    }
}

/// Add a value the way a `HashSet` does -- once.
fn add_unique(gathered: &mut Vec<Value>, value: &Value) {
    if !gathered.contains(value) {
        gathered.push(value.clone());
    }
}

/// A Java `HashSet` read back out: bucket order, insertion order inside a
/// bucket.
///
/// The bucket comes off the value's string rendering, which is what every
/// member an observable gathers here is.
fn java_set_order(values: Vec<Value>) -> Vec<Value> {
    let table = java_table_size(values.len());
    let mut placed: Vec<(usize, usize, Value)> = values
        .into_iter()
        .enumerate()
        .map(|(position, value)| {
            (
                java_bucket(&painless_to_string(&value), table),
                position,
                value,
            )
        })
        .collect();
    placed.sort_by_key(|(bucket, position, _)| (*bucket, *position));
    placed.into_iter().map(|(.., value)| value).collect()
}

/// The untagged hash gatherer in `ti_opencti/indicator`: every file digest the
/// indicator names, under `related.hash`.
fn file_hashes_into_related(event: &mut Event, _params: &Value) {
    let gathered = {
        let Some(held) = event.get("threat.indicator.file") else {
            return;
        };
        if held.is_null() {
            return;
        }
        let files: Vec<&Value> = match held {
            Value::Array(items) => items.iter().collect(),
            other => vec![other],
        };
        let mut gathered: Vec<Value> = event.get_array("related.hash").cloned().unwrap_or_default();
        for file in files {
            let Some(hashes) = file.get("hash").filter(|hashes| !hashes.is_null()) else {
                continue;
            };
            for algorithm in RELATED_ALGORITHMS {
                if let Some(digest) = hashes.get(algorithm).filter(|digest| !digest.is_null()) {
                    add_unique(&mut gathered, digest);
                }
            }
        }
        gathered
    };
    if gathered.is_empty() {
        event.remove("related.hash");
        return;
    }
    // The script reads the gathered digests out of a `HashSet`, and
    // `related.hash` carries no ordering either side of the comparison.
    let _ = event.set("related.hash", Value::Array(gathered));
}

/// `get_file_extensions` in `ti_opencti/indicator`: the extension behind an
/// indicator's file name.
fn extension_from_file_name(event: &mut Event, _params: &Value) {
    let Some(name) = event.get_string("threat.indicator.file.name") else {
        return;
    };
    let Some(extension) = extension_of(&name) else {
        return;
    };
    let _ = event.set("threat.indicator.file.extension", extension);
}

/// The untagged extension reader in `ti_opencti/indicator`'s `ecs_from_file`:
/// one extension per name an observable carries.
///
/// A file observable gathers the vendor's `name` and every
/// `x_opencti_additional_names` entry, so this side reads a LIST where the
/// pattern side reads one name.
fn extensions_from_observable_names(event: &mut Event, _params: &Value) {
    let extensions: Vec<Value> = {
        let Some(names) = event.get_array("_tmp_file.name") else {
            return;
        };
        names
            .iter()
            .filter_map(|name| extension_of(name.as_str()?))
            .map(Value::String)
            .collect()
    };
    if extensions.is_empty() {
        return;
    }
    let _ = event.set("_tmp_file.extension", Value::Array(extensions));
}

/// The extension behind one name.
///
/// The name may arrive as a whole path in either slash, so the last segment is
/// the name and the last dot in it opens the extension.
fn extension_of(fullname: &str) -> Option<String> {
    let last = java_split(fullname, &['/', '\\']).pop()?;
    if !last.contains('.') {
        return None;
    }
    let extension = java_split(last, &['.']).pop()?;
    if extension.is_empty() {
        return None;
    }
    Some(extension.to_string())
}

/// Java's `Pattern.split`, which discards the EMPTY segments a trailing
/// separator leaves behind.
fn java_split<'t>(text: &'t str, separators: &[char]) -> Vec<&'t str> {
    let mut parts: Vec<&str> = text.split(separators).collect();
    while parts.last().is_some_and(|part| part.is_empty()) {
        parts.pop();
    }
    parts
}

/// `convert_strings_to_arrays` in `ti_opencti/indicator`: the indicator fields
/// the mapping declares as arrays, wrapped where one value arrived.
fn wrap_indicator_values(event: &mut Event, _params: &Value) {
    for path in WRAPPED {
        let wrapped = {
            let Some(held) = event.get(path) else {
                continue;
            };
            if held.is_null() || held.is_array() {
                continue;
            }
            Value::Array(vec![held.clone()])
        };
        let _ = event.update(path, wrapped);
    }
}

/// Every `ti_opencti` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "b8554175dc85d017c05b4d9aeab91366d6acdc3f8af64eb21a6aa6bb3ae8254b",
        source: "ti_opencti",
        name: "fold_process_startup_info",
        run: fold_process_startup_info,
    },
    Entry {
        hash: "1961c4577dd3590f3274dce94f37b8cd36a102e2681da5afe07f0ecf8a36e27a",
        source: "ti_opencti",
        name: "hashes_to_ecs",
        run: hashes_to_ecs,
    },
    Entry {
        hash: "456e8b6a689db5e00077a004233c678b7e3ccd5bce6e7706d0885b34cb9592f3",
        source: "ti_opencti",
        name: "move_external_references",
        run: move_external_references,
    },
    Entry {
        hash: "b33df6bf77373400f179463d0460ebbcdcf5beb769cac6aeedae510eea2c97de",
        source: "ti_opencti",
        name: "group_observables",
        run: group_observables,
    },
    Entry {
        hash: "b93f01fb0e781c7a9a95652989057967ecf16263bcd9f19eb4ede69318f1c4c9",
        source: "ti_opencti",
        name: "prune_observables",
        run: prune_observables,
    },
    Entry {
        hash: "9474fbe19b0eb9e17e7900a3847e57702bf43c96096b728e7ac9b78038dfcd07",
        source: "ti_opencti",
        name: "merge_lists_of_maps",
        run: merge_lists_of_maps,
    },
    Entry {
        hash: "a99b81fb813c168c222491b143e7a08b33a646a3ea250d6acc3f67e4af4f32f4",
        source: "ti_opencti",
        name: "file_hashes_into_related",
        run: file_hashes_into_related,
    },
    Entry {
        hash: "64bc62d91f61e78499f4b56a09bd241162c4f33fd01bcc81119f5e88c5bf6c37",
        source: "ti_opencti",
        name: "extension_from_file_name",
        run: extension_from_file_name,
    },
    Entry {
        hash: "4c8968b4b14ad6efd043a2e746128602b865305d6a65200640d156c10dcb0728",
        source: "ti_opencti",
        name: "extensions_from_observable_names",
        run: extensions_from_observable_names,
    },
    Entry {
        hash: "20d4585746e459cd7c73ec3d559d15614183f4449fd8d3dd731dc0a700bf93ee",
        source: "ti_opencti",
        name: "wrap_indicator_values",
        run: wrap_indicator_values,
    },
];
