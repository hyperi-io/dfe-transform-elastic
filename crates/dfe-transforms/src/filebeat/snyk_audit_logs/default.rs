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

            parse_json_field(event, "event.original", "json")?;

            if event.has_value("json.groupId") {
                event.rename("json.groupId", "json.group_id")?;
            }

            if event.has_value("json.orgId") {
                event.rename("json.orgId", "json.org_id")?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.created") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.event") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.group_id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.org_id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json") {
                event.rename("json", "snyk.audit_logs")?;
            }

            if event.has_value("snyk.audit_logs.group_id") {
                event.rename("snyk.audit_logs.group_id", "user.group.id")?;
            }

            if let Some(v) = event
                .get("snyk.audit_logs.org_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            if event.has_value("snyk.audit_logs.projectId") {
                event.rename("snyk.audit_logs.projectId", "snyk.audit_logs.project_id")?;
            }

            if let Some(v) = event
                .get("snyk.audit_logs.user_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if event.has_value("snyk.audit_logs.event") {
                event.rename("snyk.audit_logs.event", "event.action")?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event
                        .get_str("event.action")
                        .is_some_and(|s| cached_regex!(r"\buser\b").is_match(s))
            };
            if _cond {
                event.append_unique("event.type", json!("user"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event
                        .get_str("event.action")
                        .is_some_and(|s| cached_regex!(r"\b(?:add|create)\b").is_match(s))
            };
            if _cond {
                event.append_unique("event.type", json!("creation"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event
                        .get_str("event.action")
                        .is_some_and(|s| cached_regex!(r"\bedit\b").is_match(s))
            };
            if _cond {
                event.append_unique("event.type", json!("change"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event
                        .get_str("event.action")
                        .is_some_and(|s| cached_regex!(r"\baccess\b").is_match(s))
            };
            if _cond {
                event.append_unique("event.type", json!("access"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event
                        .get_str("event.action")
                        .is_some_and(|s| cached_regex!(r"\b(?:remove|delete)\b").is_match(s))
            };
            if _cond {
                event.append_unique("event.type", json!("deletion"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event
                        .get_str("event.action")
                        .is_some_and(|s| cached_regex!(r"\bsettings\b").is_match(s))
            };
            if _cond {
                event.append_unique("event.category", json!("configuration"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event
                        .get_str("event.action")
                        .is_some_and(|s| cached_regex!(r"\bfiles\b").is_match(s))
            };
            if _cond {
                event.append_unique("event.category", json!("file"))?;
            }

            let _cond = {
                event.has_value("event.action")
                    && event
                        .get_str("event.action")
                        .is_some_and(|s| cached_regex!(r"\buser\b").is_match(s))
            };
            if _cond {
                event.append_unique("event.category", json!("iam"))?;
            }

            let _cond = { !event.has_value("event.type") };
            if _cond {
                event.set("event.type", Value::Array(vec![json!("info")]))?;
            }

            if let Some(date_str) = event.get_as_string("snyk.audit_logs.created") {
                match parse_date_out(&date_str, &["yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "snyk.audit_logs.created".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            let _cond = { event.has_value("snyk.audit_logs.user_id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("snyk.audit_logs.user_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("snyk.audit_logs.content.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("snyk.audit_logs.content.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("snyk.audit_logs.content.userPublicId") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("snyk.audit_logs.content.userPublicId")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("snyk.audit_logs.content.url") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if !uri_parts(event, "snyk.audit_logs.content.url", "url", true, false)?
                        && event
                            .get_str("snyk.audit_logs.content.url")
                            .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "snyk.audit_logs.content.url".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uri_parts")?;
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

            let _cond = {
                event
                    .get("snyk.audit_logs.content.notSupported")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // Painless script
                // Source: def blank = ctx.snyk.audit_logs.content.notSupported[''];\nif (blank == null) {\n  return;\n}\nctx.snyk.audit_logs.content.notSupported.no_extension = ctx.snyk.audit_logs.content.notSupported[''];\nctx.snyk.audit_logs.content.notSupported.remove('');
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def blank = ctx.snyk.audit_logs.content.notSupported[''];\nif (blank == null) {\n  return;\n}\nctx.snyk.audit_logs.content.notSupported.no_extension = ctx.snyk.audit_logs.content.notSupported[''];\nctx.snyk.audit_logs.content.notSupported.remove('');"#
                    ),
                )?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
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

            event.remove("snyk.audit_logs.created");
            event.remove("message");
            event.remove("json");

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
