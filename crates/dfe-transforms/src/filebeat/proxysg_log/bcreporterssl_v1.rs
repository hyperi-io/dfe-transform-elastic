// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `bcreporterssl_v1` pipeline.
pub struct BcreportersslV1;

impl Transform for BcreportersslV1 {
    fn name(&self) -> &str {
        "bcreporterssl_v1"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
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
                                event.set("proxysg.remote_to_server.certificate_observed_errors", val)?;
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
                                event.set("proxysg.remote_to_server.certificate_hostname_category", val)?;
                            }
                        }
                        if let Some(val) = record.get(36) {
                            if !val.is_empty() {
                                event.set("proxysg.client_to_server.threat_risk", val)?;
                            }
                        }
                        if let Some(val) = record.get(37) {
                            if !val.is_empty() {
                                event.set("proxysg.remote_to_server.certificate_hostname_threat_risk", val)?;
                            }
                        }
                        if let Some(val) = record.get(38) {
                            if !val.is_empty() {
                                event.set("proxysg.x_bluecoat.access_security_policy_action", val)?;
                            }
                        }
                        if let Some(val) = record.get(39) {
                            if !val.is_empty() {
                                event.set("proxysg.x_bluecoat.access_security_policy_reason", val)?;
                            }
                        }
                    }
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.set("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
