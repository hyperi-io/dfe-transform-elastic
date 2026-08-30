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

            event.remove("oberver.product");
            event.remove("oberver.vendor");

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = { event.get_str("message") == Some("retry") };
            if _cond {
                return Ok(TransformResult::Drop);
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

            let _cond = { event.has_value("event.original") };
            if _cond {
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
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.properties.timeGenerated") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.id") {
                event.rename("json.id", "microsoft_sentinel.alert.id")?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.alert.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.kind") {
                event.rename("json.kind", "microsoft_sentinel.alert.kind")?;
            }

            if event.has_value("json.name") {
                event.rename("json.name", "microsoft_sentinel.alert.name")?;
            }

            if event.has_value("json.properties.additionalData") {
                event.rename(
                    "json.properties.additionalData",
                    "microsoft_sentinel.alert.properties.additional_data",
                )?;
            }

            if event.has_value("json.properties.alertDisplayName") {
                event.rename(
                    "json.properties.alertDisplayName",
                    "microsoft_sentinel.alert.properties.alert.display_name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.alert.properties.alert.display_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("json.properties.alertLink") {
                event.rename(
                    "json.properties.alertLink",
                    "microsoft_sentinel.alert.properties.alert.link",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.alert.properties.alert.link")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.url", v)?;
            }

            if event.has_value("json.properties.alertType") {
                event.rename(
                    "json.properties.alertType",
                    "microsoft_sentinel.alert.properties.alert.type",
                )?;
            }

            if event.has_value("json.properties.compromisedEntity") {
                event.rename(
                    "json.properties.compromisedEntity",
                    "microsoft_sentinel.alert.properties.compromised_entity",
                )?;
            }

            if event.has_value("json.properties.confidenceLevel") {
                event.rename(
                    "json.properties.confidenceLevel",
                    "microsoft_sentinel.alert.properties.confidence_level",
                )?;
            }

            let _cond = {
                ["High", "Low"].contains(
                    &event
                        .get_str("microsoft_sentinel.alert.properties.confidence_level")
                        .unwrap_or(""),
                )
            };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_sentinel.alert.properties.confidence_level")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.confidence", v)?;
                }
            }

            let _cond = {
                event.get_str("microsoft_sentinel.alert.properties.confidence_level")
                    == Some("Unknown")
            };
            if _cond {
                event.set("threat.indicator.confidence", json!("Not Specified"))?;
            }

            if event.has_value("json.properties.confidenceReasons.reason") {
                event.rename(
                    "json.properties.confidenceReasons.reason",
                    "microsoft_sentinel.alert.properties.confidence_reasons.reason",
                )?;
            }

            if event.has_value("json.properties.confidenceReasons.reasonType") {
                event.rename(
                    "json.properties.confidenceReasons.reasonType",
                    "microsoft_sentinel.alert.properties.confidence_reasons.reason_type",
                )?;
            }

            let _cond = { event.has_value("json.properties.confidenceScore") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.properties.confidenceScore") {
                        if let Some(val) = event.get("json.properties.confidenceScore") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.properties.confidenceScore".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "microsoft_sentinel.alert.properties.confidence_score",
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
                        "convert_properties_confidenceScore_to_long",
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

            if event.has_value("json.properties.confidenceScoreStatus") {
                event.rename(
                    "json.properties.confidenceScoreStatus",
                    "microsoft_sentinel.alert.properties.confidence_score_status",
                )?;
            }

            if event.has_value("json.properties.description") {
                event.rename(
                    "json.properties.description",
                    "microsoft_sentinel.alert.properties.description",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.alert.properties.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = {
                event.has_value("json.properties.endTimeUtc")
                    && event.get_str("json.properties.endTimeUtc") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.properties.endTimeUtc") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("microsoft_sentinel.alert.properties.end_time_utc", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.endTimeUtc".into(),
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
                        "date_properties_endTimeUtc",
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
                .get("microsoft_sentinel.alert.properties.end_time_utc")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            if event.has_value("json.properties.friendlyName") {
                event.rename(
                    "json.properties.friendlyName",
                    "microsoft_sentinel.alert.properties.friendly_name",
                )?;
            }

            if event.has_value("json.properties.intent") {
                event.rename(
                    "json.properties.intent",
                    "microsoft_sentinel.alert.properties.intent",
                )?;
            }

            let _cond = {
                event.has_value("json.properties.processingEndTime")
                    && event.get_str("json.properties.processingEndTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.properties.processingEndTime")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "microsoft_sentinel.alert.properties.processing_end_time",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.processingEndTime".into(),
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
                        "date_properties_processingEndTime",
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

            if event.has_value("json.properties.productComponentName") {
                event.rename(
                    "json.properties.productComponentName",
                    "microsoft_sentinel.alert.properties.product.component_name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.alert.properties.product.component_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.name", v)?;
            }

            if event.has_value("json.properties.productName") {
                event.rename(
                    "json.properties.productName",
                    "microsoft_sentinel.alert.properties.product.name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.alert.properties.product.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.provider", v)?;
            }

            if event.has_value("json.properties.productVersion") {
                event.rename(
                    "json.properties.productVersion",
                    "microsoft_sentinel.alert.properties.product.version",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_sentinel.alert.properties.product.version")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.version", v)?;
            }

            if event.has_value("json.properties.providerAlertId") {
                event.rename(
                    "json.properties.providerAlertId",
                    "microsoft_sentinel.alert.properties.provider_alert_id",
                )?;
            }

            if event.has_value("json.properties.remediationSteps") {
                event.rename(
                    "json.properties.remediationSteps",
                    "microsoft_sentinel.alert.properties.remediation_steps",
                )?;
            }

            if event.has_value("json.properties.resourceIdentifiers") {
                event.rename(
                    "json.properties.resourceIdentifiers",
                    "microsoft_sentinel.alert.properties.resource_identifiers",
                )?;
            }

            if event.has_value("json.properties.severity") {
                event.rename(
                    "json.properties.severity",
                    "microsoft_sentinel.alert.properties.severity",
                )?;
            }

            let _cond = { event.has_value("microsoft_sentinel.alert.properties.severity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def temp = ctx.microsoft_sentinel.alert.properties.severity.toLowerCase(); if (temp == 'informational') {\n  ctx.event.severity = 21;\n} else if (temp == 'low') {\n  ctx.event.severity = 21;\n} else if (temp == 'medium') {\n  ctx.event.severity = 47;\n} else if (temp == 'high') {\n  ctx.event.severity = 73;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def temp = ctx.microsoft_sentinel.alert.properties.severity.toLowerCase(); if (temp == 'informational') {\n  ctx.event.severity = 21;\n} else if (temp == 'low') {\n  ctx.event.severity = 21;\n} else if (temp == 'medium') {\n  ctx.event.severity = 47;\n} else if (temp == 'high') {\n  ctx.event.severity = 73;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_set_event_severity",
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
                event.has_value("json.properties.startTimeUtc")
                    && event.get_str("json.properties.startTimeUtc") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.properties.startTimeUtc") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "microsoft_sentinel.alert.properties.start_time_utc",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.startTimeUtc".into(),
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
                        "date_properties_startTimeUtc",
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
                .get("microsoft_sentinel.alert.properties.start_time_utc")
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

            if event.has_value("json.properties.status") {
                event.rename(
                    "json.properties.status",
                    "microsoft_sentinel.alert.properties.status",
                )?;
            }

            if event.has_value("json.properties.systemAlertId") {
                event.rename(
                    "json.properties.systemAlertId",
                    "microsoft_sentinel.alert.properties.system_alert_id",
                )?;
            }

            if event.has_value("json.properties.tactics") {
                event.rename(
                    "json.properties.tactics",
                    "microsoft_sentinel.alert.properties.tactics",
                )?;
            }

            let _cond = {
                event
                    .get("microsoft_sentinel.alert.properties.tactics")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "microsoft_sentinel.alert.properties.tactics",
                    |event| {
                        event.append_unique(
                            "threat.tactic.name",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event.has_value("json.properties.timeGenerated")
                    && event.get_str("json.properties.timeGenerated") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.properties.timeGenerated") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "microsoft_sentinel.alert.properties.time_generated",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.timeGenerated".into(),
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
                        "date_properties_timeGenerated",
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
                .get("microsoft_sentinel.alert.properties.time_generated")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.properties.vendorName") {
                event.rename(
                    "json.properties.vendorName",
                    "microsoft_sentinel.alert.properties.vendor_name",
                )?;
            }

            let _cond = {
                event.has_value("json.systemData.createdAt")
                    && event.get_str("json.systemData.createdAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.systemData.createdAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("microsoft_sentinel.alert.system_data.created_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.systemData.createdAt".into(),
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
                        "date_systemData_createdAt",
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

            if event.has_value("json.systemData.createdBy") {
                event.rename(
                    "json.systemData.createdBy",
                    "microsoft_sentinel.alert.system_data.created_by",
                )?;
            }

            if event.has_value("json.systemData.createdByType") {
                event.rename(
                    "json.systemData.createdByType",
                    "microsoft_sentinel.alert.system_data.created_by_type",
                )?;
            }

            let _cond = {
                event
                    .get_str("microsoft_sentinel.alert.system_data.created_by_type")
                    .is_some_and(|s| s.to_lowercase() == "user")
            };
            if _cond {
                if let Some(v) = event
                    .get("microsoft_sentinel.alert.system_data.created_by")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
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

            let _cond = {
                event.has_value("json.systemData.lastModifiedAt")
                    && event.get_str("json.systemData.lastModifiedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.systemData.lastModifiedAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "microsoft_sentinel.alert.system_data.last_modified_at",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.systemData.lastModifiedAt".into(),
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
                        "date_systemData_lastModifiedAt",
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

            if event.has_value("json.systemData.lastModifiedBy") {
                event.rename(
                    "json.systemData.lastModifiedBy",
                    "microsoft_sentinel.alert.system_data.last_modified_by",
                )?;
            }

            if event.has_value("json.systemData.lastModifiedByType") {
                event.rename(
                    "json.systemData.lastModifiedByType",
                    "microsoft_sentinel.alert.system_data.last_modified_by_type",
                )?;
            }

            let _cond = {
                event.has_value("microsoft_sentinel.alert.system_data.last_modified_by")
                    && event
                        .get_str("microsoft_sentinel.alert.system_data.last_modified_by_type")
                        .is_some_and(|s| s.to_lowercase() == "user")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("microsoft_sentinel.alert.system_data.last_modified_by")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "microsoft_sentinel.alert.type")?;
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
                event.remove("microsoft_sentinel.alert.id");
                event.remove("microsoft_sentinel.alert.properties.alert.link");
                event.remove("microsoft_sentinel.alert.properties.description");
                event.remove("microsoft_sentinel.alert.properties.end_time_utc");
                event.remove("microsoft_sentinel.alert.properties.product.component_name");
                event.remove("microsoft_sentinel.alert.properties.product.version");
                event.remove("microsoft_sentinel.alert.properties.start_time_utc");
                event.remove("microsoft_sentinel.alert.properties.tactics");
                event.remove("microsoft_sentinel.alert.properties.time_generated");
                event.remove("microsoft_sentinel.alert.properties.product.name");
                event.remove("microsoft_sentinel.alert.properties.alert.display_name");
            }

            event.remove("json");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_to_drop_null_values",
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
