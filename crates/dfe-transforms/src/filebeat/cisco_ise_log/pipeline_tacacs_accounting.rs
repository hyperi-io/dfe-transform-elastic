// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_tacacs_accounting` pipeline.
pub struct PipelineTacacsAccounting;

impl Transform for PipelineTacacsAccounting {
    fn name(&self) -> &str {
        "pipeline_tacacs_accounting"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            event.append("event.category", json!("configuration"))?;

            event.append("event.type", json!("info"))?;

            let _cond = { event.get_i64("cisco_ise.log.segment.number") == Some(0) };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{TIMESTAMP_ISO8601:_tmp.timestamp} %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:cisco_ise.log.message.code} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},
                    if !cached_grok!("^%{TIMESTAMP_ISO8601:_tmp.timestamp} %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:cisco_ise.log.message.code} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            let _cond = {
                event.has_value("cisco_ise.log.segment.number")
                    && event
                        .get_i64("cisco_ise.log.segment.number")
                        .is_some_and(|n| n > 0)
            };
            if _cond {
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^%{GREEDYDATA:cisco_ise.log.log_details_raw},
                    if !cached_grok!("^%{GREEDYDATA:cisco_ise.log.log_details_raw},")
                        .extract_into(&input, event)?
                    {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd HH:mm:ss.SSS",
                            "yyyy-MM-dd HH:mm:ss.SSSSSS",
                            "MMM [ ]d HH:mm:ss[.SSSSSS][.SSS]",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
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
                    "date__tmp_timestamp_9ef85c6a",
                )?;
                event.remove("_tmp.timestamp");
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

            let _cond = {
                event.has_value("event.timezone") && event.get_str("event.timezone") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "yyyy-MM-dd HH:mm:ss.SSS",
                                "yyyy-MM-dd HH:mm:ss.SSSSSS",
                                "MMM [ ]d HH:mm:ss[.SSSSSS][.SSS]",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "_tmp.timestamp".into(),
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
                        "date__tmp_timestamp_1d2a12b9",
                    )?;
                    event.remove("_tmp.timestamp");
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
                if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                    for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "cisco_ise.log.log_details_raw".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(
                                    event,
                                    &format!("cisco_ise.log.log_details.{}", key),
                                    value,
                                )?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("cisco_ise.log.log_details.AVPair")
                    && event
                        .get("cisco_ise.log.log_details.AVPair")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("cisco_ise.log.log_details.AVPair") {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("cisco_ise.log.log_details.AVPair").cloned();
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
                                    if let Some(kv_str) = event.get_string("_ingest._value") {
                                        for pair in kv_str.split(", ") {
                                            if pair.trim().is_empty() {
                                                continue;
                                            }
                                            let Some((key, value)) = pair.split_once("=") else {
                                                return Err(TransformError::ParseError {
                                                    path: "_ingest._value".into(),
                                                    message: format!(
                                                        "does not contain value_split: {pair}"
                                                    ),
                                                });
                                            };
                                            {
                                                if !key.is_empty() {
                                                    kv_put(
                                                        event,
                                                        &format!("cisco_ise.log.avpair.{}", key),
                                                        value,
                                                    )?;
                                                }
                                            }
                                        }
                                    }
                                    Ok(())
                                })() {
                                    event.set("_ingest.on_failure_message", err.to_string())?;
                                    event.set("_ingest.on_failure_processor_type", "kv")?;
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
                                "cisco_ise.log.log_details.AVPair",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
            }

            event.remove("cisco_ise.log.log_details.AVPair");

            let _cond = {
                event.has_value("cisco_ise.log.log_details")
                    && event.has_value("cisco_ise.log.log_details.cisco-av-pair")
                    && !(event
                        .get("cisco_ise.log.log_details.cisco-av-pair")
                        .is_some_and(|v| v.is_array()))
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.cisco-av-pair") {
                        if let Some(kv_str) =
                            event.get_string("cisco_ise.log.log_details.cisco-av-pair")
                        {
                            for pair in kv_str.split(", ") {
                                if pair.trim().is_empty() {
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=") else {
                                    return Err(TransformError::ParseError {
                                        path: "cisco_ise.log.log_details.cisco-av-pair".into(),
                                        message: format!("does not contain value_split: {pair}"),
                                    });
                                };
                                {
                                    if !key.is_empty() {
                                        kv_put(
                                            event,
                                            &format!("cisco_ise.log.avpair.{}", key),
                                            value,
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "kv")?;
                    event.set("_ingest.on_failure_processor_tag", "kv_cisco_ise_log_log_details_cisco-av-pair_to_cisco_ise_log_avpair_97a78c34")?;
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

            event.remove("cisco_ise.log.log_details.cisco-av-pair");

            let _cond = { event.has_value("cisco_ise.log.message.description") };
            if _cond {
                // Painless script
                // Source: ctx.event.action = ctx.cisco_ise?.log?.message?.description?.splitOnToken(\":\")[0]?.toLowerCase();
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.event.action = ctx.cisco_ise?.log?.message?.description?.splitOnToken(\":\")[0]?.toLowerCase();"#
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Device IP Address") {
                    if let Some(val) = event.get("cisco_ise.log.log_details.Device IP Address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_ise.log.log_details.Device IP Address".into(),
                                message,
                            }
                        })?;
                        event.set("client.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_cisco_ise_log_log_details_Device_IP_Address_to_client_ip_b34586ce",
                )?;
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

            event.remove("cisco_ise.log.log_details.Device IP Address");

            let _cond = { event.has_value("client.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.CmdSet") {
                    event.rename("cisco_ise.log.log_details.CmdSet", "cisco_ise.log.cmdset")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.RequestLatency") {
                    if let Some(val) = event.get("cisco_ise.log.log_details.RequestLatency") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_ise.log.log_details.RequestLatency".into(),
                                message,
                            }
                        })?;
                        event.set("cisco_ise.log.request.latency", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_RequestLatency_to_cisco_ise_log_request_latency_bafbebd7")?;
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

            event.remove("cisco_ise.log.log_details.RequestLatency");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.NetworkDeviceName") {
                    event.rename(
                        "cisco_ise.log.log_details.NetworkDeviceName",
                        "cisco_ise.log.network.device.name",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Type") {
                    event.rename("cisco_ise.log.log_details.Type", "cisco_ise.log.type")?;
                }
                Ok(())
            })();

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Privilege-Level") {
                    if let Some(val) = event.get("cisco_ise.log.log_details.Privilege-Level") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_ise.log.log_details.Privilege-Level".into(),
                                message,
                            }
                        })?;
                        event.set("cisco_ise.log.privilege.level", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Privilege-Level_to_cisco_ise_log_privilege_level_1cbca70e")?;
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

            event.remove("cisco_ise.log.log_details.Privilege-Level");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Service") {
                    event.rename(
                        "cisco_ise.log.log_details.Service",
                        "cisco_ise.log.service.name",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.User") {
                    event.rename("cisco_ise.log.log_details.User", "user.name")?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Port") {
                    event.rename("cisco_ise.log.log_details.Port", "cisco_ise.log.port")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Remote-Address") {
                    event.rename(
                        "cisco_ise.log.log_details.Remote-Address",
                        "destination.address",
                    )?;
                }
                Ok(())
            })();

            let _cond = {
                event.has_value("destination.address")
                    && event.get_str("destination.address") != Some("")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("destination.address") {
                        if let Some(input) = event.get_string("destination.address") {
                            // Grok pattern: ^%{IPV4:destination.ip}$
                            // Grok pattern: ^%{IPV6:destination.ip}$
                            // Grok pattern: ^\\[%{IPV6:destination.ip}\\]$
                            if !extract_first_match(
                                &[
                                    cached_grok!("^%{IPV4:destination.ip}$"),
                                    cached_grok!("^%{IPV6:destination.ip}$"),
                                    cached_grok!("^\\[%{IPV6:destination.ip}\\]$"),
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
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("destination.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Authen-Method") {
                    event.rename(
                        "cisco_ise.log.log_details.Authen-Method",
                        "cisco_ise.log.authen_method",
                    )?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("cisco_ise.log.avpair") };
            if _cond {
                // Painless script
                // Source: def avpair = ctx.cisco_ise.log.avpair;\nfor (def field : ['start_time', 'stop_time']) {\n  def s = avpair[field];\n  if (!(s instanceof String) || s == \"0\" || !(s ==~ /^\\d+$/)) { continue; }\n  long v = Long.parseLong(s);\n  if      (v >= 1000000000000000000L) { v /= 1000000L; } // ns -> ms\n  else if (v >= 1000000000000000L)    { v /= 1000L; }    // µs -> ms\n  else if (v <  10000000000L)         { v *= 1000L; }    // s  -> ms\n  avpair[field] = v;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def avpair = ctx.cisco_ise.log.avpair;\nfor (def field : ['start_time', 'stop_time']) {\n  def s = avpair[field];\n  if (!(s instanceof String) || s == \"0\" || !(s ==~ /^\\d+$/)) { continue; }\n  long v = Long.parseLong(s);\n  if      (v >= 1000000000000000000L) { v /= 1000000L; } // ns -> ms\n  else if (v >= 1000000000000000L)    { v /= 1000L; }    // µs -> ms\n  else if (v <  10000000000L)         { v *= 1000L; }    // s  -> ms\n  avpair[field] = v;\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cisco_ise.log.avpair.start_time")
                    && event.get_str("cisco_ise.log.avpair.start_time") != Some("0")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cisco_ise.log.avpair.start_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("cisco_ise.log.avpair.start_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cisco_ise.log.avpair.start_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_cisco_ise_log_avpair_start_time_to_cisco_ise_log_avpair_start_time_14a86abc")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    if event.remove("cisco_ise.log.avpair.start_time").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "cisco_ise.log.avpair.start_time".into(),
                        });
                    }
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
                if event.has_value("cisco_ise.log.avpair.priv-lvl") {
                    if let Some(val) = event.get("cisco_ise.log.avpair.priv-lvl") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_ise.log.avpair.priv-lvl".into(),
                                message,
                            }
                        })?;
                        event.set("cisco_ise.log.avpair.priv_lvl", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_avpair_priv-lvl_to_cisco_ise_log_avpair_priv_lvl_a50b742a")?;
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

            event.remove("cisco_ise.log.avpair.priv-lvl");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.avpair.disc-cause") {
                    if let Some(val) = event.get("cisco_ise.log.avpair.disc-cause") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_ise.log.avpair.disc-cause".into(),
                                message,
                            }
                        })?;
                        event.set("cisco_ise.log.avpair.disc.cause", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_avpair_disc-cause_to_cisco_ise_log_avpair_disc_cause_40a4b86d")?;
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

            event.remove("cisco_ise.log.avpair.disc-cause");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.avpair.disc-cause-ext") {
                    if let Some(val) = event.get("cisco_ise.log.avpair.disc-cause-ext") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_ise.log.avpair.disc-cause-ext".into(),
                                message,
                            }
                        })?;
                        event.set("cisco_ise.log.avpair.disc.cause_ext", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_avpair_disc-cause-ext_to_cisco_ise_log_avpair_disc_cause_ext_42c5ea4b")?;
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

            event.remove("cisco_ise.log.avpair.disc-cause-ext");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.avpair.pre-session-time") {
                    if let Some(val) = event.get("cisco_ise.log.avpair.pre-session-time") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_ise.log.avpair.pre-session-time".into(),
                                message,
                            }
                        })?;
                        event.set("cisco_ise.log.avpair.pre_session_time", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_avpair_pre-session-time_to_cisco_ise_log_avpair_pre_session_time_4e8ca50c")?;
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

            event.remove("cisco_ise.log.avpair.pre-session-time");

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.avpair.elapsed_time") {
                    if let Some(val) = event.get("cisco_ise.log.avpair.elapsed_time") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_ise.log.avpair.elapsed_time".into(),
                                message,
                            }
                        })?;
                        event.set("cisco_ise.log.avpair.elapsed_time", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_avpair_elapsed_time_to_cisco_ise_log_avpair_elapsed_time_ef9e01f6")?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                if event.remove("cisco_ise.log.avpair.elapsed_time").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "cisco_ise.log.avpair.elapsed_time".into(),
                    });
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("cisco_ise.log.avpair.stop_time")
                    && event.get_str("cisco_ise.log.avpair.stop_time") != Some("0")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cisco_ise.log.avpair.stop_time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("cisco_ise.log.avpair.stop_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "cisco_ise.log.avpair.stop_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_cisco_ise_log_avpair_stop_time_to_cisco_ise_log_avpair_stop_time_56ddb9d2")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    if event.remove("cisco_ise.log.avpair.stop_time").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "cisco_ise.log.avpair.stop_time".into(),
                        });
                    }
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
                if event.has_value("cisco_ise.log.log_details.AcctRequest-Flags") {
                    event.rename(
                        "cisco_ise.log.log_details.AcctRequest-Flags",
                        "cisco_ise.log.acct.request.flags",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Service-Argument") {
                    event.rename(
                        "cisco_ise.log.log_details.Service-Argument",
                        "cisco_ise.log.service.argument",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.AcsSessionID") {
                    event.rename(
                        "cisco_ise.log.log_details.AcsSessionID",
                        "cisco_ise.log.acs.session.id",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.SelectedAccessService") {
                    event.rename(
                        "cisco_ise.log.log_details.SelectedAccessService",
                        "cisco_ise.log.selected.access.service",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.NetworkDeviceGroups") {
                    event.rename(
                        "cisco_ise.log.log_details.NetworkDeviceGroups",
                        "cisco_ise.log.network.device.groups",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.CPMSessionID") {
                    event.rename(
                        "cisco_ise.log.log_details.CPMSessionID",
                        "cisco_ise.log.cpm.session.id",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Model Name") {
                    event.rename(
                        "cisco_ise.log.log_details.Model Name",
                        "cisco_ise.log.model.name",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Software Version") {
                    event.rename(
                        "cisco_ise.log.log_details.Software Version",
                        "cisco_ise.log.software.version",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Network Device Profile") {
                    event.rename(
                        "cisco_ise.log.log_details.Network Device Profile",
                        "cisco_ise.log.network.device.profile",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Location") {
                    event.rename(
                        "cisco_ise.log.log_details.Location",
                        "cisco_ise.log.location",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Device Type") {
                    event.rename(
                        "cisco_ise.log.log_details.Device Type",
                        "cisco_ise.log.device.type",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.IPSEC") {
                    event.rename("cisco_ise.log.log_details.IPSEC", "cisco_ise.log.ipsec")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.Step") {
                    event.rename("cisco_ise.log.log_details.Step", "cisco_ise.log.step")?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("cisco_ise.log.log_details.Response") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("{") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find("}") else {
                            break 'dissect false;
                        };
                        captured.push(("_tmp.response", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("}") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("cisco_ise.log.log_details.Response").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "cisco_ise.log.log_details.Response".into(),
                    });
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(kv_str) = event.get_string("_tmp.response") {
                    for pair in kv_str.split("; ") {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = pair.split_once("=") else {
                            return Err(TransformError::ParseError {
                                path: "_tmp.response".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            if !key.is_empty() {
                                kv_put(event, &format!("cisco_ise.log.response.{}", key), value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

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
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
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
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
