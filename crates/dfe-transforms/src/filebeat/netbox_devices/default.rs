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

            parse_json_field(event, "event.original", "netbox.device")?;

            if event.has_value("netbox.device.primary_ip.address") {
                gsub_field(
                    event,
                    "netbox.device.primary_ip.address",
                    "netbox.device.primary_ip.address",
                    cached_regex!("/\\d+$"),
                    "",
                )?;
            }

            let _cond = { event.has_value("netbox.device.primary_ip.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netbox.device.primary_ip.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("netbox.device.primary_ip4.address") {
                gsub_field(
                    event,
                    "netbox.device.primary_ip4.address",
                    "netbox.device.primary_ip4.address",
                    cached_regex!("/\\d+$"),
                    "",
                )?;
            }

            let _cond = { event.has_value("netbox.device.primary_ip4.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netbox.device.primary_ip4.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("netbox.device.primary_ip6.address") {
                gsub_field(
                    event,
                    "netbox.device.primary_ip6.address",
                    "netbox.device.primary_ip6.address",
                    cached_regex!("/\\d+$"),
                    "",
                )?;
            }

            let _cond = { event.has_value("netbox.device.primary_ip6.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netbox.device.primary_ip6.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("netbox.device.oob_ip.address") {
                gsub_field(
                    event,
                    "netbox.device.oob_ip.address",
                    "netbox.device.oob_ip.address",
                    cached_regex!("/\\d+$"),
                    "",
                )?;
            }

            let _cond = { event.has_value("netbox.device.oob_ip.address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("netbox.device.oob_ip.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("netbox.device.longitude")
                    && event.has_value("netbox.device.latitude")
            };
            if _cond {
                // Painless script
                // Source: ctx.netbox.device.coordinates = [\n  ctx.netbox.device.longitude,\n  ctx.netbox.device.latitude\n]\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.netbox.device.coordinates = [\n  ctx.netbox.device.longitude,\n  ctx.netbox.device.latitude\n]\n"#
                    ),
                )?;
            }

            event.remove("netbox.device.latitude");
            event.remove("netbox.device.longitude");

            if event.has_value("netbox.device.url") {
                event.rename("netbox.device.url", "netbox.url")?;
            }

            if event.has_value("netbox.device.display_url") {
                event.rename("netbox.device.display_url", "netbox.display_url")?;
            }

            if event.has_value("netbox.device.display") {
                event.rename("netbox.device.display", "netbox.display")?;
            }

            if event.has_value("netbox.device.comments") {
                event.rename("netbox.device.comments", "netbox.comments")?;
            }

            if event.has_value("netbox.device.created") {
                event.rename("netbox.device.created", "netbox.created")?;
            }

            if event.has_value("netbox.device.last_updated") {
                event.rename("netbox.device.last_updated", "netbox.last_updated")?;
            }

            if event.has_value("netbox.device.custom_fields") {
                event.rename("netbox.device.custom_fields", "netbox.custom_fields")?;
            }

            if event.has_value("netbox.device.tags") {
                event.rename("netbox.device.tags", "netbox.tags")?;
            }

            if event.has_value("netbox.device.tenant") {
                event.rename("netbox.device.tenant", "netbox.tenant")?;
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
