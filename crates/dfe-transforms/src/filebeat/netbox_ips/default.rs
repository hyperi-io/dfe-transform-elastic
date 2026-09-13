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

            parse_json_field(event, "event.original", "netbox.ip")?;

            if event.has_value("netbox.ip.address") {
                gsub_field(
                    event,
                    "netbox.ip.address",
                    "netbox.ip.address",
                    cached_regex!("/\\d+$"),
                    "",
                )?;
            }

            event.append_unique(
                "related.ip",
                json!(
                    event
                        .get("netbox.ip.address")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("netbox.ip.nat_inside.address") {
                gsub_field(
                    event,
                    "netbox.ip.nat_inside.address",
                    "netbox.ip.nat_inside.address",
                    cached_regex!("/\\d+$"),
                    "",
                )?;
            }

            let _cond = { event.has_value("netbox.ip.nat_inside.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netbox.ip.nat_inside.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("netbox.ip.nat_outside") {
                foreach_array(event, "netbox.ip.nat_outside", |event| {
                    if event.has_value("_ingest._value.address") {
                        gsub_field(
                            event,
                            "_ingest._value.address",
                            "_ingest._value.address",
                            cached_regex!("/\\d+$"),
                            "",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("netbox.ip.nat_outside") {
                foreach_array(event, "netbox.ip.nat_outside", |event| {
                    let _cond = { event.has_value("_ingest._value") };
                    if _cond {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("_ingest._value.address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("netbox.ip.url") {
                event.rename("netbox.ip.url", "netbox.url")?;
            }

            if event.has_value("netbox.ip.display_url") {
                event.rename("netbox.ip.display_url", "netbox.display_url")?;
            }

            if event.has_value("netbox.ip.display") {
                event.rename("netbox.ip.display", "netbox.display")?;
            }

            if event.has_value("netbox.ip.comments") {
                event.rename("netbox.ip.comments", "netbox.comments")?;
            }

            if event.has_value("netbox.ip.created") {
                event.rename("netbox.ip.created", "netbox.created")?;
            }

            if event.has_value("netbox.ip.last_updated") {
                event.rename("netbox.ip.last_updated", "netbox.last_updated")?;
            }

            if event.has_value("netbox.ip.custom_fields") {
                event.rename("netbox.ip.custom_fields", "netbox.custom_fields")?;
            }

            if event.has_value("netbox.ip.tags") {
                event.rename("netbox.ip.tags", "netbox.tags")?;
            }

            if event.has_value("netbox.ip.tenant") {
                event.rename("netbox.ip.tenant", "netbox.tenant")?;
            }

            event.set("event.kind", json!("asset"))?;

            event.append("event.category", json!("configuration"))?;

            let _cond = { event.has_value("netbox.created") };
            if _cond {
                if let Some(date_str) = event.get_as_string("netbox.created") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("netbox.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netbox.created".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("netbox.last_updated") };
            if _cond {
                if let Some(date_str) = event.get_as_string("netbox.last_updated") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("netbox.last_updated", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "netbox.last_updated".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == \"\") {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
