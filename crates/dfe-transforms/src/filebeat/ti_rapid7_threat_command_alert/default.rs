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

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("alert"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

            let _cond = { event.has_value("json.UpdateDate") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.UpdateDate") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("rapid7.tc.alert.update_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.UpdateDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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
                if let Some(v) = event.get("rapid7.tc.alert.update_date").cloned() {
                    event.set("@timestamp", v)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "set")?;
                if let Some(v) = event.get("_ingest.timestamp").cloned() {
                    event.set("@timestamp", v)?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            event.set("event.module", json!("ti_rapid7_threat_command"))?;

            if event.has_value("json._id") {
                event.rename("json._id", "rapid7.tc.alert.id")?;
            }

            if event.has_value("json.Assets") {
                event.rename("json.Assets", "rapid7.tc.alert.assets")?;
            }

            if event.has_value("json.Assignees") {
                event.rename("json.Assignees", "rapid7.tc.alert.assignees")?;
            }

            if event.has_value("json.Details.Type") {
                event.rename("json.Details.Type", "rapid7.tc.alert.details.type")?;
            }

            if event.has_value("json.Details.SubType") {
                event.rename("json.Details.SubType", "rapid7.tc.alert.details.subtype")?;
            }

            if event.has_value("json.Details.Title") {
                event.rename("json.Details.Title", "rapid7.tc.alert.details.title")?;
            }

            if event.has_value("json.Details.Description") {
                event.rename(
                    "json.Details.Description",
                    "rapid7.tc.alert.details.description",
                )?;
            }

            if event.has_value("json.Details.Severity") {
                event.rename("json.Details.Severity", "rapid7.tc.alert.details.severity")?;
            }

            if event.has_value("json.Details.Images") {
                event.rename("json.Details.Images", "rapid7.tc.alert.details.images")?;
            }

            if event.has_value("json.Details.Source.Type") {
                event.rename(
                    "json.Details.Source.Type",
                    "rapid7.tc.alert.details.source.type",
                )?;
            }

            if event.has_value("json.Details.Source.URL") {
                event.rename(
                    "json.Details.Source.URL",
                    "rapid7.tc.alert.details.source.url",
                )?;
            }

            if event.has_value("json.Details.Source.Email") {
                event.rename(
                    "json.Details.Source.Email",
                    "rapid7.tc.alert.details.source.email",
                )?;
            }

            if event.has_value("json.Details.Source.NetworkType") {
                event.rename(
                    "json.Details.Source.NetworkType",
                    "rapid7.tc.alert.details.source.network_type",
                )?;
            }

            if event.has_value("json.Details.Source.LeakName") {
                event.rename(
                    "json.Details.Source.LeakName",
                    "rapid7.tc.alert.details.source.leak_name",
                )?;
            }

            if event.has_value("json.Details.Tags") {
                event.rename("json.Details.Tags", "rapid7.tc.alert.details.tags")?;
            }

            if event.has_value("json.TakedownStatus") {
                event.rename("json.TakedownStatus", "rapid7.tc.alert.takedown_status")?;
            }

            if event.has_value("json.RelatedIocs") {
                event.rename("json.RelatedIocs", "rapid7.tc.alert.related_iocs")?;
            }

            if event.has_value("json.RelatedThreatIDs") {
                event.rename(
                    "json.RelatedThreatIDs",
                    "rapid7.tc.alert.related_threat_ids",
                )?;
            }

            let _cond = { event.has_value("json.Details.Source.Date") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.Details.Source.Date") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("rapid7.tc.alert.details.source.date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.Details.Source.Date".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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

            let _cond = { event.has_value("json.FoundDate") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.FoundDate") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("rapid7.tc.alert.found_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.FoundDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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
                if event.has_value("json.Closed.IsClosed") {
                    if let Some(val) = event.get("json.Closed.IsClosed") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.Closed.IsClosed".into(),
                                message,
                            }
                        })?;
                        event.set("rapid7.tc.alert.is_closed", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.IsFlagged") {
                    if let Some(val) = event.get("json.IsFlagged") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.IsFlagged".into(),
                                message,
                            }
                        })?;
                        event.set("rapid7.tc.alert.is_flagged", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "rapid7.tc.alert.assets", |event| {
                    if event.has_value("_ingest._value.Type") {
                        event.rename("_ingest._value.Type", "_ingest._value.type")?;
                    }
                    Ok(())
                })?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "rapid7.tc.alert.assets", |event| {
                    if event.has_value("_ingest._value.Value") {
                        event.rename("_ingest._value.Value", "_ingest._value.value")?;
                    }
                    Ok(())
                })?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "rapid7.tc.alert.details.tags", |event| {
                    if event.has_value("_ingest._value.CreatedBy") {
                        event.rename("_ingest._value.CreatedBy", "_ingest._value.created_by")?;
                    }
                    Ok(())
                })?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "rapid7.tc.alert.details.tags", |event| {
                    if event.has_value("_ingest._value.Name") {
                        event.rename("_ingest._value.Name", "_ingest._value.name")?;
                    }
                    Ok(())
                })?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "rapid7.tc.alert.details.tags", |event| {
                    if event.has_value("_ingest._value._id") {
                        event.rename("_ingest._value._id", "_ingest._value.id")?;
                    }
                    Ok(())
                })?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "rapid7.tc.alert.details.tags", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "tags",
                            json!(
                                event
                                    .get("_ingest._value.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                    Ok(())
                })?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("rapid7.tc.alert.id").cloned() {
                    event.set("event.id", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.set("event.reference", json!(format!("https://dashboard.ti.insight.rapid7.com/#/threat-command/alerts/?search={}", event.get("rapid7.tc.alert.id").map_or_else(String::new, template_to_string))))?;
                Ok(())
            })();

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("rapid7.tc.alert.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("rapid7.tc.alert.update_date") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            event.remove("json");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n} drop(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean drop(Object o) {\n  if (o == null || o == \"\") {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n} drop(ctx);\n"#
                    ),
                )?;
                Ok(())
            })();

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
