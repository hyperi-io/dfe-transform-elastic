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

            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
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
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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

            if event.has_value("json.category") {
                event.rename("json.category", "m365_defender.event.category")?;
            }

            if event.has_value("json.operationName") {
                event.rename("json.operationName", "m365_defender.event.operation_name")?;
            }

            if event.has_value("json.Tenant") {
                event.rename("json.Tenant", "m365_defender.event.tenant.name")?;
            }

            if event.has_value("json.tenantId") {
                event.rename("json.tenantId", "m365_defender.event.tenant.id")?;
            }

            let _cond = { event.has_value("json.time") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("m365_defender.event.time", parsed)?,
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
                    event.set("_ingest.on_failure_processor_tag", "date_json_time")?;
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

            let _cond = { event.has_value("json.properties.Timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.properties.Timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("m365_defender.event.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.Timestamp".into(),
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
                        "date_json_properties_Timestamp",
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
                .get("m365_defender.event.timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                !event.has_value("m365_defender.event.category")
                    || event.get_str("m365_defender.event.category") == Some("")
            };
            if _cond {
                event.append(
                    "error.message",
                    json!("Missing required field: m365_defender.event.category."),
                )?;
            }

            let _cond = {
                event.has_value("m365_defender.event.category")
                    && !([
                        "AdvancedHunting-AlertEvidence",
                        "AdvancedHunting-AlertInfo",
                        "AdvancedHunting-BehaviorEntities",
                        "AdvancedHunting-BehaviorInfo",
                        "AdvancedHunting-CloudAppEvents",
                        "AdvancedHunting-CloudAuditEvents",
                        "AdvancedHunting-CloudProcessEvents",
                        "AdvancedHunting-CloudStorageAggregatedEvents",
                        "AdvancedHunting-DeviceEvents",
                        "AdvancedHunting-DeviceFileCertificateInfo",
                        "AdvancedHunting-DeviceFileEvents",
                        "AdvancedHunting-DeviceImageLoadEvents",
                        "AdvancedHunting-DeviceInfo",
                        "AdvancedHunting-DeviceLogonEvents",
                        "AdvancedHunting-DeviceNetworkEvents",
                        "AdvancedHunting-DeviceNetworkInfo",
                        "AdvancedHunting-DeviceProcessEvents",
                        "AdvancedHunting-DeviceRegistryEvents",
                        "AdvancedHunting-EmailAttachmentInfo",
                        "AdvancedHunting-EmailEvents",
                        "AdvancedHunting-EmailPostDeliveryEvents",
                        "AdvancedHunting-EmailUrlInfo",
                        "AdvancedHunting-IdentityDirectoryEvents",
                        "AdvancedHunting-IdentityInfo",
                        "AdvancedHunting-IdentityLogonEvents",
                        "AdvancedHunting-IdentityQueryEvents",
                        "AdvancedHunting-MessageEvents",
                        "AdvancedHunting-MessagePostDeliveryEvents",
                        "AdvancedHunting-MessageUrlInfo",
                        "AdvancedHunting-UrlClickEvents",
                    ]
                    .contains(&event.get_str("m365_defender.event.category").unwrap_or("")))
            };
            if _cond {
                event.append(
                    "error.message",
                    json!(format!(
                        "The event category {} is not supported.",
                        event
                            .get("m365_defender.event.category")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.has_value("m365_defender.event.category")
                    && (event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("alert"))
                        || event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("behavior")))
            };
            if _cond {
                // Begin nested pipeline: "pipeline_alert"
                event.set("event.kind", json!("alert"))?;
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("behavior"))
                };
                if _cond {
                    event.set("event.kind", json!("event"))?;
                }
                let _cond = {
                    event.has_value("json.properties.EntityType")
                        && event
                            .get_str("json.properties.EntityType")
                            .is_some_and(|s| s.to_lowercase() == "file")
                };
                if _cond {
                    event.append("event.category", json!("file"))?;
                }
                let _cond = {
                    event.has_value("json.properties.EntityType")
                        && event
                            .get_str("json.properties.EntityType")
                            .is_some_and(|s| s.to_lowercase() == "process")
                };
                if _cond {
                    event.append("event.category", json!("process"))?;
                }
                let _cond = {
                    event.has_value("json.properties.EntityType")
                        && event
                            .get_str("json.properties.EntityType")
                            .is_some_and(|s| s.to_lowercase() == "device")
                };
                if _cond {
                    event.append("event.category", json!("host"))?;
                }
                let _cond = {
                    event.has_value("json.properties.EntityType")
                        && event
                            .get_str("json.properties.EntityType")
                            .is_some_and(|s| s.to_lowercase() == "user")
                };
                if _cond {
                    event.append("event.category", json!("iam"))?;
                }
                let _cond = {
                    event.has_value("json.properties.Category")
                        && event.get_str("json.properties.Category").is_some_and(|s| {
                            ["malware", "ransomware"].contains(&s.to_lowercase().as_str())
                        })
                };
                if _cond {
                    event.append("event.category", json!("malware"))?;
                }
                let _cond = {
                    event.get_str("m365_defender.event.category")
                        != Some("AdvancedHunting-AlertInfo")
                        && event.has_value("json.properties.Category")
                        && event.get_str("json.properties.Category").is_some_and(|s| {
                            [
                                "persistence",
                                "privilegeescalation",
                                "suspiciousactivity",
                                "threatmanagement",
                            ]
                            .contains(&s.to_lowercase().as_str())
                        })
                };
                if _cond {
                    event.append("event.category", json!("threat"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        })
                };
                if _cond {
                    event.append("event.type", json!("indicator"))?;
                }
                let _cond = { !event.has_value("event.type") };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                let _cond = {
                    event
                        .get("json.properties.Categories")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.Categories") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.Categories",
                            "json.properties.Categories",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_properties_Categories",
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
                let _cond = {
                    event
                        .get("json.properties.AdditionalFields")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.AdditionalFields") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.AdditionalFields",
                            "json.properties.AdditionalFields",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_properties_AdditionalFields",
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
                let _cond = {
                    event
                        .get("json.properties.AttackTechniques")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.AttackTechniques") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.AttackTechniques",
                            "json.properties.AttackTechniques",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_properties_AttackTechniques",
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
                let _cond = { event.get_str("json.properties.RemoteIP") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.RemoteIP") {
                            if let Some(val) = event.get("json.properties.RemoteIP") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.RemoteIP".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.remote.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_json_properties_RemoteIP",
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
                let _cond = { event.get_str("json.properties.FileSize") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.FileSize") {
                            if let Some(val) = event.get("json.properties.FileSize") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.FileSize".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.file.size", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_json_properties_FileSize",
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
                let _cond = { event.get_str("json.properties.LocalIP") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.LocalIP") {
                            if let Some(val) = event.get("json.properties.LocalIP") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.LocalIP".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.local.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_json_properties_LocalIP",
                        )?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
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
                    event.has_value("json.properties.StartTime")
                        && event.get_str("json.properties.StartTime") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.properties.StartTime") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("m365_defender.event.start_time", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.properties.StartTime".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_StartTime")?;
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
                let _cond = {
                    event.has_value("json.properties.EndTime")
                        && event.get_str("json.properties.EndTime") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("json.properties.EndTime") {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("m365_defender.event.end_time", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.properties.EndTime".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_EndTime")?;
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
                let _cond = {
                    event.has_value("json.properties.DataAggregationEndTime")
                        && event.get_str("json.properties.DataAggregationEndTime") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.properties.DataAggregationEndTime")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event
                                    .set("m365_defender.event.data_aggregation_end_time", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.properties.DataAggregationEndTime".into(),
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
                            "date_DataAggregationEndTime",
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
                let _cond = {
                    event.has_value("json.properties.DataAggregationStartTime")
                        && event.get_str("json.properties.DataAggregationStartTime") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.properties.DataAggregationStartTime")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set(
                                    "m365_defender.event.data_aggregation_start_time",
                                    parsed,
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.properties.DataAggregationStartTime".into(),
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
                            "date_DataAggregationStartTime",
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
                let _cond = { event.get_str("json.properties.EmailClusterId") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.EmailClusterId") {
                            if let Some(val) = event.get("json.properties.EmailClusterId") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.properties.EmailClusterId".into(),
                                            message,
                                        }
                                    })?;
                                event.set("m365_defender.event.email.cluster_id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_email_cluster_id",
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
                if event.has_value("json.properties.AlertId") {
                    event.rename("json.properties.AlertId", "m365_defender.event.alert.id")?;
                }
                if event.has_value("json.properties.ServiceSource") {
                    event.rename(
                        "json.properties.ServiceSource",
                        "m365_defender.event.service_source",
                    )?;
                }
                if event.has_value("json.properties.DeviceName") {
                    event.rename(
                        "json.properties.DeviceName",
                        "m365_defender.event.device.name",
                    )?;
                }
                if event.has_value("json.properties.NetworkMessageId") {
                    event.rename(
                        "json.properties.NetworkMessageId",
                        "m365_defender.event.network.message_id",
                    )?;
                }
                if event.has_value("json.properties.OAuthApplicationId") {
                    event.rename(
                        "json.properties.OAuthApplicationId",
                        "m365_defender.event.oauth_application_id",
                    )?;
                }
                if event.has_value("json.properties.RemoteUrl") {
                    event.rename(
                        "json.properties.RemoteUrl",
                        "m365_defender.event.remote.url",
                    )?;
                }
                if event.has_value("json.properties.AttackTechniques") {
                    event.rename(
                        "json.properties.AttackTechniques",
                        "m365_defender.event.attack_techniques",
                    )?;
                }
                if event.has_value("json.properties.AccountObjectId") {
                    event.rename(
                        "json.properties.AccountObjectId",
                        "m365_defender.event.account.object_id",
                    )?;
                }
                if event.has_value("json.properties.Category") {
                    event.rename(
                        "json.properties.Category",
                        "m365_defender.event.alert.category",
                    )?;
                }
                if event.has_value("json.properties.Categories") {
                    event.rename(
                        "json.properties.Categories",
                        "m365_defender.event.alert.categories",
                    )?;
                }
                if event.has_value("json.properties.DetectionSource") {
                    event.rename(
                        "json.properties.DetectionSource",
                        "m365_defender.event.detection.source",
                    )?;
                }
                if event.has_value("json.properties.MachineGroup") {
                    event.rename(
                        "json.properties.MachineGroup",
                        "m365_defender.event.machine_group",
                    )?;
                }
                if event.has_value("json.properties.DeviceId") {
                    event.rename("json.properties.DeviceId", "m365_defender.event.device.id")?;
                }
                if event.has_value("json.properties.EvidenceDirection") {
                    event.rename(
                        "json.properties.EvidenceDirection",
                        "m365_defender.event.evidence.direction",
                    )?;
                }
                if event.has_value("json.properties.ProcessCommandLine") {
                    event.rename(
                        "json.properties.ProcessCommandLine",
                        "m365_defender.event.process.command_line",
                    )?;
                }
                if event.has_value("json.properties.RegistryKey") {
                    event.rename(
                        "json.properties.RegistryKey",
                        "m365_defender.event.registry.key",
                    )?;
                }
                if event.has_value("json.properties.RegistryValueName") {
                    event.rename(
                        "json.properties.RegistryValueName",
                        "m365_defender.event.registry.value_name",
                    )?;
                }
                if event.has_value("json.properties.RegistryValueData") {
                    event.rename(
                        "json.properties.RegistryValueData",
                        "m365_defender.event.registry.value_data",
                    )?;
                }
                if event.has_value("json.properties.SHA1") {
                    event.rename("json.properties.SHA1", "m365_defender.event.sha1")?;
                }
                if event.has_value("json.properties.FolderPath") {
                    event.rename(
                        "json.properties.FolderPath",
                        "m365_defender.event.folder_path",
                    )?;
                }
                if event.has_value("json.properties.SHA256") {
                    event.rename("json.properties.SHA256", "m365_defender.event.sha256")?;
                }
                if event.has_value("json.properties.FileName") {
                    event.rename("json.properties.FileName", "m365_defender.event.file.name")?;
                }
                if event.has_value("json.properties.ThreatFamily") {
                    event.rename(
                        "json.properties.ThreatFamily",
                        "m365_defender.event.threat.family",
                    )?;
                }
                if event.has_value("json.properties.AccountSid") {
                    event.rename(
                        "json.properties.AccountSid",
                        "m365_defender.event.account.sid",
                    )?;
                }
                if event.has_value("json.properties.AccountName") {
                    event.rename(
                        "json.properties.AccountName",
                        "m365_defender.event.account.name",
                    )?;
                }
                if event.has_value("json.properties.Title") {
                    event.rename("json.properties.Title", "m365_defender.event.title")?;
                }
                if event.has_value("json.properties.AccountDomain") {
                    event.rename(
                        "json.properties.AccountDomain",
                        "m365_defender.event.account.domain",
                    )?;
                }
                if event.has_value("json.properties.AccountUpn") {
                    event.rename(
                        "json.properties.AccountUpn",
                        "m365_defender.event.account.upn",
                    )?;
                }
                if event.has_value("json.properties.AdditionalFields") {
                    event.rename(
                        "json.properties.AdditionalFields",
                        "m365_defender.event.additional_fields",
                    )?;
                }
                if event.has_value("json.properties.Application") {
                    event.rename(
                        "json.properties.Application",
                        "m365_defender.event.application",
                    )?;
                }
                if event.has_value("json.properties.ApplicationId") {
                    if let Some(val) = event.get("json.properties.ApplicationId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.properties.ApplicationId".into(),
                                message,
                            }
                        })?;
                        event.set("m365_defender.event.application_id", converted)?;
                    }
                }
                if event.has_value("json.properties.ActionType") {
                    event.rename(
                        "json.properties.ActionType",
                        "m365_defender.event.action.type",
                    )?;
                }
                if event.has_value("json.properties.BehaviorId") {
                    event.rename(
                        "json.properties.BehaviorId",
                        "m365_defender.event.behavior_id",
                    )?;
                }
                if event.has_value("json.properties.CloudResourceType") {
                    event.rename(
                        "json.properties.CloudResourceType",
                        "m365_defender.event.cloud_resource_type",
                    )?;
                }
                if event.has_value("json.properties.CloudResourceId") {
                    event.rename(
                        "json.properties.CloudResourceId",
                        "m365_defender.event.cloud_resource_id",
                    )?;
                }
                if event.has_value("json.properties.CloudSubscriptionId") {
                    event.rename(
                        "json.properties.CloudSubscriptionId",
                        "m365_defender.event.cloud_subscription_id",
                    )?;
                }
                if event.has_value("json.properties.CloudPlatform") {
                    event.rename(
                        "json.properties.CloudPlatform",
                        "m365_defender.event.cloud_platform",
                    )?;
                }
                if event.has_value("json.properties.DataSources") {
                    event.rename(
                        "json.properties.DataSources",
                        "m365_defender.event.data_sources",
                    )?;
                }
                if event.has_value("json.properties.Description") {
                    event.rename(
                        "json.properties.Description",
                        "m365_defender.event.description",
                    )?;
                }
                if event.has_value("json.properties.DetailedEntityRole") {
                    event.rename(
                        "json.properties.DetailedEntityRole",
                        "m365_defender.event.detailed_entity_role",
                    )?;
                }
                if event.has_value("json.properties.EmailSubject") {
                    event.rename(
                        "json.properties.EmailSubject",
                        "m365_defender.event.email.subject",
                    )?;
                }
                if event.has_value("json.properties.EntityType") {
                    event.rename(
                        "json.properties.EntityType",
                        "m365_defender.event.entity_type",
                    )?;
                }
                if event.has_value("json.properties.EvidenceRole") {
                    event.rename(
                        "json.properties.EvidenceRole",
                        "m365_defender.event.evidence.role",
                    )?;
                }
                if event.has_value("json.properties.EntityRole") {
                    event.rename(
                        "json.properties.EntityRole",
                        "m365_defender.event.entity_role",
                    )?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.alert.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
                if let Some(v) = event.get("m365_defender.event.service_source").cloned() {
                    event.set("event.provider", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.behavior_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.remote.url")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.reference", v)?;
                }
                if event.has_value("json.properties.Severity") {
                    map_strings(
                        event,
                        "json.properties.Severity",
                        "m365_defender.event.severity",
                        str::to_lowercase,
                    )?;
                }
                let _cond = {
                    event
                        .get("m365_defender.event.severity")
                        .is_some_and(|v| v.is_string())
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        // Painless script
                        // Source: ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.m365_defender.event.severity;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.m365_defender.event.severity;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}"#
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "script")?;
                        event.set("_ingest.on_failure_processor_tag", "set_event_severity")?;
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
                    .get("m365_defender.event.device.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
                if event.has_value("host.name") {
                    map_strings(event, "host.name", "host.name", str::to_lowercase)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.device.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.id", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.evidence.direction")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("network.direction", v)?;
                }
                if event.has_value("network.direction") {
                    map_strings(
                        event,
                        "network.direction",
                        "network.direction",
                        str::to_lowercase,
                    )?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.process.command_line")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.command_line", v)?;
                }
                let _cond = {
                    event.has_value("process.command_line")
                        && event.get_str("process.command_line") != Some("")
                };
                if _cond {
                    // Painless script
                    // Source: // appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string into next\n// argument and command line remainder offset.\ndef readNextArg(String line, int offset) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; offset < line.length(); offset++) {\n        def c = line.charAt(offset);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"offset\": offset+1\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && offset+1 < line.length() && line.charAt(offset+1) == (char)'\"') {\n                    b.append(c);\n                    offset++;\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"offset\": line.length()\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String line) {\n    def args = new ArrayList();\n    for (int i = 0; i < line.length();) {\n        if (line.charAt(i) == (char)' ' || line.charAt(i) == (char)0x09) {\n            i++;\n            continue;\n        }\n        def next = readNextArg(line, i);\n        i = next.offset;\n        if (next.arg == '') {\n            // Empty strings will be removed later so don't bother adding them.\n            continue;\n        }\n        args.add(next.arg);\n    }\n    return args;\n}\n\nctx.process.args = commandLineToArgv(ctx.process.command_line);\nctx.process.args_count = ctx.process.args.length;\nif (ctx.process.args.length > 0) {\n    ctx.process.executable = ctx.process.args[0];\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"// appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string into next\n// argument and command line remainder offset.\ndef readNextArg(String line, int offset) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; offset < line.length(); offset++) {\n        def c = line.charAt(offset);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"offset\": offset+1\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && offset+1 < line.length() && line.charAt(offset+1) == (char)'\"') {\n                    b.append(c);\n                    offset++;\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"offset\": line.length()\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String line) {\n    def args = new ArrayList();\n    for (int i = 0; i < line.length();) {\n        if (line.charAt(i) == (char)' ' || line.charAt(i) == (char)0x09) {\n            i++;\n            continue;\n        }\n        def next = readNextArg(line, i);\n        i = next.offset;\n        if (next.arg == '') {\n            // Empty strings will be removed later so don't bother adding them.\n            continue;\n        }\n        args.add(next.arg);\n    }\n    return args;\n}\n\nctx.process.args = commandLineToArgv(ctx.process.command_line);\nctx.process.args_count = ctx.process.args.length;\nif (ctx.process.args.length > 0) {\n    ctx.process.executable = ctx.process.args[0];\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.registry.key")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("threat.indicator.registry.key", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        })
                        && event.has_value("m365_defender.event.registry.value_data")
                };
                if _cond {
                    event.append_unique(
                        "threat.indicator.registry.data.strings",
                        json!(
                            event
                                .get("m365_defender.event.registry.value_data")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.registry.value_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("threat.indicator.registry.value", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.folder_path")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("threat.indicator.file.directory", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.sha1")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("threat.indicator.file.hash.sha1", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.sha256")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("threat.indicator.file.hash.sha256", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.file.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("threat.indicator.file.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.file.size")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("threat.indicator.file.size", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.threat.family")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("threat.group.name", v)?;
                    }
                }
                let _cond = {
                    event
                        .get("m365_defender.event.attack_techniques")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        // Painless script
                        // Source: def subtechnique_name = new ArrayList();\ndef subtechnique_id = new ArrayList();\nif (!(ctx.threat instanceof HashMap)) {\n  ctx.threat = new HashMap();\n}\nif (!(ctx.threat.technique instanceof HashMap)) {\n  ctx.threat.technique = new HashMap();\n}\nif (!(ctx.threat.technique.subtechnique instanceof HashMap)) {\n  ctx.threat.technique.subtechnique = new HashMap();\n}\nfor (item in ctx.m365_defender.event.attack_techniques) {\n  subtechnique_name.add(item.substring(0,item.lastIndexOf(' ')));\n  subtechnique_id.add(item.substring(item.indexOf('(')+1,item.indexOf(')')));\n}\nctx.threat.technique.subtechnique.id = subtechnique_id;\nctx.threat.technique.subtechnique.name = subtechnique_name;\n
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"def subtechnique_name = new ArrayList();\ndef subtechnique_id = new ArrayList();\nif (!(ctx.threat instanceof HashMap)) {\n  ctx.threat = new HashMap();\n}\nif (!(ctx.threat.technique instanceof HashMap)) {\n  ctx.threat.technique = new HashMap();\n}\nif (!(ctx.threat.technique.subtechnique instanceof HashMap)) {\n  ctx.threat.technique.subtechnique = new HashMap();\n}\nfor (item in ctx.m365_defender.event.attack_techniques) {\n  subtechnique_name.add(item.substring(0,item.lastIndexOf(' ')));\n  subtechnique_id.add(item.substring(item.indexOf('(')+1,item.indexOf(')')));\n}\nctx.threat.technique.subtechnique.id = subtechnique_id;\nctx.threat.technique.subtechnique.name = subtechnique_name;\n"#
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("m365_defender.event.attack_techniques")
                        && event.get_str("m365_defender.event.attack_techniques") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        let sorted = event
                            .get("m365_defender.event.attack_techniques")
                            .and_then(|v| sort_values(v, false));
                        if event.has("m365_defender.event.attack_techniques") {
                            match sorted {
                                Some(sorted) => event.set(
                                    "m365_defender.event.attack_techniques",
                                    Value::Array(sorted),
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "m365_defender.event.attack_techniques".into(),
                                        message: "cannot sort: not an array of one comparable kind"
                                            .into(),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "sort")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "sort_m365_defender_event_attack_techniques",
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
                let _cond = {
                    event.has_value("threat.technique.subtechnique.id")
                        && event.get_str("threat.technique.subtechnique.id") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        let sorted = event
                            .get("threat.technique.subtechnique.id")
                            .and_then(|v| sort_values(v, false));
                        if event.has("threat.technique.subtechnique.id") {
                            match sorted {
                                Some(sorted) => event.set(
                                    "threat.technique.subtechnique.id",
                                    Value::Array(sorted),
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "threat.technique.subtechnique.id".into(),
                                        message: "cannot sort: not an array of one comparable kind"
                                            .into(),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "sort")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "sort_threat_technique_subtechnique_id",
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
                let _cond = {
                    event.has_value("threat.technique.subtechnique.name")
                        && event.get_str("threat.technique.subtechnique.name") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        let sorted = event
                            .get("threat.technique.subtechnique.name")
                            .and_then(|v| sort_values(v, false));
                        if event.has("threat.technique.subtechnique.name") {
                            match sorted {
                                Some(sorted) => event.set(
                                    "threat.technique.subtechnique.name",
                                    Value::Array(sorted),
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "threat.technique.subtechnique.name".into(),
                                        message: "cannot sort: not an array of one comparable kind"
                                            .into(),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "sort")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "sort_threat_technique_subtechnique_name",
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
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.folder_path")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.directory", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.sha1")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.hash.sha1", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.sha256")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.hash.sha256", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.file.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.file.size")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.size", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.registry.key")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("registry.key", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("threat"))
                            }
                            serde_json::Value::String(s) => s.contains("threat"),
                            _ => false,
                        }))
                        && event.has_value("m365_defender.event.registry.value_data")
                };
                if _cond {
                    event.append_unique(
                        "registry.data.strings",
                        json!(
                            event
                                .get("m365_defender.event.registry.value_data")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.remote.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.ip", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.local.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.ip", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.account.sid")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.account.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.account.domain")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.domain", v)?;
                }
                let _cond = { !event.has_value("user.id") };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.account.object_id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.id", v)?;
                    }
                }
                if let Some(v) = event
                    .get("m365_defender.event.email.subject")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("email.subject", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.network.message_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("email.message_id", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.cloud_platform")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.provider", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.title")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("message", v)?;
                }
                let _cond = { event.has_value("host.name") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("user.domain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("user.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("host.id") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.account.object_id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("m365_defender.event.account.object_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("user.name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
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
                let _cond = { event.has_value("m365_defender.event.sha256") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("m365_defender.event.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.sha1") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("m365_defender.event.sha1")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
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
                    event.remove("m365_defender.event.timestamp");
                    event.remove("m365_defender.event.remote.url");
                    event.remove("m365_defender.event.alert.id");
                    event.remove("m365_defender.event.cloud_platform");
                    event.remove("m365_defender.event.service_source");
                    event.remove("m365_defender.event.device.name");
                    event.remove("m365_defender.event.device.id");
                    event.remove("m365_defender.event.evidence.direction");
                    event.remove("m365_defender.event.process.command_line");
                    event.remove("m365_defender.event.registry.key");
                    event.remove("m365_defender.event.registry.value_name");
                    event.remove("m365_defender.event.registry.value_data");
                    event.remove("m365_defender.event.remote.ip");
                    event.remove("m365_defender.event.folder_path");
                    event.remove("m365_defender.event.sha1");
                    event.remove("m365_defender.event.sha256");
                    event.remove("m365_defender.event.file.name");
                    event.remove("m365_defender.event.file.size");
                    event.remove("m365_defender.event.threat.family");
                    event.remove("m365_defender.event.account.name");
                    event.remove("m365_defender.event.account.domain");
                    event.remove("m365_defender.event.account.sid");
                    event.remove("m365_defender.event.account.object_id");
                    event.remove("m365_defender.event.network.message_id");
                    event.remove("m365_defender.event.email.subject");
                    event.remove("m365_defender.event.title");
                    event.remove("m365_defender.event.behavior_id");
                }
                // End nested pipeline: "pipeline_alert"
            }

            let _cond = {
                event.has_value("m365_defender.event.category")
                    && event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("device"))
                    || event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("processevents"))
            };
            if _cond {
                // Begin nested pipeline: "pipeline_device"
                let _cond = {
                    !event.has_value("m365_defender.event.category")
                        || event.get_str("m365_defender.event.category") == Some("")
                };
                if _cond {
                    return Err(TransformError::ParseError {
                        path: "_fail".into(),
                        message: ("Event does not contain a valid category.").to_string(),
                    });
                }
                event.set("event.kind", json!("event"))?;
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("devicelogonevents"))
                };
                if _cond {
                    event.append("event.category", json!("authentication"))?;
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("deviceinfo"))
                        || (event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))
                            && event.has_value("json.properties.ActionType")
                            && !(event
                                .get_str("json.properties.ActionType")
                                .is_some_and(|s| s.to_lowercase().ends_with("apicall")))
                            && !(event
                                .get_str("json.properties.ActionType")
                                .is_some_and(|s| s.to_lowercase().contains("driverload"))))
                };
                if _cond {
                    event.append("event.category", json!("host"))?;
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("deviceevents"))
                        && event.has_value("json.properties.ActionType")
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().ends_with("apicall"))
                };
                if _cond {
                    event.append("event.category", json!("api"))?;
                }
                let _cond = {
                    event.has_value("json.properties.ActionType")
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("driverload"))
                };
                if _cond {
                    event.append("event.category", json!("driver"))?;
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("devicefileevents"))
                        || event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("devicefilecertificateinfo"))
                };
                if _cond {
                    event.append("event.category", json!("file"))?;
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("deviceprocessevents"))
                        || event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("cloudprocessevents"))
                };
                if _cond {
                    event.append("event.category", json!("process"))?;
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents"))
                };
                if _cond {
                    event.append("event.category", json!("library"))?;
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("devicenetworkevents"))
                        || event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("devicenetworkinfo"))
                };
                if _cond {
                    event.append("event.category", json!("network"))?;
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("deviceregistryevents"))
                };
                if _cond {
                    event.append("event.category", json!("registry"))?;
                }
                let _cond =
                    {
                        (event.has_value("event.category")
                            && (event.get("event.category").is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("authentication"))
                                }
                                serde_json::Value::String(s) => s.contains("authentication"),
                                _ => false,
                            }) || event.get("event.category").is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("host"))
                                }
                                serde_json::Value::String(s) => s.contains("host"),
                                _ => false,
                            })))
                            || (event.has_value("json.properties.ActionType")
                                && (event
                                    .get_str("json.properties.ActionType")
                                    .is_some_and(|s| s.to_lowercase().contains("connectionfound"))
                                    || event.get_str("json.properties.ActionType").is_some_and(
                                        |s| s.to_lowercase().contains("networksignatureinspected"),
                                    )
                                    || event.get_str("json.properties.ActionType").is_some_and(
                                        |s| s.to_lowercase().contains("devicenetworkinfo"),
                                    )))
                    };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("file"))
                            }
                            serde_json::Value::String(s) => s.contains("file"),
                            _ => false,
                        })
                        && event.has_value("json.properties.ActionType")
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase() == "filedeleted")
                };
                if _cond {
                    event.append("event.type", json!("deletion"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("file"))
                            }
                            serde_json::Value::String(s) => s.contains("file"),
                            _ => false,
                        })
                        && event.has_value("json.properties.ActionType")
                        && (event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase() == "filemodified")
                            || event
                                .get_str("json.properties.ActionType")
                                .is_some_and(|s| s.to_lowercase() == "filerenamed"))
                };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("file"))
                            }
                            serde_json::Value::String(s) => s.contains("file"),
                            _ => false,
                        })
                        && event.has_value("json.properties.ActionType")
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase() == "filecreated")
                };
                if _cond {
                    event.append("event.type", json!("creation"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && !event.has_value("event.type")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("file"))
                            }
                            serde_json::Value::String(s) => s.contains("file"),
                            _ => false,
                        })
                };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                let _cond = {
                    event.has_value("json.properties.ActionType")
                        && (event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("connectionsuccess"))
                            || event
                                .get_str("json.properties.ActionType")
                                .is_some_and(|s| s.to_lowercase().contains("driverload")))
                };
                if _cond {
                    event.append("event.type", json!("start"))?;
                }
                let _cond = {
                    event.has_value("json.properties.ActionType")
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("connectionfailed"))
                };
                if _cond {
                    event.append("event.type", json!("denied"))?;
                }
                let _cond = {
                    event.has_value("json.properties.ActionType")
                        && (event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("connectionrequest"))
                            || event
                                .get_str("json.properties.ActionType")
                                .is_some_and(|s| {
                                    s.to_lowercase().contains("listeningconnectioncreated")
                                })
                            || event
                                .get_str("json.properties.ActionType")
                                .is_some_and(|s| s.to_lowercase().contains("processcreated"))
                            || event
                                .get_str("m365_defender.event.category")
                                .is_some_and(|s| {
                                    s.to_lowercase().contains("deviceimageloadevents")
                                }))
                };
                if _cond {
                    event.append("event.type", json!("start"))?;
                }
                let _cond = {
                    event.has_value("json.properties.ActionType")
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("inboundconnectionaccepted"))
                };
                if _cond {
                    event.append("event.type", json!("start"))?;
                }
                let _cond = {
                    event.has_value("json.properties.ActionType")
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("registrykeycreated"))
                };
                if _cond {
                    event.append("event.type", json!("creation"))?;
                }
                let _cond = {
                    event.has_value("json.properties.ActionType")
                        && (event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("registrykeydeleted"))
                            || event
                                .get_str("json.properties.ActionType")
                                .is_some_and(|s| s.to_lowercase().contains("registryvaluedeleted")))
                };
                if _cond {
                    event.append("event.type", json!("deletion"))?;
                }
                let _cond = {
                    event.has_value("json.properties.ActionType")
                        && (event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("registrykeyrenamed"))
                            || event
                                .get_str("json.properties.ActionType")
                                .is_some_and(|s| s.to_lowercase().contains("registryvalueset")))
                };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                let _cond = {
                    event.has_value("json.properties.ActionType")
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("imageloaded"))
                };
                if _cond {
                    event.set("json.properties.ActionType", json!("load"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.has_value("json.properties.ActionType")
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    event.append("event.type", json!("protocol"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        })
                        && !event.has_value("event.type")
                        && event.has_value("json.properties.ActionType")
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().starts_with("open"))
                };
                if _cond {
                    event.append("event.type", json!("access"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        })
                        && !event.has_value("event.type")
                        && event.has_value("json.properties.ActionType")
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().starts_with("write"))
                };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && !event.has_value("event.type")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                let _cond = {
                    event
                        .get("json.properties.AdditionalFields")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.AdditionalFields") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.AdditionalFields",
                            "json.properties.AdditionalFields",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_json_properties_AdditionalFields",
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
                if event.has_value("json.properties.AdditionalFields") {
                    event.rename(
                        "json.properties.AdditionalFields",
                        "m365_defender.event.additional_fields",
                    )?;
                }
                let _cond = {
                    event
                        .get("m365_defender.event.additional_fields")
                        .is_some_and(|v| v.is_object())
                };
                if _cond {
                    if event.has_value("m365_defender.event.additional_fields.direction") {
                        event.rename(
                            "m365_defender.event.additional_fields.direction",
                            "m365_defender.event.network_direction",
                        )?;
                    }
                }
                let _cond = {
                    event
                        .get("m365_defender.event.additional_fields")
                        .is_some_and(|v| v.is_object())
                        && event.has_value("json.properties.ActionType")
                        && event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    if event.has_value("m365_defender.event.additional_fields.qclass_name") {
                        event.rename(
                            "m365_defender.event.additional_fields.qclass_name",
                            "m365_defender.event.dns.qclass_name",
                        )?;
                    }
                }
                let _cond = {
                    event
                        .get("m365_defender.event.additional_fields")
                        .is_some_and(|v| v.is_object())
                        && event.has_value("json.properties.ActionType")
                        && event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    if event.has_value("m365_defender.event.additional_fields.query") {
                        event.rename(
                            "m365_defender.event.additional_fields.query",
                            "m365_defender.event.dns.query",
                        )?;
                    }
                }
                let _cond = {
                    event
                        .get("m365_defender.event.additional_fields")
                        .is_some_and(|v| v.is_object())
                        && event.has_value("json.properties.ActionType")
                        && event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    if event.has_value("m365_defender.event.additional_fields.qtype_name") {
                        event.rename(
                            "m365_defender.event.additional_fields.qtype_name",
                            "m365_defender.event.dns.qtype_name",
                        )?;
                    }
                }
                let _cond = {
                    event
                        .get("m365_defender.event.additional_fields")
                        .is_some_and(|v| v.is_object())
                        && event.has_value("json.properties.ActionType")
                        && event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    if event.has_value("m365_defender.event.additional_fields.rcode_name") {
                        event.rename(
                            "m365_defender.event.additional_fields.rcode_name",
                            "m365_defender.event.dns.rcode_name",
                        )?;
                    }
                }
                let _cond = {
                    event
                        .get("m365_defender.event.additional_fields")
                        .is_some_and(|v| v.is_object())
                        && event.has_value("m365_defender.event.additional_fields.answers")
                        && event.has_value("json.properties.ActionType")
                        && event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "m365_defender.event.additional_fields.answers",
                            "m365_defender.event.additional_fields.answers",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_m365_defender_event_additional_fields_answers",
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
                let _cond = {
                    event
                        .get("m365_defender.event.additional_fields")
                        .is_some_and(|v| v.is_object())
                        && event.has_value("json.properties.ActionType")
                        && event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    if event.has_value("m365_defender.event.additional_fields.answers") {
                        event.rename(
                            "m365_defender.event.additional_fields.answers",
                            "m365_defender.event.dns.answers",
                        )?;
                    }
                }
                let _cond = {
                    event
                        .get("m365_defender.event.additional_fields")
                        .is_some_and(|v| v.is_object())
                        && event.has_value("m365_defender.event.additional_fields.TTLs")
                        && event.has_value("json.properties.ActionType")
                        && event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "m365_defender.event.additional_fields.TTLs",
                            "m365_defender.event.additional_fields.TTLs",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_m365_defender_event_additional_fields_ttls",
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
                let _cond = {
                    event
                        .get("m365_defender.event.additional_fields")
                        .is_some_and(|v| v.is_object())
                        && event.has_value("json.properties.ActionType")
                        && event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    if event.has_value("m365_defender.event.additional_fields.TTLs") {
                        event.rename(
                            "m365_defender.event.additional_fields.TTLs",
                            "m365_defender.event.dns.ttls",
                        )?;
                    }
                }
                let _cond = {
                    event
                        .get("m365_defender.event.additional_fields")
                        .is_some_and(|v| v.is_object())
                        && event.has_value("json.properties.ActionType")
                        && event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event
                            .get_str("json.properties.ActionType")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    // Painless script
                    // Source: def af = ctx.m365_defender.event.additional_fields;\nList ecs_flags = [\"AA\", \"TC\", \"RD\", \"RA\", \"AD\", \"CD\", \"DO\"];\nList flags = [];\nif (af instanceof Map) {\n    for (def flag: ecs_flags) {\n        if (af[flag] != null && af[flag] == \"true\") {\n            flags.add(flag);\n        }\n    }\n}\nif (!ctx.m365_defender.event.containsKey('dns')) {\n    ctx.m365_defender.event.dns = new HashMap();\n}\nctx.m365_defender.event.dns.header_flags = flags;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def af = ctx.m365_defender.event.additional_fields;\nList ecs_flags = [\"AA\", \"TC\", \"RD\", \"RA\", \"AD\", \"CD\", \"DO\"];\nList flags = [];\nif (af instanceof Map) {\n    for (def flag: ecs_flags) {\n        if (af[flag] != null && af[flag] == \"true\") {\n            flags.add(flag);\n        }\n    }\n}\nif (!ctx.m365_defender.event.containsKey('dns')) {\n    ctx.m365_defender.event.dns = new HashMap();\n}\nctx.m365_defender.event.dns.header_flags = flags;\n"#
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("json.properties.CrlDistributionPointUrls")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.CrlDistributionPointUrls") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.CrlDistributionPointUrls",
                            "json.properties.CrlDistributionPointUrls",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_json_properties_CrlDistributionPointUrls",
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
                let _cond = {
                    event
                        .get("json.properties.LoggedOnUsers")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.LoggedOnUsers") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.LoggedOnUsers",
                            "json.properties.LoggedOnUsers",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_json_properties_LoggedOnUsers",
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
                let _cond = {
                    event
                        .get("json.properties.ConnectedNetworks")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.ConnectedNetworks") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.ConnectedNetworks",
                            "json.properties.ConnectedNetworks",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_json_properties_ConnectedNetworks",
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
                let _cond = {
                    event
                        .get("json.properties.DefaultGateways")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.DefaultGateways") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.DefaultGateways",
                            "json.properties.DefaultGateways",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_json_properties_DefaultGateways",
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
                let _cond = {
                    event
                        .get("json.properties.DnsAddresses")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.DnsAddresses") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.DnsAddresses",
                            "json.properties.DnsAddresses",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_json_properties_DnsAddresses",
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
                let _cond = {
                    event
                        .get("json.properties.IPAddresses")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.IPAddresses") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.IPAddresses",
                            "json.properties.IPAddresses",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_json_properties_IPAddresses",
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
                let _cond = {
                    event
                        .get("json.properties.CertificateExpirationTime")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.CertificateExpirationTime") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.properties.CertificateExpirationTime")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set(
                                    "m365_defender.event.certificate.expiration_time",
                                    parsed,
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.properties.CertificateExpirationTime".into(),
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
                            "date_json_properties_CertificateExpirationTime",
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
                let _cond = {
                    event
                        .get("json.properties.InitiatingProcessCreationTime")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.InitiatingProcessCreationTime")
                            != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.properties.InitiatingProcessCreationTime")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set(
                                    "m365_defender.event.initiating_process.creation_time",
                                    parsed,
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.properties.InitiatingProcessCreationTime"
                                            .into(),
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
                            "date_json_properties_InitiatingProcessCreationTime",
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
                let _cond = {
                    event
                        .get("json.properties.InitiatingProcessParentCreationTime")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.InitiatingProcessParentCreationTime")
                            != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event
                            .get_as_string("json.properties.InitiatingProcessParentCreationTime")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set(
                                    "m365_defender.event.initiating_process.parent_creation_time",
                                    parsed,
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.properties.InitiatingProcessParentCreationTime"
                                            .into(),
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
                            "date_json_properties_InitiatingProcessParentCreationTime",
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
                let _cond = {
                    event
                        .get("json.properties.ProcessCreationTime")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.ProcessCreationTime") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.properties.ProcessCreationTime")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event
                                    .set("m365_defender.event.process.creation_time", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.properties.ProcessCreationTime".into(),
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
                            "date_json_properties_ProcessCreationTime",
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
                let _cond = {
                    event
                        .get("json.properties.CertificateCountersignatureTime")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.CertificateCountersignatureTime")
                            != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.properties.CertificateCountersignatureTime")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set(
                                    "m365_defender.event.certificate.countersignature_time",
                                    parsed,
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.properties.CertificateCountersignatureTime"
                                            .into(),
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
                            "date_json_properties_CertificateCountersignatureTime",
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
                let _cond = {
                    event
                        .get("json.properties.CertificateCreationTime")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.CertificateCreationTime") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.properties.CertificateCreationTime")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event
                                    .set("m365_defender.event.certificate.creation_time", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.properties.CertificateCreationTime".into(),
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
                            "date_json_properties_CertificateCreationTime",
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
                let _cond =
                    { event.get_str("json.properties.InitiatingProcessFileSize") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.InitiatingProcessFileSize") {
                            if let Some(val) =
                                event.get("json.properties.InitiatingProcessFileSize")
                            {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.InitiatingProcessFileSize".into(),
                                        message,
                                    }
                                })?;
                                event.set(
                                    "m365_defender.event.initiating_process.file_size",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_json_properties_InitiatingProcessFileSize",
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
                let _cond =
                    { event.get_str("json.properties.InitiatingProcessLogonId") != Some("") };
                if _cond {
                    if event.has_value("json.properties.InitiatingProcessLogonId") {
                        if let Some(val) = event.get("json.properties.InitiatingProcessLogonId") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.InitiatingProcessLogonId".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "m365_defender.event.initiating_process.logon_id",
                                converted,
                            )?;
                        }
                    }
                }
                let _cond = { event.get_str("json.properties.LogonId") != Some("") };
                if _cond {
                    if event.has_value("json.properties.LogonId") {
                        if let Some(val) = event.get("json.properties.LogonId") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.LogonId".into(),
                                    message,
                                }
                            })?;
                            event.set("m365_defender.event.logon.id", converted)?;
                        }
                    }
                }
                let _cond = { event.get_str("json.properties.ProcessId") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.ProcessId") {
                            if let Some(val) = event.get("json.properties.ProcessId") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.ProcessId".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.process.id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_json_properties_ProcessId",
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
                let _cond = { event.get_str("json.properties.ReportId") != Some("") };
                if _cond {
                    if event.has_value("json.properties.ReportId") {
                        if let Some(val) = event.get("json.properties.ReportId") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.ReportId".into(),
                                    message,
                                }
                            })?;
                            event.set("m365_defender.event.report_id", converted)?;
                        }
                    }
                }
                let _cond = { event.get_str("json.properties.IPv4Dhcp") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.IPv4Dhcp") {
                            if let Some(val) = event.get("json.properties.IPv4Dhcp") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.IPv4Dhcp".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.ipv4_dhcp", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_json_properties_IPv4Dhcp",
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
                let _cond = { event.get_str("json.properties.IPv6Dhcp") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.IPv6Dhcp") {
                            if let Some(val) = event.get("json.properties.IPv6Dhcp") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.IPv6Dhcp".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.ipv6_dhcp", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_json_properties_IPv6Dhcp",
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
                // Painless script
                // Source: def isTruthy(def val) {\n   if (val == null) {\n     // Fast return if field is absent.\n     return null;\n   }\n   if (val instanceof Boolean) {\n     return val;\n   }\n   if (val instanceof Integer) {\n     if (val == 1) {\n       return true;\n     }\n     if (val == 0) {\n       return false;\n     }\n     return null;\n   }\n   if (val instanceof String) {\n     if (val == \"1\" || val == \"true\") {\n       return true;\n     }\n     if (val == \"0\" || val == \"false\") {\n       return false;\n     }\n     return null;\n   }\n   return null;\n }\n ctx.m365_defender.event.is_root_signer_microsoft = isTruthy(ctx.json?.properties?.IsRootSignerMicrosoft);\n ctx.m365_defender.event.is_signed = isTruthy(ctx.json?.properties?.IsSigned);\n ctx.m365_defender.event.is_trusted = isTruthy(ctx.json?.properties?.IsTrusted);\n ctx.m365_defender.event.is_azure_info_protection_applied = isTruthy(ctx.json?.properties?.IsAzureInfoProtectionApplied);\n ctx.m365_defender.event.is_azure_ad_joined = isTruthy(ctx.json?.properties?.IsAzureADJoined);\n ctx.m365_defender.event.is_local_admin = isTruthy(ctx.json?.properties?.IsLocalAdmin);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def isTruthy(def val) {\n   if (val == null) {\n     // Fast return if field is absent.\n     return null;\n   }\n   if (val instanceof Boolean) {\n     return val;\n   }\n   if (val instanceof Integer) {\n     if (val == 1) {\n       return true;\n     }\n     if (val == 0) {\n       return false;\n     }\n     return null;\n   }\n   if (val instanceof String) {\n     if (val == \"1\" || val == \"true\") {\n       return true;\n     }\n     if (val == \"0\" || val == \"false\") {\n       return false;\n     }\n     return null;\n   }\n   return null;\n }\n ctx.m365_defender.event.is_root_signer_microsoft = isTruthy(ctx.json?.properties?.IsRootSignerMicrosoft);\n ctx.m365_defender.event.is_signed = isTruthy(ctx.json?.properties?.IsSigned);\n ctx.m365_defender.event.is_trusted = isTruthy(ctx.json?.properties?.IsTrusted);\n ctx.m365_defender.event.is_azure_info_protection_applied = isTruthy(ctx.json?.properties?.IsAzureInfoProtectionApplied);\n ctx.m365_defender.event.is_azure_ad_joined = isTruthy(ctx.json?.properties?.IsAzureADJoined);\n ctx.m365_defender.event.is_local_admin = isTruthy(ctx.json?.properties?.IsLocalAdmin);\n"#
                    ),
                )?;
                if event.has_value("json.properties.FolderPath") {
                    event.rename(
                        "json.properties.FolderPath",
                        "m365_defender.event.folder_path",
                    )?;
                }
                if event.has_value("json.properties.MD5") {
                    event.rename("json.properties.MD5", "m365_defender.event.md5")?;
                }
                if event.has_value("json.properties.SHA1") {
                    event.rename("json.properties.SHA1", "m365_defender.event.sha1")?;
                }
                if event.has_value("json.properties.SHA256") {
                    event.rename("json.properties.SHA256", "m365_defender.event.sha256")?;
                }
                if event.has_value("json.properties.FileName") {
                    event.rename("json.properties.FileName", "m365_defender.event.file.name")?;
                }
                if event.has_value("json.properties.FileSize") {
                    event.rename("json.properties.FileSize", "m365_defender.event.file.size")?;
                }
                if event.has_value("json.properties.DeviceName") {
                    event.rename(
                        "json.properties.DeviceName",
                        "m365_defender.event.device.name",
                    )?;
                }
                if event.has_value("json.properties.DeviceId") {
                    event.rename("json.properties.DeviceId", "m365_defender.event.device.id")?;
                }
                if event.has_value("json.properties.InitiatingProcessCommandLine") {
                    event.rename(
                        "json.properties.InitiatingProcessCommandLine",
                        "m365_defender.event.initiating_process.command_line",
                    )?;
                }
                if event.has_value("json.properties.AzureResourceId") {
                    event.rename(
                        "json.properties.AzureResourceId",
                        "m365_defender.event.azure_resource_id",
                    )?;
                }
                if event.has_value("json.properties.AwsResourceName") {
                    event.rename(
                        "json.properties.AwsResourceName",
                        "m365_defender.event.aws_resource_name",
                    )?;
                }
                if event.has_value("json.properties.GcpFullResourceName") {
                    event.rename(
                        "json.properties.GcpFullResourceName",
                        "m365_defender.event.gcp_full_resource_name",
                    )?;
                }
                if event.has_value("json.properties.ContainerImageName") {
                    event.rename(
                        "json.properties.ContainerImageName",
                        "m365_defender.event.container_image_name",
                    )?;
                }
                if event.has_value("json.properties.KubernetesNamespace") {
                    event.rename(
                        "json.properties.KubernetesNamespace",
                        "m365_defender.event.kubernetes_namespace",
                    )?;
                }
                if event.has_value("json.properties.KubernetesPodName") {
                    event.rename(
                        "json.properties.KubernetesPodName",
                        "m365_defender.event.kubernetes_pod_name",
                    )?;
                }
                if event.has_value("json.properties.KubernetesResource") {
                    event.rename(
                        "json.properties.KubernetesResource",
                        "m365_defender.event.kubernetes_resource",
                    )?;
                }
                if event.has_value("json.properties.ContainerName") {
                    event.rename(
                        "json.properties.ContainerName",
                        "m365_defender.event.container_name",
                    )?;
                }
                if event.has_value("json.properties.ContainerId") {
                    event.rename(
                        "json.properties.ContainerId",
                        "m365_defender.event.container_id",
                    )?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.container_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("container.name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.container_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("container.id", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.container_image_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("container.image.name", v)?;
                }
                if event.has_value("json.properties.ProcessName") {
                    event.rename(
                        "json.properties.ProcessName",
                        "m365_defender.event.process_name",
                    )?;
                }
                if event.has_value("json.properties.ParentProcessName") {
                    event.rename(
                        "json.properties.ParentProcessName",
                        "m365_defender.event.parent_process_name",
                    )?;
                }
                if event.has_value("json.properties.ParentProcessId") {
                    event.rename(
                        "json.properties.ParentProcessId",
                        "m365_defender.event.parent_process_id",
                    )?;
                }
                if event.has_value("json.properties.ProcessCurrentWorkingDirectory") {
                    event.rename(
                        "json.properties.ProcessCurrentWorkingDirectory",
                        "m365_defender.event.process_current_working_directory",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessMD5") {
                    event.rename(
                        "json.properties.InitiatingProcessMD5",
                        "m365_defender.event.initiating_process.md5",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessSHA1") {
                    event.rename(
                        "json.properties.InitiatingProcessSHA1",
                        "m365_defender.event.initiating_process.sha1",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessSHA256") {
                    event.rename(
                        "json.properties.InitiatingProcessSHA256",
                        "m365_defender.event.initiating_process.sha256",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessParentId") {
                    event.rename(
                        "json.properties.InitiatingProcessParentId",
                        "m365_defender.event.initiating_process.parent_id",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessId") {
                    event.rename(
                        "json.properties.InitiatingProcessId",
                        "m365_defender.event.initiating_process.id",
                    )?;
                }
                if event.has_value("json.properties.RegistryKey") {
                    event.rename(
                        "json.properties.RegistryKey",
                        "m365_defender.event.registry.key",
                    )?;
                }
                if event.has_value("json.properties.RegistryValueName") {
                    event.rename(
                        "json.properties.RegistryValueName",
                        "m365_defender.event.registry.value_name",
                    )?;
                }
                if event.has_value("json.properties.CertificateSerialNumber") {
                    event.rename(
                        "json.properties.CertificateSerialNumber",
                        "m365_defender.event.certificate.serial_number",
                    )?;
                }
                if event.has_value("json.properties.AccountName") {
                    event.rename(
                        "json.properties.AccountName",
                        "m365_defender.event.account.name",
                    )?;
                }
                if event.has_value("json.properties.RequestProtocol") {
                    event.rename(
                        "json.properties.RequestProtocol",
                        "m365_defender.event.request.protocol",
                    )?;
                }
                if event.has_value("json.properties.ActionType") {
                    event.rename(
                        "json.properties.ActionType",
                        "m365_defender.event.action.type",
                    )?;
                }
                if event.has_value("json.properties.RequestAccountDomain") {
                    event.rename(
                        "json.properties.RequestAccountDomain",
                        "m365_defender.event.request.account_domain",
                    )?;
                }
                if event.has_value("json.properties.RequestAccountName") {
                    event.rename(
                        "json.properties.RequestAccountName",
                        "m365_defender.event.request.account_name",
                    )?;
                }
                if event.has_value("json.properties.OSArchitecture") {
                    event.rename(
                        "json.properties.OSArchitecture",
                        "m365_defender.event.os.architecture",
                    )?;
                }
                if event.has_value("json.properties.OSPlatform") {
                    event.rename(
                        "json.properties.OSPlatform",
                        "m365_defender.event.os.platform",
                    )?;
                }
                if event.has_value("json.properties.OSDistribution") {
                    event.rename(
                        "json.properties.OSDistribution",
                        "m365_defender.event.os.distribution",
                    )?;
                }
                if event.has_value("json.properties.OSVersion") {
                    event.rename(
                        "json.properties.OSVersion",
                        "m365_defender.event.os.version",
                    )?;
                }
                if event.has_value("json.properties.DeviceType") {
                    event.rename(
                        "json.properties.DeviceType",
                        "m365_defender.event.device.type",
                    )?;
                }
                if event.has_value("json.properties.AccountDomain") {
                    event.rename(
                        "json.properties.AccountDomain",
                        "m365_defender.event.account.domain",
                    )?;
                }
                if event.has_value("json.properties.ClientVersion") {
                    event.rename(
                        "json.properties.ClientVersion",
                        "m365_defender.event.client_version",
                    )?;
                }
                if event.has_value("json.properties.DeviceCategory") {
                    event.rename(
                        "json.properties.DeviceCategory",
                        "m365_defender.event.device.category",
                    )?;
                }
                if event.has_value("json.properties.MacAddress") {
                    event.rename(
                        "json.properties.MacAddress",
                        "m365_defender.event.mac_address",
                    )?;
                }
                if event.has_value("json.properties.AccountSid") {
                    event.rename(
                        "json.properties.AccountSid",
                        "m365_defender.event.account.sid",
                    )?;
                }
                if event.has_value("json.properties.RequestAccountSid") {
                    event.rename(
                        "json.properties.RequestAccountSid",
                        "m365_defender.event.request.account_sid",
                    )?;
                }
                if event.has_value("json.properties.AppGuardContainerId") {
                    event.rename(
                        "json.properties.AppGuardContainerId",
                        "m365_defender.event.app_guard_container_id",
                    )?;
                }
                if event.has_value("json.properties.FileOriginUrl") {
                    event.rename(
                        "json.properties.FileOriginUrl",
                        "m365_defender.event.file.origin_url",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessAccountDomain") {
                    event.rename(
                        "json.properties.InitiatingProcessAccountDomain",
                        "m365_defender.event.initiating_process.account_domain",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessAccountName") {
                    event.rename(
                        "json.properties.InitiatingProcessAccountName",
                        "m365_defender.event.initiating_process.account_name",
                    )?;
                }
                if event.has_value("json.properties.AccountObjectId") {
                    event.rename(
                        "json.properties.AccountObjectId",
                        "m365_defender.event.account.object_id",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessAccountObjectId") {
                    event.rename(
                        "json.properties.InitiatingProcessAccountObjectId",
                        "m365_defender.event.initiating_process.account_object_id",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessAccountSid") {
                    event.rename(
                        "json.properties.InitiatingProcessAccountSid",
                        "m365_defender.event.initiating_process.account_sid",
                    )?;
                }
                if event.has_value("json.properties.AccountUpn") {
                    event.rename(
                        "json.properties.AccountUpn",
                        "m365_defender.event.account.upn",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessAccountUpn") {
                    event.rename(
                        "json.properties.InitiatingProcessAccountUpn",
                        "m365_defender.event.initiating_process.account_upn",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessFileName") {
                    event.rename(
                        "json.properties.InitiatingProcessFileName",
                        "m365_defender.event.initiating_process.file_name",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessFolderPath") {
                    event.rename(
                        "json.properties.InitiatingProcessFolderPath",
                        "m365_defender.event.initiating_process.folder_path",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessParentFileName") {
                    event.rename(
                        "json.properties.InitiatingProcessParentFileName",
                        "m365_defender.event.initiating_process.parent_file_name",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessVersionInfoCompanyName") {
                    event.rename(
                        "json.properties.InitiatingProcessVersionInfoCompanyName",
                        "m365_defender.event.initiating_process.version_info_company_name",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessVersionInfoFileDescription") {
                    event.rename(
                        "json.properties.InitiatingProcessVersionInfoFileDescription",
                        "m365_defender.event.initiating_process.version_info_file_description",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessVersionInfoInternalFileName") {
                    event.rename(
                        "json.properties.InitiatingProcessVersionInfoInternalFileName",
                        "m365_defender.event.initiating_process.version_info_internal_file_name",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessVersionInfoOriginalFileName") {
                    event.rename(
                        "json.properties.InitiatingProcessVersionInfoOriginalFileName",
                        "m365_defender.event.initiating_process.version_info_original_file_name",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessVersionInfoProductName") {
                    event.rename(
                        "json.properties.InitiatingProcessVersionInfoProductName",
                        "m365_defender.event.initiating_process.version_info_product_name",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessVersionInfoProductVersion") {
                    event.rename(
                        "json.properties.InitiatingProcessVersionInfoProductVersion",
                        "m365_defender.event.initiating_process.version_info_product_version",
                    )?;
                }
                if event.has_value("json.properties.ProcessCommandLine") {
                    event.rename(
                        "json.properties.ProcessCommandLine",
                        "m365_defender.event.process.command_line",
                    )?;
                }
                if event.has_value("json.properties.ProcessTokenElevation") {
                    event.rename(
                        "json.properties.ProcessTokenElevation",
                        "m365_defender.event.process.token_elevation",
                    )?;
                }
                if event.has_value("json.properties.ProcessVersionInfoCompanyName") {
                    event.rename(
                        "json.properties.ProcessVersionInfoCompanyName",
                        "m365_defender.event.process.version_info_company_name",
                    )?;
                }
                if event.has_value("json.properties.ProcessVersionInfoFileDescription") {
                    event.rename(
                        "json.properties.ProcessVersionInfoFileDescription",
                        "m365_defender.event.process.version_info_file_description",
                    )?;
                }
                if event.has_value("json.properties.ProcessVersionInfoInternalFileName") {
                    event.rename(
                        "json.properties.ProcessVersionInfoInternalFileName",
                        "m365_defender.event.process.version_info_internal_file_name",
                    )?;
                }
                if event.has_value("json.properties.ProcessVersionInfoOriginalFileName") {
                    event.rename(
                        "json.properties.ProcessVersionInfoOriginalFileName",
                        "m365_defender.event.process.version_info_original_file_name",
                    )?;
                }
                if event.has_value("json.properties.ProcessVersionInfoProductName") {
                    event.rename(
                        "json.properties.ProcessVersionInfoProductName",
                        "m365_defender.event.process.version_info_product_name",
                    )?;
                }
                if event.has_value("json.properties.ProcessVersionInfoProductVersion") {
                    event.rename(
                        "json.properties.ProcessVersionInfoProductVersion",
                        "m365_defender.event.process.version_info_product_version",
                    )?;
                }
                if event.has_value("json.properties.RegistryValueData") {
                    event.rename(
                        "json.properties.RegistryValueData",
                        "m365_defender.event.registry.value_data",
                    )?;
                }
                if event.has_value("json.properties.RemoteDeviceName") {
                    event.rename(
                        "json.properties.RemoteDeviceName",
                        "m365_defender.event.remote.device_name",
                    )?;
                }
                if event.has_value("json.properties.RemoteUrl") {
                    event.rename(
                        "json.properties.RemoteUrl",
                        "m365_defender.event.remote.url",
                    )?;
                }
                if event.has_value("json.properties.CrlDistributionPointUrls") {
                    event.rename(
                        "json.properties.CrlDistributionPointUrls",
                        "m365_defender.event.crl_distribution_point_urls",
                    )?;
                }
                if event.has_value("json.properties.Issuer") {
                    event.rename("json.properties.Issuer", "m365_defender.event.issuer")?;
                }
                if event.has_value("json.properties.IssuerHash") {
                    event.rename(
                        "json.properties.IssuerHash",
                        "m365_defender.event.issuer_hash",
                    )?;
                }
                if event.has_value("json.properties.SignatureType") {
                    event.rename(
                        "json.properties.SignatureType",
                        "m365_defender.event.signature_type",
                    )?;
                }
                if event.has_value("json.properties.Signer") {
                    event.rename("json.properties.Signer", "m365_defender.event.signer")?;
                }
                if event.has_value("json.properties.SignerHash") {
                    event.rename(
                        "json.properties.SignerHash",
                        "m365_defender.event.signer_hash",
                    )?;
                }
                if event.has_value("json.properties.FileOriginReferrerUrl") {
                    event.rename(
                        "json.properties.FileOriginReferrerUrl",
                        "m365_defender.event.file.origin_referrer_url",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessIntegrityLevel") {
                    event.rename(
                        "json.properties.InitiatingProcessIntegrityLevel",
                        "m365_defender.event.initiating_process.integrity_level",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessTokenElevation") {
                    event.rename(
                        "json.properties.InitiatingProcessTokenElevation",
                        "m365_defender.event.initiating_process.token_elevation",
                    )?;
                }
                if event.has_value("json.properties.PreviousFileName") {
                    event.rename(
                        "json.properties.PreviousFileName",
                        "m365_defender.event.previous.file_name",
                    )?;
                }
                if event.has_value("json.properties.PreviousFolderPath") {
                    event.rename(
                        "json.properties.PreviousFolderPath",
                        "m365_defender.event.previous.folder_path",
                    )?;
                }
                if event.has_value("json.properties.SensitivityLabel") {
                    event.rename(
                        "json.properties.SensitivityLabel",
                        "m365_defender.event.sensitivity.label",
                    )?;
                }
                if event.has_value("json.properties.SensitivitySubLabel") {
                    event.rename(
                        "json.properties.SensitivitySubLabel",
                        "m365_defender.event.sensitivity.sub_label",
                    )?;
                }
                if event.has_value("json.properties.ShareName") {
                    event.rename(
                        "json.properties.ShareName",
                        "m365_defender.event.share_name",
                    )?;
                }
                if event.has_value("json.properties.FailureReason") {
                    event.rename(
                        "json.properties.FailureReason",
                        "m365_defender.event.failure_reason",
                    )?;
                }
                if event.has_value("json.properties.AadDeviceId") {
                    event.rename(
                        "json.properties.AadDeviceId",
                        "m365_defender.event.aad_device_id",
                    )?;
                }
                if event.has_value("json.properties.DeviceSubType") {
                    event.rename(
                        "json.properties.DeviceSubType",
                        "m365_defender.event.device.sub_type",
                    )?;
                }
                if event.has_value("json.properties.JoinType") {
                    event.rename("json.properties.JoinType", "m365_defender.event.join_type")?;
                }
                if event.has_value("json.properties.MachineGroup") {
                    event.rename(
                        "json.properties.MachineGroup",
                        "m365_defender.event.machine_group",
                    )?;
                }
                if event.has_value("json.properties.MergedDeviceIds") {
                    event.rename(
                        "json.properties.MergedDeviceIds",
                        "m365_defender.event.merged_device_ids",
                    )?;
                }
                if event.has_value("json.properties.MergedToDeviceId") {
                    event.rename(
                        "json.properties.MergedToDeviceId",
                        "m365_defender.event.merged_to_device_id",
                    )?;
                }
                if event.has_value("json.properties.SensorHealthState") {
                    event.rename(
                        "json.properties.SensorHealthState",
                        "m365_defender.event.sensor_health_state",
                    )?;
                }
                if event.has_value("json.properties.IsExcluded") {
                    event.rename(
                        "json.properties.IsExcluded",
                        "m365_defender.event.is_excluded",
                    )?;
                }
                if event.has_value("json.properties.ExclusionReason") {
                    event.rename(
                        "json.properties.ExclusionReason",
                        "m365_defender.event.exclusion_reason",
                    )?;
                }
                if event.has_value("json.properties.AssetValue") {
                    event.rename(
                        "json.properties.AssetValue",
                        "m365_defender.event.asset_value",
                    )?;
                }
                if event.has_value("json.properties.ExposureLevel") {
                    event.rename(
                        "json.properties.ExposureLevel",
                        "m365_defender.event.exposure_level",
                    )?;
                }
                if event.has_value("json.properties.IsInternetFacing") {
                    event.rename(
                        "json.properties.IsInternetFacing",
                        "m365_defender.event.is_internet_facing",
                    )?;
                }
                if event.has_value("json.properties.DeviceManualTags") {
                    event.rename(
                        "json.properties.DeviceManualTags",
                        "m365_defender.event.device_manual_tags",
                    )?;
                }
                if event.has_value("json.properties.DeviceDynamicTags") {
                    event.rename(
                        "json.properties.DeviceDynamicTags",
                        "m365_defender.event.device_dynamic_tags",
                    )?;
                }
                if event.has_value("json.properties.Model") {
                    event.rename("json.properties.Model", "m365_defender.event.model")?;
                }
                if event.has_value("json.properties.OnboardingStatus") {
                    event.rename(
                        "json.properties.OnboardingStatus",
                        "m365_defender.event.onboarding_status",
                    )?;
                }
                if event.has_value("json.properties.OSBuild") {
                    event.rename("json.properties.OSBuild", "m365_defender.event.os.build")?;
                }
                if event.has_value("json.properties.OSVersionInfo") {
                    event.rename(
                        "json.properties.OSVersionInfo",
                        "m365_defender.event.os.version_info",
                    )?;
                }
                if event.has_value("json.properties.RegistryDeviceTag") {
                    event.rename(
                        "json.properties.RegistryDeviceTag",
                        "m365_defender.event.registry.device_tag",
                    )?;
                }
                if event.has_value("json.properties.Vendor") {
                    event.rename("json.properties.Vendor", "m365_defender.event.vendor")?;
                }
                if event.has_value("json.properties.LogonType") {
                    event.rename(
                        "json.properties.LogonType",
                        "m365_defender.event.logon.type",
                    )?;
                }
                if event.has_value("json.properties.Protocol") {
                    event.rename("json.properties.Protocol", "m365_defender.event.protocol")?;
                }
                if event.has_value("json.properties.RemoteIPType") {
                    event.rename(
                        "json.properties.RemoteIPType",
                        "m365_defender.event.remote.ip_type",
                    )?;
                }
                if event.has_value("json.properties.RegistryValueType") {
                    event.rename(
                        "json.properties.RegistryValueType",
                        "m365_defender.event.registry.value_type",
                    )?;
                }
                if event.has_value("json.properties.LocalIPType") {
                    event.rename(
                        "json.properties.LocalIPType",
                        "m365_defender.event.local.ip_type",
                    )?;
                }
                if event.has_value("json.properties.ConnectedNetworks") {
                    event.rename(
                        "json.properties.ConnectedNetworks",
                        "m365_defender.event.connected_networks",
                    )?;
                }
                if event.has_value("json.properties.DefaultGateways") {
                    event.rename(
                        "json.properties.DefaultGateways",
                        "m365_defender.event.default_gateways",
                    )?;
                }
                if event.has_value("json.properties.DnsAddresses") {
                    event.rename(
                        "json.properties.DnsAddresses",
                        "m365_defender.event.dns_addresses",
                    )?;
                }
                if event.has_value("json.properties.IPAddresses") {
                    event.rename(
                        "json.properties.IPAddresses",
                        "m365_defender.event.ip_addresses",
                    )?;
                }
                if event.has_value("json.properties.NetworkAdapterStatus") {
                    event.rename(
                        "json.properties.NetworkAdapterStatus",
                        "m365_defender.event.network.adapter_status",
                    )?;
                }
                if event.has_value("json.properties.NetworkAdapterType") {
                    event.rename(
                        "json.properties.NetworkAdapterType",
                        "m365_defender.event.network.adapter_type",
                    )?;
                }
                if event.has_value("json.properties.NetworkAdapterVendor") {
                    event.rename(
                        "json.properties.NetworkAdapterVendor",
                        "m365_defender.event.network.adapter_vendor",
                    )?;
                }
                if event.has_value("json.properties.TunnelType") {
                    event.rename(
                        "json.properties.TunnelType",
                        "m365_defender.event.tunnel_type",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessSignatureStatus") {
                    event.rename(
                        "json.properties.InitiatingProcessSignatureStatus",
                        "m365_defender.event.initiating_process.signature_status",
                    )?;
                }
                if event.has_value("json.properties.InitiatingProcessSignerType") {
                    event.rename(
                        "json.properties.InitiatingProcessSignerType",
                        "m365_defender.event.initiating_process.signer_type",
                    )?;
                }
                if event.has_value("json.properties.ProcessIntegrityLevel") {
                    event.rename(
                        "json.properties.ProcessIntegrityLevel",
                        "m365_defender.event.process.integrity_level",
                    )?;
                }
                if event.has_value("json.properties.PreviousRegistryKey") {
                    event.rename(
                        "json.properties.PreviousRegistryKey",
                        "m365_defender.event.previous.registry_key",
                    )?;
                }
                if event.has_value("json.properties.PreviousRegistryValueData") {
                    event.rename(
                        "json.properties.PreviousRegistryValueData",
                        "m365_defender.event.previous.registry_value_data",
                    )?;
                }
                if event.has_value("json.properties.PreviousRegistryValueName") {
                    event.rename(
                        "json.properties.PreviousRegistryValueName",
                        "m365_defender.event.previous.registry_value_name",
                    )?;
                }
                let _cond = { event.has_value("json.properties.FileOriginIP") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(val) = event.get("json.properties.FileOriginIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.FileOriginIP".into(),
                                    message,
                                }
                            })?;
                            event.set("json.properties.FileOriginIP", converted)?;
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_file_origin_ip")?;
                        if event.remove("json.properties.FileOriginIP").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "json.properties.FileOriginIP".into(),
                            });
                        }
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.properties.FileOriginIP") {
                    event.rename(
                        "json.properties.FileOriginIP",
                        "m365_defender.event.file.origin_ip",
                    )?;
                }
                let _cond = { event.has_value("json.properties.RemoteIP") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(val) = event.get("json.properties.RemoteIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.RemoteIP".into(),
                                    message,
                                }
                            })?;
                            event.set("json.properties.RemoteIP", converted)?;
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_remote_ip")?;
                        if event.remove("json.properties.RemoteIP").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "json.properties.RemoteIP".into(),
                            });
                        }
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.properties.RemoteIP") {
                    event.rename("json.properties.RemoteIP", "m365_defender.event.remote.ip")?;
                }
                let _cond = { event.has_value("json.properties.LocalIP") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(val) = event.get("json.properties.LocalIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.LocalIP".into(),
                                    message,
                                }
                            })?;
                            event.set("json.properties.LocalIP", converted)?;
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_local_ip")?;
                        if event.remove("json.properties.LocalIP").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "json.properties.LocalIP".into(),
                            });
                        }
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.properties.LocalIP") {
                    event.rename("json.properties.LocalIP", "m365_defender.event.local.ip")?;
                }
                let _cond = { event.has_value("json.properties.RequestSourceIP") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(val) = event.get("json.properties.RequestSourceIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.RequestSourceIP".into(),
                                    message,
                                }
                            })?;
                            event.set("json.properties.RequestSourceIP", converted)?;
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_request_source_ip",
                        )?;
                        if event.remove("json.properties.RequestSourceIP").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "json.properties.RequestSourceIP".into(),
                            });
                        }
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.properties.RequestSourceIP") {
                    event.rename(
                        "json.properties.RequestSourceIP",
                        "m365_defender.event.request.source_ip",
                    )?;
                }
                if event.has_value("json.properties.RequestSourcePort") {
                    event.rename(
                        "json.properties.RequestSourcePort",
                        "m365_defender.event.request.source_port",
                    )?;
                }
                if event.has_value("json.properties.RemotePort") {
                    event.rename(
                        "json.properties.RemotePort",
                        "m365_defender.event.remote.port",
                    )?;
                }
                if event.has_value("json.properties.LocalPort") {
                    event.rename(
                        "json.properties.LocalPort",
                        "m365_defender.event.local.port",
                    )?;
                }
                let _cond = { event.has_value("json.properties.PublicIP") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(val) = event.get("json.properties.PublicIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.PublicIP".into(),
                                    message,
                                }
                            })?;
                            event.set("json.properties.PublicIP", converted)?;
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_public_ip")?;
                        if event.remove("json.properties.PublicIP").is_none() {
                            return Err(TransformError::FieldNotFound {
                                path: "json.properties.PublicIP".into(),
                            });
                        }
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("json.properties.PublicIP") {
                    event.rename(
                        "json.properties.PublicIP",
                        "m365_defender.event.public_ip.value",
                    )?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }))
                        && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents")))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.folder_path")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.path", v)?;
                    }
                }
                let _cond = {
                    event.has_value("file.path")
                        && event
                            .get_as_string("file.path")
                            .is_some_and(|s| s.len() > 1)
                };
                if _cond {
                    // Painless script
                    // Source: String path = ctx.file.path;\nString sep = \"/\";\nString windows_sep = \"\\\\\";\ndef idx = -1;\nif (path.contains(windows_sep)) {\n    idx = path.lastIndexOf(windows_sep);\n}\nelse {\n    idx = path.lastIndexOf(sep);\n} \nif (idx > -1) {\n    if (ctx.file.name == null) {\n        ctx.file.name = path.substring(idx+1);\n    }\n    ctx.file.directory = path.substring(0, idx);\n    def extIdx = path.lastIndexOf(\".\");\n    if (extIdx > -1 && ctx.file.extension == null) {\n        ctx.file.extension = path.substring(extIdx+1);\n    }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"String path = ctx.file.path;\nString sep = \"/\";\nString windows_sep = \"\\\\\";\ndef idx = -1;\nif (path.contains(windows_sep)) {\n    idx = path.lastIndexOf(windows_sep);\n}\nelse {\n    idx = path.lastIndexOf(sep);\n} \nif (idx > -1) {\n    if (ctx.file.name == null) {\n        ctx.file.name = path.substring(idx+1);\n    }\n    ctx.file.directory = path.substring(0, idx);\n    def extIdx = path.lastIndexOf(\".\");\n    if (extIdx > -1 && ctx.file.extension == null) {\n        ctx.file.extension = path.substring(extIdx+1);\n    }\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }))
                        && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents")))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.md5")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.hash.md5", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }))
                        && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents")))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.sha1")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.hash.sha1", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }))
                        && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents")))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.sha256")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.hash.sha256", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }))
                        && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents")))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.file.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }))
                        && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents")))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.file.size")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("file.size", v)?;
                    }
                }
                if let Some(v) = event
                    .get("m365_defender.event.certificate.expiration_time")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.x509.not_after", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.certificate.serial_number")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.x509.serial_number", v)?;
                }
                let _cond = { event.has_value("m365_defender.event.issuer") };
                if _cond {
                    event.append(
                        "file.x509.issuer.common_name",
                        json!(
                            event
                                .get("m365_defender.event.issuer")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.signer")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.code_signature.subject_name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.is_signed")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.code_signature.exists", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.is_trusted")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.code_signature.trusted", v)?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.folder_path")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dll.path", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        })
                };
                if _cond {
                    let v = json!(format!(
                        "{}\\{}",
                        event
                            .get("m365_defender.event.folder_path")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("m365_defender.event.file.name")
                            .map_or_else(String::new, template_to_string)
                    ));
                    if !painless_is_empty_value(&v) {
                        event.set("dll.path", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.md5")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dll.hash.md5", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.md5")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dll.hash.md5", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.sha1")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dll.hash.sha1", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.sha1")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dll.hash.sha1", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.sha256")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dll.hash.sha256", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.sha256")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dll.hash.sha256", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.file.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dll.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.file.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dll.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.file.size")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dll.Ext.size", v)?;
                    }
                }
                let _cond = {
                    event.has_value("m365_defender.event.category")
                        && event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))
                        && event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| {
                                [
                                    "namedpipeevent",
                                    "dpapiaccessed",
                                    "ntallocatevirtualmemoryapicall",
                                    "getclipboarddata",
                                    "ntprotectvirtualmemoryapicall",
                                    "browserlaunchedtoopenurl",
                                    "processprimarytokenmodified",
                                    "powershellcommand",
                                    "clrunbackedmoduleloaded",
                                    "ldapsearch",
                                    "dnsqueryresponse",
                                    "ntallocatevirtualmemoryremoteapicall",
                                    "memoryremoteprotect",
                                    "screenshottaken",
                                    "antivirusscancompleted",
                                    "exploitguardwin32systemcallblocked",
                                    "getasynckeystateapicall",
                                    "appguardcreatecontainer",
                                    "exploitguardacgenforced",
                                    "writetolsassprocessmemory",
                                    "antivirusscancancelled",
                                    "controlflowguardviolation",
                                    "appcontrolpolicyapplied",
                                    "createremotethreadapicall",
                                    "auditpolicymodification",
                                    "ntmapviewofsectionremoteapicall",
                                    "appguardlaunchedwithurl",
                                    "appguardresumecontainer",
                                    "smartscreenurlwarning",
                                    "appguardbrowsetourl",
                                    "otheralertrelatedactivity",
                                    "antivirusscanfailed",
                                ]
                                .contains(&s.to_lowercase().as_str())
                            })
                };
                if _cond {
                    event.set("_temp_deviceevents_that_map_process", json!(true))?;
                }
                let _cond = {
                    !event.has_value("_temp_deviceevents_that_map_process")
                        && event.has_value("m365_defender.event.category")
                        && event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))
                };
                if _cond {
                    event.set("_temp_deviceevents_that_map_process", json!(false))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents")))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.folder_path")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.executable", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents")))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.md5")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.hash.md5", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents")))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.sha1")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.hash.sha1", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents")))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.sha256")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.hash.sha256", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceimageloadevents")))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.file.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.command_line")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.command_line", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.md5")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.hash.md5", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.sha1")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.hash.sha1", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.sha256")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.hash.sha256", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.parent_id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.group_leader.pid", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.pid", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.creation_time")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.start", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.file_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.folder_path")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.executable", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.parent_creation_time")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.group_leader.start", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.parent_file_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.group_leader.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.version_info_company_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.pe.company", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.version_info_file_description")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.pe.description", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event.get("m365_defender.event.initiating_process.version_info_original_file_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.parent.pe.original_file_name", v)?;
                }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.version_info_product_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.pe.product", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.version_info_product_version")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.pe.file_version", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.signature_status")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.code_signature.status", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                        && event.get_str("m365_defender.event.initiating_process.signature_status")
                            == Some("Valid")
                };
                if _cond {
                    event.set("process.parent.code_signature.exists", json!(true))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                        && event.get_str("m365_defender.event.initiating_process.signature_status")
                            == Some("Unsigned")
                };
                if _cond {
                    event.set("process.parent.code_signature.exists", json!(false))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                        && event.get_str("m365_defender.event.initiating_process.signature_status")
                            == Some("Valid")
                };
                if _cond {
                    event.set("process.parent.code_signature.status", json!("trusted"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                        && event.get_str("m365_defender.event.initiating_process.signature_status")
                            == Some("Valid")
                };
                if _cond {
                    event.set("process.parent.code_signature.trusted", json!(true))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || (event.has_value("_temp_deviceevents_that_map_process")
                            && event.get_bool("_temp_deviceevents_that_map_process")
                                == Some(false)))
                        && event.get_str("m365_defender.event.initiating_process.signature_status")
                            == Some("Unsigned")
                };
                if _cond {
                    event.set("process.parent.code_signature.trusted", json!(false))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        }) || event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))
                            || event.get("event.category").is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("library"))
                                }
                                serde_json::Value::String(s) => s.contains("library"),
                                _ => false,
                            }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.integrity_level")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.Ext.token.integrity_level_name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        })
                };
                if _cond {
                    // Painless script
                    // Source: String actiontype = ctx.m365_defender.event.action.type;\ndef idx = actiontype.toLowerCase().lastIndexOf('apicall');\nctx._temp_process_Ext_api_name = actiontype.substring(0, idx);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"String actiontype = ctx.m365_defender.event.action.type;\ndef idx = actiontype.toLowerCase().lastIndexOf('apicall');\nctx._temp_process_Ext_api_name = actiontype.substring(0, idx);\n"#
                        ),
                    )?;
                }
                if event.has_value("_temp_process_Ext_api_name") {
                    event.rename("_temp_process_Ext_api_name", "process.Ext.api.name")?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        })
                };
                if _cond {
                    if event.has_value("m365_defender.event.additional_fields.RegionSize") {
                        event.rename(
                            "m365_defender.event.additional_fields.RegionSize",
                            "process.Ext.api.parameters.size",
                        )?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        })
                };
                if _cond {
                    if event.has_value("m365_defender.event.additional_fields.ProtectionMask") {
                        event.rename(
                            "m365_defender.event.additional_fields.ProtectionMask",
                            "process.Ext.api.parameters.protection",
                        )?;
                    }
                }
                let _cond = { event.get_str("process.Ext.api.parameters.protection") != Some("") };
                if _cond {
                    if event.has_value("process.Ext.api.parameters.protection") {
                        if let Some(val) = event.get("process.Ext.api.parameters.protection") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "process.Ext.api.parameters.protection".into(),
                                    message,
                                }
                            })?;
                            event.set("process.Ext.api.parameters.protection", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        })
                };
                if _cond {
                    if event.has_value("m365_defender.event.additional_fields.BaseAddress") {
                        event.rename(
                            "m365_defender.event.additional_fields.BaseAddress",
                            "process.Ext.api.parameters.address",
                        )?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        })
                };
                if _cond {
                    if event.has_value("m365_defender.event.additional_fields.DesiredAccess") {
                        event.rename(
                            "m365_defender.event.additional_fields.DesiredAccess",
                            "process.Ext.api.parameters.desired_access_numeric",
                        )?;
                    }
                }
                if let Some(v) = event
                    .get("m365_defender.event.process_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.parent_process_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.parent.name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.parent_process_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.parent.pid", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.process_current_working_directory")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.working_directory", v)?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        })
                        && (event.has_value("m365_defender.event.action.type")
                            && event
                                .get_str("m365_defender.event.action.type")
                                .is_some_and(|s| {
                                    [
                                        "createremotethreadapicall",
                                        "readprocessmemoryapicall",
                                        "ntallocatevirtualmemoryremoteapicall",
                                        "openprocessapicall",
                                    ]
                                    .contains(&s.to_lowercase().as_str())
                                }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.file.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("Target.process.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        })
                        && (event.has_value("m365_defender.event.action.type")
                            && event
                                .get_str("m365_defender.event.action.type")
                                .is_some_and(|s| {
                                    [
                                        "createremotethreadapicall",
                                        "readprocessmemoryapicall",
                                        "ntallocatevirtualmemoryremoteapicall",
                                        "openprocessapicall",
                                    ]
                                    .contains(&s.to_lowercase().as_str())
                                }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.process.command_line")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("Target.process.command_line", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        })
                        && (event.has_value("m365_defender.event.action.type")
                            && event
                                .get_str("m365_defender.event.action.type")
                                .is_some_and(|s| {
                                    [
                                        "createremotethreadapicall",
                                        "readprocessmemoryapicall",
                                        "ntallocatevirtualmemoryremoteapicall",
                                        "openprocessapicall",
                                    ]
                                    .contains(&s.to_lowercase().as_str())
                                }))
                        && event.has_value("m365_defender.event.folder_path")
                        && event.has_value("m365_defender.event.file.name")
                        && event
                            .get_str("m365_defender.event.file.name")
                            .is_some_and(|p| {
                                event
                                    .get_str("m365_defender.event.folder_path")
                                    .is_some_and(|s| s.ends_with(p))
                            })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.folder_path")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("Target.process.executable", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        })
                        && (event.has_value("m365_defender.event.action.type")
                            && event
                                .get_str("m365_defender.event.action.type")
                                .is_some_and(|s| {
                                    [
                                        "createremotethreadapicall",
                                        "readprocessmemoryapicall",
                                        "ntallocatevirtualmemoryremoteapicall",
                                        "openprocessapicall",
                                    ]
                                    .contains(&s.to_lowercase().as_str())
                                }))
                        && !event.has_value("Target.process.executable")
                };
                if _cond {
                    let v = json!(format!(
                        "{}\\{}",
                        event
                            .get("m365_defender.event.folder_path")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("m365_defender.event.file.name")
                            .map_or_else(String::new, template_to_string)
                    ));
                    if !painless_is_empty_value(&v) {
                        event.set("Target.process.executable", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        })) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.command_line")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.command_line", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("library"))
                            }
                            serde_json::Value::String(s) => s.contains("library"),
                            _ => false,
                        })) && !(event.get("event.category").is_some_and(
                            |v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("process"))
                                }
                                serde_json::Value::String(s) => s.contains("process"),
                                _ => false,
                            },
                        )) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.md5")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.hash.md5", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("library"))
                            }
                            serde_json::Value::String(s) => s.contains("library"),
                            _ => false,
                        })) && !(event.get("event.category").is_some_and(
                            |v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("process"))
                                }
                                serde_json::Value::String(s) => s.contains("process"),
                                _ => false,
                            },
                        )) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.sha1")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.hash.sha1", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("library"))
                            }
                            serde_json::Value::String(s) => s.contains("library"),
                            _ => false,
                        })) && !(event.get("event.category").is_some_and(
                            |v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("process"))
                                }
                                serde_json::Value::String(s) => s.contains("process"),
                                _ => false,
                            },
                        )) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.sha256")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.hash.sha256", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        })) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.parent_id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.pid", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        })) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.pid", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        })) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.creation_time")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.start", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        })) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.file_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        })) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents")))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.folder_path")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.executable", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && event.has_value("m365_defender.event.initiating_process.folder_path")
                        && event.has_value("m365_defender.event.initiating_process.file_name")
                        && event
                            .get_str("m365_defender.event.initiating_process.file_name")
                            .is_some_and(|p| {
                                event
                                    .get_str("m365_defender.event.initiating_process.folder_path")
                                    .is_some_and(|s| s.ends_with(p))
                            })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.folder_path")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.executable", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                        && !event.has_value("process.executable")
                };
                if _cond {
                    let v = json!(format!(
                        "{}\\{}",
                        event
                            .get("m365_defender.event.initiating_process.folder_path")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("m365_defender.event.initiating_process.file_name")
                            .map_or_else(String::new, template_to_string)
                    ));
                    if !painless_is_empty_value(&v) {
                        event.set("process.executable", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.parent_file_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.parent_id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.pid", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        })) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.parent_creation_time")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.start", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        })) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.parent_file_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.parent.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.parent_id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.group_leader.pid", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.parent_file_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.group_leader.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.parent_creation_time")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.group_leader.start", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("library"))
                            }
                            serde_json::Value::String(s) => s.contains("library"),
                            _ => false,
                        })) && !(event.get("event.category").is_some_and(
                            |v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("process"))
                                }
                                serde_json::Value::String(s) => s.contains("process"),
                                _ => false,
                            },
                        )) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.version_info_company_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.pe.company", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("library"))
                            }
                            serde_json::Value::String(s) => s.contains("library"),
                            _ => false,
                        })) && !(event.get("event.category").is_some_and(
                            |v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("process"))
                                }
                                serde_json::Value::String(s) => s.contains("process"),
                                _ => false,
                            },
                        )) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.version_info_file_description")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.pe.description", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("library"))
                            }
                            serde_json::Value::String(s) => s.contains("library"),
                            _ => false,
                        })) && !(event.get("event.category").is_some_and(
                            |v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("process"))
                                }
                                serde_json::Value::String(s) => s.contains("process"),
                                _ => false,
                            },
                        )) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event.get("m365_defender.event.initiating_process.version_info_original_file_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("process.pe.original_file_name", v)?;
                }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("library"))
                            }
                            serde_json::Value::String(s) => s.contains("library"),
                            _ => false,
                        })) && !(event.get("event.category").is_some_and(
                            |v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("process"))
                                }
                                serde_json::Value::String(s) => s.contains("process"),
                                _ => false,
                            },
                        )) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.version_info_product_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.pe.product", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("library"))
                            }
                            serde_json::Value::String(s) => s.contains("library"),
                            _ => false,
                        })) && !(event.get("event.category").is_some_and(
                            |v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some("process"))
                                }
                                serde_json::Value::String(s) => s.contains("process"),
                                _ => false,
                            },
                        )) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.version_info_product_version")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.pe.file_version", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || (!(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        })) && !(event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("deviceevents"))))
                            || (event.has_value("_temp_deviceevents_that_map_process")
                                && event.get_bool("_temp_deviceevents_that_map_process")
                                    == Some(true)))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.signature_status")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("process.code_signature.status", v)?;
                    }
                }
                event.remove("_temp_deviceevents_that_map_process");
                if let Some(v) = event
                    .get("m365_defender.event.process.command_line")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.command_line", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.process.creation_time")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.start", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.process.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.pid", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.process.version_info_company_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.pe.company", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.process.version_info_file_description")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.pe.description", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.process.version_info_original_file_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.pe.original_file_name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.process.version_info_product_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.pe.product", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.process.version_info_product_version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.pe.file_version", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.device.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
                let _cond = { event.has_value("host.name") };
                if _cond {
                    map_strings(event, "host.name", "host.name", str::to_lowercase)?;
                }
                let _cond = { event.has_value("m365_defender.event.device.name") };
                if _cond {
                    map_strings(
                        event,
                        "m365_defender.event.device.name",
                        "host.hostname",
                        str::to_lowercase,
                    )?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.device.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.id", v)?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.public_ip.value")
                        && event.get_str("m365_defender.event.public_ip.value") != Some("")
                };
                if _cond {
                    event.append(
                        "host.ip",
                        json!(
                            event
                                .get("m365_defender.event.public_ip.value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.os.architecture")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.architecture", v)?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.os.platform")
                        && event
                            .get_str("m365_defender.event.os.platform")
                            .is_some_and(|s| s.to_lowercase().contains("windows"))
                };
                if _cond {
                    event.set("host.os.type", json!("windows"))?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.os.platform")
                        && event
                            .get_str("m365_defender.event.os.platform")
                            .is_some_and(|s| s.to_lowercase().contains("linux"))
                };
                if _cond {
                    event.set("host.os.type", json!("linux"))?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.os.platform")
                        && event
                            .get_str("m365_defender.event.os.platform")
                            .is_some_and(|s| s.to_lowercase().contains("macos"))
                };
                if _cond {
                    event.set("host.os.type", json!("macos"))?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.additional_fields") && !(event.get("m365_defender.event.additional_fields").is_some_and(|v| v.is_array())) && event.has_value("m365_defender.event.additional_fields.InitiatingProcessPosixEffectiveGroup") && event.has_value("m365_defender.event.additional_fields.InitiatingProcessPosixEffectiveUser") && event.has_value("m365_defender.event.additional_fields.InitiatingProcessPosixFilePermissions") && event.has_value("m365_defender.event.additional_fields.InitiatingProcessPosixProcessGroupId") && event.has_value("m365_defender.event.additional_fields.InitiatingProcessPosixRealUser") && event.has_value("m365_defender.event.additional_fields.InitiatingProcessPosixSessionId")
                };
                if _cond {
                    event.set("_tmp.posix", json!(true))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get_bool("_tmp.posix") != Some(true)
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("registry"))
                            }
                            serde_json::Value::String(s) => s.contains("registry"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("library"))
                            }
                            serde_json::Value::String(s) => s.contains("library"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }))
                };
                if _cond {
                    event.set("host.os.type", json!("windows"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("file"))
                            }
                            serde_json::Value::String(s) => s.contains("file"),
                            _ => false,
                        })
                        && (event.has_value("m365_defender.event.initiating_process.account_sid")
                            || event.has_value("m365_defender.event.request.account_sid"))
                };
                if _cond {
                    event.set("host.os.type", json!("windows"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get_bool("_tmp.posix") != Some(true)
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        })
                        && (event.has_value("m365_defender.event.initiating_process.account_sid"))
                };
                if _cond {
                    event.set("host.os.type", json!("windows"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get_bool("_tmp.posix") != Some(true)
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("authentication"))
                            }
                            serde_json::Value::String(s) => s.contains("authentication"),
                            _ => false,
                        })
                        && (event.has_value("m365_defender.event.initiating_process.account_sid")
                            || event.has_value("m365_defender.event.account.sid"))
                };
                if _cond {
                    event.set("host.os.type", json!("windows"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get_bool("_tmp.posix") != Some(true)
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && (event.has_value("m365_defender.event.initiating_process.account_sid"))
                };
                if _cond {
                    event.set("host.os.type", json!("windows"))?;
                }
                let _cond = {
                    !event.has_value("host.os.type") && event.get_bool("_tmp.posix") == Some(true)
                };
                if _cond {
                    event.set("host.os.type", json!("unix"))?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.os.platform")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.full", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.os.distribution")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.platform", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.os.version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.version", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.device.type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.type", v)?;
                }
                if event.has_value("m365_defender.event.mac_address") {
                    gsub_field(
                        event,
                        "m365_defender.event.mac_address",
                        "m365_defender.event.mac_address",
                        cached_regex!("[:.]"),
                        "-",
                    )?;
                }
                if event.has_value("m365_defender.event.mac_address") {
                    map_strings(
                        event,
                        "m365_defender.event.mac_address",
                        "m365_defender.event.mac_address",
                        str::to_uppercase,
                    )?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.mac_address")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("_tmp.mac", v)?;
                }
                let _cond = {
                    !(event.get("_tmp.mac").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("-")),
                        serde_json::Value::String(s) => s.contains("-"),
                        _ => false,
                    }))
                };
                if _cond {
                    if event.has_value("_tmp.mac") {
                        gsub_field(
                            event,
                            "_tmp.mac",
                            "_tmp.mac",
                            cached_regex!("(..)(?!$)"),
                            "$1-",
                        )?;
                    }
                }
                let _cond = { event.has_value("_tmp.mac") };
                if _cond {
                    event.append_unique(
                        "host.mac",
                        json!(
                            event
                                .get("_tmp.mac")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.registry.key")
                        && event.get_str("m365_defender.event.registry.key") != Some("")
                };
                if _cond {
                    if let Some(input) = event.get_string("m365_defender.event.registry.key") {
                        // Grok pattern: ^((?P<_tmp_registry_hive>(?:(?i:HKEY_CLASSES_ROOT|HKCR|HKEY_CURRENT_USER|HKCU|HKEY_LOCAL_MACHINE|HKLM|HKEY_USERS|HKU|HKEY_CURRENT_CONFIG|HKCC)))\\\\)?%{GREEDYDATA:registry.key}$
                        if !cached_grok_mapped!("^((?P<_tmp_registry_hive>(?:(?i:HKEY_CLASSES_ROOT|HKCR|HKEY_CURRENT_USER|HKCU|HKEY_LOCAL_MACHINE|HKLM|HKEY_USERS|HKU|HKEY_CURRENT_CONFIG|HKCC)))\\\\)?%{GREEDYDATA:registry.key}$", [("_tmp_registry_hive", "_tmp.registry.hive")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                }
                let _cond = {
                    !event.has_value("registry.key")
                        && event.has_value("m365_defender.event.previous.registry_key")
                        && event.get_str("m365_defender.event.previous.registry_key") != Some("")
                };
                if _cond {
                    if let Some(input) =
                        event.get_string("m365_defender.event.previous.registry_key")
                    {
                        // Grok pattern: ^((?P<_tmp_registry_hive>(?:(?i:HKEY_CLASSES_ROOT|HKCR|HKEY_CURRENT_USER|HKCU|HKEY_LOCAL_MACHINE|HKLM|HKEY_USERS|HKU|HKEY_CURRENT_CONFIG|HKCC)))\\\\)?%{GREEDYDATA:registry.key}$
                        if !cached_grok_mapped!("^((?P<_tmp_registry_hive>(?:(?i:HKEY_CLASSES_ROOT|HKCR|HKEY_CURRENT_USER|HKCU|HKEY_LOCAL_MACHINE|HKLM|HKEY_USERS|HKU|HKEY_CURRENT_CONFIG|HKCC)))\\\\)?%{GREEDYDATA:registry.key}$", [("_tmp_registry_hive", "_tmp.registry.hive")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                }
                let _cond = { event.has_value("_tmp.registry.hive") };
                if _cond {
                    // Painless script
                    // Source: def name = ctx._tmp.registry.hive.toUpperCase();\nif (ctx.registry == null) {\n  ctx.registry = new HashMap();\n}\nctx.registry.hive = params.getOrDefault(name, name);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def name = ctx._tmp.registry.hive.toUpperCase();\nif (ctx.registry == null) {\n  ctx.registry = new HashMap();\n}\nctx.registry.hive = params.getOrDefault(name, name);\n"#
                        ),
                        cached_params!(
                            "{\"HKEY_CLASSES_ROOT\":\"HKCR\",\"HKEY_CURRENT_USER\":\"HKCU\",\"HKEY_LOCAL_MACHINE\":\"HKLM\",\"HKEY_USERS\":\"HKU\",\"HKEY_CURRENT_CONFIG\":\"HKCC\"}"
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.registry.value_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("registry.value", v)?;
                }
                let _cond = { !event.has_value("registry.value") };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.previous.registry_value_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("registry.value", v)?;
                    }
                }
                let _cond = {
                    event.has_value("registry.key")
                        && event.get_str("registry.key") != Some("")
                        && event.has_value("registry.value")
                };
                if _cond {
                    let v = json!(format!(
                        "{}\\{}\\{}",
                        if event.get("registry.hive").is_some_and(|v| !v.is_null()
                            && v.as_str() != Some("")
                            && !matches!(v, Value::Bool(false))
                            && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            event
                                .get("registry.hive")
                                .map_or_else(String::new, template_to_string)
                        } else {
                            String::new()
                        },
                        event
                            .get("registry.key")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("registry.value")
                            .map_or_else(String::new, template_to_string)
                    ));
                    if !painless_is_empty_value(&v) {
                        event.set("registry.path", v)?;
                    }
                }
                let _cond = { event.has_value("m365_defender.event.registry.value_data") };
                if _cond {
                    event.append_unique(
                        "registry.data.strings",
                        json!(
                            event
                                .get("m365_defender.event.registry.value_data")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.previous.registry_value_data") };
                if _cond {
                    event.append_unique(
                        "registry.data.strings",
                        json!(
                            event
                                .get("m365_defender.event.previous.registry_value_data")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.registry.value_type") };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.registry.value_type")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("registry.data.type", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("authentication"))
                            }
                            serde_json::Value::String(s) => s.contains("authentication"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.remote.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.ip", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.get_str("m365_defender.event.network_direction") == Some("In")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.remote.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.ip", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("authentication"))
                            }
                            serde_json::Value::String(s) => s.contains("authentication"),
                            _ => false,
                        }))
                        && !event.has_value("m365_defender.event.network_direction")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.local.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.ip", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.get_str("m365_defender.event.network_direction") == Some("Out")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.local.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.ip", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("file"))
                            }
                            serde_json::Value::String(s) => s.contains("file"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.request.source_ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.ip", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.get_str("m365_defender.event.network_direction") == Some("In")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.remote.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.port", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("authentication"))
                            }
                            serde_json::Value::String(s) => s.contains("authentication"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.remote.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.port", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("authentication"))
                            }
                            serde_json::Value::String(s) => s.contains("authentication"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.local.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.port", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.get_str("m365_defender.event.network_direction") == Some("Out")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.local.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("source.port", v)?;
                    }
                }
                if let Some(v) = event
                    .get("m365_defender.event.request.source_port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.port", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.remote.device_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.domain", v)?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.get_str("m365_defender.event.network_direction") == Some("Out")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.remote.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.ip", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("authentication"))
                            }
                            serde_json::Value::String(s) => s.contains("authentication"),
                            _ => false,
                        }))
                        && !event.has_value("m365_defender.event.network_direction")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.remote.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.ip", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.get_str("m365_defender.event.network_direction") == Some("In")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.local.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.ip", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.has_value("destination.ip")
                };
                if _cond {
                    if let Some(v) = event
                        .get("destination.ip")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.address", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("authentication"))
                            }
                            serde_json::Value::String(s) => s.contains("authentication"),
                            _ => false,
                        }))
                        && !event.has_value("m365_defender.event.network_direction")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.remote.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.port", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.get_str("m365_defender.event.network_direction") == Some("Out")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.remote.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.port", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.get_str("m365_defender.event.network_direction") == Some("In")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.local.port")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("destination.port", v)?;
                    }
                }
                if let Some(v) = event
                    .get("m365_defender.event.account.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.request.account_domain")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.domain", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.request.account_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("library"))
                            }
                            serde_json::Value::String(s) => s.contains("library"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("registry"))
                            }
                            serde_json::Value::String(s) => s.contains("registry"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !event.has_value("user.name")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.account_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.name", v)?;
                    }
                }
                if let Some(v) = event
                    .get("m365_defender.event.account.domain")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.domain", v)?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("library"))
                            }
                            serde_json::Value::String(s) => s.contains("library"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("registry"))
                            }
                            serde_json::Value::String(s) => s.contains("registry"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !event.has_value("user.domain")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.account_domain")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.domain", v)?;
                    }
                }
                if let Some(v) = event
                    .get("m365_defender.event.account.sid")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && (event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("library"))
                            }
                            serde_json::Value::String(s) => s.contains("library"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("file"))
                            }
                            serde_json::Value::String(s) => s.contains("file"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("registry"))
                            }
                            serde_json::Value::String(s) => s.contains("registry"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        }) || event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("api"))
                            }
                            serde_json::Value::String(s) => s.contains("api"),
                            _ => false,
                        }))
                        && !event.has_value("user.id")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.initiating_process.account_sid")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.id", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("file"))
                            }
                            serde_json::Value::String(s) => s.contains("file"),
                            _ => false,
                        })
                        && !event.has_value("user.id")
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.request.account_sid")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.id", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.dns.query")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dns.question.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.dns.qclass_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dns.question.class", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.dns.qtype_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dns.question.type", v)?;
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.dns.rcode_name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dns.response_code", v)?;
                    }
                }
                let _cond = {
                    event.has_value("m365_defender.event.dns.answers")
                        && event.has_value("m365_defender.event.dns.ttls")
                };
                if _cond {
                    // Painless script
                    // Source: def answers = ctx.m365_defender.event.dns.answers; def ttls = ctx.m365_defender.event.dns.ttls; if (answers.isEmpty() || ttls.isEmpty()) {\n  return;\n} else if (answers.length != ttls.length) {\n  if (ctx.error == null) {\n    ctx.error = new HashMap();\n  }\n  if (ctx.error.message == null) {\n    ctx.error.message = new ArrayList();\n  }\n  ctx.error.message.add('DNS answers and TTLs have a different length');\n} def lst = new ArrayList(); for (def i = 0; i < answers.length; i++) {\n  lst.add([\n    \"data\": answers[i],\n    \"ttl\": (long)ttls[i]\n  ])\n} if (ctx.dns == null) {\n  ctx.dns = new HashMap();\n} ctx.dns.answers = lst;
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def answers = ctx.m365_defender.event.dns.answers; def ttls = ctx.m365_defender.event.dns.ttls; if (answers.isEmpty() || ttls.isEmpty()) {\n  return;\n} else if (answers.length != ttls.length) {\n  if (ctx.error == null) {\n    ctx.error = new HashMap();\n  }\n  if (ctx.error.message == null) {\n    ctx.error.message = new ArrayList();\n  }\n  ctx.error.message.add('DNS answers and TTLs have a different length');\n} def lst = new ArrayList(); for (def i = 0; i < answers.length; i++) {\n  lst.add([\n    \"data\": answers[i],\n    \"ttl\": (long)ttls[i]\n  ])\n} if (ctx.dns == null) {\n  ctx.dns = new HashMap();\n} ctx.dns.answers = lst;"#
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase().contains("dnsconnectioninspected"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.dns.header_flags")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("dns.header_flags", v)?;
                    }
                }
                if let Some(v) = event
                    .get("m365_defender.event.protocol")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("network.transport", v)?;
                }
                if event.has_value("network.transport") {
                    map_strings(
                        event,
                        "network.transport",
                        "network.transport",
                        str::to_lowercase,
                    )?;
                }
                let _cond = { event.get_str("network.transport") == Some("ntlm") };
                if _cond {
                    event.rename("network.transport", "network.protocol")?;
                }
                let _cond = { event.get_str("network.protocol") == Some("icmp") };
                if _cond {
                    event.rename("network.protocol", "network.transport")?;
                }
                let _cond = {
                    event.get_str("network.transport") == Some("tcpv4")
                        || event.get_str("network.transport") == Some("tcpv6")
                };
                if _cond {
                    event.set("network.transport", json!("tcp"))?;
                }
                let _cond = {
                    event.get_str("network.transport") == Some("negotiate")
                        || event.get_str("network.transport")
                            == Some("microsoft_authentication_package_v1_0")
                };
                if _cond {
                    if event.remove("network.transport").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "network.transport".into(),
                        });
                    }
                }
                if let Some(v) = event
                    .get("m365_defender.event.request.protocol")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("network.protocol", v)?;
                }
                if event.has_value("network.protocol") {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase().contains("dns"))
                };
                if _cond {
                    event.set("network.protocol", json!("dns"))?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase().contains("http"))
                };
                if _cond {
                    event.set("network.protocol", json!("http"))?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase().contains("ssl"))
                };
                if _cond {
                    event.set("network.protocol", json!("ssl"))?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase().contains("ftp"))
                };
                if _cond {
                    event.set("network.protocol", json!("ftp"))?;
                }
                let _cond = {
                    event.get_str("network.protocol") == Some("local")
                        || event.get_str("network.protocol") == Some("unknown")
                };
                if _cond {
                    if event.remove("network.protocol").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "network.protocol".into(),
                        });
                    }
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.get_str("m365_defender.event.network_direction") == Some("In")
                };
                if _cond {
                    event.set("network.direction", json!("inbound"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && event.get_str("m365_defender.event.network_direction") == Some("Out")
                };
                if _cond {
                    event.set("network.direction", json!("outbound"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("network"))
                            }
                            serde_json::Value::String(s) => s.contains("network"),
                            _ => false,
                        })
                        && !event.has_value("m365_defender.event.network_direction")
                };
                if _cond {
                    event.set("network.direction", json!("unknown"))?;
                }
                let _cond = {
                    (event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("file"))
                            }
                            serde_json::Value::String(s) => s.contains("file"),
                            _ => false,
                        }))
                        && event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase() == "filedeleted")
                };
                if _cond {
                    event.set("event.action", json!("deletion"))?;
                }
                let _cond = {
                    (event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("file"))
                            }
                            serde_json::Value::String(s) => s.contains("file"),
                            _ => false,
                        }))
                        && event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase() == "filemodified")
                };
                if _cond {
                    event.set("event.action", json!("modification"))?;
                }
                let _cond = {
                    (event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("file"))
                            }
                            serde_json::Value::String(s) => s.contains("file"),
                            _ => false,
                        }))
                        && event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase() == "filerenamed")
                };
                if _cond {
                    event.set("event.action", json!("rename"))?;
                }
                let _cond = {
                    (event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("file"))
                            }
                            serde_json::Value::String(s) => s.contains("file"),
                            _ => false,
                        }))
                        && event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase() == "filecreated")
                };
                if _cond {
                    event.set("event.action", json!("creation"))?;
                }
                let _cond = {
                    (event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("registry"))
                            }
                            serde_json::Value::String(s) => s.contains("registry"),
                            _ => false,
                        }))
                        && event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase() == "registrykeycreated")
                };
                if _cond {
                    event.set("event.action", json!("creation"))?;
                }
                let _cond = {
                    (event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("registry"))
                            }
                            serde_json::Value::String(s) => s.contains("registry"),
                            _ => false,
                        }))
                        && event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase() == "registryvalueset")
                };
                if _cond {
                    event.set("event.action", json!("modification"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("driver"))
                            }
                            serde_json::Value::String(s) => s.contains("driver"),
                            _ => false,
                        })
                };
                if _cond {
                    event.set("event.action", json!("load"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("process"))
                            }
                            serde_json::Value::String(s) => s.contains("process"),
                            _ => false,
                        })
                        && event.has_value("m365_defender.event.action.type")
                        && event
                            .get_str("m365_defender.event.action.type")
                            .is_some_and(|s| s.to_lowercase() == "processcreated")
                };
                if _cond {
                    event.set("event.action", json!("start"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.category").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("file"))
                            }
                            serde_json::Value::String(s) => s.contains("file"),
                            _ => false,
                        }))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.action.type")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        if !event.has("event.action") {
                            event.set("event.action", v)?;
                        }
                    }
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                if event.has_value("event.action") {
                    gsub_field(
                        event,
                        "event.action",
                        "event.action",
                        cached_regex!(" "),
                        "-",
                    )?;
                }
                let _cond = {
                    (!event.has_value("m365_defender.event.failure_reason")
                        || event.get_str("m365_defender.event.failure_reason") == Some(""))
                        && event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("devicelogonevents"))
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    (event.has_value("m365_defender.event.failure_reason")
                        && event.get_str("m365_defender.event.failure_reason") != Some(""))
                        && event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("devicelogonevents"))
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.client_version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("observer.version", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.device.category")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("observer.type", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.kubernetes_namespace")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.namespace", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.kubernetes_resource")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.resource.name", v)?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.kubernetes_namespace")
                        || event.has_value("m365_defender.event.kubernetes_pod_name")
                        || event.has_value("m365_defender.event.kubernetes_resource")
                };
                if _cond {
                    event.set("orchestrator.type", json!("kubernetes"))?;
                }
                let _cond = { event.has_value("m365_defender.event.azure_resource_id") };
                if _cond {
                    if !event.has("cloud.provider") {
                        event.set("cloud.provider", json!("azure"))?;
                    }
                }
                let _cond = { event.has_value("m365_defender.event.aws_resource_name") };
                if _cond {
                    if !event.has("cloud.provider") {
                        event.set("cloud.provider", json!("aws"))?;
                    }
                }
                let _cond = { event.has_value("m365_defender.event.gcp_full_resource_name") };
                if _cond {
                    if !event.has("cloud.provider") {
                        event.set("cloud.provider", json!("gcp"))?;
                    }
                }
                let _cond = {
                    event.has_value("m365_defender.event.remote.url")
                        && event.get_str("m365_defender.event.remote.url") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        uri_parts(event, "m365_defender.event.remote.url", "url", true, false)?;
                        Ok(())
                    })();
                }
                if let Some(v) = event
                    .get("m365_defender.event.file.origin_referrer_url")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("http.request.referrer", v)?;
                }
                let _cond = {
                    event.has_value("json.properties.NetworkAdapterName")
                        && event
                            .get_str("json.properties.NetworkAdapterName")
                            .is_some_and(|s| s.starts_with("{"))
                };
                if _cond {
                    if let Some(input) = event.get_string("json.properties.NetworkAdapterName") {
                        // Grok pattern: ^{%{DATA:m365_defender.event.network.adapter_name}}$
                        if !cached_grok!("^{%{DATA:m365_defender.event.network.adapter_name}}$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.has_value("json.properties.NetworkAdapterName")
                        && !(event
                            .get_str("json.properties.NetworkAdapterName")
                            .is_some_and(|s| s.starts_with("{")))
                };
                if _cond {
                    if let Some(v) = event
                        .get("json.properties.NetworkAdapterName")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("m365_defender.event.network.adapter_name", v)?;
                    }
                }
                let _cond = {
                    event
                        .get("json.properties.LoggedOnUsers")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "json.properties.LoggedOnUsers", |event| {
                        event.append(
                            "m365_defender.event.active_users",
                            json!(
                                event
                                    .get("_ingest._value.UserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    event
                        .get("m365_defender.event.active_users")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    foreach_array(event, "m365_defender.event.active_users", |event| {
                        event.append(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
                let _cond = {
                    (event.has_value("process.command_line")
                        && event.get_str("process.command_line") != Some(""))
                        || (event.has_value("process.parent.command_line")
                            && event.get_str("process.parent.command_line") != Some(""))
                };
                if _cond {
                    // Painless script
                    // Source: // appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string into next\n// argument and command line remainder offset.\ndef readNextArg(String line, int offset) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; offset < line.length(); offset++) {\n        def c = line.charAt(offset);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"offset\": offset+1\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && offset+1 < line.length() && line.charAt(offset+1) == (char)'\"') {\n                    b.append(c);\n                    offset++;\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"offset\": line.length()\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String line) {\n    def args = new ArrayList();\n    for (int i = 0; i < line.length();) {\n        if (line.charAt(i) == (char)' ' || line.charAt(i) == (char)0x09) {\n            i++;\n            continue;\n        }\n        def next = readNextArg(line, i);\n        i = next.offset;\n        if (next.arg == '') {\n            // Empty strings will be removed later so don't bother adding them.\n            continue;\n        }\n        args.add(next.arg);\n    }\n    return args;\n}\n\nif (ctx.process?.command_line != null && ctx.process.command_line != '') {\n  ctx.process.args = commandLineToArgv(ctx.process.command_line);\n  ctx.process.args_count = ctx.process.args.length;\n}\nif (ctx.process?.parent?.command_line != null && ctx.process.parent.command_line != '') {\n  ctx.process.parent.args = commandLineToArgv(ctx.process.parent.command_line);\n  ctx.process.parent.args_count = ctx.process.parent.args.length;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"// appendBSBytes appends n '\\\\' bytes to b and returns the resulting slice.\ndef appendBSBytes(StringBuilder b, int n) {\n    for (; n > 0; n--) {\n        b.append('\\\\');\n    }\n    return b;\n}\n\n// readNextArg splits command line string into next\n// argument and command line remainder offset.\ndef readNextArg(String line, int offset) {\n    def b = new StringBuilder();\n    boolean inquote;\n    int nslash;\n    for (; offset < line.length(); offset++) {\n        def c = line.charAt(offset);\n        if (c == (char)' ' || c == (char)0x09) {\n            if (!inquote) {\n                return [\n                    \"arg\":  appendBSBytes(b, nslash).toString(),\n                    \"offset\": offset+1\n                ];\n            }\n        } else if (c == (char)'\"') {\n            b = appendBSBytes(b, nslash/2);\n            if (nslash%2 == 0) {\n                // use \"Prior to 2008\" rule from\n                // http://daviddeley.com/autohotkey/parameters/parameters.htm\n                // section 5.2 to deal with double double quotes\n                if (inquote && offset+1 < line.length() && line.charAt(offset+1) == (char)'\"') {\n                    b.append(c);\n                    offset++;\n                }\n                inquote = !inquote;\n            } else {\n                b.append(c);\n            }\n            nslash = 0;\n            continue;\n        } else if (c == (char)'\\\\') {\n            nslash++;\n            continue;\n        }\n        b = appendBSBytes(b, nslash);\n        nslash = 0;\n        b.append(c);\n    }\n    return [\n        \"arg\":  appendBSBytes(b, nslash).toString(), \n        \"offset\": line.length()\n    ];\n}\n\n// commandLineToArgv splits a command line into individual argument\n// strings, following the Windows conventions documented\n// at http://daviddeley.com/autohotkey/parameters/parameters.htm#WINARGV\n// Original implementation found at: https://github.com/golang/go/commit/39c8d2b7faed06b0e91a1ad7906231f53aab45d1\ndef commandLineToArgv(String line) {\n    def args = new ArrayList();\n    for (int i = 0; i < line.length();) {\n        if (line.charAt(i) == (char)' ' || line.charAt(i) == (char)0x09) {\n            i++;\n            continue;\n        }\n        def next = readNextArg(line, i);\n        i = next.offset;\n        if (next.arg == '') {\n            // Empty strings will be removed later so don't bother adding them.\n            continue;\n        }\n        args.add(next.arg);\n    }\n    return args;\n}\n\nif (ctx.process?.command_line != null && ctx.process.command_line != '') {\n  ctx.process.args = commandLineToArgv(ctx.process.command_line);\n  ctx.process.args_count = ctx.process.args.length;\n}\nif (ctx.process?.parent?.command_line != null && ctx.process.parent.command_line != '') {\n  ctx.process.parent.args = commandLineToArgv(ctx.process.parent.command_line);\n  ctx.process.parent.args_count = ctx.process.parent.args.length;\n}"#
                        ),
                    )?;
                }
                let _cond = { event.has_value("host.ip") };
                if _cond {
                    if let Some(ip_str) = event.get_string("host.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("host.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("host.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("host.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("host.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("host.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("host.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("host.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("host.geo.location", v.clone())?;
                            }
                        }
                    }
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
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
                let _cond = { event.has_value("destination.ip") };
                if _cond {
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
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
                if event.has_value("destination.as.asn") {
                    event.rename("destination.as.asn", "destination.as.number")?;
                }
                if event.has_value("destination.as.organization_name") {
                    event.rename(
                        "destination.as.organization_name",
                        "destination.as.organization.name",
                    )?;
                }
                let _cond = { event.has_value("host.id") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.id")
                                .map_or_else(String::new, template_to_string)
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
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("user.domain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("user.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond =
                    { event.has_value("m365_defender.event.initiating_process.account_domain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("m365_defender.event.initiating_process.account_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("user.name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond =
                    { event.has_value("m365_defender.event.initiating_process.account_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("m365_defender.event.initiating_process.account_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.file.origin_ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("m365_defender.event.file.origin_ip")
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
                let _cond = { event.get("host.ip").is_some_and(|v| v.is_array()) };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        foreach_array(event, "host.ip", |event| {
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
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("m365_defender.event.ipv4_dhcp") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("m365_defender.event.ipv4_dhcp")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.ipv6_dhcp") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("m365_defender.event.ipv6_dhcp")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("file.hash.md5") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("file.hash.sha1") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.sha1")
                                .map_or_else(String::new, template_to_string)
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
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("process.hash.md5") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("process.hash.sha1") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.hash.sha1")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("process.hash.sha256") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("process.hash.md5") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.parent.hash.md5")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("process.hash.sha1") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.parent.hash.sha1")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("process.hash.sha256") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("process.parent.hash.sha256")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.issuer_hash") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("m365_defender.event.issuer_hash")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.signer_hash") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("m365_defender.event.signer_hash")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
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
                    event.remove("m365_defender.event.folder_path");
                    event.remove("m365_defender.event.md5");
                    event.remove("m365_defender.event.sha1");
                    event.remove("m365_defender.event.sha256");
                    event.remove("m365_defender.event.file.name");
                    event.remove("m365_defender.event.file.size");
                    event.remove("m365_defender.event.file.origin_referrer_url");
                    event.remove("m365_defender.event.device.name");
                    event.remove("m365_defender.event.device.id");
                    event.remove("m365_defender.event.process.version_info_company_name");
                    event.remove("m365_defender.event.process.version_info_product_name");
                    event.remove("m365_defender.event.process.version_info_product_version");
                    event.remove("m365_defender.event.process.version_info_file_description");
                    event.remove("m365_defender.event.initiating_process.file_name");
                    event.remove(
                        "m365_defender.event.initiating_process.version_info_product_version",
                    );
                    event.remove(
                        "m365_defender.event.initiating_process.version_info_file_description",
                    );
                    event.remove(
                        "m365_defender.event.initiating_process.version_info_original_file_name",
                    );
                    event.remove("m365_defender.event.initiating_process.file_size");
                    event
                        .remove("m365_defender.event.initiating_process.version_info_company_name");
                    event
                        .remove("m365_defender.event.initiating_process.version_info_product_name");
                    event.remove("m365_defender.event.initiating_process.folder_path");
                    event.remove("m365_defender.event.initiating_process.command_line");
                    event.remove("m365_defender.event.initiating_process.md5");
                    event.remove("m365_defender.event.initiating_process.sha1");
                    event.remove("m365_defender.event.initiating_process.sha256");
                    event.remove("m365_defender.event.initiating_process.parent_id");
                    event.remove("m365_defender.event.initiating_process.id");
                    event.remove("m365_defender.event.initiating_process.parent_file_name");
                    event.remove("m365_defender.event.initiating_process.parent_creation_time");
                    event.remove("m365_defender.event.initiating_process.signature_status");
                    event.remove("m365_defender.event.registry.key");
                    event.remove("m365_defender.event.registry.value_name");
                    event.remove("m365_defender.event.registry.value_data");
                    event.remove("m365_defender.event.public_ip.value");
                    event.remove("m365_defender.event.local.ip");
                    event.remove("m365_defender.event.remote.ip");
                    event.remove("m365_defender.event.request.source_ip");
                    event.remove("m365_defender.event.local.port");
                    event.remove("m365_defender.event.remote.port");
                    event.remove("m365_defender.event.request.source_port");
                    event.remove("m365_defender.event.account.name");
                    event.remove("m365_defender.event.account.sid");
                    event.remove("m365_defender.event.certificate.expiration_time");
                    event.remove("m365_defender.event.certificate.serial_number");
                    event.remove("m365_defender.event.protocol");
                    event.remove("m365_defender.event.request.protocol");
                    event.remove("m365_defender.event.request.account_domain");
                    event.remove("m365_defender.event.request.account_name");
                    event.remove("m365_defender.event.os.architecture");
                    event.remove("m365_defender.event.os.platform");
                    event.remove("m365_defender.event.os.distribution");
                    event.remove("m365_defender.event.os.version");
                    event.remove("m365_defender.event.device.type");
                    event.remove("m365_defender.event.account.domain");
                    event.remove("m365_defender.event.mac_address");
                    event.remove("m365_defender.event.client_version");
                    event.remove("m365_defender.event.device.category");
                    event.remove("m365_defender.event.action.type");
                    event.remove("m365_defender.event.is_signed");
                    event.remove("m365_defender.event.signer");
                    event.remove("m365_defender.event.issuer");
                    event.remove("m365_defender.event.is_trusted");
                    event.remove("m365_defender.event.dns.qclass_name");
                    event.remove("m365_defender.event.dns.query");
                    event.remove("m365_defender.event.dns.qtype_name");
                    event.remove("m365_defender.event.dns.rcode_name");
                    event.remove("m365_defender.event.dns.answers");
                    event.remove("m365_defender.event.dns.ttls");
                    event.remove("m365_defender.event.dns.header_flags");
                    event.remove("m365_defender.event.container_name");
                    event.remove("m365_defender.event.container_id");
                    event.remove("m365_defender.event.container_image_name");
                    event.remove("m365_defender.event.kubernetes_resource");
                    event.remove("m365_defender.event.kubernetes_namespace");
                    event.remove("m365_defender.event.process_name");
                    event.remove("m365_defender.event.parent_process_name");
                    event.remove("m365_defender.event.parent_process_id");
                    event.remove("m365_defender.event.process_current_working_directory");
                }
                event.remove("_tmp");
                // End nested pipeline: "pipeline_device"
            }

            let _cond = {
                event.has_value("m365_defender.event.category")
                    && (event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("email"))
                        || event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("urlclickevents"))
                        || event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("message")))
            };
            if _cond {
                // Begin nested pipeline: "pipeline_email"
                let _cond = {
                    !event.has_value("m365_defender.event.category")
                        || event.get_str("m365_defender.event.category") == Some("")
                };
                if _cond {
                    return Err(TransformError::ParseError {
                        path: "_fail".into(),
                        message: ("Event does not contain a valid category.").to_string(),
                    });
                }
                event.set("event.kind", json!("event"))?;
                let _cond = {
                    !(event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("urlclickevents")))
                };
                if _cond {
                    event.append("event.category", json!("email"))?;
                }
                let _cond = { event.has_value("json.properties.FileType") };
                if _cond {
                    event.append("event.category", json!("file"))?;
                }
                let _cond = {
                    !(event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("urlclickevents")))
                };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                let _cond = { !event.has_value("event.type") };
                if _cond {
                    event.append("event.type", json!("info"))?;
                }
                let _cond = {
                    event.has_value("json.properties.DetectionMethods")
                        && event
                            .get("json.properties.DetectionMethods")
                            .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.DetectionMethods") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.DetectionMethods",
                            "json.properties.DetectionMethods",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set("_ingest.on_failure_processor_tag", "json_detection_methods")?;
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
                let _cond = {
                    event.has_value("json.properties.ConfidenceLevel")
                        && event
                            .get("json.properties.ConfidenceLevel")
                            .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.ConfidenceLevel") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.ConfidenceLevel",
                            "json.properties.ConfidenceLevel",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set("_ingest.on_failure_processor_tag", "json_confidence_level")?;
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
                let _cond = {
                    event.has_value("json.properties.AdditionalFields")
                        && event
                            .get("json.properties.AdditionalFields")
                            .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.AdditionalFields") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.AdditionalFields",
                            "json.properties.AdditionalFields",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set("_ingest.on_failure_processor_tag", "json_additional_fields")?;
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
                // Painless script
                // Source: def isTruthy(def val) {\n   if (val == null) {\n     // Fast return if field is absent.\n     return null;\n   }\n   if (val instanceof Boolean) {\n     return val;\n   }\n   if (val instanceof Integer) {\n     if (val == 1) {\n       return true;\n     }\n     if (val == 0) {\n       return false;\n     }\n     return null;\n   }\n   if (val instanceof String) {\n     if (val == \"1\" || val == \"true\") {\n       return true;\n     }\n     if (val == \"0\" || val == \"false\") {\n       return false;\n     }\n     return null;\n   }\n   return null;\n }\n ctx.m365_defender.event.is_clicked_through = isTruthy(ctx.json?.properties?.IsClickedThrough);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def isTruthy(def val) {\n   if (val == null) {\n     // Fast return if field is absent.\n     return null;\n   }\n   if (val instanceof Boolean) {\n     return val;\n   }\n   if (val instanceof Integer) {\n     if (val == 1) {\n       return true;\n     }\n     if (val == 0) {\n       return false;\n     }\n     return null;\n   }\n   if (val instanceof String) {\n     if (val == \"1\" || val == \"true\") {\n       return true;\n     }\n     if (val == \"0\" || val == \"false\") {\n       return false;\n     }\n     return null;\n   }\n   return null;\n }\n ctx.m365_defender.event.is_clicked_through = isTruthy(ctx.json?.properties?.IsClickedThrough);\n"#
                    ),
                )?;
                let _cond = { event.get_str("json.properties.EmailClusterId") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.EmailClusterId") {
                            if let Some(val) = event.get("json.properties.EmailClusterId") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.properties.EmailClusterId".into(),
                                            message,
                                        }
                                    })?;
                                event.set("m365_defender.event.email.cluster_id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_email_cluster_id",
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
                let _cond = { event.get_str("json.properties.IPAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.IPAddress") {
                            if let Some(val) = event.get("json.properties.IPAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.IPAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.ip_address", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_ip_address")?;
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
                let _cond = { event.get_str("json.properties.FileSize") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.FileSize") {
                            if let Some(val) = event.get("json.properties.FileSize") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.FileSize".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.file.size", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_file_size")?;
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
                let _cond = { event.get_str("json.properties.SenderIPv4") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.SenderIPv4") {
                            if let Some(val) = event.get("json.properties.SenderIPv4") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.SenderIPv4".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.sender.ipv4", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_sender_ipv4")?;
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
                let _cond = { event.get_str("json.properties.SenderIPv6") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.SenderIPv6") {
                            if let Some(val) = event.get("json.properties.SenderIPv6") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.SenderIPv6".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.sender.ipv6", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_sender_ipv6")?;
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
                let _cond = { event.get_str("json.properties.AttachmentCount") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.AttachmentCount") {
                            if let Some(val) = event.get("json.properties.AttachmentCount") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.AttachmentCount".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.attachment_count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_attachment_count",
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
                let _cond = { event.get_str("json.properties.BulkComplaintLevel") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.BulkComplaintLevel") {
                            if let Some(val) = event.get("json.properties.BulkComplaintLevel") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.BulkComplaintLevel".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.bulk_complaint_level", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_bulk_complaint_level",
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
                let _cond = { event.get_str("json.properties.ReportId") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.ReportId") {
                            if let Some(val) = event.get("json.properties.ReportId") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.properties.ReportId".into(),
                                            message,
                                        }
                                    })?;
                                event.set("m365_defender.event.report_id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_json_properties_ReportId",
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
                let _cond = { event.get_str("json.properties.UrlCount") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.UrlCount") {
                            if let Some(val) = event.get("json.properties.UrlCount") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.UrlCount".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.url_count", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_url_count")?;
                        event.append(
                            "error.message",
                            json!(
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.IsExternalThread") {
                        if let Some(val) = event.get("json.properties.IsExternalThread") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.IsExternalThread".into(),
                                    message,
                                }
                            })?;
                            event.set("m365_defender.event.is_external_thread", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_IsExternalThread_to_boolean",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.IsOwnedThread") {
                        if let Some(val) = event.get("json.properties.IsOwnedThread") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.IsOwnedThread".into(),
                                    message,
                                }
                            })?;
                            event.set("m365_defender.event.is_owned_thread", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_IsOwnedThread_to_boolean",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.SenderEmailAddress") {
                        if let Some(input) = event.get_string("json.properties.SenderEmailAddress")
                        {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            } else {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.SenderEmailAddress".into(),
                                    message: "dissect pattern did not match".into(),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dissect_json_properties_SenderEmailAddress_70aca97e",
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
                if event.has_value("json.properties.SenderFromAddress") {
                    event.rename(
                        "json.properties.SenderFromAddress",
                        "m365_defender.event.sender.from_address",
                    )?;
                }
                if event.has_value("json.properties.NetworkMessageId") {
                    event.rename(
                        "json.properties.NetworkMessageId",
                        "m365_defender.event.network.message_id",
                    )?;
                }
                if event.has_value("json.properties.FileType") {
                    event.rename("json.properties.FileType", "m365_defender.event.file.type")?;
                }
                if event.has_value("json.properties.RecipientEmailAddress") {
                    event.rename(
                        "json.properties.RecipientEmailAddress",
                        "m365_defender.event.recipient.email_address",
                    )?;
                }
                if event.has_value("json.properties.ActionType") {
                    event.rename(
                        "json.properties.ActionType",
                        "m365_defender.event.action.type",
                    )?;
                }
                if event.has_value("json.properties.SHA256") {
                    event.rename("json.properties.SHA256", "m365_defender.event.sha256")?;
                }
                if event.has_value("json.properties.FileName") {
                    event.rename("json.properties.FileName", "m365_defender.event.file.name")?;
                }
                if event.has_value("json.properties.EmailDirection") {
                    event.rename(
                        "json.properties.EmailDirection",
                        "m365_defender.event.email.direction",
                    )?;
                }
                if event.has_value("json.properties.Subject") {
                    event.rename("json.properties.Subject", "m365_defender.event.subject")?;
                }
                if event.has_value("json.properties.DetectionMethods") {
                    event.rename(
                        "json.properties.DetectionMethods",
                        "m365_defender.event.detection.methods",
                    )?;
                }
                if event.has_value("json.properties.RecipientObjectId") {
                    event.rename(
                        "json.properties.RecipientObjectId",
                        "m365_defender.event.recipient.object_id",
                    )?;
                }
                if event.has_value("json.properties.SenderDisplayName") {
                    event.rename(
                        "json.properties.SenderDisplayName",
                        "m365_defender.event.sender.display_name",
                    )?;
                }
                if event.has_value("json.properties.SenderObjectId") {
                    event.rename(
                        "json.properties.SenderObjectId",
                        "m365_defender.event.sender.object_id",
                    )?;
                }
                if event.has_value("json.properties.ThreatNames") {
                    event.rename(
                        "json.properties.ThreatNames",
                        "m365_defender.event.threat.names",
                    )?;
                }
                if event.has_value("json.properties.ThreatTypes") {
                    event.rename(
                        "json.properties.ThreatTypes",
                        "m365_defender.event.threat.types",
                    )?;
                }
                if event.has_value("json.properties.ConfidenceLevel") {
                    event.rename(
                        "json.properties.ConfidenceLevel",
                        "m365_defender.event.confidence_level",
                    )?;
                }
                if event.has_value("json.properties.AuthenticationDetails") {
                    event.rename(
                        "json.properties.AuthenticationDetails",
                        "m365_defender.event.authentication_details",
                    )?;
                }
                if event.has_value("json.properties.AdditionalFields") {
                    event.rename(
                        "json.properties.AdditionalFields",
                        "m365_defender.event.additional_fields",
                    )?;
                }
                if event.has_value("json.properties.Connectors") {
                    event.rename(
                        "json.properties.Connectors",
                        "m365_defender.event.connectors",
                    )?;
                }
                if event.has_value("json.properties.DeliveryAction") {
                    event.rename(
                        "json.properties.DeliveryAction",
                        "m365_defender.event.delivery.action",
                    )?;
                }
                if event.has_value("json.properties.DeliveryLocation") {
                    event.rename(
                        "json.properties.DeliveryLocation",
                        "m365_defender.event.delivery.location",
                    )?;
                }
                if event.has_value("json.properties.EmailAction") {
                    event.rename(
                        "json.properties.EmailAction",
                        "m365_defender.event.email.action",
                    )?;
                }
                if event.has_value("json.properties.EmailActionPolicy") {
                    event.rename(
                        "json.properties.EmailActionPolicy",
                        "m365_defender.event.email.action_policy",
                    )?;
                }
                if event.has_value("json.properties.EmailActionPolicyGuid") {
                    event.rename(
                        "json.properties.EmailActionPolicyGuid",
                        "m365_defender.event.email.action_policy_guid",
                    )?;
                }
                if event.has_value("json.properties.EmailLanguage") {
                    event.rename(
                        "json.properties.EmailLanguage",
                        "m365_defender.event.email.language",
                    )?;
                }
                if event.has_value("json.properties.InternetMessageId") {
                    event.rename(
                        "json.properties.InternetMessageId",
                        "m365_defender.event.internet_message_id",
                    )?;
                }
                if event.has_value("json.properties.OrgLevelAction") {
                    event.rename(
                        "json.properties.OrgLevelAction",
                        "m365_defender.event.org_level.action",
                    )?;
                }
                if event.has_value("json.properties.OrgLevelPolicy") {
                    event.rename(
                        "json.properties.OrgLevelPolicy",
                        "m365_defender.event.org_level.policy",
                    )?;
                }
                if event.has_value("json.properties.SenderFromDomain") {
                    event.rename(
                        "json.properties.SenderFromDomain",
                        "m365_defender.event.sender.from_domain",
                    )?;
                }
                if event.has_value("json.properties.SenderMailFromAddress") {
                    event.rename(
                        "json.properties.SenderMailFromAddress",
                        "m365_defender.event.sender.mail_from_address",
                    )?;
                }
                if event.has_value("json.properties.SenderMailFromDomain") {
                    event.rename(
                        "json.properties.SenderMailFromDomain",
                        "m365_defender.event.sender.mail_from_domain",
                    )?;
                }
                if event.has_value("json.properties.UserLevelAction") {
                    event.rename(
                        "json.properties.UserLevelAction",
                        "m365_defender.event.user_level_action",
                    )?;
                }
                if event.has_value("json.properties.UserLevelPolicy") {
                    event.rename(
                        "json.properties.UserLevelPolicy",
                        "m365_defender.event.user_level_policy",
                    )?;
                }
                if event.has_value("json.properties.ActionResult") {
                    event.rename(
                        "json.properties.ActionResult",
                        "m365_defender.event.action.result",
                    )?;
                }
                if event.has_value("json.properties.ActionTrigger") {
                    event.rename(
                        "json.properties.ActionTrigger",
                        "m365_defender.event.action.trigger",
                    )?;
                }
                if event.has_value("json.properties.Action") {
                    event.rename("json.properties.Action", "m365_defender.event.action.value")?;
                }
                if event.has_value("json.properties.Url") {
                    event.rename("json.properties.Url", "m365_defender.event.url")?;
                }
                if event.has_value("json.properties.UrlDomain") {
                    event.rename(
                        "json.properties.UrlDomain",
                        "m365_defender.event.url_domain",
                    )?;
                }
                if event.has_value("json.properties.UrlLocation") {
                    event.rename(
                        "json.properties.UrlLocation",
                        "m365_defender.event.url_location",
                    )?;
                }
                if event.has_value("json.properties.AccountUpn") {
                    event.rename(
                        "json.properties.AccountUpn",
                        "m365_defender.event.account.upn",
                    )?;
                }
                if event.has_value("json.properties.UrlChain") {
                    event.rename("json.properties.UrlChain", "m365_defender.event.url_chain")?;
                }
                if event.has_value("json.properties.Workload") {
                    event.rename("json.properties.Workload", "m365_defender.event.workload")?;
                }
                if event.has_value("json.properties.GroupId") {
                    event.rename("json.properties.GroupId", "m365_defender.event.group_id")?;
                }
                if event.has_value("json.properties.GroupName") {
                    event.rename(
                        "json.properties.GroupName",
                        "m365_defender.event.group_name",
                    )?;
                }
                if event.has_value("json.properties.LastEditedTime") {
                    event.rename(
                        "json.properties.LastEditedTime",
                        "m365_defender.event.last_edited_time",
                    )?;
                }
                if event.has_value("json.properties.MessageFormatSubtype") {
                    event.rename(
                        "json.properties.MessageFormatSubtype",
                        "m365_defender.event.message_format_subtype",
                    )?;
                }
                if event.has_value("json.properties.MessageFormatType") {
                    event.rename(
                        "json.properties.MessageFormatType",
                        "m365_defender.event.message_format_type",
                    )?;
                }
                if event.has_value("json.properties.MessageId") {
                    event.rename(
                        "json.properties.MessageId",
                        "m365_defender.event.message_id",
                    )?;
                }
                if event.has_value("json.properties.MessageSubject") {
                    event.rename(
                        "json.properties.MessageSubject",
                        "m365_defender.event.message_subject",
                    )?;
                }
                if event.has_value("json.properties.MessageVersion") {
                    event.rename(
                        "json.properties.MessageVersion",
                        "m365_defender.event.message_version",
                    )?;
                }
                if event.has_value("json.properties.ParentMessageId") {
                    event.rename(
                        "json.properties.ParentMessageId",
                        "m365_defender.event.parent_message_id",
                    )?;
                }
                if event.has_value("json.properties.SenderType") {
                    event.rename(
                        "json.properties.SenderType",
                        "m365_defender.event.sender_type",
                    )?;
                }
                if event.has_value("json.properties.TeamsMessageId") {
                    event.rename(
                        "json.properties.TeamsMessageId",
                        "m365_defender.event.teams_message_id",
                    )?;
                }
                if event.has_value("json.properties.ThreadId") {
                    event.rename("json.properties.ThreadId", "m365_defender.event.thread_id")?;
                }
                if event.has_value("json.properties.ThreadSubtype") {
                    event.rename(
                        "json.properties.ThreadSubtype",
                        "m365_defender.event.thread_subtype",
                    )?;
                }
                if event.has_value("json.properties.SenderEmailAddress") {
                    event.rename(
                        "json.properties.SenderEmailAddress",
                        "m365_defender.event.sender_email_address",
                    )?;
                }
                if event.has_value("json.properties.LatestDeliveryLocation") {
                    event.rename(
                        "json.properties.LatestDeliveryLocation",
                        "m365_defender.event.latest_delivery_location",
                    )?;
                }
                if event.has_value("json.properties.SafetyTip") {
                    event.rename(
                        "json.properties.SafetyTip",
                        "m365_defender.event.safety_tip",
                    )?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.file.type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.extension", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.sha256")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.hash.sha256", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.file.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.file.size")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.size", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.action.type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                if event.has_value("event.action") {
                    gsub_field(
                        event,
                        "event.action",
                        "event.action",
                        cached_regex!(" "),
                        "-",
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.sender.from_address") };
                if _cond {
                    event.append_unique(
                        "email.from.address",
                        json!(
                            event
                                .get("m365_defender.event.sender.from_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.recipient.email_address") };
                if _cond {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("m365_defender.event.recipient.email_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.network.message_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("email.local_id", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.email.direction")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("email.direction", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.subject")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("email.subject", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.internet_message_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("email.message_id", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.message_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("email.message_id", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.message_subject")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("email.subject", v)?;
                }
                let _cond = { event.has_value("m365_defender.event.sender_email_address") };
                if _cond {
                    event.append_unique(
                        "email.from.address",
                        json!(
                            event
                                .get("m365_defender.event.sender_email_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.sender.ipv4") };
                if _cond {
                    event.append_unique(
                        "source.ip",
                        json!(
                            event
                                .get("m365_defender.event.sender.ipv4")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.sender.ipv6") };
                if _cond {
                    event.append_unique(
                        "source.ip",
                        json!(
                            event
                                .get("m365_defender.event.sender.ipv6")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.ip_address")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.ip", v)?;
                }
                let _cond = { event.has_value("m365_defender.event.url") };
                if _cond {
                    uri_parts(event, "m365_defender.event.url", "url", true, false)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.group_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.group.name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.group_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.group.id", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.sender_email_address")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
                let _cond = { event.has_value("m365_defender.event.sender.from_address") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("m365_defender.event.sender.from_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.recipient.email_address") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("m365_defender.event.recipient.email_address")
                                .map_or_else(String::new, template_to_string)
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
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.sender.ipv4") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("m365_defender.event.sender.ipv4")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.sender.ipv6") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("m365_defender.event.sender.ipv6")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.ip_address") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("m365_defender.event.ip_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.sender.from_domain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("m365_defender.event.sender.from_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.sender.mail_from_domain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("m365_defender.event.sender.mail_from_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.url_domain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("m365_defender.event.url_domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.sender_email_address") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("m365_defender.event.sender_email_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
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
                    event.remove("m365_defender.event.sender.from_address");
                    event.remove("m365_defender.event.network.message_id");
                    event.remove("m365_defender.event.internet_message_id");
                    event.remove("m365_defender.event.recipient.email_address");
                    event.remove("m365_defender.event.file.type");
                    event.remove("m365_defender.event.sha256");
                    event.remove("m365_defender.event.file.name");
                    event.remove("m365_defender.event.file.size");
                    event.remove("m365_defender.event.email.direction");
                    event.remove("m365_defender.event.subject");
                    event.remove("m365_defender.event.sender.ipv4");
                    event.remove("m365_defender.event.sender.ipv6");
                    event.remove("m365_defender.event.ip_address");
                    event.remove("m365_defender.event.action.type");
                    event.remove("m365_defender.event.group_id");
                    event.remove("m365_defender.event.group_name");
                    event.remove("m365_defender.event.message_id");
                    event.remove("m365_defender.event.message_subject");
                    event.remove("m365_defender.event.sender_email_address");
                }
                // End nested pipeline: "pipeline_email"
            }

            let _cond = {
                event.has_value("m365_defender.event.category")
                    && (event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("identity"))
                        || event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("cloudappevents"))
                        || event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("cloudauditevents"))
                        || event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| {
                                s.to_lowercase().contains("cloudstorageaggregatedevents")
                            }))
            };
            if _cond {
                // Begin nested pipeline: "pipeline_app_and_identity"
                let _cond = {
                    !event.has_value("m365_defender.event.category")
                        || event.get_str("m365_defender.event.category") == Some("")
                };
                if _cond {
                    return Err(TransformError::ParseError {
                        path: "_fail".into(),
                        message: ("Event does not contain a valid category.").to_string(),
                    });
                }
                let _cond = {
                    !(event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase() == "advancedhunting-identityinfo"))
                };
                if _cond {
                    event.set("event.kind", json!("event"))?;
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("identityinfo"))
                };
                if _cond {
                    event.set("event.kind", json!("asset"))?;
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("identitylogonevents"))
                };
                if _cond {
                    event.append("event.category", json!("authentication"))?;
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("identityinfo"))
                };
                if _cond {
                    event.append("event.category", json!("iam"))?;
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("cloudauditevents"))
                };
                if _cond {
                    event.append("event.category", json!("configuration"))?;
                }
                event.append("event.type", json!("info"))?;
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("identityinfo"))
                };
                if _cond {
                    event.append("event.type", json!("user"))?;
                }
                let _cond = {
                    event
                        .get("json.properties.ActivityObjects")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.ActivityObjects") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.ActivityObjects",
                            "json.properties.ActivityObjects",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_properties_ActivityObjects",
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
                let _cond = {
                    event
                        .get("json.properties.RawEventData")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.RawEventData") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.RawEventData",
                            "json.properties.RawEventData",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_properties_RawEventData",
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
                let _cond = {
                    event
                        .get("json.properties.AdditionalFields")
                        .is_some_and(|v| v.is_string())
                        && event.get_str("json.properties.AdditionalFields") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        parse_json_field(
                            event,
                            "json.properties.AdditionalFields",
                            "json.properties.AdditionalFields",
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "json")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "json_properties_AdditionalFields",
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
                let _cond = { event.get_str("json.properties.DestinationIPAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.DestinationIPAddress") {
                            if let Some(val) = event.get("json.properties.DestinationIPAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.DestinationIPAddress".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("m365_defender.event.destination.ip_address", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_json_properties_DestinationIPAddress",
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
                let _cond = { event.get_str("json.properties.DestinationPort") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.DestinationPort") {
                            if let Some(val) = event.get("json.properties.DestinationPort") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.DestinationPort".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.destination.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_json_properties_DestinationPort",
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
                let _cond = { event.get_str("json.properties.IPAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.IPAddress") {
                            if let Some(val) = event.get("json.properties.IPAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.IPAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.ip_address", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_json_properties_IPAddress",
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
                let _cond = { event.get_str("json.properties.ReportId") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.ReportId") {
                            if let Some(val) = event.get("json.properties.ReportId") {
                                let converted =
                                    convert_value(val, "string").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "json.properties.ReportId".into(),
                                            message,
                                        }
                                    })?;
                                event.set("m365_defender.event.report_id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_json_properties_ReportId",
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
                let _cond = { event.get_str("json.properties.AppInstanceId") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.AppInstanceId") {
                            if let Some(val) = event.get("json.properties.AppInstanceId") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.AppInstanceId".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.app_instance_id", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_json_properties_AppInstanceId",
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
                let _cond = { event.get_str("json.properties.Port") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.Port") {
                            if let Some(val) = event.get("json.properties.Port") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.Port".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.port", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_json_properties_Port",
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
                let _cond = { event.get_str("json.properties.ApplicationId") != Some("") };
                if _cond {
                    if event.has_value("json.properties.ApplicationId") {
                        if let Some(val) = event.get("json.properties.ApplicationId") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.ApplicationId".into(),
                                    message,
                                }
                            })?;
                            event.set("m365_defender.event.application_id", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("json.properties.DataAggregationStartTime")
                        && event.get_str("json.properties.DataAggregationStartTime") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.properties.DataAggregationStartTime")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event.set(
                                    "m365_defender.event.data_aggregation_start_time",
                                    parsed,
                                )?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.properties.DataAggregationStartTime".into(),
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
                            "date_DataAggregationStartTime",
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
                let _cond = {
                    event.has_value("json.properties.DataAggregationEndTime")
                        && event.get_str("json.properties.DataAggregationEndTime") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.properties.DataAggregationEndTime")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event
                                    .set("m365_defender.event.data_aggregation_end_time", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.properties.DataAggregationEndTime".into(),
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
                            "date_DataAggregationEndTime",
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
                let _cond = { event.get_str("json.properties.IpAddress") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("json.properties.IpAddress") {
                            if let Some(val) = event.get("json.properties.IpAddress") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "json.properties.IpAddress".into(),
                                        message,
                                    }
                                })?;
                                event.set("m365_defender.event.storage_ip_address", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_IpAddress_to_ip",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.OperationsCount") {
                        if let Some(val) = event.get("json.properties.OperationsCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.OperationsCount".into(),
                                    message,
                                }
                            })?;
                            event.set("m365_defender.event.operations_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_OperationsCount_to_long",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.SuccessfulOperationsCount") {
                        if let Some(val) = event.get("json.properties.SuccessfulOperationsCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.SuccessfulOperationsCount".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "m365_defender.event.successful_operations_count",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_SuccessfulOperationsCount_to_long",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.IsKnownSuspiciousIp") {
                        if let Some(val) = event.get("json.properties.IsKnownSuspiciousIp") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.IsKnownSuspiciousIp".into(),
                                    message,
                                }
                            })?;
                            event.set("m365_defender.event.is_known_suspicious_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_IsKnownSuspiciousIp_to_boolean",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.IsPrivateIp") {
                        if let Some(val) = event.get("json.properties.IsPrivateIp") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.IsPrivateIp".into(),
                                    message,
                                }
                            })?;
                            event.set("m365_defender.event.is_private_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_IsPrivateIp_to_boolean",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.FailedOperationsCount") {
                        if let Some(val) = event.get("json.properties.FailedOperationsCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.FailedOperationsCount".into(),
                                    message,
                                }
                            })?;
                            event.set("m365_defender.event.failed_operations_count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_FailedOperationsCount_to_long",
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
                let _cond = {
                    event.has_value("json.properties.FirstEventTimestamp")
                        && event.get_str("json.properties.FirstEventTimestamp") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.properties.FirstEventTimestamp")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event
                                    .set("m365_defender.event.first_event_timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.properties.FirstEventTimestamp".into(),
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
                            "date_FirstEventTimestamp",
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
                let _cond = {
                    event.has_value("json.properties.LastEventTimestamp")
                        && event.get_str("json.properties.LastEventTimestamp") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("json.properties.LastEventTimestamp")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("m365_defender.event.last_event_timestamp", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "json.properties.LastEventTimestamp".into(),
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
                            "date_LastEventTimestamp",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.TotalResponseLength") {
                        if let Some(val) = event.get("json.properties.TotalResponseLength") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.TotalResponseLength".into(),
                                    message,
                                }
                            })?;
                            event.set("m365_defender.event.total_response_length", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_TotalResponseLength_to_long",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.SuccessfulReadOperations") {
                        if let Some(val) = event.get("json.properties.SuccessfulReadOperations") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.SuccessfulReadOperations".into(),
                                    message,
                                }
                            })?;
                            event
                                .set("m365_defender.event.successful_read_operations", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_SuccessfulReadOperations_to_long",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.DistinctGetOperations") {
                        if let Some(val) = event.get("json.properties.DistinctGetOperations") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.DistinctGetOperations".into(),
                                    message,
                                }
                            })?;
                            event.set("m365_defender.event.distinct_get_operations", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_DistinctGetOperations_to_long",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.AnonymousSuccessfulOperations") {
                        if let Some(val) =
                            event.get("json.properties.AnonymousSuccessfulOperations")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.AnonymousSuccessfulOperations".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "m365_defender.event.anonymous_successful_operations",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_AnonymousSuccessfulOperations_to_long",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.HasAnonymousResourceNotFoundFailures") {
                        if let Some(val) =
                            event.get("json.properties.HasAnonymousResourceNotFoundFailures")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.HasAnonymousResourceNotFoundFailures"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "m365_defender.event.has_anonymous_resource_not_found_failures",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_HasAnonymousResourceNotFoundFailures_to_boolean",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.IsTorExitNode") {
                        if let Some(val) = event.get("json.properties.IsTorExitNode") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.IsTorExitNode".into(),
                                    message,
                                }
                            })?;
                            event.set("m365_defender.event.is_tor_exit_node", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_IsTorExitNode_to_boolean",
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
                // Painless script
                // Source: def isTruthy(def val) {\n   if (val == null) {\n     // Fast return if field is absent.\n     return null;\n   }\n   if (val instanceof Boolean) {\n     return val;\n   }\n   if (val instanceof Integer) {\n     if (val == 1) {\n       return true;\n     }\n     if (val == 0) {\n       return false;\n     }\n     return null;\n   }\n   if (val instanceof String) {\n     if (val == \"1\" || val == \"true\") {\n       return true;\n     }\n     if (val == \"0\" || val == \"false\") {\n       return false;\n     }\n     return null;\n   }\n   return null;\n }\n ctx.m365_defender.event.is_admin_operation = isTruthy(ctx.json?.properties?.IsAdminOperation);\n ctx.m365_defender.event.is_anonymous_proxy = isTruthy(ctx.json?.properties?.IsAnonymousProxy);\n ctx.m365_defender.event.is_external_user = isTruthy(ctx.json?.properties?.IsExternalUser);\n ctx.m365_defender.event.is_impersonated = isTruthy(ctx.json?.properties?.IsImpersonated);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def isTruthy(def val) {\n   if (val == null) {\n     // Fast return if field is absent.\n     return null;\n   }\n   if (val instanceof Boolean) {\n     return val;\n   }\n   if (val instanceof Integer) {\n     if (val == 1) {\n       return true;\n     }\n     if (val == 0) {\n       return false;\n     }\n     return null;\n   }\n   if (val instanceof String) {\n     if (val == \"1\" || val == \"true\") {\n       return true;\n     }\n     if (val == \"0\" || val == \"false\") {\n       return false;\n     }\n     return null;\n   }\n   return null;\n }\n ctx.m365_defender.event.is_admin_operation = isTruthy(ctx.json?.properties?.IsAdminOperation);\n ctx.m365_defender.event.is_anonymous_proxy = isTruthy(ctx.json?.properties?.IsAnonymousProxy);\n ctx.m365_defender.event.is_external_user = isTruthy(ctx.json?.properties?.IsExternalUser);\n ctx.m365_defender.event.is_impersonated = isTruthy(ctx.json?.properties?.IsImpersonated);\n"#
                    ),
                )?;
                let _cond = {
                    !(event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase() == "advancedhunting-identityinfo"))
                };
                if _cond {
                    if event.has_value("json.properties.City") {
                        event.rename("json.properties.City", "m365_defender.event.city")?;
                    }
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("identityinfo"))
                };
                if _cond {
                    if event.has_value("json.properties.City") {
                        event.rename("json.properties.City", "m365_defender.event.account.city")?;
                    }
                }
                if event.has_value("json.properties.AccountName") {
                    event.rename(
                        "json.properties.AccountName",
                        "m365_defender.event.account.name",
                    )?;
                }
                if event.has_value("json.properties.IsAccountEnabled") {
                    event.rename(
                        "json.properties.IsAccountEnabled",
                        "m365_defender.event.account.is_enabled",
                    )?;
                }
                if event.has_value("json.properties.AccountSid") {
                    event.rename(
                        "json.properties.AccountSid",
                        "m365_defender.event.account.sid",
                    )?;
                }
                if event.has_value("json.properties.AccountDomain") {
                    event.rename(
                        "json.properties.AccountDomain",
                        "m365_defender.event.account.domain",
                    )?;
                }
                if event.has_value("json.properties.DeviceType") {
                    event.rename(
                        "json.properties.DeviceType",
                        "m365_defender.event.device.type",
                    )?;
                }
                if event.has_value("json.properties.ActionType") {
                    event.rename(
                        "json.properties.ActionType",
                        "m365_defender.event.action.type",
                    )?;
                }
                if event.has_value("json.properties.OSPlatform") {
                    event.rename(
                        "json.properties.OSPlatform",
                        "m365_defender.event.os.platform",
                    )?;
                }
                if event.has_value("json.properties.DeviceName") {
                    event.rename(
                        "json.properties.DeviceName",
                        "m365_defender.event.device.name",
                    )?;
                }
                if event.has_value("json.properties.TargetDeviceName") {
                    event.rename(
                        "json.properties.TargetDeviceName",
                        "m365_defender.event.target.device_name",
                    )?;
                }
                if event.has_value("json.properties.Isp") {
                    event.rename("json.properties.Isp", "m365_defender.event.isp")?;
                }
                if event.has_value("json.properties.ISP") {
                    event.rename("json.properties.ISP", "m365_defender.event.isp")?;
                }
                if event.has_value("json.properties.AdditionalFields") {
                    event.rename(
                        "json.properties.AdditionalFields",
                        "m365_defender.event.additional_fields",
                    )?;
                }
                if event.has_value("json.properties.AccountObjectId") {
                    event.rename(
                        "json.properties.AccountObjectId",
                        "m365_defender.event.account.object_id",
                    )?;
                }
                if event.has_value("json.properties.AccountDisplayName") {
                    event.rename(
                        "json.properties.AccountDisplayName",
                        "m365_defender.event.account.display_name",
                    )?;
                }
                if event.has_value("json.properties.UserAgent") {
                    event.rename(
                        "json.properties.UserAgent",
                        "m365_defender.event.user_agent",
                    )?;
                }
                let _cond = {
                    !(event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase() == "advancedhunting-identityinfo"))
                };
                if _cond {
                    if event.has_value("json.properties.Country") {
                        event.rename("json.properties.Country", "m365_defender.event.country")?;
                    }
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase() == "advancedhunting-identityinfo")
                };
                if _cond {
                    if event.has_value("json.properties.Country") {
                        event.rename(
                            "json.properties.Country",
                            "m365_defender.event.account.country",
                        )?;
                    }
                }
                if event.has_value("json.properties.CountryCode") {
                    event.rename(
                        "json.properties.CountryCode",
                        "m365_defender.event.country_code",
                    )?;
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase() == "advancedhunting-identityinfo")
                };
                if _cond {
                    if event.has_value("json.properties.State") {
                        event
                            .rename("json.properties.State", "m365_defender.event.account.state")?;
                    }
                }
                if event.has_value("json.properties.Protocol") {
                    event.rename("json.properties.Protocol", "m365_defender.event.protocol")?;
                }
                if event.has_value("json.properties.AccountUpn") {
                    event.rename(
                        "json.properties.AccountUpn",
                        "m365_defender.event.account.upn",
                    )?;
                }
                if event.has_value("json.properties.Application") {
                    event.rename(
                        "json.properties.Application",
                        "m365_defender.event.application",
                    )?;
                }
                if event.has_value("json.properties.DestinationDeviceName") {
                    event.rename(
                        "json.properties.DestinationDeviceName",
                        "m365_defender.event.destination.device_name",
                    )?;
                }
                if event.has_value("json.properties.FailureReason") {
                    event.rename(
                        "json.properties.FailureReason",
                        "m365_defender.event.failure_reason",
                    )?;
                }
                if event.has_value("json.properties.Location") {
                    event.rename("json.properties.Location", "m365_defender.event.location")?;
                }
                if event.has_value("json.properties.AccountType") {
                    event.rename(
                        "json.properties.AccountType",
                        "m365_defender.event.account.type",
                    )?;
                }
                if event.has_value("json.properties.LogonType") {
                    event.rename(
                        "json.properties.LogonType",
                        "m365_defender.event.logon.type",
                    )?;
                }
                if event.has_value("json.properties.AccountId") {
                    event.rename(
                        "json.properties.AccountId",
                        "m365_defender.event.account.id",
                    )?;
                }
                if event.has_value("json.properties.TargetAccountDisplayName") {
                    event.rename(
                        "json.properties.TargetAccountDisplayName",
                        "m365_defender.event.target.account_display_name",
                    )?;
                }
                if event.has_value("json.properties.Query") {
                    event.rename("json.properties.Query", "m365_defender.event.query.value")?;
                }
                if event.has_value("json.properties.QueryTarget") {
                    event.rename(
                        "json.properties.QueryTarget",
                        "m365_defender.event.query.target",
                    )?;
                }
                if event.has_value("json.properties.QueryType") {
                    event.rename(
                        "json.properties.QueryType",
                        "m365_defender.event.query.type",
                    )?;
                }
                if event.has_value("json.properties.TargetAccountUpn") {
                    event.rename(
                        "json.properties.TargetAccountUpn",
                        "m365_defender.event.target.account_upn",
                    )?;
                }
                if event.has_value("json.properties.ActivityObjects") {
                    event.rename(
                        "json.properties.ActivityObjects",
                        "m365_defender.event.activity.objects",
                    )?;
                }
                if event.has_value("json.properties.ActivityType") {
                    event.rename(
                        "json.properties.ActivityType",
                        "m365_defender.event.activity.type",
                    )?;
                }
                if event.has_value("json.properties.IPCategory") {
                    event.rename(
                        "json.properties.IPCategory",
                        "m365_defender.event.ip_category",
                    )?;
                }
                if event.has_value("json.properties.IPTags") {
                    event.rename("json.properties.IPTags", "m365_defender.event.ip_tags")?;
                }
                if event.has_value("json.properties.ObjectId") {
                    event.rename("json.properties.ObjectId", "m365_defender.event.object.id")?;
                }
                if event.has_value("json.properties.ObjectName") {
                    event.rename(
                        "json.properties.ObjectName",
                        "m365_defender.event.object.name",
                    )?;
                }
                if event.has_value("json.properties.ObjectType") {
                    event.rename(
                        "json.properties.ObjectType",
                        "m365_defender.event.object.type",
                    )?;
                }
                if event.has_value("json.properties.RawEventData") {
                    event.rename(
                        "json.properties.RawEventData",
                        "m365_defender.event.raw_event_data",
                    )?;
                }
                if event.has_value("json.properties.UserAgentTags") {
                    event.rename(
                        "json.properties.UserAgentTags",
                        "m365_defender.event.user_agent_tags",
                    )?;
                }
                if event.has_value("json.properties.OnPremSid") {
                    event.rename(
                        "json.properties.OnPremSid",
                        "m365_defender.event.account.on_prem_sid",
                    )?;
                }
                if event.has_value("json.properties.SourceProvider") {
                    event.rename(
                        "json.properties.SourceProvider",
                        "m365_defender.event.source_provider",
                    )?;
                }
                if event.has_value("json.properties.SourceSystem") {
                    event.rename(
                        "json.properties.SourceSystem",
                        "m365_defender.event.source_system",
                    )?;
                }
                if event.has_value("json.properties.AssignedRoles") {
                    event.rename(
                        "json.properties.AssignedRoles",
                        "m365_defender.event.account.assigned_roles",
                    )?;
                }
                if event.has_value("json.properties.ChangeSource") {
                    event.rename(
                        "json.properties.ChangeSource",
                        "m365_defender.event.change_source",
                    )?;
                }
                if event.has_value("json.properties.EmailAddress") {
                    event.rename(
                        "json.properties.EmailAddress",
                        "m365_defender.event.account.email_address",
                    )?;
                }
                if event.has_value("json.properties.Address") {
                    event.rename(
                        "json.properties.Address",
                        "m365_defender.event.account.address",
                    )?;
                }
                if event.has_value("json.properties.Phone") {
                    event.rename("json.properties.Phone", "m365_defender.event.account.phone")?;
                }
                if event.has_value("json.properties.Manager") {
                    event.rename(
                        "json.properties.Manager",
                        "m365_defender.event.account.manager",
                    )?;
                }
                if event.has_value("json.properties.SipProxyAddress") {
                    event.rename(
                        "json.properties.SipProxyAddress",
                        "m365_defender.event.account.sip_proxy_address",
                    )?;
                }
                if event.has_value("json.properties.CreatedDateTime") {
                    event.rename(
                        "json.properties.CreatedDateTime",
                        "m365_defender.event.account.created",
                    )?;
                }
                if event.has_value("json.properties.JobTitle") {
                    event.rename(
                        "json.properties.JobTitle",
                        "m365_defender.event.account.job_title",
                    )?;
                }
                if event.has_value("json.properties.Department") {
                    event.rename(
                        "json.properties.Department",
                        "m365_defender.event.account.department",
                    )?;
                }
                if event.has_value("json.properties.EmployeeId") {
                    event.rename(
                        "json.properties.EmployeeId",
                        "m365_defender.event.account.employee_id",
                    )?;
                }
                if event.has_value("json.properties.Surname") {
                    event.rename(
                        "json.properties.Surname",
                        "m365_defender.event.account.surname",
                    )?;
                }
                if event.has_value("json.properties.GivenName") {
                    event.rename(
                        "json.properties.GivenName",
                        "m365_defender.event.account.given_name",
                    )?;
                }
                if event.has_value("json.properties.Type") {
                    event.rename("json.properties.Type", "m365_defender.event.type")?;
                }
                if event.has_value("json.properties.DistinguishedName") {
                    event.rename(
                        "json.properties.DistinguishedName",
                        "m365_defender.event.account.distinguished_name",
                    )?;
                }
                if event.has_value("json.properties.CloudSid") {
                    event.rename(
                        "json.properties.CloudSid",
                        "m365_defender.event.account.cloud_sid",
                    )?;
                }
                if event.has_value("json.properties.Tags") {
                    event.rename("json.properties.Tags", "m365_defender.event.account.tags")?;
                }
                if event.has_value("json.properties.BlastRadius") {
                    event.rename(
                        "json.properties.BlastRadius",
                        "m365_defender.event.account.blast_radius",
                    )?;
                }
                if event.has_value("json.properties.OtherMailAddresses") {
                    event.rename(
                        "json.properties.OtherMailAddresses",
                        "m365_defender.event.account.other_mail_addresses",
                    )?;
                }
                if event.has_value("json.properties.CompanyName") {
                    event.rename(
                        "json.properties.CompanyName",
                        "m365_defender.event.account.company_name",
                    )?;
                }
                if event.has_value("json.properties.DeletedDateTime") {
                    event.rename(
                        "json.properties.DeletedDateTime",
                        "m365_defender.event.account.deleted_date_time",
                    )?;
                }
                if event.has_value("json.properties.CriticalityLevel") {
                    event.rename(
                        "json.properties.CriticalityLevel",
                        "m365_defender.event.account.criticality_level",
                    )?;
                }
                if event.has_value("json.properties.RiskLevel") {
                    event.rename(
                        "json.properties.RiskLevel",
                        "m365_defender.event.account.risk_level",
                    )?;
                }
                if event.has_value("json.properties.RiskLevelDetails") {
                    event.rename(
                        "json.properties.RiskLevelDetails",
                        "m365_defender.event.account.risk_level_details",
                    )?;
                }
                if event.has_value("json.properties.DataSource") {
                    event.rename(
                        "json.properties.DataSource",
                        "m365_defender.event.data_source",
                    )?;
                }
                if event.has_value("json.properties.OperationName") {
                    event.rename(
                        "json.properties.OperationName",
                        "m365_defender.event.properties_operation_name",
                    )?;
                }
                if event.has_value("json.properties.ResourceId") {
                    event.rename(
                        "json.properties.ResourceId",
                        "m365_defender.event.resource_id",
                    )?;
                }
                if event.has_value("json.properties.ContainerId") {
                    event.rename(
                        "json.properties.ContainerId",
                        "m365_defender.event.container_id",
                    )?;
                }
                if event.has_value("json.properties.ContainerImageName") {
                    event.rename(
                        "json.properties.ContainerImageName",
                        "m365_defender.event.container_image_name",
                    )?;
                }
                if event.has_value("json.properties.ContainerName") {
                    event.rename(
                        "json.properties.ContainerName",
                        "m365_defender.event.container_name",
                    )?;
                }
                if event.has_value("json.properties.KubernetesNamespace") {
                    event.rename(
                        "json.properties.KubernetesNamespace",
                        "m365_defender.event.kubernetes_namespace",
                    )?;
                }
                if event.has_value("json.properties.KubernetesPodName") {
                    event.rename(
                        "json.properties.KubernetesPodName",
                        "m365_defender.event.kubernetes_pod_name",
                    )?;
                }
                if event.has_value("json.properties.KubernetesResource") {
                    event.rename(
                        "json.properties.KubernetesResource",
                        "m365_defender.event.kubernetes_resource",
                    )?;
                }
                if event.has_value("json.properties.ParentProcessId") {
                    event.rename(
                        "json.properties.ParentProcessId",
                        "m365_defender.event.parent_process_id",
                    )?;
                }
                if event.has_value("json.properties.ParentProcessName") {
                    event.rename(
                        "json.properties.ParentProcessName",
                        "m365_defender.event.parent_process_name",
                    )?;
                }
                if event.has_value("json.properties.ProcessCurrentWorkingDirectory") {
                    event.rename(
                        "json.properties.ProcessCurrentWorkingDirectory",
                        "m365_defender.event.process_current_working_directory",
                    )?;
                }
                if event.has_value("json.properties.ProcessName") {
                    event.rename(
                        "json.properties.ProcessName",
                        "m365_defender.event.process_name",
                    )?;
                }
                if event.has_value("json.properties.DataSources") {
                    event.rename(
                        "json.properties.DataSources",
                        "m365_defender.event.data_sources",
                    )?;
                }
                if event.has_value("json.properties.ResourceGroup") {
                    event.rename(
                        "json.properties.ResourceGroup",
                        "m365_defender.event.resource_group",
                    )?;
                }
                if event.has_value("json.properties.StorageAccount") {
                    event.rename(
                        "json.properties.StorageAccount",
                        "m365_defender.event.storage_account",
                    )?;
                }
                if event.has_value("json.properties.StorageContainer") {
                    event.rename(
                        "json.properties.StorageContainer",
                        "m365_defender.event.storage_container",
                    )?;
                }
                if event.has_value("json.properties.StorageFileShare") {
                    event.rename(
                        "json.properties.StorageFileShare",
                        "m365_defender.event.storage_file_share",
                    )?;
                }
                if event.has_value("json.properties.ServiceType") {
                    event.rename(
                        "json.properties.ServiceType",
                        "m365_defender.event.service_type",
                    )?;
                }
                if event.has_value("json.properties.UserAgentHeader") {
                    event.rename(
                        "json.properties.UserAgentHeader",
                        "m365_defender.event.user_agent_header",
                    )?;
                }
                if event.has_value("json.properties.OperationNamesList") {
                    event.rename(
                        "json.properties.OperationNamesList",
                        "m365_defender.event.operation_names_list",
                    )?;
                }
                if event.has_value("json.properties.AuthenticationType") {
                    event.rename(
                        "json.properties.AuthenticationType",
                        "m365_defender.event.authentication_type",
                    )?;
                }
                if event.has_value("json.properties.AccountTenantId") {
                    if let Some(val) = event.get("json.properties.AccountTenantId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.properties.AccountTenantId".into(),
                                message,
                            }
                        })?;
                        event.set("m365_defender.event.account_tenant_id", converted)?;
                    }
                }
                if event.has_value("json.properties.AccountApplicationId") {
                    event.rename(
                        "json.properties.AccountApplicationId",
                        "m365_defender.event.account_application_id",
                    )?;
                }
                if event.has_value("json.properties.SuspiciousUserAgentName") {
                    event.rename(
                        "json.properties.SuspiciousUserAgentName",
                        "m365_defender.event.suspicious_user_agent_name",
                    )?;
                }
                if event.has_value("json.properties.HashReputationMd5List") {
                    event.rename(
                        "json.properties.HashReputationMd5List",
                        "m365_defender.event.hash_reputation_md5_list",
                    )?;
                }
                if event.has_value("json.properties.SubscriptionId") {
                    event.rename(
                        "json.properties.SubscriptionId",
                        "m365_defender.event.subscription_id",
                    )?;
                }
                if event.has_value("json.properties.CountryName") {
                    event.rename(
                        "json.properties.CountryName",
                        "m365_defender.event.country_name",
                    )?;
                }
                if event.has_value("json.properties.CityName") {
                    event.rename("json.properties.CityName", "m365_defender.event.city_name")?;
                }
                if event.has_value("json.properties.ProvinceName") {
                    event.rename(
                        "json.properties.ProvinceName",
                        "m365_defender.event.province_name",
                    )?;
                }
                if event.has_value("json.properties.ClientSystemServiceName") {
                    event.rename(
                        "json.properties.ClientSystemServiceName",
                        "m365_defender.event.client_system_service_name",
                    )?;
                }
                if event.has_value("json.properties.ClientCloudPlatformName") {
                    event.rename(
                        "json.properties.ClientCloudPlatformName",
                        "m365_defender.event.client_cloud_platform_name",
                    )?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.kubernetes_namespace")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.namespace", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.kubernetes_resource")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("orchestrator.resource.name", v)?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.kubernetes_namespace")
                        || event.has_value("m365_defender.event.kubernetes_pod_name")
                        || event.has_value("m365_defender.event.kubernetes_resource")
                };
                if _cond {
                    event.set("orchestrator.type", json!("kubernetes"))?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.target.device_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.domain", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.destination.ip_address")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.ip", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.destination.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.port", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.device.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
                let _cond = { event.has_value("host.name") };
                if _cond {
                    map_strings(event, "host.name", "host.name", str::to_lowercase)?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.ip_address")
                        && event.get_str("m365_defender.event.ip_address") != Some("")
                };
                if _cond {
                    event.append(
                        "host.ip",
                        json!(
                            event
                                .get("m365_defender.event.ip_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.os.platform")
                        && event
                            .get_str("m365_defender.event.os.platform")
                            .is_some_and(|s| s.to_lowercase().contains("windows"))
                };
                if _cond {
                    event.set("host.os.type", json!("windows"))?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.os.platform")
                        && event
                            .get_str("m365_defender.event.os.platform")
                            .is_some_and(|s| s.to_lowercase().contains("linux"))
                };
                if _cond {
                    event.set("host.os.type", json!("linux"))?;
                }
                let _cond = {
                    event.has_value("m365_defender.event.os.platform")
                        && event
                            .get_str("m365_defender.event.os.platform")
                            .is_some_and(|s| s.to_lowercase().contains("macos"))
                };
                if _cond {
                    event.set("host.os.type", json!("macos"))?;
                }
                let _cond = {
                    event.has_value(
                        "m365_defender.event.additional_fields.SourceComputerOperatingSystemType",
                    )
                };
                if _cond {
                    if let Some(v) = event.get("m365_defender.event.additional_fields.SourceComputerOperatingSystemType").cloned() {
                event.set("host.os.type", v)?;
                }
                }
                if let Some(v) = event
                    .get("m365_defender.event.os.platform")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.os.name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.device.type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.type", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.container_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("container.id", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.container_image_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("container.image.name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.container_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("container.name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.parent_process_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.parent.pid", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.parent_process_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.parent.name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.process_current_working_directory")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.working_directory", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.process_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.action.type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                if event.has_value("event.action") {
                    gsub_field(
                        event,
                        "event.action",
                        "event.action",
                        cached_regex!(" "),
                        "-",
                    )?;
                }
                let _cond = {
                    (!event.has_value("m365_defender.event.failure_reason")
                        || event.get_str("m365_defender.event.failure_reason") == Some(""))
                        && event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| {
                                s.to_lowercase() == "advancedhunting-identitylogonevents"
                            })
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    (event.has_value("m365_defender.event.failure_reason")
                        && event.get_str("m365_defender.event.failure_reason") != Some(""))
                        && event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| {
                                s.to_lowercase() == "advancedhunting-identitylogonevents"
                            })
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.report_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.protocol")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("network.protocol", v)?;
                }
                if event.has_value("network.protocol") {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.account.domain")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.domain", v)?;
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("identitylogonevents"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.account.sid")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.id", v)?;
                    }
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("cloudappevents"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.account.id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("user.id", v)?;
                    }
                }
                let _cond = {
                    event
                        .get_str("m365_defender.event.category")
                        .is_some_and(|s| s.to_lowercase().contains("identityinfo"))
                };
                if _cond {
                    if let Some(v) = event.get("m365_defender.event.account.object_id").cloned() {
                        event.set("user.id", v)?;
                    }
                }
                if let Some(v) = event
                    .get("m365_defender.event.account.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                if let Some(v) = event
                    .get("m365_defender.event.raw_event_data.UserId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.name") {
                        event.set("user.name", v)?;
                    }
                }
                if let Some(v) = event
                    .get("m365_defender.event.account.display_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("user.name") {
                        event.set("user.name", v)?;
                    }
                }
                let _cond = {
                    event.has_value("m365_defender.event.account.display_name")
                        && !event.has_value("user.full_name")
                        && event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("identityinfo"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.account.display_name")
                        .cloned()
                    {
                        event.set("user.full_name", v)?;
                    }
                }
                let _cond = { event.has_value("m365_defender.event.account.email_address") };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.account.email_address")
                        .cloned()
                    {
                        event.set("user.email", v)?;
                    }
                }
                let _cond = {
                    event.has_value("m365_defender.event.user_agent")
                        && event.get_str("m365_defender.event.user_agent") != Some("")
                };
                if _cond {
                    if let Some(ua_str) = event.get_string("m365_defender.event.user_agent") {
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
                let _cond = {
                    event.has_value("m365_defender.event.user_agent_header")
                        && event.get_str("m365_defender.event.user_agent_header") != Some("")
                };
                if _cond {
                    if let Some(ua_str) = event.get_string("m365_defender.event.user_agent_header")
                    {
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
                let _cond = {
                    event.has_value("m365_defender.event.category")
                        && event
                            .get_str("m365_defender.event.category")
                            .is_some_and(|s| s.to_lowercase().contains("cloudauditevents"))
                };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.ip_address")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("client.ip", v)?;
                    }
                }
                let _cond = { event.get_bool("m365_defender.event.is_private_ip") == Some(false) };
                if _cond {
                    if let Some(v) = event
                        .get("m365_defender.event.storage_ip_address")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("client.ip", v)?;
                    }
                }
                let _cond = { event.has_value("client.ip") };
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
                let _cond = { event.has_value("user.id") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.account.display_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("m365_defender.event.account.display_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.target.account_display_name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("m365_defender.event.target.account_display_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.account.cloud_sid") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("m365_defender.event.account.cloud_sid")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.account.on_prem_sid") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("m365_defender.event.account.on_prem_sid")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.account.upn") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("m365_defender.event.account.upn")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.account.email_address") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("m365_defender.event.account.email_address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("destination.domain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("destination.domain")
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
                let _cond = { event.has_value("client.ip") };
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
                let _cond = { event.has_value("host.name") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.get("host.ip").is_some_and(|v| v.is_array()) };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        foreach_array(event, "host.ip", |event| {
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
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("user.domain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("user.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("m365_defender.event.account.name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("m365_defender.event.account.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
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
                    event.remove("m365_defender.event.target.device_name");
                    event.remove("m365_defender.event.destination.ip_address");
                    event.remove("m365_defender.event.destination.port");
                    event.remove("m365_defender.event.device.name");
                    event.remove("m365_defender.event.ip_address");
                    event.remove("m365_defender.event.os.platform");
                    event.remove("m365_defender.event.device.type");
                    event.remove("m365_defender.event.protocol");
                    event.remove("m365_defender.event.account.domain");
                    event.remove("m365_defender.event.account.email_address");
                    event.remove("m365_defender.event.account.sid");
                    event.remove("m365_defender.event.account.id");
                    event.remove("m365_defender.event.account.display_name");
                    event.remove("m365_defender.event.account.name");
                    event.remove("m365_defender.event.action.type");
                    event.remove("m365_defender.event.report_id");
                    event.remove("m365_defender.event.container_id");
                    event.remove("m365_defender.event.container_image_name");
                    event.remove("m365_defender.event.container_name");
                    event.remove("m365_defender.event.kubernetes_namespace");
                    event.remove("m365_defender.event.kubernetes_resource");
                    event.remove("m365_defender.event.parent_process_id");
                    event.remove("m365_defender.event.parent_process_name");
                    event.remove("m365_defender.event.process_current_working_directory");
                    event.remove("m365_defender.event.process_name");
                }
                // End nested pipeline: "pipeline_app_and_identity"
            }

            let _cond = {
                event.has_value("process.pid")
                    && event.has_value("process.start")
                    && event.has_value("m365_defender.event.device.id")
            };
            if _cond {
                event.set(
                    "process.entity_id",
                    json!(format!(
                        "{}|{}|{}",
                        event
                            .get("process.pid")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("process.start")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("m365_defender.event.device.id")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("process.entity_id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("process.entity_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = {
                event.has_value("process.parent.pid")
                    && event.has_value("process.parent.start")
                    && event.has_value("m365_defender.event.device.id")
            };
            if _cond {
                event.set(
                    "process.parent.entity_id",
                    json!(format!(
                        "{}|{}|{}",
                        event
                            .get("process.parent.pid")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("process.parent.start")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("m365_defender.event.device.id")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("process.parent.entity_id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set(
                        "process.parent.entity_id",
                        json!(fingerprint_default(&values)),
                    )?;
                }
            }

            let _cond = { !event.has_value("process.name") };
            if _cond {
                if let Some(v) = event
                    .get("process.executable")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.name", v)?;
                }
            }

            let _cond = { !event.has_value("process.name") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.process = ctx.process ?: [:];\nctx.process.name = ctx.process.name ?: [];\n// Normalize process.name to a list\ndef nameList = [];\nif (ctx.process.name != null) {\n  if (ctx.process.name instanceof String) {\n    nameList.add(ctx.process.name);\n  } else if (ctx.process.name instanceof List) {\n    nameList.addAll(ctx.process.name);\n  }\n}\n\n// Deduplication using HashSet\ndef currentNames = new HashSet();\ncurrentNames.addAll(nameList);\n// Handle process.command_line (string or list)\nif (ctx.process.command_line != null) {\n  // Convert string to list for unified handling\n  def cmdList = [];\n  if (ctx.process.command_line instanceof String) {\n    cmdList.add(ctx.process.command_line);\n  } else if (ctx.process.command_line instanceof List) {\n    cmdList.addAll(ctx.process.command_line);\n  }\n  for (cmd in cmdList) {\n    if (cmd != null && cmd.length() > 0) {\n      // Extract the first token\n      def parts = cmd.trim().splitOnToken(\" \");\n      if (parts.length > 0) {\n        def executable = parts[0];\n        // If executable is a path, take only the last part\n        if (executable.contains(\"/\")) {\n          def slashParts = executable.splitOnToken(\"/\");\n          executable = slashParts[slashParts.length - 1];\n        }\n        executable = /\\\"/.matcher(executable).replaceAll(\"\");\n        currentNames.add(executable);\n      }\n    }\n  }\n}\n// Update process.name with unique list\nif (currentNames != null && currentNames.size() == 1) {\n  ctx.process.name = currentNames.iterator().next();\n} else {\n  ctx.process.name = new ArrayList(currentNames);\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.process = ctx.process ?: [:];\nctx.process.name = ctx.process.name ?: [];\n// Normalize process.name to a list\ndef nameList = [];\nif (ctx.process.name != null) {\n  if (ctx.process.name instanceof String) {\n    nameList.add(ctx.process.name);\n  } else if (ctx.process.name instanceof List) {\n    nameList.addAll(ctx.process.name);\n  }\n}\n\n// Deduplication using HashSet\ndef currentNames = new HashSet();\ncurrentNames.addAll(nameList);\n// Handle process.command_line (string or list)\nif (ctx.process.command_line != null) {\n  // Convert string to list for unified handling\n  def cmdList = [];\n  if (ctx.process.command_line instanceof String) {\n    cmdList.add(ctx.process.command_line);\n  } else if (ctx.process.command_line instanceof List) {\n    cmdList.addAll(ctx.process.command_line);\n  }\n  for (cmd in cmdList) {\n    if (cmd != null && cmd.length() > 0) {\n      // Extract the first token\n      def parts = cmd.trim().splitOnToken(\" \");\n      if (parts.length > 0) {\n        def executable = parts[0];\n        // If executable is a path, take only the last part\n        if (executable.contains(\"/\")) {\n          def slashParts = executable.splitOnToken(\"/\");\n          executable = slashParts[slashParts.length - 1];\n        }\n        executable = /\\\"/.matcher(executable).replaceAll(\"\");\n        currentNames.add(executable);\n      }\n    }\n  }\n}\n// Update process.name with unique list\nif (currentNames != null && currentNames.size() == 1) {\n  ctx.process.name = currentNames.iterator().next();\n} else {\n  ctx.process.name = new ArrayList(currentNames);\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "set_process_name_from_command_line",
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

            let _cond = { event.has_value("m365_defender.event.device.id") };
            if _cond {
                if let Some(v) = event
                    .get("m365_defender.event.device.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("cloud.instance.id") {
                        event.set("cloud.instance.id", v)?;
                    }
                }
            }

            if let Some(v) = event
                .get("m365_defender.event.device.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("device.id", v)?;
            }

            if let Some(v) = event
                .get("m365_defender.event.application")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("application.name", v)?;
            }

            event.remove("json");

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
