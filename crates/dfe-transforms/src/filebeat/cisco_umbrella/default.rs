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

            event.set("observer.vendor", json!("Cisco"))?;

            event.set("observer.product", json!("Umbrella"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dnslogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dnslogs"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(csv_str) = event.get_string("event.original") {
                    let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b',')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella._tmp.time", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.identity", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.identities", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("source.address", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("source.nat.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.action", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("dns.question.type", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("dns.response_code", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("dns.question.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.categories", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.policy_identity_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.identity_types", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.blocked_categories", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            if !val.is_empty() {
                                event.set("rule.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            if !val.is_empty() {
                                event.set("destination.geo.country_iso_code", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set("organization.id", val)?;
                            }
                        }
                    }
                }
            }

            if event.has_value("dns.question.name") {
                if let Some(s) = event.get_string("dns.question.name") {
                    let re = cached_regex!("\\.$");
                    let replaced = re.replace_all(&s, "").into_owned();
                    event.set("dns.question.name", replaced)?;
                }
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dnslogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dnslogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("observer.type", json!("dns"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("iplogs"))
                        }
                        serde_json::Value::String(s) => s.contains("iplogs"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(csv_str) = event.get_string("event.original") {
                    let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b',')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella._tmp.time", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.identity", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("source.address", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("source.port", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("destination.address", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("destination.port", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.categories", val)?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("iplogs"))
                        }
                        serde_json::Value::String(s) => s.contains("iplogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("observer.type", json!("firewall"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("proxylogs"))
                        }
                        serde_json::Value::String(s) => s.contains("proxylogs"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(csv_str) = event.get_string("event.original") {
                    let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b',')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella._tmp.time", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.identity", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("source.address", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("source.nat.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("destination.address", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("http.request.mime_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.action", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("url.original", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("http.request.referrer", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("user_agent.original", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("http.response.status_code", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set("http.request.bytes", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            if !val.is_empty() {
                                event.set("http.response.bytes", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            if !val.is_empty() {
                                event.set("http.response.body.bytes", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.sha_sha256", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.categories", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.av_detections", val)?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.puas", val)?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.amp_disposition", val)?;
                            }
                        }
                        if let Some(val) = record.get(19) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.amp_malware_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(20) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.amp_score", val)?;
                            }
                        }
                        if let Some(val) = record.get(21) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.policy_identity_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(22) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.blocked_categories", val)?;
                            }
                        }
                        if let Some(val) = record.get(23) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.identities", val)?;
                            }
                        }
                        if let Some(val) = record.get(24) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.identity_types", val)?;
                            }
                        }
                        if let Some(val) = record.get(25) {
                            if !val.is_empty() {
                                event.set("http.request.method", val)?;
                            }
                        }
                        if let Some(val) = record.get(26) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.dlp_status", val)?;
                            }
                        }
                        if let Some(val) = record.get(27) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.certificate_errors", val)?;
                            }
                        }
                        if let Some(val) = record.get(28) {
                            if !val.is_empty() {
                                event.set("file.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(29) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.ruleset_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(30) {
                            if !val.is_empty() {
                                event.set("rule.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(31) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.destination_lists_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(32) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.isolate_action", val)?;
                            }
                        }
                        if let Some(val) = record.get(33) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.file_action", val)?;
                            }
                        }
                        if let Some(val) = record.get(34) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.warn_status", val)?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("proxylogs"))
                        }
                        serde_json::Value::String(s) => s.contains("proxylogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("observer.type", json!("proxy"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && (event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("cloudfirewalllogs"))
                        }
                        serde_json::Value::String(s) => s.contains("cloudfirewalllogs"),
                        _ => false,
                    }) || event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("firewalllogs"))
                        }
                        serde_json::Value::String(s) => s.contains("firewalllogs"),
                        _ => false,
                    }))
            };
            if _cond {
                if let Some(csv_str) = event.get_string("event.original") {
                    let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b',')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella._tmp.time", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.origin_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.identities", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.identity_types", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.direction", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("network.transport", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("source.bytes", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("source.address", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("source.port", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("destination.address", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("destination.port", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.datacenter", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            if !val.is_empty() {
                                event.set("rule.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.action", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.fqdns", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.destination_lists_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.first_packet_timestamp", val)?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.last_packet_timestamp", val)?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            if !val.is_empty() {
                                event.set("source.packets", val)?;
                            }
                        }
                        if let Some(val) = record.get(19) {
                            if !val.is_empty() {
                                event.set("destination.packets", val)?;
                            }
                        }
                        if let Some(val) = record.get(20) {
                            if !val.is_empty() {
                                event.set("source.bytes", val)?;
                            }
                        }
                        if let Some(val) = record.get(21) {
                            if !val.is_empty() {
                                event.set("destination.bytes", val)?;
                            }
                        }
                        if let Some(val) = record.get(22) {
                            if !val.is_empty() {
                                event.set("event.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(23) {
                            if !val.is_empty() {
                                event.set("destination.geo.country_iso_code", val)?;
                            }
                        }
                        if let Some(val) = record.get(24) {
                            if !val.is_empty() {
                                event.set("cloud.availability_zone", val)?;
                            }
                        }
                        if let Some(val) = record.get(25) {
                            if !val.is_empty() {
                                event.set("network.application", val)?;
                            }
                        }
                        if let Some(val) = record.get(26) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.private_app_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(27) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.private_flow", val)?;
                            }
                        }
                        if let Some(val) = record.get(28) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.posture_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(29) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.casi_category_ids", val)?;
                            }
                        }
                        if let Some(val) = record.get(30) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.traffic_source", val)?;
                            }
                        }
                        if let Some(val) = record.get(31) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.content_category_ids", val)?;
                            }
                        }
                        if let Some(val) = record.get(32) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.content_category_list_ids", val)?;
                            }
                        }
                        if let Some(val) = record.get(33) {
                            if !val.is_empty() {
                                event.set("organization.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(34) {
                            if !val.is_empty() {
                                event.set("source.nat.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(35) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.egress", val)?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("cisco.umbrella.first_packet_timestamp")
                    && event.get_str("cisco.umbrella.first_packet_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cisco.umbrella.first_packet_timestamp")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], None, None) {
                            event.set("cisco.umbrella.first_packet_timestamp", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_cisco_umbrella_first_packet_timestamp",
                    )?;
                    if event
                        .remove("cisco.umbrella.first_packet_timestamp")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "cisco.umbrella.first_packet_timestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("cisco.umbrella.last_packet_timestamp")
                    && event.get_str("cisco.umbrella.last_packet_timestamp") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("cisco.umbrella.last_packet_timestamp")
                    {
                        if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], None, None) {
                            event.set("cisco.umbrella.last_packet_timestamp", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_cisco_umbrella_last_packet_timestamp",
                    )?;
                    if event
                        .remove("cisco.umbrella.last_packet_timestamp")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "cisco.umbrella.last_packet_timestamp".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("log.file.path")
                    && (event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("cloudfirewalllogs"))
                        }
                        serde_json::Value::String(s) => s.contains("cloudfirewalllogs"),
                        _ => false,
                    }) || event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("firewalllogs"))
                        }
                        serde_json::Value::String(s) => s.contains("firewalllogs"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("observer.type", json!("firewall"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("intrusionlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("intrusionlogs"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(csv_str) = event.get_string("event.original") {
                    let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b',')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella._tmp.time", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.identities", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.identity_types", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.gid", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.sid", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.message", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.signature_list_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.severity", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.classification", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.cves", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("network.protocol", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set("event.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            if !val.is_empty() {
                                event.set("source.address", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            if !val.is_empty() {
                                event.set("source.port", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            if !val.is_empty() {
                                event.set("destination.address", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set("destination.port", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.action", val)?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.operation_mode", val)?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.policy_resource_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(19) {
                            if !val.is_empty() {
                                event.set("network.direction", val)?;
                            }
                        }
                        if let Some(val) = record.get(20) {
                            if !val.is_empty() {
                                event.set("rule.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(21) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.ips_config_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(22) {
                            if !val.is_empty() {
                                event.set("cloud.availability_zone", val)?;
                            }
                        }
                        if let Some(val) = record.get(23) {
                            if !val.is_empty() {
                                event.set("network.application", val)?;
                            }
                        }
                        if let Some(val) = record.get(24) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.casi_category_ids", val)?;
                            }
                        }
                        if let Some(val) = record.get(25) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.datacenter", val)?;
                            }
                        }
                        if let Some(val) = record.get(26) {
                            if !val.is_empty() {
                                event.set("organization.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(27) {
                            if !val.is_empty() {
                                event.set("source.nat.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(28) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.egress", val)?;
                            }
                        }
                        if let Some(val) = record.get(29) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.enforced_by", val)?;
                            }
                        }
                        if let Some(val) = record.get(30) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.ftd_enforcement_id", val)?;
                            }
                        }
                        if let Some(val) = record.get(31) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.ftd_enforcement_name", val)?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("intrusionlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("intrusionlogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("observer.type", json!("idps"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dlplogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dlplogs"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(csv_str) = event.get_string("event.original") {
                    let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b',')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella._tmp.time", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("event.provider", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("event.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.severity", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("network.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("file.owner", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("file.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("network.application", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("url.original", val)?;
                            }
                        }
                        if let Some(val) = record.get(9) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.action", val)?;
                            }
                        }
                        if let Some(val) = record.get(10) {
                            if !val.is_empty() {
                                event.set("rule.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(11) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.data_classification", val)?;
                            }
                        }
                        if let Some(val) = record.get(12) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.data_identifier", val)?;
                            }
                        }
                        if let Some(val) = record.get(13) {
                            if !val.is_empty() {
                                event.set("file.mime_type", val)?;
                            }
                        }
                        if let Some(val) = record.get(14) {
                            if !val.is_empty() {
                                event.set("file.size", val)?;
                            }
                        }
                        if let Some(val) = record.get(15) {
                            if !val.is_empty() {
                                event.set("file.hash.sha256", val)?;
                            }
                        }
                        if let Some(val) = record.get(16) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.file_label", val)?;
                            }
                        }
                        if let Some(val) = record.get(17) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.application_category_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(18) {
                            if !val.is_empty() {
                                event.set("network.direction", val)?;
                            }
                        }
                        if let Some(val) = record.get(19) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.private_resource_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(20) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.private_resource_group_name", val)?;
                            }
                        }
                        if let Some(val) = record.get(21) {
                            if !val.is_empty() {
                                event.set("network.protocol", val)?;
                            }
                        }
                        if let Some(val) = record.get(22) {
                            if !val.is_empty() {
                                event.set("destination.ip", val)?;
                            }
                        }
                        if let Some(val) = record.get(23) {
                            if !val.is_empty() {
                                event.set("destination.port", val)?;
                            }
                        }
                        if let Some(val) = record.get(24) {
                            if !val.is_empty() {
                                event.set("organization.id", val)?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dlplogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dlplogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("observer.type", json!("dlp"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("auditlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("auditlogs"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(csv_str) = event.get_string("event.original") {
                    let csv_str = csv_close_quote_gap(&csv_str, ',', '\"');
                    let mut rdr = csv::ReaderBuilder::new()
                        .delimiter(b',')
                        .quote(b'\"')
                        .has_headers(false)
                        .from_reader(csv_str.as_bytes());
                    if let Some(Ok(record)) = rdr.records().next() {
                        if let Some(val) = record.get(0) {
                            if !val.is_empty() {
                                event.set("event.id", val)?;
                            }
                        }
                        if let Some(val) = record.get(1) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella._tmp.time", val)?;
                            }
                        }
                        if let Some(val) = record.get(2) {
                            if !val.is_empty() {
                                event.set("user.email", val)?;
                            }
                        }
                        if let Some(val) = record.get(3) {
                            if !val.is_empty() {
                                event.set("user.name", val)?;
                            }
                        }
                        if let Some(val) = record.get(4) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.audit.type", val)?;
                            }
                        }
                        if let Some(val) = record.get(5) {
                            if !val.is_empty() {
                                event.set("event.action", val)?;
                            }
                        }
                        if let Some(val) = record.get(6) {
                            if !val.is_empty() {
                                event.set("source.address", val)?;
                            }
                        }
                        if let Some(val) = record.get(7) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.audit.before", val)?;
                            }
                        }
                        if let Some(val) = record.get(8) {
                            if !val.is_empty() {
                                event.set("cisco.umbrella.audit.after", val)?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.get_str("cisco.umbrella.audit.type") != Some("global_settings") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco.umbrella.audit.before") {
                        if let Some(kv_str) = event.get_string("cisco.umbrella.audit.before") {
                            for pair in cached_regex!("\\n").split(&kv_str).into_iter() {
                                if pair.trim().is_empty() {
                                    continue;
                                }
                                let Some((key, value)) = ({
                                    let parts = cached_regex!(":\\s*").splitn(&pair, 2);
                                    match (parts.first(), parts.get(1)) {
                                        (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                        _ => None,
                                    }
                                }) else {
                                    return Err(TransformError::ParseError {
                                        path: "cisco.umbrella.audit.before".into(),
                                        message: format!("does not contain value_split: {pair}"),
                                    });
                                };
                                {
                                    let key = key.trim_matches(|c| " ".contains(c));
                                    if !key.is_empty() {
                                        event.set(
                                            &format!("cisco.umbrella.audit.before_values.{}", key),
                                            value,
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "kv")?;
                    event.set("_ingest.on_failure_processor_tag", "kv_audit_before")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.get_str("cisco.umbrella.audit.type") != Some("global_settings") };
            if _cond {
                if event.has_value("cisco.umbrella.audit.before") {
                    if let Some(s) = event.get_string("cisco.umbrella.audit.before") {
                        let parts: Vec<Value> = cached_regex!("\\n")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        event.set("cisco.umbrella.audit.before", Value::Array(parts))?;
                    }
                }
            }

            let _cond = { event.get_str("cisco.umbrella.audit.type") != Some("global_settings") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco.umbrella.audit.after") {
                        if let Some(kv_str) = event.get_string("cisco.umbrella.audit.after") {
                            for pair in cached_regex!("\\n").split(&kv_str).into_iter() {
                                if pair.trim().is_empty() {
                                    continue;
                                }
                                let Some((key, value)) = ({
                                    let parts = cached_regex!(":\\s*").splitn(&pair, 2);
                                    match (parts.first(), parts.get(1)) {
                                        (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                        _ => None,
                                    }
                                }) else {
                                    return Err(TransformError::ParseError {
                                        path: "cisco.umbrella.audit.after".into(),
                                        message: format!("does not contain value_split: {pair}"),
                                    });
                                };
                                {
                                    let key = key.trim_matches(|c| " ".contains(c));
                                    if !key.is_empty() {
                                        event.set(
                                            &format!("cisco.umbrella.audit.after_values.{}", key),
                                            value,
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "kv")?;
                    event.set("_ingest.on_failure_processor_tag", "kv_audit_after")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.get_str("cisco.umbrella.audit.type") != Some("global_settings") };
            if _cond {
                if event.has_value("cisco.umbrella.audit.after") {
                    if let Some(s) = event.get_string("cisco.umbrella.audit.after") {
                        let parts: Vec<Value> = cached_regex!("\\n")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        event.set("cisco.umbrella.audit.after", Value::Array(parts))?;
                    }
                }
            }

            let _cond = { event.has_value("url.original") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    uri_parts(event, "url.original", "url", true, false)?;
                    Ok(())
                })();
            }

            let _cond = {
                (event
                    .get("cisco.umbrella.identity")
                    .is_some_and(|v| v.is_string()))
                    && event
                        .get("cisco.umbrella.identity")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(","))
                            }
                            serde_json::Value::String(s) => s.contains(","),
                            _ => false,
                        })
                    && (event
                        .get("cisco.umbrella.identities")
                        .is_some_and(|v| v.is_string()))
                    && event.get_str("cisco.umbrella.identity").is_some_and(|p| {
                        event
                            .get_str("cisco.umbrella.identities")
                            .is_some_and(|s| s.starts_with(p))
                    })
            };
            if _cond {
                // Painless script
                // Source: String identities_tail = ctx.cisco.umbrella.identities.substring(ctx.cisco.umbrella.identity.length());\nif (identities_tail.startsWith(',')) {\n  identities_tail = identities_tail.substring(1);\n}\nif (ctx.cisco.umbrella._tmp == null) {\n  ctx.cisco.umbrella._tmp = new HashMap();\n}\nctx.cisco.umbrella._tmp.identities_tail = identities_tail;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String identities_tail = ctx.cisco.umbrella.identities.substring(ctx.cisco.umbrella.identity.length());\nif (identities_tail.startsWith(',')) {\n  identities_tail = identities_tail.substring(1);\n}\nif (ctx.cisco.umbrella._tmp == null) {\n  ctx.cisco.umbrella._tmp = new HashMap();\n}\nctx.cisco.umbrella._tmp.identities_tail = identities_tail;"#
                    ),
                )?;
            }

            let _cond = { event.has_value("cisco.umbrella._tmp.identities_tail") };
            if _cond {
                if let Some(s) = event.get_string("cisco.umbrella._tmp.identities_tail") {
                    let parts: Vec<Value> = cached_regex!(",\\s*")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    event.set("cisco.umbrella._tmp.identities_tail", Value::Array(parts))?;
                }
            }

            let _cond = { event.has_value("cisco.umbrella._tmp.identities_tail") };
            if _cond {
                // Painless script
                // Source: def identities = new ArrayList();\nidentities.add(ctx.cisco.umbrella.identity);\nfor (identity in ctx.cisco.umbrella._tmp.identities_tail) {\n  identities.add(identity);\n}        \nctx.cisco.umbrella._tmp.identities = identities;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def identities = new ArrayList();\nidentities.add(ctx.cisco.umbrella.identity);\nfor (identity in ctx.cisco.umbrella._tmp.identities_tail) {\n  identities.add(identity);\n}        \nctx.cisco.umbrella._tmp.identities = identities;"#
                    ),
                )?;
            }

            let _cond = { event.has_value("cisco.umbrella._tmp.identities") };
            if _cond {
                if let Some(v) = event.get("cisco.umbrella._tmp.identities").cloned() {
                    event.set("cisco.umbrella.identities", v)?;
                }
            }

            let _cond = {
                event.has_value("cisco.umbrella.identities")
                    && !event.has_value("cisco.umbrella._tmp.identities")
            };
            if _cond {
                if let Some(s) = event.get_string("cisco.umbrella.identities") {
                    let parts: Vec<Value> = cached_regex!(",\\s*")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    event.set("cisco.umbrella.identities", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dnslogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dnslogs"),
                        _ => false,
                    })
                    && event.has_value("cisco.umbrella.categories")
            };
            if _cond {
                if let Some(s) = event.get_string("cisco.umbrella.categories") {
                    let parts: Vec<Value> = cached_regex!(",\\s*")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    event.set("cisco.umbrella.categories", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dnslogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dnslogs"),
                        _ => false,
                    })
                    && event.has_value("cisco.umbrella.blocked_categories")
            };
            if _cond {
                if let Some(s) = event.get_string("cisco.umbrella.blocked_categories") {
                    let parts: Vec<Value> = cached_regex!(",\\s*")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    event.set("cisco.umbrella.blocked_categories", Value::Array(parts))?;
                }
            }

            let _cond = { event.has_value("cisco.umbrella.identity_types") };
            if _cond {
                if let Some(s) = event.get_string("cisco.umbrella.identity_types") {
                    let parts: Vec<Value> = cached_regex!(",\\s*")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    event.set("cisco.umbrella.identity_types", Value::Array(parts))?;
                }
            }

            let _cond = { event.has_value("cisco.umbrella.fqdns") };
            if _cond {
                if let Some(s) = event.get_string("cisco.umbrella.fqdns") {
                    let parts: Vec<Value> = cached_regex!(",\\s*")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    event.set("cisco.umbrella.fqdns", Value::Array(parts))?;
                }
            }

            let _cond = {
                event.has_value("cisco.umbrella.identities")
                    && event.has_value("cisco.umbrella.identity_types")
                    && event
                        .get("cisco.umbrella.identities.length")
                        .filter(|v| !v.is_null())
                        == event
                            .get("cisco.umbrella.identity_types.length")
                            .filter(|v| !v.is_null())
            };
            if _cond {
                // Painless script
                // Source: void setHost(def ctx, def x) {\n  if (ctx.host == null) {\n    ctx.host = new HashMap();\n  }\n  if (ctx.host.name == null) {\n    ctx.host.name = x;\n  }\n}\nvoid setUser(def ctx, def x) {\n  if (ctx.user == null) {\n    ctx.user = new HashMap();\n  }\n  if (ctx.user.name == null) {\n    ctx.user.name = x;\n  }\n}\nvoid addNetwork(def ctx, def x) {\n  if (ctx.network == null) {\n    ctx.network = new HashMap();\n  }\n  if (ctx.network?.name == null) {\n    ArrayList al = new ArrayList();\n    ctx.network.put(\"name\", al);\n  }\n  if (!ctx.network.name.contains(x)) {\n    ctx.network.name.add(x);\n  }\n}\ndef i = 0;\nfor (cisco_identity_type in ctx.cisco.umbrella.identity_types) {\n  if ([\"AD Users\"].contains(cisco_identity_type)) {\n    setUser(ctx, ctx.cisco.umbrella.identities[i]);\n  }\n  if ([\"AD Computers\", \"Roaming Computers\", \"Anyconnect Roaming Client\", \"Mobile Devices\"].contains(cisco_identity_type)) {\n    setHost(ctx, ctx.cisco.umbrella.identities[i]);\n  }\n  if ([\"Sites\", \"Internal Networks\", \"Networks\", \"Network Devices\", \"Network Tunnels\", \"CDFW Tunnel Device\"].contains(cisco_identity_type)) {\n    addNetwork(ctx, ctx.cisco.umbrella.identities[i]);\n  }\n  i++;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void setHost(def ctx, def x) {\n  if (ctx.host == null) {\n    ctx.host = new HashMap();\n  }\n  if (ctx.host.name == null) {\n    ctx.host.name = x;\n  }\n}\nvoid setUser(def ctx, def x) {\n  if (ctx.user == null) {\n    ctx.user = new HashMap();\n  }\n  if (ctx.user.name == null) {\n    ctx.user.name = x;\n  }\n}\nvoid addNetwork(def ctx, def x) {\n  if (ctx.network == null) {\n    ctx.network = new HashMap();\n  }\n  if (ctx.network?.name == null) {\n    ArrayList al = new ArrayList();\n    ctx.network.put(\"name\", al);\n  }\n  if (!ctx.network.name.contains(x)) {\n    ctx.network.name.add(x);\n  }\n}\ndef i = 0;\nfor (cisco_identity_type in ctx.cisco.umbrella.identity_types) {\n  if ([\"AD Users\"].contains(cisco_identity_type)) {\n    setUser(ctx, ctx.cisco.umbrella.identities[i]);\n  }\n  if ([\"AD Computers\", \"Roaming Computers\", \"Anyconnect Roaming Client\", \"Mobile Devices\"].contains(cisco_identity_type)) {\n    setHost(ctx, ctx.cisco.umbrella.identities[i]);\n  }\n  if ([\"Sites\", \"Internal Networks\", \"Networks\", \"Network Devices\", \"Network Tunnels\", \"CDFW Tunnel Device\"].contains(cisco_identity_type)) {\n    addNetwork(ctx, ctx.cisco.umbrella.identities[i]);\n  }\n  i++;\n}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                if let Some(s) = event.get_string("host.name") {
                    let lowered = s.to_lowercase();
                    event.set("host.name", lowered)?;
                }
            }

            let _cond = {
                event.has_value("host.name")
                    && event.get("host.name").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
                        serde_json::Value::String(s) => s.contains("."),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("host.name") {
                        // Grok pattern: ^%{DATA:host.hostname}\\.%{GREEDYDATA:host.domain}$
                        let _ = cached_grok!("^%{DATA:host.hostname}\\.%{GREEDYDATA:host.domain}$")
                            .extract_into(&input, event)?;
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("user.name")
                    && event.get("user.name").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("user.name") {
                        // Grok pattern: ^%{GREEDYDATA:user.full_name} (\\(\\[%{GREEDYDATA}\\]\\(mailto:(?P<user_email>%{DATA:user.name}@%{DATA:user.domain})\\)\\))?$
                        // Grok pattern: ^%{GREEDYDATA:user.full_name} (\\((?P<user_email>%{DATA:user.name}@%{DATA:user.domain})\\))?$
                        let _ = extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "^%{GREEDYDATA:user.full_name} (\\(\\[%{GREEDYDATA}\\]\\(mailto:(?P<user_email>%{DATA:user.name}@%{DATA:user.domain})\\)\\))?$",
                                    [("user_email", "user.email")]
                                ),
                                cached_grok_mapped!(
                                    "^%{GREEDYDATA:user.full_name} (\\((?P<user_email>%{DATA:user.name}@%{DATA:user.domain})\\))?$",
                                    [("user_email", "user.email")]
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.set(
                    "user.id",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("user.email") == Some("null") };
            if _cond {
                if event.remove("user.email").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "user.email".into(),
                    });
                }
            }

            let _cond = { event.get_str("user.id") == Some("null") };
            if _cond {
                if event.remove("user.id").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "user.id".into(),
                    });
                }
            }

            let _cond = { event.get_str("user.name") == Some("null") };
            if _cond {
                if event.remove("user.name").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "user.name".into(),
                    });
                }
            }

            let _cond = {
                event.get("user").is_some_and(|v| v.is_object()) && event.get("user").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                event.remove("user");
            }

            let _cond = { event.has_value("cisco.umbrella._tmp.time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("cisco.umbrella._tmp.time") {
                    if let Some(parsed) =
                        parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss", "ISO8601"], None, None)
                    {
                        event.set("@timestamp", parsed)?;
                    }
                }
            }

            let _cond = { event.has_value("dns.question.name") };
            if _cond {
                event.set("dns.type", json!("query"))?;
            }

            if event.has_value("dns.question.name") {
                if let Some(domain_str) = event.get_string("dns.question.name") {
                    let domain = domain_str.to_string();
                    event.set("dns.question.domain", json!(domain.clone()))?;
                    // Public suffix list lookup for registered domain extraction
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        event.set(
                            "dns.question.registered_domain",
                            json!(rd.registered_domain),
                        )?;
                        event.set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("dns.question.subdomain", json!(sub))?;
                        }
                    }
                }
            }

            event.remove("dns.question.domain");

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

            let _cond = { event.get_str("network.direction") == Some("S2C") };
            if _cond {
                event.set("network.direction", json!("inbound"))?;
            }

            let _cond = { event.get_str("network.direction") == Some("C2S") };
            if _cond {
                event.set("network.direction", json!("outbound"))?;
            }

            let _cond = { event.get_str("network.direction") == Some("UNKNOWN") };
            if _cond {
                event.set("network.direction", json!("unknown"))?;
            }

            let _cond = { event.has_value("network.application") };
            if _cond {
                if let Some(s) = event.get_string("network.application") {
                    let lowered = s.to_lowercase();
                    event.set("network.application", lowered)?;
                }
            }

            let _cond = { event.has_value("cisco.umbrella.direction") };
            if _cond {
                if let Some(s) = event.get_string("cisco.umbrella.direction") {
                    let lowered = s.to_lowercase();
                    event.set("network.direction", lowered)?;
                }
            }

            let _cond = { event.has_value("source.bytes") };
            if _cond {
                if let Some(val) = event.get("source.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("source.bytes", converted)?;
                }
            }

            let _cond = { event.has_value("source.packets") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("source.packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.packets".into(),
                                message,
                            }
                        })?;
                        event.set("source.packets", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_source_packets_to_long",
                    )?;
                    if event.remove("source.packets").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "source.packets".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("destination.bytes") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("destination.bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.bytes".into(),
                                message,
                            }
                        })?;
                        event.set("destination.bytes", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_destination_bytes_to_long",
                    )?;
                    if event.remove("destination.bytes").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "destination.bytes".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("destination.packets") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("destination.packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.packets".into(),
                                message,
                            }
                        })?;
                        event.set("destination.packets", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_destination_packets_to_long",
                    )?;
                    if event.remove("destination.packets").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "destination.packets".into(),
                        });
                    }
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond =
                { event.has_value("destination.packets") && event.has_value("source.packets") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.network = ctx.network ?: [:];\nctx.network.packets = ctx.source.packets + ctx.destination.packets;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.network = ctx.network ?: [:];\nctx.network.packets = ctx.source.packets + ctx.destination.packets;"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_network_packets")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("destination.bytes") && event.has_value("source.bytes") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.network = ctx.network ?: [:];\nctx.network.bytes = ctx.source.bytes + ctx.destination.bytes;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.network = ctx.network ?: [:];\nctx.network.bytes = ctx.source.bytes + ctx.destination.bytes;"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_network_bytes")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("source.port") };
            if _cond {
                if let Some(val) = event.get("source.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "source.port".into(),
                            message,
                        }
                    })?;
                    event.set("source.port", converted)?;
                }
            }

            let _cond = { event.has_value("destination.port") };
            if _cond {
                if let Some(val) = event.get("destination.port") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "destination.port".into(),
                            message,
                        }
                    })?;
                    event.set("destination.port", converted)?;
                }
            }

            let _cond = { event.has_value("http.request.bytes") };
            if _cond {
                if let Some(val) = event.get("http.request.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "http.request.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("http.request.bytes", converted)?;
                }
            }

            let _cond = { event.has_value("http.response.bytes") };
            if _cond {
                if let Some(val) = event.get("http.response.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "http.response.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("http.response.bytes", converted)?;
                }
            }

            let _cond = { event.has_value("http.response.body.bytes") };
            if _cond {
                if let Some(val) = event.get("http.response.body.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "http.response.body.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("http.response.body.bytes", converted)?;
                }
            }

            let _cond = { event.has_value("http.response.status_code") };
            if _cond {
                if let Some(val) = event.get("http.response.status_code") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "http.response.status_code".into(),
                            message,
                        }
                    })?;
                    event.set("http.response.status_code", converted)?;
                }
            }

            let _cond = { event.has_value("cisco.umbrella.action") };
            if _cond {
                if let Some(s) = event.get_string("cisco.umbrella.action") {
                    let re = cached_regex!("\\s");
                    let replaced = re.replace_all(&s, "-").into_owned();
                    event.set("cisco.umbrella.action", replaced)?;
                }
            }

            let _cond = {
                event.has_value("cisco.umbrella.action")
                    && event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dnslogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dnslogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.set(
                    "event.action",
                    json!(format!(
                        "dns-request-{}",
                        event
                            .get("cisco.umbrella.action")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.has_value("http.request.method")
                    && event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("proxylogs"))
                        }
                        serde_json::Value::String(s) => s.contains("proxylogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.set(
                    "event.action",
                    json!(format!(
                        "proxy-request-{}",
                        event
                            .get("http.request.method")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.has_value("cisco.umbrella.action")
                    && event.has_value("log.file.path")
                    && (event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("firewalllogs"))
                        }
                        serde_json::Value::String(s) => s.contains("firewalllogs"),
                        _ => false,
                    }) || event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("cloudfirewalllogs"))
                        }
                        serde_json::Value::String(s) => s.contains("cloudfirewalllogs"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set(
                    "event.action",
                    json!(format!(
                        "fw-connection-{}",
                        event
                            .get("cisco.umbrella.action")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.has_value("cisco.umbrella.action")
                    && event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("intrusionlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("intrusionlogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.set(
                    "event.action",
                    json!(format!(
                        "ips-{}",
                        event
                            .get("cisco.umbrella.action")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.has_value("cisco.umbrella.action")
                    && event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dlplogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dlplogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.set(
                    "event.action",
                    json!(format!(
                        "dlp-{}",
                        event
                            .get("cisco.umbrella.action")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && !(event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("auditlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("auditlogs"),
                        _ => false,
                    }))
            };
            if _cond {
                event.append_unique("event.category", json!("network"))?;
            }

            let _cond = {
                event.has_value("cisco.umbrella.action")
                    && ["Allowed", "ALLOWED", "ALLOW", "Would-Block"]
                        .contains(&event.get_str("cisco.umbrella.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("allowed"))?;
            }

            let _cond = {
                event.has_value("cisco.umbrella.action")
                    && ["Blocked", "BLOCKED", "BLOCK"]
                        .contains(&event.get_str("cisco.umbrella.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("denied"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && !(event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("auditlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("auditlogs"),
                        _ => false,
                    }))
            };
            if _cond {
                event.append("event.type", json!("connection"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("auditlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("auditlogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.append_unique("event.category", json!("configuration"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dlplogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dlplogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("intrusionlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("intrusionlogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.append("event.category", json!("intrusion_detection"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("intrusionlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("intrusionlogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && !(event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("intrusionlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("intrusionlogs"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("intrusionlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("intrusionlogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dlplogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dlplogs"),
                        _ => false,
                    })
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("auditlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("auditlogs"),
                        _ => false,
                    })
                    && event
                        .get_str("event.action")
                        .is_some_and(|s| s.to_lowercase() == "create")
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("auditlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("auditlogs"),
                        _ => false,
                    })
                    && event
                        .get_str("event.action")
                        .is_some_and(|s| s.to_lowercase() == "update")
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                event.has_value("log.file.path")
                    && event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("auditlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("auditlogs"),
                        _ => false,
                    })
                    && event
                        .get_str("event.action")
                        .is_some_and(|s| s.to_lowercase() == "delete")
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.address") {
                    if let Some(val) = event.get("source.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.address".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if let Some(v) = event.get("source.address").cloned() {
                    event.set("source.domain", v)?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.address") {
                    if let Some(val) = event.get("destination.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "destination.address".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if let Some(v) = event.get("destination.address").cloned() {
                    event.set("destination.domain", v)?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.nat.ip") {
                    if let Some(val) = event.get("source.nat.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.nat.ip".into(),
                                message,
                            }
                        })?;
                        event.set("source.nat.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                if event.remove("source.nat.ip").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "source.nat.ip".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("source.ip") {
                // Community ID v1 hash
                if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                    event.get_string("source.ip"),
                    event.get_string("destination.ip"),
                    event
                        .get_as_string("network.iana_number")
                        .or_else(|| event.get_as_string("network.transport")),
                ) {
                    let icmp = matches!(
                        protocol.to_ascii_lowercase().as_str(),
                        "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                    );
                    let (src_field, dst_field) = if icmp {
                        ("icmp.type", "icmp.code")
                    } else {
                        ("source.port", "destination.port")
                    };
                    let src_port =
                        u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                    let dst_port =
                        u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                    match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                        Ok(cid) => event.set("network.community_id", cid)?,
                        Err(message) => {
                            return Err(TransformError::ParseError {
                                path: "network.community_id".into(),
                                message,
                            });
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("destination.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("destination.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            let _cond = {
                event.get_str("cisco.umbrella.severity") == Some("LOW")
                    && event.has_value("log.file.path")
                    && (event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("intrusionlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("intrusionlogs"),
                        _ => false,
                    }) || event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dlplogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dlplogs"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.severity", json!(1))?;
            }

            let _cond = {
                event.get_str("cisco.umbrella.severity") == Some("MEDIUM")
                    && event.has_value("log.file.path")
                    && (event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("intrusionlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("intrusionlogs"),
                        _ => false,
                    }) || event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dlplogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dlplogs"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.severity", json!(2))?;
            }

            let _cond = {
                event.get_str("cisco.umbrella.severity") == Some("HIGH")
                    && event.has_value("log.file.path")
                    && (event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("intrusionlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("intrusionlogs"),
                        _ => false,
                    }) || event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dlplogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dlplogs"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.severity", json!(3))?;
            }

            let _cond = {
                event.get_str("cisco.umbrella.severity") == Some("CRITICAL")
                    && event.has_value("log.file.path")
                    && (event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("intrusionlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("intrusionlogs"),
                        _ => false,
                    }) || event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dlplogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dlplogs"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.severity", json!(4))?;
            }

            let _cond = {
                !event.has_value("event.severity")
                    && event.has_value("log.file.path")
                    && (event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("intrusionlogs"))
                        }
                        serde_json::Value::String(s) => s.contains("intrusionlogs"),
                        _ => false,
                    }) || event.get("log.file.path").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("dlplogs"))
                        }
                        serde_json::Value::String(s) => s.contains("dlplogs"),
                        _ => false,
                    }))
            };
            if _cond {
                event.set("event.severity", json!(0))?;
            }

            let _cond = { event.has_value("file.size") };
            if _cond {
                if let Some(val) = event.get("file.size") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "file.size".into(),
                            message,
                        }
                    })?;
                    event.set("file.size", converted)?;
                }
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.nat.ip")
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("source.domain")
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cisco.umbrella.fqdns") };
            if _cond {
                foreach_array(event, "cisco.umbrella.fqdns", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("cisco.umbrella.sha_sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("cisco.umbrella.sha_sha256")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            event.remove("cisco.umbrella._tmp");
            event.remove("cisco.umbrella.direction");
            event.remove("cisco.umbrella.action");
            event.remove("log.flags");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
