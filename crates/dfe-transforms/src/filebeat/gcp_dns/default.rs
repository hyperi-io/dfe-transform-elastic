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
            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            parse_json_field(event, "event.original", "json")?;

            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.action", json!("dns-query"))?;

            event.set("cloud.provider", json!("gcp"))?;

            if let Some(date_str) = event.get_as_string("json.timestamp") {
                match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.logName").cloned() {
                    event.set("log.logger", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.severity").cloned() {
                    event.set("log.level", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("json.insertId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.resource.labels.project_id") {
                    if let Some(val) = event.get("json.resource.labels.project_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.resource.labels.project_id".into(),
                                message,
                            }
                        })?;
                        event.set("cloud.project.id", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("json.resource.labels.location") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.resource.labels.location".into(),
                            message,
                        }
                    })?;
                    event.set("cloud.region", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.authAnswer").cloned() {
                    event.set("gcp.dns.auth_answer", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.destinationIP").cloned() {
                    event.set("gcp.dns.destination_ip", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("gcp.dns.destination_ip").cloned() {
                    event.set("destination.address", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("gcp.dns.destination_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "gcp.dns.destination_ip".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.egressError").cloned() {
                    event.set("gcp.dns.egress_error", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.protocol").cloned() {
                    event.set("gcp.dns.protocol", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("gcp.dns.protocol").cloned() {
                    event.set("network.transport", v)?;
                }
                Ok(())
            })();

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            let _cond = { event.get_str("network.transport") == Some("tcp") };
            if _cond {
                event.set("network.iana_number", json!("6"))?;
            }

            let _cond = { event.get_str("network.transport") == Some("udp") };
            if _cond {
                event.set("network.iana_number", json!("17"))?;
            }

            event.set("network.protocol", json!("dns"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.queryName").cloned() {
                    event.set("gcp.dns.query_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("gcp.dns.query_name").cloned() {
                    event.set("dns.question.name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                gsub_field(
                    event,
                    "dns.question.name",
                    "dns.question.name",
                    cached_regex!("[.]$"),
                    "",
                )?;
                Ok(())
            })();

            if let Some(domain) = event.get_string("dns.question.name") {
                // Public suffix list lookup for registered domain extraction.
                // A failed lookup writes NO target field, which is what
                // Elasticsearch does.
                if let Some(rd) = registered_domain_lookup(&domain) {
                    event.set("dns.question.domain", json!(domain))?;
                    if let Some(registered) = rd.registered_domain {
                        event.set("dns.question.registered_domain", json!(registered))?;
                    }
                    event.set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
                    if let Some(sub) = rd.subdomain {
                        event.set("dns.question.subdomain", json!(sub))?;
                    }
                }
            }

            event.remove("dns.question.domain");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.queryType").cloned() {
                    event.set("gcp.dns.query_type", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("gcp.dns.query_type").cloned() {
                    event.set("dns.question.type", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.rdata").cloned() {
                    event.set("gcp.dns.rdata", v)?;
                }
                Ok(())
            })();

            let _cond =
                { event.has_value("gcp.dns.rdata") && event.get_str("gcp.dns.rdata") != Some("") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "4ddc42c58b18d32a33a88b62c403548dab0abc1c56b20d64a3cd0c2a80b2c6a8",
                        r#"{"params":null,"known":["DnsRdataAnswers"]}"#,
                        r#"def rdata = ctx.gcp.dns.rdata;\ndef dns_answers = [];\n\n// Check for truncated answers.\ndef truncated = rdata.endsWith(\"...\") ? 1 : 0;\n\n// Process answers.\ndef rdata_answers = /\\n/.split(rdata);\n\nfor (def i = 0; i < rdata_answers.length - truncated; i++) {\n    def answer_parts = /\\t/.split(rdata_answers[i]);\n\n    // Assign answer parts.\n    def name = answer_parts[0];\n    def ttl = Long.parseLong(answer_parts[1]);\n    def cls = answer_parts[2];\n    def type = answer_parts[3];\n    def data = answer_parts[4];\n\n    // Remove trailing fullstop.\n    if (name.endsWith(\".\")) {\n        name = name.substring(0, name.length() - 1);\n    }\n\n    if (data.endsWith(\".\")) {\n        data = data.substring(0, data.length() - 1);\n    }\n\n    // Uppercase type.\n    type = type.toUpperCase();\n\n    dns_answers.add([\n        \"name\": name,\n        \"ttl\": ttl,\n        \"class\": cls,\n        \"type\": type,\n        \"data\": data\n    ]);\n}\nctx.dns.answers = dns_answers;\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("json.jsonPayload.structuredRdata")
                    && event
                        .get("json.jsonPayload.structuredRdata")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "f4827da32971b945494668ea40e8527ff4331d5d64faa2f8acd41aca1c10eb64",
                            r#"{"params":null,"known":["StructuredRdataAnswers"]}"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("dns.answers")
                    && event.get("dns.answers").is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "4c0cf7d04eb019f600a6c584ac0969ce73011fcc05c7edbb293eae5dec0ca1a5",
                            r#"{"params":null,"known":["RelatedFromDnsAnswers"]}"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.responseCode").cloned() {
                    event.set("gcp.dns.response_code", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("gcp.dns.response_code").cloned() {
                    event.set("dns.response_code", v)?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("gcp.dns.response_code")
                    && event.get_str("gcp.dns.response_code") == Some("NOERROR")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("gcp.dns.response_code")
                    && event.get_str("gcp.dns.response_code") != Some("NOERROR")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.serverLatency").cloned() {
                    event.set("gcp.dns.server_latency", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.sourceIP").cloned() {
                    event.set("gcp.dns.source_ip", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("gcp.dns.source_ip").cloned() {
                    event.set("source.address", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(val) = event.get("gcp.dns.source_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "gcp.dns.source_ip".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.sourceNetwork").cloned() {
                    event.set("gcp.dns.source_network", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.vmInstanceIdString").cloned() {
                    event.set("gcp.dns.vm_instance_id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("gcp.dns.vm_instance_id").cloned() {
                    event.set("cloud.instance.id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.vmInstanceName").cloned() {
                    event.set("gcp.dns.vm_instance_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("gcp.dns.vm_instance_name").cloned() {
                    event.set("cloud.instance.name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                gsub_field(
                    event,
                    "cloud.instance.name",
                    "cloud.instance.name",
                    cached_regex!("^.*[.]"),
                    "",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.vmProjectId").cloned() {
                    event.set("gcp.dns.vm_project_id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.jsonPayload.vmZoneName").cloned() {
                    event.set("gcp.dns.vm_zone_name", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("gcp.dns.vm_zone_name").cloned() {
                    event.set("cloud.availability_zone", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.resource.labels.target_type").cloned() {
                    event.set("gcp.dns.target_type", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("json.resource.labels.source_type").cloned() {
                    event.set("gcp.dns.source_type", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("dns.question.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("dns.question.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("json") && event.get_bool("_conf.keep_json") == Some(true) };
            if _cond {
                event.rename("json", "gcp.dns.flattened")?;
            }

            event.remove("_conf");
            event.remove("json");

            // The pipeline's script, emitted as the runtime's equivalent
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
