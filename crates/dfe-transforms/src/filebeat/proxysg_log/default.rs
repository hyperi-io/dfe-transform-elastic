// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("ecs.version", json!("8.17.0"))?;

            event.set("observer.vendor", json!("Broadcom"))?;

            event.set("observer.product", json!("ProxySG"))?;

            let _cond = { event.get_str("message").is_some_and(|s| s.starts_with("#")) };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if let Some(v) = event.get("message").cloned() {
                    event.set("event.original", v)?;
                }
            }

            event.rename("message", "_temp_.message")?;

            let _cond = { event.get_str("_temp_._conf") == Some("main") };
            if _cond {
                // Begin nested pipeline: "main"
                if let Some(csv_str) = event.get_string("_temp_.message") {
                    let csv_str = csv_close_quote_gap(&csv_str, ' ', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b' ')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("_temp_.date", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("_temp_.time", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("proxysg.time_taken", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("proxysg.client.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("proxysg.server_to_client.status", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("proxysg.server.action", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("proxysg.server_to_client.bytes", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.bytes", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.method", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.uri_scheme", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.host", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.uri_port", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.uri_path", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.uri_query", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.username", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.auth_group", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            if !val.is_empty() {
                                event.set("proxysg.server.supplier_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            if !val.is_empty() {
                                event.set("proxysg.remote_to_server.content_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.referer", val)?;
                            }
                        }
                        if let Some(val) = record.get(19) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.user_agent", val)?;
                            }
                        }
                        if let Some(val) = record.get(20) {
                            if !val.is_empty() {
                                event.set("proxysg.server_to_client.filter_result", val)?;
                            }
                        }
                        if let Some(val) = record.get(21) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.categories", val)?;
                            }
                        }
                        if let Some(val) = record.get(22) {
                            if !val.is_empty() {
                                event.set("proxysg.x_virus_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(23) {
                            if !val.is_empty() {
                                event.set("proxysg.server.ip", val)?;
                            }
                        }
                    }
                }
                // End nested pipeline: "main"
            }

            let _cond = { event.get_str("_temp_._conf") == Some("bcreportermain_v1") };
            if _cond {
                // Begin nested pipeline: "bcreportermain_v1"
                gsub_field(
                    event,
                    "_temp_.message",
                    "_temp_.message",
                    cached_regex!(" {2}"),
                    " ",
                )?;
                if let Some(csv_str) = event.get_string("_temp_.message") {
                    let csv_str = csv_close_quote_gap(&csv_str, ' ', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b' ')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("_temp_.date", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("_temp_.time", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("proxysg.time_taken", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("proxysg.client.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.username", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.auth_group", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("proxysg.server.supplier_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("proxysg.server.supplier_ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("proxysg.server.supplier_country", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("proxysg.server.supplier_failures", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("proxysg.x_exception_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set("proxysg.server_to_client.filter_result", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.categories", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.referer", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            if !val.is_empty() {
                                event.set("proxysg.server_to_client.status", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set("proxysg.server.action", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.method", val)?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.rs_content_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.uri_scheme", val)?;
                            }
                        }
                        if let Some(val) = record.get(19) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.host", val)?;
                            }
                        }
                        if let Some(val) = record.get(20) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.uri_port", val)?;
                            }
                        }
                        if let Some(val) = record.get(21) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.uri_path", val)?;
                            }
                        }
                        if let Some(val) = record.get(22) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.uri_query", val)?;
                            }
                        }
                        if let Some(val) = record.get(23) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.uri_extension", val)?;
                            }
                        }
                        if let Some(val) = record.get(24) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.user_agent", val)?;
                            }
                        }
                        if let Some(val) = record.get(25) {
                            if !val.is_empty() {
                                event.set("proxysg.server.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(26) {
                            if !val.is_empty() {
                                event.set("proxysg.server_to_client.bytes", val)?;
                            }
                        }
                        if let Some(val) = record.get(27) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.bytes", val)?;
                            }
                        }
                        if let Some(val) = record.get(28) {
                            if !val.is_empty() {
                                event.set("proxysg.x_virus_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(29) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.threat_source", val)?;
                            }
                        }
                        if let Some(val) = record.get(30) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.threat_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(31) {
                            if !val.is_empty() {
                                event.set("proxysg.remote_to_server.threat_source", val)?;
                            }
                        }
                        if let Some(val) = record.get(32) {
                            if !val.is_empty() {
                                event.set("proxysg.remote_to_server.threat_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(33) {
                            if !val.is_empty() {
                                event.set("proxysg.x_bluecoat.application_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(34) {
                            if !val.is_empty() {
                                event.set("proxysg.x_bluecoat.application_operation", val)?;
                            }
                        }
                        if let Some(val) = record.get(35) {
                            if !val.is_empty() {
                                event.set("proxysg.x_bluecoat.application_groups", val)?;
                            }
                        }
                        if let Some(val) = record.get(36) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.threat_risk", val)?;
                            }
                        }
                        if let Some(val) = record.get(37) {
                            if !val.is_empty() {
                                event
                                    .set("proxysg.x_bluecoat.access_security_policy_action", val)?;
                            }
                        }
                        if let Some(val) = record.get(38) {
                            if !val.is_empty() {
                                event
                                    .set("proxysg.x_bluecoat.access_security_policy_reason", val)?;
                            }
                        }
                        if let Some(val) = record.get(39) {
                            if !val.is_empty() {
                                event.set("proxysg.x_bluecoat.transaction_uuid", val)?;
                            }
                        }
                        if let Some(val) = record.get(40) {
                            if !val.is_empty() {
                                event.set("proxysg.x_icap_reqmod_header_x_icap_metadata", val)?;
                            }
                        }
                        if let Some(val) = record.get(41) {
                            if !val.is_empty() {
                                event.set("proxysg.x_icap_respmod_header_x_icap_metadata", val)?;
                            }
                        }
                    }
                }
                // End nested pipeline: "bcreportermain_v1"
            }

            let _cond = { event.get_str("_temp_._conf") == Some("bcreporterssl_v1") };
            if _cond {
                // Begin nested pipeline: "bcreporterssl_v1"
                if let Some(csv_str) = event.get_string("_temp_.message") {
                    let csv_str = csv_close_quote_gap(&csv_str, ' ', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b' ')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("_temp_.date", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("_temp_.time", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("proxysg.time_taken", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("proxysg.client.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.username", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.auth_group", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("proxysg.server.supplier_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("proxysg.server.supplier_ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("proxysg.server.supplier_country", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("proxysg.server.supplier_failures", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("proxysg.x_exception_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set("proxysg.server_to_client.filter_result", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.categories", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            if !val.is_empty() {
                                event.set("proxysg.server_to_client.status", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            if !val.is_empty() {
                                event.set("proxysg.server.action", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.method", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.rs_content_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.uri_scheme", val)?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.host", val)?;
                            }
                        }
                        if let Some(val) = record.get(19) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.uri_port", val)?;
                            }
                        }
                        if let Some(val) = record.get(20) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.uri_extension", val)?;
                            }
                        }
                        if let Some(val) = record.get(21) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.user_agent", val)?;
                            }
                        }
                        if let Some(val) = record.get(22) {
                            if !val.is_empty() {
                                event.set("proxysg.server.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(23) {
                            if !val.is_empty() {
                                event.set("proxysg.server_to_client.bytes", val)?;
                            }
                        }
                        if let Some(val) = record.get(24) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.bytes", val)?;
                            }
                        }
                        if let Some(val) = record.get(25) {
                            if !val.is_empty() {
                                event.set("proxysg.x_virus_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(26) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.threat_source", val)?;
                            }
                        }
                        if let Some(val) = record.get(27) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.threat_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(28) {
                            if !val.is_empty() {
                                event.set("proxysg.remote_to_server.threat_source", val)?;
                            }
                        }
                        if let Some(val) = record.get(29) {
                            if !val.is_empty() {
                                event.set("proxysg.remote_to_server.threat_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(30) {
                            if !val.is_empty() {
                                event.set(
                                    "proxysg.remote_to_server.certificate_observed_errors",
                                    val,
                                )?;
                            }
                        }
                        if let Some(val) = record.get(31) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.ocsp_error", val)?;
                            }
                        }
                        if let Some(val) = record.get(32) {
                            if !val.is_empty() {
                                event.set("proxysg.remote_to_server.ocsp_error", val)?;
                            }
                        }
                        if let Some(val) = record.get(33) {
                            if !val.is_empty() {
                                event.set("proxysg.remote_to_server.connection_negotiated_cipher_strength", val)?;
                            }
                        }
                        if let Some(val) = record.get(34) {
                            if !val.is_empty() {
                                event.set("proxysg.remote_to_server.certificate_hostname", val)?;
                            }
                        }
                        if let Some(val) = record.get(35) {
                            if !val.is_empty() {
                                event.set(
                                    "proxysg.remote_to_server.certificate_hostname_category",
                                    val,
                                )?;
                            }
                        }
                        if let Some(val) = record.get(36) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.threat_risk", val)?;
                            }
                        }
                        if let Some(val) = record.get(37) {
                            if !val.is_empty() {
                                event.set(
                                    "proxysg.remote_to_server.certificate_hostname_threat_risk",
                                    val,
                                )?;
                            }
                        }
                        if let Some(val) = record.get(38) {
                            if !val.is_empty() {
                                event
                                    .set("proxysg.x_bluecoat.access_security_policy_action", val)?;
                            }
                        }
                        if let Some(val) = record.get(39) {
                            if !val.is_empty() {
                                event
                                    .set("proxysg.x_bluecoat.access_security_policy_reason", val)?;
                            }
                        }
                    }
                }
                // End nested pipeline: "bcreporterssl_v1"
            }

            let _cond = { event.get_str("_temp_._conf") == Some("ssl") };
            if _cond {
                // Begin nested pipeline: "ssl"
                if let Some(csv_str) = event.get_string("_temp_.message") {
                    let csv_str = csv_close_quote_gap(&csv_str, ' ', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b' ')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("_temp_.date", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("_temp_.time", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("proxysg.time_taken", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("proxysg.client.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("proxysg.server.action", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set(
                                    "proxysg.remote_to_server.certificate_validate_status",
                                    val,
                                )?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set(
                                    "proxysg.remote_to_server.certificate_observed_errors",
                                    val,
                                )?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.ocsp_error", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("proxysg.remote_to_server.ocsp_error", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.host", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("proxysg.server.supplier_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set(
                                    "proxysg.remote_to_server.connection_negotiated_ssl_version",
                                    val,
                                )?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            if !val.is_empty() {
                                event.set(
                                    "proxysg.remote_to_server.connection_negotiated_cipher",
                                    val,
                                )?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            if !val.is_empty() {
                                event.set(
                                    "proxysg.remote_to_server.connection_negotiated_cipher_size",
                                    val,
                                )?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            if !val.is_empty() {
                                event.set("proxysg.remote_to_server.certificate_hostname", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set(
                                    "proxysg.remote_to_server.certificate_hostname_category",
                                    val,
                                )?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            if !val.is_empty() {
                                event.set(
                                    "proxysg.client_to_server.connection_negotiated_ssl_version",
                                    val,
                                )?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            if !val.is_empty() {
                                event.set(
                                    "proxysg.client_to_server.connection_negotiated_cipher",
                                    val,
                                )?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            if !val.is_empty() {
                                event.set(
                                    "proxysg.client_to_server.connection_negotiated_cipher_size",
                                    val,
                                )?;
                            }
                        }
                        if let Some(val) = record.get(19) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.certificate_subject", val)?;
                            }
                        }
                        if let Some(val) = record.get(20) {
                            if !val.is_empty() {
                                event.set("proxysg.server.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(21) {
                            if !val.is_empty() {
                                event.set("proxysg.server.sitename", val)?;
                            }
                        }
                    }
                }
                // End nested pipeline: "ssl"
            }

            let _cond = { event.has_value("_temp_.date") && event.has_value("_temp_.time") };
            if _cond {
                event.set(
                    "@timestamp",
                    json!(format!(
                        "{}T{}Z",
                        event
                            .get("_temp_.date")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_temp_.time")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("_temp_").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "_temp_".into(),
                    });
                }
                Ok(())
            })();

            // Painless script
            // Source: boolean dropUnsetFields(Object object) {\n  if (object == \"-\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropUnsetFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropUnsetFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropUnsetFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropUnsetFields(Object object) {\n  if (object == \"-\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropUnsetFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropUnsetFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropUnsetFields(ctx);\n"#
                ),
            )?;

            if event.has_value("proxysg.client_to_server.uri_port") {
                if let Some(val) = event.get("proxysg.client_to_server.uri_port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "proxysg.client_to_server.uri_port".into(),
                            message,
                        }
                    })?;
                    event.set("proxysg.client_to_server.uri_port", converted)?;
                }
            }

            if event.has_value("proxysg.server_to_client.bytes") {
                if let Some(val) = event.get("proxysg.server_to_client.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "proxysg.server_to_client.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("server.bytes", converted)?;
                }
            }

            if event.has_value("proxysg.client_to_server.bytes") {
                if let Some(val) = event.get("proxysg.client_to_server.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "proxysg.client_to_server.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("client.bytes", converted)?;
                }
            }

            if event.has_value("proxysg.server_to_client.status") {
                if let Some(val) = event.get("proxysg.server_to_client.status") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "proxysg.server_to_client.status".into(),
                            message,
                        }
                    })?;
                    event.set("http.response.status_code", converted)?;
                }
            }

            if event.has_value("proxysg.time_taken") {
                if let Some(val) = event.get("proxysg.time_taken") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "proxysg.time_taken".into(),
                            message,
                        }
                    })?;
                    event.set("proxysg.time_taken", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("proxysg.client.ip").cloned() {
                    event.set("client.ip", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("client.ip").cloned() {
                    event.set("client.address", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("proxysg.server.ip").cloned() {
                    event.set("server.ip", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("server.ip").cloned() {
                    event.set("server.address", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("proxysg.client_to_server.uri_scheme").cloned() {
                    event.set("url.scheme", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("proxysg.client_to_server.uri_port").cloned() {
                    event.set("url.port", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("proxysg.client_to_server.uri_path").cloned() {
                    event.set("url.path", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("proxysg.client_to_server.uri_query").cloned() {
                    event.set("url.query", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("proxysg.client_to_server.username").cloned() {
                    event.set("client.user.name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("proxysg.client_to_server.referer").cloned() {
                    event.set("http.request.referrer", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("proxysg.client_to_server.user_agent").cloned() {
                    event.set("user_agent.original", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("proxysg.client_to_server.method").cloned() {
                    event.set("http.request.method", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("proxysg.client_to_server.host").cloned() {
                    event.set("url.domain", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("proxysg.time_taken") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = ctx.proxysg.time_taken * 1000000\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(r#"ctx.event.duration = ctx.proxysg.time_taken * 1000000\n"#),
                )?;
            }

            if event.has_value("url.domain") {
                if let Some(domain_str) = event.get_string("url.domain") {
                    let domain = domain_str.to_string();
                    event.set("url.domain", json!(domain.clone()))?;
                    // Public suffix list lookup for registered domain extraction
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        if let Some(registered) = rd.registered_domain {
                            event.set("url.registered_domain", json!(registered))?;
                        }
                        event.set("url.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("url.subdomain", json!(sub))?;
                        }
                    }
                }
            }

            if event.has_value("user_agent.original") {
                if let Some(ua_str) = event.get_string("user_agent.original") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name {
                            event.set("user_agent.name", json!(name))?;
                        }
                        if let Some(version) = ua.version {
                            event.set("user_agent.version", json!(version))?;
                        }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set(
                                    "user_agent.os.full",
                                    json!(format!("{} {}", os_name, os_version)),
                                )?;
                            }
                        }
                        if let Some(device) = ua.device {
                            event.set("user_agent.device.name", json!(device))?;
                        }
                    }
                }
            }

            let _cond = { !event.has_value("server.geo") && event.has_value("server.ip") };
            if _cond {
                if let Some(ip_str) = event.get_string("server.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("server.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("server.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("server.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("server.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("server.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("server.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("server.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("server.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("server.ip") {
                if let Some(ip_str) = event.get_string("server.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("server.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("server.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("server.as.asn") {
                event.rename("server.as.asn", "server.as.number")?;
            }

            if event.has_value("server.as.organization_name") {
                event.rename("server.as.organization_name", "server.as.organization.name")?;
            }

            let _cond = { !event.has_value("client.geo") && event.has_value("client.ip") };
            if _cond {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("client.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("client.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("client.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("client.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("client.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("client.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("client.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("client.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("client.ip") {
                if let Some(ip_str) = event.get_string("client.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("client.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("client.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("client.as.asn") {
                event.rename("client.as.asn", "client.as.number")?;
            }

            if event.has_value("client.as.organization_name") {
                event.rename("client.as.organization_name", "client.as.organization.name")?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("server.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("client.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("proxysg.server.supplier_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("remote.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("url.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("client.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("_temp_").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "_temp_".into(),
                        });
                    }
                    Ok(())
                })();
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
