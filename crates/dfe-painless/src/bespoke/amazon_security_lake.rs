// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `amazon_security_lake`'s OCSF lifts: the digests an object carries turned
//! into an ECS hash map, the container image digest rendered as one
//! `algorithm:value` string, the DNS answer flags gathered, and the process
//! ancestry cut off where OCSF allows it to run away.
//!
//! The digest scripts are one operation spelled seven times, once per OCSF
//! object that can hold `hashes`, and the ancestry cut is spelled three times
//! over the three roots that can hold a process.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;
use crate::helpers::{java_set_order, java_to_string_sized, painless_to_string};

/// How deep the ancestry may run before the vendor's own script stops carrying
/// it as structure.
const MAX_ANCESTRY: usize = 15;

/// `script_file_hash_*` in `pipeline_object_actor`.
fn actor_process_file_hash(event: &mut Event, params: &Value) {
    lift_digests(
        event,
        params,
        "ocsf.actor.process.file",
        "hashes",
        "file.hash",
    );
}

/// `script_file_hash_*` in `pipeline_object_file`.
fn file_hash(event: &mut Event, params: &Value) {
    lift_digests(event, params, "ocsf.file", "hashes", "file.hash");
}

/// `script_file_hash_*` in `pipeline_object_process`.
fn process_file_hash(event: &mut Event, params: &Value) {
    lift_digests(event, params, "ocsf.process.file", "hashes", "file.hash");
}

/// `script_file_hash_*` in `pipeline_object_system_activity_helper`.
fn driver_file_hash(event: &mut Event, params: &Value) {
    lift_digests(event, params, "ocsf.driver.file", "hashes", "file.hash");
}

/// `script_module_file_hash_*` in `pipeline_object_system_activity_helper`.
fn module_file_hash(event: &mut Event, params: &Value) {
    lift_digests(event, params, "ocsf.module.file", "hashes", "file.hash");
}

/// `script_job_file_hash_*` in `pipeline_object_system_activity_helper`.
fn job_file_hash(event: &mut Event, params: &Value) {
    lift_digests(event, params, "ocsf.job.file", "hashes", "file.hash");
}

/// `script_tls_client_hash_*` in `pipeline_object_tls`.
fn tls_client_hash(event: &mut Event, params: &Value) {
    lift_digests(
        event,
        params,
        "ocsf.tls.certificate",
        "fingerprints",
        "tls.client.hash",
    );
}

/// `script_container_image_hash_all` in `pipeline_object_actor`.
fn actor_container_image_hash(event: &mut Event, params: &Value) {
    render_container_hash(event, params, "ocsf.actor.process.container.hash");
}

/// `script_container_image_hash_all` in `pipeline_object_process`.
fn process_container_image_hash(event: &mut Event, params: &Value) {
    render_container_hash(event, params, "ocsf.process.container.hash");
}

/// `script_dns_header_flags` in `pipeline_category_network_activity`: the flags
/// of every answer, under the ECS names the params table gives them.
fn dns_header_flags(event: &mut Event, params: &Value) {
    let mut flags: Vec<Value> = Vec::new();
    if let Some(answers) = event.get_array("ocsf.answers") {
        for answer in answers {
            let Some(held) = answer.get("flags") else {
                continue;
            };
            if held.is_null() {
                continue;
            }
            // The script indexes the member as an array, so anything else
            // raises and nothing further is gathered.
            let Some(items) = held.as_array() else {
                return;
            };
            for flag in items {
                let Some(name) = flag.as_str() else {
                    continue;
                };
                let Some(mapped) = params.get(name) else {
                    continue;
                };
                if !flags.contains(mapped) {
                    flags.push(mapped.clone());
                }
            }
        }
    }

    let _ = event.set("dns.header_flags", Value::Array(java_set_order(flags)));
}

/// `script_actor_process_parent_process_stringify` in `pipeline_object_actor`.
fn actor_ancestry_keyword(event: &mut Event, _params: &Value) {
    cut_deep_ancestry(event, "ocsf.actor.process.parent_process");
}

/// The same stringify in `pipeline_object_process`.
fn process_ancestry_keyword(event: &mut Event, _params: &Value) {
    cut_deep_ancestry(event, "ocsf.process.parent_process");
}

/// The same stringify in `pipeline_category_identity_and_access_management`.
fn logon_process_ancestry_keyword(event: &mut Event, _params: &Value) {
    cut_deep_ancestry(event, "ocsf.logon_process.parent_process");
}

/// Collect an object's digests into `<target>.<ecs name>`, each a set of the
/// values seen under that algorithm.
///
/// An algorithm the params table does not name is dropped, and a `hashes`
/// member that is not a list is left alone -- the script's own `for` raises
/// there and writes nothing.
fn lift_digests(event: &mut Event, params: &Value, source: &str, member: &str, target: &str) {
    let Some(digests) = event.get_array(&format!("{source}.{member}")).cloned() else {
        return;
    };

    let mut collected: Vec<(String, Vec<Value>)> = Vec::new();
    for digest in &digests {
        let Some(algorithm) = digest.get("algorithm").and_then(Value::as_str) else {
            continue;
        };
        let Some(name) = params.get(algorithm).and_then(Value::as_str) else {
            continue;
        };
        let value = digest.get("value").cloned().unwrap_or(Value::Null);
        match collected.iter_mut().find(|(held, _)| held == name) {
            Some((_, values)) => {
                if !values.contains(&value) {
                    values.push(value);
                }
            }
            None => collected.push((name.to_owned(), vec![value])),
        }
    }

    let mut built = Map::with_capacity(collected.len());
    for (name, values) in collected {
        built.insert(name, Value::Array(java_set_order(values)));
    }

    // The script opens each container on the way down, which is what `set`
    // does for the dotted path anyway.
    let _ = event.set(target, Value::Object(built));
}

/// Render the container image digest as the one `algorithm:value` string ECS
/// asks for.
fn render_container_hash(event: &mut Event, params: &Value, source: &str) {
    let Some(algorithm) = event.get_str(&format!("{source}.algorithm")) else {
        return;
    };
    let Some(name) = params.get(algorithm).and_then(Value::as_str) else {
        return;
    };
    let value = event
        .get(&format!("{source}.value"))
        .map_or_else(String::new, painless_to_string);
    let rendered = format!("{name}:{value}");
    let _ = event.set(
        "container.image.hash.all",
        Value::Array(vec![Value::from(rendered)]),
    );
}

/// Replace an ancestry deeper than the vendor carries with its own rendering.
///
/// The count is of the steps BELOW the root, so a chain of fifteen ancestors
/// under `parent_process` is what trips it.
fn cut_deep_ancestry(event: &mut Event, root: &str) {
    let mut depth = 0usize;
    let mut node = event.get(root);
    while let Some(current) = node {
        match current.get("parent_process") {
            Some(next) if !next.is_null() => {
                depth += 1;
                node = Some(next);
            }
            _ => break,
        }
    }
    if depth < MAX_ANCESTRY {
        return;
    }

    let nested = format!("{root}.parent_process");
    let Some(held) = event.get(&nested).cloned() else {
        return;
    };
    // Rendered at the path it was pruned at: the empty-value prune that runs
    // ahead of this crossed a bucket-table boundary in several of these maps,
    // and a Java map keeps iterating through the table it was BUILT with.
    let mut at = nested.clone();
    let rendered = java_to_string_sized(&held, &mut at, &|path| event.map_capacity(path));
    let _ = event.set(&format!("{root}.parent_process_keyword"), rendered);
    event.remove(&nested);
}

/// Every `amazon_security_lake` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "d86bd0c95af789bedefa07991b366e1e11c08f6508659d839172bce8f204a488",
        source: "amazon_security_lake",
        name: "actor_process_file_hash",
        run: actor_process_file_hash,
    },
    Entry {
        hash: "9f02361c1192ebae0c43d431e65d7fbcacfb80ac3ed98b877af1a4fef7a47281",
        source: "amazon_security_lake",
        name: "file_hash",
        run: file_hash,
    },
    Entry {
        hash: "6573e65dcdf7ffc2915429972eedf67b51580dffccbd20858a34b2f14adc7a01",
        source: "amazon_security_lake",
        name: "process_file_hash",
        run: process_file_hash,
    },
    Entry {
        hash: "00e4c1fc632cfeda6387b97b297fd3559c12c7188a25dda565a97ba6f43677ff",
        source: "amazon_security_lake",
        name: "driver_file_hash",
        run: driver_file_hash,
    },
    Entry {
        hash: "2e1d3ca7969b5c769c1156b9debaeaefc9e8dd9526ad9df10fe94b3f21ac85c4",
        source: "amazon_security_lake",
        name: "module_file_hash",
        run: module_file_hash,
    },
    Entry {
        hash: "7be0849147a12d136244b089549ace262a7b687e48dd307c96d33a781df29408",
        source: "amazon_security_lake",
        name: "job_file_hash",
        run: job_file_hash,
    },
    Entry {
        hash: "8dbdffa745cc049ea3fa3bd5b545e3cd4f80151afa7007e0290471ccd1728c2c",
        source: "amazon_security_lake",
        name: "tls_client_hash",
        run: tls_client_hash,
    },
    Entry {
        hash: "b20ff1d66f41b43782ae2dcbb7fc8ee6e762a2f867711bf64f56f2cd9aeb1a72",
        source: "amazon_security_lake",
        name: "actor_container_image_hash",
        run: actor_container_image_hash,
    },
    Entry {
        hash: "1012ae2e8bb86538bb13d6e8a56e1f8849e9d3d9345c58520c5e4bd9e52f0618",
        source: "amazon_security_lake",
        name: "process_container_image_hash",
        run: process_container_image_hash,
    },
    Entry {
        hash: "330499e247d0cefb348dd4ed11aefc946d58bd9847bc7443a2ed453b4d0b8ee3",
        source: "amazon_security_lake",
        name: "dns_header_flags",
        run: dns_header_flags,
    },
    Entry {
        hash: "c3b64eca13a8362b1f0f127745e462d16a10f1b719ff5322630423046299d6d8",
        source: "amazon_security_lake",
        name: "actor_ancestry_keyword",
        run: actor_ancestry_keyword,
    },
    Entry {
        hash: "382d1cdbfce492a3b6f6284d62aa0774f922d5cadde6b3e212b650704ee3b044",
        source: "amazon_security_lake",
        name: "process_ancestry_keyword",
        run: process_ancestry_keyword,
    },
    Entry {
        hash: "96c350ec2f317e9bd2f86732ad562aedfc3c12ed75a85610d5669a32cd2f6efc",
        source: "amazon_security_lake",
        name: "logon_process_ancestry_keyword",
        run: logon_process_ancestry_keyword,
    },
];
