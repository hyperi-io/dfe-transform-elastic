// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! `prisma_cloud`'s vulnerability pruning: each list of vendor findings
//! replaced by the CVE identifiers it carried and nothing else.
//!
//! Five spellings of the one operation -- three over a `.data` list the host
//! record holds directly, two over the `vulnerabilities` of every layer in a
//! build history.

use serde_json::{Map, Value};

use dfe_core::Event;

use super::Entry;

/// `remove_prisma_cloud_host_all_compliance_data_except_cve` in
/// `prisma_cloud/host`.
fn all_compliance_cves(event: &mut Event, _params: &Value) {
    fold_findings(event, "prisma_cloud.host.all_compliance.data");
}

/// `remove_compliance_issues_data_except_cve` in `prisma_cloud/host`.
fn compliance_issues_cves(event: &mut Event, _params: &Value) {
    fold_findings(event, "prisma_cloud.host.compliance_issues.data");
}

/// `remove_host_vulnerabilities_data_except_cve` in `prisma_cloud/host`.
fn host_vulnerabilities_cves(event: &mut Event, _params: &Value) {
    fold_findings(event, "prisma_cloud.host.vulnerabilities.data");
}

/// The untagged history pruner in `prisma_cloud/host`: the findings of every
/// layer of the host's own build history.
fn history_cves(event: &mut Event, _params: &Value) {
    fold_history(event, "prisma_cloud.host.history");
}

/// The untagged image-history pruner in `prisma_cloud/host`: the same, over the
/// history of the image the host runs.
fn image_history_cves(event: &mut Event, _params: &Value) {
    fold_history(event, "prisma_cloud.host.image.history");
}

/// Replace a list of findings with the one-key map holding their CVEs.
fn fold_findings(event: &mut Event, path: &str) {
    let Some(findings) = event.take_array(path) else {
        return;
    };
    let _ = event.update(path, wrap_cves(&findings));
}

/// The same, for the `vulnerabilities` of every item in a history list.
fn fold_history(event: &mut Event, path: &str) {
    let Some(mut layers) = event.take_array(path) else {
        return;
    };
    for layer in &mut layers {
        let Some(record) = layer.as_object_mut() else {
            continue;
        };
        let findings = record
            .get("vulnerabilities")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        record.insert("vulnerabilities".to_owned(), wrap_cves(&findings));
    }
    let _ = event.update(path, Value::Array(layers));
}

/// The CVE identifiers of a list of findings, under the single key the script
/// leaves behind.
fn wrap_cves(findings: &[Value]) -> Value {
    let mut cves = Vec::new();
    for finding in findings {
        let Some(cve) = finding.get("cve") else {
            continue;
        };
        if cve.is_null() || cve.as_str() == Some("") {
            continue;
        }
        cves.push(cve.clone());
    }
    let mut wrapper = Map::new();
    wrapper.insert("cve".to_owned(), Value::Array(cves));
    Value::Object(wrapper)
}

/// Every `prisma_cloud` script transcribed here.
pub const ENTRIES: &[Entry] = &[
    Entry {
        hash: "41812d345b101d9df08a93a101a038837a44355c0daee1a03abe32aba92eb1d4",
        source: "prisma_cloud",
        name: "all_compliance_cves",
        run: all_compliance_cves,
    },
    Entry {
        hash: "1d8da9bce66dbb251ccba2952baa7222d8d309e033091d13e6b6e9ff942113b4",
        source: "prisma_cloud",
        name: "compliance_issues_cves",
        run: compliance_issues_cves,
    },
    Entry {
        hash: "8f8a686187db4cf425b794d772527a4990aa177b242016dffa95c92a5c230d1c",
        source: "prisma_cloud",
        name: "host_vulnerabilities_cves",
        run: host_vulnerabilities_cves,
    },
    Entry {
        hash: "043a319a16538ca40254532d90bd68b08ebdf2a585b9fbb6ded2cbd290e016d5",
        source: "prisma_cloud",
        name: "history_cves",
        run: history_cves,
    },
    Entry {
        hash: "5d4e47230f52be531701cd0b0bc56b1f817817b234c61766108b716f06900493",
        source: "prisma_cloud",
        name: "image_history_cves",
        run: image_history_cves,
    },
];
