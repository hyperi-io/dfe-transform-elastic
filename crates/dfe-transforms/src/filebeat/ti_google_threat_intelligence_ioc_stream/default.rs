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
            let _cond = { event.get_str("message") == Some("retry") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

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

            event.set("event.kind", json!("enrichment"))?;

            event.append("event.category", json!("threat"))?;

            event.append("event.type", json!("indicator"))?;

            event.set("observer.vendor", json!("Google"))?;

            event.set("observer.product", json!("Threat Intelligence"))?;

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
                if let Some(v) = event.get("json.context_attributes.notification_date") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.context_attributes.notification_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.attributes.as_owner") {
                event.rename(
                    "json.attributes.as_owner",
                    "gti.ioc_stream.attributes.as_owner",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.asn") {
                    if let Some(val) = event.get("json.attributes.asn") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.asn".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.asn", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_asn_to_long",
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
                if event.has_value(
                    "json.attributes.gti_assessment.contributing_factors.gavs_detections",
                ) {
                    if let Some(val) = event
                        .get("json.attributes.gti_assessment.contributing_factors.gavs_detections")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.gti_assessment.contributing_factors.gavs_detections".into(),
                            message,
                        })?;
                        event.set("gti.ioc_stream.attributes.assessment.contributing_factors.gavs_detections", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_gti_assessment_contributing_factors_gavs_detections",
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
                if event.has_value(
                    "json.attributes.gti_assessment.contributing_factors.malicious_sandbox_verdict",
                ) {
                    if let Some(val) = event.get("json.attributes.gti_assessment.contributing_factors.malicious_sandbox_verdict") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.gti_assessment.contributing_factors.malicious_sandbox_verdict".into(),
                            message,
                        })?;
                    event.set("gti.ioc_stream.attributes.assessment.contributing_factors.malicious_sandbox_verdict", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attributes_gti_assessment_contributing_factors_malicious_sandbox_verdict_to_boolean")?;
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
                if event.has_value("json.attributes.gti_assessment.contributing_factors.mandiant_analyst_malicious") {
                if let Some(val) = event.get("json.attributes.gti_assessment.contributing_factors.mandiant_analyst_malicious") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.gti_assessment.contributing_factors.mandiant_analyst_malicious".into(),
                            message,
                        })?;
                    event.set("gti.ioc_stream.attributes.assessment.contributing_factors.mandiant_analyst_malicious", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attributes_gti_assessment_contributing_factors_mandiant_analyst_malicious_to_boolean")?;
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
                if event.has_value("json.attributes.gti_assessment.contributing_factors.mandiant_analyst_observed_recent") {
                if let Some(val) = event.get("json.attributes.gti_assessment.contributing_factors.mandiant_analyst_observed_recent") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.gti_assessment.contributing_factors.mandiant_analyst_observed_recent".into(),
                            message,
                        })?;
                    event.set("gti.ioc_stream.attributes.assessment.contributing_factors.mandiant_analyst_observed_recent", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attributes_gti_assessment_contributing_factors_mandiant_analyst_observed_recent_to_boolean")?;
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
                if event.has_value(
                    "json.attributes.gti_assessment.contributing_factors.mandiant_confidence_score",
                ) {
                    if let Some(val) = event.get("json.attributes.gti_assessment.contributing_factors.mandiant_confidence_score") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.gti_assessment.contributing_factors.mandiant_confidence_score".into(),
                            message,
                        })?;
                    event.set("gti.ioc_stream.attributes.assessment.contributing_factors.mandiant_confidence_score", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attributes_gti_assessment_contributing_factors_mandiant_confidence_score_to_long")?;
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

            if event.has_value(
                "json.attributes.gti_assessment.contributing_factors.normalised_categories",
            ) {
                event.rename("json.attributes.gti_assessment.contributing_factors.normalised_categories", "gti.ioc_stream.attributes.assessment.contributing_factors.normalised_categories")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "json.attributes.gti_assessment.contributing_factors.pervasive_indicator",
                ) {
                    if let Some(val) = event.get(
                        "json.attributes.gti_assessment.contributing_factors.pervasive_indicator",
                    ) {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.gti_assessment.contributing_factors.pervasive_indicator".into(),
                            message,
                        })?;
                        event.set("gti.ioc_stream.attributes.assessment.contributing_factors.pervasive_indicator", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attributes_gti_assessment_contributing_factors_pervasive_indicator_to_boolean")?;
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

            if event.has_value(
                "json.attributes.gti_assessment.contributing_factors.safebrowsing_verdict",
            ) {
                event.rename("json.attributes.gti_assessment.contributing_factors.safebrowsing_verdict", "gti.ioc_stream.attributes.assessment.contributing_factors.safebrowsing_verdict")?;
            }

            if event.has_value("json.attributes.gti_assessment.description") {
                event.rename(
                    "json.attributes.gti_assessment.description",
                    "gti.ioc_stream.attributes.assessment.description",
                )?;
            }

            if event.has_value("json.attributes.gti_assessment.severity.value") {
                event.rename(
                    "json.attributes.gti_assessment.severity.value",
                    "gti.ioc_stream.attributes.assessment.severity",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.gti_assessment.threat_score.value") {
                    if let Some(val) =
                        event.get("json.attributes.gti_assessment.threat_score.value")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.gti_assessment.threat_score.value".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.assessment.threat_score",
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
                    "convert_attributes_gti_assessment_threat_score_value_to_long",
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

            if event.has_value("json.attributes.gti_assessment.verdict.value") {
                event.rename(
                    "json.attributes.gti_assessment.verdict.value",
                    "gti.ioc_stream.attributes.assessment.verdict",
                )?;
            }

            if event.has_value("json.attributes.authentihash") {
                event.rename(
                    "json.attributes.authentihash",
                    "gti.ioc_stream.attributes.authentihash",
                )?;
            }

            if event.has_value("json.attributes.autostart_locations") {
                event.rename(
                    "json.attributes.autostart_locations",
                    "gti.ioc_stream.attributes.autostart_locations",
                )?;
            }

            if event.has_value("json.attributes.available_tools") {
                event.rename(
                    "json.attributes.available_tools",
                    "gti.ioc_stream.attributes.available_tools",
                )?;
            }

            if event.has_value("json.attributes.categories.BitDefender") {
                event.rename(
                    "json.attributes.categories.BitDefender",
                    "gti.ioc_stream.attributes.vendor_categories.bitdefender",
                )?;
            }

            if event.has_value("json.attributes.categories.Sophos") {
                event.rename(
                    "json.attributes.categories.Sophos",
                    "gti.ioc_stream.attributes.vendor_categories.sophos",
                )?;
            }

            if event.has_value("json.attributes.categories.Webroot") {
                event.rename(
                    "json.attributes.categories.Webroot",
                    "gti.ioc_stream.attributes.vendor_categories.webroot",
                )?;
            }

            if event.has_value("json.attributes.continent") {
                event.rename(
                    "json.attributes.continent",
                    "gti.ioc_stream.attributes.continent",
                )?;
            }

            if event.has_value("json.attributes.country") {
                event.rename(
                    "json.attributes.country",
                    "gti.ioc_stream.attributes.country",
                )?;
            }

            let _cond = {
                event.has_value("json.attributes.creation_date")
                    && event.get_str("json.attributes.creation_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.attributes.creation_date") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("gti.ioc_stream.attributes.creation_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.creation_date".into(),
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
                        "date_attributes_creation_date",
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

            if event.has_value("json.attributes.crowdsourced_ai_results") {
                event.rename(
                    "json.attributes.crowdsourced_ai_results",
                    "gti.ioc_stream.attributes.crowdsourced_ai_results",
                )?;
            }

            if event.has_value("json.attributes.crowdsourced_ids_results") {
                event.rename(
                    "json.attributes.crowdsourced_ids_results",
                    "gti.ioc_stream.attributes.crowdsourced_ids_results",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.crowdsourced_ids_stats.high") {
                    if let Some(val) = event.get("json.attributes.crowdsourced_ids_stats.high") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.crowdsourced_ids_stats.high".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.crowdsourced_ids_stats.high",
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
                    "convert_attributes_crowdsourced_ids_stats_high_to_long",
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
                if event.has_value("json.attributes.crowdsourced_ids_stats.info") {
                    if let Some(val) = event.get("json.attributes.crowdsourced_ids_stats.info") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.crowdsourced_ids_stats.info".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.crowdsourced_ids_stats.info",
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
                    "convert_attributes_crowdsourced_ids_stats_info_to_long",
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
                if event.has_value("json.attributes.crowdsourced_ids_stats.low") {
                    if let Some(val) = event.get("json.attributes.crowdsourced_ids_stats.low") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.crowdsourced_ids_stats.low".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.crowdsourced_ids_stats.low",
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
                    "convert_attributes_crowdsourced_ids_stats_low_to_long",
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
                if event.has_value("json.attributes.crowdsourced_ids_stats.medium") {
                    if let Some(val) = event.get("json.attributes.crowdsourced_ids_stats.medium") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.crowdsourced_ids_stats.medium".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.crowdsourced_ids_stats.medium",
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
                    "convert_attributes_crowdsourced_ids_stats_medium_to_long",
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
                event
                    .get("json.attributes.crowdsourced_yara_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("json.attributes.crowdsourced_yara_results")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                                    event.get_as_string("_ingest._value.match_date")
                                {
                                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.match_date", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.match_date".into(),
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
                                    "date_attributes_crowdsourced_yara_results_match_date",
                                )?;
                                event.remove("_ingest._value.match_date");
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
                            "json.attributes.crowdsourced_yara_results",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            if event.has_value("json.attributes.crowdsourced_yara_results") {
                event.rename(
                    "json.attributes.crowdsourced_yara_results",
                    "gti.ioc_stream.attributes.crowdsourced_yara_results",
                )?;
            }

            if event.has_value("json.attributes.detectiteasy.filetype") {
                event.rename(
                    "json.attributes.detectiteasy.filetype",
                    "gti.ioc_stream.attributes.detectiteasy.filetype",
                )?;
            }

            if event.has_value("json.attributes.detectiteasy.values") {
                event.rename(
                    "json.attributes.detectiteasy.values",
                    "gti.ioc_stream.attributes.detectiteasy.values",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.downloadable") {
                    if let Some(val) = event.get("json.attributes.downloadable") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.downloadable".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.downloadable", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_downloadable_to_boolean",
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

            if event.has_value("json.attributes.exiftool.CharacterSet") {
                event.rename(
                    "json.attributes.exiftool.CharacterSet",
                    "gti.ioc_stream.attributes.exiftool.character_set",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.exiftool.CodeSize") {
                    if let Some(val) = event.get("json.attributes.exiftool.CodeSize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.exiftool.CodeSize".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.exiftool.code_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_exiftool_CodeSize_to_long",
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

            if event.has_value("json.attributes.exiftool.CompanyName") {
                event.rename(
                    "json.attributes.exiftool.CompanyName",
                    "gti.ioc_stream.attributes.exiftool.company_name",
                )?;
            }

            if event.has_value("json.attributes.exiftool.EntryPoint") {
                event.rename(
                    "json.attributes.exiftool.EntryPoint",
                    "gti.ioc_stream.attributes.exiftool.entry_point",
                )?;
            }

            if event.has_value("json.attributes.exiftool.FileDescription") {
                event.rename(
                    "json.attributes.exiftool.FileDescription",
                    "gti.ioc_stream.attributes.exiftool.file.description",
                )?;
            }

            if event.has_value("json.attributes.exiftool.FileFlagsMask") {
                event.rename(
                    "json.attributes.exiftool.FileFlagsMask",
                    "gti.ioc_stream.attributes.exiftool.file.flags_mask",
                )?;
            }

            if event.has_value("json.attributes.exiftool.FileOS") {
                event.rename(
                    "json.attributes.exiftool.FileOS",
                    "gti.ioc_stream.attributes.exiftool.file.os",
                )?;
            }

            if event.has_value("json.attributes.exiftool.FileSubtype") {
                event.rename(
                    "json.attributes.exiftool.FileSubtype",
                    "gti.ioc_stream.attributes.exiftool.file.subtype",
                )?;
            }

            if event.has_value("json.attributes.exiftool.FileType") {
                event.rename(
                    "json.attributes.exiftool.FileType",
                    "gti.ioc_stream.attributes.exiftool.file.type",
                )?;
            }

            if event.has_value("json.attributes.exiftool.FileTypeExtension") {
                event.rename(
                    "json.attributes.exiftool.FileTypeExtension",
                    "gti.ioc_stream.attributes.exiftool.file.type_extension",
                )?;
            }

            if event.has_value("json.attributes.exiftool.FileVersion") {
                event.rename(
                    "json.attributes.exiftool.FileVersion",
                    "gti.ioc_stream.attributes.exiftool.file.version",
                )?;
            }

            if event.has_value("json.attributes.exiftool.FileVersionNumber") {
                event.rename(
                    "json.attributes.exiftool.FileVersionNumber",
                    "gti.ioc_stream.attributes.exiftool.file.version_number",
                )?;
            }

            if event.has_value("json.attributes.exiftool.ImageFileCharacteristics") {
                event.rename(
                    "json.attributes.exiftool.ImageFileCharacteristics",
                    "gti.ioc_stream.attributes.exiftool.image.file_characteristics",
                )?;
            }

            if event.has_value("json.attributes.exiftool.ImageVersion") {
                event.rename(
                    "json.attributes.exiftool.ImageVersion",
                    "gti.ioc_stream.attributes.exiftool.image.version",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.exiftool.InitializedDataSize") {
                    if let Some(val) = event.get("json.attributes.exiftool.InitializedDataSize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.exiftool.InitializedDataSize".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.exiftool.initialized_data_size",
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
                    "convert_attributes_exiftool_InitializedDataSize_to_long",
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

            if event.has_value("json.attributes.exiftool.InternalName") {
                event.rename(
                    "json.attributes.exiftool.InternalName",
                    "gti.ioc_stream.attributes.exiftool.internal_name",
                )?;
            }

            if event.has_value("json.attributes.exiftool.LanguageCode") {
                event.rename(
                    "json.attributes.exiftool.LanguageCode",
                    "gti.ioc_stream.attributes.exiftool.language_code",
                )?;
            }

            if event.has_value("json.attributes.exiftool.LegalCopyright") {
                event.rename(
                    "json.attributes.exiftool.LegalCopyright",
                    "gti.ioc_stream.attributes.exiftool.legal_copyright",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.exiftool.LineCount") {
                    if let Some(val) = event.get("json.attributes.exiftool.LineCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.exiftool.LineCount".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.exiftool.line_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_exiftool_LineCount_to_long",
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

            if event.has_value("json.attributes.exiftool.LinkerVersion") {
                event.rename(
                    "json.attributes.exiftool.LinkerVersion",
                    "gti.ioc_stream.attributes.exiftool.linker_version",
                )?;
            }

            if event.has_value("json.attributes.exiftool.MachineType") {
                event.rename(
                    "json.attributes.exiftool.MachineType",
                    "gti.ioc_stream.attributes.exiftool.machine_type",
                )?;
            }

            if event.has_value("json.attributes.exiftool.MIMEEncoding") {
                event.rename(
                    "json.attributes.exiftool.MIMEEncoding",
                    "gti.ioc_stream.attributes.exiftool.mime.encoding",
                )?;
            }

            if event.has_value("json.attributes.exiftool.MIMEType") {
                event.rename(
                    "json.attributes.exiftool.MIMEType",
                    "gti.ioc_stream.attributes.exiftool.mime.type",
                )?;
            }

            if let Some(v) = event
                .get("gti.ioc_stream.attributes.exiftool.mime.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.mime_type", v)?;
            }

            if event.has_value("json.attributes.exiftool.Newlines") {
                event.rename(
                    "json.attributes.exiftool.Newlines",
                    "gti.ioc_stream.attributes.exiftool.newlines",
                )?;
            }

            if event.has_value("json.attributes.exiftool.ObjectFileType") {
                event.rename(
                    "json.attributes.exiftool.ObjectFileType",
                    "gti.ioc_stream.attributes.exiftool.object_file_type",
                )?;
            }

            if event.has_value("json.attributes.exiftool.OriginalFileName") {
                event.rename(
                    "json.attributes.exiftool.OriginalFileName",
                    "gti.ioc_stream.attributes.exiftool.original_file_name",
                )?;
            }

            if event.has_value("json.attributes.exiftool.OSVersion") {
                event.rename(
                    "json.attributes.exiftool.OSVersion",
                    "gti.ioc_stream.attributes.exiftool.os_version",
                )?;
            }

            if event.has_value("json.attributes.exiftool.PEType") {
                event.rename(
                    "json.attributes.exiftool.PEType",
                    "gti.ioc_stream.attributes.exiftool.pe_type",
                )?;
            }

            if event.has_value("json.attributes.exiftool.ProductName") {
                event.rename(
                    "json.attributes.exiftool.ProductName",
                    "gti.ioc_stream.attributes.exiftool.product.name",
                )?;
            }

            if event.has_value("json.attributes.exiftool.ProductVersion") {
                event.rename(
                    "json.attributes.exiftool.ProductVersion",
                    "gti.ioc_stream.attributes.exiftool.product.version",
                )?;
            }

            if event.has_value("json.attributes.exiftool.ProductVersionNumber") {
                event.rename(
                    "json.attributes.exiftool.ProductVersionNumber",
                    "gti.ioc_stream.attributes.exiftool.product.version_number",
                )?;
            }

            if event.has_value("json.attributes.exiftool.Subsystem") {
                event.rename(
                    "json.attributes.exiftool.Subsystem",
                    "gti.ioc_stream.attributes.exiftool.subsystem",
                )?;
            }

            if event.has_value("json.attributes.exiftool.SubsystemVersion") {
                event.rename(
                    "json.attributes.exiftool.SubsystemVersion",
                    "gti.ioc_stream.attributes.exiftool.subsystem_version",
                )?;
            }

            let _cond = {
                event.get_str("json.attributes.exiftool.TimeStamp") == Some("0000:00:00 00:00:00")
            };
            if _cond {
                event.remove("json.attributes.exiftool.TimeStamp");
            }

            let _cond = {
                event.has_value("json.attributes.exiftool.TimeStamp")
                    && event.get_str("json.attributes.exiftool.TimeStamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.attributes.exiftool.TimeStamp")
                    {
                        match parse_date_out(&date_str, &["yyyy:MM:dd HH:mm:ssXXX"], None, None) {
                            Some(parsed) => {
                                event.set("gti.ioc_stream.attributes.exiftool.timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.exiftool.TimeStamp".into(),
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
                        "date_attributes_exiftool_TimeStamp",
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
                .get("gti.ioc_stream.attributes.exiftool.timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.mtime", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.exiftool.UninitializedDataSize") {
                    if let Some(val) = event.get("json.attributes.exiftool.UninitializedDataSize") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.exiftool.UninitializedDataSize".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.exiftool.uninitialized_data_size",
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
                    "convert_attributes_exiftool_UninitializedDataSize_to_long",
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
                if event.has_value("json.attributes.exiftool.WordCount") {
                    if let Some(val) = event.get("json.attributes.exiftool.WordCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.exiftool.WordCount".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.exiftool.word_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_exiftool_WordCount_to_long",
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
                if event.has_value("json.attributes.last_analysis_stats.failure") {
                    if let Some(val) = event.get("json.attributes.last_analysis_stats.failure") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.last_analysis_stats.failure".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.failure", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_last_analysis_stats_failure_to_long",
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

            if event.has_value("json.attributes.favicon.dhash") {
                event.rename(
                    "json.attributes.favicon.dhash",
                    "gti.ioc_stream.attributes.favicon.dhash",
                )?;
            }

            if event.has_value("json.attributes.favicon.raw_md5") {
                event.rename(
                    "json.attributes.favicon.raw_md5",
                    "gti.ioc_stream.attributes.favicon.raw_md5",
                )?;
            }

            if event.has_value("json.attributes.filecondis.dhash") {
                event.rename(
                    "json.attributes.filecondis.dhash",
                    "gti.ioc_stream.attributes.filecondis.dhash",
                )?;
            }

            if event.has_value("json.attributes.filecondis.raw_md5") {
                event.rename(
                    "json.attributes.filecondis.raw_md5",
                    "gti.ioc_stream.attributes.filecondis.raw_md5",
                )?;
            }

            let _cond = {
                event.has_value("json.attributes.first_seen_itw_date")
                    && event.get_str("json.attributes.first_seen_itw_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.attributes.first_seen_itw_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event
                                .set("gti.ioc_stream.attributes.first_seen_itw_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.first_seen_itw_date".into(),
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
                        "date_attributes_first_seen_itw_date",
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
                .get("gti.ioc_stream.attributes.first_seen_itw_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.first_seen", v)?;
            }

            let _cond = {
                event.has_value("json.attributes.first_submission_date")
                    && event.get_str("json.attributes.first_submission_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.attributes.first_submission_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event
                                .set("gti.ioc_stream.attributes.first_submission_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.first_submission_date".into(),
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
                        "date_attributes_first_submission_date",
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
                if event.has_value("json.attributes.has_content") {
                    if let Some(val) = event.get("json.attributes.has_content") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.has_content".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.has_content", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_has_content_to_boolean",
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

            if event.has_value("json.attributes.jarm") {
                event.rename("json.attributes.jarm", "gti.ioc_stream.attributes.jarm")?;
            }

            let _cond = {
                event.has_value("json.attributes.last_analysis_date")
                    && event.get_str("json.attributes.last_analysis_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.attributes.last_analysis_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("gti.ioc_stream.attributes.last_analysis_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.last_analysis_date".into(),
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
                        "date_attributes_last_analysis_date",
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
                if event.has_value("json.attributes.last_analysis_stats.confirmed-timeout") {
                    if let Some(val) =
                        event.get("json.attributes.last_analysis_stats.confirmed-timeout")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.last_analysis_stats.confirmed-timeout"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.last_analysis_stats.confirmed_timeout",
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
                    "convert_attributes_last_analysis_stats_confirmed-timeout_to_long",
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
                if event.has_value("json.attributes.last_analysis_stats.harmless") {
                    if let Some(val) = event.get("json.attributes.last_analysis_stats.harmless") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.last_analysis_stats.harmless".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.last_analysis_stats.harmless",
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
                    "convert_attributes_last_analysis_stats_harmless_to_long",
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
                if event.has_value("json.attributes.last_analysis_stats.malicious") {
                    if let Some(val) = event.get("json.attributes.last_analysis_stats.malicious") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.last_analysis_stats.malicious".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.last_analysis_stats.malicious",
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
                    "convert_attributes_last_analysis_stats_malicious_to_long",
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
                if event.has_value("json.attributes.last_analysis_stats.suspicious") {
                    if let Some(val) = event.get("json.attributes.last_analysis_stats.suspicious") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.last_analysis_stats.suspicious".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.last_analysis_stats.suspicious",
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
                    "convert_attributes_last_analysis_stats_suspicious_to_long",
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
                if event.has_value("json.attributes.last_analysis_stats.timeout") {
                    if let Some(val) = event.get("json.attributes.last_analysis_stats.timeout") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.last_analysis_stats.timeout".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.last_analysis_stats.timeout",
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
                    "convert_attributes_last_analysis_stats_timeout_to_long",
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
                if event.has_value("json.attributes.last_analysis_stats.undetected") {
                    if let Some(val) = event.get("json.attributes.last_analysis_stats.undetected") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.last_analysis_stats.undetected".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.last_analysis_stats.undetected",
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
                    "convert_attributes_last_analysis_stats_undetected_to_long",
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

            if event.has_value("json.attributes.last_dns_records") {
                event.rename(
                    "json.attributes.last_dns_records",
                    "gti.ioc_stream.attributes.last_dns_records",
                )?;
            }

            let _cond = {
                event.has_value("json.attributes.last_dns_records_date")
                    && event.get_str("json.attributes.last_dns_records_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.attributes.last_dns_records_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event
                                .set("gti.ioc_stream.attributes.last_dns_records_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.last_dns_records_date".into(),
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
                        "date_attributes_last_dns_records_date",
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

            if event.has_value("json.attributes.last_final_url") {
                event.rename(
                    "json.attributes.last_final_url",
                    "gti.ioc_stream.attributes.last_final_url",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.last_http_response_code") {
                    if let Some(val) = event.get("json.attributes.last_http_response_code") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.last_http_response_code".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.last_http_response_code",
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
                    "convert_attributes_last_http_response_code_to_long",
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
                .get("gti.ioc_stream.attributes.last_http_response_code")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("http.response.status_code", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.last_http_response_content_length") {
                    if let Some(val) =
                        event.get("json.attributes.last_http_response_content_length")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.last_http_response_content_length".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.last_http_response_content_length",
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
                    "convert_attributes_last_http_response_content_length_to_long",
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

            if event.has_value("json.attributes.last_http_response_content_sha256") {
                event.rename(
                    "json.attributes.last_http_response_content_sha256",
                    "gti.ioc_stream.attributes.last_http_response_content_sha256",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.cert_signature.signature") {
                event.rename(
                    "json.attributes.last_https_certificate.cert_signature.signature",
                    "gti.ioc_stream.attributes.last_https_certificate.cert_signature.signature",
                )?;
            }

            if event.has_value(
                "json.attributes.last_https_certificate.cert_signature.signature_algorithm",
            ) {
                event.rename("json.attributes.last_https_certificate.cert_signature.signature_algorithm", "gti.ioc_stream.attributes.last_https_certificate.cert_signature.signature_algorithm")?;
            }

            if event.has_value("json.attributes.last_https_certificate.issuer.C") {
                event.rename(
                    "json.attributes.last_https_certificate.issuer.C",
                    "gti.ioc_stream.attributes.last_https_certificate.issuer.c",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.issuer.CN") {
                event.rename(
                    "json.attributes.last_https_certificate.issuer.CN",
                    "gti.ioc_stream.attributes.last_https_certificate.issuer.cn",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.issuer.L") {
                event.rename(
                    "json.attributes.last_https_certificate.issuer.L",
                    "gti.ioc_stream.attributes.last_https_certificate.issuer.l",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.issuer.O") {
                event.rename(
                    "json.attributes.last_https_certificate.issuer.O",
                    "gti.ioc_stream.attributes.last_https_certificate.issuer.o",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.issuer.OU") {
                event.rename(
                    "json.attributes.last_https_certificate.issuer.OU",
                    "gti.ioc_stream.attributes.last_https_certificate.issuer.ou",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.issuer.ST") {
                event.rename(
                    "json.attributes.last_https_certificate.issuer.ST",
                    "gti.ioc_stream.attributes.last_https_certificate.issuer.st",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.public_key.algorithm") {
                event.rename(
                    "json.attributes.last_https_certificate.public_key.algorithm",
                    "gti.ioc_stream.attributes.last_https_certificate.public_key.algorithm",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.public_key.ec.oid") {
                event.rename(
                    "json.attributes.last_https_certificate.public_key.ec.oid",
                    "gti.ioc_stream.attributes.last_https_certificate.public_key.ec.oid",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.public_key.ec.pub") {
                event.rename(
                    "json.attributes.last_https_certificate.public_key.ec.pub",
                    "gti.ioc_stream.attributes.last_https_certificate.public_key.ec.pub",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.public_key.rsa.exponent") {
                event.rename(
                    "json.attributes.last_https_certificate.public_key.rsa.exponent",
                    "gti.ioc_stream.attributes.last_https_certificate.public_key.rsa.exponent",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.last_https_certificate.public_key.rsa.key_size")
                {
                    if let Some(val) =
                        event.get("json.attributes.last_https_certificate.public_key.rsa.key_size")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path:
                                    "json.attributes.last_https_certificate.public_key.rsa.key_size"
                                        .into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.last_https_certificate.public_key.rsa.key_size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_last_https_certificate_public_key_rsa_key_size_to_long",
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

            if event.has_value("json.attributes.last_https_certificate.public_key.rsa.modulus") {
                event.rename(
                    "json.attributes.last_https_certificate.public_key.rsa.modulus",
                    "gti.ioc_stream.attributes.last_https_certificate.public_key.rsa.modulus",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.serial_number") {
                event.rename(
                    "json.attributes.last_https_certificate.serial_number",
                    "gti.ioc_stream.attributes.last_https_certificate.serial_number",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.signature_algorithm") {
                event.rename(
                    "json.attributes.last_https_certificate.signature_algorithm",
                    "gti.ioc_stream.attributes.last_https_certificate.signature_algorithm",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.last_https_certificate.size") {
                    if let Some(val) = event.get("json.attributes.last_https_certificate.size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.last_https_certificate.size".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.last_https_certificate.size",
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
                    "convert_attributes_last_https_certificate_size_to_long",
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

            if event.has_value("json.attributes.last_https_certificate.subject.CN") {
                event.rename(
                    "json.attributes.last_https_certificate.subject.CN",
                    "gti.ioc_stream.attributes.last_https_certificate.subject.cn",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.thumbprint") {
                event.rename(
                    "json.attributes.last_https_certificate.thumbprint",
                    "gti.ioc_stream.attributes.last_https_certificate.thumbprint",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.tags") {
                event.rename(
                    "json.attributes.last_https_certificate.tags",
                    "gti.ioc_stream.attributes.last_https_certificate.tags",
                )?;
            }

            if event.has_value("json.attributes.last_https_certificate.thumbprint_sha256") {
                event.rename(
                    "json.attributes.last_https_certificate.thumbprint_sha256",
                    "gti.ioc_stream.attributes.last_https_certificate.thumbprint_sha256",
                )?;
            }

            let _cond = {
                event.has_value("json.attributes.last_https_certificate.validity.not_after")
                    && event.get_str("json.attributes.last_https_certificate.validity.not_after")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("json.attributes.last_https_certificate.validity.not_after")
                    {
                        match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("gti.ioc_stream.attributes.last_https_certificate.validity.not_after", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.attributes.last_https_certificate.validity.not_after".into(),
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
                        "date_attributes_last_https_certificate_validity_not_after",
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
                event.has_value("json.attributes.last_https_certificate.validity.not_before")
                    && event.get_str("json.attributes.last_https_certificate.validity.not_before")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event
                        .get_as_string("json.attributes.last_https_certificate.validity.not_before")
                    {
                        match parse_date_out(&date_str, &["yyyy-MM-dd HH:mm:ss"], None, None) {
                        Some(parsed) => event.set("gti.ioc_stream.attributes.last_https_certificate.validity.not_before", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.attributes.last_https_certificate.validity.not_before".into(),
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
                        "date_attributes_last_https_certificate_validity_not_before",
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

            if event.has_value("json.attributes.last_https_certificate.version") {
                event.rename(
                    "json.attributes.last_https_certificate.version",
                    "gti.ioc_stream.attributes.last_https_certificate.version",
                )?;
            }

            let _cond = {
                event.has_value("json.attributes.last_https_certificate_date")
                    && event.get_str("json.attributes.last_https_certificate_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.attributes.last_https_certificate_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set(
                                "gti.ioc_stream.attributes.last_https_certificate_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.last_https_certificate_date".into(),
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
                        "date_attributes_last_https_certificate_date",
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
                event.has_value("json.attributes.last_modification_date")
                    && event.get_str("json.attributes.last_modification_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.attributes.last_modification_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event
                                .set("gti.ioc_stream.attributes.last_modification_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.last_modification_date".into(),
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
                        "date_attributes_last_modification_date",
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
                .get("gti.ioc_stream.attributes.last_modification_date")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event.has_value("json.attributes.last_seen_itw_date")
                    && event.get_str("json.attributes.last_seen_itw_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.attributes.last_seen_itw_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("gti.ioc_stream.attributes.last_seen_itw_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.last_seen_itw_date".into(),
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
                        "date_attributes_last_seen_itw_date",
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
                event.has_value("json.attributes.last_submission_date")
                    && event.get_str("json.attributes.last_submission_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.attributes.last_submission_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event
                                .set("gti.ioc_stream.attributes.last_submission_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.last_submission_date".into(),
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
                        "date_attributes_last_submission_date",
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
                event.has_value("json.attributes.last_update_date")
                    && event.get_str("json.attributes.last_update_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.attributes.last_update_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("gti.ioc_stream.attributes.last_update_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.last_update_date".into(),
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
                        "date_attributes_last_update_date",
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

            if event.has_value("json.attributes.magic") {
                event.rename("json.attributes.magic", "gti.ioc_stream.attributes.magic")?;
            }

            if event.has_value("json.attributes.magika") {
                event.rename("json.attributes.magika", "gti.ioc_stream.attributes.magika")?;
            }

            if event.has_value("json.attributes.main_icon.dhash") {
                event.rename(
                    "json.attributes.main_icon.dhash",
                    "gti.ioc_stream.attributes.main_icon.dhash",
                )?;
            }

            if event.has_value("json.attributes.main_icon.raw_md5") {
                event.rename(
                    "json.attributes.main_icon.raw_md5",
                    "gti.ioc_stream.attributes.main_icon.raw_md5",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.mandiant_ic_score") {
                    if let Some(val) = event.get("json.attributes.mandiant_ic_score") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.mandiant_ic_score".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.mandiant_ic_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_mandiant_ic_score_to_long",
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

            if event.has_value("json.attributes.md5") {
                event.rename("json.attributes.md5", "gti.ioc_stream.attributes.md5")?;
            }

            if let Some(v) = event
                .get("gti.ioc_stream.attributes.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.md5", v)?;
            }

            if event.has_value("json.attributes.meaningful_name") {
                event.rename(
                    "json.attributes.meaningful_name",
                    "gti.ioc_stream.attributes.meaningful_name",
                )?;
            }

            if let Some(v) = event
                .get("gti.ioc_stream.attributes.meaningful_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            let _cond = {
                event
                    .get("json.attributes.names")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.names", |event| {
                    event.append_unique(
                        "file.attributes",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.attributes.names") {
                event.rename("json.attributes.names", "gti.ioc_stream.attributes.names")?;
            }

            if event.has_value("json.attributes.network") {
                event.rename(
                    "json.attributes.network",
                    "gti.ioc_stream.attributes.network",
                )?;
            }

            if event.has_value("json.attributes.outgoing_links") {
                event.rename(
                    "json.attributes.outgoing_links",
                    "gti.ioc_stream.attributes.outgoing_links",
                )?;
            }

            if event.has_value("json.attributes.pe_info.compiler_product_versions") {
                event.rename(
                    "json.attributes.pe_info.compiler_product_versions",
                    "gti.ioc_stream.attributes.pe_info.compiler_product_versions",
                )?;
            }

            if event.has_value("json.attributes.pe_info.debug") {
                event.rename(
                    "json.attributes.pe_info.debug",
                    "gti.ioc_stream.attributes.pe_info.debug",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.pe_info.entry_point") {
                    if let Some(val) = event.get("json.attributes.pe_info.entry_point") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.pe_info.entry_point".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.pe_info.entry_point", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_pe_info_entry_point_to_long",
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

            if event.has_value("json.attributes.pe_info.exports") {
                event.rename(
                    "json.attributes.pe_info.exports",
                    "gti.ioc_stream.attributes.pe_info.exports",
                )?;
            }

            if event.has_value("json.attributes.pe_info.imphash") {
                event.rename(
                    "json.attributes.pe_info.imphash",
                    "gti.ioc_stream.attributes.pe_info.imphash",
                )?;
            }

            if event.has_value("json.attributes.pe_info.import_list") {
                event.rename(
                    "json.attributes.pe_info.import_list",
                    "gti.ioc_stream.attributes.pe_info.import_list",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.pe_info.machine_type") {
                    if let Some(val) = event.get("json.attributes.pe_info.machine_type") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.pe_info.machine_type".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.pe_info.machine_type", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_pe_info_machine_type_to_long",
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

            if event.has_value("json.attributes.pe_info.resource_details") {
                event.rename(
                    "json.attributes.pe_info.resource_details",
                    "gti.ioc_stream.attributes.pe_info.resource_details",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.pe_info.resource_types.RT_GROUP_ICON") {
                    if let Some(val) =
                        event.get("json.attributes.pe_info.resource_types.RT_GROUP_ICON")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.pe_info.resource_types.RT_GROUP_ICON".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.pe_info.resource_types.rt_group_icon",
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
                    "convert_attributes_pe_info_resource_types_RT_GROUP_ICON_to_long",
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
                if event.has_value("json.attributes.pe_info.resource_types.RT_ICON") {
                    if let Some(val) = event.get("json.attributes.pe_info.resource_types.RT_ICON") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.pe_info.resource_types.RT_ICON".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.pe_info.resource_types.rt_icon",
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
                    "convert_attributes_pe_info_resource_types_RT_ICON_to_long",
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
                if event.has_value("json.attributes.pe_info.resource_types.RT_MANIFEST") {
                    if let Some(val) =
                        event.get("json.attributes.pe_info.resource_types.RT_MANIFEST")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.pe_info.resource_types.RT_MANIFEST".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.pe_info.resource_types.rt_manifest",
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
                    "convert_attributes_pe_info_resource_types_RT_MANIFEST_to_long",
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
                if event.has_value("json.attributes.pe_info.resource_types.RT_VERSION") {
                    if let Some(val) =
                        event.get("json.attributes.pe_info.resource_types.RT_VERSION")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.pe_info.resource_types.RT_VERSION".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.pe_info.resource_types.rt_version",
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
                    "convert_attributes_pe_info_resource_types_RT_VERSION_to_long",
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

            if event.has_value("json.attributes.pe_info.rich_pe_header_hash") {
                event.rename(
                    "json.attributes.pe_info.rich_pe_header_hash",
                    "gti.ioc_stream.attributes.pe_info.rich_pe_header_hash",
                )?;
            }

            if event.has_value("json.attributes.pe_info.sections") {
                event.rename(
                    "json.attributes.pe_info.sections",
                    "gti.ioc_stream.attributes.pe_info.sections",
                )?;
            }

            let _cond = {
                event.has_value("json.attributes.pe_info.timestamp")
                    && event.get_str("json.attributes.pe_info.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.attributes.pe_info.timestamp")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("gti.ioc_stream.attributes.pe_info.timestamp", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.pe_info.timestamp".into(),
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
                        "date_attributes_pe_info_timestamp",
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

            if event
                .has_value("json.attributes.popular_threat_classification.popular_threat_category")
            {
                event.rename("json.attributes.popular_threat_classification.popular_threat_category", "gti.ioc_stream.attributes.popular_threat_classification.popular_threat_category")?;
            }

            if event.has_value("json.attributes.popular_threat_classification.popular_threat_name")
            {
                event.rename(
                    "json.attributes.popular_threat_classification.popular_threat_name",
                    "gti.ioc_stream.attributes.popular_threat_classification.popular_threat_name",
                )?;
            }

            if event
                .has_value("json.attributes.popular_threat_classification.suggested_threat_label")
            {
                event.rename("json.attributes.popular_threat_classification.suggested_threat_label", "gti.ioc_stream.attributes.popular_threat_classification.suggested_threat_label")?;
            }

            if event.has_value("json.attributes.redirection_chain") {
                event.rename(
                    "json.attributes.redirection_chain",
                    "gti.ioc_stream.attributes.redirection_chain",
                )?;
            }

            if event.has_value("json.attributes.regional_internet_registry") {
                event.rename(
                    "json.attributes.regional_internet_registry",
                    "gti.ioc_stream.attributes.regional_internet_registry",
                )?;
            }

            if event.has_value("json.attributes.registrar") {
                event.rename(
                    "json.attributes.registrar",
                    "gti.ioc_stream.attributes.registrar",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.reputation") {
                    if let Some(val) = event.get("json.attributes.reputation") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.reputation".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.reputation", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_reputation_to_long",
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
                if event.has_value(
                    "json.attributes.threat_severity.threat_severity_data.num_gav_detections",
                ) {
                    if let Some(val) = event.get(
                        "json.attributes.threat_severity.threat_severity_data.num_gav_detections",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.threat_severity.threat_severity_data.num_gav_detections".into(),
                            message,
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.severity_data.num_gav_detections",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attributes_threat_severity_threat_severity_data_num_gav_detections_to_long")?;
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

            if event.has_value("json.attributes.sha1") {
                event.rename("json.attributes.sha1", "gti.ioc_stream.attributes.sha1")?;
            }

            if let Some(v) = event
                .get("gti.ioc_stream.attributes.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha1", v)?;
            }

            if event.has_value("json.attributes.sha256") {
                event.rename("json.attributes.sha256", "gti.ioc_stream.attributes.sha256")?;
            }

            if let Some(v) = event
                .get("gti.ioc_stream.attributes.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.hash.sha256", v)?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.attributes.sigma_analysis_results").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                            {
                                // A foreach walks a LIST or an OBJECT: over an object Elastic
                                // binds `_ingest._key` per entry, which is what a target of
                                // `<field>.{{{_ingest._key}}}` reads.
                                let subject = event.get("_ingest._value.match_context").cloned();
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
                                            event.set(
                                                "_ingest._key",
                                                Value::String(key.to_string()),
                                            )?;
                                        }
                                        event.set("_ingest._value", item)?;
                                        // on_failure: 1 handler(s)
                                        if let Err(err) = (|| -> Result<()> {
                                            if let Some(date_str) = event.get_as_string(
                                                "_ingest._value.values.CreationUtcTime",
                                            ) {
                                                match parse_date_out(
                                                    &date_str,
                                                    &["UNIX"],
                                                    None,
                                                    None,
                                                ) {
                                                    Some(parsed) => event.set(
                                                        "_ingest._value.creation_time",
                                                        parsed,
                                                    )?,
                                                    None => {
                                                        return Err(TransformError::ParseError {
                            path: "_ingest._value.values.CreationUtcTime".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                                                    }
                                                }
                                            }
                                            Ok(())
                                        })(
                                        ) {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event
                                                .set("_ingest.on_failure_processor_type", "date")?;
                                            event.set("_ingest.on_failure_processor_tag", "date_attributes_sigma_analysis_results_match_context_values_CreationUtcTime")?;
                                            event.remove("_ingest._value.values.CreationUtcTime");
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
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
                                        "_ingest._value.match_context",
                                        if keyed {
                                            Value::Object(fields)
                                        } else {
                                            Value::Array(list)
                                        },
                                    )?;
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
                            "json.attributes.sigma_analysis_results",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.DestinationHostname") {
                            event.rename(
                                "_ingest._value.values.DestinationHostname",
                                "_ingest._value.destination_hostname",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.values.DestinationIp") {
                                if let Some(val) = event.get("_ingest._value.values.DestinationIp")
                                {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.values.DestinationIp".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.destination_ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_attributes_sigma_analysis_results_match_context_values_DestinationIp_to_ip")?;
                            event.remove("_ingest._value.values.DestinationIp");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.values.DestinationIsIpv6") {
                                if let Some(val) =
                                    event.get("_ingest._value.values.DestinationIsIpv6")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.values.DestinationIsIpv6"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.destination_is_ipv6", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_attributes_sigma_analysis_results_match_context_values_DestinationIsIpv6_to_boolean")?;
                            event.remove("_ingest._value.values.DestinationIsIpv6");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.values.DestinationPort") {
                                if let Some(val) =
                                    event.get("_ingest._value.values.DestinationPort")
                                {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.values.DestinationPort"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.destination_port", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_attributes_sigma_analysis_results_match_context_values_DestinationPort_to_long")?;
                            event.remove("_ingest._value.values.DestinationPort");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.DestinationPortName") {
                            event.rename(
                                "_ingest._value.values.DestinationPortName",
                                "_ingest._value.destination_port_name",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.Details") {
                            event.rename(
                                "_ingest._value.values.Details",
                                "_ingest._value.details",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.values.EventID") {
                                if let Some(val) = event.get("_ingest._value.values.EventID") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.values.EventID".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.event_id", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_attributes_sigma_analysis_results_match_context_values_EventID_to_long")?;
                            event.remove("_ingest._value.values.EventID");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.EventType") {
                            event.rename(
                                "_ingest._value.values.EventType",
                                "_ingest._value.event_type",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.Image") {
                            event.rename("_ingest._value.values.Image", "_ingest._value.image")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.values.Initiated") {
                                if let Some(val) = event.get("_ingest._value.values.Initiated") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.values.Initiated".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.initiated", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_attributes_sigma_analysis_results_match_context_values_Initiated_to_boolean")?;
                            event.remove("_ingest._value.values.Initiated");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.ProcessGuid") {
                            event.rename(
                                "_ingest._value.values.ProcessGuid",
                                "_ingest._value.process_guid",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.values.ProcessId") {
                                if let Some(val) = event.get("_ingest._value.values.ProcessId") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.values.ProcessId".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.process_id", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_attributes_sigma_analysis_results_match_context_values_ProcessId_to_long")?;
                            event.remove("_ingest._value.values.ProcessId");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.Protocol") {
                            event.rename(
                                "_ingest._value.values.Protocol",
                                "_ingest._value.protocol",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.RuleName") {
                            event.rename(
                                "_ingest._value.values.RuleName",
                                "_ingest._value.rule_name",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.SourceHostname") {
                            event.rename(
                                "_ingest._value.values.SourceHostname",
                                "_ingest._value.source_hostname",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.values.SourceIp") {
                                if let Some(val) = event.get("_ingest._value.values.SourceIp") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.values.SourceIp".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.source_ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_attributes_sigma_analysis_results_match_context_values_SourceIp_to_ip")?;
                            event.remove("_ingest._value.values.SourceIp");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.values.SourceIsIpv6") {
                                if let Some(val) = event.get("_ingest._value.values.SourceIsIpv6") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.values.SourceIsIpv6".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.source_is_ipv6", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_attributes_sigma_analysis_results_match_context_values_SourceIsIpv6_to_boolean")?;
                            event.remove("_ingest._value.values.SourceIsIpv6");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.values.SourcePort") {
                                if let Some(val) = event.get("_ingest._value.values.SourcePort") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.values.SourcePort".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.source_port", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_attributes_sigma_analysis_results_match_context_values_SourcePort_to_long")?;
                            event.remove("_ingest._value.values.SourcePort");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.SourcePortName") {
                            event.rename(
                                "_ingest._value.values.SourcePortName",
                                "_ingest._value.source_port_name",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.TargetFilename") {
                            event.rename(
                                "_ingest._value.values.TargetFilename",
                                "_ingest._value.target_file_name",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.TargetObject") {
                            event.rename(
                                "_ingest._value.values.TargetObject",
                                "_ingest._value.target_object",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.User") {
                            event.rename("_ingest._value.values.User", "_ingest._value.user")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event.get("json.attributes.sigma_analysis_results").cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
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
                            {
                                // A foreach walks a LIST or an OBJECT: over an object Elastic
                                // binds `_ingest._key` per entry, which is what a target of
                                // `<field>.{{{_ingest._key}}}` reads.
                                let subject = event.get("_ingest._value.match_context").cloned();
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
                                            event.set(
                                                "_ingest._key",
                                                Value::String(key.to_string()),
                                            )?;
                                        }
                                        event.set("_ingest._value", item)?;
                                        // on_failure: 1 handler(s)
                                        if let Err(err) =
                                            (|| -> Result<()> {
                                                if let Some(date_str) = event
                                                    .get_as_string("_ingest._value.values.UtcTime")
                                                {
                                                    match parse_date_out(
                                                        &date_str,
                                                        &["UNIX"],
                                                        None,
                                                        None,
                                                    ) {
                                                        Some(parsed) => event.set(
                                                            "_ingest._value.utc_time",
                                                            parsed,
                                                        )?,
                                                        None => {
                                                            return Err(TransformError::ParseError {
                            path: "_ingest._value.values.UtcTime".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                                                        }
                                                    }
                                                }
                                                Ok(())
                                            })()
                                        {
                                            event.set(
                                                "_ingest.on_failure_message",
                                                err.to_string(),
                                            )?;
                                            event
                                                .set("_ingest.on_failure_processor_type", "date")?;
                                            event.set("_ingest.on_failure_processor_tag", "date_attributes_sigma_analysis_results_match_context_values_UtcTime")?;
                                            event.remove("_ingest._value.values.UtcTime");
                                            event.remove("_ingest.on_failure_message");
                                            event.remove("_ingest.on_failure_processor_type");
                                            event.remove("_ingest.on_failure_processor_tag");
                                            if event
                                                .get_object("_ingest")
                                                .is_some_and(|m| m.is_empty())
                                            {
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
                                        "_ingest._value.match_context",
                                        if keyed {
                                            Value::Object(fields)
                                        } else {
                                            Value::Array(list)
                                        },
                                    )?;
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
                            "json.attributes.sigma_analysis_results",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.CommandLine") {
                            event.rename(
                                "_ingest._value.values.CommandLine",
                                "_ingest._value.command_line",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.Description") {
                            event.rename(
                                "_ingest._value.values.Description",
                                "_ingest._value.description",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.OriginalFileName") {
                            event.rename(
                                "_ingest._value.values.OriginalFileName",
                                "_ingest._value.original_file_name",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.FileVersion") {
                            event.rename(
                                "_ingest._value.values.FileVersion",
                                "_ingest._value.file_version",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.Product") {
                            event.rename(
                                "_ingest._value.values.Product",
                                "_ingest._value.product",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.ParentCommandLine") {
                            event.rename(
                                "_ingest._value.values.ParentCommandLine",
                                "_ingest._value.parent_command_line",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.CurrentDirectory") {
                            event.rename(
                                "_ingest._value.values.CurrentDirectory",
                                "_ingest._value.current_directory",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.Company") {
                            event.rename(
                                "_ingest._value.values.Company",
                                "_ingest._value.company",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.Hashes") {
                            event
                                .rename("_ingest._value.values.Hashes", "_ingest._value.hashes")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.ParentImage") {
                            event.rename(
                                "_ingest._value.values.ParentImage",
                                "_ingest._value.parent_image",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.IntegrityLevel") {
                            event.rename(
                                "_ingest._value.values.IntegrityLevel",
                                "_ingest._value.integrity_level",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.LogonGuid") {
                            event.rename(
                                "_ingest._value.values.LogonGuid",
                                "_ingest._value.logon_guid",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.ParentProcessGuid") {
                            event.rename(
                                "_ingest._value.values.ParentProcessGuid",
                                "_ingest._value.parent_process_guid",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.values.TerminalSessionId") {
                                if let Some(val) =
                                    event.get("_ingest._value.values.TerminalSessionId")
                                {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.values.TerminalSessionId"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.terminal_session_id", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_attributes_sigma_analysis_results_match_context_values_TerminalSessionId_to_long")?;
                            event.remove("_ingest._value.values.TerminalSessionId");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.QueryStatus") {
                            event.rename(
                                "_ingest._value.values.QueryStatus",
                                "_ingest._value.query_status",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.QueryResults") {
                            event.rename(
                                "_ingest._value.values.QueryResults",
                                "_ingest._value.query_results",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.QueryName") {
                            event.rename(
                                "_ingest._value.values.QueryName",
                                "_ingest._value.query_name",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.SignatureStatus") {
                            event.rename(
                                "_ingest._value.values.SignatureStatus",
                                "_ingest._value.signature_status",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.ImageLoaded") {
                            event.rename(
                                "_ingest._value.values.ImageLoaded",
                                "_ingest._value.image_loaded",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.values.Signed") {
                                if let Some(val) = event.get("_ingest._value.values.Signed") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.values.Signed".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.signed", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_attributes_sigma_analysis_results_match_context_values_Signed_to_boolean")?;
                            event.remove("_ingest._value.values.Signed");
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
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.Signature") {
                            event.rename(
                                "_ingest._value.values.Signature",
                                "_ingest._value.signature",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        if event.has_value("_ingest._value.values.param1") {
                            event
                                .rename("_ingest._value.values.param1", "_ingest._value.param1")?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.attributes.sigma_analysis_results")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.sigma_analysis_results", |event| {
                    foreach_array(event, "_ingest._value.match_context", |event| {
                        event.remove("_ingest._value.values.DestinationIp");
                        event.remove("_ingest._value.values.DestinationIsIpv6");
                        event.remove("_ingest._value.values.DestinationPort");
                        event.remove("_ingest._value.values.EventID");
                        event.remove("_ingest._value.values.Initiated");
                        event.remove("_ingest._value.values.ProcessId");
                        event.remove("_ingest._value.values.Signed");
                        event.remove("_ingest._value.values.SourceIp");
                        event.remove("_ingest._value.values.SourceIsIpv6");
                        event.remove("_ingest._value.values.SourcePort");
                        event.remove("_ingest._value.values.UtcTime");
                        event.remove("_ingest._value.values.TerminalSessionId");
                        event.remove("_ingest._value.values.CreationUtcTime");
                        Ok(())
                    })?;
                    Ok(())
                })?;
            }

            if event.has_value("json.attributes.sigma_analysis_results") {
                event.rename(
                    "json.attributes.sigma_analysis_results",
                    "gti.ioc_stream.attributes.sigma_analysis_results",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.sigma_analysis_stats.critical") {
                    if let Some(val) = event.get("json.attributes.sigma_analysis_stats.critical") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.sigma_analysis_stats.critical".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.sigma_analysis_stats.critical",
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
                    "convert_attributes_sigma_analysis_stats_critical_to_long",
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
                if event.has_value("json.attributes.sigma_analysis_stats.high") {
                    if let Some(val) = event.get("json.attributes.sigma_analysis_stats.high") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.sigma_analysis_stats.high".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.sigma_analysis_stats.high",
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
                    "convert_attributes_sigma_analysis_stats_high_to_long",
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
                if event.has_value("json.attributes.sigma_analysis_stats.low") {
                    if let Some(val) = event.get("json.attributes.sigma_analysis_stats.low") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.sigma_analysis_stats.low".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.sigma_analysis_stats.low",
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
                    "convert_attributes_sigma_analysis_stats_low_to_long",
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
                if event.has_value("json.attributes.sigma_analysis_stats.medium") {
                    if let Some(val) = event.get("json.attributes.sigma_analysis_stats.medium") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.sigma_analysis_stats.medium".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.sigma_analysis_stats.medium",
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
                    "convert_attributes_sigma_analysis_stats_medium_to_long",
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

            if event.has_value("json.attributes.signature_info.copyright") {
                event.rename(
                    "json.attributes.signature_info.copyright",
                    "gti.ioc_stream.attributes.signature_info.copyright",
                )?;
            }

            if event.has_value("json.attributes.signature_info.description") {
                event.rename(
                    "json.attributes.signature_info.description",
                    "gti.ioc_stream.attributes.signature_info.description",
                )?;
            }

            if event.has_value("json.attributes.signature_info.product") {
                event.rename(
                    "json.attributes.signature_info.product",
                    "gti.ioc_stream.attributes.signature_info.product",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.size") {
                    if let Some(val) = event.get("json.attributes.size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.size".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.size", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_size_to_long",
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

            if event.has_value("json.attributes.ssdeep") {
                event.rename("json.attributes.ssdeep", "gti.ioc_stream.attributes.ssdeep")?;
            }

            if event.has_value("json.attributes.tags") {
                event.rename("json.attributes.tags", "gti.ioc_stream.attributes.tags")?;
            }

            let _cond = {
                event.has_value("json.attributes.threat_severity.last_analysis_date")
                    && event.get_str("json.attributes.threat_severity.last_analysis_date")
                        != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.attributes.threat_severity.last_analysis_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set(
                                "gti.ioc_stream.attributes.threat_severity.last_analysis_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.threat_severity.last_analysis_date"
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
                        "date_attributes_threat_severity_last_analysis_date",
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

            if event.has_value("json.attributes.threat_severity.level_description") {
                event.rename(
                    "json.attributes.threat_severity.level_description",
                    "gti.ioc_stream.attributes.threat_severity.level_description",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.threat_severity.threat_severity_data.belongs_to_bad_collection") {
                if let Some(val) = event.get("json.attributes.threat_severity.threat_severity_data.belongs_to_bad_collection") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.threat_severity.threat_severity_data.belongs_to_bad_collection".into(),
                            message,
                        })?;
                    event.set("gti.ioc_stream.attributes.threat_severity.threat_severity_data.belongs_to_bad_collection", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attributes_threat_severity_threat_severity_data_belongs_to_bad_collection_to_boolean")?;
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
                if event.has_value(
                    "json.attributes.threat_severity.threat_severity_data.belongs_to_threat_actor",
                ) {
                    if let Some(val) = event.get("json.attributes.threat_severity.threat_severity_data.belongs_to_threat_actor") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.threat_severity.threat_severity_data.belongs_to_threat_actor".into(),
                            message,
                        })?;
                    event.set("gti.ioc_stream.attributes.threat_severity.threat_severity_data.belongs_to_threat_actor", converted)?;
                }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attributes_threat_severity_threat_severity_data_belongs_to_threat_actor_to_boolean")?;
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
                if event
                    .has_value("json.attributes.threat_severity.threat_severity_data.domain_rank")
                {
                    if let Some(val) = event
                        .get("json.attributes.threat_severity.threat_severity_data.domain_rank")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.threat_severity.threat_severity_data.domain_rank".into(),
                            message,
                        })?;
                        event.set("gti.ioc_stream.attributes.threat_severity.threat_severity_data.domain_rank", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_threat_severity_threat_severity_data_domain_rank_to_long",
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
                if event.has_value("json.attributes.threat_severity.threat_severity_data.has_bad_communicating_files_high") {
                if let Some(val) = event.get("json.attributes.threat_severity.threat_severity_data.has_bad_communicating_files_high") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.threat_severity.threat_severity_data.has_bad_communicating_files_high".into(),
                            message,
                        })?;
                    event.set("gti.ioc_stream.attributes.threat_severity.threat_severity_data.has_bad_communicating_files_high", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attributes_threat_severity_threat_severity_data_has_bad_communicating_files_high_to_boolean")?;
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
                if event.has_value("json.attributes.threat_severity.threat_severity_data.has_bad_communicating_files_medium") {
                if let Some(val) = event.get("json.attributes.threat_severity.threat_severity_data.has_bad_communicating_files_medium") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.threat_severity.threat_severity_data.has_bad_communicating_files_medium".into(),
                            message,
                        })?;
                    event.set("gti.ioc_stream.attributes.threat_severity.threat_severity_data.has_bad_communicating_files_medium", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attributes_threat_severity_threat_severity_data_has_bad_communicating_files_medium_to_boolean")?;
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

            if event.has_value("json.attributes.threat_severity.threat_severity_level") {
                event.rename(
                    "json.attributes.threat_severity.threat_severity_level",
                    "gti.ioc_stream.attributes.threat_severity.threat_severity_level",
                )?;
            }

            if event.has_value("json.attributes.threat_severity.version") {
                event.rename(
                    "json.attributes.threat_severity.version",
                    "gti.ioc_stream.attributes.threat_severity.version",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value(
                    "json.attributes.threat_severity.threat_severity_data.has_references",
                ) {
                    if let Some(val) = event
                        .get("json.attributes.threat_severity.threat_severity_data.has_references")
                    {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.threat_severity.threat_severity_data.has_references".into(),
                            message,
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.threat_severity_data.has_references",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attributes_threat_severity_threat_severity_data_has_references_to_boolean")?;
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
                if event.has_value(
                    "json.attributes.threat_severity.threat_severity_data.has_vulnerabilities",
                ) {
                    if let Some(val) = event.get(
                        "json.attributes.threat_severity.threat_severity_data.has_vulnerabilities",
                    ) {
                        let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.threat_severity.threat_severity_data.has_vulnerabilities".into(),
                            message,
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.threat_severity_data.has_vulnerabilities",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attributes_threat_severity_threat_severity_data_has_vulnerabilities_to_boolean")?;
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
                if event.has_value(
                    "json.attributes.threat_severity.threat_severity_data.num_av_detections",
                ) {
                    if let Some(val) = event.get(
                        "json.attributes.threat_severity.threat_severity_data.num_av_detections",
                    ) {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.threat_severity.threat_severity_data.num_av_detections".into(),
                            message,
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.threat_severity_data.num_av_detections",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attributes_threat_severity_threat_severity_data_num_av_detections_to_long")?;
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
                if event.has_value(
                    "json.attributes.threat_severity.threat_severity_data.num_detections",
                ) {
                    if let Some(val) = event
                        .get("json.attributes.threat_severity.threat_severity_data.num_detections")
                    {
                        let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.attributes.threat_severity.threat_severity_data.num_detections".into(),
                            message,
                        })?;
                        event.set(
                            "gti.ioc_stream.attributes.threat_severity_data.num_detections",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_attributes_threat_severity_threat_severity_data_num_detections_to_long")?;
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
                if event.has_value("json.attributes.times_submitted") {
                    if let Some(val) = event.get("json.attributes.times_submitted") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.times_submitted".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.times_submitted", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_times_submitted_to_long",
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

            if event.has_value("json.attributes.title") {
                event.rename("json.attributes.title", "gti.ioc_stream.attributes.title")?;
            }

            if event.has_value("json.attributes.tld") {
                event.rename("json.attributes.tld", "gti.ioc_stream.attributes.tld")?;
            }

            if event.has_value("json.attributes.tlsh") {
                event.rename("json.attributes.tlsh", "gti.ioc_stream.attributes.tlsh")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.total_votes.harmless") {
                    if let Some(val) = event.get("json.attributes.total_votes.harmless") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.total_votes.harmless".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.total_votes.harmless", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_total_votes_harmless_to_long",
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
                if event.has_value("json.attributes.total_votes.malicious") {
                    if let Some(val) = event.get("json.attributes.total_votes.malicious") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.total_votes.malicious".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.total_votes.malicious", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_total_votes_malicious_to_long",
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
                event
                    .get("json.attributes.trid")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.attributes.trid", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_ingest._value.probability") {
                            if let Some(val) = event.get("_ingest._value.probability") {
                                let converted =
                                    convert_value(val, "double").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "_ingest._value.probability".into(),
                                            message,
                                        }
                                    })?;
                                event.set("_ingest._value.probability", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_attributes_trid_probability_to_double",
                        )?;
                        event.remove("_ingest._value.probability");
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
                    Ok(())
                })?;
            }

            if event.has_value("json.attributes.trid") {
                event.rename("json.attributes.trid", "gti.ioc_stream.attributes.trid")?;
            }

            if event.has_value("json.attributes.type_description") {
                event.rename(
                    "json.attributes.type_description",
                    "gti.ioc_stream.attributes.type_description",
                )?;
            }

            if let Some(v) = event
                .get("gti.ioc_stream.attributes.type_description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.type", v)?;
            }

            if event.has_value("json.attributes.type_extension") {
                event.rename(
                    "json.attributes.type_extension",
                    "gti.ioc_stream.attributes.type_extension",
                )?;
            }

            if let Some(v) = event
                .get("gti.ioc_stream.attributes.type_extension")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.extension", v)?;
            }

            if event.has_value("json.attributes.type_tags") {
                event.rename(
                    "json.attributes.type_tags",
                    "gti.ioc_stream.attributes.type_tags",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.attributes.last_analysis_stats.type-unsupported") {
                    if let Some(val) =
                        event.get("json.attributes.last_analysis_stats.type-unsupported")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.last_analysis_stats.type-unsupported".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.type_unsupported", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_last_analysis_stats_type-unsupported_to_long",
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
                if event.has_value("json.attributes.unique_sources") {
                    if let Some(val) = event.get("json.attributes.unique_sources") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.attributes.unique_sources".into(),
                                message,
                            }
                        })?;
                        event.set("gti.ioc_stream.attributes.unique_sources", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_attributes_unique_sources_to_long",
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

            if event.has_value("json.attributes.url") {
                event.rename("json.attributes.url", "gti.ioc_stream.attributes.url")?;
            }

            if let Some(v) = event
                .get("gti.ioc_stream.attributes.url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("url.full", v)?;
            }

            let _cond = {
                event.has_value("json.attributes.whois_date")
                    && event.get_str("json.attributes.whois_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.attributes.whois_date") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("gti.ioc_stream.attributes.whois_date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.attributes.whois_date".into(),
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
                        "date_attributes_whois_date",
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

            if event.has_value("json.context_attributes.hunting_info.rule_name") {
                event.rename(
                    "json.context_attributes.hunting_info.rule_name",
                    "gti.ioc_stream.context_attributes.hunting_info.rule_name",
                )?;
            }

            if event.has_value("json.context_attributes.hunting_info.rule_tags") {
                event.rename(
                    "json.context_attributes.hunting_info.rule_tags",
                    "gti.ioc_stream.context_attributes.hunting_info.rule_tags",
                )?;
            }

            if event.has_value("json.context_attributes.hunting_info.snippet") {
                event.rename(
                    "json.context_attributes.hunting_info.snippet",
                    "gti.ioc_stream.context_attributes.hunting_info.snippet",
                )?;
            }

            if event.has_value("json.context_attributes.hunting_info.source_country") {
                event.rename(
                    "json.context_attributes.hunting_info.source_country",
                    "gti.ioc_stream.context_attributes.hunting_info.source_country",
                )?;
            }

            if event.has_value("json.context_attributes.hunting_info.source_key") {
                event.rename(
                    "json.context_attributes.hunting_info.source_key",
                    "gti.ioc_stream.context_attributes.hunting_info.source_key",
                )?;
            }

            if event.has_value("json.context_attributes.tags") {
                event.rename(
                    "json.context_attributes.tags",
                    "gti.ioc_stream.context_attributes.tags",
                )?;
            }

            let _cond = {
                event.has_value("json.context_attributes.notification_date")
                    && event.get_str("json.context_attributes.notification_date") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.context_attributes.notification_date")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set(
                                "gti.ioc_stream.context_attributes.notification_date",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.context_attributes.notification_date".into(),
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
                        "date_context_attributes_notification_date",
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
                if event.has_value("json.context_attributes.notification_id") {
                    if let Some(val) = event.get("json.context_attributes.notification_id") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.context_attributes.notification_id".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "gti.ioc_stream.context_attributes.notification_id",
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
                    "convert_context_attributes_notification_id_to_long",
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

            if event.has_value("json.context_attributes.origin") {
                event.rename(
                    "json.context_attributes.origin",
                    "gti.ioc_stream.context_attributes.origin",
                )?;
            }

            if event.has_value("json.context_attributes.sources") {
                event.rename(
                    "json.context_attributes.sources",
                    "gti.ioc_stream.context_attributes.sources",
                )?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "gti.ioc_stream.id")?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "gti.ioc_stream.type")?;
            }

            if event.has_value("json.attributes.vhash") {
                event.rename("json.attributes.vhash", "gti.ioc_stream.vhash")?;
            }

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.authentihash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.favicon.dhash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.favicon.raw_md5")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.filecondis.dhash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.filecondis.raw_md5")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.last_http_response_content_sha256")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.last_https_certificate.thumbprint")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.last_https_certificate.thumbprint_sha256")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.main_icon.dhash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.main_icon.raw_md5")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.md5")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.pe_info.imphash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.pe_info.rich_pe_header_hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.sha1")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.sha256")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.ssdeep")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.tlsh")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.vhash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("gti.ioc_stream.attributes.jarm")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { event.get_str("gti.ioc_stream.type") == Some("url") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("gti.ioc_stream.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.set(
                "threat.feed.dashboard_id",
                Value::Array(vec![
                    json!("ti_google_threat_intelligence-55f5f53b-343e-4095-b61f-1089a5273d84"),
                    json!("ti_google_threat_intelligence-fb3daf8e-b45b-4fd9-bf94-dbaf96fcfb67"),
                ]),
            )?;

            event.set("threat.feed.name", json!("GTI IOC Stream"))?;

            if let Some(v) = event
                .get("gti.ioc_stream.attributes.url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.url.original", v)?;
            }

            if let Some(v) = event
                .get("threat.indicator.url.original")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.url.full", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("threat.indicator.url.original") {
                    if !uri_parts(event, "threat.indicator.url.original", "url", true, false)?
                        && event
                            .get_str("threat.indicator.url.original")
                            .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "threat.indicator.url.original".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uri_parts")?;
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

            event.append_unique(
                "related.hosts",
                json!(
                    event
                        .get("url.domain")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if let Some(v) = event
                .get("gti.ioc_stream.attributes.country")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.geo.country_iso_code", v)?;
            }

            if let Some(v) = event
                .get("gti.ioc_stream.attributes.continent")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.geo.continent_code", v)?;
            }

            event.append_unique(
                "threat.indicator.id",
                json!(
                    event
                        .get("gti.ioc_stream.id")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if let Some(v) = event
                .get("gti.ioc_stream.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            let _cond = { event.get_str("gti.ioc_stream.type") == Some("domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("gti.ioc_stream.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("gti.ioc_stream.type") == Some("file") };
            if _cond {
                if let Some(v) = event
                    .get("gti.ioc_stream.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.file.hash.sha256", v)?;
                }
            }

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("threat.indicator.file.hash.sha256")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { event.get_str("gti.ioc_stream.type") == Some("ip_address") };
            if _cond {
                if let Some(v) = event
                    .get("gti.ioc_stream.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.ip", v)?;
                }
            }

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("threat.indicator.ip")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            let _cond = { event.has_value("gti.ioc_stream.type") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.type = params[ctx.gti.ioc_stream.type];
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ctx.threat = ctx.threat ?: [:];\nctx.threat.indicator = ctx.threat.indicator ?: [:];\nctx.threat.indicator.type = params[ctx.gti.ioc_stream.type];"#
                        ),
                        cached_params!(
                            "{\"domain\":\"domain-name\",\"file\":\"file\",\"ip_address\":\"ipv4-addr\",\"url\":\"url\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "set_threat_indicator_type_from_gti_ioc_stream_type",
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

            event.remove("json");

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
