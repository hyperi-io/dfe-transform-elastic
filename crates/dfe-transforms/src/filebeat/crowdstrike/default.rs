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

            event.set("ecs.version", json!("8.17.0"))?;

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
                parse_json_field(event, "event.original", "crowdstrike")?;
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
                event.has_value("crowdstrike")
                    && !(event.get("crowdstrike").is_some_and(|v| v.is_object()))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.remove("host.name");

            event.set("observer.vendor", json!("Crowdstrike"))?;

            event.set("observer.product", json!("Falcon"))?;

            if event.has_value("crowdstrike.event.IncidentType") {
                if let Some(val) = event.get("crowdstrike.event.IncidentType") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "crowdstrike.event.IncidentType".into(),
                            message,
                        }
                    })?;
                    event.set("crowdstrike.event.IncidentType", converted)?;
                }
            }

            if event.has_value("crowdstrike.event.PatternId") {
                if let Some(val) = event.get("crowdstrike.event.PatternId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "crowdstrike.event.PatternId".into(),
                            message,
                        }
                    })?;
                    event.set("crowdstrike.event.PatternId", converted)?;
                }
            }

            let _cond = {
                event
                    .get("crowdstrike.event")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // Painless script
                // Source: def convertToUnix(def longValue) {\n    if (longValue > 0x0100000000000000L) {\n        return (longValue / 10000) - 11644473600000L;\n    }\n    return longValue;\n}\n\nfor (def field : params.values) {\n    def fieldValue = ctx.crowdstrike.event[field];\n    if (fieldValue != null) {\n        if (fieldValue instanceof long) {\n             ctx.crowdstrike.event[field] = convertToUnix(fieldValue);\n        } else if (fieldValue instanceof String) {\n            if (!fieldValue.contains('.')) {\n               def timestamp = Long.parseLong(fieldValue);\n                ctx.crowdstrike.event[field] = convertToUnix(timestamp);\n            }\n        }\n    } \n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def convertToUnix(def longValue) {\n    if (longValue > 0x0100000000000000L) {\n        return (longValue / 10000) - 11644473600000L;\n    }\n    return longValue;\n}\n\nfor (def field : params.values) {\n    def fieldValue = ctx.crowdstrike.event[field];\n    if (fieldValue != null) {\n        if (fieldValue instanceof long) {\n             ctx.crowdstrike.event[field] = convertToUnix(fieldValue);\n        } else if (fieldValue instanceof String) {\n            if (!fieldValue.contains('.')) {\n               def timestamp = Long.parseLong(fieldValue);\n                ctx.crowdstrike.event[field] = convertToUnix(timestamp);\n            }\n        }\n    } \n}\n"#
                    ),
                    cached_params!(
                        "{\"values\":[\"StartTime\",\"EndTime\",\"ContextTimeStamp\",\"EndTimestamp\",\"IncidentEndTime\",\"IncidentStartTime\",\"ItemPostedTimestamp\",\"MatchedTimestamp\",\"MostRecentActivityTimeStamp\",\"PrecedingActivityTimeStamp\",\"StartTimestamp\",\"UTCTimestamp\"]}"
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.Tags") };
            if _cond {
                // Painless script
                // Source: if (ctx.crowdstrike.event.Tags instanceof List) {\n    for (tag in ctx.crowdstrike.event.Tags) {\n        if (tag instanceof Map) {\n          ctx.tags.add(tag[\"Key\"] + \":\" + tag[\"ValueString\"]);\n        }\n    }\n} else if (ctx.crowdstrike.event.Tags instanceof String) {\n    def values = ctx.crowdstrike.event.Tags.splitOnToken(',');\n    for (value in values) {\n        ctx.tags.add(value.trim());\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.crowdstrike.event.Tags instanceof List) {\n    for (tag in ctx.crowdstrike.event.Tags) {\n        if (tag instanceof Map) {\n          ctx.tags.add(tag[\"Key\"] + \":\" + tag[\"ValueString\"]);\n        }\n    }\n} else if (ctx.crowdstrike.event.Tags instanceof String) {\n    def values = ctx.crowdstrike.event.Tags.splitOnToken(',');\n    for (value in values) {\n        ctx.tags.add(value.trim());\n    }\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("crowdstrike.event.UTCTimestamp")
                    && event
                        .get_as_string("crowdstrike.event.UTCTimestamp")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.UTCTimestamp") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("@timestamp", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.event.UTCTimestamp")
                    && event
                        .get_as_string("crowdstrike.event.UTCTimestamp")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("crowdstrike.event.UTCTimestamp") {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        event.set("@timestamp", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.metadata.eventCreationTime")
                    && event
                        .get_as_string("crowdstrike.metadata.eventCreationTime")
                        .is_some_and(|s| s.len() <= 11)
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("crowdstrike.metadata.eventCreationTime")
                {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], Some("UTC"), None) {
                        event.set("event.created", parsed)?;
                    }
                }
            }

            let _cond = {
                event.has_value("crowdstrike.metadata.eventCreationTime")
                    && event
                        .get_as_string("crowdstrike.metadata.eventCreationTime")
                        .is_some_and(|s| s.len() >= 12)
            };
            if _cond {
                if let Some(date_str) =
                    event.get_as_string("crowdstrike.metadata.eventCreationTime")
                {
                    if let Some(parsed) = parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                    {
                        event.set("event.created", parsed)?;
                    }
                }
            }

            let _cond = {
                !event.has_value("crowdstrike.event.UTCTimestamp")
                    && event.has_value("event.created")
            };
            if _cond {
                if let Some(v) = event.get("event.created").cloned() {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = {
                event
                    .get("crowdstrike.event.SeverityName")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // Painless script
                // Source: ctx.event = ctx.event ?: [:];\nString name = ctx.crowdstrike.event.SeverityName;\nif (name.equalsIgnoreCase(\"low\") || name.equalsIgnoreCase(\"info\") || name.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (name.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (name.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (name.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.event = ctx.event ?: [:];\nString name = ctx.crowdstrike.event.SeverityName;\nif (name.equalsIgnoreCase(\"low\") || name.equalsIgnoreCase(\"info\") || name.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (name.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (name.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (name.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("crowdstrike.event")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // Painless script
                // Source: ctx.crowdstrike.event.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.crowdstrike.event.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));\n"#
                    ),
                    cached_params!("{\"values\":[null,\"\",\"-\",\"N/A\",\"NA\",0]}"),
                )?;
            }

            let _cond = {
                event
                    .get("crowdstrike.metadata")
                    .is_some_and(|v| v.is_object())
            };
            if _cond {
                // Painless script
                // Source: ctx.crowdstrike.metadata.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"ctx.crowdstrike.metadata.entrySet().removeIf(entry -> params.values.contains(entry.getValue()));\n"#
                    ),
                    cached_params!("{\"values\":[null,\"\",\"-\",\"N/A\",\"NA\"]}"),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.CommandLine") };
            if _cond {
                // Painless script
                // Source: def commandLine = ctx.crowdstrike?.event?.CommandLine;\ncommandLine = commandLine.trim();\n\nif (commandLine != \"\") {\n  def args = new ArrayList(Arrays.asList(/ /.split(commandLine)));\n  args.removeIf(arg -> arg == \"\");\n\n  ctx.process = [\n    'command_line': commandLine,\n    'args': args,\n    'executable': args.get(0)\n  ]\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def commandLine = ctx.crowdstrike?.event?.CommandLine;\ncommandLine = commandLine.trim();\n\nif (commandLine != \"\") {\n  def args = new ArrayList(Arrays.asList(/ /.split(commandLine)));\n  args.removeIf(arg -> arg == \"\");\n\n  ctx.process = [\n    'command_line': commandLine,\n    'args': args,\n    'executable': args.get(0)\n  ]\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("crowdstrike.event.ParentCommandLine") };
            if _cond {
                // Painless script
                // Source: def parentCommandLine = ctx.crowdstrike?.event?.ParentCommandLine;\nparentCommandLine = parentCommandLine.trim();\n\nif (parentCommandLine != \"\") {\n  def args = new ArrayList(Arrays.asList(/ /.split(parentCommandLine)));\n  args.removeIf(arg -> arg == \"\");\n  if (ctx.process == null) {\n    ctx.process = new HashMap();\n  }\n  ctx.process.parent = [\n    'command_line': parentCommandLine,\n    'args': args,\n    'executable': args.get(0)\n  ]\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def parentCommandLine = ctx.crowdstrike?.event?.ParentCommandLine;\nparentCommandLine = parentCommandLine.trim();\n\nif (parentCommandLine != \"\") {\n  def args = new ArrayList(Arrays.asList(/ /.split(parentCommandLine)));\n  args.removeIf(arg -> arg == \"\");\n  if (ctx.process == null) {\n    ctx.process = new HashMap();\n  }\n  ctx.process.parent = [\n    'command_line': parentCommandLine,\n    'args': args,\n    'executable': args.get(0)\n  ]\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType") == Some("AutomatedLeadSummaryEvent")
            };
            if _cond {
                // Begin nested pipeline: "automated_lead_summary"
                event.set("event.kind", json!("alert"))?;
                event.append("event.category", json!("threat"))?;
                event.append("event.type", json!("indicator"))?;
                if let Some(v) = event
                    .get("crowdstrike.event.Name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("message", v)?;
                }
                let _cond = { !event.has_value("message") || event.get_str("message") == Some("") };
                if _cond {
                    if let Some(v) = event
                        .get("crowdstrike.event.Description")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("message", v)?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.SignalStartTimestamp")
                        && event.get_i64("crowdstrike.event.SignalStartTimestamp") != Some(0)
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("crowdstrike.event.SignalStartTimestamp")
                        {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None)
                            {
                                event.set("crowdstrike.event.SignalStartTimestamp", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_signal_start_timestamp",
                        )?;
                        event.remove("crowdstrike.event.SignalStartTimestamp");
                        event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("crowdstrike.event.SignalStartTimestamp")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.start", v)?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.SignalEndTimestamp")
                        && event.get_i64("crowdstrike.event.SignalEndTimestamp") != Some(0)
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("crowdstrike.event.SignalEndTimestamp")
                        {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None)
                            {
                                event.set("crowdstrike.event.SignalEndTimestamp", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_signal_end_timestamp",
                        )?;
                        event.remove("crowdstrike.event.SignalEndTimestamp");
                        event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if let Some(v) = event
                    .get("crowdstrike.event.SignalEndTimestamp")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.end", v)?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.SignalUpdatedTimestamp")
                        && event.get_i64("crowdstrike.event.SignalUpdatedTimestamp") != Some(0)
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("crowdstrike.event.SignalUpdatedTimestamp")
                        {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None)
                            {
                                event.set("crowdstrike.event.SignalUpdatedTimestamp", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_signal_updated_timestamp",
                        )?;
                        event.remove("crowdstrike.event.SignalUpdatedTimestamp");
                        event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
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
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if let Some(Value::Array(items)) = event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.SignalAssociationTimestamp")
                                {
                                    if let Some(parsed) =
                                        parse_date_out(&date_str, &["UNIX", "UNIX_MS"], None, None)
                                    {
                                        event.set(
                                            "_ingest._value.SignalAssociationTimestamp",
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
                                    "date_threatgraph_indicators_signal_association_timestamp",
                                )?;
                                event.remove("_ingest._value.SignalAssociationTimestamp");
                                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("crowdstrike.event.ThreatgraphIndicators", Value::Array(out))?;
                    }
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.event.Score") {
                        if let Some(val) = event.get("crowdstrike.event.Score") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.Score".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.Score", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_score_to_long")?;
                    event.remove("crowdstrike.event.Score");
                    event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = {
                    event
                        .get("crowdstrike.event.Score")
                        .is_some_and(|v| v.is_number())
                };
                if _cond {
                    // Painless script
                    // Source: long score = ctx.crowdstrike.event.Score;\nctx.event = ctx.event ?: [:];\nctx.event.risk_score = (double) score;\nif (score < 40) {\n  ctx.event.severity = 21;\n} else if (score < 60) {\n  ctx.event.severity = 47;\n} else if (score < 80) {\n  ctx.event.severity = 73;\n} else {\n  ctx.event.severity = 99;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"long score = ctx.crowdstrike.event.Score;\nctx.event = ctx.event ?: [:];\nctx.event.risk_score = (double) score;\nif (score < 40) {\n  ctx.event.severity = 21;\n} else if (score < 60) {\n  ctx.event.severity = 47;\n} else if (score < 80) {\n  ctx.event.severity = 73;\n} else {\n  ctx.event.severity = 99;\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if let Some(Value::Array(items)) = event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has_value("_ingest._value.PatternId") {
                                if let Some(val) = event.get("_ingest._value.PatternId") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.PatternId".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.PatternId", converted)?;
                                }
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("crowdstrike.event.ThreatgraphIndicators", Value::Array(out))?;
                    }
                }
                let _cond = {
                    event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if let Some(Value::Array(items)) = event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            if event.has_value("_ingest._value.TemplateInstanceId") {
                                if let Some(val) = event.get("_ingest._value.TemplateInstanceId") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.TemplateInstanceId".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.TemplateInstanceId", converted)?;
                                }
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("crowdstrike.event.ThreatgraphIndicators", Value::Array(out))?;
                    }
                }
                let _cond = {
                    event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if let Some(Value::Array(items)) = event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.Severity") {
                                    if let Some(val) = event.get("_ingest._value.Severity") {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.Severity".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.Severity", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_threatgraph_indicators_severity_to_long",
                                )?;
                                event.remove("_ingest._value.Severity");
                                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("crowdstrike.event.ThreatgraphIndicators", Value::Array(out))?;
                    }
                }
                let _cond = {
                    event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if let Some(Value::Array(items)) = event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.PatternDisposition") {
                                    if let Some(val) =
                                        event.get("_ingest._value.PatternDisposition")
                                    {
                                        let converted =
                                            convert_value(val, "long").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.PatternDisposition"
                                                        .into(),
                                                    message,
                                                }
                                            })?;
                                        event
                                            .set("_ingest._value.PatternDisposition", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_threatgraph_indicators_pattern_disposition_to_long",
                                )?;
                                event.remove("_ingest._value.PatternDisposition");
                                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("crowdstrike.event.ThreatgraphIndicators", Value::Array(out))?;
                    }
                }
                if event.has("crowdstrike.event.CompositeId") {
                    event.rename("crowdstrike.event.CompositeId", "event.id")?;
                }
                if event.has("crowdstrike.event.FalconHostLink") {
                    event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if let Some(Value::Array(items)) = event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            event.append_unique(
                                "threat.indicator.id",
                                json!(
                                    event
                                        .get("_ingest._value.IndicatorId")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("crowdstrike.event.ThreatgraphIndicators", Value::Array(out))?;
                    }
                }
                let _cond = {
                    event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if let Some(Value::Array(items)) = event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            event.append_unique(
                                "threat.indicator.name",
                                json!(
                                    event
                                        .get("_ingest._value.DisplayName")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("crowdstrike.event.ThreatgraphIndicators", Value::Array(out))?;
                    }
                }
                let _cond = {
                    event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if let Some(Value::Array(items)) = event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            event.append_unique(
                                "threat.indicator.description",
                                json!(
                                    event
                                        .get("_ingest._value.Description")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("crowdstrike.event.ThreatgraphIndicators", Value::Array(out))?;
                    }
                }
                let _cond = {
                    event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .is_some_and(|v| v.is_array())
                        && !(event
                            .get("crowdstrike.event.ThreatgraphIndicators")
                            .is_none_or(|v| match v {
                                serde_json::Value::String(s) => s.is_empty(),
                                serde_json::Value::Array(a) => a.is_empty(),
                                serde_json::Value::Object(o) => o.is_empty(),
                                serde_json::Value::Null => true,
                                _ => false,
                            }))
                };
                if _cond {
                    // Painless script
                    // Source: def indicator = ctx.crowdstrike.event.ThreatgraphIndicators[0];\nif (indicator.HostId != null && indicator.HostId != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.id = indicator.HostId;\n}\nif (indicator.Hostname != null && indicator.Hostname != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.name = indicator.Hostname;\n}\nif (indicator.ProcessId != null && indicator.ProcessId != '') {\n  ctx.process = ctx.process ?: [:];\n  ctx.process.entity_id = indicator.ProcessId;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def indicator = ctx.crowdstrike.event.ThreatgraphIndicators[0];\nif (indicator.HostId != null && indicator.HostId != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.id = indicator.HostId;\n}\nif (indicator.Hostname != null && indicator.Hostname != '') {\n  ctx.host = ctx.host ?: [:];\n  ctx.host.name = indicator.Hostname;\n}\nif (indicator.ProcessId != null && indicator.ProcessId != '') {\n  ctx.process = ctx.process ?: [:];\n  ctx.process.entity_id = indicator.ProcessId;\n}"#
                        ),
                    )?;
                }
                let _cond = {
                    event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if let Some(Value::Array(items)) = event
                        .get("crowdstrike.event.ThreatgraphIndicators")
                        .cloned()
                    {
                        let mut out = Vec::with_capacity(items.len());
                        for item in items {
                            event.set("_ingest._value", item)?;
                            event.append_unique(
                                "related.hosts",
                                json!(
                                    event
                                        .get("_ingest._value.Hostname")
                                        .map_or_else(String::new, painless_to_string)
                                ),
                            )?;
                            out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                        }
                        event.remove("_ingest");
                        event.set("crowdstrike.event.ThreatgraphIndicators", Value::Array(out))?;
                    }
                }
                // End nested pipeline: "automated_lead_summary"
            }

            let _cond =
                { event.get_str("crowdstrike.metadata.eventType") == Some("CustomerIOCEvent") };
            if _cond {
                // Begin nested pipeline: "customer_ioc_event"
                event.set("event.kind", json!("enrichment"))?;
                event.append("event.category", json!("threat"))?;
                event.append("event.type", json!("indicator"))?;
                if let Some(v) = event
                    .get("crowdstrike.event.ComputerName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.hostname", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.ComputerName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.DeviceId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.id", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.FileName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.name", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.FilePath")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.path", v)?;
                }
                let _cond = { event.get_str("crowdstrike.event.IPv4") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("crowdstrike.event.IPv4") {
                            if let Some(val) = event.get("crowdstrike.event.IPv4") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "crowdstrike.event.IPv4".into(),
                                        message,
                                    }
                                })?;
                                event.set("threat.indicator.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_IPv4_to_ip")?;
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
                let _cond = { event.get_str("crowdstrike.event.IPv6") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("crowdstrike.event.IPv6") {
                            if let Some(val) = event.get("crowdstrike.event.IPv6") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "crowdstrike.event.IPv6".into(),
                                        message,
                                    }
                                })?;
                                event.set("threat.indicator.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_IPv6_to_ip")?;
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
                if event.has_value("threat.indicator.ip") {
                    if let Some(ip_str) = event.get_string("threat.indicator.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("threat.indicator.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("threat.indicator.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("threat.indicator.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("threat.indicator.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("threat.indicator.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("threat.indicator.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("threat.indicator.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("threat.indicator.geo.location", v.clone())?;
                            }
                        }
                    }
                }
                if event.has_value("threat.indicator.ip") {
                    if let Some(ip_str) = event.get_string("threat.indicator.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("threat.indicator.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("threat.indicator.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
                if event.has("threat.indicator.as.asn") {
                    event.rename("threat.indicator.as.asn", "threat.indicator.as.number")?;
                }
                if event.has("threat.indicator.as.organization_name") {
                    event.rename(
                        "threat.indicator.as.organization_name",
                        "threat.indicator.as.organization.name",
                    )?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.ParentProcessId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.parent.entity_id", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.ProcessId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.entity_id", v)?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ProcessStartTime")
                        && event.get_str("crowdstrike.event.ProcessStartTime") != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("crowdstrike.event.ProcessStartTime")
                        {
                            if let Some(parsed) = parse_date_out(&date_str, &["UNIX"], None, None) {
                                event.set("process.start", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_ProcessStartTime")?;
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
                    .get("crowdstrike.event.SHA256String")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.name", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.MD5String")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.name", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.DomainName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.name", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.IPv4")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.name", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.IPv6")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.name", v)?;
                }
                let _cond = { event.has_value("crowdstrike.event.IPv4") };
                if _cond {
                    event.set("threat.indicator.type", json!("ipv4-addr"))?;
                }
                let _cond = { event.has_value("crowdstrike.event.IPv6") };
                if _cond {
                    event.set("threat.indicator.type", json!("ipv6-addr"))?;
                }
                let _cond = { event.has_value("crowdstrike.event.DomainName") };
                if _cond {
                    event.set("threat.indicator.type", json!("domain-name"))?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.SHA256String")
                        || event.has_value("crowdstrike.event.MD5String")
                };
                if _cond {
                    event.set("threat.indicator.type", json!("file"))?;
                }
                let _cond = {
                    event.has_value("threat.indicator.ip")
                        && event.get_str("threat.indicator.ip") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("threat.indicator.ip")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.SHA256String")
                        && event.get_str("crowdstrike.event.SHA256String") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("crowdstrike.event.SHA256String")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.MD5String")
                        && event.get_str("crowdstrike.event.MD5String") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("crowdstrike.event.MD5String")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.DomainName")
                        && event.get_str("crowdstrike.event.DomainName") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("crowdstrike.event.DomainName")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                event.remove("crowdstrike.event.ComputerName");
                event.remove("crowdstrike.event.DeviceId");
                event.remove("crowdstrike.event.FileName");
                event.remove("crowdstrike.event.FilePath");
                event.remove("crowdstrike.event.ParentProcessId");
                event.remove("crowdstrike.event.ProcessId");
                event.remove("crowdstrike.event.ProcessStartTime");
                let _cond = { event.has_value("error.message") };
                if _cond {
                    event.set("event.kind", json!("pipeline_error"))?;
                }
                let _cond = { event.has_value("error.message") };
                if _cond {
                    event.append_unique("tags", json!("preserve_original_event"))?;
                }
                // End nested pipeline: "customer_ioc_event"
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType")
                    == Some("DataProtectionDetectionSummaryEvent")
            };
            if _cond {
                // Begin nested pipeline: "data_protection_detection_summary"
                event.set("event.kind", json!("alert"))?;
                event.append("event.category", json!("malware"))?;
                event.append("event.type", json!("info"))?;
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.event.DataVolume") {
                        if let Some(val) = event.get("crowdstrike.event.DataVolume") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.DataVolume".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.DataVolume", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_DataVolume_to_long",
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
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.event.ContentPatterns.ConfidenceLevel") {
                        if let Some(val) =
                            event.get("crowdstrike.event.ContentPatterns.ConfidenceLevel")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.ContentPatterns.ConfidenceLevel"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "crowdstrike.event.ContentPatterns.ConfidenceLevel",
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
                        "convert_ContentPatterns_ConfidenceLevel_to_long",
                    )?;
                    if event
                        .remove("crowdstrike.event.ContentPatterns.ConfidenceLevel")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "crowdstrike.event.ContentPatterns.ConfidenceLevel".into(),
                        });
                    }
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
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.event.ContentPatterns.MatchCount") {
                        if let Some(val) = event.get("crowdstrike.event.ContentPatterns.MatchCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.ContentPatterns.MatchCount".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.ContentPatterns.MatchCount", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_ContentPatterns_MatchCount_to_long",
                    )?;
                    if event
                        .remove("crowdstrike.event.ContentPatterns.MatchCount")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "crowdstrike.event.ContentPatterns.MatchCount".into(),
                        });
                    }
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
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.event.FilesEgressedCount") {
                        if let Some(val) = event.get("crowdstrike.event.FilesEgressedCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.FilesEgressedCount".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.FilesEgressedCount", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_FilesEgressedCount_to_long",
                    )?;
                    if event
                        .remove("crowdstrike.event.FilesEgressedCount")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "crowdstrike.event.FilesEgressedCount".into(),
                        });
                    }
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
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.event.UserNotified") {
                        if let Some(val) = event.get("crowdstrike.event.UserNotified") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.UserNotified".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.UserNotified", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_UserNotified_to_boolean",
                    )?;
                    if event.remove("crowdstrike.event.UserNotified").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "crowdstrike.event.UserNotified".into(),
                        });
                    }
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
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.event.UserMapped") {
                        if let Some(val) = event.get("crowdstrike.event.UserMapped") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.UserMapped".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.UserMapped", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_UserMapped_to_boolean",
                    )?;
                    if event.remove("crowdstrike.event.UserMapped").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "crowdstrike.event.UserMapped".into(),
                        });
                    }
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
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.event.IsClipboard") {
                        if let Some(val) = event.get("crowdstrike.event.IsClipboard") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.IsClipboard".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.IsClipboard", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_IsClipboard_to_boolean",
                    )?;
                    if event.remove("crowdstrike.event.IsClipboard").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "crowdstrike.event.IsClipboard".into(),
                        });
                    }
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
                let _cond = { event.has_value("crowdstrike.event.EventTimestamp") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("crowdstrike.event.EventTimestamp")
                        {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                            {
                                event.set("crowdstrike.event.EventTimestamp", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_EventTimestamp")?;
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
                let _cond = { event.has_value("crowdstrike.event.SessionStartTimestamp") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("crowdstrike.event.SessionStartTimestamp")
                        {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                            {
                                event.set("event.start", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_SessionStartTimestamp",
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
                let _cond = { event.has_value("crowdstrike.event.SessionEndTimestamp") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("crowdstrike.event.SessionEndTimestamp")
                        {
                            if let Some(parsed) =
                                parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                            {
                                event.set("event.end", parsed)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "date_SessionEndTimestamp",
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
                let _cond = { event.has_value("event.start") && event.has_value("event.end") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        // Painless script
                        // Source: Instant event_start = ZonedDateTime.parse(ctx.event.start).toInstant();\nInstant event_end = ZonedDateTime.parse(ctx.event.end).toInstant();\nctx.event['duration'] = ChronoUnit.NANOS.between(event_start, event_end);\n
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"Instant event_start = ZonedDateTime.parse(ctx.event.start).toInstant();\nInstant event_end = ZonedDateTime.parse(ctx.event.end).toInstant();\nctx.event['duration'] = ChronoUnit.NANOS.between(event_start, event_end);\n"#
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "script")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "script_to_set_event_duration",
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
                    .get("crowdstrike.event.Description")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("message", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.EgressEventId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.id", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.Name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.FalconHostLink")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.reference", v)?;
                }
                let _cond =
                    { event.get_str("crowdstrike.event.ResponseAction") == Some("allowed") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond =
                    { event.get_str("crowdstrike.event.ResponseAction") == Some("blocked") };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                if !event.has("event.outcome") {
                    event.set("event.outcome", json!("unknown"))?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.ContentSha")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.hash.sha256", v)?;
                }
                let _cond = { event.has_value("file.hash.sha256") };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.sha256")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.Filename")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.name", v)?;
                }
                let _cond = { event.has_value("crowdstrike.event.Filename") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        // Painless script
                        // Source: def idx = ctx.crowdstrike.event.Filename.lastIndexOf('.');\nif (idx != -1) {\n  ctx.file = ctx.file ?: [:];\n  ctx.file.extension = ctx.crowdstrike.event.Filename.substring(idx + 1).toLowerCase();\n}
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"def idx = ctx.crowdstrike.event.Filename.lastIndexOf('.');\nif (idx != -1) {\n  ctx.file = ctx.file ?: [:];\n  ctx.file.extension = ctx.crowdstrike.event.Filename.substring(idx + 1).toLowerCase();\n}"#
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "script")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "extract_file_extension_from_filename",
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
                    .get("crowdstrike.event.DataVolume")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.size", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.Hostname")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
                if event.has_value("crowdstrike.event.Platform") {
                    if let Some(s) = event.get_string("crowdstrike.event.Platform") {
                        let lowered = s.to_lowercase();
                        event.set("host.os.platform", lowered)?;
                    }
                }
                if let Some(v) = event
                    .get("crowdstrike.event.Policy.ID")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.id", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.Policy.Name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.name", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.UserSid")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.UserName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
                event.remove("crowdstrike.event.ContentSha");
                event.remove("crowdstrike.event.DataVolume");
                event.remove("crowdstrike.event.Description");
                event.remove("crowdstrike.event.EgressEventId");
                event.remove("crowdstrike.event.FalconHostLink");
                event.remove("crowdstrike.event.Filename");
                event.remove("crowdstrike.event.Hostname");
                event.remove("crowdstrike.event.Name");
                event.remove("crowdstrike.event.Platform");
                event.remove("crowdstrike.event.Policy");
                event.remove("crowdstrike.event.SessionStartTimestamp");
                event.remove("crowdstrike.event.SessionEndTimestamp");
                event.remove("crowdstrike.event.UserSid");
                let _cond = { event.has_value("error.message") };
                if _cond {
                    event.set("event.kind", json!("pipeline_error"))?;
                }
                let _cond = { event.has_value("error.message") };
                if _cond {
                    event.append_unique("tags", json!("preserve_original_event"))?;
                }
                // End nested pipeline: "data_protection_detection_summary"
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType") == Some("DetectionSummaryEvent")
            };
            if _cond {
                // Begin nested pipeline: "detection_summary"
                event.set("event.kind", json!("alert"))?;
                event.append("event.category", json!("malware"))?;
                event.append("event.type", json!("info"))?;
                if event.has("crowdstrike.event.UserName") {
                    event.rename("crowdstrike.event.UserName", "user.name")?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ProcessStartTime")
                        && event
                            .get_as_string("crowdstrike.event.ProcessStartTime")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ProcessStartTime")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("process.start", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ProcessStartTime")
                        && event
                            .get_as_string("crowdstrike.event.ProcessStartTime")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ProcessStartTime")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("process.start", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ProcessEndTime")
                        && event
                            .get_as_string("crowdstrike.event.ProcessEndTime")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.ProcessEndTime")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("process.end", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ProcessEndTime")
                        && event
                            .get_as_string("crowdstrike.event.ProcessEndTime")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.ProcessEndTime")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("process.end", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.LocalIP")
                        && event.get_str("crowdstrike.event.LocalIP") != Some("")
                };
                if _cond {
                    if event.has("crowdstrike.event.LocalIP") {
                        event.rename("crowdstrike.event.LocalIP", "source.ip")?;
                    }
                }
                if event.has("crowdstrike.event.ProcessId") {
                    event.rename("crowdstrike.event.ProcessId", "process.pid")?;
                }
                if event.has_value("crowdstrike.event.HostGroups") {
                    if let Some(s) = event.get_string("crowdstrike.event.HostGroups") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("crowdstrike.event.HostGroups", Value::Array(parts))?;
                    }
                }
                if event.has("crowdstrike.event.ParentProcessId") {
                    event.rename("crowdstrike.event.ParentProcessId", "process.parent.pid")?;
                }
                let _cond = { !event.has_value("process.parent.executable") };
                if _cond {
                    if event.has("crowdstrike.event.ParentImageFileName") {
                        event.rename(
                            "crowdstrike.event.ParentImageFileName",
                            "process.parent.executable",
                        )?;
                    }
                }
                if event.has("crowdstrike.event.PatternDispositionDescription") {
                    event.rename(
                        "crowdstrike.event.PatternDispositionDescription",
                        "event.action",
                    )?;
                }
                if event.has("crowdstrike.event.FalconHostLink") {
                    event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
                }
                if event.has("crowdstrike.event.DetectDescription") {
                    event.rename("crowdstrike.event.DetectDescription", "message")?;
                }
                let _cond = { event.has_value("message") };
                if _cond {
                    if let Some(v) = event.get("message").cloned() {
                        event.set("rule.description", v)?;
                    }
                }
                if let Some(v) = event
                    .get("crowdstrike.event.FileName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.name", v)?;
                }
                if event.has("crowdstrike.event.MachineDomain") {
                    event.rename("crowdstrike.event.MachineDomain", "host.domain")?;
                }
                if event.has("crowdstrike.event.ComputerName") {
                    event.rename("crowdstrike.event.ComputerName", "host.name")?;
                }
                if event.has("crowdstrike.event.SHA256String") {
                    event.rename("crowdstrike.event.SHA256String", "file.hash.sha256")?;
                }
                if event.has("crowdstrike.event.MD5String") {
                    event.rename("crowdstrike.event.MD5String", "file.hash.md5")?;
                }
                if event.has("crowdstrike.event.SHA1String") {
                    event.rename("crowdstrike.event.SHA1String", "file.hash.sha1")?;
                }
                let _cond = {
                    event.has_value("file.hash.sha1") && event.get_str("file.hash.sha1") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.sha1")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("file.hash.sha256")
                        && event.get_str("file.hash.sha256") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.sha256")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("file.hash.md5") && event.get_str("file.hash.md5") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.md5")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                if event.has("crowdstrike.event.FileName") {
                    event.rename("crowdstrike.event.FileName", "file.name")?;
                }
                if event.has("crowdstrike.event.FilePath") {
                    event.rename("crowdstrike.event.FilePath", "file.path")?;
                }
                if event.has("crowdstrike.event.DetectName") {
                    event.rename("crowdstrike.event.DetectName", "rule.name")?;
                }
                if event.has("crowdstrike.event.DetectId") {
                    event.rename("crowdstrike.event.DetectId", "rule.id")?;
                }
                let _cond = { event.has_value("crowdstrike.event.MacAddress") };
                if _cond {
                    if event.has("crowdstrike.event.MacAddress") {
                        event.rename("crowdstrike.event.MacAddress", "host.mac")?;
                    }
                }
                let _cond = { event.has_value("host.mac") };
                if _cond {
                    if event.has_value("host.mac") {
                        if let Some(s) = event.get_string("host.mac") {
                            let uppered = s.to_uppercase();
                            event.set("host.mac", uppered)?;
                        }
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.Technique") };
                if _cond {
                    event.append(
                        "threat.technique.name",
                        json!(
                            event
                                .get("crowdstrike.event.Technique")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.TechniqueId") };
                if _cond {
                    event.append(
                        "threat.technique.id",
                        json!(
                            event
                                .get("crowdstrike.event.TechniqueId")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.Tactic") };
                if _cond {
                    event.append(
                        "threat.tactic.name",
                        json!(
                            event
                                .get("crowdstrike.event.Tactic")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.TacticId") };
                if _cond {
                    event.append(
                        "threat.tactic.id",
                        json!(
                            event
                                .get("crowdstrike.event.TacticId")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "detection_summary"
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType") == Some("EppDetectionSummaryEvent")
            };
            if _cond {
                // Begin nested pipeline: "epp_detection_summary"
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename(
                        "crowdstrike.event.GrandParentCommandLine",
                        "crowdstrike.event.GrandparentCommandLine",
                    )?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename(
                        "crowdstrike.event.GrandParentImageFileName",
                        "crowdstrike.event.GrandparentImageFileName",
                    )?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename(
                        "crowdstrike.event.GrandParentImageFilePath",
                        "crowdstrike.event.GrandparentImageFilePath",
                    )?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename(
                        "crowdstrike.event.Hostname",
                        "crowdstrike.event.ComputerName",
                    )?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename(
                        "crowdstrike.event.LogonDomain",
                        "crowdstrike.event.MachineDomain",
                    )?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("crowdstrike.event.AgentId", "crowdstrike.event.SensorId")?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("crowdstrike.event.Name", "crowdstrike.event.DetectName")?;
                    Ok(())
                })();
                let _cond = {
                    event.has_value("crowdstrike.event.LocalIPv6")
                        && event.get_str("crowdstrike.event.LocalIPv6") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(val) = event.get("crowdstrike.event.LocalIPv6") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.LocalIPv6".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.LocalIPv6", converted)?;
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_crowdstrike_LocalIPv6_ip",
                        )?;
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            if event.remove("crowdstrike.event.LocalIPv6").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "crowdstrike.event.LocalIPv6".into(),
                                });
                            }
                            Ok(())
                        })();
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
                                    .get("_ingest.pipeline")
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
                    event
                        .get("crowdstrike.event.FilesAccessed")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(Value::Array(items)) =
                            event.get("crowdstrike.event.FilesAccessed").cloned()
                        {
                            let mut out = Vec::with_capacity(items.len());
                            for item in items {
                                event.set("_ingest._value", item)?;
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.Timestamp")
                                    {
                                        if let Some(parsed) =
                                            parse_date_out(&date_str, &["UNIX"], None, None)
                                        {
                                            event.set("_ingest._value.Timestamp", parsed)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "date")?;
                                    event.set(
                                        "_ingest.on_failure_processor_tag",
                                        "convert_crowdstrike_filesaccessed_timestamp",
                                    )?;
                                    // ignore_failure: true
                                    let _ = (|| -> Result<()> {
                                        if event.remove("_ingest._value.Timestamp").is_none() {
                                            return Err(TransformError::FieldNotFound {
                                                path: "_ingest._value.Timestamp".into(),
                                            });
                                        }
                                        Ok(())
                                    })();
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                            }
                            event.remove("_ingest");
                            event.set("crowdstrike.event.FilesAccessed", Value::Array(out))?;
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event
                        .get("crowdstrike.event.FilesWritten")
                        .is_some_and(|v| v.is_array())
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(Value::Array(items)) =
                            event.get("crowdstrike.event.FilesWritten").cloned()
                        {
                            let mut out = Vec::with_capacity(items.len());
                            for item in items {
                                event.set("_ingest._value", item)?;
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.Timestamp")
                                    {
                                        if let Some(parsed) =
                                            parse_date_out(&date_str, &["UNIX"], None, None)
                                        {
                                            event.set("_ingest._value.Timestamp", parsed)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "date")?;
                                    event.set(
                                        "_ingest.on_failure_processor_tag",
                                        "convert_crowdstrike_fileswritten_timestamp",
                                    )?;
                                    // ignore_failure: true
                                    let _ = (|| -> Result<()> {
                                        if event.remove("_ingest._value.Timestamp").is_none() {
                                            return Err(TransformError::FieldNotFound {
                                                path: "_ingest._value.Timestamp".into(),
                                            });
                                        }
                                        Ok(())
                                    })();
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                            }
                            event.remove("_ingest");
                            event.set("crowdstrike.event.FilesWritten", Value::Array(out))?;
                        }
                        Ok(())
                    })();
                }
                // Begin nested pipeline: "detection_summary"
                event.set("event.kind", json!("alert"))?;
                event.append("event.category", json!("malware"))?;
                event.append("event.type", json!("info"))?;
                if event.has("crowdstrike.event.UserName") {
                    event.rename("crowdstrike.event.UserName", "user.name")?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ProcessStartTime")
                        && event
                            .get_as_string("crowdstrike.event.ProcessStartTime")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ProcessStartTime")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("process.start", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ProcessStartTime")
                        && event
                            .get_as_string("crowdstrike.event.ProcessStartTime")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ProcessStartTime")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("process.start", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ProcessEndTime")
                        && event
                            .get_as_string("crowdstrike.event.ProcessEndTime")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.ProcessEndTime")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("process.end", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ProcessEndTime")
                        && event
                            .get_as_string("crowdstrike.event.ProcessEndTime")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.ProcessEndTime")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("process.end", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.LocalIP")
                        && event.get_str("crowdstrike.event.LocalIP") != Some("")
                };
                if _cond {
                    if event.has("crowdstrike.event.LocalIP") {
                        event.rename("crowdstrike.event.LocalIP", "source.ip")?;
                    }
                }
                if event.has("crowdstrike.event.ProcessId") {
                    event.rename("crowdstrike.event.ProcessId", "process.pid")?;
                }
                if event.has_value("crowdstrike.event.HostGroups") {
                    if let Some(s) = event.get_string("crowdstrike.event.HostGroups") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("crowdstrike.event.HostGroups", Value::Array(parts))?;
                    }
                }
                if event.has("crowdstrike.event.ParentProcessId") {
                    event.rename("crowdstrike.event.ParentProcessId", "process.parent.pid")?;
                }
                let _cond = { !event.has_value("process.parent.executable") };
                if _cond {
                    if event.has("crowdstrike.event.ParentImageFileName") {
                        event.rename(
                            "crowdstrike.event.ParentImageFileName",
                            "process.parent.executable",
                        )?;
                    }
                }
                if event.has("crowdstrike.event.PatternDispositionDescription") {
                    event.rename(
                        "crowdstrike.event.PatternDispositionDescription",
                        "event.action",
                    )?;
                }
                if event.has("crowdstrike.event.FalconHostLink") {
                    event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
                }
                if event.has("crowdstrike.event.DetectDescription") {
                    event.rename("crowdstrike.event.DetectDescription", "message")?;
                }
                let _cond = { event.has_value("message") };
                if _cond {
                    if let Some(v) = event.get("message").cloned() {
                        event.set("rule.description", v)?;
                    }
                }
                if let Some(v) = event
                    .get("crowdstrike.event.FileName")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("process.name", v)?;
                }
                if event.has("crowdstrike.event.MachineDomain") {
                    event.rename("crowdstrike.event.MachineDomain", "host.domain")?;
                }
                if event.has("crowdstrike.event.ComputerName") {
                    event.rename("crowdstrike.event.ComputerName", "host.name")?;
                }
                if event.has("crowdstrike.event.SHA256String") {
                    event.rename("crowdstrike.event.SHA256String", "file.hash.sha256")?;
                }
                if event.has("crowdstrike.event.MD5String") {
                    event.rename("crowdstrike.event.MD5String", "file.hash.md5")?;
                }
                if event.has("crowdstrike.event.SHA1String") {
                    event.rename("crowdstrike.event.SHA1String", "file.hash.sha1")?;
                }
                let _cond = {
                    event.has_value("file.hash.sha1") && event.get_str("file.hash.sha1") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.sha1")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("file.hash.sha256")
                        && event.get_str("file.hash.sha256") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.sha256")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("file.hash.md5") && event.get_str("file.hash.md5") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("file.hash.md5")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                if event.has("crowdstrike.event.FileName") {
                    event.rename("crowdstrike.event.FileName", "file.name")?;
                }
                if event.has("crowdstrike.event.FilePath") {
                    event.rename("crowdstrike.event.FilePath", "file.path")?;
                }
                if event.has("crowdstrike.event.DetectName") {
                    event.rename("crowdstrike.event.DetectName", "rule.name")?;
                }
                if event.has("crowdstrike.event.DetectId") {
                    event.rename("crowdstrike.event.DetectId", "rule.id")?;
                }
                let _cond = { event.has_value("crowdstrike.event.MacAddress") };
                if _cond {
                    if event.has("crowdstrike.event.MacAddress") {
                        event.rename("crowdstrike.event.MacAddress", "host.mac")?;
                    }
                }
                let _cond = { event.has_value("host.mac") };
                if _cond {
                    if event.has_value("host.mac") {
                        if let Some(s) = event.get_string("host.mac") {
                            let uppered = s.to_uppercase();
                            event.set("host.mac", uppered)?;
                        }
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.Technique") };
                if _cond {
                    event.append(
                        "threat.technique.name",
                        json!(
                            event
                                .get("crowdstrike.event.Technique")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.TechniqueId") };
                if _cond {
                    event.append(
                        "threat.technique.id",
                        json!(
                            event
                                .get("crowdstrike.event.TechniqueId")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.Tactic") };
                if _cond {
                    event.append(
                        "threat.tactic.name",
                        json!(
                            event
                                .get("crowdstrike.event.Tactic")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.TacticId") };
                if _cond {
                    event.append(
                        "threat.tactic.id",
                        json!(
                            event
                                .get("crowdstrike.event.TacticId")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "detection_summary"
                // End nested pipeline: "epp_detection_summary"
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType")
                    == Some("MobileDetectionSummaryEvent")
            };
            if _cond {
                // Begin nested pipeline: "mobile_detection_summary"
                event.set("event.kind", json!("alert"))?;
                event.append("event.category", json!("malware"))?;
                event.append("event.type", json!("info"))?;
                event.set("event.action", json!("mobile-detection"))?;
                let _cond = { event.has_value("crowdstrike.event.ContextTimeStamp") };
                if _cond {
                    event.remove("event.created");
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ContextTimeStamp")
                        && event
                            .get_as_string("crowdstrike.event.ContextTimeStamp")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ContextTimeStamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("event.created", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ContextTimeStamp")
                        && event
                            .get_as_string("crowdstrike.event.ContextTimeStamp")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ContextTimeStamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("event.created", parsed)?;
                        }
                    }
                }
                if event.has("crowdstrike.event.MobileDetectionId") {
                    event.rename("crowdstrike.event.MobileDetectionId", "event.id")?;
                }
                if event.has_value("event.id") {
                    if let Some(val) = event.get("event.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "event.id".into(),
                                message,
                            }
                        })?;
                        event.set("event.id", converted)?;
                    }
                }
                if event.has("crowdstrike.event.DetectId") {
                    event.rename("crowdstrike.event.DetectId", "rule.id")?;
                }
                if event.has("crowdstrike.event.DetectName") {
                    event.rename("crowdstrike.event.DetectName", "rule.name")?;
                }
                if event.has("crowdstrike.event.DetectDescription") {
                    event.rename("crowdstrike.event.DetectDescription", "rule.description")?;
                }
                let _cond = { event.has_value("crowdstrike.event.Technique") };
                if _cond {
                    event.append(
                        "threat.technique.name",
                        json!(
                            event
                                .get("crowdstrike.event.Technique")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.TechniqueId") };
                if _cond {
                    event.append(
                        "threat.technique.id",
                        json!(
                            event
                                .get("crowdstrike.event.TechniqueId")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.Tactic") };
                if _cond {
                    event.append(
                        "threat.tactic.name",
                        json!(
                            event
                                .get("crowdstrike.event.Tactic")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.TacticId") };
                if _cond {
                    event.append(
                        "threat.tactic.id",
                        json!(
                            event
                                .get("crowdstrike.event.TacticId")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                if event.has("crowdstrike.event.ComputerName") {
                    event.rename("crowdstrike.event.ComputerName", "host.name")?;
                }
                if event.has("crowdstrike.event.UserName") {
                    event.rename("crowdstrike.event.UserName", "user.name")?;
                }
                if event.has("crowdstrike.event.FalconHostLink") {
                    event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
                }
                if event.has("crowdstrike.event.SensorId") {
                    event.rename("crowdstrike.event.SensorId", "device.id")?;
                }
                if event.has("crowdstrike.event.ProcessId") {
                    event.rename("crowdstrike.event.ProcessId", "process.pid")?;
                }
                // End nested pipeline: "mobile_detection_summary"
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType")
                    == Some("OverwatchGenericDetectionSummaryEvent")
            };
            if _cond {
                // Begin nested pipeline: "overwatch_generic_detection_summary"
                event.set("event.kind", json!("alert"))?;
                event.append("event.category", json!("malware"))?;
                event.append("event.type", json!("info"))?;
                if event.has("crowdstrike.event.CompositeId") {
                    event.rename("crowdstrike.event.CompositeId", "event.id")?;
                }
                if event.has("crowdstrike.event.FalconHostLink") {
                    event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
                }
                if event.has("crowdstrike.event.Description") {
                    event.rename("crowdstrike.event.Description", "message")?;
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("crowdstrike.event.Severity") {
                        if let Some(val) = event.get("crowdstrike.event.Severity") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.Severity".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.Severity", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_severity_to_long",
                    )?;
                    event.remove("crowdstrike.event.Severity");
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
                // End nested pipeline: "overwatch_generic_detection_summary"
            }

            let _cond =
                { event.get_str("crowdstrike.metadata.eventType") == Some("IncidentSummaryEvent") };
            if _cond {
                // Begin nested pipeline: "incident_summary"
                event.set("event.kind", json!("alert"))?;
                event.append("event.category", json!("malware"))?;
                event.append("event.type", json!("info"))?;
                event.append("event.action", json!("incident"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("crowdstrike.event.UserId") {
                        if let Some(input) = event.get_string("crowdstrike.event.UserId") {
                            // Grok pattern: %{USERNAME:user.name}@%{HOSTNAME:user.domain}
                            if !cached_grok!("%{USERNAME:user.name}@%{HOSTNAME:user.domain}")
                                .extract_into(&input, event)?
                            {
                                // Grok pattern: %{GREEDYDATA:user.name}
                                if !cached_grok!("%{GREEDYDATA:user.name}")
                                    .extract_into(&input, event)?
                                {}
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("crowdstrike.event.UserId")
                        && event
                            .get_str("crowdstrike.event.UserId")
                            .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                            .is_some_and(|i| i.is_some_and(|i| i > 0))
                };
                if _cond {
                    if let Some(v) = event.get("crowdstrike.event.UserId").cloned() {
                        event.set("user.email", v)?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.IncidentStartTime")
                        && event
                            .get_as_string("crowdstrike.event.IncidentStartTime")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.IncidentStartTime")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("event.start", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.IncidentStartTime")
                        && event
                            .get_as_string("crowdstrike.event.IncidentStartTime")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.IncidentStartTime")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("event.start", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.IncidentEndTime")
                        && event
                            .get_as_string("crowdstrike.event.IncidentEndTime")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.IncidentEndTime")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("event.end", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.IncidentEndTime")
                        && event
                            .get_as_string("crowdstrike.event.IncidentEndTime")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.IncidentEndTime")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("event.end", parsed)?;
                        }
                    }
                }
                if event.has("crowdstrike.event.FalconHostLink") {
                    event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
                }
                if event.has("crowdstrike.event.HostID") {
                    event.rename("crowdstrike.event.HostID", "host.id")?;
                }
                if event.has("crowdstrike.event.IncidentID") {
                    event.rename("crowdstrike.event.IncidentID", "event.id")?;
                }
                let _cond = { event.has_value("crowdstrike.event.FineScore") };
                if _cond {
                    event.set(
                        "message",
                        json!(format!(
                            "Incident score {}",
                            event
                                .get("crowdstrike.event.FineScore")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                }
                // End nested pipeline: "incident_summary"
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType") == Some("XdrDetectionSummaryEvent")
            };
            if _cond {
                // Begin nested pipeline: "xdr_detection_summary"
                event.set("event.kind", json!("alert"))?;
                event.append("event.category", json!("malware"))?;
                event.append("event.type", json!("info"))?;
                event.set("event.action", json!("xdr-detection"))?;
                let _cond = { event.has_value("crowdstrike.event.Author") };
                if _cond {
                    event.append(
                        "rule.author",
                        json!(
                            event
                                .get("crowdstrike.event.Author")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                if event.has("crowdstrike.event.Name") {
                    event.rename("crowdstrike.event.Name", "rule.name")?;
                }
                if event.has("crowdstrike.event.DetectId") {
                    event.rename("crowdstrike.event.DetectId", "rule.id")?;
                }
                if event.has_value("crowdstrike.event.PatternId") {
                    if let Some(val) = event.get("crowdstrike.event.PatternId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "crowdstrike.event.PatternId".into(),
                                message,
                            }
                        })?;
                        event.set("rule.uuid", converted)?;
                    }
                }
                if event.has("crowdstrike.event.Description") {
                    event.rename("crowdstrike.event.Description", "message")?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.DataDomains")
                        && event
                            .get("crowdstrike.event.DataDomains")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            })
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.DataDomains") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("crowdstrike.event.DataDomains", Value::Array(parts))?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EmailAddresses")
                        && event
                            .get("crowdstrike.event.EmailAddresses")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            })
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.EmailAddresses") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("crowdstrike.event.EmailAddresses", Value::Array(parts))?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.IPV4Addresses")
                        && event
                            .get("crowdstrike.event.IPV4Addresses")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            })
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.IPV4Addresses") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("related.ip", Value::Array(parts))?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.IPV4Addresses")
                        && !(event.get("crowdstrike.event.IPV4Addresses").is_some_and(
                            |v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            },
                        ))
                };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("crowdstrike.event.IPV4Addresses")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.IPV6Addresses")
                        && event
                            .get("crowdstrike.event.IPV6Addresses")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            })
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.IPV6Addresses") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("related.ip", Value::Array(parts))?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.IPV6Addresses")
                        && !(event.get("crowdstrike.event.IPV6Addresses").is_some_and(
                            |v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            },
                        ))
                };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("crowdstrike.event.IPV6Addresses")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.HostNames")
                        && event
                            .get("crowdstrike.event.HostNames")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            })
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.HostNames") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("related.hosts", Value::Array(parts))?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.HostNames")
                        && !(event
                            .get("crowdstrike.event.HostNames")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            }))
                };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("crowdstrike.event.HostNames")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.DomainNames")
                        && event
                            .get("crowdstrike.event.DomainNames")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            })
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.DomainNames") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("related.hosts", Value::Array(parts))?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.DomainNames")
                        && !(event
                            .get("crowdstrike.event.DomainNames")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            }))
                };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("crowdstrike.event.DomainNames")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.SHA256Hashes")
                        && event
                            .get("crowdstrike.event.SHA256Hashes")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            })
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.SHA256Hashes") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("related.hash", Value::Array(parts))?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.SHA256Hashes")
                        && !(event
                            .get("crowdstrike.event.SHA256Hashes")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            }))
                };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("crowdstrike.event.SHA256Hashes")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.MD5Hashes")
                        && event
                            .get("crowdstrike.event.MD5Hashes")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            })
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.MD5Hashes") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("related.hash", Value::Array(parts))?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.MD5Hashes")
                        && !(event
                            .get("crowdstrike.event.MD5Hashes")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            }))
                };
                if _cond {
                    event.append_unique(
                        "related.hash",
                        json!(
                            event
                                .get("crowdstrike.event.MD5Hashes")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.Users")
                        && event
                            .get("crowdstrike.event.Users")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            })
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.Users") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("related.user", Value::Array(parts))?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.Users")
                        && !(event
                            .get("crowdstrike.event.Users")
                            .is_some_and(|v| match v {
                                serde_json::Value::Array(a) => {
                                    a.iter().any(|x| x.as_str() == Some(","))
                                }
                                serde_json::Value::String(s) => s.contains(","),
                                _ => false,
                            }))
                };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("crowdstrike.event.Users")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("message") };
                if _cond {
                    if let Some(v) = event.get("message").cloned() {
                        event.set("rule.description", v)?;
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.StartTimeEpoch") };
                if _cond {
                    if event.has_value("crowdstrike.event.StartTimeEpoch") {
                        if let Some(val) = event.get("crowdstrike.event.StartTimeEpoch") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.StartTimeEpoch".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.StartTimeEpoch", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.StartTimeEpoch")
                        && event
                            .get_as_string("crowdstrike.event.StartTimeEpoch")
                            .is_some_and(|s| s.len() > 18)
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.StartTimeEpoch") {
                        let re = cached_regex!("\\d{6}$");
                        let replaced = re.replace_all(&s, "").into_owned();
                        event.set("crowdstrike.event.StartTimeEpoch", replaced)?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.StartTimeEpoch")
                        && event
                            .get_as_string("crowdstrike.event.StartTimeEpoch")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTimeEpoch")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("event.start", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.StartTimeEpoch")
                        && event
                            .get_as_string("crowdstrike.event.StartTimeEpoch")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTimeEpoch")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("event.start", parsed)?;
                        }
                    }
                }
                let _cond = { event.has_value("event.start") };
                if _cond {
                    if let Some(v) = event.get("event.start").cloned() {
                        event.set("@timestamp", v)?;
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.EndTimeEpoch") };
                if _cond {
                    if event.has_value("crowdstrike.event.EndTimeEpoch") {
                        if let Some(val) = event.get("crowdstrike.event.EndTimeEpoch") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.EndTimeEpoch".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.EndTimeEpoch", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EndTimeEpoch")
                        && event
                            .get_as_string("crowdstrike.event.EndTimeEpoch")
                            .is_some_and(|s| s.len() > 18)
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.EndTimeEpoch") {
                        let re = cached_regex!("\\d{6}$");
                        let replaced = re.replace_all(&s, "").into_owned();
                        event.set("crowdstrike.event.EndTimeEpoch", replaced)?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EndTimeEpoch")
                        && event
                            .get_as_string("crowdstrike.event.EndTimeEpoch")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTimeEpoch") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("event.end", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EndTimeEpoch")
                        && event
                            .get_as_string("crowdstrike.event.EndTimeEpoch")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTimeEpoch") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("event.end", parsed)?;
                        }
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.Techniques") };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.Techniques") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("threat.technique.name", Value::Array(parts))?;
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.TechniqueIds") };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.TechniqueIds") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("threat.technique.id", Value::Array(parts))?;
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.Tactics") };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.Tactics") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("threat.tactic.name", Value::Array(parts))?;
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.TacticIds") };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.TacticIds") {
                        let parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                        event.set("threat.tactic.id", Value::Array(parts))?;
                    }
                }
                // End nested pipeline: "xdr_detection_summary"
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType") == Some("IdpDetectionSummaryEvent")
            };
            if _cond {
                // Begin nested pipeline: "ipd_detection_summary"
                event.set("event.kind", json!("alert"))?;
                event.append("event.category", json!("malware"))?;
                event.append("event.type", json!("info"))?;
                event.set("event.action", json!("ipd-detection"))?;
                let _cond = { event.get_bool("crowdstrike.event.AttemptOutcome") == Some(true) };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_bool("crowdstrike.event.AttemptOutcome") == Some(false) };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                if event.has("crowdstrike.event.DetectDescription") {
                    event.rename("crowdstrike.event.DetectDescription", "message")?;
                }
                if event.has("crowdstrike.event.LocationCountryCode") {
                    event.rename(
                        "crowdstrike.event.LocationCountryCode",
                        "host.geo.country_iso_code",
                    )?;
                }
                if event.has_value("crowdstrike.event.PatternId") {
                    if let Some(val) = event.get("crowdstrike.event.PatternId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "crowdstrike.event.PatternId".into(),
                                message,
                            }
                        })?;
                        event.set("rule.uuid", converted)?;
                    }
                }
                if event.has("crowdstrike.event.SourceAccountDomain") {
                    event.rename("crowdstrike.event.SourceAccountDomain", "user.domain")?;
                }
                if event.has("crowdstrike.event.SourceAccountName") {
                    event.rename("crowdstrike.event.SourceAccountName", "user.name")?;
                }
                if event.has("crowdstrike.event.SourceAccountObjectSid") {
                    event.rename("crowdstrike.event.SourceAccountObjectSid", "user.id")?;
                }
                if event.has("crowdstrike.event.SourceEndpointHostName") {
                    event.rename("crowdstrike.event.SourceEndpointHostName", "host.name")?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.SourceEndpointIpAddress")
                        && event.get_str("crowdstrike.event.SourceEndpointIpAddress") != Some("")
                };
                if _cond {
                    event.append(
                        "host.ip",
                        json!(
                            event
                                .get("crowdstrike.event.SourceEndpointIpAddress")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.SourceEndpointIpAddress") };
                if _cond {
                    if event
                        .remove("crowdstrike.event.SourceEndpointIpAddress")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "crowdstrike.event.SourceEndpointIpAddress".into(),
                        });
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.Technique") };
                if _cond {
                    event.append(
                        "threat.technique.name",
                        json!(
                            event
                                .get("crowdstrike.event.Technique")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.TechniqueId") };
                if _cond {
                    event.append(
                        "threat.technique.id",
                        json!(
                            event
                                .get("crowdstrike.event.TechniqueId")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.Tactic") };
                if _cond {
                    event.append(
                        "threat.tactic.name",
                        json!(
                            event
                                .get("crowdstrike.event.Tactic")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.TacticId") };
                if _cond {
                    event.append(
                        "threat.tactic.id",
                        json!(
                            event
                                .get("crowdstrike.event.TacticId")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("message") };
                if _cond {
                    if let Some(v) = event.get("message").cloned() {
                        event.set("rule.description", v)?;
                    }
                }
                if event.has("crowdstrike.event.DetectName") {
                    event.rename("crowdstrike.event.DetectName", "rule.name")?;
                }
                if event.has("crowdstrike.event.DetectId") {
                    event.rename("crowdstrike.event.DetectId", "rule.id")?;
                }
                if event.has("crowdstrike.event.FalconHostLink") {
                    event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
                }
                let _cond = { event.has_value("crowdstrike.event.ContextTimeStamp") };
                if _cond {
                    event.remove("event.created");
                }
                let _cond = { event.has_value("crowdstrike.event.ContextTimeStamp") };
                if _cond {
                    if event.has_value("crowdstrike.event.ContextTimeStamp") {
                        if let Some(val) = event.get("crowdstrike.event.ContextTimeStamp") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.ContextTimeStamp".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.ContextTimeStamp", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ContextTimeStamp")
                        && event
                            .get_as_string("crowdstrike.event.ContextTimeStamp")
                            .is_some_and(|s| s.len() > 18)
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.ContextTimeStamp") {
                        let re = cached_regex!("\\d{6}$");
                        let replaced = re.replace_all(&s, "").into_owned();
                        event.set("crowdstrike.event.ContextTimeStamp", replaced)?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ContextTimeStamp")
                        && event
                            .get_as_string("crowdstrike.event.ContextTimeStamp")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ContextTimeStamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("event.created", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ContextTimeStamp")
                        && event
                            .get_as_string("crowdstrike.event.ContextTimeStamp")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ContextTimeStamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("event.created", parsed)?;
                        }
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.AccountCreationTimeStamp") };
                if _cond {
                    if event.has_value("crowdstrike.event.AccountCreationTimeStamp") {
                        if let Some(val) = event.get("crowdstrike.event.AccountCreationTimeStamp") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.AccountCreationTimeStamp".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.AccountCreationTimeStamp", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.AccountCreationTimeStamp")
                        && event
                            .get_as_string("crowdstrike.event.AccountCreationTimeStamp")
                            .is_some_and(|s| s.len() > 18)
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.AccountCreationTimeStamp")
                    {
                        let re = cached_regex!("\\d{6}$");
                        let replaced = re.replace_all(&s, "").into_owned();
                        event.set("crowdstrike.event.AccountCreationTimeStamp", replaced)?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.AccountCreationTimeStamp")
                        && event
                            .get_as_string("crowdstrike.event.AccountCreationTimeStamp")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.AccountCreationTimeStamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("crowdstrike.event.AccountCreationTimeStamp", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.AccountCreationTimeStamp")
                        && event
                            .get_as_string("crowdstrike.event.AccountCreationTimeStamp")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.AccountCreationTimeStamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("crowdstrike.event.AccountCreationTimeStamp", parsed)?;
                        }
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.StartTime") };
                if _cond {
                    if event.has_value("crowdstrike.event.StartTime") {
                        if let Some(val) = event.get("crowdstrike.event.StartTime") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.StartTime".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.StartTime", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.StartTime")
                        && event
                            .get_as_string("crowdstrike.event.StartTime")
                            .is_some_and(|s| s.len() > 18)
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.StartTime") {
                        let re = cached_regex!("\\d{6}$");
                        let replaced = re.replace_all(&s, "").into_owned();
                        event.set("crowdstrike.event.StartTime", replaced)?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.StartTime")
                        && event
                            .get_as_string("crowdstrike.event.StartTime")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTime") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("event.start", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.StartTime")
                        && event
                            .get_as_string("crowdstrike.event.StartTime")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTime") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("event.start", parsed)?;
                        }
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.EndTime") };
                if _cond {
                    if event.has_value("crowdstrike.event.EndTime") {
                        if let Some(val) = event.get("crowdstrike.event.EndTime") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.EndTime".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.EndTime", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EndTime")
                        && event
                            .get_as_string("crowdstrike.event.EndTime")
                            .is_some_and(|s| s.len() > 18)
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.EndTime") {
                        let re = cached_regex!("\\d{6}$");
                        let replaced = re.replace_all(&s, "").into_owned();
                        event.set("crowdstrike.event.EndTime", replaced)?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EndTime")
                        && event
                            .get_as_string("crowdstrike.event.EndTime")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTime") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("event.end", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EndTime")
                        && event
                            .get_as_string("crowdstrike.event.EndTime")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTime") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("event.end", parsed)?;
                        }
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.TargetEndpointHostName") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("crowdstrike.event.TargetEndpointHostName")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.TargetDomain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("crowdstrike.event.TargetDomain")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.TargetAccountName") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("crowdstrike.event.TargetAccountName")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.AdditionalAccountDomain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("crowdstrike.event.AdditionalAccountDomain")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.AdditionalAccountName") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("crowdstrike.event.AdditionalAccountName")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.AdditionalEndpointHostName") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("crowdstrike.event.AdditionalEndpointHostName")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.AdditionalEndpointIpAddress") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("crowdstrike.event.AdditionalEndpointIpAddress")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "ipd_detection_summary"
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType")
                    == Some("ReconNotificationSummaryEvent")
            };
            if _cond {
                // Begin nested pipeline: "recon_notification_summary"
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("threat"))?;
                event.append("event.type", json!("indicator"))?;
                let _cond = { !event.has_value("crowdstrike.event.ItemType") };
                if _cond {
                    event.set("event.action", json!("recon-notification"))?;
                }
                let _cond = { event.has_value("crowdstrike.event.ItemType") };
                if _cond {
                    event.set(
                        "event.action",
                        json!(format!(
                            "recon-notification-{}",
                            event
                                .get("crowdstrike.event.ItemType")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                }
                if event.has("crowdstrike.event.ItemId") {
                    event.rename("crowdstrike.event.ItemId", "event.id")?;
                }
                if event.has("crowdstrike.event.RuleId") {
                    event.rename("crowdstrike.event.RuleId", "rule.id")?;
                }
                if event.has("crowdstrike.event.RuleName") {
                    event.rename("crowdstrike.event.RuleName", "rule.name")?;
                }
                if let Some(v) = event
                    .get("crowdstrike.event.RuleTopic")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.ruleset", v)?;
                }
                if event.has("crowdstrike.event.RuleTopic") {
                    event.rename("crowdstrike.event.RuleTopic", "rule.description")?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.MatchedTimestamp")
                        && event
                            .get_as_string("crowdstrike.event.MatchedTimestamp")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.MatchedTimestamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("event.created", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.MatchedTimestamp")
                        && event
                            .get_as_string("crowdstrike.event.MatchedTimestamp")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.MatchedTimestamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("event.created", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ItemPostedTimestamp")
                        && event
                            .get_as_string("crowdstrike.event.ItemPostedTimestamp")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ItemPostedTimestamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("event.created", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ItemPostedTimestamp")
                        && event
                            .get_as_string("crowdstrike.event.ItemPostedTimestamp")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ItemPostedTimestamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("event.created", parsed)?;
                        }
                    }
                }
                // End nested pipeline: "recon_notification_summary"
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType") == Some("IdentityProtectionEvent")
            };
            if _cond {
                // Begin nested pipeline: "identity_protection_incident"
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("iam"))?;
                event.append("event.type", json!("info"))?;
                if event.has("crowdstrike.event.IncidentType") {
                    event.rename("crowdstrike.event.IncidentType", "event.action")?;
                }
                if event.has("crowdstrike.event.IncidentDescription") {
                    event.rename("crowdstrike.event.IncidentDescription", "message")?;
                }
                if event.has("crowdstrike.event.IdentityProtectionIncidentId") {
                    event.rename("crowdstrike.event.IdentityProtectionIncidentId", "event.id")?;
                }
                if event.has("crowdstrike.event.FalconHostLink") {
                    event.rename("crowdstrike.event.FalconHostLink", "event.reference")?;
                }
                if event.has("crowdstrike.event.UserName") {
                    event.rename("crowdstrike.event.UserName", "user.name")?;
                }
                let _cond = {
                    event.has_value("user.name")
                        && event.get("user.name").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("\\"))
                            }
                            serde_json::Value::String(s) => s.contains("\\"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(input) = event.get_string("user.name") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("\\") else {
                                break 'dissect false;
                            };
                            captured.push(("user.domain", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("\\") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user.name", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "user.name".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
                if event.has("crowdstrike.event.EndpointName") {
                    event.rename("crowdstrike.event.EndpointName", "host.hostname")?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EndpointIp")
                        && event.get_str("crowdstrike.event.EndpointIp") != Some("")
                };
                if _cond {
                    event.append(
                        "host.ip",
                        json!(
                            event
                                .get("crowdstrike.event.EndpointIp")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.EndpointIp") };
                if _cond {
                    if event.remove("crowdstrike.event.EndpointIp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "crowdstrike.event.EndpointIp".into(),
                        });
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.StartTime") };
                if _cond {
                    if event.has_value("crowdstrike.event.StartTime") {
                        if let Some(val) = event.get("crowdstrike.event.StartTime") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.StartTime".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.StartTime", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.StartTime")
                        && event
                            .get_as_string("crowdstrike.event.StartTime")
                            .is_some_and(|s| s.len() > 18)
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.StartTime") {
                        let re = cached_regex!("\\d{6}$");
                        let replaced = re.replace_all(&s, "").into_owned();
                        event.set("crowdstrike.event.StartTime", replaced)?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.StartTime")
                        && event
                            .get_as_string("crowdstrike.event.StartTime")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTime") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("event.start", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.StartTime")
                        && event
                            .get_as_string("crowdstrike.event.StartTime")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTime") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("event.start", parsed)?;
                        }
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.EndTime") };
                if _cond {
                    if event.has_value("crowdstrike.event.EndTime") {
                        if let Some(val) = event.get("crowdstrike.event.EndTime") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.EndTime".into(),
                                    message,
                                }
                            })?;
                            event.set("crowdstrike.event.EndTime", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EndTime")
                        && event
                            .get_as_string("crowdstrike.event.EndTime")
                            .is_some_and(|s| s.len() > 18)
                };
                if _cond {
                    if let Some(s) = event.get_string("crowdstrike.event.EndTime") {
                        let re = cached_regex!("\\d{6}$");
                        let replaced = re.replace_all(&s, "").into_owned();
                        event.set("crowdstrike.event.EndTime", replaced)?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EndTime")
                        && event
                            .get_as_string("crowdstrike.event.EndTime")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTime") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("event.end", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EndTime")
                        && event
                            .get_as_string("crowdstrike.event.EndTime")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTime") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("event.end", parsed)?;
                        }
                    }
                }
                let _cond = { event.has_value("event.start") };
                if _cond {
                    if let Some(v) = event.get("event.start").cloned() {
                        event.set("@timestamp", v)?;
                    }
                }
                // End nested pipeline: "identity_protection_incident"
            }

            let _cond = {
                ["CSPMIOAStreamingEvent", "CSPMSearchStreamingEvent"].contains(
                    &event
                        .get_str("crowdstrike.metadata.eventType")
                        .unwrap_or(""),
                )
            };
            if _cond {
                // Begin nested pipeline: "cspm_events"
                event.set("event.kind", json!("alert"))?;
                event.append("event.category", json!("configuration"))?;
                event.append("event.type", json!("info"))?;
                event.append("event.type", json!("change"))?;
                let _cond = { event.get_str("crowdstrike.event.Disposition") == Some("Passed") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_str("crowdstrike.event.Disposition") == Some("Failed") };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                if event.has("crowdstrike.event.EventAction") {
                    event.rename("crowdstrike.event.EventAction", "event.action")?;
                }
                if event.has("crowdstrike.event.ReportUrl") {
                    event.rename("crowdstrike.event.ReportUrl", "event.reference")?;
                }
                let _cond = { event.has_value("crowdstrike.event.ResourceAttributes") };
                if _cond {
                    parse_json_field(
                        event,
                        "crowdstrike.event.ResourceAttributes",
                        "crowdstrike.event.ResourceAttributes",
                    )?;
                }
                if event.has("crowdstrike.event.EventSource") {
                    event.rename("crowdstrike.event.EventSource", "event.provider")?;
                }
                let _cond = { !event.has_value("cloud.account.id") };
                if _cond {
                    if event.has("crowdstrike.event.AccountId") {
                        event.rename("crowdstrike.event.AccountId", "cloud.account.id")?;
                    }
                }
                let _cond = { !event.has_value("cloud.region") };
                if _cond {
                    if event.has("crowdstrike.event.Region") {
                        event.rename("crowdstrike.event.Region", "cloud.region")?;
                    }
                }
                let _cond = { !event.has_value("cloud.provider") };
                if _cond {
                    if event.has("crowdstrike.event.CloudProvider") {
                        event.rename("crowdstrike.event.CloudProvider", "cloud.provider")?;
                    }
                }
                let _cond = { !event.has_value("cloud.provider") };
                if _cond {
                    if event.has("crowdstrike.event.CloudPlatform") {
                        event.rename("crowdstrike.event.CloudPlatform", "cloud.provider")?;
                    }
                }
                let _cond = { !event.has_value("cloud.service.name") };
                if _cond {
                    if event.has("crowdstrike.event.CloudService") {
                        event.rename("crowdstrike.event.CloudService", "cloud.service.name")?;
                    }
                }
                if event.has("crowdstrike.event.PolicyStatement") {
                    event.rename("crowdstrike.event.PolicyStatement", "message")?;
                }
                if event.has("crowdstrike.event.UserName") {
                    event.rename("crowdstrike.event.UserName", "user.name")?;
                }
                if event.has("crowdstrike.event.UserId") {
                    event.rename("crowdstrike.event.UserId", "user.id")?;
                }
                if event.has("crowdstrike.event.UserSourceIp") {
                    event.rename("crowdstrike.event.UserSourceIp", "source.ip")?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.Timestamp")
                        && event
                            .get_as_string("crowdstrike.event.Timestamp")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.Timestamp") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.Timestamp")
                        && event
                            .get_as_string("crowdstrike.event.Timestamp")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.Timestamp") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EventCreatedTimestamp")
                        && event
                            .get_as_string("crowdstrike.event.EventCreatedTimestamp")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.EventCreatedTimestamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EventCreatedTimestamp")
                        && event
                            .get_as_string("crowdstrike.event.EventCreatedTimestamp")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.EventCreatedTimestamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ResourceCreateTime")
                        && event.get_i64("crowdstrike.event.ResourceCreateTime") == Some(0)
                };
                if _cond {
                    event.remove("crowdstrike.event.ResourceCreateTime");
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ResourceCreateTime")
                        && event.get_i64("crowdstrike.event.ResourceCreateTime") != Some(0)
                        && event
                            .get_as_string("crowdstrike.event.ResourceCreateTime")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ResourceCreateTime")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("crowdstrike.event.ResourceCreateTime", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ResourceCreateTime")
                        && event.get_i64("crowdstrike.event.ResourceCreateTime") != Some(0)
                        && event
                            .get_as_string("crowdstrike.event.ResourceCreateTime")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ResourceCreateTime")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("crowdstrike.event.ResourceCreateTime", parsed)?;
                        }
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.Tactic") };
                if _cond {
                    event.append(
                        "threat.tactic.name",
                        json!(
                            event
                                .get("crowdstrike.event.Tactic")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("crowdstrike.event.Technique") };
                if _cond {
                    event.append(
                        "threat.technique.name",
                        json!(
                            event
                                .get("crowdstrike.event.Technique")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "cspm_events"
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType") == Some("UserActivityAuditEvent")
            };
            if _cond {
                // Begin nested pipeline: "user_activity_audit"
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("iam"))?;
                event.append("event.type", json!("change"))?;
                event.set("event.action", json!("user_activity_audit_event"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("crowdstrike.event.UserId") {
                        if let Some(input) = event.get_string("crowdstrike.event.UserId") {
                            // Grok pattern: %{USERNAME:user.name}@%{HOSTNAME:user.domain}
                            if !cached_grok!("%{USERNAME:user.name}@%{HOSTNAME:user.domain}")
                                .extract_into(&input, event)?
                            {
                                // Grok pattern: %{GREEDYDATA:user.name}
                                if !cached_grok!("%{GREEDYDATA:user.name}")
                                    .extract_into(&input, event)?
                                {}
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("crowdstrike.event.UserId")
                        && event
                            .get_str("crowdstrike.event.UserId")
                            .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                            .is_some_and(|i| i.is_some_and(|i| i > 0))
                };
                if _cond {
                    if let Some(v) = event.get("crowdstrike.event.UserId").cloned() {
                        event.set("user.email", v)?;
                    }
                }
                if event.has("crowdstrike.event.OperationName") {
                    event.rename("crowdstrike.event.OperationName", "message")?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.UserIp")
                        && event.get_str("crowdstrike.event.UserIp") != Some("")
                };
                if _cond {
                    if event.has("crowdstrike.event.UserIp") {
                        event.rename("crowdstrike.event.UserIp", "source.ip")?;
                    }
                }
                // End nested pipeline: "user_activity_audit"
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType") == Some("AuthActivityAuditEvent")
            };
            if _cond {
                // Begin nested pipeline: "auth_activity_audit"
                event.set("event.kind", json!("event"))?;
                let _cond = {
                    event.has_value("crowdstrike.event.OperationName")
                        && !(["twoFactorAuthenticate", "userAuthenticate"].contains(
                            &event
                                .get_str("crowdstrike.event.OperationName")
                                .unwrap_or(""),
                        ))
                };
                if _cond {
                    event.append("event.category", json!("iam"))?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.OperationName")
                        && ["twoFactorAuthenticate", "userAuthenticate"].contains(
                            &event
                                .get_str("crowdstrike.event.OperationName")
                                .unwrap_or(""),
                        )
                };
                if _cond {
                    event.append("event.category", json!("authentication"))?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.OperationName")
                        && [
                            "activateUser",
                            "changePassword",
                            "confirmResetPassword",
                            "deactivateUser",
                            "grantUserRoles",
                            "grantCustomerSubscriptions",
                            "revokeUserRoles",
                            "revokeCustomerSubscriptions",
                            "updateUser",
                            "updateUserRoles",
                        ]
                        .contains(
                            &event
                                .get_str("crowdstrike.event.OperationName")
                                .unwrap_or(""),
                        )
                };
                if _cond {
                    event.append("event.type", json!("user"))?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.OperationName")
                        && [
                            "activateUser",
                            "changePassword",
                            "confirmResetPassword",
                            "deactivateUser",
                            "grantUserRoles",
                            "grantCustomerSubscriptions",
                            "revokeUserRoles",
                            "revokeCustomerSubscriptions",
                            "updateUser",
                            "updateUserRoles",
                        ]
                        .contains(
                            &event
                                .get_str("crowdstrike.event.OperationName")
                                .unwrap_or(""),
                        )
                };
                if _cond {
                    event.append("event.type", json!("change"))?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.OperationName")
                        && event.get_str("crowdstrike.event.OperationName") == Some("createUser")
                };
                if _cond {
                    event.append("event.type", json!("creation"))?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.OperationName")
                        && event.get_str("crowdstrike.event.OperationName") == Some("deleteUser")
                };
                if _cond {
                    event.append("event.type", json!("deletion"))?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("crowdstrike.event.UserId") {
                        if let Some(input) = event.get_string("crowdstrike.event.UserId") {
                            // Grok pattern: %{USERNAME:user.name}@%{HOSTNAME:user.domain}
                            if !cached_grok!("%{USERNAME:user.name}@%{HOSTNAME:user.domain}")
                                .extract_into(&input, event)?
                            {
                                // Grok pattern: %{GREEDYDATA:user.name}
                                if !cached_grok!("%{GREEDYDATA:user.name}")
                                    .extract_into(&input, event)?
                                {}
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("crowdstrike.event.UserId")
                        && event
                            .get_str("crowdstrike.event.UserId")
                            .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                            .is_some_and(|i| i.is_some_and(|i| i > 0))
                };
                if _cond {
                    if let Some(v) = event.get("crowdstrike.event.UserId").cloned() {
                        event.set("user.email", v)?;
                    }
                }
                let _cond = { event.has_value("crowdstrike.event.OperationName") };
                if _cond {
                    event.append(
                        "event.action",
                        json!(
                            event
                                .get("crowdstrike.event.OperationName")
                                .map_or_else(String::new, painless_to_string)
                        ),
                    )?;
                }
                let _cond = { !event.has_value("event.action") };
                if _cond {
                    event.append("event.action", json!("AuthActivityAuditEvent"))?;
                }
                let _cond = { event.get_bool("crowdstrike.event.Success") == Some(true) };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_bool("crowdstrike.event.Success") == Some(false) };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = { !event.has_value("event.outcome") };
                if _cond {
                    event.set("event.outcome", json!("unknown"))?;
                }
                if event.has("crowdstrike.event.ServiceName") {
                    event.rename("crowdstrike.event.ServiceName", "message")?;
                }
                if event.has("crowdstrike.event.UserIp") {
                    event.rename("crowdstrike.event.UserIp", "source.ip")?;
                }
                // End nested pipeline: "auth_activity_audit"
            }

            let _cond =
                { event.get_str("crowdstrike.metadata.eventType") == Some("FirewallMatchEvent") };
            if _cond {
                // Begin nested pipeline: "firewall_match"
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("network"))?;
                event.append("event.action", json!("firewall_match_event"))?;
                event.append("event.type", json!("start"))?;
                event.append("event.type", json!("connection"))?;
                let _cond = {
                    event.has_value("crowdstrike.event.RuleAction")
                        && event.get_str("crowdstrike.event.RuleAction") == Some("1")
                };
                if _cond {
                    event.append("event.type", json!("allowed"))?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.RuleAction")
                        && event.get_str("crowdstrike.event.RuleAction") == Some("1")
                };
                if _cond {
                    event.set("_tmp_.action", json!("Allowed"))?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.RuleAction")
                        && event.get_str("crowdstrike.event.RuleAction") == Some("2")
                };
                if _cond {
                    event.append("event.type", json!("denied"))?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.RuleAction")
                        && event.get_str("crowdstrike.event.RuleAction") == Some("2")
                };
                if _cond {
                    event.set("_tmp_.action", json!("Blocked"))?;
                }
                let _cond = { !event.has_value("_tmp_.action") };
                if _cond {
                    event.set("_tmp_.action", json!("Unknown"))?;
                }
                let _cond = { event.has_value("crowdstrike.event.RuleName") };
                if _cond {
                    event.set(
                        "message",
                        json!(format!(
                            "Firewall Rule: '{}' triggered - Action: '{}'",
                            event
                                .get("crowdstrike.event.RuleName")
                                .map_or_else(String::new, painless_to_string),
                            event
                                .get("_tmp_.action")
                                .map_or_else(String::new, painless_to_string)
                        )),
                    )?;
                }
                if event.has("crowdstrike.event.Ipv") {
                    event.rename("crowdstrike.event.Ipv", "network.type")?;
                }
                if event.has_value("crowdstrike.event.PID") {
                    if let Some(val) = event.get("crowdstrike.event.PID") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "crowdstrike.event.PID".into(),
                                message,
                            }
                        })?;
                        event.set("process.pid", converted)?;
                    }
                }
                let v = json!(
                    event
                        .get("crowdstrike.event.ImageFileName")
                        .map_or_else(String::new, painless_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("process.executable", v)?;
                }
                event.remove("crowdstrike.event.ImageFileName");
                if event.has("crowdstrike.event.RuleId") {
                    event.rename("crowdstrike.event.RuleId", "rule.id")?;
                }
                if event.has("crowdstrike.event.RuleName") {
                    event.rename("crowdstrike.event.RuleName", "rule.name")?;
                }
                if event.has("crowdstrike.event.RuleGroupName") {
                    event.rename("crowdstrike.event.RuleGroupName", "rule.ruleset")?;
                }
                if event.has("crowdstrike.event.RuleDescription") {
                    event.rename("crowdstrike.event.RuleDescription", "rule.description")?;
                }
                if event.has("crowdstrike.event.RuleFamilyID") {
                    event.rename("crowdstrike.event.RuleFamilyID", "rule.category")?;
                }
                if event.has("crowdstrike.event.HostName") {
                    event.rename("crowdstrike.event.HostName", "host.name")?;
                }
                if event.has("crowdstrike.event.EventType") {
                    event.rename("crowdstrike.event.EventType", "event.code")?;
                }
                let _cond = { event.has_value("crowdstrike.event.ConnectionDirection") };
                if _cond {
                    // Painless script
                    // Source: def result = [];\nif (ctx.crowdstrike.event.ConnectionDirection == \"0\") {\n  result.add('egress');\n} else if (ctx.crowdstrike.event.ConnectionDirection == \"1\") {\n  result.add('ingress');\n} else if (ctx.crowdstrike.event.ConnectionDirection == \"3\") {\n  result.add('egress');\n  result.add('ingress');\n} else if (ctx.crowdstrike.event.ConnectionDirection == \"4\") {\n  result.add('unknown');\n}\nif (result.size() > 0) {\n  ctx.network = ctx.network ?: [:];\n}\nif (result.size() == 1) {\n  ctx.network.direction = result[0];\n} else if (result.size() > 1) {\n  ctx.network.direction = result;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def result = [];\nif (ctx.crowdstrike.event.ConnectionDirection == \"0\") {\n  result.add('egress');\n} else if (ctx.crowdstrike.event.ConnectionDirection == \"1\") {\n  result.add('ingress');\n} else if (ctx.crowdstrike.event.ConnectionDirection == \"3\") {\n  result.add('egress');\n  result.add('ingress');\n} else if (ctx.crowdstrike.event.ConnectionDirection == \"4\") {\n  result.add('unknown');\n}\nif (result.size() > 0) {\n  ctx.network = ctx.network ?: [:];\n}\nif (result.size() == 1) {\n  ctx.network.direction = result[0];\n} else if (result.size() > 1) {\n  ctx.network.direction = result;\n}\n"#
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("crowdstrike.event.RemoteAddress")
                        && event.get_str("network.direction") == Some("ingress")
                };
                if _cond {
                    if event.has("crowdstrike.event.RemoteAddress") {
                        event.rename("crowdstrike.event.RemoteAddress", "source.ip")?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.LocalAddress")
                        && event.get_str("network.direction") == Some("ingress")
                };
                if _cond {
                    if event.has("crowdstrike.event.LocalAddress") {
                        event.rename("crowdstrike.event.LocalAddress", "destination.ip")?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.LocalPort")
                        && event.get_str("network.direction") == Some("ingress")
                };
                if _cond {
                    if event.has_value("crowdstrike.event.LocalPort") {
                        if let Some(val) = event.get("crowdstrike.event.LocalPort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.LocalPort".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.RemotePort")
                        && event.get_str("network.direction") == Some("ingress")
                };
                if _cond {
                    if event.has_value("crowdstrike.event.RemotePort") {
                        if let Some(val) = event.get("crowdstrike.event.RemotePort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.RemotePort".into(),
                                    message,
                                }
                            })?;
                            event.set("source.port", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.RemoteAddress")
                        && event.get_str("network.direction") == Some("egress")
                };
                if _cond {
                    if event.has("crowdstrike.event.RemoteAddress") {
                        event.rename("crowdstrike.event.RemoteAddress", "destination.ip")?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.LocalAddress")
                        && event.get_str("network.direction") == Some("egress")
                };
                if _cond {
                    if event.has("crowdstrike.event.LocalAddress") {
                        event.rename("crowdstrike.event.LocalAddress", "source.ip")?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.LocalPort")
                        && event.get_str("network.direction") == Some("egress")
                };
                if _cond {
                    if event.has_value("crowdstrike.event.LocalPort") {
                        if let Some(val) = event.get("crowdstrike.event.LocalPort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.LocalPort".into(),
                                    message,
                                }
                            })?;
                            event.set("source.port", converted)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.RemotePort")
                        && event.get_str("network.direction") == Some("egress")
                };
                if _cond {
                    if event.has_value("crowdstrike.event.RemotePort") {
                        if let Some(val) = event.get("crowdstrike.event.RemotePort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "crowdstrike.event.RemotePort".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                }
                if event.has("crowdstrike.event.Platform") {
                    event.rename("crowdstrike.event.Platform", "host.os.platform")?;
                }
                // End nested pipeline: "firewall_match"
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType")
                    == Some("RemoteResponseSessionStartEvent")
            };
            if _cond {
                // Begin nested pipeline: "remote_response_session_start"
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("network"))?;
                event.append("event.category", json!("session"))?;
                event.append("event.action", json!("remote_response_session_start_event"))?;
                event.append("event.type", json!("start"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("crowdstrike.event.UserName") {
                        if let Some(input) = event.get_string("crowdstrike.event.UserName") {
                            // Grok pattern: %{USERNAME:user.name}@%{HOSTNAME:user.domain}
                            if !cached_grok!("%{USERNAME:user.name}@%{HOSTNAME:user.domain}")
                                .extract_into(&input, event)?
                            {
                                // Grok pattern: %{GREEDYDATA:user.name}
                                if !cached_grok!("%{GREEDYDATA:user.name}")
                                    .extract_into(&input, event)?
                                {}
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("crowdstrike.event.UserName")
                        && event
                            .get_str("crowdstrike.event.UserName")
                            .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                            .is_some_and(|i| i.is_some_and(|i| i > 0))
                };
                if _cond {
                    if let Some(v) = event.get("crowdstrike.event.UserName").cloned() {
                        event.set("user.email", v)?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.StartTimestamp")
                        && event
                            .get_as_string("crowdstrike.event.StartTimestamp")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTimestamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("event.start", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.StartTimestamp")
                        && event
                            .get_as_string("crowdstrike.event.StartTimestamp")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.StartTimestamp")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("event.start", parsed)?;
                        }
                    }
                }
                event.set("message", json!("Remote response session started."))?;
                if event.has("crowdstrike.event.HostnameField") {
                    event.rename("crowdstrike.event.HostnameField", "host.name")?;
                }
                // End nested pipeline: "remote_response_session_start"
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType")
                    == Some("RemoteResponseSessionEndEvent")
            };
            if _cond {
                // Begin nested pipeline: "remote_response_session_end"
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("network"))?;
                event.append("event.category", json!("session"))?;
                event.append("event.action", json!("remote_response_session_end_event"))?;
                event.append("event.type", json!("end"))?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("crowdstrike.event.UserName") {
                        if let Some(input) = event.get_string("crowdstrike.event.UserName") {
                            // Grok pattern: %{USERNAME:user.name}@%{HOSTNAME:user.domain}
                            if !cached_grok!("%{USERNAME:user.name}@%{HOSTNAME:user.domain}")
                                .extract_into(&input, event)?
                            {
                                // Grok pattern: %{GREEDYDATA:user.name}
                                if !cached_grok!("%{GREEDYDATA:user.name}")
                                    .extract_into(&input, event)?
                                {}
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("crowdstrike.event.UserName")
                        && event
                            .get_str("crowdstrike.event.UserName")
                            .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                            .is_some_and(|i| i.is_some_and(|i| i > 0))
                };
                if _cond {
                    if let Some(v) = event.get("crowdstrike.event.UserName").cloned() {
                        event.set("user.email", v)?;
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EndTimestamp")
                        && event
                            .get_as_string("crowdstrike.event.EndTimestamp")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTimestamp") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("event.end", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.EndTimestamp")
                        && event
                            .get_as_string("crowdstrike.event.EndTimestamp")
                            .is_some_and(|s| s.len() <= 11)
                };
                if _cond {
                    if let Some(date_str) = event.get_as_string("crowdstrike.event.EndTimestamp") {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX"], Some("UTC"), None)
                        {
                            event.set("event.end", parsed)?;
                        }
                    }
                }
                event.set("message", json!("Remote response session ended."))?;
                if event.has("crowdstrike.event.HostnameField") {
                    event.rename("crowdstrike.event.HostnameField", "host.name")?;
                }
                // End nested pipeline: "remote_response_session_end"
            }

            let _cond = {
                event.get_str("crowdstrike.metadata.eventType")
                    == Some("ScheduledReportNotificationEvent")
            };
            if _cond {
                // Begin nested pipeline: "scheduled_report_notification_event"
                event.set("event.kind", json!("event"))?;
                let _cond = {
                    event.has_value("crowdstrike.event.ExecutionMetadata.ExecutionStart")
                        && event
                            .get_as_string("crowdstrike.event.ExecutionMetadata.ExecutionStart")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ExecutionMetadata.ExecutionStart")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ExecutionMetadata.SearchWindowStart")
                        && event
                            .get_as_string("crowdstrike.event.ExecutionMetadata.SearchWindowStart")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ExecutionMetadata.SearchWindowStart")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                }
                let _cond = {
                    event.has_value("crowdstrike.event.ExecutionMetadata.SearchWindowEnd")
                        && event
                            .get_as_string("crowdstrike.event.ExecutionMetadata.SearchWindowEnd")
                            .is_some_and(|s| s.len() >= 12)
                };
                if _cond {
                    if let Some(date_str) =
                        event.get_as_string("crowdstrike.event.ExecutionMetadata.SearchWindowEnd")
                    {
                        if let Some(parsed) =
                            parse_date_out(&date_str, &["UNIX_MS"], Some("UTC"), None)
                        {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                }
                if event.has_value("crowdstrike.event.ExecutionMetadata.ExecutionDuration") {
                    if let Some(val) =
                        event.get("crowdstrike.event.ExecutionMetadata.ExecutionDuration")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "crowdstrike.event.ExecutionMetadata.ExecutionDuration"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "crowdstrike.event.ExecutionMetadata.ExecutionDuration",
                            converted,
                        )?;
                    }
                }
                if event.has_value("crowdstrike.event.ExecutionMetadata.ResultCount") {
                    if let Some(val) = event.get("crowdstrike.event.ExecutionMetadata.ResultCount")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "crowdstrike.event.ExecutionMetadata.ResultCount".into(),
                                message,
                            }
                        })?;
                        event.set("crowdstrike.event.ExecutionMetadata.ResultCount", converted)?;
                    }
                }
                if event.has("crowdstrike.event.UserID") {
                    event.rename("crowdstrike.event.UserID", "user.id")?;
                }
                let _cond = {
                    event.has_value("user.id")
                        && event.get("user.id").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("@"))
                            }
                            serde_json::Value::String(s) => s.contains("@"),
                            _ => false,
                        })
                };
                if _cond {
                    if let Some(input) = event.get_string("user.id") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            captured.push(("user.name", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user.domain", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "user.id".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                }
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
                if event.has_value("crowdstrike.event.Status") {
                    if let Some(val) = event.get("crowdstrike.event.Status") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "crowdstrike.event.Status".into(),
                                message,
                            }
                        })?;
                        event.set("crowdstrike.event.Status", converted)?;
                    }
                }
                // End nested pipeline: "scheduled_report_notification_event"
            }

            let v = json!(
                event
                    .get("process.pid")
                    .map_or_else(String::new, painless_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("process.entity_id", v)?;
            }

            let v = json!(
                event
                    .get("process.parent.pid")
                    .map_or_else(String::new, painless_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("process.parent.entity_id", v)?;
            }

            let _cond = {
                !event.has_value("user.email")
                    && event.has_value("user.name")
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("user.name").cloned() {
                    event.set("user.email", v)?;
                }
            }

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, painless_to_string)
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
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("host.name") && event.get_str("host.name") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("@timestamp") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("crowdstrike.event.SessionId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("crowdstrike.event.DetectId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("crowdstrike.event.PID") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("crowdstrike.event.RuleId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("crowdstrike.metadata.eventType") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("crowdstrike.metadata.customerIDString") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("crowdstrike.metadata.offset") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("destination.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("destination.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            let _cond = { !event.has_value("device.id") };
            if _cond {
                if let Some(v) = event
                    .get("crowdstrike.event.DeviceId")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("device.id", v)?;
                }
            }

            let _cond = {
                event
                    .get("crowdstrike.event.MitreAttack")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("crowdstrike.event.MitreAttack").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "threat.tactic.name",
                            json!(
                                event
                                    .get("_ingest._value.Tactic")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("crowdstrike.event.MitreAttack", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("crowdstrike.event.MitreAttack")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("crowdstrike.event.MitreAttack").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "threat.tactic.id",
                            json!(
                                event
                                    .get("_ingest._value.TacticID")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("crowdstrike.event.MitreAttack", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("crowdstrike.event.MitreAttack")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("crowdstrike.event.MitreAttack").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "threat.technique.name",
                            json!(
                                event
                                    .get("_ingest._value.Technique")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("crowdstrike.event.MitreAttack", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("crowdstrike.event.MitreAttack")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("crowdstrike.event.MitreAttack").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        event.append_unique(
                            "threat.technique.id",
                            json!(
                                event
                                    .get("_ingest._value.TechniqueID")
                                    .map_or_else(String::new, painless_to_string)
                            ),
                        )?;
                        out.push(event.remove("_ingest._value").unwrap_or(Value::Null));
                    }
                    event.remove("_ingest");
                    event.set("crowdstrike.event.MitreAttack", Value::Array(out))?;
                }
            }

            let _cond = {
                event
                    .get("crowdstrike.event.MitreAttack")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                if let Some(Value::Array(items)) =
                    event.get("crowdstrike.event.MitreAttack").cloned()
                {
                    let mut out = Vec::with_capacity(items.len());
                    for item in items {
                        event.set("_ingest._value", item)?;
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.PatternID") {
                                if let Some(val) = event.get("_ingest._value.PatternID") {
                                    let converted =
                                        convert_value(val, "string").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.PatternID".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.PatternID", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_mitre_attack_pattern_id_to_string",
                            )?;
                            if event.remove("_ingest._value.PatternID").is_none() {
                                return Err(TransformError::FieldNotFound {
                                    path: "_ingest._value.PatternID".into(),
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
                    event.remove("_ingest");
                    event.set("crowdstrike.event.MitreAttack", Value::Array(out))?;
                }
            }

            let _cond = { event.has_value("threat") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def tid = ctx.threat.tactic?.id;\ndef nid = ctx.threat.technique?.id;\ndef tname = ctx.threat.tactic?.name;\nif ((tid == null || tid.isEmpty()) && (nid == null || nid.isEmpty()) && (tname == null || tname.isEmpty())) {\n  return;\n}\nSet frameworks = new HashSet();\n// Handling tactics prefixed with \"CS\" or \"TA\".\nif (tid != null && !tid.isEmpty()) {\n  for (String t: tid) {\n    if (t.startsWith(\"CS\")) {\n      frameworks.add(params.framework_cs);\n    }\n    else if (t.startsWith(\"TA\")) {\n      frameworks.add(params.framework_ma);\n    }\n  }\n}\n// Handling techniques prefixed with \"CS\".\nif (nid != null && !nid.isEmpty()) {\n  for (String t: nid) {\n    if (t.startsWith(\"CS\")) {\n      frameworks.add(params.framework_cs);\n    }\n  }\n}\n// Handling falcon specific tactics.\nif (tname != null && !tname.isEmpty()) {\n  for (String t: tname) {\n    if (params.falcon_tactic_names.contains(t.toLowerCase())) {\n      frameworks.add(params.framework_cs);\n    }\n  }\n}\n\nif (frameworks.isEmpty()) {\n  return;\n}\nif (frameworks.size() == 1) {\n  ctx.threat.framework = frameworks.iterator().next();\n  return;\n}\n\nfor (def preferred : params.framework_preference) {\n  if (frameworks.contains(preferred)) {\n    ctx.threat.framework = preferred;\n    return;\n  }\n}\n\n// fallback when new frameworks are added and not yet in preference list\nctx.threat.framework = frameworks.iterator().next();\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def tid = ctx.threat.tactic?.id;\ndef nid = ctx.threat.technique?.id;\ndef tname = ctx.threat.tactic?.name;\nif ((tid == null || tid.isEmpty()) && (nid == null || nid.isEmpty()) && (tname == null || tname.isEmpty())) {\n  return;\n}\nSet frameworks = new HashSet();\n// Handling tactics prefixed with \"CS\" or \"TA\".\nif (tid != null && !tid.isEmpty()) {\n  for (String t: tid) {\n    if (t.startsWith(\"CS\")) {\n      frameworks.add(params.framework_cs);\n    }\n    else if (t.startsWith(\"TA\")) {\n      frameworks.add(params.framework_ma);\n    }\n  }\n}\n// Handling techniques prefixed with \"CS\".\nif (nid != null && !nid.isEmpty()) {\n  for (String t: nid) {\n    if (t.startsWith(\"CS\")) {\n      frameworks.add(params.framework_cs);\n    }\n  }\n}\n// Handling falcon specific tactics.\nif (tname != null && !tname.isEmpty()) {\n  for (String t: tname) {\n    if (params.falcon_tactic_names.contains(t.toLowerCase())) {\n      frameworks.add(params.framework_cs);\n    }\n  }\n}\n\nif (frameworks.isEmpty()) {\n  return;\n}\nif (frameworks.size() == 1) {\n  ctx.threat.framework = frameworks.iterator().next();\n  return;\n}\n\nfor (def preferred : params.framework_preference) {\n  if (frameworks.contains(preferred)) {\n    ctx.threat.framework = preferred;\n    return;\n  }\n}\n\n// fallback when new frameworks are added and not yet in preference list\nctx.threat.framework = frameworks.iterator().next();\n"#
                        ),
                        cached_params!(
                            "{\"falcon_tactic_names\":[\"malware\",\"exploit\",\"post-exploit\",\"machine learning\",\"custom intelligence\",\"falcon overwatch\",\"falcon intel\",\"ai powered ioa\",\"insecure security posture\"],\"framework_cs\":\"CrowdStrike Falcon Detections Framework\",\"framework_ma\":\"MITRE ATT&CK\",\"framework_preference\":[\"MITRE ATT&CK\",\"CrowdStrike Falcon Detections Framework\"]}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_threat_framework",
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

            event.remove("_tmp_");
            event.remove("crowdstrike.event.Technique");
            event.remove("crowdstrike.event.TechniqueId");
            event.remove("crowdstrike.event.Tactic");
            event.remove("crowdstrike.event.TacticId");
            event.remove("crowdstrike.event.Techniques");
            event.remove("crowdstrike.event.TechniqueIds");
            event.remove("crowdstrike.event.Tactics");
            event.remove("crowdstrike.event.TacticIds");
            event.remove("crowdstrike.event.IPv6Addresses");
            event.remove("crowdstrike.event.IPv4Addresses");
            event.remove("crowdstrike.event.ParentCommandLine");
            event.remove("crowdstrike.event.CommandLine");
            event.remove("crowdstrike.event.ProcessStartTime");
            event.remove("crowdstrike.event.IncidentStartTime");
            event.remove("crowdstrike.event.HostNames");
            event.remove("crowdstrike.event.DomainNames");
            event.remove("crowdstrike.event.Users");
            event.remove("crowdstrike.event.SHA256Hashes");
            event.remove("crowdstrike.event.MD5Hashes");
            event.remove("crowdstrike.event.Author");
            event.remove("crowdstrike.event.ProcessEndTime");
            event.remove("crowdstrike.event.IncidentEndTime");
            event.remove("crowdstrike.metadata.eventCreationTime");
            event.remove("crowdstrike.event.UTCTimestamp");
            event.remove("crowdstrike.event.ContextTimeStamp");
            event.remove("crowdstrike.event.PID");
            event.remove("crowdstrike.event.RemotePort");
            event.remove("crowdstrike.event.LocalPort");
            event.remove("crowdstrike.event.ConnectionDirection");
            event.remove("crowdstrike.event.StartTimestamp");
            event.remove("crowdstrike.event.StartTimeEpoch");
            event.remove("crowdstrike.event.AdditionalAccountDomain");
            event.remove("crowdstrike.event.AdditionalAccountName");
            event.remove("crowdstrike.event.AdditionalEndpointHostName");
            event.remove("crowdstrike.event.AdditionalEndpointIpAddress");
            event.remove("crowdstrike.event.AttemptOutcome");
            event.remove("crowdstrike.event.EndTimeEpoch");
            event.remove("crowdstrike.event.EndTimestamp");
            event.remove("crowdstrike.event.EndTime");
            event.remove("crowdstrike.event.EventCreatedTimestamp");
            event.remove("crowdstrike.event.StartTime");
            event.remove("crowdstrike.event.Disposition");
            event.remove("crowdstrike.event.MatchedTimestamp");
            event.remove("crowdstrike.event.Tags");
            event.remove("crowdstrike.event.UserId");
            event.remove("crowdstrike.event.UserName");

            // Painless script
            // Source: void handleMap(Map map) {\n    map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n    });\n}\nvoid handleList(List list) {\n    list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n    });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n    map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n    });\n}\nvoid handleList(List list) {\n    list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n    });\n}\nhandleMap(ctx);"#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
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
