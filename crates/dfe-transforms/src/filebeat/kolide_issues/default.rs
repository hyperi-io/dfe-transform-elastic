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
            event.set("ecs.version", json!("9.4.0"))?;

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

            let _cond = { event.get("event").is_some_and(|v| v.is_string()) };
            if _cond {
                if event.has_value("event") {
                    event.rename("event", "json.event")?;
                }
            }

            let _cond = { event.has_value("json.event") && event.has_value("data") };
            if _cond {
                // Painless script
                // Source: if (ctx.containsKey('id')) { ctx.json.id = ctx.remove('id'); }\nif (ctx.containsKey('timestamp')) { ctx.json.timestamp = ctx.remove('timestamp'); }\nif (ctx.containsKey('data')) { ctx.json.data = ctx.remove('data'); }
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.containsKey('id')) { ctx.json.id = ctx.remove('id'); }\nif (ctx.containsKey('timestamp')) { ctx.json.timestamp = ctx.remove('timestamp'); }\nif (ctx.containsKey('data')) { ctx.json.data = ctx.remove('data'); }"#
                    ),
                )?;
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

            let _cond = { !event.has_value("json") && event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            let _cond = { event.has_value("json.timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond =
                { !event.has_value("json.timestamp") && event.has_value("json.detected_at") };
            if _cond {
                if let Some(v) = event.get("_ingest.timestamp").cloned() {
                    event.set("_tmp.ingest_timestamp", v)?;
                }
            }

            let _cond =
                { !event.has_value("json.timestamp") && event.has_value("json.detected_at") };
            if _cond {
                // Painless script
                // Source: ZonedDateTime now = (ZonedDateTime) ctx._tmp.ingest_timestamp;\nZonedDateTime max = ZonedDateTime.parse(ctx.json.detected_at);\nif (ctx.json.resolved_at != null) {\n  ZonedDateTime resolved = ZonedDateTime.parse(ctx.json.resolved_at);\n  if (resolved.isAfter(max)) { max = resolved; }\n}\nif (ctx.json.blocks_device_at != null) {\n  ZonedDateTime blocked = ZonedDateTime.parse(ctx.json.blocks_device_at);\n  if (blocked.isAfter(max)) { max = blocked; }\n}\nif (max.isAfter(now)) { max = now; }\nctx['@timestamp'] = max.format(DateTimeFormatter.ISO_INSTANT);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ZonedDateTime now = (ZonedDateTime) ctx._tmp.ingest_timestamp;\nZonedDateTime max = ZonedDateTime.parse(ctx.json.detected_at);\nif (ctx.json.resolved_at != null) {\n  ZonedDateTime resolved = ZonedDateTime.parse(ctx.json.resolved_at);\n  if (resolved.isAfter(max)) { max = resolved; }\n}\nif (ctx.json.blocks_device_at != null) {\n  ZonedDateTime blocked = ZonedDateTime.parse(ctx.json.blocks_device_at);\n  if (blocked.isAfter(max)) { max = blocked; }\n}\nif (max.isAfter(now)) { max = now; }\nctx['@timestamp'] = max.format(DateTimeFormatter.ISO_INSTANT);"#
                    ),
                )?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "event.id")?;
            }

            let _cond = { event.has_value("json.event") };
            if _cond {
                event.set("_tmp.is_webhook", json!(true))?;
            }

            let _cond = { event.has_value("json.event") };
            if _cond {
                // Begin nested pipeline: "webhook"
                if event.has_value("json.event") {
                    event.rename("json.event", "event.action")?;
                }
                if event.has_value("json.data.device.name") {
                    event.rename("json.data.device.name", "host.name")?;
                }
                if event.has_value("json.data.device.id") {
                    if let Some(val) = event.get("json.data.device.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.device.id".into(),
                                message,
                            }
                        })?;
                        event.set("host.id", converted)?;
                    }
                }
                if event.has_value("json.data.check.name") {
                    event.rename("json.data.check.name", "rule.name")?;
                }
                if event.has_value("json.data.check.id") {
                    if let Some(val) = event.get("json.data.check.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.check.id".into(),
                                message,
                            }
                        })?;
                        event.set("rule.id", converted)?;
                    }
                }
                let _cond = { event.get("json.data.title").is_some_and(|v| v.is_string()) };
                if _cond {
                    if event.has_value("json.data.title") {
                        event.rename("json.data.title", "kolide.issues.title")?;
                    }
                }
                if event.has_value("json.data.issue_id") {
                    if let Some(val) = event.get("json.data.issue_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.issue_id".into(),
                                message,
                            }
                        })?;
                        event.set("kolide.issues.id", converted)?;
                    }
                }
                if event.has_value("json.data.check_id") {
                    if let Some(val) = event.get("json.data.check_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.check_id".into(),
                                message,
                            }
                        })?;
                        event.set("kolide.issues.check.id", converted)?;
                    }
                }
                if event.has_value("json.data.check.tags") {
                    event.rename("json.data.check.tags", "kolide.issues.check.tags")?;
                }
                let _cond = {
                    event.has_value("json.timestamp")
                        && event.get_str("event.action") == Some("issues.resolved")
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("kolide.issues.resolved_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                let _cond = {
                    event.has_value("json.timestamp")
                        && event.get_str("event.action") == Some("issues.new")
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("json.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("kolide.issues.detected_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                // End nested pipeline: "webhook"
            }

            let _cond = { !event.has_value("_tmp.is_webhook") && !event.has_value("event.action") };
            if _cond {
                event.set("event.action", json!("issue"))?;
            }

            let _cond = {
                !event.has_value("_tmp.is_webhook")
                    && !event.has_value("kolide.issues.id")
                    && event.has_value("event.id")
            };
            if _cond {
                if let Some(v) = event.get("event.id").cloned() {
                    event.set("kolide.issues.id", v)?;
                }
            }

            let _cond = { event.has_value("json.detected_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.detected_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.detected_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.resolved_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.resolved_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.resolved_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { !event.has_value("host.id") };
            if _cond {
                if event.has_value("json.device_information.identifier") {
                    event.rename("json.device_information.identifier", "host.id")?;
                }
            }

            let _cond = { !event.has_value("rule.id") };
            if _cond {
                if event.has_value("json.check_information.identifier") {
                    event.rename("json.check_information.identifier", "rule.id")?;
                }
            }

            let _cond = {
                !event.has_value("kolide.issues.title")
                    && event.get("json.title").is_some_and(|v| v.is_string())
            };
            if _cond {
                if event.has_value("json.title") {
                    event.rename("json.title", "kolide.issues.title")?;
                }
            }

            let _cond = { event.has_value("kolide.issues.title") && !event.has_value("rule.name") };
            if _cond {
                event.set(
                    "rule.name",
                    json!(
                        event
                            .get("kolide.issues.title")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("rule.reference") && event.has_value("json.check_information.link")
            };
            if _cond {
                event.set(
                    "rule.reference",
                    json!(
                        event
                            .get("json.check_information.link")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                !event.has_value("rule.reference")
                    && !event.has_value("json.check_information.link")
                    && event.has_value("rule.id")
            };
            if _cond {
                event.set(
                    "rule.reference",
                    json!(format!(
                        "https://api.kolide.com/checks/{}",
                        event
                            .get("rule.id")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            if event.has_value("json.issue_key") {
                event.rename("json.issue_key", "kolide.issues.issue_key")?;
            }

            if event.has_value("json.issue_value") {
                event.rename("json.issue_value", "kolide.issues.issue_value")?;
            }

            if event.has_value("json.value") {
                event.rename("json.value", "kolide.issues.value")?;
            }

            let _cond = { event.has_value("json.exempted") };
            if _cond {
                if let Some(v) = event.get("json.exempted").cloned() {
                    event.set("kolide.issues.exempted", v)?;
                }
            }

            event.remove("json.exempted");

            if event.has_value("json.device_information.link") {
                event.rename(
                    "json.device_information.link",
                    "kolide.issues.device_information.link",
                )?;
            }

            if event.has_value("json.check_information.link") {
                event.rename(
                    "json.check_information.link",
                    "kolide.issues.check_information.link",
                )?;
            }

            let _cond = { event.has_value("json.detected_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.detected_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.issues.detected_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.detected_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.resolved_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.resolved_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.issues.resolved_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.resolved_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.blocks_device_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.blocks_device_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.issues.blocks_device_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.blocks_device_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.last_rechecked_at") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.last_rechecked_at") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("kolide.issues.last_rechecked_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.last_rechecked_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event
                    .get("kolide.issues.value.username")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.username")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            let _cond = {
                event
                    .get("kolide.issues.value.path")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.path")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.path", v)?;
                }
            }

            let _cond = {
                event
                    .get("kolide.issues.value.md5")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.md5")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.hash.md5", v)?;
                }
            }

            let _cond = {
                event
                    .get("kolide.issues.value.fingerprint_sha256")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.fingerprint_sha256")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.hash.sha256", v)?;
                }
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                if let Some(v) = event.get("host.name").cloned() {
                    event.set("host.hostname", v)?;
                }
            }

            let _cond = { event.has_value("kolide.issues.title") };
            if _cond {
                if let Some(v) = event.get("kolide.issues.title").cloned() {
                    event.set("message", v)?;
                }
            }

            let _cond = {
                event.has_value("kolide.issues.resolved_at")
                    || event.get_str("event.action") == Some("issues.resolved")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                event
                    .get("kolide.issues.check.tags")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.check.tags")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.category", v)?;
                }
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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

            // Begin nested pipeline: "extended-mappings"
            let _cond = {
                event
                    .get("kolide.issues.value.version")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("kolide.issues.detected_version", v)?;
                }
            }
            let _cond = {
                event
                    .get("kolide.issues.detected_version")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.detected_version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("package.version", v)?;
                }
            }
            let _cond = {
                event
                    .get("kolide.issues.value.newest_version")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.newest_version")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("kolide.issues.expected_version", v)?;
                }
            }
            let _cond = {
                event
                    .get("kolide.issues.value.key_type")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.key_type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("kolide.issues.ssh_key_type", v)?;
                }
            }
            let _cond = {
                event
                    .get("kolide.issues.value.device")
                    .is_some_and(|v| v.is_string())
                    && !event.has_value("file.device")
            };
            if _cond {
                if let Some(v) = event
                    .get("kolide.issues.value.device")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.device", v)?;
                }
            }
            let _cond = {
                event
                    .get("kolide.issues.value.username")
                    .is_some_and(|v| v.is_string())
                    && !event.has_value("user.name")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("kolide.issues.value.username")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }
            // End nested pipeline: "extended-mappings"

            // Begin nested pipeline: "categorize"
            let _cond = { event.has_value("event.action") };
            if _cond {
                // Painless script
                // Source: def action = ctx.event.action;\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.kind = m.kind;\n  ctx.event.category = new ArrayList(m.category);\n} else {\n  ctx.event.kind = 'event';\n  ctx.event.category = ['configuration'];\n}\n\n// Add a security-domain category (malware/vulnerability) for specific\n// checks, keyed by check id; the base 'configuration' category is kept.\nif (ctx.rule?.id != null) {\n  def domain = params.check_category.get(ctx.rule.id);\n  if (domain != null && !ctx.event.category.contains(domain)) {\n    ctx.event.category.add(domain);\n  }\n}\n\n// blocked/resolved issues are a change; a newly detected issue is a creation.\n// blocked_device_at is a future deadline, not an immediate state: the device\n// is only actually blocked once that deadline has passed relative to this event.\nboolean pendingBlock = (ctx.kolide?.issues?.blocks_device_at != null);\nboolean blocked = false;\nif (pendingBlock && ctx['@timestamp'] != null) {\n  ZonedDateTime blockAt = ZonedDateTime.parse(ctx.kolide.issues.blocks_device_at);\n  ZonedDateTime eventTime = ZonedDateTime.parse(ctx['@timestamp']);\n  blocked = !blockAt.isAfter(eventTime);\n}\nboolean resolved = (ctx.kolide?.issues?.resolved_at != null) || (action == 'issues.resolved');\nctx.event.type = (pendingBlock || resolved) ? ['change'] : ['creation'];\n\n// Persist the computed blocked state for the fingerprint processor in\n// default.yml, so pending/blocked/resolved each fingerprint to a distinct _id.\nctx._tmp = ctx._tmp == null ? [:] : ctx._tmp;\nctx._tmp.blocked = blocked;\n\n// Precedence: a resolved issue takes priority over a still-blocking one;\n// a pending future block takes priority over a plain open issue.\nif (action == 'issue') {\n  if (resolved) {\n    ctx.event.action = 'resolved';\n  } else if (blocked) {\n    ctx.event.action = 'blocked';\n  } else if (pendingBlock) {\n    ctx.event.action = 'will_be_blocked';\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def action = ctx.event.action;\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.kind = m.kind;\n  ctx.event.category = new ArrayList(m.category);\n} else {\n  ctx.event.kind = 'event';\n  ctx.event.category = ['configuration'];\n}\n\n// Add a security-domain category (malware/vulnerability) for specific\n// checks, keyed by check id; the base 'configuration' category is kept.\nif (ctx.rule?.id != null) {\n  def domain = params.check_category.get(ctx.rule.id);\n  if (domain != null && !ctx.event.category.contains(domain)) {\n    ctx.event.category.add(domain);\n  }\n}\n\n// blocked/resolved issues are a change; a newly detected issue is a creation.\n// blocked_device_at is a future deadline, not an immediate state: the device\n// is only actually blocked once that deadline has passed relative to this event.\nboolean pendingBlock = (ctx.kolide?.issues?.blocks_device_at != null);\nboolean blocked = false;\nif (pendingBlock && ctx['@timestamp'] != null) {\n  ZonedDateTime blockAt = ZonedDateTime.parse(ctx.kolide.issues.blocks_device_at);\n  ZonedDateTime eventTime = ZonedDateTime.parse(ctx['@timestamp']);\n  blocked = !blockAt.isAfter(eventTime);\n}\nboolean resolved = (ctx.kolide?.issues?.resolved_at != null) || (action == 'issues.resolved');\nctx.event.type = (pendingBlock || resolved) ? ['change'] : ['creation'];\n\n// Persist the computed blocked state for the fingerprint processor in\n// default.yml, so pending/blocked/resolved each fingerprint to a distinct _id.\nctx._tmp = ctx._tmp == null ? [:] : ctx._tmp;\nctx._tmp.blocked = blocked;\n\n// Precedence: a resolved issue takes priority over a still-blocking one;\n// a pending future block takes priority over a plain open issue.\nif (action == 'issue') {\n  if (resolved) {\n    ctx.event.action = 'resolved';\n  } else if (blocked) {\n    ctx.event.action = 'blocked';\n  } else if (pendingBlock) {\n    ctx.event.action = 'will_be_blocked';\n  }\n}"#
                    ),
                    cached_params!(
                        "{\"exact\":{\"issue\":{\"kind\":\"event\",\"category\":[\"configuration\"]},\"issues.new\":{\"kind\":\"event\",\"category\":[\"configuration\"]},\"issues.resolved\":{\"kind\":\"event\",\"category\":[\"configuration\"]}},\"check_category\":{\"17\":\"malware\",\"18\":\"malware\",\"20\":\"malware\",\"75639\":\"malware\",\"41\":\"vulnerability\"}}"
                    ),
                )?;
            }
            // End nested pipeline: "categorize"

            let _cond = { event.has_value("event.id") && event.get_str("event.id") != Some("") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("_tmp.blocked") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("event.id") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("kolide.issues.blocks_device_at") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("kolide.issues.detected_at") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("kolide.issues.resolved_at") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set(
                            "_id",
                            json!(fingerprint_with(&values, "SHA-256", "").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_id".into(),
                                    message,
                                }
                            })?),
                        )?;
                    }
                }
            }

            event.remove("json");
            event.remove("data");
            event.remove("_tmp");

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
