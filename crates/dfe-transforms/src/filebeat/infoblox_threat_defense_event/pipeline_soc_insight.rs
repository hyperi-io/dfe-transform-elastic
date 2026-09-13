// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_soc_insight` pipeline.
pub struct PipelineSocInsight;

impl Transform for PipelineSocInsight {
    fn name(&self) -> &str {
        "pipeline_soc_insight"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                event.append("event.type", json!("indicator"))?;

                event.append("event.category", json!("threat"))?;

                if event.has_value("cef.extensions.deviceEventCategory") {
                    event.rename("cef.extensions.deviceEventCategory", "infoblox_threat_defense.event.device.event_category")?;
                }

            let _cond = { event.has_value("cef.extensions.InfobloxEventOccurredTime") && event.get_str("cef.extensions.InfobloxEventOccurredTime") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("cef.extensions.InfobloxEventOccurredTime") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("infoblox_threat_defense.event.infoblox.event.occurred_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cef.extensions.InfobloxEventOccurredTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_cef_extensions_InfobloxEventOccurredTime")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("infoblox_threat_defense.event.infoblox.event.occurred_time").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("@timestamp", v)?;
            }

            if let Some(v) = event.get("@timestamp").cloned() {
                event.set("event.created", v)?;
            }

                if event.has_value("cef.extensions.InfobloxInsightDescription") {
                    event.rename("cef.extensions.InfobloxInsightDescription", "infoblox_threat_defense.event.infoblox.insight.description")?;
                }

                if event.has_value("cef.extensions.InfobloxInsightFeedSource") {
                    event.rename("cef.extensions.InfobloxInsightFeedSource", "infoblox_threat_defense.event.infoblox.insight.feed_source")?;
                }

                if event.has_value("cef.extensions.InfobloxInsightId") {
                    event.rename("cef.extensions.InfobloxInsightId", "infoblox_threat_defense.event.infoblox.insight.id")?;
                }

            let _cond = { event.has_value("infoblox_threat_defense.event.infoblox.insight.id") };
            if _cond {
                event.append_unique("threat.indicator.id", json!(event.get("infoblox_threat_defense.event.infoblox.insight.id").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("cef.extensions.InfobloxInsightStatus").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("infoblox_threat_defense.event.infoblox.insight.status", v)?;
            }

            if let Some(v) = event.get("cef.extensions.status").filter(|v| !painless_is_empty_value(v)).cloned() {
                if !event.has("infoblox_threat_defense.event.infoblox.insight.status") {
                    event.set("infoblox_threat_defense.event.infoblox.insight.status", v)?;
                }
            }

                if event.has_value("cef.extensions.InfobloxInsightThreatType") {
                    event.rename("cef.extensions.InfobloxInsightThreatType", "infoblox_threat_defense.event.infoblox.insight.threat_type")?;
                }

                if event.has_value("cef.extensions.InfobloxInsightUserComment") {
                    event.rename("cef.extensions.InfobloxInsightUserComment", "infoblox_threat_defense.event.infoblox.insight.user_comment")?;
                }

                if event.has_value("cef.extensions.InfobloxEventsBlockedCount") {
                    event.rename("cef.extensions.InfobloxEventsBlockedCount", "infoblox_threat_defense.event.infoblox.stats.events_blocked_count")?;
                }

                if event.has_value("cef.extensions.InfobloxEventsNotBlockedCount") {
                    event.rename("cef.extensions.InfobloxEventsNotBlockedCount", "infoblox_threat_defense.event.infoblox.stats.events_not_blocked_count")?;
                }

                if event.has_value("cef.extensions.InfobloxThreatClass") {
                    event.rename("cef.extensions.InfobloxThreatClass", "infoblox_threat_defense.event.infoblox.threat.class")?;
                }

            let _cond = { event.get_str("cef.extensions.InfobloxThreatConfidence") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cef.extensions.InfobloxThreatConfidence") {
                if let Some(val) = event.get("cef.extensions.InfobloxThreatConfidence") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cef.extensions.InfobloxThreatConfidence".into(),
                            message,
                        })?;
                    event.set("infoblox_threat_defense.event.infoblox.threat.confidence", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cef_extensions_InfobloxThreatConfidence_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("cef.extensions.InfobloxThreatLevel") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cef.extensions.InfobloxThreatLevel") {
                if let Some(val) = event.get("cef.extensions.InfobloxThreatLevel") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cef.extensions.InfobloxThreatLevel".into(),
                            message,
                        })?;
                    event.set("infoblox_threat_defense.event.infoblox.threat.level", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cef_extensions_InfobloxThreatLevel_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("cef.extensions.InfobloxThreatFamily") {
                    event.rename("cef.extensions.InfobloxThreatFamily", "infoblox_threat_defense.event.infoblox.threat.family")?;
                }

                if event.has_value("cef.extensions.message") {
                    event.rename("cef.extensions.message", "infoblox_threat_defense.event.message")?;
                }

            if let Some(v) = event.get("infoblox_threat_defense.event.message").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.indicator.description", v)?;
            }

            let _cond = { event.get_str("cef.extensions.baseEventCount") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cef.extensions.baseEventCount") {
                if let Some(val) = event.get("cef.extensions.baseEventCount") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "cef.extensions.baseEventCount".into(),
                            message,
                        })?;
                    event.set("infoblox_threat_defense.event.stats.base_event_count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cef_extensions_baseEventCount_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
