// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Scripts the `jamf_compliance_reporter` package ships, transcribed by hand.

use serde_json::{Map, Value};

use super::Entry;
use crate::helpers::{java_bucket, java_table_size};
use dfe_core::event::Event;

/// The socket address families both `aue_bind`/`aue_connect` and `aue_accept`
/// map, in the vendor's own spelling.
///
/// The two duplicate keys are the vendor's, not a transcription slip: `1` maps
/// to `AF_LOCAL` and the separate key `AF_LOCAL` maps on to `AF_UNIX`. So are
/// `AF_ImapPLINK` and `AF_ECmapA`, which a find-and-replace in the integration
/// mangled and Elasticsearch emits unchanged.
const SOCKET_FAMILIES: &[(&str, &str)] = &[
    ("0", "AF_UNSPEC"),
    ("1", "AF_LOCAL"),
    ("AF_LOCAL", "AF_UNIX"),
    ("2", "AF_INET"),
    ("3", "AF_ImapPLINK"),
    ("4", "AF_PUP"),
    ("5", "AF_CHAOS"),
    ("6", "AF_NS"),
    ("7", "AF_ISO"),
    ("AF_ISO", "AF_OSI"),
    ("8", "AF_ECmapA"),
    ("9", "AF_DATAKIT"),
    ("10", "AF_CCITT"),
    ("11", "AF_SNA"),
    ("12", "AF_DECnet"),
    ("13", "AF_DLI"),
    ("14", "AF_LAT"),
    ("15", "AF_HYLINK"),
    ("16", "AF_APPLETALK"),
    ("17", "AF_ROUTE"),
    ("18", "AF_LINK"),
    ("19", "pseudo_AF_XTP"),
    ("20", "AF_COIP"),
    ("21", "AF_CNT"),
    ("22", "pseudo_AF_RTIP"),
    ("23", "AF_IPX"),
    ("24", "AF_SIP"),
    ("25", "pseudo_AF_PIP"),
];

/// Look `json.inet_family` up in the table and write the answer, null included.
///
/// `map.get` answers null for a family the table does not carry, and the
/// assignment stores that null rather than leaving the field absent.
fn socket_family(event: &mut Event, target: &str) {
    let Some(family) = event.get_as_string("json.inet_family") else {
        return;
    };
    let mapped = SOCKET_FAMILIES
        .iter()
        .find(|(key, _)| *key == family)
        .map_or(Value::Null, |(_, name)| Value::from(*name));
    let _ = event.set(target, mapped);
}

/// `jamf_compliance_reporter/log`, the `aue_execve` family: turn the vendor's
/// numbered-key argument object into the ordered list ECS wants.
///
/// The object is a Java `HashMap` by the time the script sees it, so the
/// arguments come out in bucket order rather than the order they were parsed.
/// `process.args` is assigned before the object is even read, so a document
/// with no arguments gets an empty list the later empty-value prune removes.
fn convert_args_to_array(event: &mut Event, _params: &Value) {
    let mut args: Vec<Value> = Vec::new();
    if let Some(Value::Object(source)) = event.get("json.args") {
        let table = java_table_size(source.len());
        let mut ordered: Vec<(usize, usize, &Value)> = source
            .iter()
            .enumerate()
            .map(|(position, (key, value))| (java_bucket(key, table), position, value))
            .collect();
        ordered.sort_by_key(|(bucket, position, _)| (*bucket, *position));
        args = ordered
            .into_iter()
            .map(|(_, _, value)| value.clone())
            .collect();
    }
    let _ = event.set("process.args", Value::Array(args));
}

/// `jamf_compliance_reporter/log`, the `aue_bind`/`aue_connect` family: name
/// the internet socket's address family.
fn map_inet_socket_family(event: &mut Event, _params: &Value) {
    socket_family(event, "jamf_compliance_reporter.log.socket.inet.family");
}

/// `jamf_compliance_reporter/log`, the `aue_accept` family: name the unix
/// socket's address family, off the same vendor field as the internet one.
fn map_unix_socket_family(event: &mut Event, _params: &Value) {
    socket_family(event, "jamf_compliance_reporter.log.socket.unix.family");
}

/// `jamf_compliance_reporter/log`, the `app_metrics` pipeline: the vendor's
/// CPU percentage as the ECS fraction, rounded to three places.
///
/// Java's `Math.round` is floor of the value plus a half, which is not Rust's
/// round-half-away-from-zero on a negative reading.
fn app_metrics_cpu_usage(event: &mut Event, _params: &Value) {
    let Some(percentage) = event.get_f64("json.app_metric_info.cpu_percentage") else {
        return;
    };
    let rounded = (percentage * 10.0 + 0.5).floor();
    let _ = event.set("host.cpu", Value::Object(Map::new()));
    let _ = event.set("host.cpu.usage", rounded / 1000.0);
}

/// `jamf_compliance_reporter/log`, the `aue_execve` family: name the operating
/// system off the executing environment's `ARCH`.
///
/// The vendor's own word for macOS is `macintosh`, which the params table
/// renames in place -- and the renamed value is what the allow-list is then
/// checked against. An architecture the list does not carry returns before the
/// removal, so `ARCH` stays in the document under whatever name it now has.
fn exec_env_host_os_type(event: &mut Event, params: &Value) {
    let Some(mut arch) = event.get_string("json.exec_env.env.ARCH") else {
        return;
    };
    if let Some(Value::Object(replacements)) = params.get("replacements") {
        for (from, to) in replacements {
            if arch == *from
                && let Some(renamed) = to.as_str()
            {
                arch = renamed.to_string();
                event.update("json.exec_env.env.ARCH", arch.clone());
            }
        }
    }
    let allowed = params
        .get("allowed")
        .and_then(Value::as_array)
        .is_some_and(|names| {
            names
                .iter()
                .any(|name| name.as_str() == Some(arch.as_str()))
        });
    if !allowed {
        return;
    }
    let _ = event.set("host.os.type", arch);
    event.remove("json.exec_env.env.ARCH");
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "f6892470e60c11b72f56cc6f5a5fe237eec6f7d46c498ab7ad95cc3144bc11e1",
        source: "jamf_compliance_reporter",
        name: "convert_args_to_array",
        run: convert_args_to_array,
    },
    Entry {
        hash: "8d15339af1691a0fc4697712285be3e7a3c19ff3747264b0062a1ee5d353776d",
        source: "jamf_compliance_reporter",
        name: "map_inet_socket_family",
        run: map_inet_socket_family,
    },
    Entry {
        hash: "3b0305c46580621e66d374a7653699949ef70fb74f2776d014f21ded1d8bcf29",
        source: "jamf_compliance_reporter",
        name: "app_metrics_cpu_usage",
        run: app_metrics_cpu_usage,
    },
    Entry {
        hash: "4ddcaacb63de59e8db666bd6ab6c1e66c164d79f75008bf91b9fdcc271c9bfa3",
        source: "jamf_compliance_reporter",
        name: "map_unix_socket_family",
        run: map_unix_socket_family,
    },
    Entry {
        hash: "35fdc6c0fa8eb1c48c1b42e4a76d129fe0bf1e49dd3a2cbcc7c2c9dd94e2a4ff",
        source: "jamf_compliance_reporter",
        name: "exec_env_host_os_type",
        run: exec_env_host_os_type,
    },
];
