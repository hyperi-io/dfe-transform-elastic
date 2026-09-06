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

            event.set("event.kind", json!("alert"))?;

            parse_json_field(event, "event.original", "json")?;

            if event.has_value("json.event.event_id") {
                event.rename("json.event.event_id", "event.id")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.event.event_time") {
                    match parse_date_out(&date_str, &["ISO8601"], Some("UTC"), None) {
                        Some(parsed) => event.set("event.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.event.event_time".into(),
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
                    "date_event_created_time_epoch",
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

            let _cond = { event.has_value("json.event.dst_url") };
            if _cond {
                uri_parts(event, "json.event.dst_url", "url", true, true)?;
            }

            if let Some(domain_str) = event.get_string("url.domain") {
                let domain = domain_str.to_string();
                event.set("url.domain", json!(domain.clone()))?;
                // Public suffix list lookup for registered domain extraction
                if let Some(rd) = registered_domain_lookup(&domain) {
                    if let Some(registered) = rd.registered_domain {
                        event.set("url.registered_domain", json!(registered))?;
                    }
                    event.set("url.top_level_domain", json!(rd.top_level_domain))?;
                    if let Some(sub) = rd.subdomain {
                        event.set("url.subdomain", json!(sub))?;
                    }
                }
            }

            if event.has_value("json.event.rule_id") {
                event.rename("json.event.rule_id", "rule.id")?;
            }

            if event.has_value("json.event.action") {
                event.rename("json.event.action", "event.action")?;
            }

            let _cond = { event.get_str("json.event.action") == Some("block") };
            if _cond {
                event.set("event.action", json!("blocked"))?;
            }

            let _cond = { event.get_str("json.event.action") == Some("log") };
            if _cond {
                event.set("event.action", json!("log"))?;
            }

            let _cond = { event.get_str("event.action") == Some("blocked") };
            if _cond {
                event.set("event.type", json!("denied"))?;
            }

            if event.has_value("json.event.rule_name") {
                event.rename("json.event.rule_name", "rule.name")?;
            }

            event.set("event.category", json!("intrusion_detection"))?;

            event.append("event.category", json!("network"))?;

            if event.has_value("json.event.severity") {
                if let Some(val) = event.get("json.event.severity") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.event.severity".into(),
                            message,
                        }
                    })?;
                    event.set("event.severity", converted)?;
                }
            }

            event.set("event.outcome", json!("unknown"))?;

            let _cond = {
                event.get_str("event.action") == Some("log")
                    || event.get_str("event.action") == Some("block")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            if event.has_value("json.event.product") {
                event.rename("json.event.product", "observer.product")?;
            }

            if event.has_value("json.event.vendor") {
                event.rename("json.event.vendor", "observer.vendor")?;
            }

            if event.has_value("json.event.version") {
                event.rename("json.event.version", "observer.version")?;
            }

            if event.has_value("json.event.request_type") {
                event.rename("json.event.request_type", "http.request.method")?;
            }

            if event.has_value("menlo.protocol") {
                event.rename("menlo.protocol", "network.protocol")?;
            }

            if event.has_value("json.event.userid") {
                event.rename("json.event.userid", "user.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("user.id") {
                    if let Some(input) = event.get_string("user.id") {
                        // Grok pattern: %{USERNAME:user.name}@%{HOSTNAME:user.domain}
                        // Grok pattern: %{GREEDYDATA:user.name}
                        if !extract_first_match(
                            &[
                                cached_grok!("%{USERNAME:user.name}@%{HOSTNAME:user.domain}"),
                                cached_grok!("%{GREEDYDATA:user.name}"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("user.id")
                    && event
                        .get_str("user.id")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("user.id").cloned() {
                    event.set("user.email", v)?;
                }
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.event.filename") {
                event.rename("json.event.filename", "file.name")?;
            }

            let _cond = { event.get_str("json.sha256") != Some("NA") };
            if _cond {
                if event.has_value("json.event.sha256") {
                    event.rename("json.event.sha256", "file.hash.sha256")?;
                }
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.event.categories") {
                event.rename("json.event.categories", "menlo.dlp.category")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.event.ccl_match_counts") {
                    if let Some(val) = event.get("json.event.ccl_match_counts") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.event.ccl_match_counts".into(),
                                message,
                            }
                        })?;
                        event.set("menlo.dlp.ccl.match_counts", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has_value("json.event.user_input") {
                event.rename("json.event.user_input", "menlo.dlp.user_input")?;
            }

            if event.has_value("json.event.alerted") {
                event.rename("json.event.alerted", "menlo.dlp.alerted")?;
            }

            if event.has_value("json.event.status") {
                event.rename("json.event.status", "menlo.dlp.status")?;
            }

            if event.has_value("json.event.ccl_scores") {
                if let Some(val) = event.get("json.event.ccl_scores") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.event.ccl_scores".into(),
                            message,
                        }
                    })?;
                    event.set("menlo.dlp.ccl.score", converted)?;
                }
            }

            event.rename("json.event.ccl_ids", "menlo.dlp.ccl.id")?;

            event.rename("json.event.stream_name", "menlo.dlp.stream_name")?;

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
