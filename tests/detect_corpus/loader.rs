// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The compat corpus as syslog lines, shared by the detector's test and bench.

use std::path::PathBuf;

use dfe_transform_elastic::registry;
use serde_json::Value;

/// One captured event and the source whose capture it came from.
pub struct Line {
    // Read by the scoring test and not by the bench.
    #[allow(dead_code)]
    pub source: &'static str,
    pub text: String,
}

/// `DFE_COMPAT_CORPUS`, else the checkout's `testdata/compat`.
pub fn corpus_root() -> PathBuf {
    std::env::var_os("DFE_COMPAT_CORPUS").map_or_else(
        || PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata/compat"),
        PathBuf::from,
    )
}

/// What a device would have put on the wire: the vendor payload as a string.
fn text_of(doc: &Value) -> Option<String> {
    let original = doc.pointer("/event/original").and_then(Value::as_str);
    if doc.get("cef").is_some() && original.is_some() {
        return original.map(str::to_owned);
    }
    if let Some(message) = doc.get("message").and_then(Value::as_str) {
        return Some(message.to_owned());
    }
    if let Some(original) = original {
        return Some(original.to_owned());
    }
    doc.get("json")
        .filter(|j| j.is_object())
        .map(Value::to_string)
}

/// Every capture the registry has a data stream for, or `None` when the
/// corpus is not on this machine.
pub fn load() -> Option<Vec<Line>> {
    let root = corpus_root();
    if !root.is_dir() {
        return None;
    }
    let mut by_dataset: Vec<_> = registry::sources()
        .filter_map(|name| registry::dataset(name).map(|d| (d, name)))
        .collect();
    by_dataset.sort_unstable();

    let mut lines = Vec::new();
    for (dataset, source) in by_dataset {
        let Some((package, stream)) = dataset.split_once('.') else {
            continue;
        };
        let Ok(cases) = std::fs::read_dir(root.join(package).join(stream)) else {
            continue;
        };
        let mut inputs: Vec<_> = cases
            .filter_map(Result::ok)
            .map(|case| case.path().join("input.ndjson"))
            .collect();
        inputs.sort();
        for input in inputs {
            let Ok(text) = std::fs::read_to_string(&input) else {
                continue;
            };
            lines.extend(
                text.lines()
                    .filter_map(|l| serde_json::from_str::<Value>(l).ok())
                    .filter_map(|doc| text_of(&doc))
                    .map(|text| Line { source, text }),
            );
        }
    }
    Some(lines)
}
