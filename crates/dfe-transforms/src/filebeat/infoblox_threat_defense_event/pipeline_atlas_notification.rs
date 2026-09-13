// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_atlas_notification` pipeline.
pub struct PipelineAtlasNotification;

impl Transform for PipelineAtlasNotification {
    fn name(&self) -> &str {
        "pipeline_atlas_notification"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
                if event.has_value("cef.extensions.deviceEventCategory") {
                    event.rename("cef.extensions.deviceEventCategory", "infoblox_threat_defense.event.device.event_category")?;
                }

                if event.has_value("cef.extensions.message") {
                    event.rename("cef.extensions.message", "infoblox_threat_defense.event.message")?;
                }

                if event.has_value("cef.extensions.status") {
                    event.rename("cef.extensions.status", "infoblox_threat_defense.event.status")?;
                }

                if event.has_value("cef.extensions.InfobloxNotificationType") {
                    event.rename("cef.extensions.InfobloxNotificationType", "infoblox_threat_defense.event.infoblox.notification.type")?;
                }

                if event.has_value("cef.extensions.InfobloxNotificationSubType") {
                    event.rename("cef.extensions.InfobloxNotificationSubType", "infoblox_threat_defense.event.infoblox.notification.sub_type")?;
                }

                if event.has_value("cef.extensions.InfobloxOnPremHostName") {
                    event.rename("cef.extensions.InfobloxOnPremHostName", "infoblox_threat_defense.event.infoblox.on_prem_host_name")?;
                }

            let _cond = { event.get_str("infoblox_threat_defense.event.infoblox.on_prem_host_name") != Some("(none)") };
            if _cond {
            if let Some(v) = event.get("infoblox_threat_defense.event.infoblox.on_prem_host_name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.hostname", v)?;
            }
            }

            let _cond = { event.has_value("infoblox_threat_defense.event.infoblox.on_prem_host_name") && event.get_str("infoblox_threat_defense.event.infoblox.on_prem_host_name") != Some("(none)") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("infoblox_threat_defense.event.infoblox.on_prem_host_name").map_or_else(String::new, template_to_string)))?;
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
