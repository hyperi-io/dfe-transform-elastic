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

            event.set("event.kind", json!("asset"))?;

            event.append("event.category", json!("host"))?;

            event.append("event.type", json!("info"))?;

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

            let _cond = {
                event.has_value("event.original")
                    && event.get("event.original").is_some_and(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_message")?;
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

            if event.has_value("json.accountID") {
                event.rename("json.accountID", "prisma_cloud.host_profile.account_id")?;
            }

            if let Some(v) = event
                .get("prisma_cloud.host_profile.account_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            if event.has_value("json._id") {
                event.rename("json._id", "prisma_cloud.host_profile._id")?;
            }

            if let Some(v) = event
                .get("prisma_cloud.host_profile._id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("prisma_cloud.host_profile._id") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("prisma_cloud.host_profile._id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.collections") {
                event.rename("json.collections", "prisma_cloud.host_profile.collections")?;
            }

            let _cond =
                { event.has_value("json.created") && event.get_str("json.created") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.created") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("prisma_cloud.host_profile.created", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.created".into(),
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
                        "date_prisma_cloud_host_profile_created",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
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
                .get("prisma_cloud.host_profile.created")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if event.has_value("json.labels") {
                event.rename("json.labels", "prisma_cloud.host_profile.labels")?;
            }

            if let Some(v) = event
                .get("prisma_cloud.host_profile.labels")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("labels", v)?;
            }

            if event.has_value("json.geoip.countries") {
                event.rename(
                    "json.geoip.countries",
                    "prisma_cloud.host_profile.geoip.countries",
                )?;
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.geoip.countries")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.host_profile.geoip.countries",
                        |event| {
                            // on_failure: 2 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if event.has_value("_ingest._value.ip") {
                                    if let Some(val) = event.get("_ingest._value.ip") {
                                        let converted =
                                            convert_value(val, "ip").map_err(|message| {
                                                TransformError::ParseError {
                                                    path: "_ingest._value.ip".into(),
                                                    message,
                                                }
                                            })?;
                                        event.set("_ingest._value.ip", converted)?;
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "convert")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "convert_prisma_cloud_host_profile_ip_to_ip",
                                )?;
                                event.remove("_ingest._value.ip");
                                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.geoip.countries")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(
                        event,
                        "prisma_cloud.host_profile.geoip.countries",
                        |event| {
                            event.append_unique(
                                "related.ip",
                                json!(
                                    event
                                        .get("_ingest._value.ip")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        },
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.geoip.countries")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event
                            .get("prisma_cloud.host_profile.geoip.countries")
                            .cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.modified")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.modified", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.modified".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "date")?;
                                    event.remove("_ingest._value.modified");
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "prisma_cloud.host_profile.geoip.countries",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("json.geoip.modified")
                    && event.get_str("json.geoip.modified") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.geoip.modified") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("prisma_cloud.host_profile.geoip.modified", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.geoip.modified".into(),
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
                        "date_prisma_cloud_host_profile_modified",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
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

            let _cond = { event.get_str("json.hash") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.hash") {
                        if let Some(val) = event.get("json.hash") {
                            let converted = convert_value(val, "string").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.hash".into(),
                                    message,
                                }
                            })?;
                            event.set("prisma_cloud.host_profile.hash", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_prisma_cloud_host_profile_hash_keyword",
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

            event.append_unique(
                "related.hash",
                json!(
                    event
                        .get("prisma_cloud.host_profile.hash")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.apps") {
                event.rename("json.apps", "prisma_cloud.host_profile.apps")?;
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        if event.has_value("_ingest._value.startupProcess") {
                            event.rename(
                                "_ingest._value.startupProcess",
                                "_ingest._value.startup_process",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        event.append_unique(
                            "process.command_line",
                            json!(
                                event
                                    .get("_ingest._value.startup_process.command")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.startup_process.interactive") {
                                if let Some(val) =
                                    event.get("_ingest._value.startup_process.interactive")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.startup_process.interactive"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.startup_process.interactive",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_prisma_cloud_host_profile_apps_startupProcess_interactive_to_interactive")?;
                            event.remove("_ingest._value.startup_process.interactive");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        event.append_unique(
                            "process.interactive",
                            json!(
                                event
                                    .get("_ingest._value.startup_process.interactive")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        event.append_unique(
                            "process.hash.md5",
                            json!(
                                event
                                    .get("_ingest._value.startup_process.md5")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value.startup_process.md5")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.startup_process.modified") {
                                if let Some(val) =
                                    event.get("_ingest._value.startup_process.modified")
                                {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.startup_process.modified"
                                                    .into(),
                                                message,
                                            }
                                        })?;
                                    event.set(
                                        "_ingest._value.startup_process.modified",
                                        converted,
                                    )?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set("_ingest.on_failure_processor_tag", "convert_prisma_cloud_host_profile_apps_startupProcess_modified_to_modified")?;
                            event.remove("_ingest._value.startup_process.modified");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("prisma_cloud.host_profile.apps").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.startup_process.time")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => event.set(
                                                "_ingest._value.startup_process.time",
                                                parsed,
                                            )?,
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.startup_process.time"
                                                        .into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "date")?;
                                    event.remove("_ingest._value.startup_process.time");
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "prisma_cloud.host_profile.apps",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        event.remove("_ingest._value.startup_process.command");
                        event.remove("_ingest._value.startup_process.interactive");
                        event.remove("_ingest._value.startup_process.md5");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        if event.has_value("_ingest._value.outgoingPorts") {
                            event.rename(
                                "_ingest._value.outgoingPorts",
                                "_ingest._value.outgoing_ports",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.outgoing_ports", |event| {
                                if event.has_value("_ingest._value.processPath") {
                                    event.rename(
                                        "_ingest._value.processPath",
                                        "_ingest._value.process_path",
                                    )?;
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.outgoing_ports", |event| {
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.ip") {
                                        if let Some(val) = event.get("_ingest._value.ip") {
                                            let converted =
                                                convert_value(val, "ip").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.ip".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.ip", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    event.set("_ingest.on_failure_processor_tag", "convert_prisma_cloud_host_profile_apps_outgoing_ports_ip_to_ip")?;
                                    event.remove("_ingest._value.ip");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.outgoing_ports", |event| {
                                event.append_unique(
                                    "related.ip",
                                    json!(
                                        event
                                            .get("_ingest._value.ip")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.outgoing_ports", |event| {
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.port") {
                                        if let Some(val) = event.get("_ingest._value.port") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.port".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.port", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    event.set(
                                        "_ingest.on_failure_processor_tag",
                                        "convert_prisma_cloud_host_profile_outgoing_ports_port",
                                    )?;
                                    event.remove("_ingest._value.port");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("prisma_cloud.host_profile.apps").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject =
                                            event.get("_ingest._value.outgoing_ports").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
                                            _ => Vec::new(),
                                        };
                                        if !entries.is_empty() {
                                            // A NESTED loop borrows the same slots, so the enclosing
                                            // entry is saved and put back afterwards.
                                            let enclosing = event.get("_ingest._value").cloned();
                                            let enclosing_key = event.get("_ingest._key").cloned();
                                            let mut list = Vec::with_capacity(entries.len());
                                            let mut fields = Map::new();
                                            for (key, item) in entries {
                                                if let Some(key) = key.as_deref() {
                                                    event.set(
                                                        "_ingest._key",
                                                        Value::String(key.to_string()),
                                                    )?;
                                                }
                                                event.set("_ingest._value", item)?;
                                                // on_failure: 1 handler(s)
                                                if let Err(err) = (|| -> Result<()> {
                                                    if let Some(date_str) = event
                                                        .get_as_string("_ingest._value.modified")
                                                    {
                                                        match parse_date_out(
                                                            &date_str,
                                                            &["ISO8601"],
                                                            None,
                                                            None,
                                                        ) {
                                                            Some(parsed) => event.set(
                                                                "_ingest._value.modified",
                                                                parsed,
                                                            )?,
                                                            None => {
                                                                return Err(TransformError::ParseError {
                            path: "_ingest._value.modified".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                                                            }
                                                        }
                                                    }
                                                    Ok(())
                                                })(
                                                ) {
                                                    event.set(
                                                        "_ingest.on_failure_message",
                                                        err.to_string(),
                                                    )?;
                                                    event.set(
                                                        "_ingest.on_failure_processor_type",
                                                        "date",
                                                    )?;
                                                    event.remove("_ingest._value.modified");
                                                    event.remove("_ingest.on_failure_message");
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
                                                    if event
                                                        .get_object("_ingest")
                                                        .is_some_and(|m| m.is_empty())
                                                    {
                                                        event.remove("_ingest");
                                                    }
                                                }
                                                let left = event.remove("_ingest._value");
                                                match key {
                                                    // An entry the body renamed AWAY is gone from the
                                                    // object, which is how a foreach lifts fields up.
                                                    Some(key) => {
                                                        if let Some(value) = left {
                                                            fields.insert(key, value);
                                                        }
                                                    }
                                                    None => list.push(left.unwrap_or(Value::Null)),
                                                }
                                            }
                                            match enclosing {
                                                Some(previous) => {
                                                    event.set("_ingest._value", previous)?;
                                                }
                                                None => {
                                                    event.remove("_ingest");
                                                }
                                            }
                                            if let Some(previous) = enclosing_key {
                                                event.set("_ingest._key", previous)?;
                                            }
                                            event.set(
                                                "_ingest._value.outgoing_ports",
                                                if keyed {
                                                    Value::Object(fields)
                                                } else {
                                                    Value::Array(list)
                                                },
                                            )?;
                                        }
                                    }
                                    Ok(())
                                })();
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "prisma_cloud.host_profile.apps",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        if event.has_value("_ingest._value.listeningPorts") {
                            event.rename(
                                "_ingest._value.listeningPorts",
                                "_ingest._value.listening_ports",
                            )?;
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.listening_ports", |event| {
                                if event.has_value("_ingest._value.processPath") {
                                    event.rename(
                                        "_ingest._value.processPath",
                                        "_ingest._value.process_path",
                                    )?;
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("prisma_cloud.host_profile.apps").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject =
                                            event.get("_ingest._value.listening_ports").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
                                            _ => Vec::new(),
                                        };
                                        if !entries.is_empty() {
                                            // A NESTED loop borrows the same slots, so the enclosing
                                            // entry is saved and put back afterwards.
                                            let enclosing = event.get("_ingest._value").cloned();
                                            let enclosing_key = event.get("_ingest._key").cloned();
                                            let mut list = Vec::with_capacity(entries.len());
                                            let mut fields = Map::new();
                                            for (key, item) in entries {
                                                if let Some(key) = key.as_deref() {
                                                    event.set(
                                                        "_ingest._key",
                                                        Value::String(key.to_string()),
                                                    )?;
                                                }
                                                event.set("_ingest._value", item)?;
                                                // on_failure: 1 handler(s)
                                                if let Err(err) = (|| -> Result<()> {
                                                    if let Some(date_str) = event
                                                        .get_as_string("_ingest._value.modified")
                                                    {
                                                        match parse_date_out(
                                                            &date_str,
                                                            &["ISO8601"],
                                                            None,
                                                            None,
                                                        ) {
                                                            Some(parsed) => event.set(
                                                                "_ingest._value.modified",
                                                                parsed,
                                                            )?,
                                                            None => {
                                                                return Err(TransformError::ParseError {
                            path: "_ingest._value.modified".into(),
                            message: format!("unable to parse date [{date_str}]"),
                            });
                                                            }
                                                        }
                                                    }
                                                    Ok(())
                                                })(
                                                ) {
                                                    event.set(
                                                        "_ingest.on_failure_message",
                                                        err.to_string(),
                                                    )?;
                                                    event.set(
                                                        "_ingest.on_failure_processor_type",
                                                        "date",
                                                    )?;
                                                    event.remove("_ingest._value.modified");
                                                    event.remove("_ingest.on_failure_message");
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
                                                    if event
                                                        .get_object("_ingest")
                                                        .is_some_and(|m| m.is_empty())
                                                    {
                                                        event.remove("_ingest");
                                                    }
                                                }
                                                let left = event.remove("_ingest._value");
                                                match key {
                                                    // An entry the body renamed AWAY is gone from the
                                                    // object, which is how a foreach lifts fields up.
                                                    Some(key) => {
                                                        if let Some(value) = left {
                                                            fields.insert(key, value);
                                                        }
                                                    }
                                                    None => list.push(left.unwrap_or(Value::Null)),
                                                }
                                            }
                                            match enclosing {
                                                Some(previous) => {
                                                    event.set("_ingest._value", previous)?;
                                                }
                                                None => {
                                                    event.remove("_ingest");
                                                }
                                            }
                                            if let Some(previous) = enclosing_key {
                                                event.set("_ingest._key", previous)?;
                                            }
                                            event.set(
                                                "_ingest._value.listening_ports",
                                                if keyed {
                                                    Value::Object(fields)
                                                } else {
                                                    Value::Array(list)
                                                },
                                            )?;
                                        }
                                    }
                                    Ok(())
                                })();
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "prisma_cloud.host_profile.apps",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.listening_ports", |event| {
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.port") {
                                        if let Some(val) = event.get("_ingest._value.port") {
                                            let converted =
                                                convert_value(val, "long").map_err(|message| {
                                                    TransformError::ParseError {
                                                        path: "_ingest._value.port".into(),
                                                        message,
                                                    }
                                                })?;
                                            event.set("_ingest._value.port", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    event.set("_ingest.on_failure_processor_tag", "convert_prisma_cloud_host_profile_listening_ports_port_to_long")?;
                                    event.remove("_ingest._value.port");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.processes", |event| {
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.interactive") {
                                        if let Some(val) = event.get("_ingest._value.interactive") {
                                            let converted = convert_value(val, "boolean").map_err(
                                                |message| TransformError::ParseError {
                                                    path: "_ingest._value.interactive".into(),
                                                    message,
                                                },
                                            )?;
                                            event.set("_ingest._value.interactive", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    event.set("_ingest.on_failure_processor_tag", "convert_prisma_cloud_host_profile_listening_ports_interactive")?;
                                    event.remove("_ingest._value.interactive");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.processes", |event| {
                                event.append_unique(
                                    "process.interactive",
                                    json!(
                                        event
                                            .get("_ingest._value.interactive")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("prisma_cloud.host_profile.apps").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // ignore_failure: true
                                let _ = (|| -> Result<()> {
                                    {
                                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                                        // binds `_ingest._key` per entry, which is what a target of
                                        // `<field>.{{{_ingest._key}}}` reads.
                                        let subject =
                                            event.get("_ingest._value.processes").cloned();
                                        let keyed = matches!(subject, Some(Value::Object(_)));
                                        let entries: Vec<(Option<String>, Value)> = match subject {
                                            Some(Value::Array(items)) => {
                                                items.into_iter().map(|v| (None, v)).collect()
                                            }
                                            Some(Value::Object(fields)) => fields
                                                .into_iter()
                                                .map(|(k, v)| (Some(k), v))
                                                .collect(),
                                            _ => Vec::new(),
                                        };
                                        if !entries.is_empty() {
                                            // A NESTED loop borrows the same slots, so the enclosing
                                            // entry is saved and put back afterwards.
                                            let enclosing = event.get("_ingest._value").cloned();
                                            let enclosing_key = event.get("_ingest._key").cloned();
                                            let mut list = Vec::with_capacity(entries.len());
                                            let mut fields = Map::new();
                                            for (key, item) in entries {
                                                if let Some(key) = key.as_deref() {
                                                    event.set(
                                                        "_ingest._key",
                                                        Value::String(key.to_string()),
                                                    )?;
                                                }
                                                event.set("_ingest._value", item)?;
                                                // on_failure: 1 handler(s)
                                                if let Err(err) = (|| -> Result<()> {
                                                    if let Some(date_str) =
                                                        event.get_as_string("_ingest._value.time")
                                                    {
                                                        match parse_date_out(
                                                            &date_str,
                                                            &["ISO8601"],
                                                            None,
                                                            None,
                                                        ) {
                                                            Some(parsed) => event.set(
                                                                "_ingest._value.time",
                                                                parsed,
                                                            )?,
                                                            None => {
                                                                return Err(
                                                                    TransformError::ParseError {
                                                                        path: "_ingest._value.time"
                                                                            .into(),
                                                                        message: format!(
                                                                            "unable to parse date [{date_str}]"
                                                                        ),
                                                                    },
                                                                );
                                                            }
                                                        }
                                                    }
                                                    Ok(())
                                                })(
                                                ) {
                                                    event.set(
                                                        "_ingest.on_failure_message",
                                                        err.to_string(),
                                                    )?;
                                                    event.set(
                                                        "_ingest.on_failure_processor_type",
                                                        "date",
                                                    )?;
                                                    event.remove("_ingest._value.time");
                                                    event.remove("_ingest.on_failure_message");
                                                    event.remove(
                                                        "_ingest.on_failure_processor_type",
                                                    );
                                                    event
                                                        .remove("_ingest.on_failure_processor_tag");
                                                    if event
                                                        .get_object("_ingest")
                                                        .is_some_and(|m| m.is_empty())
                                                    {
                                                        event.remove("_ingest");
                                                    }
                                                }
                                                let left = event.remove("_ingest._value");
                                                match key {
                                                    // An entry the body renamed AWAY is gone from the
                                                    // object, which is how a foreach lifts fields up.
                                                    Some(key) => {
                                                        if let Some(value) = left {
                                                            fields.insert(key, value);
                                                        }
                                                    }
                                                    None => list.push(left.unwrap_or(Value::Null)),
                                                }
                                            }
                                            match enclosing {
                                                Some(previous) => {
                                                    event.set("_ingest._value", previous)?;
                                                }
                                                None => {
                                                    event.remove("_ingest");
                                                }
                                            }
                                            if let Some(previous) = enclosing_key {
                                                event.set("_ingest._key", previous)?;
                                            }
                                            event.set(
                                                "_ingest._value.processes",
                                                if keyed {
                                                    Value::Object(fields)
                                                } else {
                                                    Value::Array(list)
                                                },
                                            )?;
                                        }
                                    }
                                    Ok(())
                                })();
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "prisma_cloud.host_profile.apps",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.processes", |event| {
                                // on_failure: 2 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if event.has_value("_ingest._value.modified") {
                                        if let Some(val) = event.get("_ingest._value.modified") {
                                            let converted = convert_value(val, "boolean").map_err(
                                                |message| TransformError::ParseError {
                                                    path: "_ingest._value.modified".into(),
                                                    message,
                                                },
                                            )?;
                                            event.set("_ingest._value.modified", converted)?;
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "convert")?;
                                    event.set("_ingest.on_failure_processor_tag", "convert_prisma_cloud_host_profile_apps_processes_modified_to_modified")?;
                                    event.remove("_ingest._value.modified");
                                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.processes", |event| {
                                event.append_unique(
                                    "process.hash.md5",
                                    json!(
                                        event
                                            .get("_ingest._value.md5")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.processes", |event| {
                                event.append_unique(
                                    "related.hash",
                                    json!(
                                        event
                                            .get("_ingest._value.md5")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.processes", |event| {
                                event.append_unique(
                                    "process.command_line",
                                    json!(
                                        event
                                            .get("_ingest._value.command")
                                            .map_or_else(String::new, template_to_string)
                                    ),
                                )?;
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.apps")
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.apps", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            foreach_array(event, "_ingest._value.processes", |event| {
                                event.remove("_ingest._value.command");
                                event.remove("_ingest._value.interactive");
                                event.remove("_ingest._value.md5");
                                Ok(())
                            })?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            if event.has_value("json.sshEvents") {
                event.rename("json.sshEvents", "prisma_cloud.host_profile.ssh_events")?;
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.ssh_events")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.ssh_events", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.interactive") {
                                if let Some(val) = event.get("_ingest._value.interactive") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.interactive".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.interactive", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_prisma_cloud_host_profile_ssh_events_interactive",
                            )?;
                            event.remove("_ingest._value.interactive");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.ssh_events")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.ssh_events", |event| {
                        event.append_unique(
                            "process.interactive",
                            json!(
                                event
                                    .get("_ingest._value.interactive")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.ssh_events")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.ssh_events", |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if event.has_value("_ingest._value.modified") {
                                if let Some(val) = event.get("_ingest._value.modified") {
                                    let converted =
                                        convert_value(val, "boolean").map_err(|message| {
                                            TransformError::ParseError {
                                                path: "_ingest._value.modified".into(),
                                                message,
                                            }
                                        })?;
                                    event.set("_ingest._value.modified", converted)?;
                                }
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_prisma_cloud_host_profile_ssh_events_modified_to_modified",
                            )?;
                            event.remove("_ingest._value.modified");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.ssh_events")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.ssh_events", |event| {
                        event.append_unique(
                            "process.hash.md5",
                            json!(
                                event
                                    .get("_ingest._value.md5")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.ssh_events")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.ssh_events", |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value.md5")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.ssh_events")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.ssh_events", |event| {
                        event.append_unique(
                            "process.command_line",
                            json!(
                                event
                                    .get("_ingest._value.command")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.ssh_events")
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.ssh_events", |event| {
                        event.remove("_ingest._value.command");
                        event.remove("_ingest._value.interactive");
                        event.remove("_ingest._value.md5");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.ssh_events")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("prisma_cloud.host_profile.ssh_events").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.loginTime")
                                    {
                                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.login_time", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.loginTime".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "date")?;
                                    event.remove("_ingest._value.loginTime");
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "prisma_cloud.host_profile.ssh_events",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.ssh_events")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    foreach_array(event, "prisma_cloud.host_profile.ssh_events", |event| {
                        event.remove("_ingest._value.loginTime");
                        Ok(())
                    })?;
                    Ok(())
                })();
            }

            let _cond = {
                event
                    .get("prisma_cloud.host_profile.ssh_events")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("prisma_cloud.host_profile.ssh_events").cloned();
                        let keyed = matches!(subject, Some(Value::Object(_)));
                        let entries: Vec<(Option<String>, Value)> = match subject {
                            Some(Value::Array(items)) => {
                                items.into_iter().map(|v| (None, v)).collect()
                            }
                            Some(Value::Object(fields)) => {
                                fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                            }
                            _ => Vec::new(),
                        };
                        if !entries.is_empty() {
                            // A NESTED loop borrows the same slots, so the enclosing
                            // entry is saved and put back afterwards.
                            let enclosing = event.get("_ingest._value").cloned();
                            let enclosing_key = event.get("_ingest._key").cloned();
                            let mut list = Vec::with_capacity(entries.len());
                            let mut fields = Map::new();
                            for (key, item) in entries {
                                if let Some(key) = key.as_deref() {
                                    event.set("_ingest._key", Value::String(key.to_string()))?;
                                }
                                event.set("_ingest._value", item)?;
                                // on_failure: 1 handler(s)
                                if let Err(err) = (|| -> Result<()> {
                                    if let Some(date_str) =
                                        event.get_as_string("_ingest._value.time")
                                    {
                                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                            Some(parsed) => {
                                                event.set("_ingest._value.time", parsed)?
                                            }
                                            None => {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value.time".into(),
                                                    message: format!(
                                                        "unable to parse date [{date_str}]"
                                                    ),
                                                });
                                            }
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "date")?;
                                    event.remove("_ingest._value.time");
                                    event.remove("_ingest.on_failure_message");
                                    event.remove("_ingest.on_failure_processor_type");
                                    event.remove("_ingest.on_failure_processor_tag");
                                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                        event.remove("_ingest");
                                    }
                                }
                                let left = event.remove("_ingest._value");
                                match key {
                                    // An entry the body renamed AWAY is gone from the
                                    // object, which is how a foreach lifts fields up.
                                    Some(key) => {
                                        if let Some(value) = left {
                                            fields.insert(key, value);
                                        }
                                    }
                                    None => list.push(left.unwrap_or(Value::Null)),
                                }
                            }
                            match enclosing {
                                Some(previous) => {
                                    event.set("_ingest._value", previous)?;
                                }
                                None => {
                                    event.remove("_ingest");
                                }
                            }
                            if let Some(previous) = enclosing_key {
                                event.set("_ingest._key", previous)?;
                            }
                            event.set(
                                "prisma_cloud.host_profile.ssh_events",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("json.time") && event.get_str("json.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("prisma_cloud.host_profile.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.time".into(),
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
                        "date_rename_time_to_custom_name",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
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
                event.remove("prisma_cloud.host_profile.account_id");
                event.remove("prisma_cloud.host_profile._id");
                event.remove("prisma_cloud.host_profile.created");
                event.remove("prisma_cloud.host_profile.labels");
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

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
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
                        "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
