// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Scripts the auditd package ships, transcribed by hand.

use serde_json::Value;

use super::Entry;
use dfe_core::event::Event;

/// `auditd/log`, processor `script_execve_process_args`: lift the `aNN`
/// keys an EXECVE record carries into the ordered `process.args`.
///
/// Every key matching `a<digits><rest>` is taken OUT of `auditd.log`, the
/// `_len` siblings included -- their removal is the whole reason they are
/// collected. What survives is sorted by the numeric argument index, which is
/// what puts `a10` after `a9`. An index that does not start at zero means the
/// kernel truncated the head of the command line, and the vendor marks that
/// with a synthetic first argument rather than dropping the record.
fn execve_process_args(event: &mut Event, _params: &Value) {
    let Some(Value::Object(log)) = event.get("auditd.log") else {
        return;
    };
    let mut matched: Vec<(i64, String, bool)> = Vec::new();
    for key in log.keys() {
        let Some(tail) = key.strip_prefix('a') else {
            continue;
        };
        let digits = tail.len() - tail.trim_start_matches(|c: char| c.is_ascii_digit()).len();
        if digits == 0 {
            continue;
        }
        let Ok(index) = tail[..digits].parse::<i64>() else {
            continue;
        };
        matched.push((index, key.clone(), &tail[digits..] == "_len"));
    }

    let mut path = String::from("auditd.log.");
    let mark = path.len();
    let mut args: Vec<(i64, Value)> = Vec::with_capacity(matched.len());
    for (index, key, is_len) in matched {
        path.truncate(mark);
        path.push_str(&key);
        let value = event.remove(&path);
        if is_len {
            continue;
        }
        args.push((index, value.unwrap_or(Value::Null)));
    }
    if args.is_empty() {
        return;
    }
    args.sort_by_key(|(index, _)| *index);

    let first_index = args[0].0;
    let mut values: Vec<Value> = args.into_iter().map(|(_, value)| value).collect();
    if first_index == 0 {
        let executable = values[0].clone();
        let _ = event.set("process.args", Value::Array(values));
        let _ = event.set("process.executable", executable);
    } else {
        values.insert(
            0,
            Value::from(format!("[... {first_index} truncated arguments ...]")),
        );
        let _ = event.set("process.args", Value::Array(values));
    }
}

/// `auditd/log`, processor `script_process_args`: count the arguments, and
/// only where they really are a list.
fn process_args_count(event: &mut Event, _params: &Value) {
    let Some(args) = event.get_array("process.args") else {
        return;
    };
    let count = i64::try_from(args.len()).unwrap_or(i64::MAX);
    let _ = event.set("process.args_count", count);
}

/// The transcriptions this module registers.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "c8255593b2075af7a5825fda5385cf422cc336f190ae8b38e2b4163e9be38581",
        source: "auditd",
        name: "execve_process_args",
        run: execve_process_args,
    },
    Entry {
        hash: "18aff539c9164aa22dedbedeb3d2bdd20fd97ec7873a829e676b2108537f96df",
        source: "auditd",
        name: "process_args_count",
        run: process_args_count,
    },
];
