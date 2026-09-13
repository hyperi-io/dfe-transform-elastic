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
            event.set("ecs.version", json!("9.3.0"))?;

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

            parse_json_field(event, "event.original", "beyondtrust_epm.audit")?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("beyondtrust_epm.audit.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("change"))?;

            event.append("event.category", json!("configuration"))?;

            let _cond = {
                event.has_value("beyondtrust_epm.audit.agentDataAuditing.newTimestamp")
                    && event.get_str("beyondtrust_epm.audit.agentDataAuditing.newTimestamp")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("beyondtrust_epm.audit.agentDataAuditing.newTimestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "beyondtrust_epm.audit.agentDataAuditing.newTimestamp",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "beyondtrust_epm.audit.agentDataAuditing.newTimestamp"
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
                        "date_agentDataAuditing_newTimestamp",
                    )?;
                    event.remove("beyondtrust_epm.audit.agentDataAuditing.newTimestamp");
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
                event.has_value("beyondtrust_epm.audit.agentDataAuditing.oldTimestamp")
                    && event.get_str("beyondtrust_epm.audit.agentDataAuditing.oldTimestamp")
                        != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("beyondtrust_epm.audit.agentDataAuditing.oldTimestamp")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "beyondtrust_epm.audit.agentDataAuditing.oldTimestamp",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "beyondtrust_epm.audit.agentDataAuditing.oldTimestamp"
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
                        "date_agentDataAuditing_oldTimestamp",
                    )?;
                    event.remove("beyondtrust_epm.audit.agentDataAuditing.oldTimestamp");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("beyondtrust_epm.audit.apiClientDataAuditing.deleted") {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.apiClientDataAuditing.deleted")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_epm.audit.apiClientDataAuditing.deleted".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.apiClientDataAuditing.deleted",
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
                    "convert_apiClientDataAuditing_deleted_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.apiClientDataAuditing.deleted");
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
                if event.has_value("beyondtrust_epm.audit.apiClientDataAuditing.secretUpdated") {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.apiClientDataAuditing.secretUpdated")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_epm.audit.apiClientDataAuditing.secretUpdated"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.apiClientDataAuditing.secretUpdated",
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
                    "convert_apiClientDataAuditing_secretUpdated_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.apiClientDataAuditing.secretUpdated");
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
                if event.has_value("beyondtrust_epm.audit.authorizationRequestDataAuditing.authRequestConfigChanged") {
                if let Some(val) = event.get("beyondtrust_epm.audit.authorizationRequestDataAuditing.authRequestConfigChanged") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.authorizationRequestDataAuditing.authRequestConfigChanged".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.authorizationRequestDataAuditing.authRequestConfigChanged", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_authorizationRequestDataAuditing_authRequestConfigChanged_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.authorizationRequestDataAuditing.authRequestConfigChanged");
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
                if event.has_value("beyondtrust_epm.audit.authorizationRequestDataAuditing.authRequestIntegrationEnabled") {
                if let Some(val) = event.get("beyondtrust_epm.audit.authorizationRequestDataAuditing.authRequestIntegrationEnabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.authorizationRequestDataAuditing.authRequestIntegrationEnabled".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.authorizationRequestDataAuditing.authRequestIntegrationEnabled", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_authorizationRequestDataAuditing_authRequestIntegrationEnabled_to_boolean")?;
                event.remove("beyondtrust_epm.audit.authorizationRequestDataAuditing.authRequestIntegrationEnabled");
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
                if event.has_value("beyondtrust_epm.audit.authorizationRequestDataAuditing.oldAuthRequestIntegrationEnabled") {
                if let Some(val) = event.get("beyondtrust_epm.audit.authorizationRequestDataAuditing.oldAuthRequestIntegrationEnabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.authorizationRequestDataAuditing.oldAuthRequestIntegrationEnabled".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.authorizationRequestDataAuditing.oldAuthRequestIntegrationEnabled", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_authorizationRequestDataAuditing_oldAuthRequestIntegrationEnabled_to_boolean")?;
                event.remove("beyondtrust_epm.audit.authorizationRequestDataAuditing.oldAuthRequestIntegrationEnabled");
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
                if event.has_value("beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.oldPmRequestsLimitMinutes") {
                if let Some(val) = event.get("beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.oldPmRequestsLimitMinutes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.oldPmRequestsLimitMinutes".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.oldPmRequestsLimitMinutes", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_autoUpdateRateLimitDataAuditing_oldPmRequestsLimitMinutes_to_long",
                )?;
                event.remove("beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.oldPmRequestsLimitMinutes");
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
                if event.has_value(
                    "beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.oldPmRequestsLimits",
                ) {
                    if let Some(val) = event.get(
                        "beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.oldPmRequestsLimits",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.oldPmRequestsLimits".into(),
                            message,
                        })?;
                        event.set("beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.oldPmRequestsLimits", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_autoUpdateRateLimitDataAuditing_oldPmRequestsLimits_to_long",
                )?;
                event.remove(
                    "beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.oldPmRequestsLimits",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.pmRequestsLimitMinutes",
                ) {
                    if let Some(val) = event.get("beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.pmRequestsLimitMinutes") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.pmRequestsLimitMinutes".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.pmRequestsLimitMinutes", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_autoUpdateRateLimitDataAuditing_pmRequestsLimitMinutes_to_long",
                )?;
                event.remove(
                    "beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.pmRequestsLimitMinutes",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.pmRequestsLimits",
                ) {
                    if let Some(val) = event.get(
                        "beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.pmRequestsLimits",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.pmRequestsLimits".into(),
                            message,
                        })?;
                        event.set("beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.pmRequestsLimits", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_autoUpdateRateLimitDataAuditing_pmRequestsLimits_to_long",
                )?;
                event.remove(
                    "beyondtrust_epm.audit.autoUpdateRateLimitDataAuditing.pmRequestsLimits",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdConfigChanged",
                ) {
                    if let Some(val) = event.get(
                        "beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdConfigChanged",
                    ) {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdConfigChanged".into(),
                            message,
                        })?;
                        event.set("beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdConfigChanged", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_azureADIntegrationDataAuditing_azureAdConfigChanged_to_boolean",
                )?;
                event.remove(
                    "beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdConfigChanged",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdIntegrationEnabled") {
                if let Some(val) = event.get("beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdIntegrationEnabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdIntegrationEnabled".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdIntegrationEnabled", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_azureADIntegrationDataAuditing_azureAdIntegrationEnabled_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdIntegrationEnabled");
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
                if event.has_value("beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdUseCertificateAuth") {
                if let Some(val) = event.get("beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdUseCertificateAuth") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdUseCertificateAuth".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdUseCertificateAuth", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_azureADIntegrationDataAuditing_azureAdUseCertificateAuth_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.azureADIntegrationDataAuditing.azureAdUseCertificateAuth");
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
                if event.has_value("beyondtrust_epm.audit.azureADIntegrationDataAuditing.oldAzureAdIntegrationEnabled") {
                if let Some(val) = event.get("beyondtrust_epm.audit.azureADIntegrationDataAuditing.oldAzureAdIntegrationEnabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.azureADIntegrationDataAuditing.oldAzureAdIntegrationEnabled".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.azureADIntegrationDataAuditing.oldAzureAdIntegrationEnabled", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_azureADIntegrationDataAuditing_oldAzureAdIntegrationEnabled_to_boolean")?;
                event.remove("beyondtrust_epm.audit.azureADIntegrationDataAuditing.oldAzureAdIntegrationEnabled");
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
                if event.has_value("beyondtrust_epm.audit.azureADIntegrationDataAuditing.oldAzureAdUseCertificateAuth") {
                if let Some(val) = event.get("beyondtrust_epm.audit.azureADIntegrationDataAuditing.oldAzureAdUseCertificateAuth") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.azureADIntegrationDataAuditing.oldAzureAdUseCertificateAuth".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.azureADIntegrationDataAuditing.oldAzureAdUseCertificateAuth", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_azureADIntegrationDataAuditing_oldAzureAdUseCertificateAuth_to_boolean")?;
                event.remove("beyondtrust_epm.audit.azureADIntegrationDataAuditing.oldAzureAdUseCertificateAuth");
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
                if event.has_value(
                    "beyondtrust_epm.audit.computerPolicyDataAuditing.deactivatedAgentDeletionDays",
                ) {
                    if let Some(val) = event.get("beyondtrust_epm.audit.computerPolicyDataAuditing.deactivatedAgentDeletionDays") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.computerPolicyDataAuditing.deactivatedAgentDeletionDays".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.computerPolicyDataAuditing.deactivatedAgentDeletionDays", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_computerPolicyDataAuditing_deactivatedAgentDeletionDays_to_long",
                )?;
                event.remove(
                    "beyondtrust_epm.audit.computerPolicyDataAuditing.deactivatedAgentDeletionDays",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("beyondtrust_epm.audit.computerPolicyDataAuditing.enableDeactivatedAgentDeletion") {
                if let Some(val) = event.get("beyondtrust_epm.audit.computerPolicyDataAuditing.enableDeactivatedAgentDeletion") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.computerPolicyDataAuditing.enableDeactivatedAgentDeletion".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.computerPolicyDataAuditing.enableDeactivatedAgentDeletion", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_computerPolicyDataAuditing_enableDeactivatedAgentDeletion_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.computerPolicyDataAuditing.enableDeactivatedAgentDeletion");
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
                if event.has_value("beyondtrust_epm.audit.computerPolicyDataAuditing.inactivityAgentDeactivationDays") {
                if let Some(val) = event.get("beyondtrust_epm.audit.computerPolicyDataAuditing.inactivityAgentDeactivationDays") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.computerPolicyDataAuditing.inactivityAgentDeactivationDays".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.computerPolicyDataAuditing.inactivityAgentDeactivationDays", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_computerPolicyDataAuditing_inactivityAgentDeactivationDays_to_long",
                )?;
                event.remove("beyondtrust_epm.audit.computerPolicyDataAuditing.inactivityAgentDeactivationDays");
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
                if event.has_value("beyondtrust_epm.audit.computerPolicyDataAuditing.oldDeactivatedAgentDeletionDays") {
                if let Some(val) = event.get("beyondtrust_epm.audit.computerPolicyDataAuditing.oldDeactivatedAgentDeletionDays") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.computerPolicyDataAuditing.oldDeactivatedAgentDeletionDays".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.computerPolicyDataAuditing.oldDeactivatedAgentDeletionDays", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_computerPolicyDataAuditing_oldDeactivatedAgentDeletionDays_to_long",
                )?;
                event.remove("beyondtrust_epm.audit.computerPolicyDataAuditing.oldDeactivatedAgentDeletionDays");
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
                if event.has_value("beyondtrust_epm.audit.computerPolicyDataAuditing.oldEnableDeactivatedAgentDeletion") {
                if let Some(val) = event.get("beyondtrust_epm.audit.computerPolicyDataAuditing.oldEnableDeactivatedAgentDeletion") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.computerPolicyDataAuditing.oldEnableDeactivatedAgentDeletion".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.computerPolicyDataAuditing.oldEnableDeactivatedAgentDeletion", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_computerPolicyDataAuditing_oldEnableDeactivatedAgentDeletion_to_boolean")?;
                event.remove("beyondtrust_epm.audit.computerPolicyDataAuditing.oldEnableDeactivatedAgentDeletion");
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
                if event.has_value("beyondtrust_epm.audit.computerPolicyDataAuditing.oldInactivityAgentDeactivationDays") {
                if let Some(val) = event.get("beyondtrust_epm.audit.computerPolicyDataAuditing.oldInactivityAgentDeactivationDays") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.computerPolicyDataAuditing.oldInactivityAgentDeactivationDays".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.computerPolicyDataAuditing.oldInactivityAgentDeactivationDays", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_computerPolicyDataAuditing_oldInactivityAgentDeactivationDays_to_long",
                )?;
                event.remove("beyondtrust_epm.audit.computerPolicyDataAuditing.oldInactivityAgentDeactivationDays");
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
                event.has_value("beyondtrust_epm.audit.created")
                    && event.get_str("beyondtrust_epm.audit.created") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("beyondtrust_epm.audit.created") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("beyondtrust_epm.audit.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "beyondtrust_epm.audit.created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_created")?;
                    event.remove("beyondtrust_epm.audit.created");
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
                .get("beyondtrust_epm.audit.created")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event
                .get("beyondtrust_epm.audit.created")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if let Some(v) = event
                .get("beyondtrust_epm.audit.details")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("beyondtrust_epm.audit.groupDataAuditing.newIsDefault") {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.groupDataAuditing.newIsDefault")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_epm.audit.groupDataAuditing.newIsDefault".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.groupDataAuditing.newIsDefault",
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
                    "convert_groupDataAuditing_newIsDefault_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.groupDataAuditing.newIsDefault");
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
                if event.has_value("beyondtrust_epm.audit.groupDataAuditing.oldIsDefault") {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.groupDataAuditing.oldIsDefault")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_epm.audit.groupDataAuditing.oldIsDefault".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.groupDataAuditing.oldIsDefault",
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
                    "convert_groupDataAuditing_oldIsDefault_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.groupDataAuditing.oldIsDefault");
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

            if event.has_value("beyondtrust_epm.audit.id") {
                if let Some(val) = event.get("beyondtrust_epm.audit.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "beyondtrust_epm.audit.id".into(),
                            message,
                        }
                    })?;
                    event.set("beyondtrust_epm.audit.id", converted)?;
                }
            }

            if let Some(v) = event
                .get("beyondtrust_epm.audit.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("beyondtrust_epm.audit.installationKeyDataAuditing.deleted") {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.installationKeyDataAuditing.deleted")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_epm.audit.installationKeyDataAuditing.deleted"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.installationKeyDataAuditing.deleted",
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
                    "convert_installationKeyDataAuditing_deleted_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.installationKeyDataAuditing.deleted");
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
                if event.has_value("beyondtrust_epm.audit.installationKeyDataAuditing.newDisabled")
                {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.installationKeyDataAuditing.newDisabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "beyondtrust_epm.audit.installationKeyDataAuditing.newDisabled"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.installationKeyDataAuditing.newDisabled",
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
                    "convert_installationKeyDataAuditing_newDisabled_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.installationKeyDataAuditing.newDisabled");
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
                if event.has_value("beyondtrust_epm.audit.installationKeyDataAuditing.oldDisabled")
                {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.installationKeyDataAuditing.oldDisabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "beyondtrust_epm.audit.installationKeyDataAuditing.oldDisabled"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.installationKeyDataAuditing.oldDisabled",
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
                    "convert_installationKeyDataAuditing_oldDisabled_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.installationKeyDataAuditing.oldDisabled");
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
                if event.has_value("beyondtrust_epm.audit.locked") {
                    if let Some(val) = event.get("beyondtrust_epm.audit.locked") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_epm.audit.locked".into(),
                                message,
                            }
                        })?;
                        event.set("beyondtrust_epm.audit.locked", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_locked_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.locked");
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
                if event.has_value("beyondtrust_epm.audit.managementRuleDataAuditing.newPriority") {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.managementRuleDataAuditing.newPriority")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "beyondtrust_epm.audit.managementRuleDataAuditing.newPriority"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.managementRuleDataAuditing.newPriority",
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
                    "convert_managementRuleDataAuditing_newPriority_to_long",
                )?;
                event.remove("beyondtrust_epm.audit.managementRuleDataAuditing.newPriority");
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
                if event.has_value("beyondtrust_epm.audit.managementRuleDataAuditing.oldPriority") {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.managementRuleDataAuditing.oldPriority")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "beyondtrust_epm.audit.managementRuleDataAuditing.oldPriority"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.managementRuleDataAuditing.oldPriority",
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
                    "convert_managementRuleDataAuditing_oldPriority_to_long",
                )?;
                event.remove("beyondtrust_epm.audit.managementRuleDataAuditing.oldPriority");
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
                if event.has_value("beyondtrust_epm.audit.mmcRemoteClientDataAuditing.enabled") {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.mmcRemoteClientDataAuditing.enabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_epm.audit.mmcRemoteClientDataAuditing.enabled"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.mmcRemoteClientDataAuditing.enabled",
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
                    "convert_mmcRemoteClientDataAuditing_enabled_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.mmcRemoteClientDataAuditing.enabled");
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
                if event.has_value("beyondtrust_epm.audit.mmcRemoteClientDataAuditing.oldEnabled") {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.mmcRemoteClientDataAuditing.oldEnabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "beyondtrust_epm.audit.mmcRemoteClientDataAuditing.oldEnabled"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.mmcRemoteClientDataAuditing.oldEnabled",
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
                    "convert_mmcRemoteClientDataAuditing_oldEnabled_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.mmcRemoteClientDataAuditing.oldEnabled");
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
                if event.has_value("beyondtrust_epm.audit.openIdConfigDataAuditing.secretUpdated") {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.openIdConfigDataAuditing.secretUpdated")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "beyondtrust_epm.audit.openIdConfigDataAuditing.secretUpdated"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.openIdConfigDataAuditing.secretUpdated",
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
                    "convert_openIdConfigDataAuditing_secretUpdated_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.openIdConfigDataAuditing.secretUpdated");
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
                if event.has_value("beyondtrust_epm.audit.reputationSettingsDataAuditing.oldReputationIntegrationEnabled") {
                if let Some(val) = event.get("beyondtrust_epm.audit.reputationSettingsDataAuditing.oldReputationIntegrationEnabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.reputationSettingsDataAuditing.oldReputationIntegrationEnabled".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.reputationSettingsDataAuditing.oldReputationIntegrationEnabled", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_reputationSettingsDataAuditing_oldReputationIntegrationEnabled_to_boolean")?;
                event.remove("beyondtrust_epm.audit.reputationSettingsDataAuditing.oldReputationIntegrationEnabled");
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
                if event.has_value(
                    "beyondtrust_epm.audit.reputationSettingsDataAuditing.reputationConfigChanged",
                ) {
                    if let Some(val) = event.get("beyondtrust_epm.audit.reputationSettingsDataAuditing.reputationConfigChanged") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.reputationSettingsDataAuditing.reputationConfigChanged".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.reputationSettingsDataAuditing.reputationConfigChanged", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_reputationSettingsDataAuditing_reputationConfigChanged_to_boolean",
                )?;
                event.remove(
                    "beyondtrust_epm.audit.reputationSettingsDataAuditing.reputationConfigChanged",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("beyondtrust_epm.audit.reputationSettingsDataAuditing.reputationIntegrationEnabled") {
                if let Some(val) = event.get("beyondtrust_epm.audit.reputationSettingsDataAuditing.reputationIntegrationEnabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.reputationSettingsDataAuditing.reputationIntegrationEnabled".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.reputationSettingsDataAuditing.reputationIntegrationEnabled", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_reputationSettingsDataAuditing_reputationIntegrationEnabled_to_boolean")?;
                event.remove("beyondtrust_epm.audit.reputationSettingsDataAuditing.reputationIntegrationEnabled");
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
                if event
                    .has_value("beyondtrust_epm.audit.securitySettingsDataAuditing.oldTokenTimeout")
                {
                    if let Some(val) = event
                        .get("beyondtrust_epm.audit.securitySettingsDataAuditing.oldTokenTimeout")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.securitySettingsDataAuditing.oldTokenTimeout".into(),
                            message,
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.securitySettingsDataAuditing.oldTokenTimeout",
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
                    "convert_securitySettingsDataAuditing_oldTokenTimeout_to_long",
                )?;
                event.remove("beyondtrust_epm.audit.securitySettingsDataAuditing.oldTokenTimeout");
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
                if event
                    .has_value("beyondtrust_epm.audit.securitySettingsDataAuditing.tokenTimeout")
                {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.securitySettingsDataAuditing.tokenTimeout")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.securitySettingsDataAuditing.tokenTimeout".into(),
                            message,
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.securitySettingsDataAuditing.tokenTimeout",
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
                    "convert_securitySettingsDataAuditing_tokenTimeout_to_long",
                )?;
                event.remove("beyondtrust_epm.audit.securitySettingsDataAuditing.tokenTimeout");
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
                if event.has_value(
                    "beyondtrust_epm.audit.siemIntegrationBaseDetailModel.siemIntegrationEnabled",
                ) {
                    if let Some(val) = event.get("beyondtrust_epm.audit.siemIntegrationBaseDetailModel.siemIntegrationEnabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.siemIntegrationBaseDetailModel.siemIntegrationEnabled".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.siemIntegrationBaseDetailModel.siemIntegrationEnabled", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_siemIntegrationBaseDetailModel_siemIntegrationEnabled_to_boolean",
                )?;
                event.remove(
                    "beyondtrust_epm.audit.siemIntegrationBaseDetailModel.siemIntegrationEnabled",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "beyondtrust_epm.audit.siemIntegrationQradarAuditing.siemIntegrationEnabled",
                ) {
                    if let Some(val) = event.get("beyondtrust_epm.audit.siemIntegrationQradarAuditing.siemIntegrationEnabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.siemIntegrationQradarAuditing.siemIntegrationEnabled".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.siemIntegrationQradarAuditing.siemIntegrationEnabled", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_siemIntegrationQradarAuditing_siemIntegrationEnabled_to_boolean",
                )?;
                event.remove(
                    "beyondtrust_epm.audit.siemIntegrationQradarAuditing.siemIntegrationEnabled",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "beyondtrust_epm.audit.siemIntegrationS3Auditing.siemIntegrationEnabled",
                ) {
                    if let Some(val) = event.get(
                        "beyondtrust_epm.audit.siemIntegrationS3Auditing.siemIntegrationEnabled",
                    ) {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.siemIntegrationS3Auditing.siemIntegrationEnabled".into(),
                            message,
                        })?;
                        event.set("beyondtrust_epm.audit.siemIntegrationS3Auditing.siemIntegrationEnabled", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_siemIntegrationS3Auditing_siemIntegrationEnabled_to_boolean",
                )?;
                event.remove(
                    "beyondtrust_epm.audit.siemIntegrationS3Auditing.siemIntegrationEnabled",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("beyondtrust_epm.audit.siemIntegrationS3Auditing.siemSseEnabled")
                {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.siemIntegrationS3Auditing.siemSseEnabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "beyondtrust_epm.audit.siemIntegrationS3Auditing.siemSseEnabled"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.siemIntegrationS3Auditing.siemSseEnabled",
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
                    "convert_siemIntegrationS3Auditing_siemSseEnabled_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.siemIntegrationS3Auditing.siemSseEnabled");
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
                if event.has_value(
                    "beyondtrust_epm.audit.siemIntegrationSentinelAuditing.siemIntegrationEnabled",
                ) {
                    if let Some(val) = event.get("beyondtrust_epm.audit.siemIntegrationSentinelAuditing.siemIntegrationEnabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.siemIntegrationSentinelAuditing.siemIntegrationEnabled".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.siemIntegrationSentinelAuditing.siemIntegrationEnabled", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_siemIntegrationSentinelAuditing_siemIntegrationEnabled_to_boolean",
                )?;
                event.remove(
                    "beyondtrust_epm.audit.siemIntegrationSentinelAuditing.siemIntegrationEnabled",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "beyondtrust_epm.audit.siemIntegrationSplunkAuditing.siemIntegrationEnabled",
                ) {
                    if let Some(val) = event.get("beyondtrust_epm.audit.siemIntegrationSplunkAuditing.siemIntegrationEnabled") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "beyondtrust_epm.audit.siemIntegrationSplunkAuditing.siemIntegrationEnabled".into(),
                            message,
                        })?;
                    event.set("beyondtrust_epm.audit.siemIntegrationSplunkAuditing.siemIntegrationEnabled", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_siemIntegrationSplunkAuditing_siemIntegrationEnabled_to_boolean",
                )?;
                event.remove(
                    "beyondtrust_epm.audit.siemIntegrationSplunkAuditing.siemIntegrationEnabled",
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

            if let Some(v) = event
                .get("beyondtrust_epm.audit.user")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = {
                event.has_value("beyondtrust_epm.audit.userDataAuditing.deletedAt")
                    && event.get_str("beyondtrust_epm.audit.userDataAuditing.deletedAt") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("beyondtrust_epm.audit.userDataAuditing.deletedAt")
                    {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event
                                .set("beyondtrust_epm.audit.userDataAuditing.deletedAt", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "beyondtrust_epm.audit.userDataAuditing.deletedAt".into(),
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
                        "date_userDataAuditing_deletedAt",
                    )?;
                    event.remove("beyondtrust_epm.audit.userDataAuditing.deletedAt");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("beyondtrust_epm.audit.userDataAuditing.newDisabled") {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.userDataAuditing.newDisabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_epm.audit.userDataAuditing.newDisabled".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.userDataAuditing.newDisabled",
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
                    "convert_userDataAuditing_newDisabled_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.userDataAuditing.newDisabled");
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
                if event.has_value("beyondtrust_epm.audit.userDataAuditing.oldDisabled") {
                    if let Some(val) =
                        event.get("beyondtrust_epm.audit.userDataAuditing.oldDisabled")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "beyondtrust_epm.audit.userDataAuditing.oldDisabled".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "beyondtrust_epm.audit.userDataAuditing.oldDisabled",
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
                    "convert_userDataAuditing_oldDisabled_to_boolean",
                )?;
                event.remove("beyondtrust_epm.audit.userDataAuditing.oldDisabled");
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
                .get("beyondtrust_epm.audit.userId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            let _cond = { event.has_value("beyondtrust_epm.audit.userId") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("beyondtrust_epm.audit.userId")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondtrust_epm.audit.user") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("beyondtrust_epm.audit.user")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("beyondtrust_epm.audit.changedBy") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("beyondtrust_epm.audit.changedBy")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("beyondtrust_epm.audit.details");
            event.remove("beyondtrust_epm.audit.id");
            event.remove("beyondtrust_epm.audit.user");
            event.remove("beyondtrust_epm.audit.userId");
            event.remove("beyondtrust_epm.audit.created");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
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

            // Painless script
            // Source: String camelToSnake(String str) {\n  StringBuilder result = new StringBuilder();\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0) {\n        char prev = str.charAt(i - 1);\n        boolean nextIsLower = (i + 1 < str.length()) && Character.isLowerCase(str.charAt(i + 1));\n        boolean prevIsDigit = Character.isDigit(prev);\n        if (Character.isLowerCase(prev) || prevIsDigit || (Character.isUpperCase(prev) && nextIsLower)) {\n          result.append('_');\n        }\n      }\n      result.append(Character.toLowerCase(c));\n    } else {\n      result.append(c);\n    }\n  }\n  return result.toString();\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      String newKey = camelToSnake(entry.getKey());\n      newObj[newKey] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\nctx.beyondtrust_epm = ctx.beyondtrust_epm ?: [:];\nif (ctx.beyondtrust_epm?.audit != null) {\n  ctx.beyondtrust_epm.audit = convertToSnakeCase(ctx.beyondtrust_epm.audit);\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"String camelToSnake(String str) {\n  StringBuilder result = new StringBuilder();\n  for (int i = 0; i < str.length(); i++) {\n    char c = str.charAt(i);\n    if (Character.isUpperCase(c)) {\n      if (i > 0) {\n        char prev = str.charAt(i - 1);\n        boolean nextIsLower = (i + 1 < str.length()) && Character.isLowerCase(str.charAt(i + 1));\n        boolean prevIsDigit = Character.isDigit(prev);\n        if (Character.isLowerCase(prev) || prevIsDigit || (Character.isUpperCase(prev) && nextIsLower)) {\n          result.append('_');\n        }\n      }\n      result.append(Character.toLowerCase(c));\n    } else {\n      result.append(c);\n    }\n  }\n  return result.toString();\n}\ndef convertToSnakeCase(def obj) {\n  if (obj instanceof Map) {\n    def newObj = [:];\n    for (entry in obj.entrySet()) {\n      String newKey = camelToSnake(entry.getKey());\n      newObj[newKey] = convertToSnakeCase(entry.getValue());\n    }\n    return newObj;\n  } else if (obj instanceof List) {\n    def newList = [];\n    for (item in obj) {\n      newList.add(convertToSnakeCase(item));\n    }\n    return newList;\n  } else {\n    return obj;\n  }\n}\nctx.beyondtrust_epm = ctx.beyondtrust_epm ?: [:];\nif (ctx.beyondtrust_epm?.audit != null) {\n  ctx.beyondtrust_epm.audit = convertToSnakeCase(ctx.beyondtrust_epm.audit);\n}"#
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
