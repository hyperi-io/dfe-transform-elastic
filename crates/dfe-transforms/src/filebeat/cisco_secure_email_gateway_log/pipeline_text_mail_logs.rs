// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_text_mail_logs` pipeline.
pub struct PipelineTextMailLogs;

impl Transform for PipelineTextMailLogs {
    fn name(&self) -> &str {
        "pipeline_text_mail_logs"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

                if let Some(input) = event.get_string("cisco_secure_email_gateway.log.message") {
                    // Grok pattern: ^graymail \\[CONFIG\\] %{WORD:cisco_secure_email_gateway.log.vendor_action} %{GREEDYDATA:cisco_secure_email_gateway.log.object}$
                    // Grok pattern: ^URL_REP_CLIENT: %{WORD:cisco_secure_email_gateway.log.object_attr} %{DATA:cisco_secure_email_gateway.log.type}. Triggering %{WORD:cisco_secure_email_gateway.log.vendor_action} of %{GREEDYDATA:cisco_secure_email_gateway.log.object}\\.$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{GREEDYDATA:cisco_secure_email_gateway.log.subject}. Severity: %{WORD:cisco_secure_email_gateway.log.severity} \\(Risk Factor: %{NUMBER:cisco_secure_email_gateway.log.risk_factor:long}\\). %{GREEDYDATA:cisco_secure_email_gateway.log.description}$
                    // Grok pattern: ^A System/Warning %{DATA:event.kind} was sent to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$
                    // Grok pattern: ^%{WORD:cisco_secure_email_gateway.log.connection_status} %{WORD:network.protocol} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} interface Management \\(%{IP:cisco_secure_email_gateway.log.interface}\\) address %{IP:cisco_secure_email_gateway.log.address} reverse dns host %{DATA:dns.question.name} verified %{WORD:cisco_secure_email_gateway.log.verified}$
                    // Grok pattern: ^%{WORD:cisco_secure_email_gateway.log.connection_status} MID %{NUMBER:email.message_id} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} From: <%{DATA:email.from.address}>$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} RID %{DATA:cisco_secure_email_gateway.log.recipient_id} To: <%{DATA:email.to.address}>$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} ready %{NUMBER:cisco_secure_email_gateway.log.read_bytes:long} bytes from <%{DATA:email.from.address}>$
                    // Grok pattern: ^ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} %{WORD:cisco_secure_email_gateway.log.connection_status}$
                    // Grok pattern: ^%{DATA:cisco_secure_email_gateway.log.message_status} %{WORD:network.protocol} DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} interface %{IP:cisco_secure_email_gateway.log.interface} address %{IP:cisco_secure_email_gateway.log.address}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message_status} DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} MID %{NUMBER:email.message_id} to RID \\[%{DATA:cisco_secure_email_gateway.log.recipient_id}\\](\\s\\[%{DATA:cisco_secure_email_gateway.log.email_participants}\\])?$
                    // Grok pattern: ^DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} %{WORD:cisco_secure_email_gateway.log.connection_status}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.description}: Unable to send System/Warning %{DATA:event.kind} to %{DATA:email.to.address} with subject \"%{GREEDYDATA:email.subject}\"\\.$
                    // Grok pattern: ^Internal %{DATA:network.protocol} system successfully sent a message to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$
                    // Grok pattern: ^Internal %{DATA:network.protocol} giving up on message to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$
                    // Grok pattern: ^Internal %{DATA:network.protocol} Error: %{GREEDYDATA:cisco_secure_email_gateway.log.description} to host %{IP:destination.ip}:%{NUMBER:destination.port:long} for recipient %{DATA:email.to.address}: %{GREEDYDATA:cisco_secure_email_gateway.log.subject}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} RID \\[%{DATA:cisco_secure_email_gateway.log.recipient_id}\\] Response %{GREEDYDATA:cisco_secure_email_gateway.log.response}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} Subject \"%{GREEDYDATA:cisco_secure_email_gateway.log.subject}\"$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{GREEDYDATA:cisco_secure_email_gateway.log.message_status} for delivery$
                    // Grok pattern: ^Message %{GREEDYDATA:cisco_secure_email_gateway.log.message_status} MID %{NUMBER:email.message_id} done$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} interim verdict using engine: %{GREEDYDATA:cisco_secure_email_gateway.log.engine}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} interim AV verdict using %{GREEDYDATA:cisco_secure_email_gateway.log.engine}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} using engine: %{GREEDYDATA:cisco_secure_email_gateway.log.engine}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{GREEDYDATA:cisco_secure_email_gateway.log.subject}: verdict %{GREEDYDATA:cisco_secure_email_gateway.log.verdict_scale}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} Message-ID '\\<%{GREEDYDATA:cisco_secure_email_gateway.log.email}\\>'$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: Verification %{GREEDYDATA:cisco_secure_email_gateway.log.verified}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: mailfrom identity %{DATA:email.from.address} %{GREEDYDATA:cisco_secure_email_gateway.log.verified} \\(v\\=spf1\\)$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} matched all recipients for per-recipient policy %{WORD:cisco_secure_email_gateway.log.policy} in the %{DATA:email.direction} table$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} SDR: Tracker Header : %{GREEDYDATA:cisco_secure_email_gateway.log.email_tracker_header}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} SDR: Domains for which SDR is requested: reverse DNS host: %{DATA:dns.question.name}, helo: %{DATA:cisco_secure_email_gateway.log.helo}, env-from: %{DATA:cisco_secure_email_gateway.log.env}, header-from: %{DATA:email.from.address}, reply-to: %{DATA:email.to.address}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} SDR: Consolidated Sender Threat Level: %{DATA:cisco_secure_email_gateway.log.threat_level}, Threat Category: %{DATA:cisco_secure_email_gateway.log.threat_category}, Suspected Domain\\(s\\) : %{GREEDYDATA:cisco_secure_email_gateway.log.suspected_domains} \\(other reasons for verdict\\)\\. Sender Maturity: %{GREEDYDATA:cisco_secure_email_gateway.log.maturity} for domain: %{DATA:cisco_secure_email_gateway.log.domain}$
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}\\. %{GREEDYDATA:cisco_secure_email_gateway.log.description}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: Message from domain %{DATA:email.from.address}, DMARC %{DATA:cisco_secure_email_gateway.log.verified} \\(SPF aligned %{DATA:cisco_secure_email_gateway.log.spf_aligned}, DKIM aligned %{DATA:cisco_secure_email_gateway.log.dkim_aligned}\\)$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: pass signature %{DATA:cisco_secure_email_gateway.log.verified} \\(%{GREEDYDATA:cisco_secure_email_gateway.log.details}\\)$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} SHA %{BASE16NUM:file.hash.sha256} filename %{GREEDYDATA:file.name}\\.%{WORD:file.extension} %{GREEDYDATA:cisco_secure_email_gateway.log.details}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} SHA %{BASE16NUM:file.hash.sha256} filename %{DATA:file.name} %{GREEDYDATA:cisco_secure_email_gateway.log.details}$
                    // Grok pattern: ^MID %{NUMBER:email.message_id} %{DATA:file.extension} file %{GREEDYDATA:cisco_secure_email_gateway.log.details}$
                    // Grok pattern: \\bMID %{NUMBER:email.message_id}(?: ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id})?
                    // Grok pattern: ^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!("^graymail \\[CONFIG\\] %{WORD:cisco_secure_email_gateway.log.vendor_action} %{GREEDYDATA:cisco_secure_email_gateway.log.object}$"),
                            cached_grok!("^URL_REP_CLIENT: %{WORD:cisco_secure_email_gateway.log.object_attr} %{DATA:cisco_secure_email_gateway.log.type}. Triggering %{WORD:cisco_secure_email_gateway.log.vendor_action} of %{GREEDYDATA:cisco_secure_email_gateway.log.object}\\.$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} %{GREEDYDATA:cisco_secure_email_gateway.log.subject}. Severity: %{WORD:cisco_secure_email_gateway.log.severity} \\(Risk Factor: %{NUMBER:cisco_secure_email_gateway.log.risk_factor:long}\\). %{GREEDYDATA:cisco_secure_email_gateway.log.description}$"),
                            cached_grok!("^A System/Warning %{DATA:event.kind} was sent to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$"),
                            cached_grok!("^%{WORD:cisco_secure_email_gateway.log.connection_status} %{WORD:network.protocol} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} interface Management \\(%{IP:cisco_secure_email_gateway.log.interface}\\) address %{IP:cisco_secure_email_gateway.log.address} reverse dns host %{DATA:dns.question.name} verified %{WORD:cisco_secure_email_gateway.log.verified}$"),
                            cached_grok!("^%{WORD:cisco_secure_email_gateway.log.connection_status} MID %{NUMBER:email.message_id} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id}$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} From: <%{DATA:email.from.address}>$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} RID %{DATA:cisco_secure_email_gateway.log.recipient_id} To: <%{DATA:email.to.address}>$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} ready %{NUMBER:cisco_secure_email_gateway.log.read_bytes:long} bytes from <%{DATA:email.from.address}>$"),
                            cached_grok!("^ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id} %{WORD:cisco_secure_email_gateway.log.connection_status}$"),
                            cached_grok!("^%{DATA:cisco_secure_email_gateway.log.message_status} %{WORD:network.protocol} DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} interface %{IP:cisco_secure_email_gateway.log.interface} address %{IP:cisco_secure_email_gateway.log.address}$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message_status} DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} MID %{NUMBER:email.message_id} to RID \\[%{DATA:cisco_secure_email_gateway.log.recipient_id}\\](\\s\\[%{DATA:cisco_secure_email_gateway.log.email_participants}\\])?$"),
                            cached_grok!("^DCID %{NUMBER:cisco_secure_email_gateway.log.delivery_connection_id} %{WORD:cisco_secure_email_gateway.log.connection_status}$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.description}: Unable to send System/Warning %{DATA:event.kind} to %{DATA:email.to.address} with subject \"%{GREEDYDATA:email.subject}\"\\.$"),
                            cached_grok!("^Internal %{DATA:network.protocol} system successfully sent a message to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$"),
                            cached_grok!("^Internal %{DATA:network.protocol} giving up on message to %{DATA:email.to.address} with subject %{GREEDYDATA:email.subject}\\.$"),
                            cached_grok!("^Internal %{DATA:network.protocol} Error: %{GREEDYDATA:cisco_secure_email_gateway.log.description} to host %{IP:destination.ip}:%{NUMBER:destination.port:long} for recipient %{DATA:email.to.address}: %{GREEDYDATA:cisco_secure_email_gateway.log.subject}$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} RID \\[%{DATA:cisco_secure_email_gateway.log.recipient_id}\\] Response %{GREEDYDATA:cisco_secure_email_gateway.log.response}$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} Subject \"%{GREEDYDATA:cisco_secure_email_gateway.log.subject}\"$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} %{GREEDYDATA:cisco_secure_email_gateway.log.message_status} for delivery$"),
                            cached_grok!("^Message %{GREEDYDATA:cisco_secure_email_gateway.log.message_status} MID %{NUMBER:email.message_id} done$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} interim verdict using engine: %{GREEDYDATA:cisco_secure_email_gateway.log.engine}$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} interim AV verdict using %{GREEDYDATA:cisco_secure_email_gateway.log.engine}$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} using engine: %{GREEDYDATA:cisco_secure_email_gateway.log.engine}$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} %{GREEDYDATA:cisco_secure_email_gateway.log.subject}: verdict %{GREEDYDATA:cisco_secure_email_gateway.log.verdict_scale}$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} Message-ID '\\<%{GREEDYDATA:cisco_secure_email_gateway.log.email}\\>'$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: Verification %{GREEDYDATA:cisco_secure_email_gateway.log.verified}$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: mailfrom identity %{DATA:email.from.address} %{GREEDYDATA:cisco_secure_email_gateway.log.verified} \\(v\\=spf1\\)$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} matched all recipients for per-recipient policy %{WORD:cisco_secure_email_gateway.log.policy} in the %{DATA:email.direction} table$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} SDR: Tracker Header : %{GREEDYDATA:cisco_secure_email_gateway.log.email_tracker_header}$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} SDR: Domains for which SDR is requested: reverse DNS host: %{DATA:dns.question.name}, helo: %{DATA:cisco_secure_email_gateway.log.helo}, env-from: %{DATA:cisco_secure_email_gateway.log.env}, header-from: %{DATA:email.from.address}, reply-to: %{DATA:email.to.address}$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} SDR: Consolidated Sender Threat Level: %{DATA:cisco_secure_email_gateway.log.threat_level}, Threat Category: %{DATA:cisco_secure_email_gateway.log.threat_category}, Suspected Domain\\(s\\) : %{GREEDYDATA:cisco_secure_email_gateway.log.suspected_domains} \\(other reasons for verdict\\)\\. Sender Maturity: %{GREEDYDATA:cisco_secure_email_gateway.log.maturity} for domain: %{DATA:cisco_secure_email_gateway.log.domain}$"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.subject}\\. %{GREEDYDATA:cisco_secure_email_gateway.log.description}$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: Message from domain %{DATA:email.from.address}, DMARC %{DATA:cisco_secure_email_gateway.log.verified} \\(SPF aligned %{DATA:cisco_secure_email_gateway.log.spf_aligned}, DKIM aligned %{DATA:cisco_secure_email_gateway.log.dkim_aligned}\\)$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} %{DATA:network.protocol}: pass signature %{DATA:cisco_secure_email_gateway.log.verified} \\(%{GREEDYDATA:cisco_secure_email_gateway.log.details}\\)$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} SHA %{BASE16NUM:file.hash.sha256} filename %{GREEDYDATA:file.name}\\.%{WORD:file.extension} %{GREEDYDATA:cisco_secure_email_gateway.log.details}$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} SHA %{BASE16NUM:file.hash.sha256} filename %{DATA:file.name} %{GREEDYDATA:cisco_secure_email_gateway.log.details}$"),
                            cached_grok!("^MID %{NUMBER:email.message_id} %{DATA:file.extension} file %{GREEDYDATA:cisco_secure_email_gateway.log.details}$"),
                            cached_grok!("\\bMID %{NUMBER:email.message_id}(?: ICID %{NUMBER:cisco_secure_email_gateway.log.injection_connection_id})?"),
                            cached_grok!("^%{GREEDYDATA:cisco_secure_email_gateway.log.message}$"),
                        ],
                        &input,
                        event,
                    )?;
                }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(event, "network.protocol", "network.protocol", str::to_lowercase)?;
                Ok(())
            })();

            let _cond = { event.get_str("cisco_secure_email_gateway.log.type") == Some("changed") };
            if _cond {
            event.set("event.type", json!("change"))?;
            }

            let _cond = { event.get_str("cisco_secure_email_gateway.log.spf_aligned") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_secure_email_gateway.log.spf_aligned") {
                map_strings(event, "cisco_secure_email_gateway.log.spf_aligned", "cisco_secure_email_gateway.log.spf_aligned", str::to_lowercase)?;
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "lowercase")?;
                event.set("_ingest.on_failure_processor_tag", "lowercase_spf_aligned")?;
                        event.append("error.message", json!(format!("Processor {} with tag fail-{} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("cisco_secure_email_gateway.log.dkim_aligned") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_secure_email_gateway.log.dkim_aligned") {
                map_strings(event, "cisco_secure_email_gateway.log.dkim_aligned", "cisco_secure_email_gateway.log.dkim_aligned", str::to_lowercase)?;
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "lowercase")?;
                event.set("_ingest.on_failure_processor_tag", "lowercase_dkim_aligned")?;
                        event.append("error.message", json!(format!("Processor {} with tag fail-{} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("cisco_secure_email_gateway.log.spf_aligned") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_secure_email_gateway.log.spf_aligned") {
                if let Some(val) = event.get("cisco_secure_email_gateway.log.spf_aligned") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_secure_email_gateway.log.spf_aligned".into(),
                            message,
                        })?;
                    event.set("cisco_secure_email_gateway.log.spf_aligned", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_secure_email_gateway_log_spf_aligned")?;
                        event.append("error.message", json!(format!("Processor {} with tag fail-{} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("cisco_secure_email_gateway.log.dkim_aligned") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cisco_secure_email_gateway.log.dkim_aligned") {
                if let Some(val) = event.get("cisco_secure_email_gateway.log.dkim_aligned") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "cisco_secure_email_gateway.log.dkim_aligned".into(),
                            message,
                        })?;
                    event.set("cisco_secure_email_gateway.log.dkim_aligned", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_secure_email_gateway_log_dkim_aligned")?;
                        event.append("error.message", json!(format!("Processor {} with tag fail-{} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("cisco_secure_email_gateway.log.interface") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("cisco_secure_email_gateway.log.interface").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("cisco_secure_email_gateway.log.address") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("cisco_secure_email_gateway.log.address").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique("related.ip", json!(event.get("destination.ip").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })();
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
