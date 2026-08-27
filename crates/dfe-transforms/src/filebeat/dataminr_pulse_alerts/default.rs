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
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = { event.get_str("message") == Some("retry") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.set("ecs.version", json!("8.11.0"))?;

            event.set("event.kind", json!("event"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            parse_json_field(event, "event.original", "json")?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // SKIPPED: nested pipeline "logs-dataminr_pulse.alerts@custom" is not in this pipeline set
                Ok(())
            })();

            if let Some(v) = event
                .get("json.alertId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            let _cond = {
                event.has_value("json.alertTimestamp")
                    && event.get_str("json.alertTimestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.alertTimestamp") {
                        match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.alertTimestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_alertTimestamp")?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if let Some(v) = event
                .get("json.dataminrAlertUrl")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.url", v)?;
            }

            let _cond = { event.has_value("json.alertType.name") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def alertType = ctx.json.alertType.name;\nif (alertType == 'Flash') {\n  ctx.event.severity = 90;\n} else if (alertType == 'Urgent') {\n  ctx.event.severity = 50;\n} else {\n  ctx.event.severity = 30;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def alertType = ctx.json.alertType.name;\nif (alertType == 'Flash') {\n  ctx.event.severity = 90;\n} else if (alertType == 'Urgent') {\n  ctx.event.severity = 50;\n} else {\n  ctx.event.severity = 30;\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_map_alert_severity",
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
                                .get("_ingest.pipeline")
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
            }

            let _cond = { event.has_value("json.alertType.name") };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            if event.has_value("json.alertType.name") {
                event.rename("json.alertType.name", "dataminr_pulse.alert_type.name")?;
            }

            if let Some(v) = event
                .get("json.headline")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = {
                event.has_value("json.publicPost.timestamp")
                    && event.get_str("json.publicPost.timestamp") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.publicPost.timestamp") {
                        match parse_date_out(&date_str, &["ISO8601", "epoch_millis"], None, None) {
                            Some(parsed) => event.set("event.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.publicPost.timestamp".into(),
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
                        "date_publicPost_timestamp",
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
                                .get("_ingest.pipeline")
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
            }

            let _cond = {
                event.has_value("json.publicPost.href")
                    && event.get_str("json.publicPost.href") != Some("")
            };
            if _cond {
                if event.has_value("json.publicPost.href") {
                    event.rename("json.publicPost.href", "dataminr_pulse.source.href")?;
                }
            }

            let _cond = {
                event.has_value("json.publicPost.channels")
                    && event.get_str("json.publicPost.channels") != Some("")
            };
            if _cond {
                if event.has_value("json.publicPost.channels") {
                    event.rename("json.publicPost.channels", "dataminr_pulse.source.channels")?;
                }
            }

            let _cond = { event.has_value("json.publicPost.media") };
            if _cond {
                foreach_array(event, "json.publicPost.media", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        event.append_unique(
                            "dataminr_pulse.source.media.href",
                            json!(
                                event
                                    .get("_ingest._value.href")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "append")?;
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
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("json.alertTopics") };
            if _cond {
                foreach_array(event, "json.alertTopics", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        event.append_unique(
                            "dataminr_pulse.categories.name",
                            json!(
                                event
                                    .get("_ingest._value.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "append")?;
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
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("json.alertCompanies") };
            if _cond {
                foreach_array(event, "json.alertCompanies", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        event.append_unique(
                            "dataminr_pulse.companies.name",
                            json!(
                                event
                                    .get("_ingest._value.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "append")?;
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
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("json.alertSectors") };
            if _cond {
                foreach_array(event, "json.alertSectors", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        event.append_unique(
                            "dataminr_pulse.sectors.name",
                            json!(
                                event
                                    .get("_ingest._value.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "append")?;
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
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.liveBrief.0.summary") {
                event.rename(
                    "json.liveBrief.0.summary",
                    "dataminr_pulse.live_brief.summary",
                )?;
            }

            if event.has_value("json.liveBrief.0.version") {
                event.rename(
                    "json.liveBrief.0.version",
                    "dataminr_pulse.live_brief.version",
                )?;
            }

            if event.has_value("json.liveBrief.0.timestamp") {
                event.rename(
                    "json.liveBrief.0.timestamp",
                    "dataminr_pulse.live_brief.timestamp",
                )?;
            }

            let _cond = { event.has_value("json.estimatedEventLocation.coordinates") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def coords = ctx.json.estimatedEventLocation.coordinates;\nif (coords instanceof List && coords.size() == 2) {\n  if (ctx.source == null) { ctx.source = [:]; }\n  if (ctx.source.geo == null) { ctx.source.geo = [:]; }\n  if (ctx.source.geo.location == null) { ctx.source.geo.location = [:]; }\n  ctx.source.geo.location.lat = coords[0];\n  ctx.source.geo.location.lon = coords[1];\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def coords = ctx.json.estimatedEventLocation.coordinates;\nif (coords instanceof List && coords.size() == 2) {\n  if (ctx.source == null) { ctx.source = [:]; }\n  if (ctx.source.geo == null) { ctx.source.geo = [:]; }\n  if (ctx.source.geo.location == null) { ctx.source.geo.location = [:]; }\n  ctx.source.geo.location.lat = coords[0];\n  ctx.source.geo.location.lon = coords[1];\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_map_geo_coordinates",
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
                                .get("_ingest.pipeline")
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
            }

            if let Some(v) = event
                .get("json.estimatedEventLocation.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.geo.name", v)?;
            }

            let _cond = {
                event.get("json.intelAgents").is_some_and(|v| v.is_array()) && event.get("json.intelAgents").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) && event.has_value("json.intelAgents.0.summary")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def summary = ctx.json.intelAgents[0].summary;\nif (summary instanceof String) {\n  if (ctx.dataminr_pulse == null) { ctx.dataminr_pulse = [:]; }\n  if (ctx.dataminr_pulse.intel_agents == null) { ctx.dataminr_pulse.intel_agents = [:]; }\n  ctx.dataminr_pulse.intel_agents.summary = summary;\n} else if (summary instanceof List) {\n  def parts = [];\n  for (item in summary) {\n    if (item instanceof Map) {\n      def title = item.containsKey('title') ? item.title : '';\n      def content = '';\n      if (item.containsKey('content') && item.content instanceof List) {\n        content = String.join(' ', item.content);\n      }\n      if (title != '' && content != '') {\n        parts.add(title + ': ' + content);\n      } else if (content != '') {\n        parts.add(content);\n      } else if (title != '') {\n        parts.add(title);\n      }\n    } else if (item instanceof String) {\n      parts.add(item);\n    }\n  }\n  if (parts.size() > 0) {\n    if (ctx.dataminr_pulse == null) { ctx.dataminr_pulse = [:]; }\n    if (ctx.dataminr_pulse.intel_agents == null) { ctx.dataminr_pulse.intel_agents = [:]; }\n    ctx.dataminr_pulse.intel_agents.summary = String.join(' | ', parts);\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def summary = ctx.json.intelAgents[0].summary;\nif (summary instanceof String) {\n  if (ctx.dataminr_pulse == null) { ctx.dataminr_pulse = [:]; }\n  if (ctx.dataminr_pulse.intel_agents == null) { ctx.dataminr_pulse.intel_agents = [:]; }\n  ctx.dataminr_pulse.intel_agents.summary = summary;\n} else if (summary instanceof List) {\n  def parts = [];\n  for (item in summary) {\n    if (item instanceof Map) {\n      def title = item.containsKey('title') ? item.title : '';\n      def content = '';\n      if (item.containsKey('content') && item.content instanceof List) {\n        content = String.join(' ', item.content);\n      }\n      if (title != '' && content != '') {\n        parts.add(title + ': ' + content);\n      } else if (content != '') {\n        parts.add(content);\n      } else if (title != '') {\n        parts.add(title);\n      }\n    } else if (item instanceof String) {\n      parts.add(item);\n    }\n  }\n  if (parts.size() > 0) {\n    if (ctx.dataminr_pulse == null) { ctx.dataminr_pulse = [:]; }\n    if (ctx.dataminr_pulse.intel_agents == null) { ctx.dataminr_pulse.intel_agents = [:]; }\n    ctx.dataminr_pulse.intel_agents.summary = String.join(' | ', parts);\n  }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_intelAgents_summary",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag '{}' in pipeline {} failed with message: {}",
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("json.listsMatched") };
            if _cond {
                foreach_array(event, "json.listsMatched", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        event.append_unique(
                            "dataminr_pulse.watchlists_matched_by_type.name",
                            json!(
                                event
                                    .get("_ingest._value.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "append")?;
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
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.get("json.intelAgents").is_some_and(|v| v.is_array()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def threatActorNames = [];\ndef threatActorAliases = [];\ndef threatActorCountries = [];\ndef vulnerabilityNames = [];\ndef malwareList = [];\ndef platformsSet = new LinkedHashSet();\ndef urlList = [];\ndef ipList = [];\ndef enrichments = [];\ndef firstVulnCvss = null;\ndef firstVulnDescription = null;\n\nfor (intelAgent in ctx.json.intelAgents) {\n  if (!(intelAgent.discoveredEntities instanceof List)) { continue; }\n  for (entity in intelAgent.discoveredEntities) {\n    if (entity.type == 'threatActor' && entity.name != null && entity.name != '') {\n      if (!threatActorNames.contains(entity.name)) {\n        threatActorNames.add(entity.name);\n      }\n      if (entity.aliases instanceof List) {\n        for (alias in entity.aliases) {\n          if (!threatActorAliases.contains(alias)) {\n            threatActorAliases.add(alias);\n          }\n        }\n      }\n      if (entity.countryOfOrigin != null) {\n        def countries = entity.countryOfOrigin instanceof List ? entity.countryOfOrigin : [entity.countryOfOrigin];\n        for (country in countries) {\n          if (country instanceof String && !threatActorCountries.contains(country)) {\n            threatActorCountries.add(country);\n          }\n        }\n      }\n    } else if (entity.type == 'vulnerability') {\n      def vulnName = entity.cve != null && entity.cve != '' ? entity.cve\n                   : entity.name != null && entity.name != '' ? entity.name\n                   : entity.id != null && entity.id != '' ? entity.id\n                   : null;\n      if (vulnName != null && !vulnerabilityNames.contains(vulnName)) {\n        vulnerabilityNames.add(vulnName);\n      }\n      if (firstVulnCvss == null && entity.cvss != null) {\n        firstVulnCvss = entity.cvss;\n      }\n      if (firstVulnDescription == null && entity.summary != null && entity.summary != '') {\n        firstVulnDescription = entity.summary;\n      }\n    } else if (entity.type == 'malware' && entity.name != null && entity.name != '') {\n      if (!malwareList.contains(entity.name)) {\n        malwareList.add(entity.name);\n      }\n      if (entity.affectedOperatingSystems instanceof List) {\n        def osMapping = [\n          'Windows': ['win', 'windows', 'win32', 'win64', 'microsoft windows'],\n          'Linux': ['elf', 'linux', 'unix'],\n          'macOS': ['mac', 'macos', 'osx', 'os x', 'darwin', 'mac os', 'mac os x'],\n          'AWS': ['aws', 'amazon web services'],\n          'Azure': ['azure', 'microsoft azure'],\n          'Azure AD': ['azure ad', 'azure active directory', 'azuread'],\n          'GCP': ['gcp', 'google cloud', 'google cloud platform'],\n          'Network': ['network'],\n          'Office 365': ['office 365', 'o365', 'office365', 'microsoft 365', 'm365'],\n          'SaaS': ['saas', 'software as a service']\n          ];\n        for (os in entity.affectedOperatingSystems) {\n          if (os == null) continue;\n\n          def osLower = os.toLowerCase();\n          def normalized = null;\n        \n          for (entry in osMapping.entrySet()) {\n            if (entry.getValue().contains(osLower)) {\n            normalized = entry.getKey();\n            break;\n            }\n          }\n          platformsSet.add(normalized);\n      }\n    }\n\n    } else if (entity.type == 'url' && entity.name != null && entity.name != '') {\n      def refanged = entity.name.replace('[.]', '.');\n      if (!urlList.contains(refanged)) {\n        urlList.add(refanged);\n      }\n      def e = [:];\n      e.indicator = [:];\n      e.indicator.type = 'url';\n      e.indicator.name = entity.name;\n      e.indicator.url = ['original': refanged];\n      enrichments.add(e);\n    } else if (entity.type == 'ipAddress' && entity.ip != null && entity.ip != '') {\n      def refanged = entity.ip.replace('[.]', '.');\n      if (!ipList.contains(refanged)) {\n        ipList.add(refanged);\n      }\n      def e = [:];\n      e.indicator = [:];\n      e.indicator.name = entity.ip;\n      e.indicator.ip = refanged;\n      e.indicator.type = refanged.contains(':') ? 'ipv6-addr' : 'ipv4-addr';\n      if (entity.ports instanceof List && entity.ports.size() > 0) {\n        e.indicator.port = entity.ports;\n      }\n      enrichments.add(e);\n    }\n  }\n}\n\nif (ctx.dataminr_pulse == null) { ctx.dataminr_pulse = [:]; }\n\nif (threatActorNames.size() > 0) {\n  ctx.dataminr_pulse.threatactor = [:];\n  ctx.dataminr_pulse.threatactor.name = threatActorNames;\n  if (threatActorAliases.size() > 0) { ctx.dataminr_pulse.threatactor.alias = threatActorAliases; }\n  if (threatActorCountries.size() > 0) { ctx.dataminr_pulse.threatactor.country_of_origin = threatActorCountries; }\n  if (ctx.threat == null) { ctx.threat = [:]; }\n  ctx.threat.group = [:];\n  ctx.threat.group.name = threatActorNames[0];\n  if (threatActorAliases.size() > 0) { ctx.threat.group.alias = threatActorAliases; }\n  if (threatActorCountries.size() > 0) {\n    ctx.threat.indicator = [:];\n    ctx.threat.indicator.geo = [:];\n    ctx.threat.indicator.geo.country_iso_code = threatActorCountries;\n  }\n  ctx.threat.framework = 'MITRE ATT&CK';\n}\n\nif (vulnerabilityNames.size() > 0) {\n  ctx.dataminr_pulse.vulnerability = [:];\n  ctx.dataminr_pulse.vulnerability.name = vulnerabilityNames;\n  ctx.vulnerability = [:];\n  ctx.vulnerability.id = vulnerabilityNames[0];\n  if (firstVulnCvss != null) {\n    ctx.vulnerability.score = [:];\n    ctx.vulnerability.score.base = firstVulnCvss;\n  }\n  if (firstVulnDescription != null) {\n    ctx.vulnerability.description = firstVulnDescription;\n  }\n}\n\nif (malwareList.size() > 0) {\n  ctx.dataminr_pulse.malware = malwareList;\n  if (ctx.threat == null) { ctx.threat = [:]; }\n  ctx.threat.software = [:];\n  ctx.threat.software.name = malwareList[0];\n  ctx.threat.software.type = 'Malware';\n  if (platformsSet.size() > 0) {\n    ctx.dataminr_pulse.platforms = new ArrayList(platformsSet);\n    ctx.threat.software.platforms = new ArrayList(platformsSet);\n  }\n}\n\nif (urlList.size() > 0) { ctx.dataminr_pulse.url = urlList; }\n\nif (ipList.size() > 0) {\n  ctx.dataminr_pulse.ip = ipList;\n  if (ctx.related == null) { ctx.related = [:]; }\n  ctx.related.ip = ipList;\n}\n\nif (enrichments.size() > 0) {\n  if (ctx.threat == null) { ctx.threat = [:]; }\n  ctx.threat.enrichments = enrichments;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def threatActorNames = [];\ndef threatActorAliases = [];\ndef threatActorCountries = [];\ndef vulnerabilityNames = [];\ndef malwareList = [];\ndef platformsSet = new LinkedHashSet();\ndef urlList = [];\ndef ipList = [];\ndef enrichments = [];\ndef firstVulnCvss = null;\ndef firstVulnDescription = null;\n\nfor (intelAgent in ctx.json.intelAgents) {\n  if (!(intelAgent.discoveredEntities instanceof List)) { continue; }\n  for (entity in intelAgent.discoveredEntities) {\n    if (entity.type == 'threatActor' && entity.name != null && entity.name != '') {\n      if (!threatActorNames.contains(entity.name)) {\n        threatActorNames.add(entity.name);\n      }\n      if (entity.aliases instanceof List) {\n        for (alias in entity.aliases) {\n          if (!threatActorAliases.contains(alias)) {\n            threatActorAliases.add(alias);\n          }\n        }\n      }\n      if (entity.countryOfOrigin != null) {\n        def countries = entity.countryOfOrigin instanceof List ? entity.countryOfOrigin : [entity.countryOfOrigin];\n        for (country in countries) {\n          if (country instanceof String && !threatActorCountries.contains(country)) {\n            threatActorCountries.add(country);\n          }\n        }\n      }\n    } else if (entity.type == 'vulnerability') {\n      def vulnName = entity.cve != null && entity.cve != '' ? entity.cve\n                   : entity.name != null && entity.name != '' ? entity.name\n                   : entity.id != null && entity.id != '' ? entity.id\n                   : null;\n      if (vulnName != null && !vulnerabilityNames.contains(vulnName)) {\n        vulnerabilityNames.add(vulnName);\n      }\n      if (firstVulnCvss == null && entity.cvss != null) {\n        firstVulnCvss = entity.cvss;\n      }\n      if (firstVulnDescription == null && entity.summary != null && entity.summary != '') {\n        firstVulnDescription = entity.summary;\n      }\n    } else if (entity.type == 'malware' && entity.name != null && entity.name != '') {\n      if (!malwareList.contains(entity.name)) {\n        malwareList.add(entity.name);\n      }\n      if (entity.affectedOperatingSystems instanceof List) {\n        def osMapping = [\n          'Windows': ['win', 'windows', 'win32', 'win64', 'microsoft windows'],\n          'Linux': ['elf', 'linux', 'unix'],\n          'macOS': ['mac', 'macos', 'osx', 'os x', 'darwin', 'mac os', 'mac os x'],\n          'AWS': ['aws', 'amazon web services'],\n          'Azure': ['azure', 'microsoft azure'],\n          'Azure AD': ['azure ad', 'azure active directory', 'azuread'],\n          'GCP': ['gcp', 'google cloud', 'google cloud platform'],\n          'Network': ['network'],\n          'Office 365': ['office 365', 'o365', 'office365', 'microsoft 365', 'm365'],\n          'SaaS': ['saas', 'software as a service']\n          ];\n        for (os in entity.affectedOperatingSystems) {\n          if (os == null) continue;\n\n          def osLower = os.toLowerCase();\n          def normalized = null;\n        \n          for (entry in osMapping.entrySet()) {\n            if (entry.getValue().contains(osLower)) {\n            normalized = entry.getKey();\n            break;\n            }\n          }\n          platformsSet.add(normalized);\n      }\n    }\n\n    } else if (entity.type == 'url' && entity.name != null && entity.name != '') {\n      def refanged = entity.name.replace('[.]', '.');\n      if (!urlList.contains(refanged)) {\n        urlList.add(refanged);\n      }\n      def e = [:];\n      e.indicator = [:];\n      e.indicator.type = 'url';\n      e.indicator.name = entity.name;\n      e.indicator.url = ['original': refanged];\n      enrichments.add(e);\n    } else if (entity.type == 'ipAddress' && entity.ip != null && entity.ip != '') {\n      def refanged = entity.ip.replace('[.]', '.');\n      if (!ipList.contains(refanged)) {\n        ipList.add(refanged);\n      }\n      def e = [:];\n      e.indicator = [:];\n      e.indicator.name = entity.ip;\n      e.indicator.ip = refanged;\n      e.indicator.type = refanged.contains(':') ? 'ipv6-addr' : 'ipv4-addr';\n      if (entity.ports instanceof List && entity.ports.size() > 0) {\n        e.indicator.port = entity.ports;\n      }\n      enrichments.add(e);\n    }\n  }\n}\n\nif (ctx.dataminr_pulse == null) { ctx.dataminr_pulse = [:]; }\n\nif (threatActorNames.size() > 0) {\n  ctx.dataminr_pulse.threatactor = [:];\n  ctx.dataminr_pulse.threatactor.name = threatActorNames;\n  if (threatActorAliases.size() > 0) { ctx.dataminr_pulse.threatactor.alias = threatActorAliases; }\n  if (threatActorCountries.size() > 0) { ctx.dataminr_pulse.threatactor.country_of_origin = threatActorCountries; }\n  if (ctx.threat == null) { ctx.threat = [:]; }\n  ctx.threat.group = [:];\n  ctx.threat.group.name = threatActorNames[0];\n  if (threatActorAliases.size() > 0) { ctx.threat.group.alias = threatActorAliases; }\n  if (threatActorCountries.size() > 0) {\n    ctx.threat.indicator = [:];\n    ctx.threat.indicator.geo = [:];\n    ctx.threat.indicator.geo.country_iso_code = threatActorCountries;\n  }\n  ctx.threat.framework = 'MITRE ATT&CK';\n}\n\nif (vulnerabilityNames.size() > 0) {\n  ctx.dataminr_pulse.vulnerability = [:];\n  ctx.dataminr_pulse.vulnerability.name = vulnerabilityNames;\n  ctx.vulnerability = [:];\n  ctx.vulnerability.id = vulnerabilityNames[0];\n  if (firstVulnCvss != null) {\n    ctx.vulnerability.score = [:];\n    ctx.vulnerability.score.base = firstVulnCvss;\n  }\n  if (firstVulnDescription != null) {\n    ctx.vulnerability.description = firstVulnDescription;\n  }\n}\n\nif (malwareList.size() > 0) {\n  ctx.dataminr_pulse.malware = malwareList;\n  if (ctx.threat == null) { ctx.threat = [:]; }\n  ctx.threat.software = [:];\n  ctx.threat.software.name = malwareList[0];\n  ctx.threat.software.type = 'Malware';\n  if (platformsSet.size() > 0) {\n    ctx.dataminr_pulse.platforms = new ArrayList(platformsSet);\n    ctx.threat.software.platforms = new ArrayList(platformsSet);\n  }\n}\n\nif (urlList.size() > 0) { ctx.dataminr_pulse.url = urlList; }\n\nif (ipList.size() > 0) {\n  ctx.dataminr_pulse.ip = ipList;\n  if (ctx.related == null) { ctx.related = [:]; }\n  ctx.related.ip = ipList;\n}\n\nif (enrichments.size() > 0) {\n  if (ctx.threat == null) { ctx.threat = [:]; }\n  ctx.threat.enrichments = enrichments;\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "extract_entities_and_map_ecs",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag '{}' in pipeline {} failed with message: {}",
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: // Initialize event.category as array if not exists\nif (ctx.event == null) { ctx.event = [:]; }\nif (ctx.event.category == null) { ctx.event.category = []; }\nif (ctx.event.type == null) { ctx.event.type = []; }\n\nboolean hasEntities = false;\n\n// Check for vulnerability\nif (ctx.dataminr_pulse?.vulnerability?.name != null && ctx.dataminr_pulse.vulnerability.name.size() > 0) {\n  if (!ctx.event.category.contains('vulnerability')) {\n    ctx.event.category.add('vulnerability');\n  }\n  if (!ctx.event.type.contains('info')) {\n    ctx.event.type.add('info');\n  }\n  hasEntities = true;\n}\n\n// Check for threat actor\nif (ctx.dataminr_pulse?.threatactor?.name != null && ctx.dataminr_pulse.threatactor.name.size() > 0) {\n  if (!ctx.event.category.contains('threat')) {\n    ctx.event.category.add('threat');\n  }\n  if (!ctx.event.type.contains('indicator')) {\n    ctx.event.type.add('indicator');\n  }\n  hasEntities = true;\n}\n\n// Check for malware\nif (ctx.dataminr_pulse?.malware != null && ctx.dataminr_pulse.malware.size() > 0) {\n  if (!ctx.event.category.contains('malware')) {\n    ctx.event.category.add('malware');\n  }\n  if (!ctx.event.type.contains('info')) {\n    ctx.event.type.add('info');\n  }\n  hasEntities = true;\n}\n\n// Check for network indicators (IPs or URLs)\nif ((ctx.dataminr_pulse?.ip != null && ctx.dataminr_pulse.ip.size() > 0) ||\n    (ctx.dataminr_pulse?.url != null && ctx.dataminr_pulse.url.size() > 0)) {\n  if (!ctx.event.category.contains('threat')) {\n    ctx.event.category.add('threat');\n  }\n  if (!ctx.event.type.contains('indicator')) {\n    ctx.event.type.add('indicator');\n  }\n  hasEntities = true;\n}\n\n// Fallback to 'web' and 'info' if no specific entities found\nif (!hasEntities) {\n  ctx.event.category.add('web');\n  if (!ctx.event.type.contains('info')) {\n    ctx.event.type.add('info');\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"// Initialize event.category as array if not exists\nif (ctx.event == null) { ctx.event = [:]; }\nif (ctx.event.category == null) { ctx.event.category = []; }\nif (ctx.event.type == null) { ctx.event.type = []; }\n\nboolean hasEntities = false;\n\n// Check for vulnerability\nif (ctx.dataminr_pulse?.vulnerability?.name != null && ctx.dataminr_pulse.vulnerability.name.size() > 0) {\n  if (!ctx.event.category.contains('vulnerability')) {\n    ctx.event.category.add('vulnerability');\n  }\n  if (!ctx.event.type.contains('info')) {\n    ctx.event.type.add('info');\n  }\n  hasEntities = true;\n}\n\n// Check for threat actor\nif (ctx.dataminr_pulse?.threatactor?.name != null && ctx.dataminr_pulse.threatactor.name.size() > 0) {\n  if (!ctx.event.category.contains('threat')) {\n    ctx.event.category.add('threat');\n  }\n  if (!ctx.event.type.contains('indicator')) {\n    ctx.event.type.add('indicator');\n  }\n  hasEntities = true;\n}\n\n// Check for malware\nif (ctx.dataminr_pulse?.malware != null && ctx.dataminr_pulse.malware.size() > 0) {\n  if (!ctx.event.category.contains('malware')) {\n    ctx.event.category.add('malware');\n  }\n  if (!ctx.event.type.contains('info')) {\n    ctx.event.type.add('info');\n  }\n  hasEntities = true;\n}\n\n// Check for network indicators (IPs or URLs)\nif ((ctx.dataminr_pulse?.ip != null && ctx.dataminr_pulse.ip.size() > 0) ||\n    (ctx.dataminr_pulse?.url != null && ctx.dataminr_pulse.url.size() > 0)) {\n  if (!ctx.event.category.contains('threat')) {\n    ctx.event.category.add('threat');\n  }\n  if (!ctx.event.type.contains('indicator')) {\n    ctx.event.type.add('indicator');\n  }\n  hasEntities = true;\n}\n\n// Fallback to 'web' and 'info' if no specific entities found\nif (!hasEntities) {\n  ctx.event.category.add('web');\n  if (!ctx.event.type.contains('info')) {\n    ctx.event.type.add('info');\n  }\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "set_event_category_based_on_entities",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag '{}' in pipeline {} failed with message: {}",
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
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("json.metadata.cyber.hashValues") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def hashes = ctx.json.metadata.cyber.hashValues;\nif (hashes instanceof List && hashes.size() > 0) {\n  if (ctx.file == null) { ctx.file = [:]; }\n  if (ctx.file.hash == null) { ctx.file.hash = [:]; }\n  if (ctx.related == null) { ctx.related = [:]; }\n  if (ctx.related.hash == null) { ctx.related.hash = []; }\n\n  for (hash in hashes) {\n    if (hash.type != null && hash.value != null) {\n      def hashType = hash.type.toLowerCase();\n      if (hashType == 'md5') {\n        ctx.file.hash.md5 = hash.value;\n        if (!ctx.related.hash.contains(hash.value)) {\n          ctx.related.hash.add(hash.value);\n        }\n      } else if (hashType == 'sha1') {\n        ctx.file.hash.sha1 = hash.value;\n        if (!ctx.related.hash.contains(hash.value)) {\n          ctx.related.hash.add(hash.value);\n        }\n      } else if (hashType == 'sha256') {\n        ctx.file.hash.sha256 = hash.value;\n        if (!ctx.related.hash.contains(hash.value)) {\n          ctx.related.hash.add(hash.value);\n        }\n      } else if (hashType == 'sha512') {\n        ctx.file.hash.sha512 = hash.value;\n        if (!ctx.related.hash.contains(hash.value)) {\n          ctx.related.hash.add(hash.value);\n        }\n      }\n    }\n  }\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def hashes = ctx.json.metadata.cyber.hashValues;\nif (hashes instanceof List && hashes.size() > 0) {\n  if (ctx.file == null) { ctx.file = [:]; }\n  if (ctx.file.hash == null) { ctx.file.hash = [:]; }\n  if (ctx.related == null) { ctx.related = [:]; }\n  if (ctx.related.hash == null) { ctx.related.hash = []; }\n\n  for (hash in hashes) {\n    if (hash.type != null && hash.value != null) {\n      def hashType = hash.type.toLowerCase();\n      if (hashType == 'md5') {\n        ctx.file.hash.md5 = hash.value;\n        if (!ctx.related.hash.contains(hash.value)) {\n          ctx.related.hash.add(hash.value);\n        }\n      } else if (hashType == 'sha1') {\n        ctx.file.hash.sha1 = hash.value;\n        if (!ctx.related.hash.contains(hash.value)) {\n          ctx.related.hash.add(hash.value);\n        }\n      } else if (hashType == 'sha256') {\n        ctx.file.hash.sha256 = hash.value;\n        if (!ctx.related.hash.contains(hash.value)) {\n          ctx.related.hash.add(hash.value);\n        }\n      } else if (hashType == 'sha512') {\n        ctx.file.hash.sha512 = hash.value;\n        if (!ctx.related.hash.contains(hash.value)) {\n          ctx.related.hash.add(hash.value);\n        }\n      }\n    }\n  }\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_map_hash_values")?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("json.alertId") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("json.alertId") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }
            }

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    }))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("event.original");
                    Ok(())
                })();
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            if let Some(v) = event
                .get("json.headline")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            event.remove("json");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).size() == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).size() == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag '{}' in pipeline {} failed with message: {}",
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
