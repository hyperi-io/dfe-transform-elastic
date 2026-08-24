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
            event.set("ecs.version", json!("8.17.0"))?;

            if let Some(v) = event.get("message").cloned() {
                event.set("event.original", v)?;
            }

            event.remove("message");

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: %{SYSLOG5424PRI}%{NONNEGINT:syslog5424_ver} +(?:(?:(?P<syslog5424_ts>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?))(?:-?%{ISO8601_TIMEZONE:_temp_.tz})?)|-) +(?:%{SYSLOG5424PRINTASCII:syslog5424_host}|-) +(-|%{SYSLOG5424PRINTASCII:syslog5424_app}) +(-|%{SYSLOG5424PRINTASCII:syslog5424_proc}) +(?::-|%{SYSLOG5424PRINTASCII:syslog5424_msgid}) +\\[%{GREEDYDATA:syslog5424_sd}\\]
                    let _ = cached_grok!("%{SYSLOG5424PRI}%{NONNEGINT:syslog5424_ver} +(?:(?:(?P<syslog5424_ts>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?))(?:-?%{ISO8601_TIMEZONE:_temp_.tz})?)|-) +(?:%{SYSLOG5424PRINTASCII:syslog5424_host}|-) +(-|%{SYSLOG5424PRINTASCII:syslog5424_app}) +(-|%{SYSLOG5424PRINTASCII:syslog5424_proc}) +(?::-|%{SYSLOG5424PRINTASCII:syslog5424_msgid}) +\\[%{GREEDYDATA:syslog5424_sd}\\]").extract_into(&input, event)?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set("_ingest.on_failure_processor_tag", "grok_syslog_line")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "fail-{}",
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: (format!(
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
                    ))
                    .to_string(),
                });
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(kv_str) = event.get_string("syslog5424_sd") {
                    for pair in cached_regex!("(?<!\\\\\")(?<=\"); (?=\\w)")
                        .split(&kv_str)
                        .into_iter()
                    {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = ({
                            let parts =
                                cached_regex!("(?i)(?<=[0-9a-z]):{1,2}(?=\")").splitn(&pair, 2);
                            match (parts.first(), parts.get(1)) {
                                (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                _ => None,
                            }
                        }) else {
                            return Err(TransformError::ParseError {
                                path: "syslog5424_sd".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let key = key.trim_matches(|c| " ".contains(c));
                            let value = match (value.chars().next(), value.chars().last()) {
                                (Some('('), Some(')'))
                                | (Some('['), Some(']'))
                                | (Some('<'), Some('>'))
                                | (Some('"'), Some('"'))
                                | (Some('\''), Some('\''))
                                    if value.chars().count() > 1 =>
                                {
                                    &value[1..value.len() - 1]
                                }
                                _ => &value[..],
                            };
                            let value = value.trim_matches(|c| " ".contains(c));
                            if [
                                "flags",
                                "layer_uuid",
                                "__policy_id_tag",
                                "version",
                                "rounded_bytes",
                                "db_tag",
                                "update_service",
                            ]
                            .contains(&key)
                            {
                                continue;
                            }
                            if !key.is_empty() {
                                kv_put(event, &format!("checkpoint.{}", key), value)?;
                            }
                        }
                    }
                }
                Ok(())
            })();

            let _cond = { !event.has_value("checkpoint") };
            if _cond {
                if let Some(input) = event.get_string("syslog5424_sd") {
                    // Grok pattern: (?:%{NOTSPACE} +)?%{GREEDYDATA:syslog5424_sd}
                    let _ = cached_grok!("(?:%{NOTSPACE} +)?%{GREEDYDATA:syslog5424_sd}")
                        .extract_into(&input, event)?;
                }
            }

            let _cond = { !event.has_value("checkpoint") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("syslog5424_sd") {
                        for pair in cached_regex!("(?<=\") ").split(&kv_str).into_iter() {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = ({
                                let parts =
                                    cached_regex!("(?i)(?<=[0-9a-z])=(?=\")").splitn(&pair, 2);
                                match (parts.first(), parts.get(1)) {
                                    (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                    _ => None,
                                }
                            }) else {
                                return Err(TransformError::ParseError {
                                    path: "syslog5424_sd".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                let key = key.trim_matches(|c| " ".contains(c));
                                let value = match (value.chars().next(), value.chars().last()) {
                                    (Some('('), Some(')'))
                                    | (Some('['), Some(']'))
                                    | (Some('<'), Some('>'))
                                    | (Some('"'), Some('"'))
                                    | (Some('\''), Some('\''))
                                        if value.chars().count() > 1 =>
                                    {
                                        &value[1..value.len() - 1]
                                    }
                                    _ => &value[..],
                                };
                                let value = value.trim_matches(|c| " ".contains(c));
                                if [
                                    "flags",
                                    "layer_uuid",
                                    "__policy_id_tag",
                                    "version",
                                    "rounded_bytes",
                                    "db_tag",
                                    "update_service",
                                    "ProductName",
                                    "ProductFamily",
                                    "UP_match_table",
                                    "ROW_END",
                                ]
                                .contains(&key)
                                {
                                    continue;
                                }
                                if !key.is_empty() {
                                    kv_put(event, &format!("checkpoint.{}", key), value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint") {
                    foreach_array(event, "checkpoint", |event| {
                        map_strings(event, "_ingest._key", "_ingest._key", str::to_lowercase)?;
                        Ok(())
                    })?;
                }
                Ok(())
            })();

            event.remove("syslog5424_sd");
            event.remove("syslog5424_app");
            event.remove("syslog5424_host");
            event.remove("syslog5424_msgid");
            event.remove("syslog5424_pri");
            event.remove("syslog5424_proc");
            event.remove("syslog5424_ver");
            event.remove("host");

            if event.has("@timestamp") {
                event.rename("@timestamp", "event.created")?;
            }

            let _cond = { event.get_str("_temp_.tz") == Some("Z") };
            if _cond {
                event.set("_temp_.tz", json!("UTC"))?;
            }

            let _cond = {
                event.has_value("_conf.tz_offset")
                    && event.get_str("_conf.tz_offset") != Some("local")
            };
            if _cond {
                if let Some(v) = event.get("_conf.tz_offset").cloned() {
                    if !event.has("_temp_.tz") {
                        event.set("_temp_.tz", v)?;
                    }
                }
            }

            let _cond = { event.has_value("event.timezone") };
            if _cond {
                if let Some(v) = event.get("event.timezone").cloned() {
                    if !event.has("_temp_.tz") {
                        event.set("_temp_.tz", v)?;
                    }
                }
            }

            if !event.has("_temp_.tz") {
                event.set("_temp_.tz", json!("UTC"))?;
            }

            if let Some(v) = event.get("_temp_.tz").cloned() {
                event.set("event.timezone", v)?;
            }

            if event.has_value("event.timezone") {
                gsub_field(
                    event,
                    "event.timezone",
                    "event.timezone",
                    cached_regex!("([+-][0-9]{2})([0-9]{2})"),
                    "$1:$2",
                )?;
            }

            if event.has_value("event.timezone") {
                gsub_field(
                    event,
                    "event.timezone",
                    "event.timezone",
                    cached_regex!("([+-])([0-9]):?([0-9]{2})"),
                    "$10$2:$3",
                )?;
            }

            let _cond = { !event.has_value("checkpoint.time") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("syslog5424_ts") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "UNIX"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "syslog5424_ts".into(),
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
                        "date_syslog5424_ts_a23f7813",
                    )?;
                    event.remove("event.timezone");
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("syslog5424_ts") {
                            match parse_date_out(&date_str, &["ISO8601", "UNIX"], None, None) {
                                Some(parsed) => event.set("@timestamp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "syslog5424_ts".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_utc_fallback")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "fail-{}",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )),
                        )?;
                        return Err(TransformError::ParseError {
                            path: "_fail".into(),
                            message: (format!(
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
                            ))
                            .to_string(),
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

            let _cond =
                { !event.has_value("checkpoint.loguid") && !event.has_value("checkpoint.time") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("event.original") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }
            }

            let _cond = { !event.has_value("_id") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("checkpoint.loguid") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("checkpoint.time") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("checkpoint.segment_time") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("checkpoint.lastupdatetime") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("checkpoint.sequencenum") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("checkpoint.update_count") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }
            }

            let _cond = {
                !(["Log In", "Log Out"]
                    .contains(&event.get_str("checkpoint.operation").unwrap_or("")))
                    && !(["Log In", "Log Out"]
                        .contains(&event.get_str("checkpoint.action").unwrap_or("")))
            };
            if _cond {
                event.append_unique("event.category", json!("network"))?;
            }

            event.set("observer.vendor", json!("Checkpoint"))?;

            let _cond = { !event.has_value("checkpoint.type") };
            if _cond {
                event.set("observer.type", json!("firewall"))?;
            }

            let v = json!(
                event
                    .get("checkpoint.product")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("observer.product", v)?;
            }

            if event.has("checkpoint.src") {
                event.rename("checkpoint.src", "source.ip")?;
            }

            let _cond = { !event.has_value("source.ip") };
            if _cond {
                if event.has("checkpoint.client_ip") {
                    event.rename("checkpoint.client_ip", "source.ip")?;
                }
            }

            let _cond = {
                event.get_str("checkpoint.xlatesrc") != Some("0.0.0.0")
                    && event.get_str("checkpoint.xlatesrc") != Some("")
            };
            if _cond {
                if event.has("checkpoint.xlatesrc") {
                    event.rename("checkpoint.xlatesrc", "source.nat.ip")?;
                }
            }

            if event.has("checkpoint.dst") {
                event.rename("checkpoint.dst", "destination.ip")?;
            }

            let _cond = {
                event.get_str("checkpoint.xlatedst") != Some("0.0.0.0")
                    && event.get_str("checkpoint.xlatedst") != Some("")
            };
            if _cond {
                if event.has("checkpoint.xlatedst") {
                    event.rename("checkpoint.xlatedst", "destination.nat.ip")?;
                }
            }

            if event.has("checkpoint.uid") {
                event.rename("checkpoint.uid", "source.user.id")?;
            }

            let _cond = { event.has_value("checkpoint.src_user_name") };
            if _cond {
                if let Some(input) = event.get_string("checkpoint.src_user_name") {
                    // Grok pattern: ^%{DATA:source.user.full_name} \\(%{EMAILADDRESS:source.user.email}\\)$
                    // Grok pattern: ^%{DATA:source.user.full_name} \\((?P<source_user_name>(?:[^()@]+))\\)$
                    // Grok pattern: ^%{EMAILADDRESS:source.user.email}$
                    // Grok pattern: ^%{DATA:source.user.name}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^%{DATA:source.user.full_name} \\(%{EMAILADDRESS:source.user.email}\\)$"
                            ),
                            cached_grok_mapped!(
                                "^%{DATA:source.user.full_name} \\((?P<source_user_name>(?:[^()@]+))\\)$",
                                [("source_user_name", "source.user.name")]
                            ),
                            cached_grok!("^%{EMAILADDRESS:source.user.email}$"),
                            cached_grok!("^%{DATA:source.user.name}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = {
                event.has_value("checkpoint.administrator")
                    && !event.has_value("source.user.name")
                    && !event.has_value("source.user.email")
            };
            if _cond {
                if let Some(input) = event.get_string("checkpoint.administrator") {
                    // Grok pattern: ^%{DATA:source.user.full_name} \\(%{EMAILADDRESS:source.user.email}\\)$
                    // Grok pattern: ^%{DATA:source.user.full_name} \\((?P<source_user_name>(?:[^()@]+))\\)$
                    // Grok pattern: ^%{EMAILADDRESS:source.user.email}$
                    // Grok pattern: ^%{DATA:source.user.name}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^%{DATA:source.user.full_name} \\(%{EMAILADDRESS:source.user.email}\\)$"
                            ),
                            cached_grok_mapped!(
                                "^%{DATA:source.user.full_name} \\((?P<source_user_name>(?:[^()@]+))\\)$",
                                [("source_user_name", "source.user.name")]
                            ),
                            cached_grok!("^%{EMAILADDRESS:source.user.email}$"),
                            cached_grok!("^%{DATA:source.user.name}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { event.has_value("checkpoint.user") && !event.has_value("user.name") };
            if _cond {
                if let Some(input) = event.get_string("checkpoint.user") {
                    // Grok pattern: ^%{DATA:user.full_name} \\(%{EMAILADDRESS:user.email}\\)$
                    // Grok pattern: ^%{DATA:user.full_name} \\((?P<user_name>(?:[^()@]+))\\)$
                    // Grok pattern: ^%{EMAILADDRESS:user.email}$
                    // Grok pattern: ^%{DATA:user.name}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^%{DATA:user.full_name} \\(%{EMAILADDRESS:user.email}\\)$"
                            ),
                            cached_grok_mapped!(
                                "^%{DATA:user.full_name} \\((?P<user_name>(?:[^()@]+))\\)$",
                                [("user_name", "user.name")]
                            ),
                            cached_grok!("^%{EMAILADDRESS:user.email}$"),
                            cached_grok!("^%{DATA:user.name}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint.failed_login_factor_num") {
                    if let Some(val) = event.get("checkpoint.failed_login_factor_num") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "checkpoint.failed_login_factor_num".into(),
                                message,
                            }
                        })?;
                        event.set("checkpoint.failed_login_factor_num", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint.client_outbound_packets") {
                    if let Some(val) = event.get("checkpoint.client_outbound_packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "checkpoint.client_outbound_packets".into(),
                                message,
                            }
                        })?;
                        event.set("source.packets", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint.server_outbound_packets") {
                    if let Some(val) = event.get("checkpoint.server_outbound_packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "checkpoint.server_outbound_packets".into(),
                                message,
                            }
                        })?;
                        event.set("destination.packets", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint.client_outbound_bytes") {
                    if let Some(val) = event.get("checkpoint.client_outbound_bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "checkpoint.client_outbound_bytes".into(),
                                message,
                            }
                        })?;
                        event.set("source.bytes", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { !event.has_value("source.bytes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("checkpoint.sent_byte") {
                        if let Some(val) = event.get("checkpoint.sent_byte") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "checkpoint.sent_byte".into(),
                                    message,
                                }
                            })?;
                            event.set("source.bytes", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint.server_outbound_bytes") {
                    if let Some(val) = event.get("checkpoint.server_outbound_bytes") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "checkpoint.server_outbound_bytes".into(),
                                message,
                            }
                        })?;
                        event.set("destination.bytes", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { !event.has_value("destination.bytes") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("checkpoint.received_bytes") {
                        if let Some(val) = event.get("checkpoint.received_bytes") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "checkpoint.received_bytes".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.bytes", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("checkpoint.service") != Some("4294967295") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("checkpoint.service") {
                        if let Some(val) = event.get("checkpoint.service") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "checkpoint.service".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.get_str("checkpoint.xlatedport") != Some("0") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("checkpoint.xlatedport") {
                        if let Some(val) = event.get("checkpoint.xlatedport") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "checkpoint.xlatedport".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.nat.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint.s_port") {
                    if let Some(val) = event.get("checkpoint.s_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "checkpoint.s_port".into(),
                                message,
                            }
                        })?;
                        event.set("source.port", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.get_str("checkpoint.xlatesport") != Some("0") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("checkpoint.xlatesport") {
                        if let Some(val) = event.get("checkpoint.xlatesport") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "checkpoint.xlatesport".into(),
                                    message,
                                }
                            })?;
                            event.set("source.nat.port", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            if event.has("checkpoint.mac_source_address") {
                event.rename("checkpoint.mac_source_address", "source.mac")?;
            }

            if event.has("checkpoint.src_machine_name") {
                event.rename("checkpoint.src_machine_name", "source.domain")?;
            }

            if event.has("checkpoint.destination_dns_hostname") {
                event.rename("checkpoint.destination_dns_hostname", "destination.domain")?;
            }

            let _cond = { !event.has_value("server.domain") };
            if _cond {
                if event.has("checkpoint.dst_machine_name") {
                    event.rename("checkpoint.dst_machine_name", "destination.domain")?;
                }
            }

            let _cond = { event.has_value("checkpoint.dst_user_name") };
            if _cond {
                if let Some(input) = event.get_string("checkpoint.dst_user_name") {
                    // Grok pattern: ^%{DATA:destination.user.full_name} \\(%{EMAILADDRESS:destination.user.email}\\)$
                    // Grok pattern: ^%{DATA:destination.user.full_name} \\((?P<destination_user_name>(?:[^()@]+))\\)$
                    // Grok pattern: ^%{EMAILADDRESS:destination.user.email}$
                    // Grok pattern: ^%{DATA:destination.user.name}$
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^%{DATA:destination.user.full_name} \\(%{EMAILADDRESS:destination.user.email}\\)$"
                            ),
                            cached_grok_mapped!(
                                "^%{DATA:destination.user.full_name} \\((?P<destination_user_name>(?:[^()@]+))\\)$",
                                [("destination_user_name", "destination.user.name")]
                            ),
                            cached_grok!("^%{EMAILADDRESS:destination.user.email}$"),
                            cached_grok!("^%{DATA:destination.user.name}$"),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = { event.has_value("checkpoint.dst_user_dn") };
            if _cond {
                if let Some(v) = event.get("checkpoint.dst_user_dn").cloned() {
                    event.set("destination.user.domain", v)?;
                }
            }

            if event.has("checkpoint.src_user_group") {
                event.rename("checkpoint.src_user_group", "source.user.group.name")?;
            }

            let _cond = {
                event.get_str("checkpoint.operation") == Some("Log In")
                    || event.get_str("checkpoint.operation") == Some("Log Out")
            };
            if _cond {
                event.append_unique("event.category", json!("authentication"))?;
            }

            if event.has("checkpoint.originsicname") {
                event.rename("checkpoint.originsicname", "checkpoint.origin_sic_name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint.origin_sic_name") {
                    if let Some(input) = event.get_string("checkpoint.origin_sic_name") {
                        // Grok pattern: (?i)^CN=%{DATA:_temp_.sic_cn},O=%{GREEDYDATA}$
                        let _ = cached_grok!("(?i)^CN=%{DATA:_temp_.sic_cn},O=%{GREEDYDATA}$")
                            .extract_into(&input, event)?;
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("_temp_.sic_cn")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("observer.hostname") {
                    event.set("observer.hostname", v)?;
                }
            }

            let _cond = {
                ["Prevent", "Detect", "Quarantine"]
                    .contains(&event.get_str("checkpoint.rule_action").unwrap_or(""))
                    || (!event.has_value("checkpoint.rule_action")
                        && ["Prevent", "Detect", "Quarantine"]
                            .contains(&event.get_str("checkpoint.action").unwrap_or("")))
            };
            if _cond {
                event.set("event.kind", json!("alert"))?;
            }

            let _cond = { !event.has_value("event.kind") };
            if _cond {
                event.set("event.kind", json!("event"))?;
            }

            let _cond = {
                ["Accept", "Allow"].contains(&event.get_str("checkpoint.rule_action").unwrap_or(""))
                    || (!event.has_value("checkpoint.rule_action")
                        && ["Accept", "Allow"]
                            .contains(&event.get_str("checkpoint.action").unwrap_or("")))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                (["Accept", "Allow"]
                    .contains(&event.get_str("checkpoint.rule_action").unwrap_or(""))
                    || (!event.has_value("checkpoint.rule_action")
                        && ["Accept", "Allow"]
                            .contains(&event.get_str("checkpoint.action").unwrap_or(""))))
                    && (event.get_str("checkpoint.operation") != Some("Log In")
                        && event.get_str("checkpoint.operation") != Some("Log Out"))
            };
            if _cond {
                event.append("event.type", json!("allowed"))?;
                event.append("event.type", json!("connection"))?;
            }

            let _cond = { event.get_str("checkpoint.audit_status") == Some("Success") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("checkpoint.audit_status") == Some("Failure") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = {
                ["Drop", "Reject", "Block", "Prevent"]
                    .contains(&event.get_str("checkpoint.rule_action").unwrap_or(""))
                    || (!event.has_value("checkpoint.rule_action")
                        && ["Drop", "Reject", "Block", "Prevent"]
                            .contains(&event.get_str("checkpoint.action").unwrap_or("")))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                ["Drop", "Reject", "Block", "Prevent"]
                    .contains(&event.get_str("checkpoint.rule_action").unwrap_or(""))
                    || (!event.has_value("checkpoint.rule_action")
                        && ["Drop", "Reject", "Block", "Prevent"]
                            .contains(&event.get_str("checkpoint.action").unwrap_or("")))
            };
            if _cond {
                event.append("event.type", json!("connection"))?;
                event.append("event.type", json!("denied"))?;
            }

            let _cond = { event.has_value("checkpoint.malware_action") };
            if _cond {
                event.append_unique("event.category", json!("malware"))?;
            }

            let _cond = {
                ["Detect", "Prevent"]
                    .contains(&event.get_str("checkpoint.rule_action").unwrap_or(""))
                    || (!event.has_value("checkpoint.rule_action")
                        && ["Detect", "Prevent"]
                            .contains(&event.get_str("checkpoint.action").unwrap_or("")))
            };
            if _cond {
                event.append_unique("event.category", json!("intrusion_detection"))?;
            }

            let _cond = { event.get_str("checkpoint.action") == Some("Log In") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("checkpoint.action") == Some("Failed Log In") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("checkpoint.operation") == Some("Log Out") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("checkpoint.operation") == Some("Log In")
                    && (event.get_str("checkpoint.audit_status") == Some("Success")
                        || !event.has_value("checkpoint.audit_status"))
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                ["Log In", "Failed Log In"]
                    .contains(&event.get_str("checkpoint.action").unwrap_or(""))
            };
            if _cond {
                event.append_unique("event.category", json!("authentication"))?;
            }

            let _cond = {
                event.get_str("checkpoint.action") == Some("Log In")
                    || event.get_str("checkpoint.operation") == Some("Log In")
            };
            if _cond {
                event.append_unique("event.type", json!("start"))?;
            }

            let _cond = {
                event.get_str("checkpoint.action") == Some("Log Out")
                    || event.get_str("checkpoint.operation") == Some("Log Out")
            };
            if _cond {
                event.append_unique("event.type", json!("end"))?;
            }

            let _cond = { event.get_str("checkpoint.action") == Some("Log In") };
            if _cond {
                event.set("checkpoint.action", json!("logged-in"))?;
            }

            let _cond = { event.get_str("checkpoint.action") == Some("Failed Log In") };
            if _cond {
                event.append_unique("event.type", json!("denied"))?;
            }

            let _cond = { event.get_str("checkpoint.action") == Some("Failed Log In") };
            if _cond {
                event.set("checkpoint.action", json!("logon-failed"))?;
            }

            if let Some(v) = event
                .get("checkpoint.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("checkpoint.host_ip") };
            if _cond {
                if let Some(val) = event.get("checkpoint.host_ip") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "checkpoint.host_ip".into(),
                            message,
                        })?;
                    event.set("host.ip", converted)?;
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("source.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.nat.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.nat.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("checkpoint.origin") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("checkpoint.origin")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("checkpoint.origin_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("checkpoint.origin_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("checkpoint.endpoint_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("checkpoint.endpoint_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("checkpoint.file_md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("checkpoint.file_md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("checkpoint.file_sha1") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("checkpoint.file_sha1")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("checkpoint.file_sha256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("checkpoint.file_sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("checkpoint.from") {
                event.rename("checkpoint.from", "source.user.email")?;
            }

            if event.has("checkpoint.to") {
                event.rename("checkpoint.to", "destination.user.email")?;
            }

            let _cond = { event.has_value("destination.user.email") };
            if _cond {
                event.set(
                    "email.from.address",
                    Value::Array(vec![json!(
                        event
                            .get("destination.user.email")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = { event.has_value("destination.user.email") };
            if _cond {
                event.set(
                    "email.to.address",
                    Value::Array(vec![json!(
                        event
                            .get("destination.user.email")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = { event.has_value("checkpoint.mime_from") };
            if _cond {
                event.append(
                    "email.from.address",
                    json!(
                        event
                            .get("checkpoint.mime_from")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("checkpoint.mime_to") };
            if _cond {
                event.append(
                    "email.to.address",
                    json!(
                        event
                            .get("checkpoint.mime_to")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("checkpoint.email_subject") };
            if _cond {
                if let Some(v) = event.get("checkpoint.email_subject").cloned() {
                    event.set("email.subject", v)?;
                }
            }

            let _cond = { event.has_value("checkpoint.bcc") };
            if _cond {
                event.append(
                    "email.bcc.address",
                    json!(
                        event
                            .get("checkpoint.bcc")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("checkpoint.cc") };
            if _cond {
                event.append(
                    "email.cc.address",
                    json!(
                        event
                            .get("checkpoint.cc")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("checkpoint.delivery_time") };
            if _cond {
                if let Some(v) = event.get("checkpoint.delivery_time").cloned() {
                    event.set("email.delivery_timestamp", v)?;
                }
            }

            let _cond = { event.has_value("checkpoint.email_message_id") };
            if _cond {
                if let Some(v) = event.get("checkpoint.email_message_id").cloned() {
                    event.set("email.message_id", v)?;
                }
            }

            let _cond = { event.has_value("checkpoint.email_queue_id") };
            if _cond {
                if let Some(v) = event.get("checkpoint.email_queue_id").cloned() {
                    event.set("email.local_id", v)?;
                }
            }

            if event.has("checkpoint.usercheck_incident_uid") {
                event.rename("checkpoint.usercheck_incident_uid", "destination.user.id")?;
            }

            if event.has("checkpoint.service_name") {
                event.rename("checkpoint.service_name", "destination.service.name")?;
            }

            if event.has("checkpoint.mac_destination_address") {
                event.rename("checkpoint.mac_destination_address", "destination.mac")?;
            }

            if event.has("checkpoint.dns_type") {
                event.rename("checkpoint.dns_type", "dns.question.type")?;
            }

            if event.has("checkpoint.domain_name") {
                event.rename("checkpoint.domain_name", "dns.question.name")?;
            }

            let _cond = {
                event.get_str("checkpoint.operation") == Some("Log Out")
                    && !event.has_value("source.user.domain")
            };
            if _cond {
                if event.has("dns.question.name") {
                    event.rename("dns.question.name", "source.user.domain")?;
                }
            }

            let _cond = {
                event.get_str("checkpoint.operation") == Some("Log In")
                    && event.get_str("checkpoint.audit_status") == Some("Failure")
                    && (!event.has_value("event.reason")
                        || event.get_str("event.reason") == Some(""))
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("checkpoint.additional_info") {
                        if let Some(input) = event.get_string("checkpoint.additional_info") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) =
                                    remaining.strip_prefix("Administrator failed to log in: ")
                                else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("event.reason", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            if event.has("checkpoint.dns_message_type") {
                event.rename("checkpoint.dns_message_type", "dns.type")?;
            }

            if event.has("checkpoint.tid") {
                event.rename("checkpoint.tid", "dns.id")?;
            }

            if event.has("checkpoint.loguid") {
                event.rename("checkpoint.loguid", "event.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint.sequencenum") {
                    if let Some(val) = event.get("checkpoint.sequencenum") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "checkpoint.sequencenum".into(),
                                message,
                            }
                        })?;
                        event.set("event.sequence", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint.severity") {
                    if let Some(val) = event.get("checkpoint.severity") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "checkpoint.severity".into(),
                                message,
                            }
                        })?;
                        event.set("event.severity", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has("checkpoint.action") {
                event.rename("checkpoint.action", "event.action")?;
            }

            let _cond = {
                event.get_str("checkpoint.operation") == Some("Log In")
                    && event.get_str("checkpoint.audit_status") == Some("Failure")
            };
            if _cond {
                event.set("event.action", json!("logon-failed"))?;
            }

            let _cond = {
                event.get_str("checkpoint.operation") == Some("Log In")
                    && (event.get_str("checkpoint.audit_status") == Some("Success")
                        || !event.has_value("checkpoint.audit_status"))
            };
            if _cond {
                event.set("event.action", json!("logged-in"))?;
            }

            let _cond = { event.get_str("checkpoint.operation") == Some("Log Out") };
            if _cond {
                event.set("event.action", json!("logged-out"))?;
            }

            let _cond = {
                event.get_str("checkpoint.operation") == Some("Log In")
                    && event.get_str("observer.product") == Some("Expert Shell")
            };
            if _cond {
                let v = json!(
                    event
                        .get("checkpoint.device_name")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    if !event.has("host.name") {
                        event.set("host.name", v)?;
                    }
                }
            }

            let _cond = {
                event.get_str("checkpoint.operation") == Some("Log In")
                    && event.get_str("observer.product") == Some("Expert Shell")
            };
            if _cond {
                let v = json!(
                    event
                        .get("checkpoint.device_type")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    if !event.has("host.type") {
                        event.set("host.type", v)?;
                    }
                }
            }

            let _cond = {
                ["Log In", "Log Out"].contains(&event.get_str("checkpoint.operation").unwrap_or(""))
                    && event.get_str("checkpoint.machine") != Some("localhost")
            };
            if _cond {
                let v = json!(
                    event
                        .get("checkpoint.machine")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    if !event.has("source.domain") {
                        event.set("source.domain", v)?;
                    }
                }
            }

            let _cond = {
                event.get_str("checkpoint.operation") == Some("Log In")
                    && event.get_str("observer.product") == Some("Expert Shell")
            };
            if _cond {
                if !event.has("network.protocol") {
                    event.set("network.protocol", json!("ssh"))?;
                }
            }

            let v = json!(
                event
                    .get("source.user.name")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                if !event.has("user.name") {
                    event.set("user.name", v)?;
                }
            }

            let v = json!(
                event
                    .get("source.user.full_name")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                if !event.has("user.full_name") {
                    event.set("user.full_name", v)?;
                }
            }

            let v = json!(
                event
                    .get("source.user.id")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                if !event.has("user.id") {
                    event.set("user.id", v)?;
                }
            }

            let v = json!(
                event
                    .get("source.user.group.name")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                if !event.has("user.group.name") {
                    event.set("user.group.name", v)?;
                }
            }

            let v = json!(
                event
                    .get("source.user.email")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                if !event.has("user.email") {
                    event.set("user.email", v)?;
                }
            }

            let v = json!(
                event
                    .get("source.user.domain")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                if !event.has("user.domain") {
                    event.set("user.domain", v)?;
                }
            }

            let _cond = {
                event.get_str("message") == Some("Administrator Login")
                    || event.get_str("message") == Some("Administrator Logout")
                    || event.get_str("message") == Some("Administrator Expert Shell login")
            };
            if _cond {
                event.append_unique("user.roles", json!("administrator"))?;
            }

            let v = json!(
                event
                    .get("checkpoint.operation_number")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                if !event.has("event.code") {
                    event.set("event.code", v)?;
                }
            }

            if event.has_value("user.name") {
                map_strings(event, "user.name", "user.name", str::to_lowercase)?;
            }

            if event.has_value("user.id") {
                map_strings(event, "user.id", "user.id", str::to_lowercase)?;
            }

            if event.has_value("user.email") {
                map_strings(event, "user.email", "user.email", str::to_lowercase)?;
            }

            if event.has_value("user.domain") {
                map_strings(event, "user.domain", "user.domain", str::to_lowercase)?;
            }

            if event.has_value("user.group.name") {
                map_strings(
                    event,
                    "user.group.name",
                    "user.group.name",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("source.user.name") {
                map_strings(
                    event,
                    "source.user.name",
                    "source.user.name",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("destination.user.name") {
                map_strings(
                    event,
                    "destination.user.name",
                    "destination.user.name",
                    str::to_lowercase,
                )?;
            }

            if event.has("checkpoint.packet_capture") {
                event.rename("checkpoint.packet_capture", "event.url")?;
            }

            if event.has("checkpoint.start_time") {
                event.rename("checkpoint.start_time", "event.start")?;
            }

            let _cond = { !event.has_value("event.start") };
            if _cond {
                if event.has("checkpoint.first_detection") {
                    event.rename("checkpoint.first_detection", "event.start")?;
                }
            }

            if event.has("checkpoint.last_detection") {
                event.rename("checkpoint.last_detection", "event.end")?;
            }

            let _cond = { event.has_value("checkpoint.app_risk") };
            if _cond {
                if let Some(val) = event.get("checkpoint.app_risk") {
                    let converted = convert_value(val, "float").map_err(|message| {
                        TransformError::ParseError {
                            path: "checkpoint.app_risk".into(),
                            message,
                        }
                    })?;
                    event.set("checkpoint.app_risk", converted)?;
                }
            }

            if event.has("checkpoint.app_risk") {
                event.rename("checkpoint.app_risk", "event.risk_score")?;
            }

            if event.has("checkpoint.file_id") {
                event.rename("checkpoint.file_id", "file.inode")?;
            }

            if event.has("checkpoint.file_type") {
                event.rename("checkpoint.file_type", "file.type")?;
            }

            if event.has("checkpoint.file_name") {
                event.rename("checkpoint.file_name", "file.name")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint.file_size") {
                    if let Some(val) = event.get("checkpoint.file_size") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "checkpoint.file_size".into(),
                                message,
                            }
                        })?;
                        event.set("file.size", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has("checkpoint.file_md5") {
                event.rename("checkpoint.file_md5", "file.hash.md5")?;
            }

            if event.has("checkpoint.file_sha1") {
                event.rename("checkpoint.file_sha1", "file.hash.sha1")?;
            }

            if event.has("checkpoint.file_sha256") {
                event.rename("checkpoint.file_sha256", "file.hash.sha256")?;
            }

            if event.has("checkpoint.dlp_file_name") {
                event.rename("checkpoint.dlp_file_name", "file.name")?;
            }

            if event.has("checkpoint.user_group") {
                event.rename("checkpoint.user_group", "group.name")?;
            }

            if event.has("checkpoint.os_version") {
                event.rename("checkpoint.os_version", "host.os.version")?;
            }

            if event.has("checkpoint.os_name") {
                event.rename("checkpoint.os_name", "host.os.name")?;
            }

            if event.has("checkpoint.method") {
                event.rename("checkpoint.method", "http.request.method")?;
            }

            if event.has("checkpoint.referrer") {
                event.rename("checkpoint.referrer", "http.request.referrer")?;
            }

            if event.has("checkpoint.service_id") {
                event.rename("checkpoint.service_id", "network.application")?;
            }

            if event.has("checkpoint.ifdir") {
                event.rename("checkpoint.ifdir", "network.direction")?;
            }

            if event.has_value("checkpoint.bytes") {
                if let Some(val) = event.get("checkpoint.bytes") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "checkpoint.bytes".into(),
                            message,
                        }
                    })?;
                    event.set("checkpoint.bytes", converted)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint.icmp_code") {
                    if let Some(val) = event.get("checkpoint.icmp_code") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "checkpoint.icmp_code".into(),
                                message,
                            }
                        })?;
                        event.set("checkpoint.icmp_code", converted)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint.icmp_type") {
                    if let Some(val) = event.get("checkpoint.icmp_type") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "checkpoint.icmp_type".into(),
                                message,
                            }
                        })?;
                        event.set("checkpoint.icmp_type", converted)?;
                    }
                }
                Ok(())
            })();

            if event.has("checkpoint.bytes") {
                event.rename("checkpoint.bytes", "network.bytes")?;
            }

            if event.has("checkpoint.proto") {
                event.rename("checkpoint.proto", "network.iana_number")?;
            }

            let _cond = { event.has_value("network.iana_number") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def iana_number = ctx.network.iana_number;\nif (iana_number == '0' && ctx.source?.ip?.contains(':')) {\n    ctx.network.transport = 'hopopt';\n} else if (iana_number == '1') {\n    ctx.network.transport = 'icmp';\n} else if (iana_number == '2') {\n    ctx.network.transport = 'igmp';\n} else if (iana_number == '6') {\n    ctx.network.transport = 'tcp';\n} else if (iana_number == '8') {\n    ctx.network.transport = 'egp';\n} else if (iana_number == '17') {\n    ctx.network.transport = 'udp';\n} else if (iana_number == '47') {\n    ctx.network.transport = 'gre';\n} else if (iana_number == '50') {\n    ctx.network.transport = 'esp';\n} else if (iana_number == '58') {\n    ctx.network.transport = 'ipv6-icmp';\n} else if (iana_number == '112') {\n    ctx.network.transport = 'vrrp';\n} else if (iana_number == '114') {\n    ctx.network.transport = '0-hop';\n} else if (iana_number == '132') {\n    ctx.network.transport = 'sctp';\n} else if (iana_number == '4294967295') {\n    iana_number = null;\n} else {\n    ctx.network.transport = iana_number;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def iana_number = ctx.network.iana_number;\nif (iana_number == '0' && ctx.source?.ip?.contains(':')) {\n    ctx.network.transport = 'hopopt';\n} else if (iana_number == '1') {\n    ctx.network.transport = 'icmp';\n} else if (iana_number == '2') {\n    ctx.network.transport = 'igmp';\n} else if (iana_number == '6') {\n    ctx.network.transport = 'tcp';\n} else if (iana_number == '8') {\n    ctx.network.transport = 'egp';\n} else if (iana_number == '17') {\n    ctx.network.transport = 'udp';\n} else if (iana_number == '47') {\n    ctx.network.transport = 'gre';\n} else if (iana_number == '50') {\n    ctx.network.transport = 'esp';\n} else if (iana_number == '58') {\n    ctx.network.transport = 'ipv6-icmp';\n} else if (iana_number == '112') {\n    ctx.network.transport = 'vrrp';\n} else if (iana_number == '114') {\n    ctx.network.transport = '0-hop';\n} else if (iana_number == '132') {\n    ctx.network.transport = 'sctp';\n} else if (iana_number == '4294967295') {\n    iana_number = null;\n} else {\n    ctx.network.transport = iana_number;\n}\n"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("checkpoint.subs_exp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("checkpoint.subs_exp") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "EEE MMM dd HH:mm:ss yyyy",
                                "EEE MMM  d HH:mm:ss yyyy",
                                "EEE MMM d HH:mm:ss yyyy",
                                "ISO8601",
                                "UNIX",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("checkpoint.subs_exp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "checkpoint.subs_exp".into(),
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
                        "date_checkpoint_subs_exp_to_checkpoint_subs_exp_19f24900",
                    )?;
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("checkpoint.subs_exp") {
                            match parse_date_out(
                                &date_str,
                                &[
                                    "EEE MMM dd HH:mm:ss yyyy",
                                    "EEE MMM  d HH:mm:ss yyyy",
                                    "EEE MMM d HH:mm:ss yyyy",
                                    "ISO8601",
                                    "UNIX",
                                ],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("checkpoint.subs_exp", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "checkpoint.subs_exp".into(),
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
                            "date_checkpoint_subs_exp_to_checkpoint_subs_exp_84a34f9e",
                        )?;
                        event.remove("checkpoint.subs_exp");
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
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
                    .get("checkpoint.packets")
                    .is_some_and(|v| v.is_string())
                    && event.get("checkpoint.packets").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("<")),
                        serde_json::Value::String(s) => s.contains("<"),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: String packetsStr = ctx.checkpoint.packets.trim();\nif (packetsStr.startsWith('(sample')) {\n  int closeParenIdx = packetsStr.indexOf(')');\n  if (closeParenIdx > 0) {\n    ctx.checkpoint.packets_data_is_sampled = true;\n    packetsStr = packetsStr.substring(closeParenIdx + 1).trim();\n  }\n}\nif (packetsStr.endsWith('\";')) {\n  packetsStr = packetsStr.substring(0, packetsStr.length() - 2);\n}\ndef parsed = [];\nString[] entries = packetsStr.splitOnToken('>');\nfor (int i = 0; i < entries.length; i++) {\n  String entry = entries[i].trim();\n  if (entry.length() == 0) continue;\n  if (entry.startsWith('<')) {\n    entry = entry.substring(1);\n  }\n  def packet = new HashMap();\n  String[] parts = entry.splitOnToken(';');\n  String tuple = parts[0];\n  if (parts.length > 1) {\n    packet.put('interface', ['name': parts[1]]);\n  }\n  String[] fields = tuple.splitOnToken(',');\n  if (fields.length >= 5) {\n    packet.put('source', ['ip': fields[0], 'port': Long.parseLong(fields[1])]);\n    packet.put('destination', ['ip': fields[2], 'port': Long.parseLong(fields[3])]);\n    packet.put('network', ['iana_number': fields[4]]);\n    parsed.add(packet);\n  }\n}\nif (parsed.size() > 0) {\n  ctx.checkpoint.packets_dropped = parsed;\n  ctx.checkpoint.remove('packets');\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String packetsStr = ctx.checkpoint.packets.trim();\nif (packetsStr.startsWith('(sample')) {\n  int closeParenIdx = packetsStr.indexOf(')');\n  if (closeParenIdx > 0) {\n    ctx.checkpoint.packets_data_is_sampled = true;\n    packetsStr = packetsStr.substring(closeParenIdx + 1).trim();\n  }\n}\nif (packetsStr.endsWith('\";')) {\n  packetsStr = packetsStr.substring(0, packetsStr.length() - 2);\n}\ndef parsed = [];\nString[] entries = packetsStr.splitOnToken('>');\nfor (int i = 0; i < entries.length; i++) {\n  String entry = entries[i].trim();\n  if (entry.length() == 0) continue;\n  if (entry.startsWith('<')) {\n    entry = entry.substring(1);\n  }\n  def packet = new HashMap();\n  String[] parts = entry.splitOnToken(';');\n  String tuple = parts[0];\n  if (parts.length > 1) {\n    packet.put('interface', ['name': parts[1]]);\n  }\n  String[] fields = tuple.splitOnToken(',');\n  if (fields.length >= 5) {\n    packet.put('source', ['ip': fields[0], 'port': Long.parseLong(fields[1])]);\n    packet.put('destination', ['ip': fields[2], 'port': Long.parseLong(fields[3])]);\n    packet.put('network', ['iana_number': fields[4]]);\n    parsed.add(packet);\n  }\n}\nif (parsed.size() > 0) {\n  ctx.checkpoint.packets_dropped = parsed;\n  ctx.checkpoint.remove('packets');\n}\n"#
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint.packets") {
                    if let Some(val) = event.get("checkpoint.packets") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "checkpoint.packets".into(),
                                message,
                            }
                        })?;
                        event.set("checkpoint.packets", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = {
                event
                    .get("checkpoint.packets")
                    .is_some_and(|v| v.is_number())
            };
            if _cond {
                if event.has("checkpoint.packets") {
                    event.rename("checkpoint.packets", "network.packets")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("checkpoint.packet_amount") {
                    if let Some(val) = event.get("checkpoint.packet_amount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "checkpoint.packet_amount".into(),
                                message,
                            }
                        })?;
                        event.set("checkpoint.packet_amount", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = {
                !event.has_value("network.packets")
                    && event
                        .get("checkpoint.packet_amount")
                        .is_some_and(|v| v.is_number())
            };
            if _cond {
                if let Some(v) = event
                    .get("checkpoint.packet_amount")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("network.packets", v)?;
                }
            }

            if event.has("checkpoint.layer_name") {
                event.rename("checkpoint.layer_name", "network.name")?;
            }

            if event.has("checkpoint.app_name") {
                event.rename("checkpoint.app_name", "network.application")?;
            }

            if event.has("checkpoint.client_inbound_interface") {
                event.rename(
                    "checkpoint.client_inbound_interface",
                    "observer.ingress.interface.name",
                )?;
            }

            if event.has("checkpoint.client_outbound_interface") {
                event.rename(
                    "checkpoint.client_outbound_interface",
                    "observer.egress.interface.name",
                )?;
            }

            let _cond = {
                !event.has_value("observer.ingress.interface.name")
                    && event.get_str("network.direction") == Some("inbound")
            };
            if _cond {
                if event.has("checkpoint.ifname") {
                    event.rename("checkpoint.ifname", "observer.ingress.interface.name")?;
                }
            }

            let _cond = {
                !event.has_value("observer.egress.interface.name")
                    && event.get_str("network.direction") == Some("outbound")
            };
            if _cond {
                if event.has("checkpoint.ifname") {
                    event.rename("checkpoint.ifname", "observer.egress.interface.name")?;
                }
            }

            if event.has("checkpoint.type") {
                event.rename("checkpoint.type", "observer.type")?;
            }

            let _cond = { event.has_value("checkpoint.origin") };
            if _cond {
                event.append_unique(
                    "observer.ip",
                    json!(
                        event
                            .get("checkpoint.origin")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("checkpoint.origin")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.name", v)?;
            }

            event.remove("checkpoint.origin");

            if event.has("checkpoint.mac_address") {
                event.rename("checkpoint.mac_address", "_temp_.observer.mac")?;
            }

            if event.has_value("_temp_.observer.mac") {
                gsub_field(
                    event,
                    "_temp_.observer.mac",
                    "_temp_.observer.mac",
                    cached_regex!("[:]"),
                    "-",
                )?;
            }

            if event.has_value("_temp_.observer.mac") {
                map_strings(
                    event,
                    "_temp_.observer.mac",
                    "_temp_.observer.mac",
                    str::to_uppercase,
                )?;
            }

            let _cond = { event.has_value("_temp_.observer.mac") };
            if _cond {
                event.append(
                    "observer.mac",
                    json!(
                        event
                            .get("_temp_.observer.mac")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("checkpoint.origin_ip") && !event.has_value("observer.ip") };
            if _cond {
                event.append_unique(
                    "observer.ip",
                    json!(
                        event
                            .get("checkpoint.origin_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("checkpoint.endpoint_ip") && !event.has_value("observer.ip") };
            if _cond {
                event.append(
                    "observer.ip",
                    json!(
                        event
                            .get("checkpoint.endpoint_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has("checkpoint.outzone") {
                event.rename("checkpoint.outzone", "observer.egress.zone")?;
            }

            if event.has("checkpoint.inzone") {
                event.rename("checkpoint.inzone", "observer.ingress.zone")?;
            }

            let _cond = { !event.has_value("observer.egress.zone") };
            if _cond {
                if event.has("checkpoint.security_outzone") {
                    event.rename("checkpoint.security_outzone", "observer.egress.zone")?;
                }
            }

            let _cond = { !event.has_value("observer.ingress.zone") };
            if _cond {
                if event.has("checkpoint.security_inzone") {
                    event.rename("checkpoint.security_inzone", "observer.ingress.zone")?;
                }
            }

            if event.has("checkpoint.update_version") {
                event.rename("checkpoint.update_version", "observer.version")?;
            }

            if event.has("checkpoint.process_md5") {
                event.rename("checkpoint.process_md5", "process.hash.md5")?;
            }

            if event.has("checkpoint.process_name") {
                event.rename("checkpoint.process_name", "process.name")?;
            }

            if event.has("checkpoint.parent_process_md5") {
                event.rename("checkpoint.parent_process_md5", "process.parent.hash.md5")?;
            }

            if event.has("checkpoint.parent_process_name") {
                event.rename("checkpoint.parent_process_name", "process.parent.name")?;
            }

            if event.has("checkpoint.matched_category") {
                event.rename("checkpoint.matched_category", "rule.category")?;
            }

            let _cond = { !event.has_value("rule.category") };
            if _cond {
                if event.has("checkpoint.categories") {
                    event.rename("checkpoint.categories", "rule.category")?;
                }
            }

            if event.has("checkpoint.malware_action") {
                event.rename("checkpoint.malware_action", "rule.description")?;
            }

            if event.has("checkpoint.malware_rule_id") {
                event.rename("checkpoint.malware_rule_id", "rule.id")?;
            }

            let _cond = { !event.has_value("rule.id") };
            if _cond {
                if event.has("checkpoint.app_rule_id") {
                    event.rename("checkpoint.app_rule_id", "rule.id")?;
                }
            }

            if event.has("checkpoint.objectname") {
                event.rename("checkpoint.objectname", "rule.name")?;
            }

            let _cond = { !event.has_value("rule.name") };
            if _cond {
                if event.has("checkpoint.rule_name") {
                    event.rename("checkpoint.rule_name", "rule.name")?;
                }
            }

            let _cond = { !event.has_value("rule.name") };
            if _cond {
                if event.has("checkpoint.malware_rule_name") {
                    event.rename("checkpoint.malware_rule_name", "rule.name")?;
                }
            }

            let _cond = { !event.has_value("rule.name") };
            if _cond {
                if event.has("checkpoint.app_rule_name") {
                    event.rename("checkpoint.app_rule_name", "rule.name")?;
                }
            }

            let _cond = { !event.has_value("rule.name") };
            if _cond {
                if event.has("checkpoint.dlp_rule_name") {
                    event.rename("checkpoint.dlp_rule_name", "rule.name")?;
                }
            }

            if event.has("checkpoint.smartdefence_profile") {
                event.rename("checkpoint.smartdefence_profile", "rule.ruleset")?;
            }

            let _cond = { !event.has_value("rule.ruleset") };
            if _cond {
                if event.has("checkpoint.policy") {
                    event.rename("checkpoint.policy", "rule.ruleset")?;
                }
            }

            if event.has("checkpoint.rule_uid") {
                event.rename("checkpoint.rule_uid", "rule.uuid")?;
            }

            let _cond = { !event.has_value("rule.uuid") };
            if _cond {
                if event.has("checkpoint.dlp_rule_uid") {
                    event.rename("checkpoint.dlp_rule_uid", "rule.uuid")?;
                }
            }

            if event.has("checkpoint.url") {
                event.rename("checkpoint.url", "url.original")?;
            }

            let _cond = { !event.has_value("url.original") };
            if _cond {
                if event.has("checkpoint.resource") {
                    event.rename("checkpoint.resource", "url.original")?;
                }
            }

            if event.has("checkpoint.http_host") {
                event.rename("checkpoint.http_host", "url.domain")?;
            }

            if event.has("checkpoint.web_client_type") {
                event.rename("checkpoint.web_client_type", "user_agent.name")?;
            }

            if event.has("checkpoint.user_agent") {
                event.rename("checkpoint.user_agent", "user_agent.original")?;
            }

            if event.has("checkpoint.industry_reference") {
                event.rename("checkpoint.industry_reference", "vulnerability.id")?;
            }

            let _cond = {
                event.has_value("checkpoint.time")
                    && (event.get("checkpoint.time").is_some_and(|v| v.is_string())
                        || event.get("checkpoint.time").is_some_and(|v| v.is_array()))
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ZonedDateTime convert(String time) {\n  ZonedDateTime zdt;\n  try {\n    Instant instant;\n    long temp_time = Long.parseLong(time);\n    if (String.valueOf(temp_time).length() > 10) {\n      instant = Instant.ofEpochMilli(temp_time);\n    } else {\n      instant = Instant.ofEpochMilli(temp_time * 1000L);\n    }\n    zdt = ZonedDateTime.ofInstant(instant, ZoneId.of('Z'));\n  }\n  catch (NumberFormatException nfe) {\n    zdt = ZonedDateTime.parse(time);\n  }\n  return zdt\n}\n\n// Handle single time field.\nif (ctx.checkpoint.time instanceof String) {\n  ctx.checkpoint._temp_unixms = convert(ctx.checkpoint.time);\n  return;\n}\n\n// Some log lines have more than one time. Pick the earliest and retain all.\nList zdt = new ArrayList();\nfor (def time: ctx.checkpoint.time) {\n  zdt.add(convert(time));\n}\nctx.checkpoint.times = zdt;\nctx.checkpoint._temp_unixms = Collections.min(zdt);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ZonedDateTime convert(String time) {\n  ZonedDateTime zdt;\n  try {\n    Instant instant;\n    long temp_time = Long.parseLong(time);\n    if (String.valueOf(temp_time).length() > 10) {\n      instant = Instant.ofEpochMilli(temp_time);\n    } else {\n      instant = Instant.ofEpochMilli(temp_time * 1000L);\n    }\n    zdt = ZonedDateTime.ofInstant(instant, ZoneId.of('Z'));\n  }\n  catch (NumberFormatException nfe) {\n    zdt = ZonedDateTime.parse(time);\n  }\n  return zdt\n}\n\n// Handle single time field.\nif (ctx.checkpoint.time instanceof String) {\n  ctx.checkpoint._temp_unixms = convert(ctx.checkpoint.time);\n  return;\n}\n\n// Some log lines have more than one time. Pick the earliest and retain all.\nList zdt = new ArrayList();\nfor (def time: ctx.checkpoint.time) {\n  zdt.add(convert(time));\n}\nctx.checkpoint.times = zdt;\nctx.checkpoint._temp_unixms = Collections.min(zdt);\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "checkpoint_time_conversion_script",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "fail-{}",
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    return Err(TransformError::ParseError {
                        path: "_fail".into(),
                        message: (format!(
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
                        ))
                        .to_string(),
                    });
                }
            }

            if event.has("checkpoint._temp_unixms") {
                event.rename("checkpoint._temp_unixms", "@timestamp")?;
            }

            let _cond = { event.has_value("checkpoint.last_hit_time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("checkpoint.last_hit_time") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "checkpoint.last_hit_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("checkpoint.lastupdatetime") };
            if _cond {
                if let Some(date_str) = event.get_as_string("checkpoint.lastupdatetime") {
                    match parse_date_out(&date_str, &["UNIX"], None, None) {
                        Some(parsed) => event.set("event.end", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "checkpoint.lastupdatetime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("checkpoint.creation_time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("checkpoint.creation_time") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX"], None, None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "checkpoint.creation_time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("checkpoint.login_timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("checkpoint.login_timestamp") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX"], None, None) {
                        Some(parsed) => event.set("checkpoint.login_timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "checkpoint.login_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has("checkpoint.duration") {
                event.rename("checkpoint.duration", "event.duration")?;
            }

            if event.has_value("event.duration") {
                if let Some(val) = event.get("event.duration") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "event.duration".into(),
                            message,
                        }
                    })?;
                    event.set("event.duration", converted)?;
                }
            }

            if event.has_value("checkpoint.session_timeout") {
                if let Some(val) = event.get("checkpoint.session_timeout") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "checkpoint.session_timeout".into(),
                            message,
                        }
                    })?;
                    event.set("checkpoint.session_timeout", converted)?;
                }
            }

            let _cond = { event.has_value("event.duration") };
            if _cond {
                // Painless script
                // Source: ctx.event.duration = ctx.event.duration * 1000000000L
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(r#"ctx.event.duration = ctx.event.duration * 1000000000L"#),
                )?;
            }

            if event.has_value("checkpoint.update_count") {
                if let Some(val) = event.get("checkpoint.update_count") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "checkpoint.update_count".into(),
                            message,
                        }
                    })?;
                    event.set("checkpoint.update_count", converted)?;
                }
            }

            if event.has_value("checkpoint.connection_count") {
                if let Some(val) = event.get("checkpoint.connection_count") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "checkpoint.connection_count".into(),
                            message,
                        }
                    })?;
                    event.set("checkpoint.connection_count", converted)?;
                }
            }

            if event.has_value("checkpoint.aggregated_log_count") {
                if let Some(val) = event.get("checkpoint.aggregated_log_count") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "checkpoint.aggregated_log_count".into(),
                            message,
                        }
                    })?;
                    event.set("checkpoint.aggregated_log_count", converted)?;
                }
            }

            if event.has("checkpoint.message") {
                event.rename("checkpoint.message", "message")?;
            }

            let _cond = { !event.has_value("message") };
            if _cond {
                if event.has("checkpoint.reason") {
                    event.rename("checkpoint.reason", "message")?;
                }
            }

            let _cond = { !event.has_value("message") };
            if _cond {
                if event.has("checkpoint.subject") {
                    event.rename("checkpoint.subject", "message")?;
                }
            }

            let _cond = { event.has_value("checkpoint.sys_message") };
            if _cond {
                gsub_field(
                    event,
                    "checkpoint.sys_message",
                    "checkpoint.sys_message",
                    cached_regex!("^:\""),
                    "",
                )?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
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
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
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

            let _cond = { event.has_value("user.full_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.full_name")
                            .map_or_else(String::new, template_to_string)
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
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { !event.has_value("source.user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("source.user.email") {
                        if let Some(input) = event.get_string("source.user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("source.user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("source.user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("source.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("source.user.full_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.full_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { !event.has_value("destination.user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("destination.user.email") {
                        if let Some(input) = event.get_string("destination.user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("destination.user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("destination.user.domain", remaining));
                                true
                            };
                            if matched {
                                for (path, value) in captured {
                                    event.set(path, value)?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("destination.user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("destination.user.full_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.full_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("source.bytes")
                    && event.has_value("destination.bytes")
                    && !event.has_value("network.bytes")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.network.bytes = ctx.source.bytes + ctx.destination.bytes"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("source.packets")
                    && event.has_value("destination.packets")
                    && !event.has_value("network.packets")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.network.packets = ctx.source.packets + ctx.destination.packets
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.network.packets = ctx.source.packets + ctx.destination.packets"#
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("checkpoint.action_reason")
                    && event
                        .get("checkpoint.action_reason")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some(" "))
                            }
                            serde_json::Value::String(s) => s.contains(" "),
                            _ => false,
                        })
            };
            if _cond {
                if event.has("checkpoint.action_reason") {
                    event.rename("checkpoint.action_reason", "checkpoint.action_reason_msg")?;
                }
            }

            let _cond = { !event.has_value("source.geo") };
            if _cond {
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
            }

            let _cond = { !event.has_value("destination.geo") };
            if _cond {
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

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
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

            let _cond = {
                event.has_value("_temp_.external_zones")
                    && event.has_value("_temp_.internal_zones")
                    && event.has_value("observer.ingress.zone")
                    && event.has_value("observer.egress.zone")
                    && event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
                    && event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
            };
            if _cond {
                event.set("network.direction", json!("inbound"))?;
            }

            let _cond = {
                event.has_value("_temp_.external_zones")
                    && event.has_value("_temp_.internal_zones")
                    && event.has_value("observer.ingress.zone")
                    && event.has_value("observer.egress.zone")
                    && event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
                    && event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
            };
            if _cond {
                event.set("network.direction", json!("outbound"))?;
            }

            let _cond = {
                event.has_value("_temp_.external_zones")
                    && event.has_value("_temp_.internal_zones")
                    && event.has_value("observer.ingress.zone")
                    && event.has_value("observer.egress.zone")
                    && event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
                    && event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
            };
            if _cond {
                event.set("network.direction", json!("internal"))?;
            }

            let _cond = {
                event.has_value("_temp_.external_zones")
                    && event.has_value("_temp_.internal_zones")
                    && event.has_value("observer.ingress.zone")
                    && event.has_value("observer.egress.zone")
                    && event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
                    && event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })
            };
            if _cond {
                event.set("network.direction", json!("external"))?;
            }

            let _cond = {
                event.has_value("_temp_.external_zones")
                    && event.has_value("_temp_.internal_zones")
                    && event.has_value("observer.ingress.zone")
                    && event.has_value("observer.egress.zone")
                    && ((!(event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })) && !(event.get("_temp_.internal_zones").is_some_and(|v| {
                        match (v, event.get("observer.egress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    }))) || (!(event.get("_temp_.external_zones").is_some_and(|v| {
                        match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        }
                    })) && !(event.get("_temp_.internal_zones").is_some_and(
                        |v| match (v, event.get("observer.ingress.zone")) {
                            (serde_json::Value::Array(a), Some(n)) => a.iter().any(|x| x == n),
                            (serde_json::Value::String(s), Some(serde_json::Value::String(n))) => {
                                s.contains(n.as_str())
                            }
                            _ => false,
                        },
                    ))))
            };
            if _cond {
                event.set("network.direction", json!("unknown"))?;
            }

            event.remove("checkpoint.ifname");
            event.remove("checkpoint.server_outbound_interface");
            event.remove("checkpoint.client_outbound_packets");
            event.remove("checkpoint.server_outbound_packets");
            event.remove("checkpoint.client_outbound_bytes");
            event.remove("checkpoint.server_outbound_bytes");
            event.remove("checkpoint.client_inbound_packets");
            event.remove("checkpoint.server_inbound_packets");
            event.remove("checkpoint.client_inbound_bytes");
            event.remove("checkpoint.server_inbound_bytes");
            event.remove("checkpoint.sent_byte");
            event.remove("checkpoint.received_bytes");
            event.remove("checkpoint.service");
            event.remove("checkpoint.xlatedport");
            event.remove("checkpoint.s_port");
            event.remove("checkpoint.xlatesport");
            event.remove("checkpoint.contextnum");
            event.remove("checkpoint.sequencenum");
            event.remove("checkpoint.file_size");
            event.remove("checkpoint.product");
            event.remove("checkpoint.severity");
            event.remove("checkpoint.xlatesrc");
            event.remove("checkpoint.xlatedst");
            event.remove("checkpoint.uid");
            event.remove("checkpoint.time");
            event.remove("checkpoint.__nsons");
            event.remove("checkpoint.__p_dport");
            event.remove("checkpoint.__pos");
            event.remove("checkpoint.hll_key");
            event.remove("checkpoint.segment_time");
            event.remove("checkpoint.lastupdatetime");
            event.remove("checkpoint.last_hit_time");
            event.remove("checkpoint.creation_time");
            event.remove("checkpoint.endpoint_ip");
            event.remove("checkpoint.origin_ip");
            event.remove("syslog5424_ts");
            event.remove("_temp_");
            event.remove("_conf");

            if event.has("checkpoint.times") {
                event.rename("checkpoint.times", "checkpoint.time")?;
            }

            let _cond = { event.has_value("checkpoint.time") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("checkpoint.time") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event.set("checkpoint.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "checkpoint.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })();
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(value -> dropEmptyFields(value));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

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
