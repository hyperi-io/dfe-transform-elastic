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

            event.set("event.kind", json!("alert"))?;

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

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.EndTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.ProcessingEndTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.StartTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.SystemAlertId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.TimeGenerated") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.AlertLink") {
                event.rename("json.AlertLink", "microsoft_sentinel.event.alert.link")?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.event.alert.link")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.url", v)?;
            }

            if event.has_value("json.AlertName") {
                event.rename("json.AlertName", "microsoft_sentinel.event.alert.name")?;
            }

            if event.has_value("json.AlertSeverity") {
                event.rename(
                    "json.AlertSeverity",
                    "microsoft_sentinel.event.alert.severity",
                )?;
            }

            if event.has_value("json.AlertType") {
                event.rename("json.AlertType", "microsoft_sentinel.event.alert.type")?;
            }

            if event.has_value("json.CompromisedEntity") {
                event.rename(
                    "json.CompromisedEntity",
                    "microsoft_sentinel.event.compromised_entity",
                )?;
            }

            if event.has_value("json.ConfidenceLevel") {
                event.rename(
                    "json.ConfidenceLevel",
                    "microsoft_sentinel.event.confidence.level",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.event.confidence.level")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.confidence", v)?;
            }

            let _cond = {
                event.has_value("json.ConfidenceScore")
                    && event.get_str("json.ConfidenceScore") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ConfidenceScore") {
                        if let Some(val) = event.get("json.ConfidenceScore") {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ConfidenceScore".into(),
                                    message,
                                }
                            })?;
                            event.set("microsoft_sentinel.event.confidence.score", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ConfidenceScore_to_double",
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

            if event.has_value("json.Description") {
                event.rename("json.Description", "microsoft_sentinel.event.description")?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.event.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if event.has_value("json.DisplayName") {
                event.rename("json.DisplayName", "microsoft_sentinel.event.display_name")?;
            }

            let _cond =
                { event.has_value("json.EndTime") && event.get_str("json.EndTime") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.EndTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("microsoft_sentinel.event.end_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.EndTime".into(),
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

            if let Some(v) = event
                .get("microsoft_sentinel.event.end_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            let _cond = {
                event.get("json.Entities").is_some_and(|v| v.is_string())
                    && event.get_str("json.Entities") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "json.Entities", "microsoft_sentinel.event.entities")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set("_ingest.on_failure_processor_tag", "json_Entities")?;
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

            if event.has_value("json.ExtendedLinks") {
                event.rename(
                    "json.ExtendedLinks",
                    "microsoft_sentinel.event.extended.links",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.event.extended.links")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reference", v)?;
            }

            let _cond = {
                event
                    .get("json.ExtendedProperties")
                    .is_some_and(|v| v.is_string())
                    && event.get_str("json.ExtendedProperties") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "json.ExtendedProperties",
                        "microsoft_sentinel.event.extended.properties",
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "json_ExtendedProperties",
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

            if event.has_value("json._Internal_WorkspaceResourceId") {
                event.rename(
                    "json._Internal_WorkspaceResourceId",
                    "microsoft_sentinel.event.internal_workspace_resource_id",
                )?;
            }

            let _cond = {
                event.has_value("json.IsIncident") && event.get_i64("json.IsIncident") == Some(0)
            };
            if _cond {
                event.set("json.IsIncident", json!(false))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.IsIncident") {
                    if let Some(val) = event.get("json.IsIncident") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.IsIncident".into(),
                                message,
                            }
                        })?;
                        event.set("microsoft_sentinel.event.is_incident", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_IsIncident_to_boolean",
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

            if event.has_value("json._ItemId") {
                event.rename("json._ItemId", "microsoft_sentinel.event.item_id")?;
            }

            let _cond = {
                event.has_value("json.ProcessingEndTime")
                    && event.get_str("json.ProcessingEndTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.ProcessingEndTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("microsoft_sentinel.event.processing_end_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.ProcessingEndTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_ProcessingEndTime")?;
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

            if event.has_value("json.ProductComponentName") {
                event.rename(
                    "json.ProductComponentName",
                    "microsoft_sentinel.event.product.component_name",
                )?;
            }

            if event.has_value("json.ProductName") {
                event.rename("json.ProductName", "microsoft_sentinel.event.product.name")?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.event.product.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.product", v)?;
            }

            if event.has_value("json.ProviderName") {
                event.rename(
                    "json.ProviderName",
                    "microsoft_sentinel.event.provider_name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.event.provider_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.provider", v)?;
            }

            if event.has_value("json.RemediationSteps") {
                event.rename(
                    "json.RemediationSteps",
                    "microsoft_sentinel.event.remediation_steps",
                )?;
            }

            if event.has_value("json.ResourceId") {
                event.rename("json.ResourceId", "microsoft_sentinel.event.resource_id")?;
            }

            if event.has_value("json.SourceComputerId") {
                event.rename(
                    "json.SourceComputerId",
                    "microsoft_sentinel.event.source.computer_id",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.event.source.computer_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
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

            if event.has_value("json.SourceSystem") {
                event.rename(
                    "json.SourceSystem",
                    "microsoft_sentinel.event.source.system",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.event.source.system")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.type", v)?;
            }

            let _cond = {
                event.has_value("json.StartTime") && event.get_str("json.StartTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.StartTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("microsoft_sentinel.event.start_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.StartTime".into(),
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

            if let Some(v) = event
                .get("microsoft_sentinel.event.start_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            let _cond = { event.has_value("event.start") && event.has_value("event.end") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: Instant event_start = ZonedDateTime.parse(ctx.event.start).toInstant();\nInstant event_end = ZonedDateTime.parse(ctx.event.end).toInstant();\nctx.event['duration'] = ChronoUnit.NANOS.between(event_start, event_end);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"Instant event_start = ZonedDateTime.parse(ctx.event.start).toInstant();\nInstant event_end = ZonedDateTime.parse(ctx.event.end).toInstant();\nctx.event['duration'] = ChronoUnit.NANOS.between(event_start, event_end);\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_set_event_duration",
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

            if event.has_value("json.Status") {
                event.rename("json.Status", "microsoft_sentinel.event.status")?;
            }

            if event.has_value("json.SystemAlertId") {
                event.rename(
                    "json.SystemAlertId",
                    "microsoft_sentinel.event.system_alert_id",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.event.system_alert_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.Tactics") {
                event.rename("json.Tactics", "microsoft_sentinel.event.tactics")?;
            }

            let _cond = {
                event
                    .get("microsoft_sentinel.event.tactics")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("microsoft_sentinel.event.tactics") {
                        if let Some(s) = event.get_string("microsoft_sentinel.event.tactics") {
                            let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                            event.set("microsoft_sentinel.event.tactics", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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
                    .get("microsoft_sentinel.event.tactics")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "microsoft_sentinel.event.tactics", |event| {
                    event.append_unique(
                        "threat.tactic.name",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.Techniques") {
                event.rename("json.Techniques", "microsoft_sentinel.event.techniques")?;
            }

            let _cond = {
                event
                    .get("microsoft_sentinel.event.techniques")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("microsoft_sentinel.event.techniques") {
                        if let Some(s) = event.get_string("microsoft_sentinel.event.techniques") {
                            let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                            event
                                .set("microsoft_sentinel.event.techniques", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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
                    .get("microsoft_sentinel.event.techniques")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "microsoft_sentinel.event.techniques", |event| {
                    event.append_unique(
                        "threat.technique.name",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.TenantId") {
                event.rename("json.TenantId", "microsoft_sentinel.event.tenant_id")?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.event.tenant_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            let _cond = {
                event.has_value("json.TimeGenerated")
                    && event.get_str("json.TimeGenerated") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.TimeGenerated") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("microsoft_sentinel.event.time_generated", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.TimeGenerated".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_TimeGenerated")?;
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
                .get("microsoft_sentinel.event.time_generated")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.Type") {
                event.rename("json.Type", "microsoft_sentinel.event.type")?;
            }

            if event.has_value("json.VendorName") {
                event.rename("json.VendorName", "microsoft_sentinel.event.vendor.name")?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.event.vendor.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.vendor", v)?;
            }

            if event.has_value("json.VendorOriginalId") {
                event.rename(
                    "json.VendorOriginalId",
                    "microsoft_sentinel.event.vendor.original_id",
                )?;
            }

            if event.has_value("json.WorkspaceResourceGroup") {
                event.rename(
                    "json.WorkspaceResourceGroup",
                    "microsoft_sentinel.event.workspace.resource_group",
                )?;
            }

            if event.has_value("json.WorkspaceSubscriptionId") {
                event.rename(
                    "json.WorkspaceSubscriptionId",
                    "microsoft_sentinel.event.workspace.subscription_id",
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
                event.remove("microsoft_sentinel.event.alert.link");
                event.remove("microsoft_sentinel.event.confidence.level");
                event.remove("microsoft_sentinel.event.description");
                event.remove("microsoft_sentinel.event.end_time");
                event.remove("microsoft_sentinel.event.extended.links");
                event.remove("microsoft_sentinel.event.product.name");
                event.remove("microsoft_sentinel.event.provider_name");
                event.remove("microsoft_sentinel.event.source.computer_id");
                event.remove("microsoft_sentinel.event.source.system");
                event.remove("microsoft_sentinel.event.start_time");
                event.remove("microsoft_sentinel.event.system_alert_id");
                event.remove("microsoft_sentinel.event.tactics");
                event.remove("microsoft_sentinel.event.techniques");
                event.remove("microsoft_sentinel.event.tenant_id");
                event.remove("microsoft_sentinel.event.time_generated");
                event.remove("microsoft_sentinel.event.vendor.name");
            }

            event.remove("json");

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
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
                        "Processor '{}' {}failed with message '{}'",
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
