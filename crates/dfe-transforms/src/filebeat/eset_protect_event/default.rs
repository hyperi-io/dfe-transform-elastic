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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                gsub_field(
                    event,
                    "message",
                    "message",
                    cached_regex!(
                        "^(\\xEF\\xBB\\xBF|\\xFF\\xFE\\x00\\x00|\\x00\\x00\\xFE\\xFF|\\xFF\\xFE|\\xFE\\xFF)"
                    ),
                    "",
                )?;
                Ok(())
            })();

            let _cond = {
                event.get_str("message") == Some("HEARTBEAT")
                    && event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("drop_heartbeat_message")),
                        serde_json::Value::String(s) => s.contains("drop_heartbeat_message"),
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
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

            let _cond = { event.get_str("event.original") == Some("HEARTBEAT") };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            // SKIPPED: condition not transpiled: ctx.event?.original.startsWith("{\"")
            #[allow(unreachable_code, unused_variables)]
            if false {
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
            }

            // SKIPPED: condition not transpiled: ! ctx.event?.original.startsWith("{\"")
            #[allow(unreachable_code, unused_variables)]
            if false {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: Detection type: %{DATA:json.threat_name} Detection name: %{DATA:json.threat_name} Computer name: %{HOSTNAME:json.hostname} Logged user: %{DATA:user.domain}\\\\%{DATA:json.user} Time of occurrence: (?P<json_occurred_plaintext>(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}, %{HOUR}:%{MINUTE}:%{SECOND} (?:AM|PM) UTC\\+%{INT})) Scanner: %{DATA:json.scanner_id} Action performed: %{DATA:json.action_taken}$
                        // Grok pattern: ^%{DATA:message}\\s+%{HOSTNAME:json.hostname} (?P<json_occurred_plaintext>(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}, %{HOUR}:%{MINUTE}:%{SECOND} (?:AM|PM) UTC\\+%{INT})) %{DATA:eset_protect.event.threat_type} %{NOTSPACE:threat.indicator.name} (?P<json_object_type>(?:File)) %{GREEDYDATA:json.object_uri}$
                        let _ = extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "Detection type: %{DATA:json.threat_name} Detection name: %{DATA:json.threat_name} Computer name: %{HOSTNAME:json.hostname} Logged user: %{DATA:user.domain}\\\\%{DATA:json.user} Time of occurrence: (?P<json_occurred_plaintext>(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}, %{HOUR}:%{MINUTE}:%{SECOND} (?:AM|PM) UTC\\+%{INT})) Scanner: %{DATA:json.scanner_id} Action performed: %{DATA:json.action_taken}$",
                                    [("json_occurred_plaintext", "json.occurred_plaintext")]
                                ),
                                cached_grok_mapped!(
                                    "^%{DATA:message}\\s+%{HOSTNAME:json.hostname} (?P<json_occurred_plaintext>(?:%{MONTHNUM}/%{MONTHDAY}/%{YEAR}, %{HOUR}:%{MINUTE}:%{SECOND} (?:AM|PM) UTC\\+%{INT})) %{DATA:eset_protect.event.threat_type} %{NOTSPACE:threat.indicator.name} (?P<json_object_type>(?:File)) %{GREEDYDATA:json.object_uri}$",
                                    [
                                        ("json_occurred_plaintext", "json.occurred_plaintext"),
                                        ("json_object_type", "json.object_type")
                                    ]
                                ),
                            ],
                            &input,
                            event,
                        )?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set("_ingest.on_failure_processor_tag", "grok_event_original")?;
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
            }

            // SKIPPED: condition not transpiled: ! ctx.event?.original.startsWith("{\"") && ctx.json?.hostname != null
            #[allow(unreachable_code, unused_variables)]
            if false {
                event.append_unique("tags", json!("eset_notification"))?;
            }

            let _cond = { event.get_str("json.event_type") != Some("Audit_Event") };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = { event.get_str("json.event_type") == Some("Audit_Event") };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            let _cond = {
                event.has_value("json.event_type")
                    && ["HipsAggregated_Event", "FirewallAggregated_Event"]
                        .contains(&event.get_str("json.event_type").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("intrusion_detection"))?;
            }

            let _cond = {
                event.has_value("json.event_type")
                    && ["Threat_Event"].contains(&event.get_str("json.event_type").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("threat"))?;
            }

            let _cond = {
                event.has_value("json.event_type")
                    && ["FilteredWebsites_Event"]
                        .contains(&event.get_str("json.event_type").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("web"))?;
            }

            let _cond = {
                event.has_value("json.event_type")
                    && ["BlockedFiles_Event"]
                        .contains(&event.get_str("json.event_type").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("file"))?;
            }

            let _cond = {
                event.has_value("json.event_type")
                    && !(["Threat_Event"].contains(&event.get_str("json.event_type").unwrap_or("")))
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                event.has_value("json.event_type")
                    && ["Threat_Event"].contains(&event.get_str("json.event_type").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("indicator"))?;
            }

            if event.has_value("json.group_description") {
                event.rename(
                    "json.group_description",
                    "eset_protect.event.group_description",
                )?;
            }

            if event.has_value("json.group_name") {
                event.rename("json.group_name", "eset_protect.event.group_name")?;
            }

            if let Some(v) = event
                .get("eset_protect.event.group_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("group.name", v)?;
            }

            if event.has_value("json.hostname") {
                event.rename("json.hostname", "eset_protect.event.hostname")?;
            }

            let _cond = {
                event
                    .get("eset_protect.event.hostname")
                    .is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(".")),
                        serde_json::Value::String(s) => s.contains("."),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(v) = event.get("eset_protect.event.hostname").cloned() {
                    event.set("host.name", v)?;
                }
            }

            let _cond = { event.has_value("eset_protect.event.hostname") };
            if _cond {
                if let Some(input) = event.get_string("eset_protect.event.hostname") {
                    // Grok pattern: (%{DATA:host.hostname}\\.%{GREEDYDATA:host.domain}|%{GREEDYDATA:host.hostname})
                    let _ = cached_grok!("(%{DATA:host.hostname}\\.%{GREEDYDATA:host.domain}|%{GREEDYDATA:host.hostname})").extract_into(&input, event)?;
                }
            }

            let _cond = { event.has_value("host.hostname") && !event.has_value("host.name") };
            if _cond {
                if let Some(v) = event.get("host.hostname").cloned() {
                    event.set("host.name", v)?;
                }
            }

            if event.has_value("host.name") {
                map_strings(event, "host.name", "host.name", str::to_lowercase)?;
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

            let _cond = { event.has_value("host.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.ipv4") && event.get_str("json.ipv4") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ipv4") {
                        if let Some(val) = event.get("json.ipv4") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ipv4".into(),
                                    message,
                                }
                            })?;
                            event.set("eset_protect.event.ipv4", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_ipv4_to_ip")?;
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
            }

            let _cond = { event.has_value("eset_protect.event.ipv4") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("eset_protect.event.ipv4")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("eset_protect.event.ipv4") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("eset_protect.event.ipv4")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.ipv6") && event.get_str("json.ipv6") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ipv6") {
                        if let Some(val) = event.get("json.ipv6") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ipv6".into(),
                                    message,
                                }
                            })?;
                            event.set("eset_protect.event.ipv6", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_ipv6_to_ip")?;
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
            }

            let _cond = { event.has_value("eset_protect.event.ipv6") };
            if _cond {
                event.append_unique(
                    "host.ip",
                    json!(
                        event
                            .get("eset_protect.event.ipv6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("eset_protect.event.ipv6") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("eset_protect.event.ipv6")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { !event.has_value("json.occurred") };
            if _cond {
                if event.has_value("json.occured") {
                    event.rename("json.occured", "json.occurred")?;
                }
            }

            let _cond = {
                event.has_value("json.occurred_plaintext")
                    && event.get_str("json.occurred_plaintext") != Some("")
            };
            if _cond {
                gsub_field(
                    event,
                    "json.occurred_plaintext",
                    "json.occurred",
                    cached_regex!("UTC([+-])([0-9])$"),
                    "UTC$10$2:00",
                )?;
            }

            let _cond =
                { event.has_value("json.occurred") && event.get_str("json.occurred") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.occurred") {
                        match parse_date_out(
                            &date_str,
                            &["dd-MMM-yyyy HH:mm:ss", "M/d/yy, h:m:s a z"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("eset_protect.event.occurred", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.occurred".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_occurred")?;
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
            }

            if let Some(v) = event
                .get("eset_protect.event.occurred")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.os_name") {
                event.rename("json.os_name", "eset_protect.event.os_name")?;
            }

            if let Some(v) = event
                .get("eset_protect.event.os_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.os.name", v)?;
            }

            if event.has_value("json.severity") {
                event.rename("json.severity", "eset_protect.event.severity")?;
            }

            if event.has_value("json.source_uuid") {
                event.rename("json.source_uuid", "eset_protect.event.source_uuid")?;
            }

            if let Some(v) = event
                .get("eset_protect.event.source_uuid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.id", v)?;
            }

            let _cond = { event.has_value("eset_protect.event.source_uuid") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("eset_protect.event.source_uuid")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.event_type") {
                event.rename("json.event_type", "eset_protect.event.type")?;
            }

            let _cond = {
                event.has_value("eset_protect.event.type")
                    && [
                        "Threat_Event",
                        "HipsAggregated_Event",
                        "FirewallAggregated_Event",
                        "FilteredWebsites_Event",
                        "BlockedFiles_Event",
                    ]
                    .contains(&event.get_str("eset_protect.event.type").unwrap_or(""))
            };
            if _cond {
                event.set("threat.indicator.provider", json!("ESET PROTECT"))?;
            }

            let _cond = {
                event.get_str("eset_protect.event.type") == Some("BlockedFiles_Event")
                    || event
                        .get_str("json.object_type")
                        .is_some_and(|s| s.to_lowercase() == "file")
            };
            if _cond {
                event.set("threat.indicator.type", json!("file"))?;
            }

            if event.has_value("json.account") {
                event.rename("json.account", "eset_protect.event.account")?;
            }

            let _cond = { event.has_value("eset_protect.event.account") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("eset_protect.event.account")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.action") {
                event.rename("json.action", "eset_protect.event.action")?;
            }

            let _cond = { event.get_str("eset_protect.event.action") != Some("") };
            if _cond {
                if let Some(v) = event
                    .get("eset_protect.event.action")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
            }

            if event.has_value("json.action_taken") {
                event.rename("json.action_taken", "eset_protect.event.action_taken")?;
            }

            let _cond = { !event.has_value("event.action") };
            if _cond {
                if let Some(v) = event
                    .get("eset_protect.event.action_taken")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        if let Some(s) = event.get_string("event.action") {
                            let mut parts: Vec<Value> = cached_regex!("\\s+")
                                .split(&s)
                                .into_iter()
                                .map(|p| json!(p))
                                .collect();
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                            event.set("event.action", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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
            }

            let _cond = { event.get("event.action").is_some_and(|v| v.is_array()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    let joined = event.get("event.action").and_then(|v| join_values(v, "-"));
                    if let Some(joined) = joined {
                        event.set("event.action", json!(joined))?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "join")?;
                    event.set("_ingest.on_failure_processor_tag", "join_event_action")?;
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
            }

            if event.has_value("json.action_error") {
                event.rename("json.action_error", "eset_protect.event.action_error")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.aggregate_count") {
                    if let Some(val) = event.get("json.aggregate_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.aggregate_count".into(),
                                message,
                            }
                        })?;
                        event.set("eset_protect.event.aggregate_count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_aggregate_count_to_long",
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

            if let Some(v) = event
                .get("eset_protect.event.aggregate_count")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.sightings", v)?;
            }

            if event.has_value("json.application") {
                event.rename("json.application", "eset_protect.event.application")?;
            }

            if event.has_value("json.cause") {
                event.rename("json.cause", "eset_protect.event.cause")?;
            }

            if let Some(v) = event
                .get("eset_protect.event.cause")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            if event.has_value("json.circumstances") {
                event.rename("json.circumstances", "eset_protect.event.circumstances")?;
            }

            if let Some(v) = event
                .get("eset_protect.event.circumstances")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.computer_severity_score") {
                    if let Some(val) = event.get("json.computer_severity_score") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.computer_severity_score".into(),
                                message,
                            }
                        })?;
                        event.set("eset_protect.event.computer_severity_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_computer_severity_score_to_long",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.count") {
                    if let Some(val) = event.get("json.count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.count".into(),
                                message,
                            }
                        })?;
                        event.set("eset_protect.event.count", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_count_to_long")?;
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

            if event.has_value("json.description") {
                event.rename("json.description", "eset_protect.event.description")?;
            }

            if event.has_value("json.detail") {
                event.rename("json.detail", "eset_protect.event.detail")?;
            }

            if let Some(v) = event
                .get("eset_protect.event.detail")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if event.has_value("json.domain") {
                event.rename("json.domain", "eset_protect.event.domain")?;
            }

            if event.has_value("json.eialarmid") {
                event.rename("json.eialarmid", "eset_protect.event.eialarmid")?;
            }

            if event.has_value("json.eiconsolelink") {
                event.rename("json.eiconsolelink", "eset_protect.event.eiconsolelink")?;
            }

            if let Some(v) = event
                .get("eset_protect.event.eiconsolelink")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reference", v)?;
            }

            if event.has_value("json.engine_version") {
                event.rename("json.engine_version", "eset_protect.event.engine_version")?;
            }

            if event.has_value("json.event") {
                event.rename("json.event", "eset_protect.event.name")?;
            }

            if event.has_value("json.trigger_event") {
                event.rename("json.trigger_event", "eset_protect.event.trigger_event")?;
            }

            if event.has_value("json.detection_uuid") {
                event.rename("json.detection_uuid", "eset_protect.event.detection_uuid")?;
            }

            if let Some(v) = event
                .get("eset_protect.event.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = {
                event.has_value("json.firstseen") && event.get_str("json.firstseen") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.firstseen") {
                        match parse_date_out(
                            &date_str,
                            &["yyyyMMdd'T'HHmmss", "dd-MMM-yyyy HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("eset_protect.event.firstseen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.firstseen".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_firstseen")?;
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
            }

            if let Some(v) = event
                .get("eset_protect.event.firstseen")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.first_seen", v)?;
            }

            if event.has_value("json.handled") {
                if let Some(val) = event.get("json.handled") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.handled".into(),
                            message,
                        }
                    })?;
                    event.set("eset_protect.event.handled", converted)?;
                }
            }

            let _cond = { event.get_str("eset_protect.event.handled") != Some("1") };
            if _cond {
                event.set("eset_protect.event.is_handled", json!(true))?;
            }

            let _cond = { event.get_str("eset_protect.event.handled") != Some("0") };
            if _cond {
                event.set("eset_protect.event.is_handled", json!(false))?;
            }

            if event.has_value("json.hash") {
                event.rename("json.hash", "eset_protect.event.hash")?;
            }

            // SKIPPED: condition not transpiled: ctx.eset_protect?.event?.type == 'BlockedFiles_Event' || 'file'.equalsIgnoreCase(ctx.json?.object_type)
            #[allow(unreachable_code, unused_variables)]
            if false {
                if let Some(v) = event
                    .get("eset_protect.event.hash")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.hash.sha1", v)?;
                }
            }

            if event.has_value("file.hash.sha1") {
                map_strings(event, "file.hash.sha1", "file.hash.sha1", str::to_lowercase)?;
            }

            // SKIPPED: condition not transpiled: ctx.eset_protect?.event?.type == 'BlockedFiles_Event' || 'file'.equalsIgnoreCase(ctx.json?.object_type)
            #[allow(unreachable_code, unused_variables)]
            if false {
                if let Some(v) = event
                    .get("file.hash.sha1")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.file.hash.sha1", v)?;
                }
            }

            let _cond = { event.has_value("eset_protect.event.hash") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("eset_protect.event.hash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("related.hash") {
                map_strings(event, "related.hash", "related.hash", str::to_lowercase)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.inbound") {
                    if let Some(val) = event.get("json.inbound") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.inbound".into(),
                                message,
                            }
                        })?;
                        event.set("eset_protect.event.inbound", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_inbound_to_boolean",
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

            let _cond = { event.get_bool("eset_protect.event.inbound") == Some(true) };
            if _cond {
                event.set("network.direction", json!("inbound"))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.need_restart") {
                    if let Some(val) = event.get("json.need_restart") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.need_restart".into(),
                                message,
                            }
                        })?;
                        event.set("eset_protect.event.need_restart", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_need_restart_to_boolean",
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

            if event.has_value("json.object_type") {
                event.rename("json.object_type", "eset_protect.event.object_type")?;
            }

            let _cond = {
                event
                    .get_str("eset_protect.event.object_type")
                    .is_some_and(|s| s.to_lowercase() == "file")
            };
            if _cond {
                event.set("file.type", json!("file"))?;
            }

            if event.has_value("json.object_uri") {
                event.rename("json.object_uri", "eset_protect.event.object_uri")?;
            }

            let _cond = {
                event.has_value("eset_protect.event.object_uri")
                    && event
                        .get("eset_protect.event.object_uri")
                        .is_some_and(|v| v.is_string())
                    && event
                        .get_str("eset_protect.event.object_uri")
                        .is_some_and(|s| s.to_lowercase().starts_with("mailto:"))
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: String uri = ctx.eset_protect.event.object_uri;\n\n// Match query parameters from the mailto: URI\n// Assumption: the from= value contains plain-text addresses in angle brackets.\n// If ESET starts URL-encoding these addresses in the future, decode before applying this matcher.\njava.util.regex.Matcher fromMatch = /(?:\\?|&)from=([^&]+)/.matcher(uri);\njava.util.regex.Matcher subjectMatch = /(?:\\?|&)subject=([^&]+)/.matcher(uri);\njava.util.regex.Matcher attachmentMatch = /(?:\\?|&)attachment=([^&]+)/.matcher(uri);\n\n// Extract RFC 5321 addresses enclosed in angle brackets from the from= value with best effort.\nList fromAddresses = new ArrayList();\nif (fromMatch.find()) {\n    java.util.regex.Matcher addressMatch = /<\\s*([A-Za-z0-9.!#$%&'*+\\/?^_`{|}~-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,})\\s*>/.matcher(fromMatch.group(1));\n    while (addressMatch.find()) {\n        fromAddresses.add(addressMatch.group(1));\n    }\n}\n\nString subject = subjectMatch.find() ? subjectMatch.group(1) : null;\nString attachment = attachmentMatch.find() ? attachmentMatch.group(1) : null;\n\n// Nothing actionable found – skip field population\nif (fromAddresses.isEmpty() && subject == null && attachment == null) {\n    return;\n}\n\n// Ensure the top-level email map exists before writing into it\nif (!(ctx.email instanceof Map)) {\n    ctx.email = new HashMap();\n}\n\n// Populate ECS email.* fields\nif (!fromAddresses.isEmpty()) {\n    Map from = ctx.email['from'] instanceof Map ? (Map) ctx.email['from'] : new HashMap();\n    from['address'] = fromAddresses;\n    ctx.email['from'] = from;\n}\nif (subject != null) {\n    ctx.email['subject'] = subject;\n}\nif (attachment != null) {\n    Map attachments = new HashMap();\n    Map file = new HashMap();\n    file['name'] = attachment;\n    attachments['file'] = file;\n    List attachmentList = new ArrayList();\n    attachmentList.add(attachments);\n    ctx.email['attachments'] = attachmentList;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"String uri = ctx.eset_protect.event.object_uri;\n\n// Match query parameters from the mailto: URI\n// Assumption: the from= value contains plain-text addresses in angle brackets.\n// If ESET starts URL-encoding these addresses in the future, decode before applying this matcher.\njava.util.regex.Matcher fromMatch = /(?:\\?|&)from=([^&]+)/.matcher(uri);\njava.util.regex.Matcher subjectMatch = /(?:\\?|&)subject=([^&]+)/.matcher(uri);\njava.util.regex.Matcher attachmentMatch = /(?:\\?|&)attachment=([^&]+)/.matcher(uri);\n\n// Extract RFC 5321 addresses enclosed in angle brackets from the from= value with best effort.\nList fromAddresses = new ArrayList();\nif (fromMatch.find()) {\n    java.util.regex.Matcher addressMatch = /<\\s*([A-Za-z0-9.!#$%&'*+\\/?^_`{|}~-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,})\\s*>/.matcher(fromMatch.group(1));\n    while (addressMatch.find()) {\n        fromAddresses.add(addressMatch.group(1));\n    }\n}\n\nString subject = subjectMatch.find() ? subjectMatch.group(1) : null;\nString attachment = attachmentMatch.find() ? attachmentMatch.group(1) : null;\n\n// Nothing actionable found – skip field population\nif (fromAddresses.isEmpty() && subject == null && attachment == null) {\n    return;\n}\n\n// Ensure the top-level email map exists before writing into it\nif (!(ctx.email instanceof Map)) {\n    ctx.email = new HashMap();\n}\n\n// Populate ECS email.* fields\nif (!fromAddresses.isEmpty()) {\n    Map from = ctx.email['from'] instanceof Map ? (Map) ctx.email['from'] : new HashMap();\n    from['address'] = fromAddresses;\n    ctx.email['from'] = from;\n}\nif (subject != null) {\n    ctx.email['subject'] = subject;\n}\nif (attachment != null) {\n    Map attachments = new HashMap();\n    Map file = new HashMap();\n    file['name'] = attachment;\n    attachments['file'] = file;\n    List attachmentList = new ArrayList();\n    attachmentList.add(attachments);\n    ctx.email['attachments'] = attachmentList;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_parse_mailto_uri",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "object_uri for mails could not be parsed: {}",
                            event
                                .get("eset_protect.event.object_uri")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.append_unique("event.kind", json!("pipeline_error"))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("eset_protect.event.object_uri")
                    && event
                        .get_str("eset_protect.event.object_type")
                        .is_some_and(|s| s.to_lowercase() == "file")
            };
            if _cond {
                if let Some(v) = event.get("eset_protect.event.object_uri").cloned() {
                    event.set("file.path", v)?;
                }
            }

            let _cond = {
                event.has_value("file.path")
                    && event.get_str("file.path") != Some("script")
                    && !(event.get("file.path").is_some_and(|v| v.is_string())
                        && event
                            .get_str("file.path")
                            .is_some_and(|s| s.to_lowercase().starts_with("mailto:")))
            };
            if _cond {
                if let Some(input) = event.get_string("file.path") {
                    // Grok pattern: ^(file:///)?(?P<file_path>(?:(?P<file_drive_letter>\\w):(?P<file_directory>/.*/|/)?(?P<file_name>.+(\\.(?P<file_extension>.+))?)?))$
                    // Grok pattern: .*/(?P<file_name>.+\\.(?P<file_extension>.+))$
                    let _ = extract_first_match(
                        &[
                            cached_grok_mapped!(
                                "^(file:///)?(?P<file_path>(?:(?P<file_drive_letter>\\w):(?P<file_directory>/.*/|/)?(?P<file_name>.+(\\.(?P<file_extension>.+))?)?))$",
                                [
                                    ("file_path", "file.path"),
                                    ("file_drive_letter", "file.drive_letter"),
                                    ("file_directory", "file.directory"),
                                    ("file_name", "file.name"),
                                    ("file_extension", "file.extension")
                                ]
                            ),
                            cached_grok_mapped!(
                                ".*/(?P<file_name>.+\\.(?P<file_extension>.+))$",
                                [
                                    ("file_name", "file.name"),
                                    ("file_extension", "file.extension")
                                ]
                            ),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            if event.has_value("json.operation") {
                event.rename("json.operation", "eset_protect.event.operation")?;
            }

            if event.has_value("json.process_name") {
                event.rename("json.process_name", "eset_protect.event.processname")?;
            }

            if event.has_value("json.processname") {
                event.rename("json.processname", "eset_protect.event.processname")?;
            }

            if let Some(v) = event
                .get("eset_protect.event.processname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            if event.has_value("json.command_line") {
                event.rename("json.command_line", "eset_protect.event.command_line")?;
            }

            let _cond = { event.get_str("eset_protect.event.processname") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("eset_protect.event.processname") {
                        if let Some(input) = event.get_string("eset_protect.event.processname") {
                            // Grok pattern: ^%{GREEDYDATA:json._temp}\\\\%{DATA:process.name}$
                            // Grok pattern: ^%{GREEDYDATA:json._temp}/%{DATA:process.name}$
                            // Grok pattern: ^%{DATA:process.name}$
                            let _ = extract_first_match(
                                &[
                                    cached_grok!(
                                        "^%{GREEDYDATA:json._temp}\\\\%{DATA:process.name}$"
                                    ),
                                    cached_grok!("^%{GREEDYDATA:json._temp}/%{DATA:process.name}$"),
                                    cached_grok!("^%{DATA:process.name}$"),
                                ],
                                &input,
                                event,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set("_ingest.on_failure_processor_tag", "grok_processname")?;
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
            }

            if event.has_value("json.protocol") {
                event.rename("json.protocol", "eset_protect.event.protocol")?;
            }

            if let Some(v) = event
                .get("eset_protect.event.protocol")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("network.protocol", v)?;
            }

            if event.has_value("network.protocol") {
                map_strings(
                    event,
                    "network.protocol",
                    "network.protocol",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("json.result") {
                event.rename("json.result", "eset_protect.event.result")?;
            }

            let _cond = { event.get_str("eset_protect.event.result") == Some("Success") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("eset_protect.event.result") == Some("Failed") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            if event.has_value("json.rule_id") {
                event.rename("json.rule_id", "eset_protect.event.rule_id")?;
            }

            if event.has_value("json.rule_name") {
                event.rename("json.rule_name", "eset_protect.event.rulename")?;
            }

            if event.has_value("json.rulename") {
                event.rename("json.rulename", "eset_protect.event.rulename")?;
            }

            let _cond = { event.get_str("eset_protect.event.rulename") != Some("") };
            if _cond {
                if let Some(v) = event
                    .get("eset_protect.event.rulename")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.name", v)?;
                }
            }

            let _cond = { !event.has_value("rule.name") };
            if _cond {
                if let Some(v) = event
                    .get("eset_protect.event.rule_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("rule.name", v)?;
                }
            }

            if event.has_value("json.scan_id") {
                event.rename("json.scan_id", "eset_protect.event.scan_id")?;
            }

            if event.has_value("json.scanner_id") {
                event.rename("json.scanner_id", "eset_protect.event.scanner_id")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.severity_score") {
                    if let Some(val) = event.get("json.severity_score") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.severity_score".into(),
                                message,
                            }
                        })?;
                        event.set("eset_protect.event.severity_score", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_severity_score_to_long",
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

            if let Some(v) = event
                .get("eset_protect.event.severity_score")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.severity", v)?;
            }

            let _cond = {
                event.has_value("json.source_address")
                    && event.get_str("json.source_address") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.source_address") {
                        if let Some(val) = event.get("json.source_address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.source_address".into(),
                                    message,
                                }
                            })?;
                            event.set("eset_protect.event.source_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_source_address_to_ip",
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
            }

            if let Some(v) = event
                .get("eset_protect.event.source_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            let _cond = { event.has_value("eset_protect.event.source_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("eset_protect.event.source_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.source_address_type") {
                event.rename(
                    "json.source_address_type",
                    "eset_protect.event.source_address_type",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.source_port") {
                    if let Some(val) = event.get("json.source_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.source_port".into(),
                                message,
                            }
                        })?;
                        event.set("eset_protect.event.source_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_source_port_to_long",
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

            if let Some(v) = event
                .get("eset_protect.event.source_port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if event.has_value("json.target") {
                event.rename("json.target", "eset_protect.event.target")?;
            }

            let _cond = {
                event.has_value("json.target_address")
                    && event.get_str("json.target_address") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.target_address") {
                        if let Some(val) = event.get("json.target_address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.target_address".into(),
                                    message,
                                }
                            })?;
                            event.set("eset_protect.event.target_address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_target_address_to_ip",
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
            }

            if let Some(v) = event
                .get("eset_protect.event.target_address")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
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

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            let _cond = { event.has_value("eset_protect.event.target_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("eset_protect.event.target_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.target_address_type") {
                event.rename(
                    "json.target_address_type",
                    "eset_protect.event.target_address_type",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.target_port") {
                    if let Some(val) = event.get("json.target_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.target_port".into(),
                                message,
                            }
                        })?;
                        event.set("eset_protect.event.target_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_target_port_to_long",
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

            if let Some(v) = event
                .get("eset_protect.event.target_port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("json.threat_flags") {
                event.rename("json.threat_flags", "eset_protect.event.threat_flags")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threat_handled") {
                    if let Some(val) = event.get("json.threat_handled") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threat_handled".into(),
                                message,
                            }
                        })?;
                        event.set("eset_protect.event.threat_handled", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_threat_handled_to_boolean",
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

            if event.has_value("json.threat_name") {
                event.rename("json.threat_name", "eset_protect.event.threat_name")?;
            }

            if let Some(v) = event
                .get("eset_protect.event.threat_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            if event.has_value("json.threat_type") {
                event.rename("json.threat_type", "eset_protect.event.threat_type")?;
            }

            if event.has_value("json.user") {
                event.rename("json.user", "eset_protect.event.username")?;
            }

            if event.has_value("json.username") {
                event.rename("json.username", "eset_protect.event.username")?;
            }

            let _cond = { event.get_str("eset_protect.event.username") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("eset_protect.event.username") {
                        if let Some(input) = event.get_string("eset_protect.event.username") {
                            // Grok pattern: ^%{HOSTNAME:user.domain}\\\\%{USERNAME:user.name}$
                            // Grok pattern: ^%{HOSTNAME:user.domain}\\\\\\\\%{USERNAME:user.name}$
                            // Grok pattern: ^%{USERNAME:user.name}@%{HOSTNAME:user.domain}$
                            // Grok pattern: ^%{GREEDYDATA:user.name}$
                            let _ = extract_first_match(
                                &[
                                    cached_grok!(
                                        "^%{HOSTNAME:user.domain}\\\\%{USERNAME:user.name}$"
                                    ),
                                    cached_grok!(
                                        "^%{HOSTNAME:user.domain}\\\\\\\\%{USERNAME:user.name}$"
                                    ),
                                    cached_grok!("^%{USERNAME:user.name}@%{HOSTNAME:user.domain}$"),
                                    cached_grok!("^%{GREEDYDATA:user.name}$"),
                                ],
                                &input,
                                event,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set("_ingest.on_failure_processor_tag", "grok_user_name")?;
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
                event.remove("eset_protect.event.group_name");
                event.remove("eset_protect.event.hostname");
                event.remove("eset_protect.event.ipv4");
                event.remove("eset_protect.event.ipv6");
                event.remove("eset_protect.event.occurred");
                event.remove("eset_protect.event.os_name");
                event.remove("eset_protect.event.source_uuid");
                event.remove("eset_protect.event.aggregate_count");
                event.remove("eset_protect.event.cause");
                event.remove("eset_protect.event.circumstances");
                event.remove("eset_protect.event.detail");
                event.remove("eset_protect.event.eiconsolelink");
                event.remove("eset_protect.event.name");
                event.remove("eset_protect.event.firstseen");
                event.remove("eset_protect.event.processname");
                event.remove("eset_protect.event.protocol");
                event.remove("eset_protect.event.severity_score");
                event.remove("eset_protect.event.source_port");
                event.remove("eset_protect.event.target_port");
                event.remove("eset_protect.event.threat_name");
            }

            event.remove("json");

            // Painless script
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
                ),
            )?;

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
