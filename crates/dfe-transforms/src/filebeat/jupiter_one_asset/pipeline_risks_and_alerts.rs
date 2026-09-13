// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_risks_and_alerts` pipeline.
pub struct PipelineRisksAndAlerts;

impl Transform for PipelineRisksAndAlerts {
    fn name(&self) -> &str {
        "pipeline_risks_and_alerts"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("alert"))?;

            let _cond = { event.get("jupiter_one.asset.entity._class").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Vulnerability")), serde_json::Value::String(s) => s.contains("Vulnerability"), _ => false }) };
            if _cond {
                event.append("event.category", json!("vulnerability"))?;
            }

            let _cond = { event.get("jupiter_one.asset.entity._class").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Vulnerability")), serde_json::Value::String(s) => s.contains("Vulnerability"), _ => false }) };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

                dot_expand(event, "jupiter_one.asset.properties", "*")?;

            let _cond = { event.has_value("jupiter_one.asset.properties.approved_on") && event.get_str("jupiter_one.asset.properties.approved_on") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("jupiter_one.asset.properties.approved_on") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("jupiter_one.asset.properties.approved_on", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "jupiter_one.asset.properties.approved_on".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_jupiter_one_asset_properties_approved_on_into_jupiter_one_asset_properties_approved_on_6c305ad2")?;
                        if event.remove("jupiter_one.asset.properties.approved_on").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.approved_on".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.reported_on") && event.get_str("jupiter_one.asset.properties.reported_on") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("jupiter_one.asset.properties.reported_on") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("jupiter_one.asset.properties.reported_on", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "jupiter_one.asset.properties.reported_on".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_jupiter_one_asset_properties_reported_on_into_jupiter_one_asset_properties_reported_on_cc7dfab6")?;
                        if event.remove("jupiter_one.asset.properties.reported_on").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.reported_on".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.detected_on") && event.get_str("jupiter_one.asset.properties.detected_on") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("jupiter_one.asset.properties.detected_on") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("jupiter_one.asset.properties.detected_on", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "jupiter_one.asset.properties.detected_on".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_jupiter_one_asset_properties_detected_on_into_jupiter_one_asset_properties_detected_on_2b57c1b8")?;
                        if event.remove("jupiter_one.asset.properties.detected_on").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.detected_on".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.published_on") && event.get_str("jupiter_one.asset.properties.published_on") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("jupiter_one.asset.properties.published_on") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("jupiter_one.asset.properties.published_on", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "jupiter_one.asset.properties.published_on".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_jupiter_one_asset_properties_published_on_into_jupiter_one_asset_properties_published_on_1d9721a2")?;
                        if event.remove("jupiter_one.asset.properties.published_on").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.published_on".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
            if event.has_value("jupiter_one.asset.properties.total_number_of_affected_entities") {
                if let Some(val) = event.get("jupiter_one.asset.properties.total_number_of_affected_entities") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.total_number_of_affected_entities".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.total_number_of_affected_entities", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_total_number_of_affected_entities_to_long_cf15bf27")?;
                        if event.remove("jupiter_one.asset.properties.total_number_of_affected_entities").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.total_number_of_affected_entities".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.numeric_severity") {
                if let Some(val) = event.get("jupiter_one.asset.properties.numeric_severity") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.numeric_severity".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.numeric_severity", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_numeric_severity_to_long_5fa6bfd5")?;
                        if event.remove("jupiter_one.asset.properties.numeric_severity").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.numeric_severity".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.remediation_sla") {
                if let Some(val) = event.get("jupiter_one.asset.properties.remediation_sla") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.remediation_sla".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.remediation_sla", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_remediation_sla_to_long_231e4476")?;
                        if event.remove("jupiter_one.asset.properties.remediation_sla").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.remediation_sla".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.exploit_status") {
                if let Some(val) = event.get("jupiter_one.asset.properties.exploit_status") {
                    let converted = convert_value(val, "integer")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.exploit_status".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.exploit_status", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_exploit_status_to_long_155fdf22")?;
                        if event.remove("jupiter_one.asset.properties.exploit_status").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.exploit_status".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.exploitability") {
                if let Some(val) = event.get("jupiter_one.asset.properties.exploitability") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.exploitability".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.exploitability", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_exploitability_to_double_f4e14120")?;
                        if event.remove("jupiter_one.asset.properties.exploitability").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.exploitability".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.impact") {
                if let Some(val) = event.get("jupiter_one.asset.properties.impact") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.impact".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.impact", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_impact_to_double_2d857527")?;
                        if event.remove("jupiter_one.asset.properties.impact").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.impact".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.score") {
                if let Some(val) = event.get("jupiter_one.asset.properties.score") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.score".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_score_to_double_4eba43d9")?;
                        if event.remove("jupiter_one.asset.properties.score").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.score".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.open") {
                if let Some(val) = event.get("jupiter_one.asset.properties.open") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.open".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.open", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_open_to_boolean_1597bca6")?;
                        if event.remove("jupiter_one.asset.properties.open").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.open".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.approved") {
                if let Some(val) = event.get("jupiter_one.asset.properties.approved") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.approved".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.approved", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_approved_to_boolean_154e80e1")?;
                        if event.remove("jupiter_one.asset.properties.approved").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.approved".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.exception") {
                if let Some(val) = event.get("jupiter_one.asset.properties.exception") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.exception".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.exception", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_exception_to_boolean_7c919b89")?;
                        if event.remove("jupiter_one.asset.properties.exception").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.exception".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.production") {
                if let Some(val) = event.get("jupiter_one.asset.properties.production") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.production".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.production", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_production_to_boolean_65f41341")?;
                        if event.remove("jupiter_one.asset.properties.production").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.production".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.blocks_production") {
                if let Some(val) = event.get("jupiter_one.asset.properties.blocks_production") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.blocks_production".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.blocks_production", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_blocks_production_to_boolean_b3238d5c")?;
                        if event.remove("jupiter_one.asset.properties.blocks_production").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.blocks_production".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.tag.production") {
                if let Some(val) = event.get("jupiter_one.asset.properties.tag.production") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.tag.production".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.tag.production", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_tag_production_to_boolean_b22c5127")?;
                        if event.remove("jupiter_one.asset.properties.tag.production").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.tag.production".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.blocking") {
                if let Some(val) = event.get("jupiter_one.asset.properties.blocking") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.blocking".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.blocking", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_blocking_to_boolean_ac1cebd9")?;
                        if event.remove("jupiter_one.asset.properties.blocking").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.blocking".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.get_str("jupiter_one.asset.properties.device_local_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.device_local_ip") {
                if let Some(val) = event.get("jupiter_one.asset.properties.device_local_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.device_local_ip".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.device_local_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_device_local_ip_to_ip_3375672f")?;
                        if event.remove("jupiter_one.asset.properties.device_local_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.device_local_ip".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("jupiter_one.asset.properties.device_external_ip") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("jupiter_one.asset.properties.device_external_ip") {
                if let Some(val) = event.get("jupiter_one.asset.properties.device_external_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "jupiter_one.asset.properties.device_external_ip".into(),
                            message,
                        })?;
                    event.set("jupiter_one.asset.properties.device_external_ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_jupiter_one_asset_properties_device_external_ip_to_ip_b86c1d0b")?;
                        if event.remove("jupiter_one.asset.properties.device_external_ip").is_none() {
                            return Err(TransformError::FieldNotFound { path: "jupiter_one.asset.properties.device_external_ip".into() });
                        }
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.reporter") };
            if _cond {
                event.append_unique("related.user", json!(event.get("jupiter_one.asset.properties.reporter").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.device_local_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("jupiter_one.asset.properties.device_local_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.user_id") };
            if _cond {
                event.append_unique("related.user", json!(event.get("jupiter_one.asset.properties.user_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.device_external_ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("jupiter_one.asset.properties.device_external_ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.device_hostname") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("jupiter_one.asset.properties.device_hostname").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.user_name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("jupiter_one.asset.properties.user_name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get("jupiter_one.asset.properties.approvers").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "jupiter_one.asset.properties.approvers", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }

            if let Some(v) = event.get("jupiter_one.asset.properties.level").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("log.level", v)?;
            }

            if let Some(v) = event.get("jupiter_one.asset.properties.device_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.id", v)?;
            }

            if let Some(v) = event.get("jupiter_one.asset.properties.user_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.id", v)?;
            }

            if let Some(v) = event.get("jupiter_one.asset.properties.device_os_version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.os.version", v)?;
            }

            if let Some(v) = event.get("jupiter_one.asset.properties.device_hostname").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.hostname", v)?;
            }

            if let Some(v) = event.get("jupiter_one.asset.properties.device_platform_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.os.platform", v)?;
            }

            if let Some(v) = event.get("jupiter_one.asset.properties.user_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }

            if event.has_value("log.level") {
                map_strings(event, "log.level", "log.level", str::to_lowercase)?;
            }

            let _cond = { event.get("jupiter_one.asset.entity._class").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Vulnerability")), serde_json::Value::String(s) => s.contains("Vulnerability"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("jupiter_one.asset.properties.cve_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.id", v)?;
            }
            }

            let _cond = { event.get("jupiter_one.asset.entity._class").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Vulnerability")), serde_json::Value::String(s) => s.contains("Vulnerability"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("jupiter_one.asset.properties.score").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.score.base", v)?;
            }
            }

            let _cond = { event.get("jupiter_one.asset.entity._class").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Vulnerability")), serde_json::Value::String(s) => s.contains("Vulnerability"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("jupiter_one.asset.properties.severity").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("vulnerability.severity", v)?;
            }
            }

            if event.has_value("vulnerability.severity") {
                map_strings(event, "vulnerability.severity", "vulnerability.severity", str::to_lowercase)?;
            }

            let _cond = { event.get("jupiter_one.asset.entity._class").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Finding")), serde_json::Value::String(s) => s.contains("Finding"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("jupiter_one.asset.properties.filename").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.indicator.file.name", v)?;
            }
            }

            let _cond = { event.get("jupiter_one.asset.entity._class").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Finding")), serde_json::Value::String(s) => s.contains("Finding"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("jupiter_one.asset.properties.filepath").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.indicator.file.path", v)?;
            }
            }

            let _cond = { event.get("jupiter_one.asset.entity._class").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Finding")), serde_json::Value::String(s) => s.contains("Finding"), _ => false }) };
            if _cond {
            if let Some(v) = event.get("jupiter_one.asset.properties.device_external_ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.indicator.ip", v)?;
            }
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.device_mac_address") };
            if _cond {
                event.append_unique("host.mac", json!(event.get("jupiter_one.asset.properties.device_mac_address").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.category") && event.get("jupiter_one.asset.entity._class").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Vulnerability")), serde_json::Value::String(s) => s.contains("Vulnerability"), _ => false }) };
            if _cond {
                event.append_unique("vulnerability.category", json!(event.get("jupiter_one.asset.properties.category").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.technique_id") && event.get("jupiter_one.asset.entity._class").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Finding")), serde_json::Value::String(s) => s.contains("Finding"), _ => false }) };
            if _cond {
                event.append_unique("threat.technique.id", json!(event.get("jupiter_one.asset.properties.technique_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.tactic_id") && event.get("jupiter_one.asset.entity._class").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Finding")), serde_json::Value::String(s) => s.contains("Finding"), _ => false }) };
            if _cond {
                event.append_unique("threat.tactic.id", json!(event.get("jupiter_one.asset.properties.tactic_id").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.tactic") && event.get("jupiter_one.asset.entity._class").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Finding")), serde_json::Value::String(s) => s.contains("Finding"), _ => false }) };
            if _cond {
                event.append_unique("threat.tactic.name", json!(event.get("jupiter_one.asset.properties.tactic").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.technique") && event.get("jupiter_one.asset.entity._class").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("Finding")), serde_json::Value::String(s) => s.contains("Finding"), _ => false }) };
            if _cond {
                event.append_unique("threat.technique.name", json!(event.get("jupiter_one.asset.properties.technique").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("jupiter_one.asset.properties.cve_id") };
            if _cond {
            event.set("vulnerability.enumeration", json!("CVE"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
