// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! Compare every transform against what Elastic's own engine produced.
//!
//! `scripts/compat.py generate` runs raw source data through the real ingest
//! pipelines in a throwaway Elasticsearch container and writes the confirmed
//! documents to `testdata/compat/<source>/<data stream>/<fixture>/`. This test
//! reads those files. It starts no container and needs no network.
//!
//! The corpus derives from Elastic-Licensed pipelines, so it is not committed.
//! A missing corpus is reported and skipped -- unlike the committed fixtures,
//! where absence is a failure.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use dfe_runtime::event::Event;
use dfe_runtime::testutil::diff::{DiffKind, JsonDiff, MatchMode};
use dfe_runtime::testutil::{flatten_value, policy};
use dfe_runtime::transform::{Transform, TransformResult};
use dfe_transforms::filebeat;
use serde_json::Value;

/// Where `compat.py` writes by default. `DFE_COMPAT_CORPUS` overrides it, the
/// same variable the tool reads.
fn corpus_root() -> PathBuf {
    std::env::var_os("DFE_COMPAT_CORPUS").map_or_else(
        || {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../testdata/compat")
                .to_path_buf()
        },
        PathBuf::from,
    )
}

/// The transform a capture belongs to.
///
/// The corpus is keyed by Elastic's PACKAGE and data stream, which is not our
/// source name: the azure package holds four of our modules, and fortinet's
/// package is `fortinet_fortigate`. A capture with no transform here is a gap
/// worth failing on rather than skipping.
fn transform_for(package: &str, data_stream: &str) -> Option<&'static dyn Transform> {
    Some(match (package, data_stream) {
        ("azure", "activitylogs") => &filebeat::azure_activitylogs::default::Default,
        ("azure", "auditlogs") => &filebeat::azure_auditlogs::default::Default,
        ("azure", "platformlogs") => &filebeat::azure_platformlogs::default::Default,
        ("azure", "signinlogs") => &filebeat::azure_signinlogs::default::Default,
        // Every arm names its data stream; a package wildcard would swallow the
        // next stream onboarded under it.
        ("cisco_asa", "log") => &filebeat::cisco_asa::default::Default,
        ("cisco_ftd", "log") => &filebeat::cisco_ftd::default::Default,
        ("cisco_ios", "log") => &filebeat::cisco_ios::default::Default,
        ("cisco_meraki", "log") => &filebeat::cisco_meraki::default::Default,
        ("cisco_meraki", "events") => &filebeat::cisco_meraki_events::default::Default,
        ("cisco_nexus", "log") => &filebeat::cisco_nexus::default::Default,
        ("cisco_umbrella", "log") => &filebeat::cisco_umbrella::default::Default,
        ("crowdstrike", "falcon") => &filebeat::crowdstrike::default::Default,
        ("crowdstrike", "alert") => &filebeat::crowdstrike_alert::default::Default,
        ("crowdstrike", "host") => &filebeat::crowdstrike_host::default::Default,
        ("crowdstrike", "identity_protection_assessment") => {
            &filebeat::crowdstrike_identity_protection_assessment::default::Default
        }
        ("crowdstrike", "identity_protection_timeline") => {
            &filebeat::crowdstrike_identity_protection_timeline::default::Default
        }
        ("crowdstrike", "vulnerability") => &filebeat::crowdstrike_vulnerability::default::Default,
        ("fortinet_fortigate", "log") => &filebeat::fortinet::default::Default,
        ("microsoft_dnsserver", "analytical") => {
            &filebeat::microsoft_dnsserver_analytical::default::Default
        }
        ("microsoft_dnsserver", "audit") => &filebeat::microsoft_dnsserver_audit::default::Default,
        ("o365", "audit") => &filebeat::o365::default::Default,
        ("okta", "system") => &filebeat::okta::default::Default,
        ("panw", "panos") => &filebeat::panw::default::Default,
        ("entityanalytics_entra_id", "entity") => {
            &filebeat::entityanalytics_entra_id::default::Default
        }
        ("checkpoint", "firewall") => &filebeat::checkpoint::default::Default,
        ("microsoft_defender_endpoint", "log") => {
            &filebeat::microsoft_defender_endpoint_log::default::Default
        }
        ("microsoft_defender_endpoint", "machine") => {
            &filebeat::microsoft_defender_endpoint_machine::default::Default
        }
        ("microsoft_defender_endpoint", "machine_action") => {
            &filebeat::microsoft_defender_endpoint_machine_action::default::Default
        }
        ("microsoft_defender_endpoint", "vulnerability") => {
            &filebeat::microsoft_defender_endpoint_vulnerability::default::Default
        }
        ("proofpoint_on_demand", "audit") => {
            &filebeat::proofpoint_on_demand_audit::default::Default
        }
        ("proofpoint_on_demand", "mail") => &filebeat::proofpoint_on_demand_mail::default::Default,
        ("proofpoint_on_demand", "message") => {
            &filebeat::proofpoint_on_demand_message::default::Default
        }
        ("mimecast", "archive_search_logs") => {
            &filebeat::mimecast_archive_search_logs::default::Default
        }
        ("mimecast", "audit_events") => &filebeat::mimecast_audit_events::default::Default,
        ("mimecast", "cloud_integrated_logs") => {
            &filebeat::mimecast_cloud_integrated_logs::default::Default
        }
        ("mimecast", "dlp_logs") => &filebeat::mimecast_dlp_logs::default::Default,
        ("mimecast", "message_release_logs") => {
            &filebeat::mimecast_message_release_logs::default::Default
        }
        ("mimecast", "siem_logs") => &filebeat::mimecast_siem_logs::default::Default,
        ("mimecast", "threat_intel_malware_customer") => {
            &filebeat::mimecast_threat_intel_malware_customer::default::Default
        }
        ("mimecast", "threat_intel_malware_grid") => {
            &filebeat::mimecast_threat_intel_malware_grid::default::Default
        }
        ("mimecast", "ttp_ap_logs") => &filebeat::mimecast_ttp_ap_logs::default::Default,
        ("mimecast", "ttp_ip_logs") => &filebeat::mimecast_ttp_ip_logs::default::Default,
        ("mimecast", "ttp_url_logs") => &filebeat::mimecast_ttp_url_logs::default::Default,
        ("gcp", "audit") => &filebeat::gcp_audit::default::Default,
        ("gcp", "billing") => &filebeat::gcp_billing::default::Default,
        ("gcp", "cloudrun_metrics") => &filebeat::gcp_cloudrun_metrics::default::Default,
        ("gcp", "cloudsql_mysql") => &filebeat::gcp_cloudsql_mysql::default::Default,
        ("gcp", "cloudsql_postgresql") => &filebeat::gcp_cloudsql_postgresql::default::Default,
        ("gcp", "cloudsql_sqlserver") => &filebeat::gcp_cloudsql_sqlserver::default::Default,
        ("gcp", "compute") => &filebeat::gcp_compute::default::Default,
        ("gcp", "dataproc") => &filebeat::gcp_dataproc::default::Default,
        ("gcp", "dns") => &filebeat::gcp_dns::default::Default,
        ("gcp", "firestore") => &filebeat::gcp_firestore::default::Default,
        ("gcp", "gke") => &filebeat::gcp_gke::default::Default,
        ("gcp", "loadbalancing_logs") => &filebeat::gcp_loadbalancing_logs::default::Default,
        ("gcp", "loadbalancing_metrics") => &filebeat::gcp_loadbalancing_metrics::default::Default,
        ("gcp", "pubsub") => &filebeat::gcp_pubsub::default::Default,
        ("gcp", "redis") => &filebeat::gcp_redis::default::Default,
        ("gcp", "storage") => &filebeat::gcp_storage::default::Default,
        ("gcp", "firewall") => &filebeat::gcp_firewall::default::Default,
        ("gcp", "vpcflow") => &filebeat::gcp_vpcflow::default::Default,
        ("sentinel_one", "activity") => &filebeat::sentinel_one_activity::default::Default,
        ("sentinel_one", "agent") => &filebeat::sentinel_one_agent::default::Default,
        ("sentinel_one", "alert") => &filebeat::sentinel_one_alert::default::Default,
        ("sentinel_one", "application") => &filebeat::sentinel_one_application::default::Default,
        ("sentinel_one", "application_risk") => {
            &filebeat::sentinel_one_application_risk::default::Default
        }
        ("sentinel_one", "group") => &filebeat::sentinel_one_group::default::Default,
        ("sentinel_one", "threat_event") => &filebeat::sentinel_one_threat_event::default::Default,
        ("sentinel_one", "unified_alert") => {
            &filebeat::sentinel_one_unified_alert::default::Default
        }
        ("sentinel_one", "threat") => &filebeat::sentinel_one_threat::default::Default,
        ("windows", "applocker_exe_and_dll") => {
            &filebeat::windows_applocker_exe_and_dll::default::Default
        }
        ("windows", "applocker_msi_and_script") => {
            &filebeat::windows_applocker_msi_and_script::default::Default
        }
        ("windows", "applocker_packaged_app_deployment") => {
            &filebeat::windows_applocker_packaged_app_deployment::default::Default
        }
        ("windows", "applocker_packaged_app_execution") => {
            &filebeat::windows_applocker_packaged_app_execution::default::Default
        }
        ("windows", "forwarded") => &filebeat::windows_forwarded::default::Default,
        ("windows", "powershell") => &filebeat::windows_powershell::default::Default,
        ("windows", "powershell_operational") => {
            &filebeat::windows_powershell_operational::default::Default
        }
        ("windows", "sysmon_operational") => {
            &filebeat::windows_sysmon_operational::default::Default
        }
        ("windows", "windows_defender") => &filebeat::windows_windows_defender::default::Default,
        ("zscaler_zia", "alerts") => &filebeat::zscaler_zia_alerts::default::Default,
        ("zscaler_zia", "audit") => &filebeat::zscaler_zia_audit::default::Default,
        ("zscaler_zia", "email_dlp") => &filebeat::zscaler_zia_email_dlp::default::Default,
        ("zscaler_zia", "endpoint_dlp") => &filebeat::zscaler_zia_endpoint_dlp::default::Default,
        ("zscaler_zia", "saas_security") => &filebeat::zscaler_zia_saas_security::default::Default,
        ("zscaler_zia", "saas_security_activity") => {
            &filebeat::zscaler_zia_saas_security_activity::default::Default
        }
        ("zscaler_zia", "sandbox_report") => {
            &filebeat::zscaler_zia_sandbox_report::default::Default
        }
        ("zscaler_zia", "sandbox_verdict") => {
            &filebeat::zscaler_zia_sandbox_verdict::default::Default
        }
        ("zscaler_zia", "dns") => &filebeat::zscaler_zia_dns::default::Default,
        ("zscaler_zia", "firewall") => &filebeat::zscaler_zia_firewall::default::Default,
        ("zscaler_zia", "tunnel") => &filebeat::zscaler_zia_tunnel::default::Default,
        ("zscaler_zia", "web") => &filebeat::zscaler_zia_web::default::Default,
        ("m365_defender", "alert") => &filebeat::m365_defender_alert::default::Default,
        ("m365_defender", "incident") => &filebeat::m365_defender_incident::default::Default,
        ("m365_defender", "vulnerability") => {
            &filebeat::m365_defender_vulnerability::default::Default
        }
        ("m365_defender", "event") => &filebeat::m365_defender_event::default::Default,
        ("aws", "apigateway_logs") => &filebeat::aws_apigateway_logs::default::Default,
        ("aws", "awshealth") => &filebeat::aws_awshealth::default::Default,
        ("aws", "billing") => &filebeat::aws_billing::default::Default,
        ("aws", "cloudfront_logs") => &filebeat::aws_cloudfront_logs::default::Default,
        ("aws", "cloudtrail") => &filebeat::aws_cloudtrail::default::Default,
        ("aws", "cloudwatch_logs") => &filebeat::aws_cloudwatch_logs::default::Default,
        ("aws", "cloudwatch_metrics") => &filebeat::aws_cloudwatch_metrics::default::Default,
        ("aws", "config") => &filebeat::aws_config::default::Default,
        ("aws", "dynamodb") => &filebeat::aws_dynamodb::default::Default,
        ("aws", "ec2_logs") => &filebeat::aws_ec2_logs::default::Default,
        ("aws", "ec2_metrics") => &filebeat::aws_ec2_metrics::default::Default,
        ("aws", "elb_logs") => &filebeat::aws_elb_logs::default::Default,
        ("aws", "emr_logs") => &filebeat::aws_emr_logs::default::Default,
        ("aws", "guardduty") => &filebeat::aws_guardduty::default::Default,
        ("aws", "inspector") => &filebeat::aws_inspector::default::Default,
        ("aws", "kafka_metrics") => &filebeat::aws_kafka_metrics::default::Default,
        ("aws", "kinesis") => &filebeat::aws_kinesis::default::Default,
        ("aws", "lambda") => &filebeat::aws_lambda::default::Default,
        ("aws", "lambda_logs") => &filebeat::aws_lambda_logs::default::Default,
        ("aws", "natgateway") => &filebeat::aws_natgateway::default::Default,
        ("aws", "rds") => &filebeat::aws_rds::default::Default,
        ("aws", "redshift") => &filebeat::aws_redshift::default::Default,
        ("aws", "route53_public_logs") => &filebeat::aws_route53_public_logs::default::Default,
        ("aws", "route53_resolver_logs") => &filebeat::aws_route53_resolver_logs::default::Default,
        ("aws", "s3_daily_storage") => &filebeat::aws_s3_daily_storage::default::Default,
        ("aws", "s3_request") => &filebeat::aws_s3_request::default::Default,
        ("aws", "s3access") => &filebeat::aws_s3access::default::Default,
        ("aws", "securityhub_findings") => &filebeat::aws_securityhub_findings::default::Default,
        ("aws", "securityhub_findings_full_posture") => {
            &filebeat::aws_securityhub_findings_full_posture::default::Default
        }
        ("aws", "securityhub_insights") => &filebeat::aws_securityhub_insights::default::Default,
        ("aws", "sqs") => &filebeat::aws_sqs::default::Default,
        ("aws", "transitgateway") => &filebeat::aws_transitgateway::default::Default,
        ("aws", "usage") => &filebeat::aws_usage::default::Default,
        ("aws", "vpcflow") => &filebeat::aws_vpcflow::default::Default,
        ("aws", "vpn") => &filebeat::aws_vpn::default::Default,
        ("aws", "waf") => &filebeat::aws_waf::default::Default,
        ("aws", "firewall_logs") => &filebeat::aws_firewall_logs::default::Default,
        ("azure", "application_gateway") => &filebeat::azure_application_gateway::default::Default,
        ("azure", "eventhub") => &filebeat::azure_eventhub::default::Default,
        ("azure", "events") => &filebeat::azure_events::default::Default,
        ("azure", "firewall_logs") => &filebeat::azure_firewall_logs::default::Default,
        ("azure", "graphactivitylogs") => &filebeat::azure_graphactivitylogs::default::Default,
        ("azure", "identity_protection") => &filebeat::azure_identity_protection::default::Default,
        ("azure", "provisioning") => &filebeat::azure_provisioning::default::Default,
        ("azure", "springcloudlogs") => &filebeat::azure_springcloudlogs::default::Default,
        ("auth0", "logs") => &filebeat::auth0_logs::default::Default,
        ("coredns", "log") => &filebeat::coredns_log::default::Default,
        ("netflow", "log") => &filebeat::netflow_log::default::Default,
        _ => return None,
    })
}

/// One fixture's confirmed output, and where it came from.
struct Captured {
    source: String,
    data_stream: String,
    fixture: String,
    engine: String,
    /// The integrations commit the pipelines were taken from. A score is only
    /// comparable to another measured against the same one.
    integrations_sha: String,
    /// The pipeline `compat.py` installed. Anything not prefixed `compat-`
    /// was written by an earlier tool and the capture is stale.
    entry_pipeline: String,
    input: Vec<Value>,
    expected: Vec<Value>,
}

impl Captured {
    /// Whether Elastic's own run failed on every event.
    ///
    /// A uniform `pipeline_error` almost always means the input shape rather
    /// than the pipeline -- a fixture already in the Beats envelope wrapped a
    /// second time. Comparing against it reads as a broken transform.
    fn capture_failed(&self) -> bool {
        !self.expected.is_empty()
            && self
                .expected
                .iter()
                .all(|e| e.pointer("/event/kind").and_then(Value::as_str) == Some("pipeline_error"))
    }

    fn is_stale(&self) -> bool {
        !self.entry_pipeline.starts_with("compat-")
    }
}

fn read_ndjson(path: &Path) -> Vec<Value> {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

/// Every captured fixture under the corpus root.
fn captured() -> Vec<Captured> {
    let root = corpus_root();
    let mut out = Vec::new();

    let Ok(sources) = std::fs::read_dir(&root) else {
        return out;
    };
    for source in sources.flatten().filter(|e| e.path().is_dir()) {
        let source_name = source.file_name().to_string_lossy().into_owned();
        let Ok(streams) = std::fs::read_dir(source.path()) else {
            continue;
        };
        for stream in streams.flatten().filter(|e| e.path().is_dir()) {
            let stream_name = stream.file_name().to_string_lossy().into_owned();
            let Ok(fixtures) = std::fs::read_dir(stream.path()) else {
                continue;
            };
            for fixture in fixtures.flatten().filter(|e| e.path().is_dir()) {
                let dir = fixture.path();
                let input = read_ndjson(&dir.join("input.ndjson"));
                let expected = read_ndjson(&dir.join("expected.ndjson"));
                if input.is_empty() || expected.is_empty() {
                    continue;
                }
                let meta: Value = std::fs::read_to_string(dir.join("meta.json"))
                    .ok()
                    .and_then(|t| serde_json::from_str(&t).ok())
                    .unwrap_or(Value::Null);
                out.push(Captured {
                    source: source_name.clone(),
                    data_stream: stream_name.clone(),
                    fixture: fixture.file_name().to_string_lossy().into_owned(),
                    engine: meta
                        .get("elasticsearch_version")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown")
                        .to_string(),
                    integrations_sha: meta
                        .get("integrations_sha")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown")
                        .to_string(),
                    entry_pipeline: meta
                        .get("entry_pipeline")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                    input,
                    expected,
                });
            }
        }
    }
    out.sort_by(|a, b| (&a.source, &a.fixture).cmp(&(&b.source, &b.fixture)));
    out
}

/// A running count of events and of the fields inside them.
///
/// An event score alone cannot separate a transform that is one field short on
/// every event from one that is unrecognisably wrong, and the two need
/// completely different work. 20% of events at 96% of fields is nearly done;
/// 20% at 40% is not started.
#[derive(Default, Clone, Copy)]
struct Score {
    events: usize,
    events_matched: usize,
    events_errored: usize,
    /// Events Elastic's own pipeline failed on, so the capture holds only its
    /// failure and no expectation of correct output.
    events_unanswered: usize,
    fields: usize,
    fields_wrong: usize,
    fields_extra: usize,
}

impl Score {
    fn add(&mut self, other: Self) {
        self.events += other.events;
        self.events_matched += other.events_matched;
        self.events_errored += other.events_errored;
        self.events_unanswered += other.events_unanswered;
        self.fields += other.fields;
        self.fields_wrong += other.fields_wrong;
        self.fields_extra += other.fields_extra;
    }

    fn line(&self) -> String {
        let pct = |n: usize, d: usize| {
            if d == 0 {
                100.0
            } else {
                100.0 * n as f64 / d as f64
            }
        };
        let unanswered = if self.events_unanswered == 0 {
            String::new()
        } else {
            format!(", {} elastic-errored", self.events_unanswered)
        };
        format!(
            "events {}/{} ({:.0}%), fields {}/{} ({:.1}%), {} extra, {} errors{unanswered}",
            self.events_matched,
            self.events,
            pct(self.events_matched, self.events),
            self.fields - self.fields_wrong,
            self.fields,
            pct(self.fields - self.fields_wrong, self.fields),
            self.fields_extra,
            self.events_errored,
        )
    }
}

/// How many of an expected document's fields are actually compared.
///
/// The policy's skipped paths are not measured -- counting them would inflate
/// every field score by the same fixed amount and hide movement.
fn compared_field_count(source: &str, expected: &Value) -> usize {
    flatten_value(expected)
        .keys()
        .filter(|path| !policy().skips(Some(source), path))
        .count()
}

/// One entry in the events-unlocked ranking.
struct Blocker {
    path: String,
    /// Failing events this path is wrong in.
    appears: usize,
    /// Events that would pass once this path AND everything above it is fixed.
    unlocks: usize,
}

/// The fields whose repair would unlock the most events.
///
/// Greedy set cover over the failing events: take the path wrong in the most
/// of them, count the events it finishes off, strip it, and repeat. Ranking by
/// raw frequency instead conflates "appears often" with "worth fixing" -- a
/// path wrong in 400 events that are ALSO wrong in four other places unlocks
/// nothing on its own, and `unlocks` is what says so.
fn events_unlocked(failures: &[BTreeSet<String>], top: usize) -> Vec<Blocker> {
    let mut events: Vec<BTreeSet<String>> = failures.to_vec();
    let mut ranked = Vec::new();

    while ranked.len() < top {
        events.retain(|event| !event.is_empty());
        if events.is_empty() {
            break;
        }

        let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
        for event in &events {
            for path in event {
                *counts.entry(path.as_str()).or_default() += 1;
            }
        }

        // Ties break on the lexicographically first path, so the ranking is the
        // same on every run and a diff of two reports means something.
        let Some((path, appears)) = counts
            .iter()
            .max_by_key(|(path, count)| (**count, std::cmp::Reverse(*path)))
            .map(|(path, count)| ((*path).to_owned(), *count))
        else {
            break;
        };

        let unlocks = events
            .iter()
            .filter(|event| event.len() == 1 && event.contains(path.as_str()))
            .count();
        for event in &mut events {
            event.remove(path.as_str());
        }
        ranked.push(Blocker {
            path,
            appears,
            unlocks,
        });
    }
    ranked
}

/// Report how each transform compares against Elastic's confirmed output.
///
/// Reporting, not ratcheting: the corpus is regenerated against whichever
/// engine and integrations commit the operator chose, so a number here is only
/// comparable to another run over the same corpus. `scripts/compat.py audit`
/// is where the tracked figures live.
#[test]
fn transforms_match_elastics_confirmed_output() {
    let fixtures = captured();
    if fixtures.is_empty() {
        println!(
            "no compat corpus at {} -- run `python3 scripts/compat.py generate --all`",
            corpus_root().display()
        );
        return;
    }

    // The corpus was captured with MaxMind's databases and ours are DB-IP
    // Lite, so every geoip-derived field is excluded from the comparison. The
    // enrichment's SIDE EFFECTS are not excluded, though: an ASN hit MaxMind
    // does not have makes gcp/vpcflow's `source.as.asn` rename land on an
    // occupied `source.as.number`, which fails the document and skips the
    // twenty-nine removes behind it. Comparing against output built from a
    // database we do not have means running without one.
    assert!(
        dfe_runtime::enrichment::geoip_global::disable(),
        "a lookup has already loaded the databases -- disable must come first"
    );

    // Whole-diff output for the named sources, comma-separated. Printing every
    // difference for every source buries the summary the ranking exists to give.
    let detail: BTreeSet<String> = std::env::var("DFE_COMPAT_DETAIL")
        .unwrap_or_default()
        .split(',')
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect();
    let dump = std::env::var("DFE_COMPAT_DUMP").ok();

    // Score only the named sources, comma-separated. For the inner loop while
    // one source is being worked on; the ratchet is SKIPPED under it, because a
    // partial run cannot say whether another source went down.
    let only: BTreeSet<String> = std::env::var("DFE_COMPAT_ONLY")
        .unwrap_or_default()
        .split(',')
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .collect();

    let mut unmapped = Vec::new();
    let mut by_source: BTreeMap<String, Score> = BTreeMap::new();
    let mut failures: BTreeMap<String, Vec<BTreeSet<String>>> = BTreeMap::new();
    let mut total = Score::default();

    for capture in &fixtures {
        if !only.is_empty() && !only.contains(&capture.source) {
            continue;
        }
        let Some(transform) = transform_for(&capture.source, &capture.data_stream) else {
            unmapped.push(format!("{}/{}", capture.source, capture.data_stream));
            continue;
        };

        // Say why a capture is not worth comparing BEFORE printing a score
        // against it, or a zero reads as a broken transform.
        if capture.is_stale() {
            println!(
                "[{}/{}] SKIPPED: written by an earlier tool ({}), regenerate it",
                capture.source, capture.fixture, capture.entry_pipeline,
            );
            continue;
        }
        if capture.capture_failed() {
            println!(
                "[{}/{}] SKIPPED: Elastic errored on all {} events, so the capture \
                 carries no expectation -- check the input shape",
                capture.source,
                capture.fixture,
                capture.expected.len(),
            );
            continue;
        }

        let mut score = Score::default();
        for (i, raw) in capture.input.iter().enumerate() {
            let Some(expected) = capture.expected.get(i) else {
                continue;
            };
            // Elastic's own pipeline failed on this one, so the capture holds
            // its failure and no expectation of correct output. Scoring
            // against it counts our CORRECT output as a miss. The bare
            // `_compat_error` OBJECT is the same thing one layer down --
            // simulate errored with no on_failure to shape a document.
            if expected.pointer("/event/kind").and_then(Value::as_str) == Some("pipeline_error")
                || expected.get("_compat_error").is_some_and(Value::is_object)
            {
                score.events_unanswered += 1;
                continue;
            }

            // Elasticsearch returned NOTHING for this document -- its
            // pipeline dropped it, and the capture records the marker so the
            // corpus stays aligned. The matching outcome is our transform
            // dropping it too, scored as one whole-event field.
            let expected_drop =
                expected.get("_compat_error").and_then(Value::as_str) == Some("no result");

            score.events += 1;
            score.fields += if expected_drop {
                1
            } else {
                compared_field_count(&capture.source, expected)
            };

            let mut event = Event::new(raw.clone());
            match transform.transform(&mut event) {
                Err(_) => {
                    score.events_errored += 1;
                    continue;
                }
                Ok(TransformResult::Drop) if expected_drop => {
                    score.events_matched += 1;
                    continue;
                }
                Ok(TransformResult::Drop) => {
                    // Dropped an event Elastic kept: every expected field is
                    // gone, and the ranking hears about it under one name.
                    score.fields_wrong += compared_field_count(&capture.source, expected);
                    failures
                        .entry(capture.source.clone())
                        .or_default()
                        .push(BTreeSet::from(["_dropped".to_string()]));
                    continue;
                }
                Ok(_) if expected_drop => {
                    // Kept an event Elastic dropped.
                    score.fields_wrong += 1;
                    failures
                        .entry(capture.source.clone())
                        .or_default()
                        .push(BTreeSet::from(["_not_dropped".to_string()]));
                    continue;
                }
                Ok(_) => {}
            }

            let diff = JsonDiff::compare_for(
                Some(&capture.source),
                expected,
                event.as_value(),
                MatchMode::Semantic,
            );
            if diff.is_match() {
                score.events_matched += 1;
                continue;
            }

            if detail.contains(&capture.source) {
                println!("  {}[{i}]: {diff}", capture.fixture);
            }
            // A diff names the fields that disagree; it does not say what ELSE
            // the transform wrote, which is where a stray value's real source
            // shows up. `DFE_COMPAT_DUMP=<fixture>` prints the whole document.
            if dump.as_deref() == Some(capture.fixture.as_str()) {
                println!(
                    "  {}[{i}] GOT: {}",
                    capture.fixture,
                    serde_json::to_string(event.as_value()).unwrap_or_default()
                );
            }

            let mut paths = BTreeSet::new();
            for field in &diff.diffs {
                match field.kind {
                    DiffKind::Extra { .. } => score.fields_extra += 1,
                    DiffKind::Missing { .. } | DiffKind::Mismatch { .. } => score.fields_wrong += 1,
                }
                paths.insert(field.path.clone());
            }
            failures
                .entry(capture.source.clone())
                .or_default()
                .push(paths);
        }

        println!(
            "[{}/{}] {} (es {})",
            capture.source,
            capture.fixture,
            score.line(),
            capture.engine,
        );
        by_source
            .entry(capture.source.clone())
            .or_default()
            .add(score);
        total.add(score);
    }

    println!("\n=== per source ===");
    for (source, score) in &by_source {
        println!("{source:<20} {}", score.line());
        for blocker in events_unlocked(failures.get(source).map_or(&[], Vec::as_slice), 6) {
            println!(
                "      wrong in {:>5}, unlocks {:>5}   {}",
                blocker.appears, blocker.unlocks, blocker.path
            );
        }
    }
    println!("\n{:<20} {}", "TOTAL", total.line());

    assert!(
        unmapped.is_empty(),
        "the corpus holds sources with no transform mapped in this test: {unmapped:?}"
    );

    // A partial run cannot say whether another source went down, so it reports
    // and never ratchets. The whole-corpus run stays the only gate.
    if only.is_empty() {
        check_baseline(&by_source, &provenance(&fixtures));
    } else {
        println!("\nDFE_COMPAT_ONLY is set, so the baseline was NOT checked");
    }
}

/// The integrations commit and engine version the whole corpus was taken at.
///
/// `None` when the captures disagree, which means the corpus was regenerated
/// piecemeal and no single baseline describes it.
fn provenance(fixtures: &[Captured]) -> Option<(String, String)> {
    let mut seen: Option<(String, String)> = None;
    for capture in fixtures {
        let here = (capture.integrations_sha.clone(), capture.engine.clone());
        match &seen {
            None => seen = Some(here),
            Some(first) if *first == here => {}
            Some(_) => return None,
        }
    }
    seen
}

/// Per-source scores this corpus is expected to reach.
#[derive(serde::Deserialize)]
struct Baseline {
    integrations_sha: String,
    elasticsearch_version: String,
    sources: BTreeMap<String, Expected>,
}

#[derive(serde::Deserialize)]
struct Expected {
    events: usize,
    events_total: usize,
    fields_wrong: usize,
}

/// The baseline entry a measured score is written as.
fn entry(score: Score) -> String {
    format!(
        "{{ \"events\": {}, \"events_total\": {}, \"fields_wrong\": {} }}",
        score.events_matched, score.events, score.fields_wrong
    )
}

/// Fail if a source scores below what it scored when the baseline was written.
///
/// Only asserted when the corpus on disk was captured at the same integrations
/// commit and engine version the baseline names. Anything else is reported --
/// a score against different pipelines is a different measurement, and
/// ratcheting one against the other would fail for the wrong reason.
fn check_baseline(measured: &BTreeMap<String, Score>, provenance: &Option<(String, String)>) {
    const PATH: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/compat-baseline.json"
    );
    let Ok(text) = std::fs::read_to_string(PATH) else {
        println!("\nno baseline at {PATH}");
        return;
    };
    let baseline: Baseline = serde_json::from_str(&text).expect("parse compat-baseline.json");

    let Some((sha, engine)) = provenance else {
        println!("\nbaseline NOT asserted: the corpus mixes provenances, so regenerate it whole");
        return;
    };
    if *sha != baseline.integrations_sha || *engine != baseline.elasticsearch_version {
        println!(
            "\nbaseline NOT asserted: corpus is {}/{engine}, baseline is {}/{}",
            &sha[..12.min(sha.len())],
            &baseline.integrations_sha[..12.min(baseline.integrations_sha.len())],
            baseline.elasticsearch_version,
        );
        return;
    }

    let mut failures = Vec::new();
    let mut improved = Vec::new();
    let mut resized = Vec::new();
    for (source, expected) in &baseline.sources {
        let Some(score) = measured.get(source) else {
            failures.push(format!(
                "{source}: in the baseline and absent from the corpus"
            ));
            continue;
        };
        // A different set of events is a different measurement: neither the
        // event floor nor the field count carries across it, so the entry
        // constrains nothing until it is rewritten -- the same consequence as
        // a source nobody wrote down, and so the same failure.
        if score.events != expected.events_total {
            failures.push(format!(
                "{source}: {} events in the corpus, baseline was written against {} -- \
                 nothing constrains it until the entry is rewritten",
                score.events, expected.events_total
            ));
            resized.push(format!("RESIZE  \"{source}\": {},", entry(*score)));
            continue;
        }
        if score.events_matched < expected.events {
            failures.push(format!(
                "{source}: {}/{} events, baseline {}",
                score.events_matched, score.events, expected.events
            ));
        }
        if score.fields_wrong > expected.fields_wrong {
            failures.push(format!(
                "{source}: {} fields wrong, baseline {}",
                score.fields_wrong, expected.fields_wrong
            ));
        }
        if score.events_matched > expected.events || score.fields_wrong < expected.fields_wrong {
            improved.push(format!("  \"{source}\": {},", entry(*score)));
        }
    }

    // A source scored and never written down is measured by nobody: it may
    // rot to zero without failing anything. Capturing one is not finished
    // until its score is a floor, so this is a failure with the line to paste.
    for (source, score) in measured {
        if !baseline.sources.contains_key(source) {
            failures.push(format!(
                "{source}: scored and not in the baseline -- add {}",
                entry(*score)
            ));
        }
    }

    if !improved.is_empty() {
        println!("\nbaseline can be raised -- paste into tests/compat-baseline.json:");
        for line in &improved {
            println!("{line}");
        }
    }

    // Marked rather than paste-ready: rewriting one of these accepts a score
    // nothing was holding, so it goes through `raise_baseline.py --resize`,
    // which the plain raise run will not do for you.
    if !resized.is_empty() {
        println!(
            "\nthese hold a different set of events than the baseline was written \
             against -- rewrite with scripts/raise_baseline.py --resize once the new \
             total is the intended one:"
        );
        for line in &resized {
            println!("{line}");
        }
    }

    assert!(
        failures.is_empty(),
        "the corpus regressed against tests/compat-baseline.json:\n  {}",
        failures.join("\n  ")
    );
}

/// Every package and data stream `compat.py` captures must reach a transform.
#[test]
fn every_captured_source_is_a_registered_transform() {
    for capture in captured() {
        assert!(
            transform_for(&capture.source, &capture.data_stream).is_some(),
            "{}/{} is in the corpus with no transform",
            capture.source,
            capture.data_stream
        );
    }
}

/// The same check with no corpus on disk. The corpus is gitignored, so the
/// test above is vacuous on a fresh clone and a newly declared source would be
/// silently skipped rather than scored.
#[test]
fn every_declared_source_reaches_a_transform() {
    #[derive(serde::Deserialize)]
    struct Declaration {
        sources: std::collections::BTreeMap<String, Declared>,
    }

    #[derive(serde::Deserialize)]
    struct Declared {
        package: String,
        data_stream: String,
    }

    const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../sources.yaml");
    let text = std::fs::read_to_string(PATH).expect("read sources.yaml");
    let declaration: Declaration = serde_yaml_ng::from_str(&text).expect("parse sources.yaml");

    for (name, declared) in &declaration.sources {
        assert!(
            transform_for(&declared.package, &declared.data_stream).is_some(),
            "{name} is declared as {}/{} and `transform_for` does not map it",
            declared.package,
            declared.data_stream
        );
    }
}

/// No arm of `transform_for` may match a data stream by package alone.
///
/// A wildcard answers for streams that do not exist yet, so a newly onboarded
/// one is scored against the wrong transform while the checks above stay green.
#[test]
fn no_package_answers_for_a_stream_it_was_not_declared_with() {
    #[derive(serde::Deserialize)]
    struct Declaration {
        sources: std::collections::BTreeMap<String, Declared>,
    }

    #[derive(serde::Deserialize)]
    struct Declared {
        package: String,
    }

    const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../sources.yaml");
    let text = std::fs::read_to_string(PATH).expect("read sources.yaml");
    let declaration: Declaration = serde_yaml_ng::from_str(&text).expect("parse sources.yaml");

    let packages: std::collections::BTreeSet<&str> = declaration
        .sources
        .values()
        .map(|declared| declared.package.as_str())
        .collect();

    // Every offender is reported, because narrowing one arm at a time costs a
    // build per package.
    let wildcards: Vec<&str> = packages
        .into_iter()
        .filter(|package| transform_for(package, "a_data_stream_that_does_not_exist").is_some())
        .collect();

    assert!(
        wildcards.is_empty(),
        "these packages match ANY data stream -- narrow each arm to the streams \
         it is really for: {wildcards:?}"
    );
}
