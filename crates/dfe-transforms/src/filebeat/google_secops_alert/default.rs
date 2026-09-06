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

            event.set("event.kind", json!("alert"))?;

            if event.has_value("json") {
                event.rename("json", "google_secops.alert")?;
            }

            let _cond = {
                event.has_value("google_secops.alert.createdTime")
                    && event.get_str("google_secops.alert.createdTime") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("google_secops.alert.createdTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("google_secops.alert.createdTime", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_secops.alert.createdTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_createdTime")?;
                    event.remove("google_secops.alert.createdTime");
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
                .get("google_secops.alert.createdTime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.detection.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.description", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_secops.alert.detection.riskScore") {
                    if let Some(val) = event.get("google_secops.alert.detection.riskScore") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_secops.alert.detection.riskScore".into(),
                                message,
                            }
                        })?;
                        event.set("google_secops.alert.detection.riskScore", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_detection_riskScore_to_long",
                )?;
                event.remove("google_secops.alert.detection.riskScore");
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
                .get("google_secops.alert.detection.riskScore")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.risk_score", v)?;
            }

            let _cond = {
                event.get_str("google_secops.alert.detection.variables.risk_score.int64Val")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("google_secops.alert.detection.variables.risk_score.int64Val")
                    {
                        if let Some(val) =
                            event.get("google_secops.alert.detection.variables.risk_score.int64Val")
                        {
                            let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_secops.alert.detection.variables.risk_score.int64Val".into(),
                            message,
                        })?;
                            event.set(
                                "google_secops.alert.detection.variables.risk_score.int64Val",
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
                        "convert_detection_variables_risk_score_int64Val_to_long",
                    )?;
                    event.remove("google_secops.alert.detection.variables.risk_score.int64Val");
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
                event.get_str("google_secops.alert.detection.variables.risk_score.value")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("google_secops.alert.detection.variables.risk_score.value") {
                        if let Some(val) =
                            event.get("google_secops.alert.detection.variables.risk_score.value")
                        {
                            let converted =
                                convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                            path: "google_secops.alert.detection.variables.risk_score.value".into(),
                            message,
                        }
                                })?;
                            event.set(
                                "google_secops.alert.detection.variables.risk_score.value",
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
                        "convert_detection_variables_risk_score_value_to_long",
                    )?;
                    event.remove("google_secops.alert.detection.variables.risk_score.value");
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
                .get("google_secops.alert.detection.ruleId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.detection.ruleName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.detection.ruleName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("google_secops.alert.friendly_name", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.detection.ruleVersion")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.version", v)?;
            }

            let _cond = {
                event.has_value("google_secops.alert.detectionTime")
                    && event.get_str("google_secops.alert.detectionTime") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("google_secops.alert.detectionTime")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("google_secops.alert.detectionTime", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_secops.alert.detectionTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_detectionTime")?;
                    event.remove("google_secops.alert.detectionTime");
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
                .get("google_secops.alert.detectionTime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event
                    .has_value("google_secops.alert.event.metadata.baseLabels.allowScopedAccess")
                {
                    if let Some(val) =
                        event.get("google_secops.alert.event.metadata.baseLabels.allowScopedAccess")
                    {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_secops.alert.event.metadata.baseLabels.allowScopedAccess".into(),
                            message,
                        })?;
                        event.set(
                            "google_secops.alert.event.metadata.baseLabels.allowScopedAccess",
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
                    "convert_event_metadata_baseLabels_allowScopedAccess_to_boolean",
                )?;
                event.remove("google_secops.alert.event.metadata.baseLabels.allowScopedAccess");
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
                .get("google_secops.alert.event.metadata.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "google_secops.alert.event.metadata.enrichmentLabels.allowScopedAccess",
                ) {
                    if let Some(val) = event.get(
                        "google_secops.alert.event.metadata.enrichmentLabels.allowScopedAccess",
                    ) {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_secops.alert.event.metadata.enrichmentLabels.allowScopedAccess".into(),
                            message,
                        })?;
                        event.set(
                            "google_secops.alert.event.metadata.enrichmentLabels.allowScopedAccess",
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
                    "convert_event_metadata_enrichmentLabels_allowScopedAccess_to_boolean",
                )?;
                event.remove(
                    "google_secops.alert.event.metadata.enrichmentLabels.allowScopedAccess",
                );
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
                event.has_value("google_secops.alert.event.metadata.eventTimestamp")
                    && event.get_str("google_secops.alert.event.metadata.eventTimestamp")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_secops.alert.event.metadata.eventTimestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("google_secops.alert.event.metadata.eventTimestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_secops.alert.event.metadata.eventTimestamp"
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
                        "date_event_metadata_eventTimestamp",
                    )?;
                    event.remove("google_secops.alert.event.metadata.eventTimestamp");
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
                event.has_value("google_secops.alert.event.metadata.ingestedTimestamp")
                    && event.get_str("google_secops.alert.event.metadata.ingestedTimestamp")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_secops.alert.event.metadata.ingestedTimestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "google_secops.alert.event.metadata.ingestedTimestamp",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_secops.alert.event.metadata.ingestedTimestamp"
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
                        "date_event_metadata_ingestedTimestamp",
                    )?;
                    event.remove("google_secops.alert.event.metadata.ingestedTimestamp");
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
                .get("google_secops.alert.event.metadata.productName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.product", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.metadata.vendorName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.vendor", v)?;
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.dns.answers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.dns.answers") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.dns.answers",
                        |event| {
                            event.append_unique(
                                "dns.answers.data",
                                json!(
                                    event
                                        .get("_ingest._value.data")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.dns.answers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.dns.answers") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.dns.answers",
                        |event| {
                            event.append_unique(
                                "dns.answers.name",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.dns.answers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.dns.answers") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.dns.answers",
                        |event| {
                            event.append_unique(
                                "dns.answers.type",
                                json!(
                                    event
                                        .get("_ingest._value.type")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.dns.questions")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.dns.questions") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.dns.questions",
                        |event| {
                            event.append_unique(
                                "dns.question.name",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.dns.questions")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.dns.questions") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.dns.questions",
                        |event| {
                            event.append_unique(
                                "dns.question.type",
                                json!(
                                    event
                                        .get("_ingest._value.type")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.email.bcc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.email.bcc") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.email.bcc",
                        |event| {
                            event.append_unique(
                                "email.bcc.address",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.email.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.email.cc") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.email.cc",
                        |event| {
                            event.append_unique(
                                "email.cc.address",
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
            }

            let _cond = { event.has_value("google_secops.alert.event.network.email.from") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("google_secops.alert.event.network.email.from")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert.event.network.email.replyTo") };
            if _cond {
                event.append_unique(
                    "email.reply_to.address",
                    json!(
                        event
                            .get("google_secops.alert.event.network.email.replyTo")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.email.subject")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.email.subject") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.email.subject",
                        |event| {
                            event.append_unique(
                                "email.subject",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.email.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.email.to") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.email.to",
                        |event| {
                            event.append_unique(
                                "email.to.address",
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
            }

            if let Some(v) = event
                .get("google_secops.alert.event.network.http.method")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.request.method", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_secops.alert.event.network.http.responseCode") {
                    if let Some(val) =
                        event.get("google_secops.alert.event.network.http.responseCode")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_secops.alert.event.network.http.responseCode".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "google_secops.alert.event.network.http.responseCode",
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
                    "convert_event_network_http_responseCode_to_long",
                )?;
                event.remove("google_secops.alert.event.network.http.responseCode");
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
                .get("google_secops.alert.event.network.http.responseCode")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.status_code", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.network.http.userAgent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user_agent.original", v)?;
            }

            let _cond = { event.has_value("user_agent.original") };
            if _cond {
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
            }

            if let Some(v) = event
                .get("google_secops.alert.event.network.ipProtocol")
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

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.asset.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.asset.ip") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.asset.ip",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
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
                                    "convert_event_principal_asset_ip_to_ip",
                                )?;
                                event.remove("_ingest._value");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.asset.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.asset.ip") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.asset.ip",
                        |event| {
                            event.append_unique(
                                "related.ip",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.about")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.about") {
                    foreach_array(event, "google_secops.alert.event.about", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.user.userDisplayName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.about")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.about") {
                    foreach_array(event, "google_secops.alert.event.about", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.user.userid")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.intermediary")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.intermediary") {
                    foreach_array(event, "google_secops.alert.event.intermediary", |event| {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("_ingest._value.hostname")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = { event.has_value("google_secops.alert.event.principal.asset.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("google_secops.alert.event.principal.asset.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert.event.src.asset.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("google_secops.alert.event.src.asset.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.file.fullPath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.path", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.file.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.file.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.file.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            let _cond = { event.has_value("google_secops.alert.event.principal.file.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert.event.principal.file.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert.event.principal.file.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert.event.principal.file.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert.event.principal.file.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert.event.principal.file.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.group.groupDisplayName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.group.name", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("google_secops.alert.event.principal.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("google_secops.alert.event.principal.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.ipGeoArtifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.ipGeoArtifact") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.ipGeoArtifact",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.ip") {
                                    if let Some(val) = event.get("_ingest._value.ip") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.ip".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.ip", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_event_principal_ipGeoArtifact_ip_to_ip",
                                )?;
                                event.remove("_ingest._value.ip");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.ipGeoArtifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.ipGeoArtifact") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.ipGeoArtifact",
                        |event| {
                            event.append_unique(
                                "host.ip",
                                json!(
                                    event
                                        .get("_ingest._value.ip")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.ipGeoArtifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.ipGeoArtifact") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.ipGeoArtifact",
                        |event| {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("_ingest._value.ip")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.ipGeoArtifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.ipGeoArtifact") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.ipGeoArtifact",
                        |event| {
                            event.append_unique(
                                "host.geo.country_name",
                                json!(
                                    event
                                        .get("_ingest._value.location.countryOrRegion")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.ipGeoArtifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.ipGeoArtifact") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.ipGeoArtifact",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event
                                    .has_value("_ingest._value.location.regionCoordinates.latitude")
                                {
                                    if let Some(val) = event
                                        .get("_ingest._value.location.regionCoordinates.latitude")
                                    {
                                        let converted =
                                            convert_value(val, "double").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.location.regionCoordinates.latitude".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.location.regionCoordinates.lat",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_event_principal_ipGeoArtifact_location_regionCoordinates_latitude_to_double")?;
                                event.remove("_ingest._value.location.regionCoordinates.lat");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.ipGeoArtifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.ipGeoArtifact") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.ipGeoArtifact",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value(
                                    "_ingest._value.location.regionCoordinates.longitude",
                                ) {
                                    if let Some(val) = event
                                        .get("_ingest._value.location.regionCoordinates.longitude")
                                    {
                                        let converted =
                                            convert_value(val, "double").map_err(|message| {
                                                TransformError::ParseError {
                    path: "_ingest._value.location.regionCoordinates.longitude".into(),
                    message,
                    }
                                            })?;
                                        event.set(
                                            "_ingest._value.location.regionCoordinates.lon",
                                            converted,
                                        )?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set("_ingest.on_failure_processor_tag", "convert_event_principal_ipGeoArtifact_location_regionCoordinates_longitude_to_double")?;
                                event.remove("_ingest._value.location.regionCoordinates.lon");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.ipGeoArtifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.ipGeoArtifact") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.ipGeoArtifact",
                        |event| {
                            event.append_unique(
                                "host.geo.region_name",
                                json!(
                                    event
                                        .get("_ingest._value.location.state")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.ipGeoArtifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.ipGeoArtifact") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.ipGeoArtifact",
                        |event| {
                            event.remove("_ingest._value.location.regionCoordinates.latitude");
                            event.remove("_ingest._value.location.regionCoordinates.longitude");
                            Ok(())
                        },
                    )?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def locationList = new ArrayList();\nif (ctx.google_secops?.alert?.event?.principal?.ipGeoArtifact instanceof List) {\n  for (list in ctx.google_secops.alert.event.principal.ipGeoArtifact) {\n    if (list.location.regionCoordinates != null && list.location.regionCoordinates != ''){\n      locationList.add(list.location.regionCoordinates);\n    }\n  }\n}\nif (!(ctx.host instanceof HashMap)) {\n  ctx.host = new HashMap();\n}\nif (!(ctx.host.geo instanceof HashMap)) {\n  ctx.host.geo = new HashMap();\n}\nctx.host.geo.location = locationList;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def locationList = new ArrayList();\nif (ctx.google_secops?.alert?.event?.principal?.ipGeoArtifact instanceof List) {\n  for (list in ctx.google_secops.alert.event.principal.ipGeoArtifact) {\n    if (list.location.regionCoordinates != null && list.location.regionCoordinates != ''){\n      locationList.add(list.location.regionCoordinates);\n    }\n  }\n}\nif (!(ctx.host instanceof HashMap)) {\n  ctx.host = new HashMap();\n}\nif (!(ctx.host.geo instanceof HashMap)) {\n  ctx.host.geo = new HashMap();\n}\nctx.host.geo.location = locationList;"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set("_ingest.on_failure_processor_tag", "map_host_geo_location")?;
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
                event
                    .get("google_secops.alert.event.principal.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.ip") {
                    foreach_array(event, "google_secops.alert.event.principal.ip", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
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
                                "convert_event_principal_ip_to_ip",
                            )?;
                            event.remove("_ingest._value");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.ip") {
                    foreach_array(event, "google_secops.alert.event.principal.ip", |event| {
                        event.append_unique(
                            "host.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.ip") {
                    foreach_array(event, "google_secops.alert.event.principal.ip", |event| {
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.mac")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.mac") {
                    foreach_array(event, "google_secops.alert.event.principal.mac", |event| {
                        event.append_unique(
                            "host.mac",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("host.mac") {
                    gsub_field(event, "host.mac", "host.mac", cached_regex!("[-:.]"), "-")?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_host_mac")?;
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
                if event.has_value("host.mac") {
                    map_strings(event, "host.mac", "host.mac", str::to_uppercase)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uppercase")?;
                event.set("_ingest.on_failure_processor_tag", "uppercase_host_mac")?;
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_secops.alert.event.principal.port") {
                    if let Some(val) = event.get("google_secops.alert.event.principal.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_secops.alert.event.principal.port".into(),
                                message,
                            }
                        })?;
                        event.set("google_secops.alert.event.principal.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_event_principal_port_to_long",
                )?;
                event.remove("google_secops.alert.event.principal.port");
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
                .get("google_secops.alert.event.principal.process.commandLine")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.process.file.fullPath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.process.file.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.md5", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.process.file.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.process.file.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha256", v)?;
            }

            let _cond = { event.has_value("google_secops.alert.event.principal.process.file.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert.event.principal.process.file.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("google_secops.alert.event.principal.process.file.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert.event.principal.process.file.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("google_secops.alert.event.principal.process.file.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert.event.principal.process.file.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.process.parentProcess.commandLine")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.command_line", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.process.parentProcess.file.fullPath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.executable", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.process.parentProcess.file.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.hash.md5", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.process.parentProcess.file.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.hash.sha1", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.process.parentProcess.file.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.hash.sha256", v)?;
            }

            let _cond = {
                event
                    .has_value("google_secops.alert.event.principal.process.parentProcess.file.md5")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get(
                                "google_secops.alert.event.principal.process.parentProcess.file.md5"
                            )
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value(
                    "google_secops.alert.event.principal.process.parentProcess.file.sha1",
                )
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert.event.principal.process.parentProcess.file.sha1").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value(
                    "google_secops.alert.event.principal.process.parentProcess.file.sha256",
                )
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert.event.principal.process.parentProcess.file.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.get_str("google_secops.alert.event.principal.process.parentProcess.pid")
                    != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event
                        .has_value("google_secops.alert.event.principal.process.parentProcess.pid")
                    {
                        if let Some(val) = event
                            .get("google_secops.alert.event.principal.process.parentProcess.pid")
                        {
                            let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "google_secops.alert.event.principal.process.parentProcess.pid".into(),
                            message,
                        })?;
                            event.set(
                                "google_secops.alert.event.principal.process.parentProcess.pid",
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
                        "convert_event_principal_process_parentProcess_pid_to_long",
                    )?;
                    event.remove("google_secops.alert.event.principal.process.parentProcess.pid");
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
                .get("google_secops.alert.event.principal.process.parentProcess.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.pid", v)?;
            }

            let _cond = {
                event.has_value("google_secops.alert.event.principal.process.parentProcess.parentProcess.file.md5")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert.event.principal.process.parentProcess.parentProcess.file.md5").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert.event.principal.process.parentProcess.parentProcess.file.sha1")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert.event.principal.process.parentProcess.parentProcess.file.sha1").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert.event.principal.process.parentProcess.parentProcess.file.sha256")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert.event.principal.process.parentProcess.parentProcess.file.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert.event.principal.process.parentProcess.parentProcess.parentProcess.file.md5")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert.event.principal.process.parentProcess.parentProcess.parentProcess.file.md5").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert.event.principal.process.parentProcess.parentProcess.parentProcess.file.sha1")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert.event.principal.process.parentProcess.parentProcess.parentProcess.file.sha1").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert.event.principal.process.parentProcess.parentProcess.parentProcess.file.sha256")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert.event.principal.process.parentProcess.parentProcess.parentProcess.file.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert.event.principal.process.parentProcess.parentProcess.parentProcess.parentProcess.file.md5")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert.event.principal.process.parentProcess.parentProcess.parentProcess.parentProcess.file.md5").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert.event.principal.process.parentProcess.parentProcess.parentProcess.parentProcess.file.sha1")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert.event.principal.process.parentProcess.parentProcess.parentProcess.parentProcess.file.sha1").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = {
                event.has_value("google_secops.alert.event.principal.process.parentProcess.parentProcess.parentProcess.parentProcess.file.sha256")
            };
            if _cond {
                event.append_unique("related.hash", json!(event.get("google_secops.alert.event.principal.process.parentProcess.parentProcess.parentProcess.parentProcess.file.sha256").map_or_else(String::new, template_to_string)))?;
            }

            let _cond =
                { event.get_str("google_secops.alert.event.principal.process.pid") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("google_secops.alert.event.principal.process.pid") {
                        if let Some(val) =
                            event.get("google_secops.alert.event.principal.process.pid")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "google_secops.alert.event.principal.process.pid".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "google_secops.alert.event.principal.process.pid",
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
                        "convert_event_principal_process_pid_to_long",
                    )?;
                    event.remove("google_secops.alert.event.principal.process.pid");
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
                .get("google_secops.alert.event.principal.process.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pid", v)?;
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.user.attribute.roles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.user.attribute.roles") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.user.attribute.roles",
                        |event| {
                            event.append_unique(
                                "user.roles",
                                json!(
                                    event
                                        .get("_ingest._value.name")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.user.emailAddresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.user.emailAddresses") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.user.emailAddresses",
                        |event| {
                            event.append_unique(
                                "user.email",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.user.emailAddresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.user.emailAddresses") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.user.emailAddresses",
                        |event| {
                            event.append_unique(
                                "related.user",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.user.groupIdentifiers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.user.groupIdentifiers") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.user.groupIdentifiers",
                        |event| {
                            event.append_unique(
                                "user.group.id",
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
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.user.userDisplayName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond =
                { event.has_value("google_secops.alert.event.principal.user.userDisplayName") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_secops.alert.event.principal.user.userDisplayName")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.principal.user.userid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("google_secops.alert.event.principal.user.userid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_secops.alert.event.principal.user.userid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.securityResult")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.securityResult") {
                    foreach_array(event, "google_secops.alert.event.securityResult", |event| {
                        if event.has_value("_ingest._value.action") {
                            foreach_array(event, "_ingest._value.action", |event| {
                                event.append_unique(
                                    "event.action",
                                    json!(
                                        event
                                            .get("_ingest._value")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                        }
                        Ok(())
                    })?;
                }
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.securityResult")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.securityResult") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("google_secops.alert.event.securityResult")
                            .cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.firstDiscoveredTime")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => event.set(
                                                "_ingest._value.firstDiscoveredTime",
                                                parsed,
                                            )?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.firstDiscoveredTime"
                                                        .into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
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
                                        "date_event_securityResult_firstDiscoveredTime",
                                    )?;
                                    event.remove("_ingest._value.firstDiscoveredTime");
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "google_secops.alert.event.securityResult",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.google_secops?.alert?.event?.securityResult instanceof List) {\n  for (list in ctx.google_secops?.alert?.event.securityResult) {\n    if (list[\"severity\"] != null && list[\"severity\"] != '') {\n      if (list[\"severity\"].toUpperCase() == 'CRITICAL') {\n        ctx.event.severity = 99\n      } else if (list[\"severity\"].toUpperCase() == 'ERROR') {\n        ctx.event.severity = 99\n      } else if (list[\"severity\"].toUpperCase() == 'HIGH') {\n        ctx.event.severity = 73\n      } else if (list[\"severity\"].toUpperCase() == 'INFORMATIONAL') {\n        ctx.event.severity = 21\n      }  else if (list[\"severity\"].toUpperCase() == 'LOW') {\n        ctx.event.severity = 21\n      } else if (list[\"severity\"].toUpperCase() == 'MEDIUM') {\n        ctx.event.severity = 47\n      } else if (list[\"severity\"].toUpperCase() == 'NONE') {\n        ctx.event.severity = 21\n      } else if (list[\"severity\"].toUpperCase() == 'UNKNOWN_SEVERITY') {\n        ctx.event.severity = 21\n      }\n    }\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.google_secops?.alert?.event?.securityResult instanceof List) {\n  for (list in ctx.google_secops?.alert?.event.securityResult) {\n    if (list[\"severity\"] != null && list[\"severity\"] != '') {\n      if (list[\"severity\"].toUpperCase() == 'CRITICAL') {\n        ctx.event.severity = 99\n      } else if (list[\"severity\"].toUpperCase() == 'ERROR') {\n        ctx.event.severity = 99\n      } else if (list[\"severity\"].toUpperCase() == 'HIGH') {\n        ctx.event.severity = 73\n      } else if (list[\"severity\"].toUpperCase() == 'INFORMATIONAL') {\n        ctx.event.severity = 21\n      }  else if (list[\"severity\"].toUpperCase() == 'LOW') {\n        ctx.event.severity = 21\n      } else if (list[\"severity\"].toUpperCase() == 'MEDIUM') {\n        ctx.event.severity = 47\n      } else if (list[\"severity\"].toUpperCase() == 'NONE') {\n        ctx.event.severity = 21\n      } else if (list[\"severity\"].toUpperCase() == 'UNKNOWN_SEVERITY') {\n        ctx.event.severity = 21\n      }\n    }\n  }\n}"#
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

            let _cond = {
                event
                    .get("google_secops.alert.event.src.asset.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.src.asset.ip") {
                    foreach_array(event, "google_secops.alert.event.src.asset.ip", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
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
                                "convert_event_src_asset_ip_to_ip",
                            )?;
                            event.remove("_ingest._value");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.src.asset.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.src.asset.ip") {
                    foreach_array(event, "google_secops.alert.event.src.asset.ip", |event| {
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
            }

            let _cond = { event.has_value("google_secops.alert.event.src.file.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert.event.src.file.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert.event.src.file.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert.event.src.file.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert.event.src.file.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert.event.src.file.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert.event.target.asset.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("google_secops.alert.event.target.asset.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.src.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.domain", v)?;
            }

            let _cond = { event.has_value("google_secops.alert.event.src.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("google_secops.alert.event.src.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.src.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.src.ip") {
                    foreach_array(event, "google_secops.alert.event.src.ip", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
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
                                "convert_event_src_ip_to_ip",
                            )?;
                            event.remove("_ingest._value");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.src.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.src.ip") {
                    foreach_array(event, "google_secops.alert.event.src.ip", |event| {
                        event.append_unique(
                            "source.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.src.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.src.ip") {
                    foreach_array(event, "google_secops.alert.event.src.ip", |event| {
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.src.mac")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.src.mac") {
                    foreach_array(event, "google_secops.alert.event.src.mac", |event| {
                        event.append_unique(
                            "source.mac",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("source.mac") {
                    gsub_field(
                        event,
                        "source.mac",
                        "source.mac",
                        cached_regex!("[-:.]"),
                        "-",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_source_mac")?;
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
                if event.has_value("source.mac") {
                    map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uppercase")?;
                event.set("_ingest.on_failure_processor_tag", "uppercase_source_mac")?;
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
                event
                    .get("google_secops.alert.event.src.user.emailAddresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.src.user.emailAddresses") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.src.user.emailAddresses",
                        |event| {
                            event.append_unique(
                                "source.user.email",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.src.user.emailAddresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.src.user.emailAddresses") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.src.user.emailAddresses",
                        |event| {
                            event.append_unique(
                                "related.user",
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
            }

            if let Some(v) = event
                .get("google_secops.alert.event.src.user.userid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.id", v)?;
            }

            let _cond = { event.has_value("google_secops.alert.event.src.user.userid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_secops.alert.event.src.user.userid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.target.asset.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.target.asset.ip") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.target.asset.ip",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
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
                                    "convert_event_target_asset_ip_to_ip",
                                )?;
                                event.remove("_ingest._value");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.target.asset.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.target.asset.ip") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.target.asset.ip",
                        |event| {
                            event.append_unique(
                                "related.ip",
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
            }

            if let Some(v) = event
                .get("google_secops.alert.event.target.cloud.project.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.target.project.name", v)?;
            }

            let _cond = {
                event.has_value("google_secops.alert.event.target.file.lastModificationTime")
                    && event.get_str("google_secops.alert.event.target.file.lastModificationTime")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("google_secops.alert.event.target.file.lastModificationTime")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "google_secops.alert.event.target.file.lastModificationTime",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path:
                                        "google_secops.alert.event.target.file.lastModificationTime"
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
                        "date_event_target_file_lastModificationTime",
                    )?;
                    event.remove("google_secops.alert.event.target.file.lastModificationTime");
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

            let _cond = { event.has_value("google_secops.alert.event.target.file.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert.event.target.file.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert.event.target.file.sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert.event.target.file.sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert.event.target.file.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("google_secops.alert.event.target.file.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.target.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
            }

            let _cond = { event.has_value("google_secops.alert.event.target.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("google_secops.alert.event.target.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.target.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.target.ip") {
                    foreach_array(event, "google_secops.alert.event.target.ip", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value") {
                                if let Some(val) = event.get("_ingest._value") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
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
                                "convert_event_target_ip_to_ip",
                            )?;
                            event.remove("_ingest._value");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.target.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.target.ip") {
                    foreach_array(event, "google_secops.alert.event.target.ip", |event| {
                        event.append_unique(
                            "destination.ip",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.target.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.target.ip") {
                    foreach_array(event, "google_secops.alert.event.target.ip", |event| {
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.target.mac")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.target.mac") {
                    foreach_array(event, "google_secops.alert.event.target.mac", |event| {
                        event.append_unique(
                            "destination.mac",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("destination.mac") {
                    gsub_field(
                        event,
                        "destination.mac",
                        "destination.mac",
                        cached_regex!("[-:.]"),
                        "-",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set("_ingest.on_failure_processor_tag", "gsub_destination_mac")?;
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
                if event.has_value("destination.mac") {
                    map_strings(
                        event,
                        "destination.mac",
                        "destination.mac",
                        str::to_uppercase,
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uppercase")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "uppercase_destination_mac",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("google_secops.alert.event.target.port") {
                    if let Some(val) = event.get("google_secops.alert.event.target.port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "google_secops.alert.event.target.port".into(),
                                message,
                            }
                        })?;
                        event.set("google_secops.alert.event.target.port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_event_target_port_to_long",
                )?;
                event.remove("google_secops.alert.event.target.port");
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
                .get("google_secops.alert.event.target.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            let _cond = {
                event.has_value("google_secops.alert.event.target.process.file.firstSeenTime")
                    && event.get_str("google_secops.alert.event.target.process.file.firstSeenTime")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "google_secops.alert.event.target.process.file.firstSeenTime",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "google_secops.alert.event.target.process.file.firstSeenTime",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                path: "google_secops.alert.event.target.process.file.firstSeenTime".into(),
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
                        "date_event_target_process_file_firstSeenTime",
                    )?;
                    event.remove("google_secops.alert.event.target.process.file.firstSeenTime");
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
                    .has_value("google_secops.alert.event.target.process.file.lastModificationTime")
                    && event.get_str(
                        "google_secops.alert.event.target.process.file.lastModificationTime",
                    ) != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string(
                        "google_secops.alert.event.target.process.file.lastModificationTime",
                    ) {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("google_secops.alert.event.target.process.file.lastModificationTime", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "google_secops.alert.event.target.process.file.lastModificationTime".into(),
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
                        "date_event_target_process_file_lastModificationTime",
                    )?;
                    event.remove(
                        "google_secops.alert.event.target.process.file.lastModificationTime",
                    );
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
                    .get("google_secops.alert.event.target.user.emailAddresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.target.user.emailAddresses") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.target.user.emailAddresses",
                        |event| {
                            event.append_unique(
                                "destination.user.email",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.target.user.emailAddresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.target.user.emailAddresses") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.target.user.emailAddresses",
                        |event| {
                            event.append_unique(
                                "related.user",
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
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.processAncestors")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.processAncestors") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.processAncestors",
                        |event| {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value.parentProcess.file.md5")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.processAncestors")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.processAncestors") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.processAncestors",
                        |event| {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value.parentProcess.file.sha1")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.processAncestors")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.processAncestors") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.processAncestors",
                        |event| {
                            event.append_unique(
                                "related.hash",
                                json!(
                                    event
                                        .get("_ingest._value.parentProcess.file.sha256")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.target.user.groupIdentifiers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.target.user.groupIdentifiers") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.target.user.groupIdentifiers",
                        |event| {
                            event.append_unique(
                                "destination.user.group.id",
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
            }

            if let Some(v) = event
                .get("google_secops.alert.event.target.user.userDisplayName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.name", v)?;
            }

            if let Some(v) = event
                .get("google_secops.alert.event.target.user.userid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.user.id", v)?;
            }

            let _cond =
                { event.has_value("google_secops.alert.event.target.user.userDisplayName") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_secops.alert.event.target.user.userDisplayName")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("google_secops.alert.event.target.user.userid") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("google_secops.alert.event.target.user.userid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("google_secops.alert.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event.has_value("google_secops.alert.timeWindow.endTime")
                    && event.get_str("google_secops.alert.timeWindow.endTime") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_secops.alert.timeWindow.endTime")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("google_secops.alert.timeWindow.endTime", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_secops.alert.timeWindow.endTime".into(),
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
                        "date_timeWindow_endTime",
                    )?;
                    event.remove("google_secops.alert.timeWindow.endTime");
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
                .get("google_secops.alert.timeWindow.endTime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            let _cond = {
                event.has_value("google_secops.alert.timeWindow.startTime")
                    && event.get_str("google_secops.alert.timeWindow.startTime") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("google_secops.alert.timeWindow.startTime")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("google_secops.alert.timeWindow.startTime", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "google_secops.alert.timeWindow.startTime".into(),
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
                        "date_timeWindow_startTime",
                    )?;
                    event.remove("google_secops.alert.timeWindow.startTime");
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
                .get("google_secops.alert.timeWindow.startTime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.target.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.target.ip") {
                    foreach_array(event, "google_secops.alert.event.target.ip", |event| {
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
                            event.remove("_ingest._value");
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.target.user.emailAddresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.target.user.emailAddresses") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.target.user.emailAddresses",
                        |event| {
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
                                event.remove("_ingest._value");
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.target.user.groupIdentifiers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.target.user.groupIdentifiers") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.target.user.groupIdentifiers",
                        |event| {
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
                                event.remove("_ingest._value");
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.email.bcc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.email.bcc") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.email.bcc",
                        |event| {
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
                                event.remove("_ingest._value");
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.email.cc")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.email.cc") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.email.cc",
                        |event| {
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
                                event.remove("_ingest._value");
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.email.subject")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.email.subject") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.email.subject",
                        |event| {
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
                                event.remove("_ingest._value");
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.email.to")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.email.to") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.email.to",
                        |event| {
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
                                event.remove("_ingest._value");
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.user.groupIdentifiers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.user.groupIdentifiers") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.user.groupIdentifiers",
                        |event| {
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
                                event.remove("_ingest._value");
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.user.emailAddresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.user.emailAddresses") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.user.emailAddresses",
                        |event| {
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
                                event.remove("_ingest._value");
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.src.user.emailAddresses")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.src.user.emailAddresses") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.src.user.emailAddresses",
                        |event| {
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
                                event.remove("_ingest._value");
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.src.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.src.ip") {
                    foreach_array(event, "google_secops.alert.event.src.ip", |event| {
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
                            event.remove("_ingest._value");
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.ip")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.ip") {
                    foreach_array(event, "google_secops.alert.event.principal.ip", |event| {
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
                            event.remove("_ingest._value");
                        }
                        Ok(())
                    })?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.dns.answers")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.dns.answers") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.dns.answers",
                        |event| {
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
                                event.remove("_ingest._value.data");
                                event.remove("_ingest._value.name");
                                event.remove("_ingest._value.type");
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.network.dns.questions")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.network.dns.questions") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.network.dns.questions",
                        |event| {
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
                                event.remove("_ingest._value.name");
                                event.remove("_ingest._value.type");
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.ipGeoArtifact")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.ipGeoArtifact") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.ipGeoArtifact",
                        |event| {
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
                                event.remove("_ingest._value.location.countryOrRegion");
                                event.remove("_ingest._value.location.state");
                                event.remove("_ingest._value.ip");
                            }
                            Ok(())
                        },
                    )?;
                }
            }

            let _cond = {
                event
                    .get("google_secops.alert.event.principal.user.attribute.roles")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("google_secops.alert.event.principal.user.attribute.roles") {
                    foreach_array(
                        event,
                        "google_secops.alert.event.principal.user.attribute.roles",
                        |event| {
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
                                event.remove("_ingest._value.name");
                            }
                            Ok(())
                        },
                    )?;
                }
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
                event.remove("google_secops.alert.createdTime");
                event.remove("google_secops.alert.detection.description");
                event.remove("google_secops.alert.detection.riskScore");
                event.remove("google_secops.alert.detection.ruleId");
                event.remove("google_secops.alert.detection.ruleName");
                event.remove("google_secops.alert.detection.ruleVersion");
                event.remove("google_secops.alert.detectionTime");
                event.remove("google_secops.alert.event.metadata.description");
                event.remove("google_secops.alert.event.metadata.productName");
                event.remove("google_secops.alert.event.metadata.vendorName");
                event.remove("google_secops.alert.event.network.email.from");
                event.remove("google_secops.alert.event.network.email.replyTo");
                event.remove("google_secops.alert.event.network.http.method");
                event.remove("google_secops.alert.event.network.http.responseCode");
                event.remove("google_secops.alert.event.network.http.userAgent");
                event.remove("google_secops.alert.event.network.ipProtocol");
                event.remove("google_secops.alert.event.principal.file.fullPath");
                event.remove("google_secops.alert.event.principal.file.md5");
                event.remove("google_secops.alert.event.principal.file.sha1");
                event.remove("google_secops.alert.event.principal.file.sha256");
                event.remove("google_secops.alert.event.principal.group.groupDisplayName");
                event.remove("google_secops.alert.event.principal.hostname");
                event.remove("google_secops.alert.event.principal.process.commandLine");
                event.remove("google_secops.alert.event.principal.process.file.fullPath");
                event.remove("google_secops.alert.event.principal.process.file.md5");
                event.remove("google_secops.alert.event.principal.process.file.sha1");
                event.remove("google_secops.alert.event.principal.process.file.sha256");
                event.remove(
                    "google_secops.alert.event.principal.process.parentProcess.commandLine",
                );
                event.remove(
                    "google_secops.alert.event.principal.process.parentProcess.file.fullPath",
                );
                event.remove("google_secops.alert.event.principal.process.parentProcess.file.md5");
                event.remove("google_secops.alert.event.principal.process.parentProcess.file.sha1");
                event.remove(
                    "google_secops.alert.event.principal.process.parentProcess.file.sha256",
                );
                event.remove("google_secops.alert.event.principal.process.parentProcess.pid");
                event.remove("google_secops.alert.event.principal.process.pid");
                event.remove("google_secops.alert.event.principal.user.userDisplayName");
                event.remove("google_secops.alert.event.principal.user.userid");
                event.remove("google_secops.alert.event.src.hostname");
                event.remove("google_secops.alert.event.src.user.userid");
                event.remove("google_secops.alert.event.target.cloud.project.name");
                event.remove("google_secops.alert.event.target.hostname");
                event.remove("google_secops.alert.event.target.port");
                event.remove("google_secops.alert.event.target.user.userDisplayName");
                event.remove("google_secops.alert.event.target.user.userid");
                event.remove("google_secops.alert.id");
                event.remove("google_secops.alert.timeWindow.endTime");
                event.remove("google_secops.alert.timeWindow.startTime");
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
