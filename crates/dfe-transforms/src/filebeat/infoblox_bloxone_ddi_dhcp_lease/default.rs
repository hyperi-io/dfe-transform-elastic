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

            event.set("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("network")]))?;

            event.set("event.type", Value::Array(vec![json!("protocol")]))?;

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

            let _cond = {
                event.get("json.results").is_some_and(|v| v.is_array()) && event.get("json.results").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.ends") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.last_updated") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.starts") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.get_str("json.address") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.address") {
                        if let Some(val) = event.get("json.address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.address".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dhcp_lease.address", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("infoblox_bloxone_ddi.dhcp_lease.address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                Ok(())
            })();

            if event.has_value("json.client_id") {
                event.rename(
                    "json.client_id",
                    "infoblox_bloxone_ddi.dhcp_lease.client_id",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("infoblox_bloxone_ddi.dhcp_lease.client_id")
                    .cloned()
                {
                    event.set("client.user.id", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("json.ends") && event.get_str("json.ends") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.ends") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("infoblox_bloxone_ddi.dhcp_lease.ends", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.ends".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("infoblox_bloxone_ddi.dhcp_lease.ends").cloned() {
                    event.set("event.end", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.fingerprint") {
                event.rename(
                    "json.fingerprint",
                    "infoblox_bloxone_ddi.dhcp_lease.fingerprint.value",
                )?;
            }

            if event.has_value("json.fingerprint_processed") {
                event.rename(
                    "json.fingerprint_processed",
                    "infoblox_bloxone_ddi.dhcp_lease.fingerprint.processed",
                )?;
            }

            if event.has_value("json.ha_group") {
                event.rename("json.ha_group", "infoblox_bloxone_ddi.dhcp_lease.ha_group")?;
            }

            if event.has_value("json.hardware") {
                gsub_field(
                    event,
                    "json.hardware",
                    "json.hardware",
                    cached_regex!("[:.]"),
                    "-",
                )?;
            }

            if event.has_value("json.hardware") {
                map_strings(event, "json.hardware", "json.hardware", str::to_uppercase)?;
            }

            if event.has_value("json.hardware") {
                event.rename("json.hardware", "infoblox_bloxone_ddi.dhcp_lease.hardware")?;
            }

            if event.has_value("json.host") {
                event.rename("json.host", "infoblox_bloxone_ddi.dhcp_lease.host")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("infoblox_bloxone_ddi.dhcp_lease.host").cloned() {
                    event.set("host.name", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("host.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("json.hostname") {
                event.rename("json.hostname", "infoblox_bloxone_ddi.dhcp_lease.hostname")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("infoblox_bloxone_ddi.dhcp_lease.hostname")
                    .cloned()
                {
                    event.set("host.hostname", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("host.hostname") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.hostname")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("json.iaid") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.iaid") {
                        if let Some(val) = event.get("json.iaid") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.iaid".into(),
                                    message,
                                }
                            })?;
                            event.set("infoblox_bloxone_ddi.dhcp_lease.iaid", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
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
                event.has_value("json.last_updated")
                    && event.get_str("json.last_updated") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.last_updated") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("infoblox_bloxone_ddi.dhcp_lease.last_updated", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.last_updated".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("infoblox_bloxone_ddi.dhcp_lease.last_updated")
                    .cloned()
                {
                    event.set("@timestamp", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.options") {
                event.rename("json.options", "infoblox_bloxone_ddi.dhcp_lease.options")?;
            }

            let _cond = {
                event.has_value("json.preferred_lifetime")
                    && event.get_str("json.preferred_lifetime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.preferred_lifetime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "infoblox_bloxone_ddi.dhcp_lease.preferred_lifetime",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.preferred_lifetime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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

            let _cond = { event.get_str("json.protocol") == Some("ip4") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("json.protocol", json!("ipv4"))?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("json.protocol") == Some("ip6") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("json.protocol", json!("ipv6"))?;
                    Ok(())
                })();
            }

            if event.has_value("json.protocol") {
                event.rename("json.protocol", "infoblox_bloxone_ddi.dhcp_lease.protocol")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event
                    .get("infoblox_bloxone_ddi.dhcp_lease.protocol")
                    .cloned()
                {
                    event.set("network.type", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(event, "network.type", "network.type", str::to_lowercase)?;
                Ok(())
            })();

            if event.has_value("json.space") {
                event.rename("json.space", "infoblox_bloxone_ddi.dhcp_lease.space")?;
            }

            let _cond =
                { event.has_value("json.starts") && event.get_str("json.starts") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.starts") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("infoblox_bloxone_ddi.dhcp_lease.starts", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.starts".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("infoblox_bloxone_ddi.dhcp_lease.starts").cloned() {
                    event.set("event.start", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.state") {
                event.rename("json.state", "infoblox_bloxone_ddi.dhcp_lease.state")?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "infoblox_bloxone_ddi.dhcp_lease.type")?;
            }

            let _cond = {
                event
                    .get("infoblox_bloxone_ddi.dhcp_lease.options")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "infoblox_bloxone_ddi.dhcp_lease.options",
                        "infoblox_bloxone_ddi.dhcp_lease.options",
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event
                            .remove("infoblox_bloxone_ddi.dhcp_lease.options")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "infoblox_bloxone_ddi.dhcp_lease.options".into(),
                            });
                        }
                        Ok(())
                    })();
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

            event.remove("json");

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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("infoblox_bloxone_ddi.dhcp_lease.last_updated");
                    event.remove("infoblox_bloxone_ddi.dhcp_lease.client_id");
                    event.remove("infoblox_bloxone_ddi.dhcp_lease.ends");
                    event.remove("infoblox_bloxone_ddi.dhcp_lease.starts");
                    event.remove("infoblox_bloxone_ddi.dhcp_lease.hostname");
                    event.remove("infoblox_bloxone_ddi.dhcp_lease.host");
                    event.remove("infoblox_bloxone_ddi.dhcp_lease.protocol");
                    Ok(())
                })();
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
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
