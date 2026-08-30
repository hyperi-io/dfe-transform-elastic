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

            event.set("event.kind", json!("event"))?;

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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.surfaced_timestamp") {
                    match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.surfaced_timestamp".into(),
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
                    "date_surfaced_timestamp",
                )?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.sekey") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.surfaced_timestamp") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.bypass_status_enabled") {
                event.rename(
                    "json.bypass_status_enabled",
                    "cisco_duo.trust_monitor.bypass_status_enabled",
                )?;
            }

            if event.has_value("json.enabled_by") {
                event.rename("json.enabled_by", "cisco_duo.trust_monitor.enabled_by")?;
            }

            if event.has_value("json.enabled_for") {
                event.rename("json.enabled_for", "cisco_duo.trust_monitor.enabled_for")?;
            }

            if event.has_value("json.explanations") {
                event.rename("json.explanations", "cisco_duo.trust_monitor.explanations")?;
            }

            if event.has_value("json.from_common_netblock") {
                event.rename(
                    "json.from_common_netblock",
                    "cisco_duo.trust_monitor.from_common_netblock",
                )?;
            }

            if event.has_value("json.from_new_user") {
                event.rename(
                    "json.from_new_user",
                    "cisco_duo.trust_monitor.from_new_user",
                )?;
            }

            if event.has_value("json.low_risk_ip") {
                event.rename("json.low_risk_ip", "cisco_duo.trust_monitor.low_risk_ip")?;
            }

            if event.has_value("json.priority_event") {
                event.rename(
                    "json.priority_event",
                    "cisco_duo.trust_monitor.priority_event",
                )?;
            }

            if event.has_value("json.priority_reasons") {
                event.rename(
                    "json.priority_reasons",
                    "cisco_duo.trust_monitor.priority_reasons",
                )?;
            }

            if event.has_value("json.sekey") {
                event.rename("json.sekey", "cisco_duo.trust_monitor.sekey")?;
            }

            if event.has_value("json.state") {
                event.rename("json.state", "cisco_duo.trust_monitor.state")?;
            }

            if event.has_value("json.state_updated_timestamp") {
                event.rename(
                    "json.state_updated_timestamp",
                    "cisco_duo.trust_monitor.state_updated_timestamp",
                )?;
            }

            if event.has_value("json.surfaced_auth") {
                event.rename(
                    "json.surfaced_auth",
                    "cisco_duo.trust_monitor.surfaced_auth",
                )?;
            }

            if event.has_value("json.triaged_as_interesting") {
                event.rename(
                    "json.triaged_as_interesting",
                    "cisco_duo.trust_monitor.triaged_as_interesting",
                )?;
            }

            if event.has_value("json.triage_event_uri") {
                event.rename(
                    "json.triage_event_uri",
                    "cisco_duo.trust_monitor.triage_event_uri",
                )?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "cisco_duo.trust_monitor.type")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("cisco_duo.trust_monitor.sekey").cloned() {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_duo.trust_monitor.triage_event_uri") {
                    uri_parts(
                        event,
                        "cisco_duo.trust_monitor.triage_event_uri",
                        "url",
                        true,
                        false,
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                    ),
                )?;
                Ok(())
            })();

            event.remove("json");

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
