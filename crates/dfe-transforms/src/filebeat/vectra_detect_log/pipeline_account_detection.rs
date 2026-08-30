// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_account_detection` pipeline.
pub struct PipelineAccountDetection;

impl Transform for PipelineAccountDetection {
    fn name(&self) -> &str {
        "pipeline_account_detection"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("alert"))?;

                if event.has_value("json.href") {
                    event.rename("json.href", "vectra_detect.log.href")?;
                }

            if let Some(v) = event.get("vectra_detect.log.href").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reference", v)?;
            }

            let _cond = { event.has_value("event.reference") };
            if _cond {
                uri_parts(event, "event.reference", "url", true, false)?;
            }

                if event.has_value("json.dvchost") {
                    event.rename("json.dvchost", "vectra_detect.log.dvchost")?;
                }

            if let Some(v) = event.get("vectra_detect.log.dvchost").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.hostname", v)?;
            }

            let _cond = { event.has_value("observer.hostname") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("observer.hostname").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("json.severity") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.severity") {
                if let Some(val) = event.get("json.severity") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.severity".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.severity", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_severity_to_double")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get("json.service_info").is_some_and(|v| v.is_string()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.service_info", "json.service_info")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_to_split_service_info")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.service_info") {
                    event.rename("json.service_info", "vectra_detect.log.service.info")?;
                }

            let _cond = { event.get("vectra_detect.log.service.info").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.service.info", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.count") {
                    if let Some(val) = event.get("_ingest._value.count") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.count".into(),
                    message,
                    })?;
                    event.set("_ingest._value.counts", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_service_info_count_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("vectra_detect.log.service.info").is_some_and(|v| v.is_array()) };
            if _cond {
            if event.has_value("vectra_detect.log.service.info") {
                foreach_array(event, "vectra_detect.log.service.info", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value.account_uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }
            }

            let _cond = { event.get("vectra_detect.log.service.info").is_some_and(|v| v.is_array()) };
            if _cond {
            if event.has_value("vectra_detect.log.service.info") {
                foreach_array(event, "vectra_detect.log.service.info", |event| {
                    event.remove("_ingest._value.count");
                    Ok(())
                })?;
            }
            }

                if event.has_value("json.service_name") {
                    event.rename("json.service_name", "vectra_detect.log.service.name")?;
                }

            if let Some(v) = event.get("vectra_detect.log.service.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("service.name", v)?;
            }

                if event.has_value("json.account_uid") {
                    event.rename("json.account_uid", "vectra_detect.log.account.uid")?;
                }

            let _cond = { event.get("vectra_detect.log.account.uid").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")), serde_json::Value::String(s) => s.contains("@"), _ => false }) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("vectra_detect.log.account.uid") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("@") else { break 'dissect false };
                        captured.push(("vectra_detect.log.account.user_id", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("@") else { break 'dissect false };
                        remaining = rest;
                        captured.push(("user.domain", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                    else {
                        return Err(TransformError::ParseError {
                            path: "vectra_detect.log.account.uid".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dissect")?;
                event.set("_ingest.on_failure_processor_tag", "dissect_account_uid")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("vectra_detect.log.account.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("user.domain") && event.get_str("user.domain") != Some("") && event.get_str("vectra_detect.log.account.user_id") != Some("") };
            if _cond {
            if let Some(v) = event.get("vectra_detect.log.account.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.email", v)?;
            }
            }

            if event.has_value("user.domain") {
                if let Some(domain_str) = event.get_string("user.domain") {
                    let domain = domain_str.to_string();
                    event.set("vectra_detect.log.user.domain", json!(domain.clone()))?;
                    // Public suffix list lookup for registered domain extraction
                    if let Some(rd) = registered_domain_lookup(&domain) {
                        if let Some(registered) = rd.registered_domain {
                            event.set("vectra_detect.log.user.registered_domain", json!(registered))?;
                        }
                        event.set("vectra_detect.log.user.top_level_domain", json!(rd.top_level_domain))?;
                        if let Some(sub) = rd.subdomain {
                            event.set("vectra_detect.log.user.subdomain", json!(sub))?;
                        }
                    }
                }
            }

                if event.has_value("json.account_name") {
                    event.rename("json.account_name", "vectra_detect.log.account.name")?;
                }

            let _cond = { event.get("json.account_info").is_some_and(|v| v.is_string()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.account_info", "json.account_info")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_to_split_account_info")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.account_info") {
                    event.rename("json.account_info", "vectra_detect.log.account.info")?;
                }

            let _cond = { event.get("vectra_detect.log.account.info").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.account.info", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.count") {
                    if let Some(val) = event.get("_ingest._value.count") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.count".into(),
                    message,
                    })?;
                    event.set("_ingest._value.counts", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_account_info_count_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                    }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("vectra_detect.log.account.info").is_some_and(|v| v.is_array()) };
            if _cond {
            if event.has_value("vectra_detect.log.account.info") {
                foreach_array(event, "vectra_detect.log.account.info", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value.account_uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }
            }

            let _cond = { event.get("vectra_detect.log.account.info").is_some_and(|v| v.is_array()) };
            if _cond {
            if event.has_value("vectra_detect.log.account.info") {
                foreach_array(event, "vectra_detect.log.account.info", |event| {
                    event.remove("_ingest._value.count");
                    Ok(())
                })?;
            }
            }

            if let Some(v) = event.get("vectra_detect.log.account.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.name", v)?;
            }

            let _cond = { event.get_str("json.threat") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.threat") {
                if let Some(val) = event.get("json.threat") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.threat".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.threat.score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_threat_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.detection_id") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.detection_id") {
                if let Some(val) = event.get("json.detection_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.detection_id".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.detection.id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_detection_id_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("vectra_detect.log.detection.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.id", v)?;
            }

                if event.has_value("json.d_type_vname") {
                    event.rename("json.d_type_vname", "vectra_detect.log.d_type.vname")?;
                }

                if event.has_value("json.d_type") {
                    event.rename("json.d_type", "vectra_detect.log.d_type.name")?;
                }

            let _cond = { event.get_str("json.certainty") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.certainty") {
                if let Some(val) = event.get("json.certainty") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.certainty".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.certainty", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_certainty_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.dd_dst_port") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.dd_dst_port") {
                if let Some(val) = event.get("json.dd_dst_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.dd_dst_port".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.dd.dst.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dd_dst_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("vectra_detect.log.dd.dst.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }

            let _cond = { event.get_str("json.dd_dst_ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.dd_dst_ip") {
                if let Some(val) = event.get("json.dd_dst_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.dd_dst_ip".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.dd.dst.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dd_dst_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("vectra_detect.log.dd.dst.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
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

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("destination.ip").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.dd_dst_dns") {
                    event.rename("json.dd_dst_dns", "vectra_detect.log.dd.dst.dns")?;
                }

            if let Some(v) = event.get("vectra_detect.log.dd.dst.dns").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.domain", v)?;
            }

            let _cond = { event.get_str("json.dd_bytes_sent") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.dd_bytes_sent") {
                if let Some(val) = event.get("json.dd_bytes_sent") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.dd_bytes_sent".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.dd.bytes.sent", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dd_bytes_sent_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.dd_bytes_rcvd") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.dd_bytes_rcvd") {
                if let Some(val) = event.get("json.dd_bytes_rcvd") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.dd_bytes_rcvd".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.dd.bytes.rcvd", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dd_bytes_rcvd_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.category") {
                    event.rename("json.category", "vectra_detect.log.category")?;
                }

            let _cond = { event.get_str("json.triaged") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.triaged") {
                if let Some(val) = event.get("json.triaged") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.triaged".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.triaged", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_triaged_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.proxied_dst") {
                    event.rename("json.proxied_dst", "vectra_detect.log.proxied_dst")?;
                }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
