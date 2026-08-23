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

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("alert"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.get("json.value").is_some_and(|v| v.is_array()) && event.get("json.value").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = {
                event.get("json.value").is_some_and(|v| v.is_array()) && event.get("json.value").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0)
            };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("There is an unexpected 'value' array with elements. \nThis indicates a misconfiguration of the input.\n").to_string(),
                });
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.lastUpdateDateTime") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.incidentId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.providerAlertId") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("json.evidence") };
            if _cond {
                // Painless script
                // Source: boolean drop(Object object) {\n  if (object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx.json.evidence);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean drop(Object object) {\n  if (object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx.json.evidence);"#
                    ),
                )?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: def eventCategory = new HashSet(); def eventType = new HashSet(); for (evidence in ctx.json.evidence) {\n  String mapping = params[evidence[\"@odata.type\"]];\n  if (mapping != null) {\n    eventCategory.add(mapping);\n    if(eventCategory.contains('registry')){\n      eventType.add('access');\n    }\n    else if(eventCategory.contains('threat')){\n      eventType.add('indicator');\n    }\n    else{\n      eventType.add('info');\n    }\n  }\n} if(ctx.json?.determination != null){\n  String mapping = params[ctx.json.determination.toLowerCase()];\n  if(mapping != null) {\n    eventCategory.add(mapping);\n    if(eventCategory.contains('threat')){\n      eventType.add('indicator');\n    }\n    else{\n      eventType.add('info');\n    }\n  }\n} if (!eventType.isEmpty()) {\n  def type = new ArrayList();\n  for(def t:eventType) {\n    type.add(t);\n  }\n  Collections.sort(type);\n  ctx.event.type = type;\n} if (!eventCategory.isEmpty()) {\n  def category = new ArrayList();\n  for(def c:eventCategory) {\n    category.add(c);\n  }\n  Collections.sort(category);\n  ctx.event.category = category;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def eventCategory = new HashSet(); def eventType = new HashSet(); for (evidence in ctx.json.evidence) {\n  String mapping = params[evidence[\"@odata.type\"]];\n  if (mapping != null) {\n    eventCategory.add(mapping);\n    if(eventCategory.contains('registry')){\n      eventType.add('access');\n    }\n    else if(eventCategory.contains('threat')){\n      eventType.add('indicator');\n    }\n    else{\n      eventType.add('info');\n    }\n  }\n} if(ctx.json?.determination != null){\n  String mapping = params[ctx.json.determination.toLowerCase()];\n  if(mapping != null) {\n    eventCategory.add(mapping);\n    if(eventCategory.contains('threat')){\n      eventType.add('indicator');\n    }\n    else{\n      eventType.add('info');\n    }\n  }\n} if (!eventType.isEmpty()) {\n  def type = new ArrayList();\n  for(def t:eventType) {\n    type.add(t);\n  }\n  Collections.sort(type);\n  ctx.event.type = type;\n} if (!eventCategory.isEmpty()) {\n  def category = new ArrayList();\n  for(def c:eventCategory) {\n    category.add(c);\n  }\n  Collections.sort(category);\n  ctx.event.category = category;\n}\n"#
                    ),
                    cached_params!(
                        "{\"#microsoft.graph.security.analyzedMessageEvidence\":\"email\",\"#microsoft.graph.security.deviceEvidence\":\"host\",\"#microsoft.graph.security.fileEvidence\":\"file\",\"#microsoft.graph.security.ipEvidence\":\"network\",\"#microsoft.graph.security.mailClusterEvidence\":\"email\",\"#microsoft.graph.security.mailboxEvidence\":\"email\",\"#microsoft.graph.security.processEvidence\":\"process\",\"#microsoft.graph.security.registryKeyEvidence\":\"registry\",\"#microsoft.graph.security.registryValueEvidence\":\"registry\",\"#microsoft.graph.security.securityGroupEvidence\":\"iam\",\"#microsoft.graph.security.userEvidence\":\"iam\",\"apt\":\"threat\",\"malicioususeractivity\":\"malware\",\"malware\":\"malware\",\"phishing\":\"email\"}"
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                dot_expand(event, "json", "@odata.type")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dot_expander")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "dot_expander_@odata_type",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has("json.@odata.type") {
                event.rename("json.@odata.type", "m365_defender.alert.odata_type")?;
            }

            let _cond = {
                event.has_value("json.lastUpdateDateTime")
                    && event.get_str("json.lastUpdateDateTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.lastUpdateDateTime") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_lastUpdateDateTime",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("@timestamp")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("m365_defender.alert.last_update_datetime", v)?;
            }

            if event.has("json.description") {
                event.rename("json.description", "m365_defender.alert.description")?;
            }

            if let Some(v) = event
                .get("m365_defender.alert.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if event.has("json.tenantId") {
                event.rename("json.tenantId", "m365_defender.alert.tenant_id")?;
            }

            if let Some(v) = event
                .get("m365_defender.alert.tenant_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            let _cond = {
                event.has_value("json.createdDateTime")
                    && event.get_str("json.createdDateTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.createdDateTime") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("m365_defender.alert.created_datetime", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_createdDateTime")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("m365_defender.alert.created_datetime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            let _cond = {
                event.has_value("json.lastActivityDateTime")
                    && event.get_str("json.lastActivityDateTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.lastActivityDateTime") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("m365_defender.alert.last_activity_datetime", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_lastActivityDateTime",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("m365_defender.alert.last_activity_datetime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.end", v)?;
            }

            if event.has("json.id") {
                event.rename("json.id", "m365_defender.alert.id")?;
            }

            if let Some(v) = event
                .get("m365_defender.alert.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has("json.serviceSource") {
                event.rename("json.serviceSource", "m365_defender.alert.service_source")?;
            }

            if let Some(v) = event
                .get("m365_defender.alert.service_source")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.provider", v)?;
            }

            if event.has("json.severity") {
                event.rename("json.severity", "m365_defender.alert.severity")?;
            }

            let _cond = {
                event
                    .get("m365_defender.alert.severity")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.m365_defender.alert.severity;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\nString risk_score_value = ctx.m365_defender.alert.severity;\nif (risk_score_value.equalsIgnoreCase(\"low\") || risk_score_value.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (risk_score_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (risk_score_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (risk_score_value.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_event_severity")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
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
                event.has_value("json.firstActivityDateTime")
                    && event.get_str("json.firstActivityDateTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.firstActivityDateTime") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("m365_defender.alert.first_activity_datetime", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_firstActivityDateTime",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("m365_defender.alert.first_activity_datetime")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.start", v)?;
            }

            let _cond = { event.has_value("event.start") && event.has_value("event.end") };
            if _cond {
                // Painless script
                // Source: Instant eventstart = ZonedDateTime.parse(ctx.event?.start).toInstant();\nInstant eventend = ZonedDateTime.parse(ctx.event?.end).toInstant();\nctx.event['duration'] = ChronoUnit.NANOS.between(eventstart, eventend);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"Instant eventstart = ZonedDateTime.parse(ctx.event?.start).toInstant();\nInstant eventend = ZonedDateTime.parse(ctx.event?.end).toInstant();\nctx.event['duration'] = ChronoUnit.NANOS.between(eventstart, eventend);\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("json.alertWebUrl") };
            if _cond {
                uri_parts(
                    event,
                    "json.alertWebUrl",
                    "m365_defender.alert.web_url",
                    true,
                    false,
                )?;
            }

            if let Some(v) = event
                .get("m365_defender.alert.web_url.original")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.url", v)?;
            }

            if event.has("json.assignedTo") {
                event.rename("json.assignedTo", "m365_defender.alert.assigned_to")?;
            }

            let _cond = { event.has_value("m365_defender.alert.assigned_to") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("m365_defender.alert.assigned_to")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("m365_defender.alert.assigned_to")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.user.name", v)?;
            }

            if event.has("json.threatFamilyName") {
                event.rename(
                    "json.threatFamilyName",
                    "m365_defender.alert.threat_family_name",
                )?;
            }

            if event.has("json.category") {
                event.rename("json.category", "m365_defender.alert.category")?;
            }

            let _cond = { event.has_value("m365_defender.alert.category") };
            if _cond {
                event.append_unique(
                    "threat.tactic.name",
                    json!(
                        event
                            .get("m365_defender.alert.category")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if event.has("json.mitreTechniques") {
                event.rename(
                    "json.mitreTechniques",
                    "m365_defender.alert.mitre_techniques",
                )?;
            }

            if let Some(v) = event
                .get("m365_defender.alert.mitre_techniques")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.technique.subtechnique.id", v)?;
            }

            if event.has("json.actorDisplayName") {
                event.rename(
                    "json.actorDisplayName",
                    "m365_defender.alert.actor_display_name",
                )?;
            }

            if event.has("json.classification") {
                event.rename("json.classification", "m365_defender.alert.classification")?;
            }

            let _cond = { event.get("json.comments").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.comments", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.createdByDisplayName")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has("json.comments") {
                event.rename("json.comments", "m365_defender.alert.comments")?;
            }

            if event.has("json.detectorId") {
                event.rename("json.detectorId", "m365_defender.alert.detector_id")?;
            }

            if event.has("json.detectionSource") {
                event.rename(
                    "json.detectionSource",
                    "m365_defender.alert.detection_source",
                )?;
            }

            if event.has("json.determination") {
                event.rename("json.determination", "m365_defender.alert.determination")?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("_ingest._value.createdDateTime")
                        {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["ISO8601"], None, None)
                            {
                                event.set("_ingest._value.created_datetime", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_foreach_evidence_createdDateTime",
                        )?;
                        event.remove("_ingest._value.createdDateTime");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.remediationStatus") {
                        event.rename(
                            "_ingest._value.remediationStatus",
                            "_ingest._value.remediation_status",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has_value("_ingest._value.ipInterfaces") {
                        foreach_array(event, "_ingest._value.ipInterfaces", |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value") {
                                    if let Some(val) = event.get("_ingest._value") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "foreach_convert_ipInterfaces",
                                )?;
                                event.remove("_ingest._value");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.ipInterfaces") {
                        event.rename(
                            "_ingest._value.ipInterfaces",
                            "_ingest._value.ip_interfaces",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has_value("_ingest._value.ip_interfaces") {
                        foreach_array(event, "_ingest._value.ip_interfaces", |event| {
                            event.append_unique(
                                "host.ip",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has_value("_ingest._value.ip_interfaces") {
                        foreach_array(event, "_ingest._value.ip_interfaces", |event| {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.detailedRoles") {
                        event.rename(
                            "_ingest._value.detailedRoles",
                            "_ingest._value.detailed_roles",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.remediationStatusDetails") {
                        event.rename(
                            "_ingest._value.remediationStatusDetails",
                            "_ingest._value.remediation_status_details",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has("json.incidentId") {
                event.rename("json.incidentId", "m365_defender.alert.incident_id")?;
            }

            let _cond = { event.has_value("json.incidentWebUrl") };
            if _cond {
                uri_parts(
                    event,
                    "json.incidentWebUrl",
                    "m365_defender.alert.incident_web_url",
                    true,
                    false,
                )?;
            }

            if event.has("json.providerAlertId") {
                event.rename(
                    "json.providerAlertId",
                    "m365_defender.alert.provider_alert_id",
                )?;
            }

            if event.has("json.recommendedActions") {
                event.rename(
                    "json.recommendedActions",
                    "m365_defender.alert.recommended_actions",
                )?;
            }

            let _cond = {
                event.has_value("json.resolvedDateTime")
                    && event.get_str("json.resolvedDateTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.resolvedDateTime") {
                        if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                            event.set("m365_defender.alert.resolved_datetime", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_resolvedDateTime")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has("json.status") {
                event.rename("json.status", "m365_defender.alert.status")?;
            }

            if event.has("json.threatDisplayName") {
                event.rename(
                    "json.threatDisplayName",
                    "m365_defender.alert.threat_display_name",
                )?;
            }

            if event.has("json.title") {
                event.rename("json.title", "m365_defender.alert.title")?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        dot_expand(event, "_ingest._value", "@odata.type")?;
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.@odata.type") {
                        event.rename("_ingest._value.@odata.type", "_ingest._value.odata_type")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("_ingest._value.receivedDateTime")
                        {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["ISO8601"], None, None)
                            {
                                event.set("_ingest._value.received_datetime", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_foreach_evidence_receivedDateTime",
                        )?;
                        event.remove("_ingest._value.receivedDateTime");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "email.delivery_timestamp",
                        json!(
                            event
                                .get("_ingest._value.received_datetime")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.antiSpamDirection") {
                        event.rename(
                            "_ingest._value.antiSpamDirection",
                            "_ingest._value.antispam_direction",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "email.direction",
                        json!(
                            event
                                .get("_ingest._value.antispam_direction")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "email.subject",
                        json!(
                            event
                                .get("_ingest._value.subject")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.recipientEmailAddress") {
                        event.rename(
                            "_ingest._value.recipientEmailAddress",
                            "_ingest._value.recipient_email_address",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.recipient_email_address")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("_ingest._value.recipient_email_address")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.attachmentsCount") {
                                if let Some(val) = event.get("_ingest._value.attachmentsCount") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.attachmentsCount".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.attachments_count", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_attachmentsCount",
                            )?;
                            if event.remove("_ingest._value.attachmentsCount").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.attachmentsCount".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.deliveryAction") {
                        event.rename(
                            "_ingest._value.deliveryAction",
                            "_ingest._value.delivery_action",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.deliveryLocation") {
                        event.rename(
                            "_ingest._value.deliveryLocation",
                            "_ingest._value.delivery_location",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "email.message_id",
                        json!(
                            event
                                .get("_ingest._value.internetMessageId")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "email.local_id",
                        json!(
                            event
                                .get("_ingest._value.networkMessageId")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        dot_expand(event, "_ingest._value.p1Sender", "@odata.type")?;
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.p1Sender.@odata.type") {
                        event.rename(
                            "_ingest._value.p1Sender.@odata.type",
                            "_ingest._value.p1_sender.odata_type",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        dot_expand(event, "_ingest._value.p2Sender", "@odata.type")?;
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.p2Sender.@odata.type") {
                        event.rename(
                            "_ingest._value.p2Sender.@odata.type",
                            "_ingest._value.p2_sender.odata_type",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.p1Sender.displayName") {
                        event.rename(
                            "_ingest._value.p1Sender.displayName",
                            "_ingest._value.p1_sender.display_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.p1_sender.display_name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.p1Sender.domainName") {
                        event.rename(
                            "_ingest._value.p1Sender.domainName",
                            "_ingest._value.p1_sender.domain_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.p1_sender.domain_name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.p1Sender.emailAddress") {
                        event.rename(
                            "_ingest._value.p1Sender.emailAddress",
                            "_ingest._value.p1_sender.email_address",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "email.from.address",
                        json!(
                            event
                                .get("_ingest._value.p1_sender.email_address")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "email.sender.address",
                        json!(
                            event
                                .get("_ingest._value.p1_sender.email_address")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.p1_sender.email_address")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.p2Sender.displayName") {
                        event.rename(
                            "_ingest._value.p2Sender.displayName",
                            "_ingest._value.p2_sender.display_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.p2_sender.display_name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.p2Sender.domainName") {
                        event.rename(
                            "_ingest._value.p2Sender.domainName",
                            "_ingest._value.p2_sender.domain_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.p2_sender.domain_name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.p2Sender.emailAddress") {
                        event.rename(
                            "_ingest._value.p2Sender.emailAddress",
                            "_ingest._value.p2_sender.email_address",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "email.from.address",
                        json!(
                            event
                                .get("_ingest._value.p2_sender.email_address")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.p2_sender.email_address")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.senderIp") {
                                if let Some(val) = event.get("_ingest._value.senderIp") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.senderIp".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.sender_ip", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_senderIp",
                            )?;
                            if event.remove("_ingest._value.senderIp").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.senderIp".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "source.ip",
                        json!(
                            event
                                .get("_ingest._value.sender_ip")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value.sender_ip")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.threatDetectionMethods") {
                        event.rename(
                            "_ingest._value.threatDetectionMethods",
                            "_ingest._value.threat_detection_methods",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.urlCount") {
                                if let Some(val) = event.get("_ingest._value.urlCount") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.urlCount".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.url_count", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_urlCount",
                            )?;
                            if event.remove("_ingest._value.urlCount").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.urlCount".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.instanceId") {
                                if let Some(val) = event.get("_ingest._value.instanceId") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.instanceId".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.instance_id", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_instanceId",
                            )?;
                            if event.remove("_ingest._value.instanceId").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.instanceId".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "cloud.instance.id",
                        json!(
                            event
                                .get("_ingest._value.instance_id")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.instanceName") {
                        event.rename(
                            "_ingest._value.instanceName",
                            "_ingest._value.instance_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "cloud.instance.name",
                        json!(
                            event
                                .get("_ingest._value.instance_name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.appId") {
                                if let Some(val) = event.get("_ingest._value.appId") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.appId".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.app_id", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_appID",
                            )?;
                            if event.remove("_ingest._value.appId").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.appId".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.displayName") {
                        event
                            .rename("_ingest._value.displayName", "_ingest._value.display_name")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.saasAppId") {
                                if let Some(val) = event.get("_ingest._value.saasAppId") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.saasAppId".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.saas_app_id", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_saasAppId",
                            )?;
                            if event.remove("_ingest._value.saasAppId").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.saasAppId".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.vmMetadata.cloudProvider") {
                        event.rename(
                            "_ingest._value.vmMetadata.cloudProvider",
                            "_ingest._value.vm_metadata.cloud_provider",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.mdeDeviceId") {
                        event
                            .rename("_ingest._value.mdeDeviceId", "_ingest._value.mde_device_id")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.mde_device_id")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.osPlatform") {
                        event.rename("_ingest._value.osPlatform", "_ingest._value.os_platform")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "host.os.name",
                        json!(
                            event
                                .get("_ingest._value.os_platform")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.os_platform")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "host.os.version",
                        json!(
                            event
                                .get("_ingest._value.version")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.version")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "device.id",
                        json!(
                            event
                                .get("_ingest._value.azureAdDeviceId")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.azureAdDeviceId") {
                        event.rename(
                            "_ingest._value.azureAdDeviceId",
                            "_ingest._value.azure_ad_device_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.defenderAvStatus") {
                        event.rename(
                            "_ingest._value.defenderAvStatus",
                            "_ingest._value.defender_av_status",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.deviceDnsName") {
                        event.rename(
                            "_ingest._value.deviceDnsName",
                            "_ingest._value.device_dns_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.device_dns_name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "host.hostname",
                        json!(
                            event
                                .get("_ingest._value.device_dns_name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("_ingest._value.firstSeenDateTime")
                        {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["ISO8601"], None, None)
                            {
                                event.set("_ingest._value.first_seen_datetime", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_foreach_firstSeenDateTime",
                        )?;
                        event.remove("_ingest._value.firstSeenDateTime");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.healthStatus") {
                        event.rename(
                            "_ingest._value.healthStatus",
                            "_ingest._value.health_status",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has_value("_ingest._value.loggedOnUsers") {
                        foreach_array(event, "_ingest._value.loggedOnUsers", |event| {
                            if event.has("_ingest._value.accountName") {
                                event.rename(
                                    "_ingest._value.accountName",
                                    "_ingest._value.account_name",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has_value("_ingest._value.loggedOnUsers") {
                        foreach_array(event, "_ingest._value.loggedOnUsers", |event| {
                            event.append_unique(
                                "related.user",
                                json!(
                                    event
                                        .get("_ingest._value.account_name")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has_value("_ingest._value.loggedOnUsers") {
                        foreach_array(event, "_ingest._value.loggedOnUsers", |event| {
                            if event.has("_ingest._value.domainName") {
                                event.rename(
                                    "_ingest._value.domainName",
                                    "_ingest._value.domain_name",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has_value("_ingest._value.loggedOnUsers") {
                        foreach_array(event, "_ingest._value.loggedOnUsers", |event| {
                            event.append_unique(
                                "related.hosts",
                                json!(
                                    event
                                        .get("_ingest._value.domain_name")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has_value("_ingest._value.loggedOnUsers") {
                        foreach_array(event, "_ingest._value.loggedOnUsers", |event| {
                            // ignore_failure: true
                            let _ = (|| -> Result<()> {
                                dot_expand(event, "_ingest._value", "@odata.type")?;
                                Ok(())
                            })();
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has_value("_ingest._value.loggedOnUsers") {
                        foreach_array(event, "_ingest._value.loggedOnUsers", |event| {
                            if event.has("_ingest._value.@odata.type") {
                                event.rename(
                                    "_ingest._value.@odata.type",
                                    "_ingest._value.odata_type",
                                )?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.loggedOnUsers") {
                        event.rename(
                            "_ingest._value.loggedOnUsers",
                            "_ingest._value.logged_on_users",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.onboardingStatus") {
                        event.rename(
                            "_ingest._value.onboardingStatus",
                            "_ingest._value.onboarding_status",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.osBuild") {
                                if let Some(val) = event.get("_ingest._value.osBuild") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.osBuild".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.os_build", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_osBuild",
                            )?;
                            if event.remove("_ingest._value.osBuild").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.osBuild".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.rbacGroupId") {
                                if let Some(val) = event.get("_ingest._value.rbacGroupId") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.rbacGroupId".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.rbac_group.id", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_rbacGroupId",
                            )?;
                            if event.remove("_ingest._value.rbacGroupId").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.rbacGroupId".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.rbacGroupName") {
                        event.rename(
                            "_ingest._value.rbacGroupName",
                            "_ingest._value.rbac_group.name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.riskScore") {
                        event.rename("_ingest._value.riskScore", "_ingest._value.risk_score")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        dot_expand(event, "_ingest._value.vmMetadata", "@odata.type")?;
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.vmMetadata.@odata.type") {
                        event.rename(
                            "_ingest._value.vmMetadata.@odata.type",
                            "_ingest._value.vm_metadata.odata_type",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.vmMetadata.resourceId") {
                        event.rename(
                            "_ingest._value.vmMetadata.resourceId",
                            "_ingest._value.vm_metadata.resource_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.vmMetadata.subscriptionId") {
                        event.rename(
                            "_ingest._value.vmMetadata.subscriptionId",
                            "_ingest._value.vm_metadata.subscription_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.vmMetadata.vmId") {
                        event.rename(
                            "_ingest._value.vmMetadata.vmId",
                            "_ingest._value.vm_metadata.vm_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.detectionStatus") {
                        event.rename(
                            "_ingest._value.detectionStatus",
                            "_ingest._value.detection_status",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "event.action",
                        json!(
                            event
                                .get("_ingest._value.detection_status")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        dot_expand(event, "_ingest._value.fileDetails", "@odata.type")?;
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.fileDetails.@odata.type") {
                        event.rename(
                            "_ingest._value.fileDetails.@odata.type",
                            "_ingest._value.file_details.odata_type",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.fileDetails.sha1") {
                        event.rename(
                            "_ingest._value.fileDetails.sha1",
                            "_ingest._value.file_details.sha1",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "file.hash.sha1",
                        json!(
                            event
                                .get("_ingest._value.file_details.sha1")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.file_details.sha1")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.fileDetails.sha256") {
                        event.rename(
                            "_ingest._value.fileDetails.sha256",
                            "_ingest._value.file_details.sha256",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "file.hash.sha256",
                        json!(
                            event
                                .get("_ingest._value.file_details.sha256")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.file_details.sha256")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.fileDetails.fileName") {
                        event.rename(
                            "_ingest._value.fileDetails.fileName",
                            "_ingest._value.file_details.name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "file.name",
                        json!(
                            event
                                .get("_ingest._value.file_details.name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.fileDetails.filePath") {
                        event.rename(
                            "_ingest._value.fileDetails.filePath",
                            "_ingest._value.file_details.path",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "file.path",
                        json!(
                            event
                                .get("_ingest._value.file_details.path")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.fileDetails.fileSize") {
                                if let Some(val) = event.get("_ingest._value.fileDetails.fileSize")
                                {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.fileDetails.fileSize".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.file_details.size", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_fileSize",
                            )?;
                            if event
                                .remove("_ingest._value.fileDetails.fileSize")
                                .is_none()
                            {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.fileDetails.fileSize".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.fileDetails.filePublisher") {
                        event.rename(
                            "_ingest._value.fileDetails.filePublisher",
                            "_ingest._value.file_details.publisher",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.fileDetails.issuer") {
                        event.rename(
                            "_ingest._value.fileDetails.issuer",
                            "_ingest._value.file_details.issuer",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.fileDetails.signer") {
                        event.rename(
                            "_ingest._value.fileDetails.signer",
                            "_ingest._value.file_details.signer",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.ipAddress") {
                                if let Some(val) = event.get("_ingest._value.ipAddress") {
                                    let converted =
                                        convert_value(val, "ip").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.ipAddress".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.ip_address", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_ipAddress",
                            )?;
                            if event.remove("_ingest._value.ipAddress").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.ipAddress".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "host.ip",
                        json!(
                            event
                                .get("_ingest._value.ip_address")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value.ip_address")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "source.geo.country_iso_code",
                        json!(
                            event
                                .get("_ingest._value.country_letter_code")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "url.original",
                        json!(
                            event
                                .get("_ingest._value.url")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has_value("_ingest._value.urls") {
                        foreach_array(event, "_ingest._value.urls", |event| {
                            event.append_unique(
                                "url.original",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        dot_expand(event, "_ingest._value.userAccount", "@odata.type")?;
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.userAccount.@odata.type") {
                        event.rename(
                            "_ingest._value.userAccount.@odata.type",
                            "_ingest._value.user_account.odata_type",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.userAccount.domainName") {
                        event.rename(
                            "_ingest._value.userAccount.domainName",
                            "_ingest._value.user_account.domain_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_ingest._value.user_account.domain_name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.userAccount.azureAdUserId") {
                        event.rename(
                            "_ingest._value.userAccount.azureAdUserId",
                            "_ingest._value.user_account.azure_ad_user_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.user_account.azure_ad_user_id")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.userAccount.userPrincipalName") {
                        event.rename(
                            "_ingest._value.userAccount.userPrincipalName",
                            "_ingest._value.user_account.user_principal_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.user_account.user_principal_name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.primaryAddress") {
                        event.rename(
                            "_ingest._value.primaryAddress",
                            "_ingest._value.primary_address",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "user.email",
                        json!(
                            event
                                .get("_ingest._value.primary_address")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.primary_address")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.userAccount.accountName") {
                        event.rename(
                            "_ingest._value.userAccount.accountName",
                            "_ingest._value.user_account.account_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.user_account.account_name")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.userAccount.userSid") {
                        event.rename(
                            "_ingest._value.userAccount.userSid",
                            "_ingest._value.user_account.user_sid",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_ingest._value.user_account.user_sid")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.clusterBy") {
                        event.rename("_ingest._value.clusterBy", "_ingest._value.cluster_by")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.clusterByValue") {
                        event.rename(
                            "_ingest._value.clusterByValue",
                            "_ingest._value.cluster_by_value",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.emailCount") {
                                if let Some(val) = event.get("_ingest._value.emailCount") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.emailCount".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.email_count", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_emailCount",
                            )?;
                            if event.remove("_ingest._value.emailCount").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.emailCount".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.networkMessageIds") {
                        event.rename(
                            "_ingest._value.networkMessageIds",
                            "_ingest._value.network_message_ids",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.objectId") {
                        event.rename("_ingest._value.objectId", "_ingest._value.object_id")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.processCommandLine") {
                        event.rename(
                            "_ingest._value.processCommandLine",
                            "_ingest._value.process.command_line",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "process.command_line",
                        json!(
                            event
                                .get("_ingest._value.process.command_line")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        dot_expand(event, "_ingest._value.imageFile", "@odata.type")?;
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.imageFile.@odata.type") {
                        event.rename(
                            "_ingest._value.imageFile.@odata.type",
                            "_ingest._value.image_file.odata_type",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.imageFile.sha1") {
                        event.rename(
                            "_ingest._value.imageFile.sha1",
                            "_ingest._value.image_file.sha1",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "process.hash.sha1",
                        json!(
                            event
                                .get("_ingest._value.image_file.sha1")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.image_file.sha1")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.imageFile.sha256") {
                        event.rename(
                            "_ingest._value.imageFile.sha256",
                            "_ingest._value.image_file.sha256",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "process.hash.sha256",
                        json!(
                            event
                                .get("_ingest._value.image_file.sha256")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.image_file.sha256")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        dot_expand(
                            event,
                            "_ingest._value.parentProcessImageFile",
                            "@odata.type",
                        )?;
                        Ok(())
                    })();
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.parentProcessImageFile.@odata.type") {
                        event.rename(
                            "_ingest._value.parentProcessImageFile.@odata.type",
                            "_ingest._value.parent_process.image_file.odata_type",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.parentProcessImageFile.sha1") {
                        event.rename(
                            "_ingest._value.parentProcessImageFile.sha1",
                            "_ingest._value.parent_process.image_file.sha1",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "process.parent.hash.sha1",
                        json!(
                            event
                                .get("_ingest._value.parent_process.image_file.sha1")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.parent_process.image_file.sha1")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.parentProcessImageFile.sha256") {
                        event.rename(
                            "_ingest._value.parentProcessImageFile.sha256",
                            "_ingest._value.parent_process.image_file.sha256",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "process.parent.hash.sha256",
                        json!(
                            event
                                .get("_ingest._value.parent_process.image_file.sha256")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("_ingest._value.parent_process.image_file.sha256")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.parentProcessId") {
                                if let Some(val) = event.get("_ingest._value.parentProcessId") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.parentProcessId".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.parent_process.id", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_parentProcessId",
                            )?;
                            if event.remove("_ingest._value.parentProcessId").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.parentProcessId".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("_ingest._value.parentProcessCreationDateTime")
                        {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["ISO8601"], None, None)
                            {
                                event.set(
                                    "_ingest._value.parent_process.creation_datetime",
                                    parsed,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_foreach_evidence_parentProcessCreationDateTime",
                        )?;
                        event.remove("_ingest._value.parentProcessCreationDateTime");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "process.parent.start",
                        json!(
                            event
                                .get("_ingest._value.parent_process.creation_datetime")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.processId") {
                                if let Some(val) = event.get("_ingest._value.processId") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.processId".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.process.id", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_processId",
                            )?;
                            if event.remove("_ingest._value.processId").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.processId".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("_ingest._value.processCreationDateTime")
                        {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["ISO8601"], None, None)
                            {
                                event.set("_ingest._value.process.creation_datetime", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_foreach_evidence_processCreationDateTime",
                        )?;
                        event.remove("_ingest._value.processCreationDateTime");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "process.start",
                        json!(
                            event
                                .get("_ingest._value.process.creation_datetime")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "process.name",
                        json!(
                            event
                                .get("_ingest._value.imageFile.fileName")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.imageFile.fileName") {
                        event.rename(
                            "_ingest._value.imageFile.fileName",
                            "_ingest._value.image_file.name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.imageFile.filePath") {
                        event.rename(
                            "_ingest._value.imageFile.filePath",
                            "_ingest._value.image_file.path",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.imageFile.filePublisher") {
                        event.rename(
                            "_ingest._value.imageFile.filePublisher",
                            "_ingest._value.image_file.publisher",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.imageFile.fileSize") {
                                if let Some(val) = event.get("_ingest._value.imageFile.fileSize") {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.imageFile.fileSize".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.image_file.size", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_imagefile_filesize",
                            )?;
                            if event.remove("_ingest._value.imageFile.fileSize").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.imageFile.fileSize".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.imageFile.issuer") {
                        event.rename(
                            "_ingest._value.imageFile.issuer",
                            "_ingest._value.image_file.issuer",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.imageFile.signer") {
                        event.rename(
                            "_ingest._value.imageFile.signer",
                            "_ingest._value.image_file.signer",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.parentProcessImageFile.fileName") {
                        event.rename(
                            "_ingest._value.parentProcessImageFile.fileName",
                            "_ingest._value.parent_process.image_file.name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.parentProcessImageFile.filePath") {
                        event.rename(
                            "_ingest._value.parentProcessImageFile.filePath",
                            "_ingest._value.parent_process.image_file.path",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.parentProcessImageFile.filePublisher") {
                        event.rename(
                            "_ingest._value.parentProcessImageFile.filePublisher",
                            "_ingest._value.parent_process.image_file.publisher",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                if let Some(Value::Array(items)) = event.get("json.evidence").cloned() {
                    // A NESTED loop borrows the same `_ingest._value` slot, so
                    // the enclosing element is saved and put back afterwards.
                    let enclosing = event.get("_ingest._value").cloned();
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.parentProcessImageFile.fileSize") {
                                if let Some(val) =
                                    event.get("_ingest._value.parentProcessImageFile.fileSize")
                                {
                                    let converted =
                                        convert_value(val, "long").map_err(|message| {
                                            TransformError::ParseError {
                                                path:
                                                    "_ingest._value.parentProcessImageFile.fileSize"
                                                        .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.parent_process.image_file.size",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_foreach_evidence_parentProcessImageFile_filesize",
                            )?;
                            if event
                                .remove("_ingest._value.parentProcessImageFile.fileSize")
                                .is_none()
                            {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.parentProcessImageFile.fileSize".into(),
                                });
                            }
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    match enclosing {
                        Some(previous) => {
                            event.set("_ingest._value", previous)?;
                        }
                        None => {
                            event.remove("_ingest");
                        }
                    }
                    event.set("json.evidence", Value::Array(out))?;
                }
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.parentProcessImageFile.issuer") {
                        event.rename(
                            "_ingest._value.parentProcessImageFile.issuer",
                            "_ingest._value.parent_process.image_file.issuer",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.parentProcessImageFile.signer") {
                        event.rename(
                            "_ingest._value.parentProcessImageFile.signer",
                            "_ingest._value.parent_process.image_file.signer",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: def convertToOrderedArray(def list) {\n  def result = new ArrayList();\n  for (element in list) {\n    result.add(element);\n  }\n  Collections.sort(result);\n  return result;\n}\n\nString joinPathAndName(String path, String name) {\n  if (path == null || name == null) {\n    return null;\n  }\n  String separator = path.contains(\"\\\\\") ? \"\\\\\" : \"/\";\n  if (!path.endsWith(separator)) {\n    path = path + separator;\n  }\n  return path + name;\n}\n\nvoid maybeAddExecutable(def destination, def imageFile) {\n  if (imageFile == null) {\n    return;\n  }\n  if (imageFile?.path != null && imageFile?.name != null) {\n    destination.add(joinPathAndName(imageFile.path, imageFile.name));\n  } else if (imageFile?.name != null) {\n    destination.add(imageFile.name);\n  }\n}\n\ndef processExecutable = new HashSet();\ndef processParentExecutable = new HashSet();\ndef fileSize = new HashSet();\ndef processPid = new HashSet();\ndef processParentPid = new HashSet();\ndef processEntityId = new HashSet();\ndef processParentEntityId = new HashSet();\n\nfor (evidence in ctx.json.evidence) {\n  maybeAddExecutable(processExecutable, evidence?.image_file);\n  maybeAddExecutable(processParentExecutable, evidence?.parent_process?.image_file);\n\n  if (evidence?.odata_type == null) {\n    continue;\n  }\n  if (evidence.odata_type == '#microsoft.graph.security.fileEvidence') {\n    if (evidence?.file_details?.size != null) {\n      fileSize.add(evidence.file_details.size);\n    }\n  } else if (evidence.odata_type == '#microsoft.graph.security.processEvidence') {\n    if (evidence?.process?.id != null) {\n      processPid.add(evidence.process.id);\n    }\n    if (evidence?.parent_process?.id != null) {\n      processParentPid.add(evidence.parent_process.id);\n    }\n    if (evidence?.process?.id != null && evidence?.process?.creation_datetime != null && evidence?.mde_device_id != null) {\n      processEntityId.add(evidence.process.id + '|' + evidence.process.creation_datetime + '|' + evidence.mde_device_id);\n    }\n    if (evidence?.parent_process?.id != null && evidence?.parent_process?.creation_datetime != null && evidence?.mde_device_id != null) {\n      processParentEntityId.add(evidence.parent_process.id + '|' + evidence.parent_process.creation_datetime + '|' + evidence.mde_device_id);\n    }\n  }\n}\n\nctx.file = ctx.file ?: [:];\nctx.process = ctx.process ?: [:];\nctx.process.parent = ctx.process.parent ?: [:];\n\nif (!fileSize.isEmpty()) {\n  ctx.file.size = convertToOrderedArray(fileSize);\n}\nif (!processPid.isEmpty()) {\n  ctx.process.pid = convertToOrderedArray(processPid);\n}\nif (!processParentPid.isEmpty()) {\n  ctx.process.parent.pid = convertToOrderedArray(processParentPid);\n}\nif (!processEntityId.isEmpty()) {\n  ctx.process.entity_id = convertToOrderedArray(processEntityId);\n}\nif (!processParentEntityId.isEmpty()) {\n  ctx.process.parent.entity_id = convertToOrderedArray(processParentEntityId);\n}\nif (!processExecutable.isEmpty()) {\n  def execList = new ArrayList(processExecutable);\n  if (execList.size() == 1) {\n    ctx.process.executable = execList.get(0);\n  } else {\n    ctx.process.executable = execList;\n  }\n}\nif (!processParentExecutable.isEmpty()) {\n  def execList = new ArrayList(processParentExecutable);\n  if (execList.size() == 1) {\n    ctx.process.parent.executable = execList.get(0);\n  } else {\n    ctx.process.parent.executable = execList;\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def convertToOrderedArray(def list) {\n  def result = new ArrayList();\n  for (element in list) {\n    result.add(element);\n  }\n  Collections.sort(result);\n  return result;\n}\n\nString joinPathAndName(String path, String name) {\n  if (path == null || name == null) {\n    return null;\n  }\n  String separator = path.contains(\"\\\\\") ? \"\\\\\" : \"/\";\n  if (!path.endsWith(separator)) {\n    path = path + separator;\n  }\n  return path + name;\n}\n\nvoid maybeAddExecutable(def destination, def imageFile) {\n  if (imageFile == null) {\n    return;\n  }\n  if (imageFile?.path != null && imageFile?.name != null) {\n    destination.add(joinPathAndName(imageFile.path, imageFile.name));\n  } else if (imageFile?.name != null) {\n    destination.add(imageFile.name);\n  }\n}\n\ndef processExecutable = new HashSet();\ndef processParentExecutable = new HashSet();\ndef fileSize = new HashSet();\ndef processPid = new HashSet();\ndef processParentPid = new HashSet();\ndef processEntityId = new HashSet();\ndef processParentEntityId = new HashSet();\n\nfor (evidence in ctx.json.evidence) {\n  maybeAddExecutable(processExecutable, evidence?.image_file);\n  maybeAddExecutable(processParentExecutable, evidence?.parent_process?.image_file);\n\n  if (evidence?.odata_type == null) {\n    continue;\n  }\n  if (evidence.odata_type == '#microsoft.graph.security.fileEvidence') {\n    if (evidence?.file_details?.size != null) {\n      fileSize.add(evidence.file_details.size);\n    }\n  } else if (evidence.odata_type == '#microsoft.graph.security.processEvidence') {\n    if (evidence?.process?.id != null) {\n      processPid.add(evidence.process.id);\n    }\n    if (evidence?.parent_process?.id != null) {\n      processParentPid.add(evidence.parent_process.id);\n    }\n    if (evidence?.process?.id != null && evidence?.process?.creation_datetime != null && evidence?.mde_device_id != null) {\n      processEntityId.add(evidence.process.id + '|' + evidence.process.creation_datetime + '|' + evidence.mde_device_id);\n    }\n    if (evidence?.parent_process?.id != null && evidence?.parent_process?.creation_datetime != null && evidence?.mde_device_id != null) {\n      processParentEntityId.add(evidence.parent_process.id + '|' + evidence.parent_process.creation_datetime + '|' + evidence.mde_device_id);\n    }\n  }\n}\n\nctx.file = ctx.file ?: [:];\nctx.process = ctx.process ?: [:];\nctx.process.parent = ctx.process.parent ?: [:];\n\nif (!fileSize.isEmpty()) {\n  ctx.file.size = convertToOrderedArray(fileSize);\n}\nif (!processPid.isEmpty()) {\n  ctx.process.pid = convertToOrderedArray(processPid);\n}\nif (!processParentPid.isEmpty()) {\n  ctx.process.parent.pid = convertToOrderedArray(processParentPid);\n}\nif (!processEntityId.isEmpty()) {\n  ctx.process.entity_id = convertToOrderedArray(processEntityId);\n}\nif (!processParentEntityId.isEmpty()) {\n  ctx.process.parent.entity_id = convertToOrderedArray(processParentEntityId);\n}\nif (!processExecutable.isEmpty()) {\n  def execList = new ArrayList(processExecutable);\n  if (execList.size() == 1) {\n    ctx.process.executable = execList.get(0);\n  } else {\n    ctx.process.executable = execList;\n  }\n}\nif (!processParentExecutable.isEmpty()) {\n  def execList = new ArrayList(processParentExecutable);\n  if (execList.size() == 1) {\n    ctx.process.parent.executable = execList.get(0);\n  } else {\n    ctx.process.parent.executable = execList;\n  }\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.registryHive") {
                        event.rename(
                            "_ingest._value.registryHive",
                            "_ingest._value.registry_hive",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "registry.hive",
                        json!(
                            event
                                .get("_ingest._value.registry_hive")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.registryKey") {
                        event
                            .rename("_ingest._value.registryKey", "_ingest._value.registry_key")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "registry.key",
                        json!(
                            event
                                .get("_ingest._value.registry_key")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.registryValueType") {
                        event.rename(
                            "_ingest._value.registryValueType",
                            "_ingest._value.registry_value_type",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "registry.data.type",
                        json!(
                            event
                                .get("_ingest._value.registry_value_type")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.registryValue") {
                        event.rename(
                            "_ingest._value.registryValue",
                            "_ingest._value.registry_value",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "registry.value",
                        json!(
                            event
                                .get("_ingest._value.registry_value")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.registryValueName") {
                        event.rename(
                            "_ingest._value.registryValueName",
                            "_ingest._value.registry_value_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    if event.has("_ingest._value.securityGroupId") {
                        event.rename(
                            "_ingest._value.securityGroupId",
                            "_ingest._value.security_group_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.append_unique(
                        "group.id",
                        json!(
                            event
                                .get("_ingest._value.security_group_id")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: def convertToOrderedArray(def list) {\n  def result = new ArrayList();\n  for (element in list) {\n    result.add(element);\n  }\n  Collections.sort(result);\n  return result;\n}\nif (!(ctx.cloud instanceof HashMap)) {\n  ctx.cloud = new HashMap();\n}\ndef cloudProvider = new HashSet();\nif (!(ctx.group instanceof HashMap)) {\n  ctx.group = new HashMap();\n}\ndef groupName = new HashSet();\nif (!(ctx.host instanceof HashMap)) {\n  ctx.host = new HashMap();\n}\ndef hostId = new HashSet();\nif (!(ctx.user instanceof HashMap)) {\n  ctx.user = new HashMap();\n}\ndef userDomain = new HashSet();\ndef userId = new HashSet();\ndef userName = new HashSet();\ndef userEmail = new HashSet();\nif (!(ctx.process instanceof HashMap)) {\n  ctx.process = new HashMap();\n}\nctx.process.user = new HashMap();\ndef processUserId = new HashSet();\ndef processUserName = new HashSet();\nfor (evidence in ctx.json.evidence) {\n  if (evidence?.odata_type == null) {\n    continue\n  }\n  if (evidence?.odata_type == '#microsoft.graph.security.securityGroupEvidence') {\n    if (evidence?.display_name != null) {\n      groupName.add(evidence.display_name);\n    }\n  }\n  if (evidence?.odata_type == '#microsoft.graph.security.deviceEvidence') {\n    if (evidence?.mde_device_id != null) {\n      hostId.add(evidence.mde_device_id);\n    }\n  }\n  if (['#microsoft.graph.security.mailboxEvidence', '#microsoft.graph.security.userEvidence'].contains(evidence.odata_type)) {\n    if (evidence?.user_account?.domain_name != null) {\n      userDomain.add(evidence.user_account.domain_name);\n    }\n    if (evidence?.user_account?.azure_ad_user_id != null) {\n      userId.add(evidence.user_account.azure_ad_user_id);\n    }\n    if (evidence?.user_account?.user_principal_name != null) {\n      userEmail.add(evidence.user_account.user_principal_name);\n    }\n    if (evidence?.user_account?.account_name != null) {\n      userName.add(evidence.user_account.account_name);\n    }\n  }\n  if (evidence?.odata_type == '#microsoft.graph.security.processEvidence') {\n    if (evidence?.user_account?.azure_ad_user_id != null) {\n      processUserId.add(evidence.user_account.azure_ad_user_id);\n    }\n    if (evidence?.user_account?.account_name != null) {\n      processUserName.add(evidence.user_account.account_name);\n    }\n  }\n  if (evidence?.vm_metadata?.cloud_provider != null && evidence.vm_metadata.cloud_provider.toLowerCase() == 'azure') {\n    cloudProvider.add('azure');\n  }\n}\nif (!cloudProvider.isEmpty()) {\n  ctx.cloud.provider = convertToOrderedArray(cloudProvider);\n}\nif (!groupName.isEmpty()) {\n  ctx.group.name = convertToOrderedArray(groupName);\n}\nif (!hostId.isEmpty()) {\n  ctx.host.id = convertToOrderedArray(hostId);\n}\nif (!userDomain.isEmpty()) {\n  ctx.user.domain = convertToOrderedArray(userDomain);\n}\nif (!userId.isEmpty()) {\n  ctx.user.id = convertToOrderedArray(userId);\n}\nif (!userName.isEmpty()) {\n  ctx.user.name = convertToOrderedArray(userName);\n}\nif (!userEmail.isEmpty()) {\n  ctx.user.email = convertToOrderedArray(userEmail);\n}\nif (!processUserId.isEmpty()) {\n  ctx.process.user.id = convertToOrderedArray(processUserId);\n}\nif (!processUserName.isEmpty()) {\n  ctx.process.user.name = convertToOrderedArray(processUserName);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def convertToOrderedArray(def list) {\n  def result = new ArrayList();\n  for (element in list) {\n    result.add(element);\n  }\n  Collections.sort(result);\n  return result;\n}\nif (!(ctx.cloud instanceof HashMap)) {\n  ctx.cloud = new HashMap();\n}\ndef cloudProvider = new HashSet();\nif (!(ctx.group instanceof HashMap)) {\n  ctx.group = new HashMap();\n}\ndef groupName = new HashSet();\nif (!(ctx.host instanceof HashMap)) {\n  ctx.host = new HashMap();\n}\ndef hostId = new HashSet();\nif (!(ctx.user instanceof HashMap)) {\n  ctx.user = new HashMap();\n}\ndef userDomain = new HashSet();\ndef userId = new HashSet();\ndef userName = new HashSet();\ndef userEmail = new HashSet();\nif (!(ctx.process instanceof HashMap)) {\n  ctx.process = new HashMap();\n}\nctx.process.user = new HashMap();\ndef processUserId = new HashSet();\ndef processUserName = new HashSet();\nfor (evidence in ctx.json.evidence) {\n  if (evidence?.odata_type == null) {\n    continue\n  }\n  if (evidence?.odata_type == '#microsoft.graph.security.securityGroupEvidence') {\n    if (evidence?.display_name != null) {\n      groupName.add(evidence.display_name);\n    }\n  }\n  if (evidence?.odata_type == '#microsoft.graph.security.deviceEvidence') {\n    if (evidence?.mde_device_id != null) {\n      hostId.add(evidence.mde_device_id);\n    }\n  }\n  if (['#microsoft.graph.security.mailboxEvidence', '#microsoft.graph.security.userEvidence'].contains(evidence.odata_type)) {\n    if (evidence?.user_account?.domain_name != null) {\n      userDomain.add(evidence.user_account.domain_name);\n    }\n    if (evidence?.user_account?.azure_ad_user_id != null) {\n      userId.add(evidence.user_account.azure_ad_user_id);\n    }\n    if (evidence?.user_account?.user_principal_name != null) {\n      userEmail.add(evidence.user_account.user_principal_name);\n    }\n    if (evidence?.user_account?.account_name != null) {\n      userName.add(evidence.user_account.account_name);\n    }\n  }\n  if (evidence?.odata_type == '#microsoft.graph.security.processEvidence') {\n    if (evidence?.user_account?.azure_ad_user_id != null) {\n      processUserId.add(evidence.user_account.azure_ad_user_id);\n    }\n    if (evidence?.user_account?.account_name != null) {\n      processUserName.add(evidence.user_account.account_name);\n    }\n  }\n  if (evidence?.vm_metadata?.cloud_provider != null && evidence.vm_metadata.cloud_provider.toLowerCase() == 'azure') {\n    cloudProvider.add('azure');\n  }\n}\nif (!cloudProvider.isEmpty()) {\n  ctx.cloud.provider = convertToOrderedArray(cloudProvider);\n}\nif (!groupName.isEmpty()) {\n  ctx.group.name = convertToOrderedArray(groupName);\n}\nif (!hostId.isEmpty()) {\n  ctx.host.id = convertToOrderedArray(hostId);\n}\nif (!userDomain.isEmpty()) {\n  ctx.user.domain = convertToOrderedArray(userDomain);\n}\nif (!userId.isEmpty()) {\n  ctx.user.id = convertToOrderedArray(userId);\n}\nif (!userName.isEmpty()) {\n  ctx.user.name = convertToOrderedArray(userName);\n}\nif (!userEmail.isEmpty()) {\n  ctx.user.email = convertToOrderedArray(userEmail);\n}\nif (!processUserId.isEmpty()) {\n  ctx.process.user.id = convertToOrderedArray(processUserId);\n}\nif (!processUserName.isEmpty()) {\n  ctx.process.user.name = convertToOrderedArray(processUserName);\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("process.parent.entity_id")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "process.parent.entity_id", |event| {
                    {
                        let mut values = Vec::new();
                        if let Some(v) = event.get("_ingest._value") {
                            values.push(v.clone());
                        }
                        if !values.is_empty() {
                            event.set("_ingest._value", json!(fingerprint_default(&values)))?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("process.entity_id").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "process.entity_id", |event| {
                    {
                        let mut values = Vec::new();
                        if let Some(v) = event.get("_ingest._value") {
                            values.push(v.clone());
                        }
                        if !values.is_empty() {
                            event.set("_ingest._value", json!(fingerprint_default(&values)))?;
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.evidence").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.evidence", |event| {
                    event.remove("_ingest._value.createdDateTime");
                    event.remove("_ingest._value.receivedDateTime");
                    event.remove("_ingest._value.attachmentsCount");
                    event.remove("_ingest._value.firstSeenDateTime");
                    event.remove("_ingest._value.parentProcessCreationDateTime");
                    event.remove("_ingest._value.processCreationDateTime");
                    event.remove("_ingest._value.senderIp");
                    event.remove("_ingest._value.urlCount");
                    event.remove("_ingest._value.instanceId");
                    event.remove("_ingest._value.appId");
                    event.remove("_ingest._value.saasAppId");
                    event.remove("_ingest._value.osBuild");
                    event.remove("_ingest._value.rbacGroupId");
                    event.remove("_ingest._value.fileDetails.fileSize");
                    event.remove("_ingest._value.ipAddress");
                    event.remove("_ingest._value.emailCount");
                    event.remove("_ingest._value.parentProcessId");
                    event.remove("_ingest._value.processId");
                    event.remove("_ingest._value.imageFile.fileSize");
                    event.remove("_ingest._value.parentProcessImageFile.fileSize");
                    event.remove("_ingest._value.internetMessageId");
                    event.remove("_ingest._value.networkMessageId");
                    Ok(())
                })?;
            }

            if event.has("json.evidence") {
                event.rename("json.evidence", "m365_defender.alert.evidence")?;
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("m365_defender.alert.last_update_datetime");
                event.remove("m365_defender.alert.description");
                event.remove("m365_defender.alert.created_datetime");
                event.remove("m365_defender.alert.last_activity_datetime");
                event.remove("m365_defender.alert.id");
                event.remove("m365_defender.alert.service_source");
                event.remove("m365_defender.alert.first_activity_datetime");
                event.remove("m365_defender.alert.web_url");
                event.remove("m365_defender.alert.assigned_to");
                event.remove("m365_defender.alert.category");
                event.remove("m365_defender.alert.mitre_techniques");
                event.remove("m365_defender.alert.tenant_id");
            }

            let _cond = {
                event
                    .get("m365_defender.alert.evidence")
                    .is_some_and(|v| v.is_array())
                    && (!event.has_value("tags")
                        || !(event.get("tags").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => a
                                .iter()
                                .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                            serde_json::Value::String(s) => {
                                s.contains("preserve_duplicate_custom_fields")
                            }
                            _ => false,
                        })))
            };
            if _cond {
                foreach_array(event, "m365_defender.alert.evidence", |event| {
                    event.remove("_ingest._value.ip_address");
                    event.remove("_ingest._value.detection_status");
                    event.remove("_ingest._value.received_datetime");
                    event.remove("_ingest._value.antispam_direction");
                    event.remove("_ingest._value.subject");
                    event.remove("_ingest._value.recipient_email_address");
                    event.remove("_ingest._value.instance_id");
                    event.remove("_ingest._value.instance_name");
                    event.remove("_ingest._value.vm_metadata.cloud_provider");
                    event.remove("_ingest._value.os_platform");
                    event.remove("_ingest._value.version");
                    event.remove("_ingest._value.file_details.sha1");
                    event.remove("_ingest._value.file_details.sha256");
                    event.remove("_ingest._value.file_details.name");
                    event.remove("_ingest._value.file_details.path");
                    event.remove("_ingest._value.file_details.size");
                    event.remove("_ingest._value.process.command_line");
                    event.remove("_ingest._value.image_file.sha1");
                    event.remove("_ingest._value.image_file.sha256");
                    event.remove("_ingest._value.parent_process.image_file.sha1");
                    event.remove("_ingest._value.parent_process.image_file.sha256");
                    event.remove("_ingest._value.parent_process.id");
                    event.remove("_ingest._value.parent_process.creation_datetime");
                    event.remove("_ingest._value.process.id");
                    event.remove("_ingest._value.process.creation_datetime");
                    event.remove("_ingest._value.registry_value_type");
                    event.remove("_ingest._value.registry_hive");
                    event.remove("_ingest._value.registry_key");
                    event.remove("_ingest._value.registry_value");
                    event.remove("_ingest._value.security_group_id");
                    Ok(())
                })?;
            }

            let _cond = { !event.has_value("process.name") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.process = ctx.process ?: [:];\nctx.process.name = ctx.process.name ?: [];\n// Normalize process.name to a list\ndef nameList = [];\nif (ctx.process.name != null) {\n  if (ctx.process.name instanceof String) {\n    nameList.add(ctx.process.name);\n  } else if (ctx.process.name instanceof List) {\n    nameList.addAll(ctx.process.name);\n  }\n}\n\n// Deduplication using HashSet\ndef currentNames = new HashSet();\ncurrentNames.addAll(nameList);\n// Handle process.command_line (string or list)\nif (ctx.process.command_line != null) {\n  // Convert string to list for unified handling\n  def cmdList = [];\n  if (ctx.process.command_line instanceof String) {\n    cmdList.add(ctx.process.command_line);\n  } else if (ctx.process.command_line instanceof List) {\n    cmdList.addAll(ctx.process.command_line);\n  }\n  for (cmd in cmdList) {\n    if (cmd != null && cmd.length() > 0) {\n      // Extract the first token\n      def parts = cmd.trim().splitOnToken(\" \");\n      if (parts.length > 0) {\n        def executable = parts[0];\n        // If executable is a path, take only the last part\n        if (executable.contains(\"/\")) {\n          def slashParts = executable.splitOnToken(\"/\");\n          executable = slashParts[slashParts.length - 1];\n        }\n        executable = /\\\"/.matcher(executable).replaceAll(\"\");\n        currentNames.add(executable);\n      }\n    }\n  }\n}\n// Update process.name with unique list\nif (currentNames != null && currentNames.size() == 1) {\n  ctx.process.name = currentNames.iterator().next();\n} else {\n  ctx.process.name = new ArrayList(currentNames);\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.process = ctx.process ?: [:];\nctx.process.name = ctx.process.name ?: [];\n// Normalize process.name to a list\ndef nameList = [];\nif (ctx.process.name != null) {\n  if (ctx.process.name instanceof String) {\n    nameList.add(ctx.process.name);\n  } else if (ctx.process.name instanceof List) {\n    nameList.addAll(ctx.process.name);\n  }\n}\n\n// Deduplication using HashSet\ndef currentNames = new HashSet();\ncurrentNames.addAll(nameList);\n// Handle process.command_line (string or list)\nif (ctx.process.command_line != null) {\n  // Convert string to list for unified handling\n  def cmdList = [];\n  if (ctx.process.command_line instanceof String) {\n    cmdList.add(ctx.process.command_line);\n  } else if (ctx.process.command_line instanceof List) {\n    cmdList.addAll(ctx.process.command_line);\n  }\n  for (cmd in cmdList) {\n    if (cmd != null && cmd.length() > 0) {\n      // Extract the first token\n      def parts = cmd.trim().splitOnToken(\" \");\n      if (parts.length > 0) {\n        def executable = parts[0];\n        // If executable is a path, take only the last part\n        if (executable.contains(\"/\")) {\n          def slashParts = executable.splitOnToken(\"/\");\n          executable = slashParts[slashParts.length - 1];\n        }\n        executable = /\\\"/.matcher(executable).replaceAll(\"\");\n        currentNames.add(executable);\n      }\n    }\n  }\n}\n// Update process.name with unique list\nif (currentNames != null && currentNames.size() == 1) {\n  ctx.process.name = currentNames.iterator().next();\n} else {\n  ctx.process.name = new ArrayList(currentNames);\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "set_process_name_from_command_line",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} in pipeline {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_pipeline")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            event.remove("json");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_to_drop_null_values",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

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
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, painless_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, painless_to_string)
                    )),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
