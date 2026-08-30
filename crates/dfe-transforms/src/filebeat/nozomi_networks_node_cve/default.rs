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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "json_event_original_a68ecd77",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            event.set("event.kind", json!("alert"))?;

            event.append_unique("event.category", json!("vulnerability"))?;

            event.append_unique("event.type", json!("info"))?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.cve_update_time") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.record_created_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.time") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.appliance_host") {
                event.rename(
                    "json.appliance_host",
                    "nozomi_networks.node_cve.appliance_host",
                )?;
            }

            if let Some(v) = event
                .get("nozomi_networks.node_cve.appliance_host")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("nozomi_networks.node_cve.appliance_host") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("nozomi_networks.node_cve.appliance_host")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.appliance_id") {
                event.rename("json.appliance_id", "nozomi_networks.node_cve.appliance_id")?;
            }

            if let Some(v) = event
                .get("nozomi_networks.node_cve.appliance_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            let _cond = { event.get_str("json.appliance_ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.appliance_ip") {
                        if let Some(val) = event.get("json.appliance_ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.appliance_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("nozomi_networks.node_cve.appliance_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_appliance_ip_to_ip_b3313175",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            let _cond = { event.has_value("nozomi_networks.node_cve.appliance_ip") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("nozomi_networks.node_cve.appliance_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("nozomi_networks.node_cve.appliance_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("nozomi_networks.node_cve.appliance_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.asset_id") {
                event.rename("json.asset_id", "nozomi_networks.node_cve.asset_id")?;
            }

            if let Some(v) = event
                .get("nozomi_networks.node_cve.asset_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.id", v)?;
            }

            if event.has_value("json.cve") {
                event.rename("json.cve", "nozomi_networks.node_cve.cve")?;
            }

            if let Some(v) = event
                .get("nozomi_networks.node_cve.cve")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.id", v)?;
            }

            let _cond = {
                event.has_value("vulnerability.id")
                    && event.get("vulnerability.id").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("CVE")),
                        serde_json::Value::String(s) => s.contains("CVE"),
                        _ => false,
                    })
            };
            if _cond {
                event.set(
                    "vulnerability.reference",
                    json!(format!(
                        "https://www.cve.org/CVERecord?id={}",
                        event
                            .get("vulnerability.id")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = { event.has_value("vulnerability.id") };
            if _cond {
                // Painless script
                // Source: String vulnerability_id = ctx.vulnerability.id.toUpperCase();\nfor (String enum: params.vulnerability_enumeration) {\n  if (vulnerability_id.contains(enum)) {\n    ctx.vulnerability.put('enumeration', enum);\n    return;\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"String vulnerability_id = ctx.vulnerability.id.toUpperCase();\nfor (String enum: params.vulnerability_enumeration) {\n  if (vulnerability_id.contains(enum)) {\n    ctx.vulnerability.put('enumeration', enum);\n    return;\n  }\n}\n"#
                    ),
                    cached_params!("{\"vulnerability_enumeration\":[\"CVE\",\"TVM\"]}"),
                )?;
            }

            let _cond = { event.has_value("json.cve_creation_time") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.cve_creation_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("nozomi_networks.node_cve.cve_creation_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.cve_creation_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_cve_creation_time_fbb5414e",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            if let Some(v) = event
                .get("nozomi_networks.node_cve.cve_creation_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.published_date", v)?;
            }

            if let Some(v) = event
                .get("nozomi_networks.node_cve.cve_creation_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.cve_epss_score") {
                    if let Some(val) = event.get("json.cve_epss_score") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.cve_epss_score".into(),
                                message,
                            }
                        })?;
                        event.set("nozomi_networks.node_cve.cve_epss_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cve_epss_score_to_double_2105d4ca",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.cve_references") {
                event.rename(
                    "json.cve_references",
                    "nozomi_networks.node_cve.cve_references",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.cve_score") {
                    if let Some(val) = event.get("json.cve_score") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.cve_score".into(),
                                message,
                            }
                        })?;
                        event.set("nozomi_networks.node_cve.cve_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cve_score_to_double_2a8032e0",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if let Some(v) = event
                .get("nozomi_networks.node_cve.cve_score")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.score.base", v)?;
            }

            if event.has_value("json.cve_source") {
                event.rename("json.cve_source", "nozomi_networks.node_cve.cve_source")?;
            }

            if event.has_value("json.cve_summary") {
                event.rename("json.cve_summary", "nozomi_networks.node_cve.cve_summary")?;
            }

            if let Some(v) = event
                .get("nozomi_networks.node_cve.cve_summary")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("vulnerability.description", v)?;
            }

            if let Some(v) = event
                .get("nozomi_networks.node_cve.cve_summary")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = { event.has_value("json.cve_update_time") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.cve_update_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("nozomi_networks.node_cve.cve_update_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.cve_update_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_cve_update_time_7c12b66b",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            if let Some(v) = event
                .get("nozomi_networks.node_cve.cve_update_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            if event.has_value("json.cwe_id") {
                event.rename("json.cwe_id", "nozomi_networks.node_cve.cwe_id")?;
            }

            if event.has_value("json.cwe_name") {
                event.rename("json.cwe_name", "nozomi_networks.node_cve.cwe_name")?;
            }

            let _cond = { event.has_value("nozomi_networks.node_cve.cwe_name") };
            if _cond {
                event.append_unique(
                    "vulnerability.category",
                    json!(
                        event
                            .get("nozomi_networks.node_cve.cwe_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.firmware_version") {
                event.rename(
                    "json.firmware_version",
                    "nozomi_networks.node_cve.firmware_version",
                )?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "nozomi_networks.node_cve.id")?;
            }

            if let Some(v) = event
                .get("nozomi_networks.node_cve.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.installed_on") {
                event.rename("json.installed_on", "nozomi_networks.node_cve.installed_on")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.is_kev") {
                    if let Some(val) = event.get("json.is_kev") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.is_kev".into(),
                                message,
                            }
                        })?;
                        event.set("nozomi_networks.node_cve.is_kev", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_is_kev_to_boolean_5bc419d1",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.latest_hotfix") {
                event.rename(
                    "json.latest_hotfix",
                    "nozomi_networks.node_cve.latest_hotfix",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.likelihood") {
                    if let Some(val) = event.get("json.likelihood") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.likelihood".into(),
                                message,
                            }
                        })?;
                        event.set("nozomi_networks.node_cve.likelihood", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_likelihood_to_double_99cf26da",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.matching_cpes") {
                event.rename(
                    "json.matching_cpes",
                    "nozomi_networks.node_cve.matching_cpes",
                )?;
            }

            if event.has_value("json.minimum_hotfix") {
                event.rename(
                    "json.minimum_hotfix",
                    "nozomi_networks.node_cve.minimum_hotfix",
                )?;
            }

            if event.has_value("json.name") {
                event.rename("json.name", "nozomi_networks.node_cve.name")?;
            }

            if event.has_value("json.node_firmware_version") {
                event.rename(
                    "json.node_firmware_version",
                    "nozomi_networks.node_cve.node_firmware_version",
                )?;
            }

            if event.has_value("json.node_id") {
                event.rename("json.node_id", "nozomi_networks.node_cve.node_id")?;
            }

            if event.has_value("json.node_label") {
                event.rename("json.node_label", "nozomi_networks.node_cve.node_label")?;
            }

            if event.has_value("json.node_os") {
                event.rename("json.node_os", "nozomi_networks.node_cve.node_os")?;
            }

            if event.has_value("json.node_product_name") {
                event.rename(
                    "json.node_product_name",
                    "nozomi_networks.node_cve.node_product_name",
                )?;
            }

            if event.has_value("json.node_type") {
                event.rename("json.node_type", "nozomi_networks.node_cve.node_type")?;
            }

            if event.has_value("json.node_vendor") {
                event.rename("json.node_vendor", "nozomi_networks.node_cve.node_vendor")?;
            }

            let _cond = { event.get("json.nodes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.nodes", |event| {
                    // on_failure: 3 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value") {
                            if let Some(val) = event.get("_ingest._value") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_nodes_to_ip_0d056542",
                        )?;
                        event.append_unique(
                            "nozomi_networks.node_cve.nodes_hosts",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest._value");
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

            let _cond = { event.get("json.nodes").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.nodes", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.nodes") {
                event.rename("json.nodes", "nozomi_networks.node_cve.nodes_ip")?;
            }

            if event.has_value("json.os") {
                event.rename("json.os", "nozomi_networks.node_cve.os")?;
            }

            let _cond = { event.get_str("nozomi_networks.node_cve.firmware_version") == Some("") };
            if _cond {
                if let Some(v) = event
                    .get("nozomi_networks.node_cve.os")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.family", v)?;
                }
            }

            if event.has_value("json.probability") {
                event.rename("json.probability", "nozomi_networks.node_cve.probability")?;
            }

            if event.has_value("json.product_name") {
                event.rename("json.product_name", "nozomi_networks.node_cve.product_name")?;
            }

            let _cond = { event.has_value("json.record_created_at") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.record_created_at") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("nozomi_networks.node_cve.record_created_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.record_created_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_record_created_at_5688d3ce",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            if let Some(v) = event
                .get("nozomi_networks.node_cve.record_created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = { event.get("json.references").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.references", |event| {
                    event.append_unique(
                        "vulnerability.scanner.vendor",
                        json!(
                            event
                                .get("_ingest._value.source")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.references").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.references", |event| {
                    event.append_unique(
                        "vulnerability.reference",
                        json!(
                            event
                                .get("_ingest._value.url")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.references") {
                event.rename("json.references", "nozomi_networks.node_cve.references")?;
            }

            if event.has_value("json.resolution_reason") {
                event.rename(
                    "json.resolution_reason",
                    "nozomi_networks.node_cve.resolution_reason",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.resolved") {
                    if let Some(val) = event.get("json.resolved") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.resolved".into(),
                                message,
                            }
                        })?;
                        event.set("nozomi_networks.node_cve.resolved", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_resolved_to_boolean_ed4b7b56",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.resolved_source") {
                event.rename(
                    "json.resolved_source",
                    "nozomi_networks.node_cve.resolved_source",
                )?;
            }

            let _cond = { event.has_value("json.time") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("nozomi_networks.node_cve.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_time_2d9b5f63")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
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

            if event.has_value("json.type") {
                event.rename("json.type", "nozomi_networks.node_cve.type")?;
            }

            if event.has_value("json.vendor") {
                event.rename("json.vendor", "nozomi_networks.node_cve.vendor")?;
            }

            if event.has_value("json.zone") {
                event.rename("json.zone", "nozomi_networks.node_cve.zone")?;
            }

            if event.has_value("json.zones") {
                event.rename("json.zones", "nozomi_networks.node_cve.zones")?;
            }

            let _cond = {
                event
                    .get("nozomi_networks.node_cve.references")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "nozomi_networks.node_cve.references", |event| {
                    let _cond = {
                        !event.has_value("tags")
                            || !(event.get("tags").is_some_and(|v| match v {
                                serde_json::Value::Array(a) => a.iter().any(|x| {
                                    x.as_str() == Some("preserve_duplicate_custom_fields")
                                }),
                                serde_json::Value::String(s) => {
                                    s.contains("preserve_duplicate_custom_fields")
                                }
                                _ => false,
                            }))
                    };
                    if _cond {
                        event.remove("_ingest._value.url");
                        event.remove("_ingest._value.source");
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("nozomi_networks.node_cve.appliance_host");
                event.remove("nozomi_networks.node_cve.appliance_id");
                event.remove("nozomi_networks.node_cve.appliance_ip");
                event.remove("nozomi_networks.node_cve.asset_id");
                event.remove("nozomi_networks.node_cve.cve");
                event.remove("nozomi_networks.node_cve.cve_creation_time");
                event.remove("nozomi_networks.node_cve.cve_score");
                event.remove("nozomi_networks.node_cve.cve_summary");
                event.remove("nozomi_networks.node_cve.cve_update_time");
                event.remove("nozomi_networks.node_cve.cwe_name");
                event.remove("nozomi_networks.node_cve.id");
                event.remove("nozomi_networks.node_cve.record_created_at");
            }

            event.remove("json");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
                        "Processor '{}'\n{}failed with message '{}'",
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
                                "with tag '{}'\n",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
