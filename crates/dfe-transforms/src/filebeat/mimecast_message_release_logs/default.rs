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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
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

            parse_json_field(event, "event.original", "mimecast")?;

            let _cond = { event.has_value("mimecast.released") };
            if _cond {
                if let Some(date_str) = event.get_as_string("mimecast.released") {
                    match parse_date_out(
                        &date_str,
                        &["yyyy-MM-dd'T'HH:mm:ssZ", "yyyy-MM-dd'T'HH:mm:ssZZZZZ"],
                        Some("UTC"),
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "mimecast.released".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("mimecast.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("@timestamp") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("mimecast.fromEnv.emailAddress") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("mimecast.fromEnv.emailAddress")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("mimecast.fromHdr.emailAddress") };
            if _cond {
                event.append_unique(
                    "email.from.address",
                    json!(
                        event
                            .get("mimecast.fromHdr.emailAddress")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get("mimecast.to").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "mimecast.to", |event| {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("_ingest._value.emailAddress")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if let Some(v) = event
                .get("mimecast.route")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.direction", v)?;
            }

            if let Some(v) = event
                .get("mimecast.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.local_id", v)?;
            }

            if let Some(v) = event
                .get("mimecast.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            let _cond = { event.get_str("email.direction") == Some("outbound") };
            if _cond {
                if let Some(v) = event
                    .get("email.from.address")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event.get_str("email.direction") == Some("outbound")
                    && event.has_value("mimecast.fromEnv.emailAddress")
            };
            if _cond {
                event.append(
                    "user.full_name",
                    json!(
                        event
                            .get("mimecast.fromEnv.displayableName")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("email.direction") == Some("inbound") };
            if _cond {
                if let Some(v) = event
                    .get("email.to.address")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event.get_str("email.direction") == Some("inbound")
                    && event.get("mimecast.to").is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "mimecast.to", |event| {
                    event.append_unique(
                        "user.full_name",
                        json!(
                            event
                                .get("_ingest._value.displayableName")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: def splitmail(String email) {\n  String[] parts = email.splitOnToken(\"@\");\n  if (parts.length != 2) {\n      return null;\n  }\n  return parts;\n}\ndef users = new HashSet();\ndef hosts = new HashSet();\nif (ctx.mimecast?.fromEnv?.displayableName != null) {\n  users.add(ctx.mimecast.fromEnv.displayableName);\n}\nif (ctx.mimecast?.operator instanceof Map) {\n  // mimecast.operator is now an object, so to avoid\n  // breaking mappings, move the email address to its\n  // root.\n  ctx.mimecast.operator = ctx.mimecast.operator.emailAddress;\n}\nif (ctx.mimecast?.operator != null) {\n  def parts = splitmail(ctx.mimecast.operator);\n  if (parts != null) {\n    users.add(parts[0]);\n    hosts.add(parts[1]);\n  }\n  users.add(ctx.mimecast.operator);\n}\nif (ctx.mimecast?.fromEnv?.emailAddress != null) {\n  def parts = splitmail(ctx.mimecast.fromEnv.emailAddress);\n  if (parts != null) {\n    users.add(parts[0]);\n    hosts.add(parts[1]);\n  }\n  users.add(ctx.mimecast.fromEnv.emailAddress);\n}\nif (ctx.mimecast?.fromHdr?.displayableName != null) {\n  users.add(ctx.mimecast.fromHdr.displayableName);\n}\nif (ctx.mimecast?.fromHdr?.emailAddress != null) {\n  def parts = splitmail(ctx.mimecast.fromHdr.emailAddress);\n  if (parts != null) {\n    users.add(parts[0]);\n    hosts.add(parts[1]);\n  }\n  users.add(ctx.mimecast.fromHdr.emailAddress);\n}\nfor (def to: ctx.mimecast.to) {\n  if (to.displayableName != null) {\n    users.add(to.displayableName);\n  }\n  if (to.emailAddress != null) {\n    def parts = splitmail(to.emailAddress);\n    if (parts != null) {\n      users.add(parts[0]);\n      hosts.add(parts[1]);\n    }\n    users.add(to.emailAddress);\n  }\n}\nif (users.size() != 0 || hosts.size() != 0) {\n  if (ctx.related == null) {\n    ctx.related = new HashMap();\n  }\n  if (users.size() != 0 && ctx.related.user == null) {\n    ctx.related.user = new ArrayList();\n    for (def u: users) {\n      ctx.related.user.add(u);\n    }\n    Collections.sort(ctx.related.user);\n  }\n  if (hosts.size() != 0 && ctx.related.hosts == null) {\n    ctx.related.hosts = new ArrayList();\n    for (def h: hosts) {\n      ctx.related.hosts.add(h);\n    }\n    Collections.sort(ctx.related.hosts);\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def splitmail(String email) {\n  String[] parts = email.splitOnToken(\"@\");\n  if (parts.length != 2) {\n      return null;\n  }\n  return parts;\n}\ndef users = new HashSet();\ndef hosts = new HashSet();\nif (ctx.mimecast?.fromEnv?.displayableName != null) {\n  users.add(ctx.mimecast.fromEnv.displayableName);\n}\nif (ctx.mimecast?.operator instanceof Map) {\n  // mimecast.operator is now an object, so to avoid\n  // breaking mappings, move the email address to its\n  // root.\n  ctx.mimecast.operator = ctx.mimecast.operator.emailAddress;\n}\nif (ctx.mimecast?.operator != null) {\n  def parts = splitmail(ctx.mimecast.operator);\n  if (parts != null) {\n    users.add(parts[0]);\n    hosts.add(parts[1]);\n  }\n  users.add(ctx.mimecast.operator);\n}\nif (ctx.mimecast?.fromEnv?.emailAddress != null) {\n  def parts = splitmail(ctx.mimecast.fromEnv.emailAddress);\n  if (parts != null) {\n    users.add(parts[0]);\n    hosts.add(parts[1]);\n  }\n  users.add(ctx.mimecast.fromEnv.emailAddress);\n}\nif (ctx.mimecast?.fromHdr?.displayableName != null) {\n  users.add(ctx.mimecast.fromHdr.displayableName);\n}\nif (ctx.mimecast?.fromHdr?.emailAddress != null) {\n  def parts = splitmail(ctx.mimecast.fromHdr.emailAddress);\n  if (parts != null) {\n    users.add(parts[0]);\n    hosts.add(parts[1]);\n  }\n  users.add(ctx.mimecast.fromHdr.emailAddress);\n}\nfor (def to: ctx.mimecast.to) {\n  if (to.displayableName != null) {\n    users.add(to.displayableName);\n  }\n  if (to.emailAddress != null) {\n    def parts = splitmail(to.emailAddress);\n    if (parts != null) {\n      users.add(parts[0]);\n      hosts.add(parts[1]);\n    }\n    users.add(to.emailAddress);\n  }\n}\nif (users.size() != 0 || hosts.size() != 0) {\n  if (ctx.related == null) {\n    ctx.related = new HashMap();\n  }\n  if (users.size() != 0 && ctx.related.user == null) {\n    ctx.related.user = new ArrayList();\n    for (def u: users) {\n      ctx.related.user.add(u);\n    }\n    Collections.sort(ctx.related.user);\n  }\n  if (hosts.size() != 0 && ctx.related.hosts == null) {\n    ctx.related.hosts = new ArrayList();\n    for (def h: hosts) {\n      ctx.related.hosts.add(h);\n    }\n    Collections.sort(ctx.related.hosts);\n  }\n}\n"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("mimecast.spamProcessingDetail.verdict") {
                    event.rename(
                        "mimecast.spamProcessingDetail.verdict",
                        "mimecast.spamProcessingDetail.spamVerdict",
                    )?;
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("mimecast.status")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            event.set("event.category", Value::Array(vec![json!("email")]))?;

            if let Some(v) = event.get("mimecast.id").cloned() {
                event.set("event.id", v)?;
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("mimecast.rejectReason")
                    && event.get_str("mimecast.rejectReason") != Some("")
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if let Some(v) = event
                .get("mimecast.rejectReason")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            if let Some(v) = event
                .get("mimecast.spamScore")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.risk_score", v)?;
            }

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                    ),
                )?;
                Ok(())
            })();

            event.rename("mimecast", "mimecast.message_release_logs")?;

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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
