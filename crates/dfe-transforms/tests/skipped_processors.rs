// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! How many processors never run because their `if` condition did not
//! transpile.
//!
//! An Elastic ingest processor can carry an `if`. Where the generator cannot
//! express that condition in Rust it emits the processor's body inside
//! `if false` behind a `// SKIPPED:` marker, so the processor is present and
//! unreachable. That is the deliberate safe direction -- running it would
//! apply the processor where the pipeline says not to, and one that then reads
//! an absent field aborts the whole event through `?`, losing every field
//! rather than one (`codegen/processor.rs::wrap_conditional`).
//!
//! Safe is not free. A skipped `set` leaves a field unwritten and a skipped
//! `drop` keeps an event Elastic discards, so this is a defect count:
//!
//! 1. The number is STATED rather than rediscovered.
//! 2. It cannot grow. A new untranspilable condition fails this test.
//!
//! Lower the ceilings as conditions are taught to the transpiler. Never raise
//! them.
//!
//! **This file measured a different marker until 2026-09-03 and had read ZERO
//! since 2026-08-19.** It counted `// TODO: conditional: `, left where a guard
//! was dropped and the body ran UNGUARDED. `8fd4e550` regenerated the six
//! sources it ceilinged and the new codegen stopped emitting that marker
//! entirely -- so both tests passed on an empty scan, and the ceilings said
//! nothing. The unguarded class is now closed by construction; this counts its
//! successor. Hence `FILES_SCANNED`: a scan that finds nothing must fail
//! rather than pass.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The marker the generator leaves where a condition defeated the transpiler.
const MARKER: &str = "// SKIPPED: condition not transpiled: ";

/// The walk has to actually reach the generated tree.
///
/// Not a property of the code under test -- a property of this test. Reading
/// zero markers is indistinguishable from scanning zero files, and that is
/// exactly how the previous version of this file passed for a fortnight while
/// measuring nothing.
const FILES_SCANNED: usize = 2_500;

/// Total skipped processors as measured today. Only ever goes down.
const CEILING: usize = 282;

/// Per-source ceilings, so a fix in one source cannot be cancelled out by a
/// regression in another and still pass the total.
const PER_SOURCE: &[(&str, usize)] = &[
    ("agentless_hello_world_generic", 1),
    ("arista_ngfw_log", 1),
    ("aws_securityhub_finding", 5),
    ("barracuda_cloudgen_firewall_log", 2),
    ("beelzebub_logs", 2),
    ("beyondtrust_epm_event", 3),
    ("bitdefender_push_notifications", 1),
    ("cisco_duo_auth", 1),
    ("cisco_ise_log", 24),
    ("claude_code_events", 1),
    ("claude_cowork_events", 1),
    ("coredns_log", 1),
    ("cybereason_malop_connection", 1),
    ("cybereason_suspicions_process", 6),
    ("darktrace_ai_analyst_alert", 1),
    ("doppel_alerts", 1),
    ("ece_adminconsole", 6),
    ("elastic_agent_elastic_agent_logs", 2),
    ("elasticsearch_audit", 4),
    ("elasticsearch_querylog", 4),
    ("elasticsearch_server", 4),
    ("entityanalytics_ad_entity", 11),
    ("envoyproxy_log", 9),
    ("eset_protect_event", 5),
    ("extrahop_investigation", 1),
    ("f5_bigip_log", 8),
    ("forcepoint_web_logs", 2),
    ("github_audit", 1),
    ("google_workspace_login", 1),
    ("hackerone_report", 6),
    ("hadoop_datanode", 2),
    ("haproxy_log", 1),
    ("hashicorp_vault_audit", 1),
    ("iptables_log", 1),
    ("juniper_srx_log", 4),
    ("kibana_audit", 5),
    ("kibana_log", 5),
    ("lyve_cloud_audit", 4),
    ("microsoft_exchange_online_message_trace_log", 1),
    ("microsoft_exchange_server_httpproxy", 1),
    ("microsoft_exchange_server_imap4_pop3", 3),
    ("microsoft_exchange_server_messagetracking", 2),
    ("microsoft_exchange_server_smtp", 3),
    ("opencanary_events", 1),
    ("oracle_database_audit", 2),
    ("pfsense_log", 7),
    ("platform_observability_kibana_audit", 5),
    ("platform_observability_kibana_log", 5),
    ("qualys_vmdr_asset_host_detection", 2),
    ("qualys_vmdr_knowledge_base", 1),
    ("qualys_was_vulnerability", 3),
    ("sentinel_one_cloud_funnel_event", 19),
    ("servicenow_event", 3),
    ("slack_audit", 1),
    ("snyk_audit_logs", 8),
    ("symantec_endpoint_security_event", 5),
    ("sysdig_event", 4),
    ("tanium_threat_response", 2),
    ("ti_cif3_feed", 1),
    ("ti_eclecticiq_threat", 20),
    ("ti_opencti_indicator", 1),
    ("trend_micro_vision_one_detection", 2),
    ("trend_micro_vision_one_telemetry", 1),
    ("vectra_detect_log", 7),
    ("vsphere_log", 31),
    ("zscaler_zpa_user_status", 2),
];

/// Skipped processors whose body DROPS the event, so an event Elastic
/// discards is kept instead. Six today, all a `drop` gated on a condition the
/// transpiler cannot read.
const DROPS_NEVER_TAKEN: usize = 6;

fn source_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// Skipped processors per integration directory, the total, and the file count
/// the walk covered.
fn survey() -> (BTreeMap<String, usize>, usize, usize) {
    let root = source_root();
    let mut files = Vec::new();
    rust_files(&root, &mut files);
    let scanned = files.len();

    let mut per_source: BTreeMap<String, usize> = BTreeMap::new();
    let mut total = 0;

    for path in files {
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        let count = text.matches(MARKER).count();
        if count == 0 {
            continue;
        }
        total += count;

        // `<root>/filebeat/<source>/<file>.rs` -- the integration directory.
        let source = path
            .parent()
            .and_then(Path::file_name)
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        *per_source.entry(source).or_default() += count;
    }

    (per_source, total, scanned)
}

#[test]
fn skipped_processors_do_not_increase() {
    let (per_source, total, scanned) = survey();

    for (source, count) in &per_source {
        println!("{count:5}  {source}");
    }
    println!("{total} processor(s) skipped over {scanned} generated files");

    assert!(
        scanned >= FILES_SCANNED,
        "the walk covered {scanned} files, under the {FILES_SCANNED} floor -- \
         it is not reading the generated tree, so every count below is a zero \
         that means nothing"
    );
    assert!(
        total <= CEILING,
        "skipped processors rose from {CEILING} to {total} -- a condition the \
         transpiler used to read no longer transpiles, so the processor behind \
         it stopped running"
    );

    for (source, ceiling) in PER_SOURCE {
        let count = per_source.get(*source).copied().unwrap_or(0);
        assert!(
            count <= *ceiling,
            "{source}: skipped processors rose from {ceiling} to {count}"
        );
    }

    // A source not in the table must have none, or the table is stale.
    for (source, count) in &per_source {
        assert!(
            PER_SOURCE.iter().any(|(name, _)| name == source),
            "{source} has {count} skipped processor(s) and no ceiling -- add one"
        );
    }
}

/// A skipped `set` leaves one field unwritten. A skipped DROP keeps an event
/// Elastic throws away, so every field of an event that should not exist
/// reaches the sink.
///
/// The mirror of what this file used to count: an unguarded drop discarded
/// everything, and this keeps everything. Both come from a condition the
/// generator could not express.
#[test]
fn no_more_drops_are_skipped_than_today() {
    /// Lines to read past the marker before deciding the block is not a drop.
    const WINDOW: usize = 8;

    let root = source_root();
    let mut files = Vec::new();
    rust_files(&root, &mut files);
    let scanned = files.len();

    let mut offenders = Vec::new();
    for path in files {
        let text = std::fs::read_to_string(&path).unwrap_or_default();
        let lines: Vec<&str> = text.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if !line.contains(MARKER) {
                continue;
            }
            let end = (i + 1 + WINDOW).min(lines.len());
            if lines[i + 1..end]
                .iter()
                .any(|l| l.contains("TransformResult::Drop"))
            {
                offenders.push(format!("{}:{}", path.display(), i + 1));
            }
        }
    }

    for site in &offenders {
        println!("drop never taken: {site}");
    }

    assert!(
        scanned >= FILES_SCANNED,
        "the walk covered {scanned} files, under the {FILES_SCANNED} floor"
    );
    assert!(
        offenders.len() <= DROPS_NEVER_TAKEN,
        "{} drops are now unreachable, up from {DROPS_NEVER_TAKEN} -- an event \
         the pipeline discards is being kept: {offenders:?}",
        offenders.len()
    );
}
