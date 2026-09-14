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

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^(?:<%{NUMBER:log.syslog.priority:long}>)?%{SYSLOGTIMESTAMP:_tmp.timestamp} %{ISO8601_TIMEZONE:_tmp.timezone} %{DATA:host.hostname} %{DATA:cisco_ise.log.category.name} %{GREEDYDATA:_tmp.message}$
                    // Grok pattern: ^(?:<%{NUMBER:log.syslog.priority:long}>)?%{SYSLOGTIMESTAMP:_tmp.timestamp} %{DATA:host.hostname} %{DATA:cisco_ise.log.category.name} %{GREEDYDATA:_tmp.message}$
                    // Grok pattern: ^(?P<_tmp_timestamp>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?))%{ISO8601_TIMEZONE:_tmp.timezone} %{DATA:host.hostname} %{DATA:cisco_ise.log.category.name} %{GREEDYDATA:_tmp.message}$
                    // Grok pattern: ^%{DATA:cisco_ise.log.category.name} %{DATA:cisco_ise.log.message.id} %{NONNEGINT:cisco_ise.log.segment.total:long} %{NONNEGINT:cisco_ise.log.segment.number:long} %{GREEDYDATA:_tmp.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^(?:<%{NUMBER:log.syslog.priority:long}>)?%{SYSLOGTIMESTAMP:_tmp.timestamp} %{ISO8601_TIMEZONE:_tmp.timezone} %{DATA:host.hostname} %{DATA:cisco_ise.log.category.name} %{GREEDYDATA:_tmp.message}$"
                            ),
                            cached_grok!(
                                "^(?:<%{NUMBER:log.syslog.priority:long}>)?%{SYSLOGTIMESTAMP:_tmp.timestamp} %{DATA:host.hostname} %{DATA:cisco_ise.log.category.name} %{GREEDYDATA:_tmp.message}$"
                            ),
                            cached_grok_mapped!(
                                "^(?P<_tmp_timestamp>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?))%{ISO8601_TIMEZONE:_tmp.timezone} %{DATA:host.hostname} %{DATA:cisco_ise.log.category.name} %{GREEDYDATA:_tmp.message}$",
                                [("_tmp_timestamp", "_tmp.timestamp")]
                            ),
                            cached_grok!(
                                "^%{DATA:cisco_ise.log.category.name} %{DATA:cisco_ise.log.message.id} %{NONNEGINT:cisco_ise.log.segment.total:long} %{NONNEGINT:cisco_ise.log.segment.number:long} %{GREEDYDATA:_tmp.message}$"
                            ),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "grok")?;
                event.set("_ingest.on_failure_processor_tag", "grok_time_details")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "{}: {}",
                        event
                            .get("_ingest.on_failure_processor_tag")
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

            let _cond = { event.has_value("_tmp.message") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("_tmp.message") {
                        // Grok pattern: ^(?P<cisco_ise_log_log_severity_level>(?:[^:]*?))%{SPACE}: (?P<event_action>(?:[^:]*?))%{SPACE}: %{GREEDYDATA:message}$
                        // Grok pattern: ^%{NUMBER:cisco_ise.log.message.id} %{NUMBER:cisco_ise.log.segment.total:long} %{NUMBER:cisco_ise.log.segment.number:long} %{GREEDYDATA:message}$
                        // Grok pattern: ^%{GREEDYDATA:message}$
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "^(?P<cisco_ise_log_log_severity_level>(?:[^:]*?))%{SPACE}: (?P<event_action>(?:[^:]*?))%{SPACE}: %{GREEDYDATA:message}$",
                                    [
                                        (
                                            "cisco_ise_log_log_severity_level",
                                            "cisco_ise.log.log_severity_level"
                                        ),
                                        ("event_action", "event.action")
                                    ]
                                ),
                                cached_grok!(
                                    "^%{NUMBER:cisco_ise.log.message.id} %{NUMBER:cisco_ise.log.segment.total:long} %{NUMBER:cisco_ise.log.segment.number:long} %{GREEDYDATA:message}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set("_ingest.on_failure_processor_tag", "grok_message_details")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "{}: {}",
                            event
                                .get("_ingest.on_failure_processor_tag")
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                map_strings(event, "message", "message", |s| s.trim().to_string())?;
                Ok(())
            })();

            let _cond =
                { event.has_value("host.hostname") && event.get_str("host.hostname") != Some("") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("host.hostname") {
                        if let Some(val) = event.get("host.hostname") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "host.hostname".into(),
                                    message,
                                }
                            })?;
                            event.set("host.ip", converted)?;
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("host.ip") };
            if _cond {
                if event.remove("host.hostname").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "host.hostname".into(),
                    });
                }
            }

            let _cond = { event.has_value("host.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("host.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

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

            let _cond = { event.has_value("_conf.tz_offset") };
            if _cond {
                if event.has_value("_conf.tz_offset") {
                    event.rename("_conf.tz_offset", "event.timezone")?;
                }
            }

            if let Some(v) = event
                .get("_tmp.timezone")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.timezone", v)?;
            }

            let _cond = { event.has_value("_tmp.timestamp") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["MMM [ ]d[d] HH:mm:ss[.SSSSSS][.SSS]", "ISO8601"],
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
                        "default-tmp-timestamp parsing",
                    )?;
                    event.remove("_tmp.timestamp");
                    event.append(
                        "error.message",
                        json!(format!(
                            "{}: {}",
                            event
                                .get("_ingest.on_failure_processor_tag")
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
                event.has_value("_tmp.timestamp")
                    && event.has_value("event.timezone")
                    && event.get_str("event.timezone") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["MMM [ ]d[d] HH:mm:ss[.SSSSSS][.SSS]", "ISO8601"],
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
                        "default-tmp-timestamp-with timezone parsing",
                    )?;
                    event.remove("_tmp.timestamp");
                    event.append(
                        "error.message",
                        json!(format!(
                            "{}: {}",
                            event
                                .get("_ingest.on_failure_processor_tag")
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

            let _cond =
                { event.get_str("cisco_ise.log.category.name") == Some("CISE_Policy_Diagnostics") };
            if _cond {
                // Begin nested pipeline: "pipeline_policy_diagnostics"
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
                        let mut kv_gap = false;
                        for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                    event.has_value("cisco_ise.log.message.description")
                        && event.get_str("cisco_ise.log.message.description") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                            // Grok pattern: ^%{DATA:event.action}:
                            if !cached_grok!("^%{DATA:event.action}:")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Device IP Address") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Device IP Address")
                        {
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
                if event.has_value("cisco_ise.log.log_details.Protocol") {
                    event.rename("cisco_ise.log.log_details.Protocol", "network.protocol")?;
                }
                if event.has_value("network.protocol") {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                }
                let _cond = { event.has_value("cisco_ise.log.log_details.RequestReceivedTime") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("cisco_ise.log.log_details.RequestReceivedTime")
                        {
                            match parse_date_out(&date_str, &["UNIX"], None, None) {
                                Some(parsed) => {
                                    event.set("cisco_ise.log.request.received_time", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "cisco_ise.log.log_details.RequestReceivedTime"
                                            .into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_cisco_ise_log_log_details_RequestReceivedTime_to_cisco_ise_log_request_received_time_c877ce0d")?;
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
                event.remove("cisco_ise.log.log_details.RequestReceivedTime");
                if event.has_value("cisco_ise.log.log_details.PolicyType") {
                    event.rename(
                        "cisco_ise.log.log_details.PolicyType",
                        "cisco_ise.log.policy.type",
                    )?;
                }
                if event.has_value("cisco_ise.log.log_details.AcsSessionID") {
                    event.rename(
                        "cisco_ise.log.log_details.AcsSessionID",
                        "cisco_ise.log.acs.session.id",
                    )?;
                }
                if event.has_value("cisco_ise.log.log_details.AuthorizationPolicyMatchedRule") {
                    event.rename(
                        "cisco_ise.log.log_details.AuthorizationPolicyMatchedRule",
                        "cisco_ise.log.auth.policy.matched.rule",
                    )?;
                }
                if event.has_value("cisco_ise.log.log_details.CurrentIDStoreName") {
                    event.rename(
                        "cisco_ise.log.log_details.CurrentIDStoreName",
                        "cisco_ise.log.currentid.store_name",
                    )?;
                }
                if event.has_value("cisco_ise.log.log_details.ISEPolicySetName") {
                    event.rename(
                        "cisco_ise.log.log_details.ISEPolicySetName",
                        "cisco_ise.log.ise.policy.set_name",
                    )?;
                }
                if event.has_value("cisco_ise.log.log_details.IdentityPolicyMatchedRule") {
                    event.rename(
                        "cisco_ise.log.log_details.IdentityPolicyMatchedRule",
                        "cisco_ise.log.identity.policy.matched.rule",
                    )?;
                }
                if event.has_value("cisco_ise.log.log_details.IdentitySelectionMatchedRule") {
                    event.rename(
                        "cisco_ise.log.log_details.IdentitySelectionMatchedRule",
                        "cisco_ise.log.identity.selection.matched.rule",
                    )?;
                }
                let _cond = { event.has_value("cisco_ise.log.log_details.OriginalUserName") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.name",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.OriginalUserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_ise.log.log_details.OriginalUserName") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.OriginalUserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.remove("cisco_ise.log.log_details.OriginalUserName");
                if event.has_value("cisco_ise.log.log_details.SelectedAccessService") {
                    event.rename(
                        "cisco_ise.log.log_details.SelectedAccessService",
                        "cisco_ise.log.selected.access.service",
                    )?;
                }
                if event.has_value("cisco_ise.log.log_details.SelectedAuthorizationProfiles") {
                    event.rename(
                        "cisco_ise.log.log_details.SelectedAuthorizationProfiles",
                        "cisco_ise.log.selected.authorization.profiles",
                    )?;
                }
                let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.name",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.UserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.UserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.remove("cisco_ise.log.log_details.UserName");
                if event.has_value("cisco_ise.log.log_details.CPMSessionID") {
                    event.rename(
                        "cisco_ise.log.log_details.CPMSessionID",
                        "cisco_ise.log.cpm.session.id",
                    )?;
                }
                event.remove("_tmp");
                event.remove("cisco_ise.log.log_details_raw");
                // End nested pipeline: "pipeline_policy_diagnostics"
            }

            let _cond = { event.get_str("cisco_ise.log.category.name") == Some("CISE_Guest") };
            if _cond {
                // Begin nested pipeline: "pipeline_guest"
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
                                "MMM [ ]d[d] HH:mm:ss[.SSSSSS][.SSS]",
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
                    event.set("_ingest.on_failure_processor_tag", "guest-first-date")?;
                    event.remove("_tmp.timestamp");
                    event.append(
                        "error.message",
                        json!(format!(
                            "{}: {}",
                            event
                                .get("_ingest.on_failure_processor_tag")
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
                                    "MMM [ ]d[d] HH:mm:ss[.SSSSSS][.SSS]",
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
                        event.set("_ingest.on_failure_processor_tag", "guest-second-date")?;
                        event.remove("_tmp.timestamp");
                        event.append(
                            "error.message",
                            json!(format!(
                                "{}: {}",
                                event
                                    .get("_ingest.on_failure_processor_tag")
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                        let mut kv_gap = false;
                        for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.UserType") {
                        event.rename(
                            "cisco_ise.log.log_details.UserType",
                            "cisco_ise.log.user.type",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.GuestUserName") {
                        event.rename("cisco_ise.log.log_details.GuestUserName", "user.name")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("user.name")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("cisco_ise.log.guest.user.name", v)?;
                    }
                    Ok(())
                })();
                let _cond = { !event.has_value("user.name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("cisco_ise.log.log_details.UserName") {
                            event.rename("cisco_ise.log.log_details.UserName", "user.name")?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { !event.has_value("cisco_ise.log.guest.user.name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event
                            .get("user.name")
                            .filter(|v| !painless_is_empty_value(v))
                            .cloned()
                        {
                            event.set("cisco_ise.log.guest.user.name", v)?;
                        }
                        Ok(())
                    })();
                }
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IpAddress") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.IpAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.IpAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cisco_ise_log_log_details_IpAddress_to_source_ip_e43c3b18",
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
                event.remove("cisco_ise.log.log_details.IpAddress");
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("source.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AuthenticationIdentityStore") {
                        event.rename(
                            "cisco_ise.log.log_details.AuthenticationIdentityStore",
                            "cisco_ise.log.authentication.identity_store",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.PortalName") {
                        event.rename(
                            "cisco_ise.log.log_details.PortalName",
                            "cisco_ise.log.portal.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IdentityGroup") {
                        event.rename(
                            "cisco_ise.log.log_details.IdentityGroup",
                            "cisco_ise.log.identity.group",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.PsnHostName") {
                        event.rename(
                            "cisco_ise.log.log_details.PsnHostName",
                            "cisco_ise.log.psn.hostname",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.ResponseTime") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.ResponseTime") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.ResponseTime".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.response.time", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_ResponseTime_to_cisco_ise_log_response_time_1c336c93")?;
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
                event.remove("cisco_ise.log.log_details.ResponseTime");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.FailureReason") {
                        event.rename(
                            "cisco_ise.log.log_details.FailureReason",
                            "cisco_ise.log.failure.reason",
                        )?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "pipeline_guest"
            }

            let _cond = { event.get_str("cisco_ise.log.category.name") == Some("CISE_MyDevices") };
            if _cond {
                // Begin nested pipeline: "pipeline_mydevices"
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
                        let mut kv_gap = false;
                        for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.UserName") {
                        event.rename("cisco_ise.log.log_details.UserName", "user.name")?;
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IpAddress") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.IpAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.IpAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cisco_ise_log_log_details_IpAddress_to_source_ip_e43c3b18",
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
                event.remove("cisco_ise.log.log_details.IpAddress");
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("source.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AuthenticationIdentityStore") {
                        event.rename(
                            "cisco_ise.log.log_details.AuthenticationIdentityStore",
                            "cisco_ise.log.authentication.identity_store",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.PortalName") {
                        event.rename(
                            "cisco_ise.log.log_details.PortalName",
                            "cisco_ise.log.portal.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IdentityGroup") {
                        event.rename(
                            "cisco_ise.log.log_details.IdentityGroup",
                            "cisco_ise.log.identity.group",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.PsnHostName") {
                        event.rename(
                            "cisco_ise.log.log_details.PsnHostName",
                            "cisco_ise.log.psn.hostname",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.EPMacAddress") {
                        event.rename(
                            "cisco_ise.log.log_details.EPMacAddress",
                            "cisco_ise.log.ep.mac.address",
                        )?;
                    }
                    Ok(())
                })();
                if event.has_value("cisco_ise.log.ep.mac.address") {
                    gsub_field(
                        event,
                        "cisco_ise.log.ep.mac.address",
                        "cisco_ise.log.ep.mac.address",
                        cached_regex!("[-:.]"),
                        "-",
                    )?;
                }
                if event.has_value("cisco_ise.log.ep.mac.address") {
                    map_strings(
                        event,
                        "cisco_ise.log.ep.mac.address",
                        "cisco_ise.log.ep.mac.address",
                        str::to_uppercase,
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("cisco_ise.log.ep.mac.address")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("client.mac", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.EPIdentityGroup") {
                        event.rename(
                            "cisco_ise.log.log_details.EPIdentityGroup",
                            "cisco_ise.log.ep.identity_group",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Staticassignment") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Staticassignment") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Staticassignment".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.static.assignment", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Staticassignment_to_cisco_ise_log_static_assignment_0a5c5e91")?;
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
                event.remove("cisco_ise.log.log_details.Staticassignment");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.EndPointProfiler") {
                        event.rename(
                            "cisco_ise.log.log_details.EndPointProfiler",
                            "cisco_ise.log.endpoint.profiler",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.EndPointPolicy") {
                        event.rename(
                            "cisco_ise.log.log_details.EndPointPolicy",
                            "cisco_ise.log.endpoint.policy",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.DeviceName") {
                        event.rename(
                            "cisco_ise.log.log_details.DeviceName",
                            "cisco_ise.log.device.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.DeviceRegistrationStatus") {
                        event.rename(
                            "cisco_ise.log.log_details.DeviceRegistrationStatus",
                            "cisco_ise.log.device.registration_status",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("cisco_ise.log.log_details.ResponseTime") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_ise.log.log_details.ResponseTime".into(),
                                message,
                            }
                        })?;
                        event.set("cisco_ise.log.response.time", converted)?;
                    }
                    Ok(())
                })();
                event.remove("cisco_ise.log.log_details.ResponseTime");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.EndpointCoA") {
                        event.rename(
                            "cisco_ise.log.log_details.EndpointCoA",
                            "cisco_ise.log.endpoint.coa",
                        )?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "pipeline_mydevices"
            }

            let _cond = {
                event.get_str("cisco_ise.log.category.name")
                    == Some("CISE_Internal_Operations_Diagnostics")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_internal_operations_diagnostics"
                event.set("event.kind", json!("event"))?;
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
                let _cond = {
                    event.has_value("cisco_ise.log.message.description")
                        && event.get_str("cisco_ise.log.message.description") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                            // Grok pattern: ^%{DATA:event.action}:
                            if !cached_grok!("^%{DATA:event.action}:")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("34120")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["32025", "34126", "34127"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("configuration"))?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                        let mut kv_gap = false;
                        for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.DestinationPort") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.DestinationPort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.DestinationPort".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationPort_to_destination_port_ad144c54")?;
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
                event.remove("cisco_ise.log.log_details.DestinationPort");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.LoggerName") {
                        event.rename(
                            "cisco_ise.log.log_details.LoggerName",
                            "cisco_ise.log.logger.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.LogFileName") {
                        event.rename(
                            "cisco_ise.log.log_details.LogFileName",
                            "cisco_ise.log.file.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.LogErrorMessage") {
                        event.rename(
                            "cisco_ise.log.log_details.LogErrorMessage",
                            "cisco_ise.log.error.message",
                        )?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "pipeline_internal_operations_diagnostics"
            }

            let _cond =
                { event.get_str("cisco_ise.log.category.name") == Some("CISE_Threat_Centric_NAC") };
            if _cond {
                // Begin nested pipeline: "pipeline_threat_centric_nac"
                event.set("event.kind", json!("event"))?;
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
                let _cond = {
                    event.has_value("cisco_ise.log.message.description")
                        && event.get_str("cisco_ise.log.message.description") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                            // Grok pattern: ^%{DATA:event.action}:
                            if !cached_grok!("^%{DATA:event.action}:")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("91110")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["91004", "91018"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("configuration"))?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_ise.log.message.code") };
                if _cond {
                    // Painless script
                    // Source: def eventCategory = new ArrayList();\ndef categoryReferenceTable = [\n  [\"messageCodeArray\": [\"91110\"], \"name\": \"authentication\"],\n  [\"messageCodeArray\": [\"91004\",\"91018\"], \"name\": \"configuration\"]\n];\nfor (entry in categoryReferenceTable) {\n  if (entry.messageCodeArray.contains(ctx.cisco_ise.log.message.code)) {\n    eventCategory.add(entry.name);\n  }\n}\nctx.event.action = ctx.cisco_ise?.log?.message?.description?.splitOnToken(\":\")[0]?.toLowerCase();\nctx.event.category = eventCategory;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def eventCategory = new ArrayList();\ndef categoryReferenceTable = [\n  [\"messageCodeArray\": [\"91110\"], \"name\": \"authentication\"],\n  [\"messageCodeArray\": [\"91004\",\"91018\"], \"name\": \"configuration\"]\n];\nfor (entry in categoryReferenceTable) {\n  if (entry.messageCodeArray.contains(ctx.cisco_ise.log.message.code)) {\n    eventCategory.add(entry.name);\n  }\n}\nctx.event.action = ctx.cisco_ise?.log?.message?.description?.splitOnToken(\":\")[0]?.toLowerCase();\nctx.event.category = eventCategory;\n"#
                        ),
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                        let mut kv_gap = false;
                        for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Details") {
                        event
                            .rename("cisco_ise.log.log_details.Details", "cisco_ise.log.details")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AdapterInstanceName") {
                        event.rename(
                            "cisco_ise.log.log_details.AdapterInstanceName",
                            "cisco_ise.log.adapter_instance.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AdapterInstanceUuid") {
                        event.rename(
                            "cisco_ise.log.log_details.AdapterInstanceUuid",
                            "cisco_ise.log.adapter_instance.uuid",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Status") {
                        event.rename("cisco_ise.log.log_details.Status", "cisco_ise.log.status")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Connectivity") {
                        event.rename(
                            "cisco_ise.log.log_details.Connectivity",
                            "cisco_ise.log.connectivity",
                        )?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "pipeline_threat_centric_nac"
            }

            let _cond = {
                event.get_str("cisco_ise.log.category.name")
                    == Some("CISE_Posture_and_Client_Provisioning_Audit")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_posture_and_client_provisioning_audit"
                event.set("event.kind", json!("event"))?;
                event.append("event.category", json!("malware"))?;
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
                            "date__tmp_timestamp_d0798b77",
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
                        let mut kv_gap = false;
                        for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.OperationID") {
                        event.rename(
                            "cisco_ise.log.log_details.OperationID",
                            "cisco_ise.log.operation.id",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.OperationType") {
                        event.rename(
                            "cisco_ise.log.log_details.OperationType",
                            "cisco_ise.log.operation.type",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.OperationStatus") {
                        event.rename(
                            "cisco_ise.log.log_details.OperationStatus",
                            "cisco_ise.log.operation.status",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AdminName") {
                        event.rename("cisco_ise.log.log_details.AdminName", "client.user.name")?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "pipeline_posture_and_client_provisioning_audit"
            }

            let _cond =
                { event.get_str("cisco_ise.log.category.name") == Some("CISE_RADIUS_Accounting") };
            if _cond {
                // Begin nested pipeline: "pipeline_radius_accounting"
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
                                "ISO8601",
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
                        "date__tmp_timestamp_to_@timestamp_247df7be",
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
                            "date__tmp_timestamp_d0798b77",
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
                        let mut kv_gap = false;
                        for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                    event.has_value("cisco_ise.log.message.description")
                        && event.get_str("cisco_ise.log.message.description") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                            // Grok pattern: ^%{DATA:event.action}:
                            if !cached_grok!("^%{DATA:event.action}:")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Device IP Address") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Device IP Address")
                        {
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
                    if event.has_value("cisco_ise.log.log_details.User-Name") {
                        event.rename("cisco_ise.log.log_details.User-Name", "user.name")?;
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NAS-IP-Address") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.NAS-IP-Address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.NAS-IP-Address".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.nas.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_NAS-IP-Address_to_cisco_ise_log_nas_ip_ae27d25e")?;
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
                event.remove("cisco_ise.log.log_details.NAS-IP-Address");
                let _cond = { event.has_value("cisco_ise.log.nas.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("cisco_ise.log.nas.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NAS-Port") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.NAS-Port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.NAS-Port".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.nas.port.number", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_NAS-Port_to_cisco_ise_log_nas_port_number_8385ddaf")?;
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
                event.remove("cisco_ise.log.log_details.NAS-Port");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Framed-IP-Address") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Framed-IP-Address")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Framed-IP-Address".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.framed.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Framed-IP-Address_to_cisco_ise_log_framed_ip_a7ec88de")?;
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
                event.remove("cisco_ise.log.log_details.Framed-IP-Address");
                let _cond = { event.has_value("cisco_ise.log.framed.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("cisco_ise.log.framed.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Class") {
                        event.rename("cisco_ise.log.log_details.Class", "cisco_ise.log.class")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Called-Station-ID") {
                        event.rename(
                            "cisco_ise.log.log_details.Called-Station-ID",
                            "cisco_ise.log.called_station.id",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Calling-Station-ID") {
                        event.rename(
                            "cisco_ise.log.log_details.Calling-Station-ID",
                            "cisco_ise.log.calling_station.id",
                        )?;
                    }
                    Ok(())
                })();
                if event.has_value("cisco_ise.log.log_details.NAS-Identifier") {
                    event.rename(
                        "cisco_ise.log.log_details.NAS-Identifier",
                        "cisco_ise.log.nas.identifier",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Acct-Status-Type") {
                        event.rename(
                            "cisco_ise.log.log_details.Acct-Status-Type",
                            "cisco_ise.log.acct.status.type",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Acct-Session-Id") {
                        event.rename(
                            "cisco_ise.log.log_details.Acct-Session-Id",
                            "cisco_ise.log.acct.session.id",
                        )?;
                    }
                    Ok(())
                })();
                if event.has_value("cisco_ise.log.log_details.Acct-Authentic") {
                    event.rename(
                        "cisco_ise.log.log_details.Acct-Authentic",
                        "cisco_ise.log.acct.authentic",
                    )?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Acct-Session-Time") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Acct-Session-Time")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Acct-Session-Time".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.acct.session.time", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Acct-Session-Time_to_cisco_ise_log_acct_session_time_5b0d7897")?;
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
                event.remove("cisco_ise.log.log_details.Acct-Session-Time");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Step") {
                        event.rename("cisco_ise.log.log_details.Step", "cisco_ise.log.step")?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("cisco_ise.log.log_details.Event-Timestamp") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("cisco_ise.log.log_details.Event-Timestamp")
                        {
                            match parse_date_out(&date_str, &["UNIX"], None, None) {
                                Some(parsed) => {
                                    event.set("cisco_ise.log.event.timestamp", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "cisco_ise.log.log_details.Event-Timestamp".into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_cisco_ise_log_log_details_Event-Timestamp_to_cisco_ise_log_event_timestamp_1b4cee3a")?;
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
                event.remove("cisco_ise.log.log_details.Event-Timestamp");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NAS-Port-Type") {
                        event.rename(
                            "cisco_ise.log.log_details.NAS-Port-Type",
                            "cisco_ise.log.nas.port.type",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Tunnel-Type") {
                        event.rename(
                            "cisco_ise.log.log_details.Tunnel-Type",
                            "cisco_ise.log.tunnel.type",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Tunnel-Medium-Type") {
                        event.rename(
                            "cisco_ise.log.log_details.Tunnel-Medium-Type",
                            "cisco_ise.log.tunnel.medium.type",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Tunnel-Private-Group-ID") {
                        event.rename(
                            "cisco_ise.log.log_details.Tunnel-Private-Group-ID",
                            "cisco_ise.log.tunnel.private.group_id",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Airespace-Wlan-Id") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Airespace-Wlan-Id")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Airespace-Wlan-Id".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.airespace.wlan.id", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Airespace-Wlan-Id_to_cisco_ise_log_airespace_wlan_id_86981e05")?;
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
                event.remove("cisco_ise.log.log_details.Airespace-Wlan-Id");
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
                    if event.has_value("cisco_ise.log.log_details.AllowedProtocolMatchedRule") {
                        event.rename(
                            "cisco_ise.log.log_details.AllowedProtocolMatchedRule",
                            "cisco_ise.log.allowed_protocol.matched.rule",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Acct-Delay-Time") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Acct-Delay-Time") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Acct-Delay-Time".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.acct.delay_time", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Acct-Delay-Time_to_cisco_ise_log_acct_delay_time_75a0bf0e")?;
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
                event.remove("cisco_ise.log.log_details.Acct-Delay-Time");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Acct-Input-Octets") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Acct-Input-Octets")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Acct-Input-Octets".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.acct.input.octets", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Acct-Input-Octets_to_cisco_ise_log_acct_input_octets_d217c96b")?;
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
                event.remove("cisco_ise.log.log_details.Acct-Input-Octets");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Acct-Output-Octets") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Acct-Output-Octets")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Acct-Output-Octets".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.acct.output.octets", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Acct-Output-Octets_to_cisco_ise_log_acct_output_octets_436a1c8f")?;
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
                event.remove("cisco_ise.log.log_details.Acct-Output-Octets");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Acct-Input-Packets") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Acct-Input-Packets")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Acct-Input-Packets".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.acct.input.packets", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Acct-Input-Packets_to_cisco_ise_log_acct_input_packets_7d2f5d95")?;
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
                event.remove("cisco_ise.log.log_details.Acct-Input-Packets");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Acct-Output-Packets") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.Acct-Output-Packets")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Acct-Output-Packets".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.acct.output.packets", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Acct-Output-Packets_to_cisco_ise_log_acct_output_packets_006bd949")?;
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
                event.remove("cisco_ise.log.log_details.Acct-Output-Packets");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Acct-Terminate-Cause") {
                        event.rename(
                            "cisco_ise.log.log_details.Acct-Terminate-Cause",
                            "cisco_ise.log.acct.terminate_cause",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.undefined-52") {
                        event.rename(
                            "cisco_ise.log.log_details.undefined-52",
                            "cisco_ise.log.undefined_52",
                        )?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "pipeline_radius_accounting"
            }

            let _cond =
                { event.get_str("cisco_ise.log.category.name") == Some("CISE_Failed_Attempts") };
            if _cond {
                // Begin nested pipeline: "pipeline_failed_attempts"
                event.set("event.kind", json!("event"))?;
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
                let _cond = {
                    event.has_value("cisco_ise.log.message.description")
                        && event.get_str("cisco_ise.log.message.description") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                            // Grok pattern: ^%{DATA:event.action}:
                            if !cached_grok!("^%{DATA:event.action}:")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set("event.outcome", json!("failure"))?;
                    Ok(())
                })();
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "5400", "5401", "5405", "5411", "5412", "5418", "5423", "5435", "5440",
                            "5448",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5440")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("session"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "5400", "5401", "5405", "5411", "5412", "5418", "5423", "5435", "5440",
                            "5448",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("info"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5405", "5411", "5418", "5435"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("end"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5440")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("start"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5402", "5403", "5407"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5402", "5403", "5407"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("network"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5402", "5403", "5407"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("denied"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5408", "5409", "5410"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5408", "5409", "5410"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("network"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5408", "5409", "5410"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("info"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5449"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5449"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("end"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5406", "5441", "5442", "5443"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("network"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5406", "5441", "5442", "5443"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("denied"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5419", "5422"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("network"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5419", "5422"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("denied"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "5413", "5414", "5417", "5420", "5421", "5434", "5436", "5437", "5438",
                            "5439",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("network"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "5413", "5414", "5417", "5420", "5421", "5434", "5436", "5437", "5438",
                            "5439",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("info"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5416")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("network"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5416")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("end"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5450")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("network"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5450")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("connection"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5415")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("iam"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5415")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("user"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5415")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("change"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5451")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5451")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("end"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5452")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5452")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("info"))?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                        let mut kv_gap = false;
                        for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Response") {
                        if let Some(input) = event.get_string("cisco_ise.log.log_details.Response")
                        {
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
                    }
                    Ok(())
                })();
                event.remove("cisco_ise.log.log_details.Response");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("_tmp.response") {
                        let mut kv_gap = false;
                        for pair in kv_str.split("; ") {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "_tmp.response".into(),
                                    split: "=".into(),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("cisco_ise.log.response.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("cisco_ise.log.log_details.acme-av-pair")
                        && event
                            .get("cisco_ise.log.log_details.acme-av-pair")
                            .is_some_and(|v| v.is_array())
                };
                if _cond {
                    if event.has_value("cisco_ise.log.log_details.acme-av-pair") {
                        {
                            // A foreach walks a LIST or an OBJECT: over an object Elastic
                            // binds `_ingest._key` per entry, which is what a target of
                            // `<field>.{{{_ingest._key}}}` reads.
                            let subject =
                                event.get("cisco_ise.log.log_details.acme-av-pair").cloned();
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // on_failure: 1 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if let Some(kv_str) = event.get_string("_ingest._value") {
                                            let mut kv_gap = false;
                                            for pair in kv_str.split(", ") {
                                                if pair.is_empty() {
                                                    kv_gap = true;
                                                    continue;
                                                }
                                                let Some((key, value)) =
                                                    pair.split_once("=").filter(|_| !kv_gap)
                                                else {
                                                    return Err(TransformError::KvValueSplit {
                                                        field: "_ingest._value".into(),
                                                        split: "=".into(),
                                                    });
                                                };
                                                {
                                                    if !key.is_empty() {
                                                        kv_put(
                                                            event,
                                                            &format!(
                                                                "cisco_ise.log.acme-av-pair.{}",
                                                                key
                                                            ),
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
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
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
                                    "cisco_ise.log.log_details.acme-av-pair",
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
                let _cond = {
                    event.has_value("cisco_ise.log.log_details.acme-av-pair")
                        && !(event
                            .get("cisco_ise.log.log_details.acme-av-pair")
                            .is_some_and(|v| v.is_array()))
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(kv_str) =
                            event.get_string("cisco_ise.log.log_details.acme-av-pair")
                        {
                            let mut kv_gap = false;
                            for pair in kv_str.split(", ") {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                                else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "cisco_ise.log.log_details.acme-av-pair".into(),
                                        split: "=".into(),
                                    });
                                };
                                {
                                    if !key.is_empty() {
                                        kv_put(
                                            event,
                                            &format!("cisco_ise.log.acme-av-pair.{}", key),
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
                        event.set("_ingest.on_failure_processor_tag", "kv_cisco_ise_log_log_details_acme-av-pair_to_cisco_ise_log_acme-av-pair_6cc5cb52")?;
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
                event.remove("cisco_ise.log.log_details.acme-av-pair");
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
                    if event.has_value("cisco_ise.log.log_details.ADDomain") {
                        event.rename(
                            "cisco_ise.log.log_details.ADDomain",
                            "cisco_ise.log.ad.domain.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AllowedProtocolMatchedRule") {
                        event.rename(
                            "cisco_ise.log.log_details.AllowedProtocolMatchedRule",
                            "cisco_ise.log.allowed_protocol.matched.rule",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AuthenticationIdentityStore") {
                        event.rename(
                            "cisco_ise.log.log_details.AuthenticationIdentityStore",
                            "cisco_ise.log.authentication.identity_store",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AuthenticationMethod") {
                        event.rename(
                            "cisco_ise.log.log_details.AuthenticationMethod",
                            "cisco_ise.log.authentication.method",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Called-Station-ID") {
                        event.rename(
                            "cisco_ise.log.log_details.Called-Station-ID",
                            "cisco_ise.log.called_station.id",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Calling-Station-ID") {
                        event.rename(
                            "cisco_ise.log.log_details.Calling-Station-ID",
                            "cisco_ise.log.calling_station.id",
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
                    if event.has_value("cisco_ise.log.log_details.DetailedInfo") {
                        event.rename(
                            "cisco_ise.log.log_details.DetailedInfo",
                            "cisco_ise.log.detailed_info",
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
                    if event.has_value("cisco_ise.log.log_details.EAP-Key-Name") {
                        event.rename(
                            "cisco_ise.log.log_details.EAP-Key-Name",
                            "cisco_ise.log.eap_key.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.EapAuthentication") {
                        event.rename(
                            "cisco_ise.log.log_details.EapAuthentication",
                            "cisco_ise.log.eap.authentication",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.EapChainingResult") {
                        event.rename(
                            "cisco_ise.log.log_details.EapChainingResult",
                            "cisco_ise.log.eap.chaining_result",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.EapTunnel") {
                        event.rename(
                            "cisco_ise.log.log_details.EapTunnel",
                            "cisco_ise.log.eap.tunnel",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.EndPointMACAddress") {
                        event.rename(
                            "cisco_ise.log.log_details.EndPointMACAddress",
                            "cisco_ise.log.endpoint.mac.address",
                        )?;
                    }
                    Ok(())
                })();
                if event.has_value("cisco_ise.log.endpoint.mac.address") {
                    gsub_field(
                        event,
                        "cisco_ise.log.endpoint.mac.address",
                        "cisco_ise.log.endpoint.mac.address",
                        cached_regex!("[-:.]"),
                        "-",
                    )?;
                }
                if event.has_value("cisco_ise.log.endpoint.mac.address") {
                    map_strings(
                        event,
                        "cisco_ise.log.endpoint.mac.address",
                        "cisco_ise.log.endpoint.mac.address",
                        str::to_uppercase,
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("cisco_ise.log.endpoint.mac.address")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("client.mac", v)?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.FailureReason") {
                        event.rename(
                            "cisco_ise.log.log_details.FailureReason",
                            "cisco_ise.log.failure.reason",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Framed-IP-Address") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Framed-IP-Address")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Framed-IP-Address".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.framed.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Framed-IP-Address_to_cisco_ise_log_framed_ip_a7ec88de")?;
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
                event.remove("cisco_ise.log.log_details.Framed-IP-Address");
                let _cond = { event.has_value("cisco_ise.log.framed.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("cisco_ise.log.framed.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Framed-MTU") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Framed-MTU") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Framed-MTU".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.framed.mtu", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Framed-MTU_to_cisco_ise_log_framed_mtu_58cef504")?;
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
                event.remove("cisco_ise.log.log_details.Framed-MTU");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.GroupsOrAttributesProcessFailure")
                    {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.GroupsOrAttributesProcessFailure")
                        {
                            let converted =
                                convert_value(val, "boolean").map_err(|message| {
                                    TransformError::ParseError {
                path: "cisco_ise.log.log_details.GroupsOrAttributesProcessFailure".into(),
                message,
                }
                                })?;
                            event.set("cisco_ise.log.groups.process_failure", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_GroupsOrAttributesProcessFailure_to_cisco_ise_log_groups_process_failure_91f199c0")?;
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
                event.remove("cisco_ise.log.log_details.GroupsOrAttributesProcessFailure");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IdentitySelectionMatchedRule") {
                        event.rename(
                            "cisco_ise.log.log_details.IdentitySelectionMatchedRule",
                            "cisco_ise.log.identity.selection.matched.rule",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.ISEPolicySetName") {
                        event.rename(
                            "cisco_ise.log.log_details.ISEPolicySetName",
                            "cisco_ise.log.ise.policy.set_name",
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
                    if event.has_value("cisco_ise.log.log_details.NAS-IP-Address") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.NAS-IP-Address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.NAS-IP-Address".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.nas.ip", converted)?;
                        }
                    }
                    Ok(())
                })();
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    event.remove("cisco_ise.log.log_details.NAS-IP-Address");
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "remove")?;
                    if event.remove("NAS-IP-Address").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "NAS-IP-Address".into(),
                        });
                    }
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
                let _cond = { event.has_value("cisco_ise.log.nas.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("cisco_ise.log.nas.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NAS-Port") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.NAS-Port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.NAS-Port".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.nas.port.number", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_NAS-Port_to_cisco_ise_log_nas_port_number_8385ddaf")?;
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
                event.remove("cisco_ise.log.log_details.NAS-Port");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NAS-Port-Id") {
                        event.rename(
                            "cisco_ise.log.log_details.NAS-Port-Id",
                            "cisco_ise.log.nas.port.id",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NAS-Port-Type") {
                        event.rename(
                            "cisco_ise.log.log_details.NAS-Port-Type",
                            "cisco_ise.log.nas.port.type",
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
                    if event.has_value("cisco_ise.log.log_details.RadiusPacketType") {
                        event.rename(
                            "cisco_ise.log.log_details.RadiusPacketType",
                            "cisco_ise.log.radius_packet.type",
                        )?;
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
                    if event
                        .has_value("cisco_ise.log.log_details.SelectedAuthenticationIdentityStores")
                    {
                        event.rename(
                            "cisco_ise.log.log_details.SelectedAuthenticationIdentityStores",
                            "cisco_ise.log.selected.authentication.identity_stores",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Service-Type") {
                        event.rename(
                            "cisco_ise.log.log_details.Service-Type",
                            "cisco_ise.log.service.type",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.State") {
                        event.rename("cisco_ise.log.log_details.State", "cisco_ise.log.state")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.UseCase") {
                        event
                            .rename("cisco_ise.log.log_details.UseCase", "cisco_ise.log.usecase")?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.DestinationIPAddress") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.DestinationIPAddress")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.DestinationIPAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationIPAddress_to_destination_ip_a431dedf")?;
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
                event.remove("cisco_ise.log.log_details.DestinationIPAddress");
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.DestinationPort") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.DestinationPort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.DestinationPort".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationPort_to_destination_port_ad144c54")?;
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
                event.remove("cisco_ise.log.log_details.DestinationPort");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Device IP Address") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Device IP Address")
                        {
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Device Port") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Device Port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Device Port".into(),
                                    message,
                                }
                            })?;
                            event.set("client.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cisco_ise_log_log_details_Device_Port_to_client_port_cf795c9b",
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
                event.remove("cisco_ise.log.log_details.Device Port");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Acct-Session-Id") {
                        event.rename(
                            "cisco_ise.log.log_details.Acct-Session-Id",
                            "cisco_ise.log.acct.session.id",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Acct-Status-Type") {
                        event.rename(
                            "cisco_ise.log.log_details.Acct-Status-Type",
                            "cisco_ise.log.acct.status.type",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.DTLSSupport") {
                        event.rename(
                            "cisco_ise.log.log_details.DTLSSupport",
                            "cisco_ise.log.dtls_support",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IpAddress") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.IpAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.IpAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cisco_ise_log_log_details_IpAddress_to_source_ip_e43c3b18",
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
                event.remove("cisco_ise.log.log_details.IpAddress");
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("source.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IPSEC") {
                        event.rename("cisco_ise.log.log_details.IPSEC", "cisco_ise.log.ipsec")?;
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
                    if event.has_value("cisco_ise.log.log_details.NetworkDeviceProfileId") {
                        event.rename(
                            "cisco_ise.log.log_details.NetworkDeviceProfileId",
                            "cisco_ise.log.network.device.profile_id",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NetworkDeviceProfileName") {
                        event.rename(
                            "cisco_ise.log.log_details.NetworkDeviceProfileName",
                            "cisco_ise.log.network.device.profile_name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.OpenSSLErrorMessage") {
                        event.rename(
                            "cisco_ise.log.log_details.OpenSSLErrorMessage",
                            "cisco_ise.log.openssl.error.message",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.OpenSSLErrorStack") {
                        event.rename(
                            "cisco_ise.log.log_details.OpenSSLErrorStack",
                            "cisco_ise.log.openssl.error.stack",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.PortalName") {
                        event.rename(
                            "cisco_ise.log.log_details.PortalName",
                            "cisco_ise.log.portal.name",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.ResponseTime") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.ResponseTime") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.ResponseTime".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.response.time", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_ResponseTime_to_cisco_ise_log_response_time_1c336c93")?;
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
                event.remove("cisco_ise.log.log_details.ResponseTime");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Session-Timeout") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Session-Timeout") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Session-Timeout".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.session.timeout", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Session-Timeout_to_cisco_ise_log_session_timeout_9f86aa72")?;
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
                event.remove("cisco_ise.log.log_details.Session-Timeout");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Step") {
                        event.rename("cisco_ise.log.log_details.Step", "cisco_ise.log.step")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.StepLatency") {
                        event.rename(
                            "cisco_ise.log.log_details.StepLatency",
                            "cisco_ise.log.step_latency",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.TLSCipher") {
                        event.rename(
                            "cisco_ise.log.log_details.TLSCipher",
                            "cisco_ise.log.tls.cipher",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.TLSVersion") {
                        event.rename(
                            "cisco_ise.log.log_details.TLSVersion",
                            "cisco_ise.log.tls.version",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.TotalFailedAttempts") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.TotalFailedAttempts")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.TotalFailedAttempts".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.total.failed_attempts", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_TotalFailedAttempts_to_cisco_ise_log_total_failed_attempts_0829b850")?;
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
                event.remove("cisco_ise.log.log_details.TotalFailedAttempts");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.TotalFailedTime") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.TotalFailedTime") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.TotalFailedTime".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.total.failed_time", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_TotalFailedTime_to_cisco_ise_log_total_failed_time_daedbbd4")?;
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
                event.remove("cisco_ise.log.log_details.TotalFailedTime");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.UserType") {
                        event.rename(
                            "cisco_ise.log.log_details.UserType",
                            "cisco_ise.log.user.type",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Protocol") {
                        event.rename("cisco_ise.log.log_details.Protocol", "network.protocol")?;
                    }
                    Ok(())
                })();
                if event.has_value("network.protocol") {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                }
                let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        event.append_unique(
                            "user.name",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.UserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "append")?;
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
                let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.UserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "append")?;
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
                event.remove("cisco_ise.log.log_details.UserName");
                let _cond = { event.has_value("cisco_ise.log.log_details.User-Name") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        event.append_unique(
                            "user.name",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.User-Name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "append")?;
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
                let _cond = { event.has_value("cisco_ise.log.log_details.User-Name") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.User-Name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "append")?;
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
                event.remove("cisco_ise.log.log_details.User-Name");
                // End nested pipeline: "pipeline_failed_attempts"
            }

            let _cond = {
                event.get_str("cisco_ise.log.category.name") == Some("CISE_Passed_Authentications")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_passed_authentications"
                event.set("event.kind", json!("event"))?;
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
                let _cond = {
                    event.has_value("cisco_ise.log.message.description")
                        && event.get_str("cisco_ise.log.message.description") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                            // Grok pattern: ^%{DATA:event.action}:
                            if !cached_grok!("^%{DATA:event.action}:")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "5200", "5201", "5206", "5231", "5233", "5235", "5237", "5238", "5239",
                            "5240",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "5200", "5201", "5206", "5231", "5233", "5235", "5237", "5238", "5239",
                            "5240",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("success"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5404", "5434", "5413"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("failure"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "5200", "5201", "5206", "5231", "5233", "5235", "5237", "5238", "5239",
                            "5240",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("info"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5202", "5203"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5202", "5203"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("network"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5202", "5203"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("allowed"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5202", "5203"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("success"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5204")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("iam"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5204")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("user"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5204")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("change"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5204")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("success"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5205")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("network"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5205")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("info"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5205")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("success"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5232", "5234", "5236"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("network"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5232", "5234", "5236"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("allowed"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["5232", "5234", "5236"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("success"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5241")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("network"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5241")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("connection"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("5241")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("success"))?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                        let mut kv_gap = false;
                        for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                event.remove("cisco_ise.log.log_details.Response");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("_tmp.response") {
                        let mut kv_gap = false;
                        for pair in kv_str.split("; ") {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "_tmp.response".into(),
                                    split: "=".into(),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("cisco_ise.log.response.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                if event.has_value("cisco_ise.log.log_details.AcsSessionID") {
                    event.rename(
                        "cisco_ise.log.log_details.AcsSessionID",
                        "cisco_ise.log.acs.session.id",
                    )?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Airespace-Wlan-Id") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Airespace-Wlan-Id")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Airespace-Wlan-Id".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.airespace.wlan.id", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Airespace-Wlan-Id_to_cisco_ise_log_airespace_wlan_id_86981e05")?;
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
                event.remove("cisco_ise.log.log_details.Airespace-Wlan-Id");
                if event.has_value("cisco_ise.log.log_details.allowEasyWiredSession") {
                    event.rename(
                        "cisco_ise.log.log_details.allowEasyWiredSession",
                        "cisco_ise.log.allow.easy.wired.session",
                    )?;
                }
                if event.has_value("cisco_ise.log.log_details.AuthorizationPolicyMatchedRule") {
                    event.rename(
                        "cisco_ise.log.log_details.AuthorizationPolicyMatchedRule",
                        "cisco_ise.log.auth.policy.matched.rule",
                    )?;
                }
                if event.has_value("cisco_ise.log.log_details.AuthenticationIdentityStore") {
                    event.rename(
                        "cisco_ise.log.log_details.AuthenticationIdentityStore",
                        "cisco_ise.log.authentication.identity_store",
                    )?;
                }
                if event.has_value("cisco_ise.log.log_details.AuthenticationMethod") {
                    event.rename(
                        "cisco_ise.log.log_details.AuthenticationMethod",
                        "cisco_ise.log.authentication.method",
                    )?;
                }
                if event.has_value("cisco_ise.log.log_details.AuthenticationStatus") {
                    event.rename(
                        "cisco_ise.log.log_details.AuthenticationStatus",
                        "cisco_ise.log.authentication.status",
                    )?;
                }
                if event.has_value("cisco_ise.log.log_details.Calling-Station-ID") {
                    event.rename(
                        "cisco_ise.log.log_details.Calling-Station-ID",
                        "cisco_ise.log.calling_station.id",
                    )?;
                }
                // Painless script
                // Source: if (ctx.cisco_ise.log.log_details.get(\"cisco-av-pair\") == null) {\n  return;\n}\nif (ctx.cisco_ise.log.log_details.get(\"cisco-av-pair\") instanceof String) {\n  ctx.cisco_ise.log.log_details[\"cisco-av-pair\"] = [ctx.cisco_ise.log.log_details.get(\"cisco-av-pair\")];\n}\n\ndef attributes = [:];\nctx.cisco_ise.log.log_details.get(\"cisco-av-pair\")?.forEach((v) -> {\n  def firstEq = v.indexOf('=');\n  if (firstEq <= 0) {\n    return true;\n  }\n  def topKey = v.substring(0, firstEq).trim();\n  def rest = v.substring(firstEq + 1);\n\n  // Only mdm-tlv uses nested subkeys (device-platform=win, ...). Other pairs\n  // (e.g. FQSubjectName=...cn=...,ou=...) may contain '=' in the value and must stay scalar.\n  if (!\"mdm-tlv\".equals(topKey)) {\n    attributes[topKey] = rest.trim();\n    return true;\n  }\n\n  def inEscape = false;\n  def start = 0;\n  def key = \"\";\n  if (!attributes.containsKey(\"mdm-tlv\")) {\n    attributes[\"mdm-tlv\"] = [:];\n  }\n  def m = attributes[\"mdm-tlv\"];\n\n  for (def i = 0, n = rest.length(); i < n; ++i) {\n    def c = rest.charAt(i);\n    if (inEscape) {\n      inEscape = false;\n      continue;\n    }\n    if (c == (char)'\\\\') {\n      inEscape = true;\n      continue;\n    }\n\n    if (c == (char)'=') {\n      key = rest.substring(start, i).trim();\n      def nextSplit = rest.indexOf(\"=\", i + 1);\n      if (nextSplit != -1 && rest.charAt(nextSplit - 1) != (char)'\\\\') {\n        if (!m.containsKey(key)) {\n          m[key] = [:];\n        }\n        m = m[key];\n      }\n      start = i + 1;\n    }\n    if (i == n - 1) {\n      m[key] = rest.substring(start, n).trim();\n    }\n  }\n\n  return true;\n});\n\nif (attributes.size() > 0) {\n  ctx.cisco_ise.log[\"cisco_av_pair\"] = attributes;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.cisco_ise.log.log_details.get(\"cisco-av-pair\") == null) {\n  return;\n}\nif (ctx.cisco_ise.log.log_details.get(\"cisco-av-pair\") instanceof String) {\n  ctx.cisco_ise.log.log_details[\"cisco-av-pair\"] = [ctx.cisco_ise.log.log_details.get(\"cisco-av-pair\")];\n}\n\ndef attributes = [:];\nctx.cisco_ise.log.log_details.get(\"cisco-av-pair\")?.forEach((v) -> {\n  def firstEq = v.indexOf('=');\n  if (firstEq <= 0) {\n    return true;\n  }\n  def topKey = v.substring(0, firstEq).trim();\n  def rest = v.substring(firstEq + 1);\n\n  // Only mdm-tlv uses nested subkeys (device-platform=win, ...). Other pairs\n  // (e.g. FQSubjectName=...cn=...,ou=...) may contain '=' in the value and must stay scalar.\n  if (!\"mdm-tlv\".equals(topKey)) {\n    attributes[topKey] = rest.trim();\n    return true;\n  }\n\n  def inEscape = false;\n  def start = 0;\n  def key = \"\";\n  if (!attributes.containsKey(\"mdm-tlv\")) {\n    attributes[\"mdm-tlv\"] = [:];\n  }\n  def m = attributes[\"mdm-tlv\"];\n\n  for (def i = 0, n = rest.length(); i < n; ++i) {\n    def c = rest.charAt(i);\n    if (inEscape) {\n      inEscape = false;\n      continue;\n    }\n    if (c == (char)'\\\\') {\n      inEscape = true;\n      continue;\n    }\n\n    if (c == (char)'=') {\n      key = rest.substring(start, i).trim();\n      def nextSplit = rest.indexOf(\"=\", i + 1);\n      if (nextSplit != -1 && rest.charAt(nextSplit - 1) != (char)'\\\\') {\n        if (!m.containsKey(key)) {\n          m[key] = [:];\n        }\n        m = m[key];\n      }\n      start = i + 1;\n    }\n    if (i == n - 1) {\n      m[key] = rest.substring(start, n).trim();\n    }\n  }\n\n  return true;\n});\n\nif (attributes.size() > 0) {\n  ctx.cisco_ise.log[\"cisco_av_pair\"] = attributes;\n}\n"#
                    ),
                )?;
                event.remove("cisco_ise.log.log_details.cisco-av-pair");
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.cisco_av_pair.coa-push") {
                        if let Some(val) = event.get("cisco_ise.log.cisco_av_pair.coa-push") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.cisco_av_pair.coa-push".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.cisco_av_pair.coa-push", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_cisco_av_pair_coa-push_to_cisco_ise_log_cisco_av_pair_coa-push_17c73a21")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    if event
                        .remove("cisco_ise.log.cisco_av_pair.coa-push")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "cisco_ise.log.cisco_av_pair.coa-push".into(),
                        });
                    }
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.cisco-av-pair.cts-environment-version") {
                        if let Some(val) =
                            event.get("cisco_ise.log.cisco-av-pair.cts-environment-version")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.cisco-av-pair.cts-environment-version"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "cisco_ise.log.cisco-av-pair.cts-environment-version",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_cisco-av-pair_cts-environment-version_to_cisco_ise_log_cisco-av-pair_cts-environment-version_0f389da9")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("cisco_ise.log.cisco_av_pair.cts-environment-version");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.ClientLatency") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.ClientLatency") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.ClientLatency".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.client.latency", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_ClientLatency_to_cisco_ise_log_client_latency_108a319b")?;
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
                event.remove("cisco_ise.log.log_details.ClientLatency");
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
                    if event.has_value("cisco_ise.log.log_details.DTLSSupport") {
                        event.rename(
                            "cisco_ise.log.log_details.DTLSSupport",
                            "cisco_ise.log.dtls_support",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.EndPointMACAddress") {
                        event.rename(
                            "cisco_ise.log.log_details.EndPointMACAddress",
                            "cisco_ise.log.endpoint.mac.address",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event
                        .get("cisco_ise.log.endpoint.mac.address")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("client.mac", v)?;
                    }
                    Ok(())
                })();
                if event.has_value("cisco_ise.log.endpoint.mac.address") {
                    gsub_field(
                        event,
                        "cisco_ise.log.endpoint.mac.address",
                        "cisco_ise.log.endpoint.mac.address",
                        cached_regex!("[-:.]"),
                        "-",
                    )?;
                }
                if event.has_value("cisco_ise.log.endpoint.mac.address") {
                    map_strings(
                        event,
                        "cisco_ise.log.endpoint.mac.address",
                        "cisco_ise.log.endpoint.mac.address",
                        str::to_uppercase,
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.GuestUserName") {
                        event.rename(
                            "cisco_ise.log.log_details.GuestUserName",
                            "cisco_ise.log.guest.user.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IdentityGroup") {
                        event.rename(
                            "cisco_ise.log.log_details.IdentityGroup",
                            "cisco_ise.log.identity.group",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IdentityPolicyMatchedRule") {
                        event.rename(
                            "cisco_ise.log.log_details.IdentityPolicyMatchedRule",
                            "cisco_ise.log.identity.policy.matched.rule",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IpAddress") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.IpAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.IpAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("source.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cisco_ise_log_log_details_IpAddress_to_source_ip_e43c3b18",
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
                event.remove("cisco_ise.log.log_details.IpAddress");
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("source.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IPSEC") {
                        event.rename("cisco_ise.log.log_details.IPSEC", "cisco_ise.log.ipsec")?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IsThirdPartyDeviceFlow") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.IsThirdPartyDeviceFlow")
                        {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.IsThirdPartyDeviceFlow".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.is_third_party_device_flow", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_IsThirdPartyDeviceFlow_to_cisco_ise_log_is_third_party_device_flow_7674ad73")?;
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
                event.remove("cisco_ise.log.log_details.IsThirdPartyDeviceFlow");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.ISEPolicySetName") {
                        event.rename(
                            "cisco_ise.log.log_details.ISEPolicySetName",
                            "cisco_ise.log.ise.policy.set_name",
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
                    if event.has_value("cisco_ise.log.log_details.MisconfiguredClientFixReason") {
                        event.rename(
                            "cisco_ise.log.log_details.MisconfiguredClientFixReason",
                            "cisco_ise.log.misconfigured.client.fix.reason",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NAS-IP-Address") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.NAS-IP-Address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.NAS-IP-Address".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.nas.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_NAS-IP-Address_to_cisco_ise_log_nas_ip_ae27d25e")?;
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
                event.remove("cisco_ise.log.log_details.NAS-IP-Address");
                let _cond = { event.has_value("cisco_ise.log.nas.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("cisco_ise.log.nas.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NAS-Port") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.NAS-Port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.NAS-Port".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.nas.port.number", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_NAS-Port_to_cisco_ise_log_nas_port_number_8385ddaf")?;
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
                event.remove("cisco_ise.log.log_details.NAS-Port");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NAS-Port-Id") {
                        event.rename(
                            "cisco_ise.log.log_details.NAS-Port-Id",
                            "cisco_ise.log.nas.port.id",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NAS-Port-Type") {
                        event.rename(
                            "cisco_ise.log.log_details.NAS-Port-Type",
                            "cisco_ise.log.nas.port.type",
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
                    if event.has_value("cisco_ise.log.log_details.NetworkDeviceProfileName") {
                        event.rename(
                            "cisco_ise.log.log_details.NetworkDeviceProfileName",
                            "cisco_ise.log.network.device.profile_name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NetworkDeviceProfileId") {
                        event.rename(
                            "cisco_ise.log.log_details.NetworkDeviceProfileId",
                            "cisco_ise.log.network.device.profile_id",
                        )?;
                    }
                    Ok(())
                })();
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
                    if event.has_value("cisco_ise.log.log_details.PortalName") {
                        event.rename(
                            "cisco_ise.log.log_details.PortalName",
                            "cisco_ise.log.portal.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.PostureAssessmentStatus") {
                        event.rename(
                            "cisco_ise.log.log_details.PostureAssessmentStatus",
                            "cisco_ise.log.posture.assessment.status",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.PsnHostName") {
                        event.rename(
                            "cisco_ise.log.log_details.PsnHostName",
                            "cisco_ise.log.psn.hostname",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.RadiusFlowType") {
                        event.rename(
                            "cisco_ise.log.log_details.RadiusFlowType",
                            "cisco_ise.log.radius.flow.type",
                        )?;
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.ResponseTime") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.ResponseTime") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.ResponseTime".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.response.time", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_ResponseTime_to_cisco_ise_log_response_time_1c336c93")?;
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
                event.remove("cisco_ise.log.log_details.ResponseTime");
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
                    if event
                        .has_value("cisco_ise.log.log_details.SelectedAuthenticationIdentityStores")
                    {
                        event.rename(
                            "cisco_ise.log.log_details.SelectedAuthenticationIdentityStores",
                            "cisco_ise.log.selected.authentication.identity_stores",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.SelectedAuthorizationProfiles") {
                        event.rename(
                            "cisco_ise.log.log_details.SelectedAuthorizationProfiles",
                            "cisco_ise.log.selected.authorization.profiles",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IdentitySelectionMatchedRule") {
                        event.rename(
                            "cisco_ise.log.log_details.IdentitySelectionMatchedRule",
                            "cisco_ise.log.identity.selection.matched.rule",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Service-Type") {
                        event.rename(
                            "cisco_ise.log.log_details.Service-Type",
                            "cisco_ise.log.service.type",
                        )?;
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
                    if event.has_value("cisco_ise.log.log_details.StepData") {
                        event.rename(
                            "cisco_ise.log.log_details.StepData",
                            "cisco_ise.log.step_data",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.TotalAuthenLatency") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.TotalAuthenLatency")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.TotalAuthenLatency".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.total.authen.latency", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_TotalAuthenLatency_to_cisco_ise_log_total_authen_latency_bebbe609")?;
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
                event.remove("cisco_ise.log.log_details.TotalAuthenLatency");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.UseCase") {
                        event
                            .rename("cisco_ise.log.log_details.UseCase", "cisco_ise.log.usecase")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.UserType") {
                        event.rename(
                            "cisco_ise.log.log_details.UserType",
                            "cisco_ise.log.user.type",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.name",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.UserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.UserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.remove("cisco_ise.log.log_details.UserName");
                let _cond = {
                    event.has_value("cisco_ise.log.log_details")
                        && event.has_value("cisco_ise.log.log_details.User-Name")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.name",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.User-Name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.log_details")
                        && event.has_value("cisco_ise.log.log_details.User-Name")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.User-Name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.remove("cisco_ise.log.log_details.User-Name");
                let _cond = { event.has_value("cisco_ise.log.log_details.OriginalUserName") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.name",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.OriginalUserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_ise.log.log_details.OriginalUserName") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.OriginalUserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.remove("cisco_ise.log.log_details.OriginalUserName");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.DestinationIPAddress") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.DestinationIPAddress")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.DestinationIPAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationIPAddress_to_destination_ip_a431dedf")?;
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
                event.remove("cisco_ise.log.log_details.DestinationIPAddress");
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.DestinationPort") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.DestinationPort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.DestinationPort".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationPort_to_destination_port_ad144c54")?;
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
                event.remove("cisco_ise.log.log_details.DestinationPort");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Device IP Address") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Device IP Address")
                        {
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
                    if event.has_value("cisco_ise.log.log_details.Protocol") {
                        event.rename("cisco_ise.log.log_details.Protocol", "network.protocol")?;
                    }
                    Ok(())
                })();
                if event.has_value("network.protocol") {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                }
                // End nested pipeline: "pipeline_passed_authentications"
            }

            let _cond =
                { event.get_str("cisco_ise.log.category.name") == Some("CISE_RADIUS_Diagnostics") };
            if _cond {
                // Begin nested pipeline: "pipeline_radius_diagnostics"
                event.set("event.kind", json!("event"))?;
                let _cond = { event.get_i64("cisco_ise.log.segment.number") == Some(0) };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{TIMESTAMP_ISO8601:_tmp.timestamp} %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:cisco_ise.log.message.code} %{DATA:log.syslog.severity.name} RADIUS: An Access-Request MUST contain at least a NAS-IP-Address, NAS-IPv6-Address, or a NAS-Identifier; Continue processing, %{GREEDYDATA:cisco_ise.log.log_details_raw},
                        // Grok pattern: ^%{TIMESTAMP_ISO8601:_tmp.timestamp} %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:cisco_ise.log.message.code} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{TIMESTAMP_ISO8601:_tmp.timestamp} %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:cisco_ise.log.message.code} %{DATA:log.syslog.severity.name} RADIUS: An Access-Request MUST contain at least a NAS-IP-Address, NAS-IPv6-Address, or a NAS-Identifier; Continue processing, %{GREEDYDATA:cisco_ise.log.log_details_raw},"
                                ),
                                cached_grok!(
                                    "^%{TIMESTAMP_ISO8601:_tmp.timestamp} %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:cisco_ise.log.message.code} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},"
                                ),
                            ],
                            &input,
                            event,
                        )? {
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
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("11015") };
                if _cond {
                    event.set("cisco_ise.log.message.description", json!("RADIUS: An Access-Request MUST contain at least a NAS-IP-Address NAS-IPv6-Address, or a NAS-Identifier; Continue processing"))?;
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
                let _cond = {
                    event.has_value("cisco_ise.log.message.description")
                        && event.get_str("cisco_ise.log.message.description") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                            // Grok pattern: ^%{DATA:event.action}:
                            if !cached_grok!("^%{DATA:event.action}:")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["11001", "11002", "11004", "11005", "11006", "11015"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("iam"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "11036", "11038", "11507", "11823", "12300", "12301", "12302", "12305",
                            "12307", "12309", "12318", "12500", "12814", "12817",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["11027", "12500", "12800", "12805", "12814", "12817"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("network"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("11117")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("session"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["11017", "11018"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("configuration"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "11001", "11002", "11004", "11005", "11006", "11015", "11017", "11018",
                            "11027", "11036", "11038", "11117", "11507", "11823", "12300", "12301",
                            "12302", "12305", "12307", "12309", "12318", "12500", "12800", "12805",
                            "12814", "12817",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("info"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["11823", "12307", "12309", "12817"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("end"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("11117")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("start"))?;
                        Ok(())
                    })();
                }
                gsub_field(
                    event,
                    "cisco_ise.log.log_details_raw",
                    "cisco_ise.log.log_details_raw",
                    cached_regex!("\\\\,"),
                    "",
                )?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                        let mut kv_gap = false;
                        for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                event.remove("cisco_ise.log.log_details.Response");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("_tmp.response") {
                        let mut kv_gap = false;
                        for pair in kv_str.split("; ") {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "_tmp.response".into(),
                                    split: "=".into(),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("cisco_ise.log.response.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Acct-Session-Id") {
                        event.rename(
                            "cisco_ise.log.log_details.Acct-Session-Id",
                            "cisco_ise.log.acct.session.id",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Acct-Status-Type") {
                        event.rename(
                            "cisco_ise.log.log_details.Acct-Status-Type",
                            "cisco_ise.log.acct.status.type",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Airespace-Wlan-Id") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Airespace-Wlan-Id")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Airespace-Wlan-Id".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.airespace.wlan.id", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Airespace-Wlan-Id_to_cisco_ise_log_airespace_wlan_id_86981e05")?;
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
                event.remove("cisco_ise.log.log_details.Airespace-Wlan-Id");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Calling-Station-ID") {
                        event.rename(
                            "cisco_ise.log.log_details.Calling-Station-ID",
                            "cisco_ise.log.calling_station.id",
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
                    if event.has_value("cisco_ise.log.log_details.DetailedInfo") {
                        event.rename(
                            "cisco_ise.log.log_details.DetailedInfo",
                            "cisco_ise.log.detailed_info",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.EapAuthentication") {
                        event.rename(
                            "cisco_ise.log.log_details.EapAuthentication",
                            "cisco_ise.log.eap.authentication",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.EapTunnel") {
                        event.rename(
                            "cisco_ise.log.log_details.EapTunnel",
                            "cisco_ise.log.eap.tunnel",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NAS-IP-Address") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.NAS-IP-Address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.NAS-IP-Address".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.nas.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_NAS-IP-Address_to_cisco_ise_log_nas_ip_ae27d25e")?;
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
                event.remove("cisco_ise.log.log_details.NAS-IP-Address");
                let _cond = { event.has_value("cisco_ise.log.nas.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("cisco_ise.log.nas.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NAS-Port") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.NAS-Port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.NAS-Port".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.nas.port.number", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_NAS-Port_to_cisco_ise_log_nas_port_number_8385ddaf")?;
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
                event.remove("cisco_ise.log.log_details.NAS-Port");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.OpenSSLErrorMessage") {
                        event.rename(
                            "cisco_ise.log.log_details.OpenSSLErrorMessage",
                            "cisco_ise.log.openssl.error.message",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.OpenSSLErrorStack") {
                        event.rename(
                            "cisco_ise.log.log_details.OpenSSLErrorStack",
                            "cisco_ise.log.openssl.error.stack",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.NAS-Port-Type") {
                        event.rename(
                            "cisco_ise.log.log_details.NAS-Port-Type",
                            "cisco_ise.log.nas.port.type",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.RadiusIdentifier") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.RadiusIdentifier") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.RadiusIdentifier".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.radius_identifier", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_RadiusIdentifier_to_cisco_ise_log_radius_identifier_2c79dc58")?;
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
                event.remove("cisco_ise.log.log_details.RadiusIdentifier");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.RadiusPacketType") {
                        event.rename(
                            "cisco_ise.log.log_details.RadiusPacketType",
                            "cisco_ise.log.radius.packet.type",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Session-Timeout") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Session-Timeout") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Session-Timeout".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.session.timeout", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_Session-Timeout_to_cisco_ise_log_session_timeout_9f86aa72")?;
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
                event.remove("cisco_ise.log.log_details.Session-Timeout");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.State") {
                        event.rename("cisco_ise.log.log_details.State", "cisco_ise.log.state")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.UseCase") {
                        event
                            .rename("cisco_ise.log.log_details.UseCase", "cisco_ise.log.usecase")?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.DestinationIPAddress") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.DestinationIPAddress")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.DestinationIPAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationIPAddress_to_destination_ip_a431dedf")?;
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
                event.remove("cisco_ise.log.log_details.DestinationIPAddress");
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.DestinationPort") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.DestinationPort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.DestinationPort".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationPort_to_destination_port_ad144c54")?;
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
                event.remove("cisco_ise.log.log_details.DestinationPort");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Device IP Address") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Device IP Address")
                        {
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Device Port") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.Device Port") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.Device Port".into(),
                                    message,
                                }
                            })?;
                            event.set("client.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cisco_ise_log_log_details_Device_Port_to_client_port_cf795c9b",
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
                event.remove("cisco_ise.log.log_details.Device Port");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Service-Type") {
                        event.rename("cisco_ise.log.log_details.Service-Type", "service.type")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.User-Name") {
                        event.rename("cisco_ise.log.log_details.User-Name", "user.name")?;
                    }
                    Ok(())
                })();
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
                // End nested pipeline: "pipeline_radius_diagnostics"
            }

            let _cond =
                { event.get_str("cisco_ise.log.category.name") == Some("CISE_AD_Connector") };
            if _cond {
                // Begin nested pipeline: "pipeline_ad_connector"
                event.set("event.kind", json!("event"))?;
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(
                        event,
                        "cisco_ise.log.log_details_raw",
                        "cisco_ise.log.log_details_raw",
                        |s| s.trim().to_string(),
                    )?;
                    Ok(())
                })();
                let _cond = {
                    event.has_value("cisco_ise.log.message.description")
                        && event.get_str("cisco_ise.log.message.description") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                            // Grok pattern: ^%{DATA:event.action}:
                            if !cached_grok!("^%{DATA:event.action}:")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "25012", "25013", "25015", "25016", "25017", "25018", "25033",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["25037", "25041", "25046", "25058"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("configuration"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "25012", "25013", "25015", "25016", "25017", "25018", "25033", "25037",
                            "25041", "25046", "25058",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("info"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["25012", "25018", "51020", "51021"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("end"))?;
                        Ok(())
                    })();
                }
                gsub_field(
                    event,
                    "cisco_ise.log.log_details_raw",
                    "cisco_ise.log.log_details_raw",
                    cached_regex!("\\\\,"),
                    "",
                )?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                        let mut kv_gap = false;
                        for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                    if event.has_value("cisco_ise.log.log_details.AD-Admin") {
                        event.rename(
                            "cisco_ise.log.log_details.AD-Admin",
                            "cisco_ise.log.ad.admin",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AD-Domain") {
                        event.rename(
                            "cisco_ise.log.log_details.AD-Domain",
                            "cisco_ise.log.ad.domain.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AD-Domain-Controller") {
                        event.rename(
                            "cisco_ise.log.log_details.AD-Domain-Controller",
                            "cisco_ise.log.ad.domain.controller",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AD-Error-Details") {
                        event.rename(
                            "cisco_ise.log.log_details.AD-Error-Details",
                            "cisco_ise.log.ad.error.details",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AD-Forest") {
                        event.rename(
                            "cisco_ise.log.log_details.AD-Forest",
                            "cisco_ise.log.ad.forest",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AD-Hostname") {
                        event.rename(
                            "cisco_ise.log.log_details.AD-Hostname",
                            "cisco_ise.log.ad.hostname",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("cisco_ise.log.ad.hostname") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("cisco_ise.log.ad.hostname")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AD-IP-Address") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.AD-IP-Address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.AD-IP-Address".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.ad.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_AD-IP-Address_to_cisco_ise_log_ad_ip_fe2eed06")?;
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
                event.remove("cisco_ise.log.log_details.AD-IP-Address");
                let _cond = { event.has_value("cisco_ise.log.ad.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("cisco_ise.log.ad.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AD-Log-Id") {
                        event.rename(
                            "cisco_ise.log.log_details.AD-Log-Id",
                            "cisco_ise.log.ad.log_id",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AD-Organization-Unit") {
                        event.rename(
                            "cisco_ise.log.log_details.AD-Organization-Unit",
                            "cisco_ise.log.ad.organization_unit",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AD-Site") {
                        event
                            .rename("cisco_ise.log.log_details.AD-Site", "cisco_ise.log.ad.site")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AD-Log") {
                        event.rename("cisco_ise.log.log_details.AD-Log", "cisco_ise.log.ad.log")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AD-Srv-Query") {
                        event.rename(
                            "cisco_ise.log.log_details.AD-Srv-Query",
                            "cisco_ise.log.ad.srv.query",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AD-Srv-Record") {
                        event.rename(
                            "cisco_ise.log.log_details.AD-Srv-Record",
                            "cisco_ise.log.ad.srv.record",
                        )?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "pipeline_ad_connector"
            }

            let _cond = {
                event.get_str("cisco_ise.log.category.name")
                    == Some("CISE_Authentication_Flow_Diagnostics")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_authentication_flow_diagnostics"
                event.set("event.kind", json!("event"))?;
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
                        // Grok pattern: %{GREEDYDATA:cisco_ise.log.log_details_raw},
                        if !cached_grok!("%{GREEDYDATA:cisco_ise.log.log_details_raw},")
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
                        let mut kv_gap = false;
                        for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                event.remove("cisco_ise.log.log_details.Response");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("_tmp.response") {
                        let mut kv_gap = false;
                        for pair in kv_str.split("; ") {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "_tmp.response".into(),
                                    split: "=".into(),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("cisco_ise.log.response.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("cisco_ise.log.message.description")
                        && event.get_str("cisco_ise.log.message.description") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                            // Grok pattern: ^%{DATA:event.action}:
                            if !cached_grok!("^%{DATA:event.action}:")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append("event.category", json!("iam"))?;
                    Ok(())
                })();
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["22040", "22057", "22061", "22060", "22037"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
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
                    if event.has_value("cisco_ise.log.log_details.AuthenticationIdentityStore") {
                        event.rename(
                            "cisco_ise.log.log_details.AuthenticationIdentityStore",
                            "cisco_ise.log.selected.authentication.identity_stores",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AuthenticationMethod") {
                        event.rename(
                            "cisco_ise.log.log_details.AuthenticationMethod",
                            "cisco_ise.log.authentication.method",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Calling-Station-ID") {
                        event.rename(
                            "cisco_ise.log.log_details.Calling-Station-ID",
                            "cisco_ise.log.calling_station.id",
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
                    if event.has_value("cisco_ise.log.log_details.CurrentIDStoreName") {
                        event.rename(
                            "cisco_ise.log.log_details.CurrentIDStoreName",
                            "cisco_ise.log.currentid.store_name",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("cisco_ise.log.log_details.DestinationIPAddress") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cisco_ise.log.log_details.DestinationIPAddress") {
                            if let Some(val) =
                                event.get("cisco_ise.log.log_details.DestinationIPAddress")
                            {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cisco_ise.log.log_details.DestinationIPAddress"
                                            .into(),
                                        message,
                                    }
                                })?;
                                event.set("destination.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationIPAddress_to_destination_ip_bf35ca8e")?;
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
                event.remove("cisco_ise.log.log_details.DestinationIPAddress");
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
                let _cond = { event.has_value("cisco_ise.log.log_details.NAS-IP-Address") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cisco_ise.log.log_details.NAS-IP-Address") {
                            if let Some(val) = event.get("cisco_ise.log.log_details.NAS-IP-Address")
                            {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "cisco_ise.log.log_details.NAS-IP-Address".into(),
                                        message,
                                    }
                                })?;
                                event.set("cisco_ise.log.nas.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_NAS-IP-Address_to_cisco_ise_log_nas_ip_04e2547a")?;
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
                event.remove("cisco_ise.log.log_details.NAS-IP-Address");
                let _cond = { event.has_value("cisco_ise.log.nas.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("cisco_ise.log.nas.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_ise.log.log_details.OriginalUserName") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.name",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.OriginalUserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_ise.log.log_details.OriginalUserName") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.OriginalUserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.remove("cisco_ise.log.log_details.OriginalUserName");
                let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.name",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.UserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_ise.log.log_details.UserName") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.UserName")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.remove("cisco_ise.log.log_details.UserName");
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
                let _cond =
                    { event.has_value("cisco_ise.log.log_details.WorkflowCurrentIDStoreIndex") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cisco_ise.log.log_details.WorkflowCurrentIDStoreIndex")
                        {
                            if let Some(val) =
                                event.get("cisco_ise.log.log_details.WorkflowCurrentIDStoreIndex")
                            {
                                let converted =
                                    convert_value(val, "long").map_err(|message| {
                                        TransformError::ParseError {
                path: "cisco_ise.log.log_details.WorkflowCurrentIDStoreIndex".into(),
                message,
                }
                                    })?;
                                event.set(
                                    "cisco_ise.log.workflow.current_id.store_index",
                                    converted,
                                )?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_WorkflowCurrentIDStoreIndex_to_cisco_ise_log_workflow_current_id_store_index_8a3cdeab")?;
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
                event.remove("cisco_ise.log.log_details.WorkflowCurrentIDStoreIndex");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.WorkflowIfAuthenticationFailed") {
                        event.rename(
                            "cisco_ise.log.log_details.WorkflowIfAuthenticationFailed",
                            "cisco_ise.log.workflow.if.authentication_failed",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.WorkflowIfProcessError") {
                        event.rename(
                            "cisco_ise.log.log_details.WorkflowIfProcessError",
                            "cisco_ise.log.workflow.if.process_error",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.WorkflowIfUserNotFound") {
                        event.rename(
                            "cisco_ise.log.log_details.WorkflowIfUserNotFound",
                            "cisco_ise.log.workflow.if.user_not_found",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.WorkflowSequenceType") {
                        event.rename(
                            "cisco_ise.log.log_details.WorkflowSequenceType",
                            "cisco_ise.log.workflow.sequence.type",
                        )?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "pipeline_authentication_flow_diagnostics"
            }

            let _cond = {
                event.get_str("cisco_ise.log.category.name")
                    == Some("CISE_Administrative_and_Operational_Audit")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_administrative_and_operational_audit"
                event.set("event.kind", json!("event"))?;
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
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("60067") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.log_details_raw") {
                            // Grok pattern: ConfigVersionId=%{DATA:cisco_ise.log.log_details.ConfigVersionId}, OperationMessageText={%{DATA:cisco_ise.log.log_details.OperationMessageText}}
                            if !cached_grok!("ConfigVersionId=%{DATA:cisco_ise.log.log_details.ConfigVersionId}, OperationMessageText={%{DATA:cisco_ise.log.log_details.OperationMessageText}}").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    ["61025", "61026"]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.log_details_raw") {
                            // Grok pattern: ConfigVersionId=%{DATA:cisco_ise.log.log_details.ConfigVersionId}, AdminInterface=%{DATA:cisco_ise.log.log_details.AdminInterface}, AdminIPAddress=%{DATA:cisco_ise.log.log_details.AdminIPAddress}, , OperationMessageText=%{DATA:cisco_ise.log.log_details.OperationMessageText}, AcsInstance=%{GREEDYDATA:cisco_ise.log.log_details.AcsInstance}
                            if !cached_grok!("ConfigVersionId=%{DATA:cisco_ise.log.log_details.ConfigVersionId}, AdminInterface=%{DATA:cisco_ise.log.log_details.AdminInterface}, AdminIPAddress=%{DATA:cisco_ise.log.log_details.AdminIPAddress}, , OperationMessageText=%{DATA:cisco_ise.log.log_details.OperationMessageText}, AcsInstance=%{GREEDYDATA:cisco_ise.log.log_details.AcsInstance}").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "grok")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "grok_cisco_ise_log_log_details_raw_7eca9d29",
                        )?;
                        if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                            let mut kv_gap = false;
                            for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                                else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "cisco_ise.log.log_details_raw".into(),
                                        split: "=".into(),
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
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52001") };
                if _cond {
                    if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                        let mut kv_gap = false;
                        for pair in cached_regex!("(?<!\\\\), ").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                }
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52001") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("cisco_ise.log.log_details.log_detail")
                        {
                            // Grok pattern: ConfigChangeData=%{DATA:cisco_ise.log.log_details.ConfigChangeData}, ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{DATA:cisco_ise.log.log_details.ObjectName}, Component=%{DATA:cisco_ise.log.log_details.Component}, ObjectInternalID=%{GREEDYDATA:cisco_ise.log.log_details.ObjectInternalID}
                            // Grok pattern: ConfigChangeData=%{DATA:cisco_ise.log.log_details.ConfigChangeData}, ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{DATA:cisco_ise.log.log_details.ObjectName}, OperationMessageText=%{GREEDYDATA:cisco_ise.log.log_details.OperationMessageText}
                            // Grok pattern: ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{DATA:cisco_ise.log.log_details.ObjectName}, Component=%{DATA:cisco_ise.log.log_details.Component}, ObjectInternalID=%{GREEDYDATA:cisco_ise.log.log_details.ObjectInternalID}
                            // Grok pattern: ConfigChangeData=%{DATA:cisco_ise.log.log_details.ConfigChangeData}, ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{GREEDYDATA:cisco_ise.log.log_details.ObjectName}
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "ConfigChangeData=%{DATA:cisco_ise.log.log_details.ConfigChangeData}, ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{DATA:cisco_ise.log.log_details.ObjectName}, Component=%{DATA:cisco_ise.log.log_details.Component}, ObjectInternalID=%{GREEDYDATA:cisco_ise.log.log_details.ObjectInternalID}"
                                    ),
                                    cached_grok!(
                                        "ConfigChangeData=%{DATA:cisco_ise.log.log_details.ConfigChangeData}, ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{DATA:cisco_ise.log.log_details.ObjectName}, OperationMessageText=%{GREEDYDATA:cisco_ise.log.log_details.OperationMessageText}"
                                    ),
                                    cached_grok!(
                                        "ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{DATA:cisco_ise.log.log_details.ObjectName}, Component=%{DATA:cisco_ise.log.log_details.Component}, ObjectInternalID=%{GREEDYDATA:cisco_ise.log.log_details.ObjectInternalID}"
                                    ),
                                    cached_grok!(
                                        "ConfigChangeData=%{DATA:cisco_ise.log.log_details.ConfigChangeData}, ObjectType=%{DATA:cisco_ise.log.log_details.ObjectType}, ObjectName=%{GREEDYDATA:cisco_ise.log.log_details.ObjectName}"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52001") };
                if _cond {
                    event.remove("cisco_ise.log.log_details.log_detail");
                }
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52001") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("cisco_ise.log.log_details.ConfigChangeData")
                        {
                            // Grok pattern: ^%{DATA:_tmp.temp}, Log Severity Level = %{DATA:cisco_ise.log.log_details.LogSeverityLevel}\\\\,Local Logging = %{DATA:cisco_ise.log.log_details.LocalLogging}\\\\,Assigned Targets = {%{DATA:cisco_ise.log.log_details.AssignedTargets}}
                            if !cached_grok!("^%{DATA:_tmp.temp}, Log Severity Level = %{DATA:cisco_ise.log.log_details.LogSeverityLevel}\\\\,Local Logging = %{DATA:cisco_ise.log.log_details.LocalLogging}\\\\,Assigned Targets = {%{DATA:cisco_ise.log.log_details.AssignedTargets}}").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52002") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.log_details_raw") {
                            // Grok pattern: ConfigVersionId=%{DATA:cisco_ise.log.log_details.ConfigVersionId}, AdminInterface=%{DATA:cisco_ise.log.log_details.AdminInterface}, AdminIPAddress=%{DATA:cisco_ise.log.log_details.AdminIPAddress}, %{GREEDYDATA:cisco_ise.log.log_details.log_detail}
                            if !cached_grok!("ConfigVersionId=%{DATA:cisco_ise.log.log_details.ConfigVersionId}, AdminInterface=%{DATA:cisco_ise.log.log_details.AdminInterface}, AdminIPAddress=%{DATA:cisco_ise.log.log_details.AdminIPAddress}, %{GREEDYDATA:cisco_ise.log.log_details.log_detail}").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52002") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("cisco_ise.log.log_details.log_detail")
                        {
                            // Grok pattern: AdminSession=%{DATA:cisco_ise.log.log_details.AdminSession}, AdminName=%{DATA:cisco_ise.log.log_details.AdminName}, ConfigChangeData=%{GREEDYDATA:cisco_ise.log.log_details.ConfigChangeData}
                            // Grok pattern: AdminName=%{DATA:cisco_ise.log.log_details.AdminName}, ConfigChangeData=%{GREEDYDATA:cisco_ise.log.log_details.ConfigChangeData}
                            // Grok pattern: AdminName=%{DATA:cisco_ise.log.log_details.AdminName}, %{GREEDYDATA:cisco_ise.log.log_details.log_description}
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "AdminSession=%{DATA:cisco_ise.log.log_details.AdminSession}, AdminName=%{DATA:cisco_ise.log.log_details.AdminName}, ConfigChangeData=%{GREEDYDATA:cisco_ise.log.log_details.ConfigChangeData}"
                                    ),
                                    cached_grok!(
                                        "AdminName=%{DATA:cisco_ise.log.log_details.AdminName}, ConfigChangeData=%{GREEDYDATA:cisco_ise.log.log_details.ConfigChangeData}"
                                    ),
                                    cached_grok!(
                                        "AdminName=%{DATA:cisco_ise.log.log_details.AdminName}, %{GREEDYDATA:cisco_ise.log.log_details.log_description}"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52002") };
                if _cond {
                    event.remove("cisco_ise.log.log_details.log_detail");
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.log_description") {
                        if let Some(kv_str) =
                            event.get_string("cisco_ise.log.log_details.log_description")
                        {
                            let mut kv_gap = false;
                            for pair in kv_str.split(", ") {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                                else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "cisco_ise.log.log_details.log_description".into(),
                                        split: "=".into(),
                                    });
                                };
                                {
                                    let key = key.trim_matches(|c: char| matches!(c, ' '));
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
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "kv")?;
                    event.set("_ingest.on_failure_processor_tag", "kv_cisco_ise_log_log_details_log_description_to_cisco_ise_log_log_details_d92c0c09")?;
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
                event.remove("cisco_ise.log.log_details.log_description");
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52002") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("cisco_ise.log.log_details.ConfigChangeData")
                        {
                            // Grok pattern: ^%{DATA:_tmp.temp}, %{GREEDYDATA:_tmp.ConfigChangeData}$
                            if !cached_grok!(
                                "^%{DATA:_tmp.temp}, %{GREEDYDATA:_tmp.ConfigChangeData}$"
                            )
                            .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52002") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(kv_str) = event.get_string("_tmp.ConfigChangeData") {
                            let mut kv_gap = false;
                            for pair in kv_str.split(", ") {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                                else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "_tmp.ConfigChangeData".into(),
                                        split: "=".into(),
                                    });
                                };
                                {
                                    let key = key.trim_matches(|c: char| matches!(c, ' '));
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
                }
                let _cond = {
                    !(["60067", "61025", "61026", "52001", "52002"]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or("")))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                            let mut kv_gap = false;
                            for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                                else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "cisco_ise.log.log_details_raw".into(),
                                        split: "=".into(),
                                    });
                                };
                                {
                                    let key = key.trim_matches(|c: char| matches!(c, ' '));
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
                }
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("60067") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(kv_str) =
                            event.get_string("cisco_ise.log.log_details.OperationMessageText")
                        {
                            let mut kv_gap = false;
                            for pair in kv_str.split(", ") {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                                else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "cisco_ise.log.log_details.OperationMessageText"
                                            .into(),
                                        split: "=".into(),
                                    });
                                };
                                {
                                    let key = key.trim_matches(|c: char| matches!(c, ' '));
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
                }
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("52001") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(s) =
                            event.get_string("cisco_ise.log.log_details.AssignedTargets")
                        {
                            let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                            if parts.len() > 1 {
                                while parts.last().and_then(Value::as_str) == Some("") {
                                    parts.pop();
                                }
                            }
                            event.set("cisco_ise.log.assigned_targets", Value::Array(parts))?;
                        }
                        Ok(())
                    })();
                }
                event.remove("cisco_ise.log.log_details.AssignedTargets");
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
                let _cond = {
                    event.has_value("cisco_ise.log.message.description")
                        && event.get_str("cisco_ise.log.message.description") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                            // Grok pattern: ^%{DATA:event.action}:
                            if !cached_grok!("^%{DATA:event.action}:")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "51001", "51002", "51020", "51021", "52000", "52001", "52002", "60077",
                            "60078", "60461", "61077", "58005", "60094", "60093", "60134", "60188",
                            "60116", "60080", "60115", "60081", "60084",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("iam"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "51001", "51002", "51020", "51021", "60077", "60078", "61077", "60188",
                            "60116", "60080", "60115", "60081",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["61025", "61026", "60134"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("network"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["60067", "60070", "60456", "58005"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("process"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["52000", "52001", "52002", "60084"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("configuration"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["51001", "51002", "51020", "51021"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("admin"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["52001", "60084"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("change"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["61025", "61026"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("connection"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("52000")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("creation"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("52002")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("deletion"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("61026")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("end"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["60116", "60080", "60115", "60081"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("user"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "51001", "51002", "51020", "51021", "52000", "52001", "52002", "60067",
                            "60070", "60077", "60078", "60456", "60461", "61025", "61026", "61077",
                            "58005", "60094", "60093", "60134", "60188", "60116", "60080", "60115",
                            "60081", "60084",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("info"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["60067", "60456", "61025"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("start"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "51001", "51002", "60078", "60080", "60115", "60116", "61077",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("success"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["51000", "51020", "51021", "60077", "60081", "60188"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("failure"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("cisco_ise.log.message.code") == Some("60084")
                        && cached_regex!(r"(?i)successfully").is_match(
                            event
                                .get_str("cisco_ise.log.log_details.OperationMessageText")
                                .unwrap_or(""),
                        )
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("success"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("cisco_ise.log.message.code") == Some("60084")
                        && cached_regex!(r"(?i)(?:failed|failure|unsuccessful|error)").is_match(
                            event
                                .get_str("cisco_ise.log.log_details.OperationMessageText")
                                .unwrap_or(""),
                        )
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("failure"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("cisco_ise.log.message.code") == Some("60084")
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("unknown"))?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_ise.log.message.code") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        // Painless script
                        // Source: def eventCategory = new ArrayList();\ndef eventType = new ArrayList();\ndef categoryReferenceTable = [\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\",\"52000\",\"52001\",\"52002\",\"60077\",\"60078\",\"60461\",\"61077\",\"58005\",\"60094\",\"60093\",\"60134\",\"60188\",\"60116\",\"60080\",\"60115\",\"60081\",\"60084\"], \"name\": \"iam\"],\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\",\"60077\",\"60078\",\"61077\",\"60188\",\"60116\",\"60080\",\"60115\",\"60081\"], \"name\": \"authentication\"],\n  [\"messageCodeArray\": [\"61025\",\"61026\",\"60134\"], \"name\": \"network\"],\n  [\"messageCodeArray\": [\"60067\",\"60070\",\"60456\",\"58005\"], \"name\": \"process\"],\n  [\"messageCodeArray\": [\"52000\",\"52001\",\"52002\",\"60084\"], \"name\": \"configuration\"]\n];\ndef typeReferenceTable = [\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\"], \"name\": \"admin\"],\n  [\"messageCodeArray\": [\"52001\",\"60084\"], \"name\": \"change\"],\n  [\"messageCodeArray\": [\"61025\", \"61026\"], \"name\": \"connection\"],\n  [\"messageCodeArray\": [\"52000\"], \"name\": \"creation\"],\n  [\"messageCodeArray\": [\"52002\"], \"name\": \"deletion\"],\n  [\"messageCodeArray\": [\"61026\"], \"name\": \"end\"],\n  [\"messageCodeArray\": [\"60116\",\"60080\",\"60115\",\"60081\"], \"name\": \"user\"],\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\",\"52000\",\"52001\",\"52002\",\"60067\",\"60070\",\"60077\",\"60078\",\"60456\",\"60461\",\"61025\",\"61026\",\"61077\",\"58005\",\"60094\",\"60093\",\"60134\",\"60188\",\"60116\",\"60080\",\"60115\",\"60081\",\"60084\"], \"name\": \"info\"],\n  [\"messageCodeArray\": [\"60067\",\"60456\",\"61025\"], \"name\": \"start\"]\n];\n\nfor (entry in categoryReferenceTable) {\n  if (entry.messageCodeArray.contains(ctx.cisco_ise.log.message.code)) {\n    eventCategory.add(entry.name);\n  }\n}\nfor (entry in typeReferenceTable) {\n  if (entry.messageCodeArray.contains(ctx.cisco_ise.log.message.code)) {\n    eventType.add(entry.name);\n  }\n}\n\nctx.event.action = ctx.cisco_ise?.log?.message?.description?.splitOnToken(\":\")[0]?.toLowerCase();\nctx.event.category = eventCategory;\nctx.event.type = eventType;\n
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"def eventCategory = new ArrayList();\ndef eventType = new ArrayList();\ndef categoryReferenceTable = [\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\",\"52000\",\"52001\",\"52002\",\"60077\",\"60078\",\"60461\",\"61077\",\"58005\",\"60094\",\"60093\",\"60134\",\"60188\",\"60116\",\"60080\",\"60115\",\"60081\",\"60084\"], \"name\": \"iam\"],\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\",\"60077\",\"60078\",\"61077\",\"60188\",\"60116\",\"60080\",\"60115\",\"60081\"], \"name\": \"authentication\"],\n  [\"messageCodeArray\": [\"61025\",\"61026\",\"60134\"], \"name\": \"network\"],\n  [\"messageCodeArray\": [\"60067\",\"60070\",\"60456\",\"58005\"], \"name\": \"process\"],\n  [\"messageCodeArray\": [\"52000\",\"52001\",\"52002\",\"60084\"], \"name\": \"configuration\"]\n];\ndef typeReferenceTable = [\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\"], \"name\": \"admin\"],\n  [\"messageCodeArray\": [\"52001\",\"60084\"], \"name\": \"change\"],\n  [\"messageCodeArray\": [\"61025\", \"61026\"], \"name\": \"connection\"],\n  [\"messageCodeArray\": [\"52000\"], \"name\": \"creation\"],\n  [\"messageCodeArray\": [\"52002\"], \"name\": \"deletion\"],\n  [\"messageCodeArray\": [\"61026\"], \"name\": \"end\"],\n  [\"messageCodeArray\": [\"60116\",\"60080\",\"60115\",\"60081\"], \"name\": \"user\"],\n  [\"messageCodeArray\": [\"51001\",\"51002\",\"51020\",\"51021\",\"52000\",\"52001\",\"52002\",\"60067\",\"60070\",\"60077\",\"60078\",\"60456\",\"60461\",\"61025\",\"61026\",\"61077\",\"58005\",\"60094\",\"60093\",\"60134\",\"60188\",\"60116\",\"60080\",\"60115\",\"60081\",\"60084\"], \"name\": \"info\"],\n  [\"messageCodeArray\": [\"60067\",\"60456\",\"61025\"], \"name\": \"start\"]\n];\n\nfor (entry in categoryReferenceTable) {\n  if (entry.messageCodeArray.contains(ctx.cisco_ise.log.message.code)) {\n    eventCategory.add(entry.name);\n  }\n}\nfor (entry in typeReferenceTable) {\n  if (entry.messageCodeArray.contains(ctx.cisco_ise.log.message.code)) {\n    eventType.add(entry.name);\n  }\n}\n\nctx.event.action = ctx.cisco_ise?.log?.message?.description?.splitOnToken(\":\")[0]?.toLowerCase();\nctx.event.category = eventCategory;\nctx.event.type = eventType;\n"#
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AcsInstance") {
                        event.rename(
                            "cisco_ise.log.log_details.AcsInstance",
                            "cisco_ise.log.acs.instance",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AdminInterface") {
                        event.rename(
                            "cisco_ise.log.log_details.AdminInterface",
                            "cisco_ise.log.admin.interface",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AdminIPAddress") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.AdminIPAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.AdminIPAddress".into(),
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
                        "convert_cisco_ise_log_log_details_AdminIPAddress_to_client_ip_732626ad",
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
                event.remove("cisco_ise.log.log_details.AdminIPAddress");
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
                    if event.has_value("cisco_ise.log.log_details.AdminName") {
                        event.rename("cisco_ise.log.log_details.AdminName", "client.user.name")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AdminSession") {
                        event.rename(
                            "cisco_ise.log.log_details.AdminSession",
                            "cisco_ise.log.admin.session",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AuthenticationIdentityStore") {
                        event.rename(
                            "cisco_ise.log.log_details.AuthenticationIdentityStore",
                            "cisco_ise.log.authentication.identity_store",
                        )?;
                    }
                    Ok(())
                })();
                if event.has_value("cisco_ise.log.log_details.ConfigChangeData") {
                    event.rename(
                        "cisco_ise.log.log_details.ConfigChangeData",
                        "cisco_ise.log.config_change.data",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Component") {
                        event.rename(
                            "cisco_ise.log.log_details.Component",
                            "cisco_ise.log.component",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.DestinationPort") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.DestinationPort") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.DestinationPort".into(),
                                    message,
                                }
                            })?;
                            event.set("destination.port", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DestinationPort_to_destination_port_baa6773e")?;
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
                event.remove("cisco_ise.log.log_details.DestinationPort");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.FailureReason") {
                        event.rename(
                            "cisco_ise.log.log_details.FailureReason",
                            "cisco_ise.log.failure.reason",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.FailureFlag") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.FailureFlag") {
                            let converted = convert_value(val, "boolean").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.FailureFlag".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.failure.flag", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_FailureFlag_to_cisco_ise_log_failure_flag_3adca0f9")?;
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
                event.remove("cisco_ise.log.log_details.FailureFlag");
                if event.has_value("cisco_ise.log.log_details.LocalLogging") {
                    event.rename(
                        "cisco_ise.log.log_details.LocalLogging",
                        "cisco_ise.log.local_logging",
                    )?;
                }
                let _cond = { !event.has_value("log.syslog.severity.name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("cisco_ise.log.log_details.LogSeverityLevel") {
                            event.rename(
                                "cisco_ise.log.log_details.LogSeverityLevel",
                                "log.syslog.severity.name",
                            )?;
                        }
                        Ok(())
                    })();
                }
                event.remove("cisco_ise.log.log_details.LogSeverityLevel");
                if event.has_value("cisco_ise.log.log_details.LogErrorMessage") {
                    event.rename(
                        "cisco_ise.log.log_details.LogErrorMessage",
                        "cisco_ise.log.log_error.message",
                    )?;
                }
                if event.has_value("cisco_ise.log.log_details.LoggerName") {
                    event.rename("cisco_ise.log.log_details.LoggerName", "log.logger")?;
                }
                if event.has_value("cisco_ise.log.log_details.MessageCode") {
                    event.rename(
                        "cisco_ise.log.log_details.MessageCode",
                        "cisco_ise.log.message.code",
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.FeedServiceFeed") {
                        event.rename(
                            "cisco_ise.log.log_details.FeedServiceFeed",
                            "cisco_ise.log.feed_service.feed.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.FeedServiceFeedVersion") {
                        event.rename(
                            "cisco_ise.log.log_details.FeedServiceFeedVersion",
                            "cisco_ise.log.feed_service.feed.version",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.FeedServiceHost") {
                        event.rename(
                            "cisco_ise.log.log_details.FeedServiceHost",
                            "cisco_ise.log.feed_service.host",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.FeedServicePort") {
                        event.rename(
                            "cisco_ise.log.log_details.FeedServicePort",
                            "cisco_ise.log.feed_service.port",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("cisco_ise.log.log_details.FeedServiceQueryToTime")
                        && event.get_str("cisco_ise.log.log_details.FeedServiceQueryToTime")
                            != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) =
                            event.get_as_string("cisco_ise.log.log_details.FeedServiceQueryToTime")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => {
                                    event.set("cisco_ise.log.feed_service.query.to_time", parsed)?
                                }
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "cisco_ise.log.log_details.FeedServiceQueryToTime"
                                            .into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_cisco_ise_log_log_details_FeedServiceQueryToTime_to_cisco_ise_log_feed_service_query_to_time_6ec220c3")?;
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
                event.remove("cisco_ise.log.log_details.FeedServiceQueryToTime");
                let _cond = {
                    event.has_value("cisco_ise.log.log_details.FeedServiceQueryFromTime")
                        && event.get_str("cisco_ise.log.log_details.FeedServiceQueryFromTime")
                            != Some("")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event
                            .get_as_string("cisco_ise.log.log_details.FeedServiceQueryFromTime")
                        {
                            match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                Some(parsed) => event
                                    .set("cisco_ise.log.feed_service.query.from_time", parsed)?,
                                None => {
                                    return Err(TransformError::ParseError {
                                        path: "cisco_ise.log.log_details.FeedServiceQueryFromTime"
                                            .into(),
                                        message: format!("unable to parse date [{date_str}]"),
                                    });
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "date_cisco_ise_log_log_details_FeedServiceQueryFromTime_to_cisco_ise_log_feed_service_query_from_time_5f7ec641")?;
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
                event.remove("cisco_ise.log.log_details.FeedServiceQueryFromTime");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IdentityGroup") {
                        event.rename(
                            "cisco_ise.log.log_details.IdentityGroup",
                            "cisco_ise.log.identity.group",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.IpAddress") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.IpAddress") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.IpAddress".into(),
                                    message,
                                }
                            })?;
                            event.set("host.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_cisco_ise_log_log_details_IpAddress_to_host_ip_0f848bff",
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
                event.remove("cisco_ise.log.log_details.IpAddress");
                let _cond = { event.has_value("host.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("host.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.ObjectName") {
                        event.rename(
                            "cisco_ise.log.log_details.ObjectName",
                            "cisco_ise.log.object.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.ObjectInternalID") {
                        event.rename(
                            "cisco_ise.log.log_details.ObjectInternalID",
                            "cisco_ise.log.object.internal.id",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.ObjectType") {
                        event.rename(
                            "cisco_ise.log.log_details.ObjectType",
                            "cisco_ise.log.object.type",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["60080", "60081"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("cisco_ise.log.log_details.OperationMessageText")
                        {
                            // Grok pattern: (?:Accepted|Failed) password for (?:invalid user )?%{DATA:user.name} from %{IP:source.ip}
                            // Grok pattern: Invalid user %{DATA:user.name} from %{IP:source.ip}
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "(?:Accepted|Failed) password for (?:invalid user )?%{DATA:user.name} from %{IP:source.ip}"
                                    ),
                                    cached_grok!(
                                        "Invalid user %{DATA:user.name} from %{IP:source.ip}"
                                    ),
                                ],
                                &input,
                                event,
                            )? {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("source.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.OperationMessageText") {
                        event.rename(
                            "cisco_ise.log.log_details.OperationMessageText",
                            "cisco_ise.log.operation_message.text",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.PortalName") {
                        event.rename(
                            "cisco_ise.log.log_details.PortalName",
                            "cisco_ise.log.portal.name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.PsnHostName") {
                        event.rename(
                            "cisco_ise.log.log_details.PsnHostName",
                            "cisco_ise.log.psn.hostname",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = {
                    event.has_value("cisco_ise.log.psn.hostname")
                        && event.get_str("cisco_ise.log.psn.hostname") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("cisco_ise.log.psn.hostname")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.RequestResponseType") {
                        event.rename(
                            "cisco_ise.log.log_details.RequestResponseType",
                            "cisco_ise.log.request_response.type",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.ResponseTime") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.ResponseTime") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.ResponseTime".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.response.time", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_ResponseTime_to_cisco_ise_log_response_time_bd1b08c1")?;
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
                event.remove("cisco_ise.log.log_details.ResponseTime");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.UserName") {
                        event.rename("cisco_ise.log.log_details.UserName", "user.name")?;
                    }
                    Ok(())
                })();
                let _cond = {
                    !event.has_value("user.name")
                        && event.has_value("client.user.name")
                        && event
                            .get_str("client.user.name")
                            .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                            .is_some_and(|i| i.is_none_or(|i| i == 0))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(v) = event.get("client.user.name").cloned() {
                            event.set("user.name", v)?;
                        }
                        Ok(())
                    })();
                }
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
                // End nested pipeline: "pipeline_administrative_and_operational_audit"
            }

            let _cond =
                { event.get_str("cisco_ise.log.category.name") == Some("CISE_System_Statistics") };
            if _cond {
                // Begin nested pipeline: "pipeline_system_statistics"
                event.set("event.kind", json!("event"))?;
                event.append("event.action", json!("system-stats"))?;
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
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("70001") };
                if _cond {
                    if let Some(input) = event.get_string("cisco_ise.log.log_details_raw") {
                        // Grok pattern: ^ConfigVersionId=%{INT:cisco_ise.log.log_details.ConfigVersionId}, SysStatsAcsProcessHealth= %{GREEDYDATA:_tmp.SysStatsAcsProcessHealth}
                        if !cached_grok!("^ConfigVersionId=%{INT:cisco_ise.log.log_details.ConfigVersionId}, SysStatsAcsProcessHealth= %{GREEDYDATA:_tmp.SysStatsAcsProcessHealth}").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                    }
                }
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("70001") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(kv_str) = event.get_string("_tmp.SysStatsAcsProcessHealth") {
                            let mut kv_gap = false;
                            for pair in kv_str.split("; ") {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                                else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "_tmp.SysStatsAcsProcessHealth".into(),
                                        split: "=".into(),
                                    });
                                };
                                {
                                    if !key.is_empty() {
                                        kv_put(
                                            event,
                                            &format!(
                                                "cisco_ise.log.log_details.SysStatsAcsProcessHealth.{}",
                                                key
                                            ),
                                            value,
                                        )?;
                                    }
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("cisco_ise.log.message.code") != Some("70001") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                            let mut kv_gap = false;
                            for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                                else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "cisco_ise.log.log_details_raw".into(),
                                        split: "=".into(),
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
                }
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("70011") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("cisco_ise.log.log_details.OperationCounters") {
                            if let Some(kv_str) =
                                event.get_string("cisco_ise.log.log_details.OperationCounters")
                            {
                                let mut kv_gap = false;
                                for pair in kv_str.split(", ") {
                                    if pair.is_empty() {
                                        kv_gap = true;
                                        continue;
                                    }
                                    let Some((key, value)) =
                                        pair.split_once("=").filter(|_| !kv_gap)
                                    else {
                                        return Err(TransformError::KvValueSplit {
                                            field: "cisco_ise.log.log_details.OperationCounters"
                                                .into(),
                                            split: "=".into(),
                                        });
                                    };
                                    {
                                        if !key.is_empty() {
                                            kv_put(event, &format!("_tmp.{}", key), value)?;
                                        }
                                    }
                                }
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "kv")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "kv_cisco_ise_log_log_details_OperationCounters_to__tmp_5f844101",
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
                }
                let _cond = { event.get_str("cisco_ise.log.message.code") == Some("70011") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("_tmp.Counter") {
                            if let Some(kv_str) = event.get_string("_tmp.Counter") {
                                let mut kv_gap = false;
                                for pair in kv_str.split(",") {
                                    if pair.is_empty() {
                                        kv_gap = true;
                                        continue;
                                    }
                                    let Some((key, value)) =
                                        pair.split_once(":").filter(|_| !kv_gap)
                                    else {
                                        return Err(TransformError::KvValueSplit {
                                            field: "_tmp.Counter".into(),
                                            split: ":".into(),
                                        });
                                    };
                                    {
                                        if !key.is_empty() {
                                            kv_put(
                                                event,
                                                &format!(
                                                    "cisco_ise.log.log_details.Counters.{}",
                                                    key
                                                ),
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
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "kv__tmp_Counter_to_cisco_ise_log_log_details_Counters_09a758f5",
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
                }
                let _cond = { event.has_value("cisco_ise.log.log_details.Counters") };
                if _cond {
                    event.remove("cisco_ise.log.log_details.OperationCounters");
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
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["70000", "70011"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("host"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("70001")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("process"))?;
                        Ok(())
                    })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.ActiveSessionCount") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.ActiveSessionCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.ActiveSessionCount".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.active_session.count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_ActiveSessionCount_to_cisco_ise_log_active_session_count_14b15f84")?;
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
                event.remove("cisco_ise.log.log_details.ActiveSessionCount");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AverageRadiusRequestLatency") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.AverageRadiusRequestLatency")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.AverageRadiusRequestLatency"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.average.radius.request.latency", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_AverageRadiusRequestLatency_to_cisco_ise_log_average_radius_request_latency_1bb1d9e3")?;
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
                event.remove("cisco_ise.log.log_details.AverageRadiusRequestLatency");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AverageTacacsRequestLatency") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.AverageTacacsRequestLatency")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.AverageTacacsRequestLatency"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.average.tacacs.request.latency", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_AverageTacacsRequestLatency_to_cisco_ise_log_average_tacacs_request_latency_0ce56aa1")?;
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
                event.remove("cisco_ise.log.log_details.AverageTacacsRequestLatency");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.Counters") {
                        event.rename(
                            "cisco_ise.log.log_details.Counters",
                            "cisco_ise.log.operation_counters.counters",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.DeltaRadiusRequestCount") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.DeltaRadiusRequestCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.DeltaRadiusRequestCount"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.delta.radius.request.count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DeltaRadiusRequestCount_to_cisco_ise_log_delta_radius_request_count_bb7c1a15")?;
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
                event.remove("cisco_ise.log.log_details.DeltaRadiusRequestCount");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.DeltaTacacsRequestCount") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.DeltaTacacsRequestCount")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.DeltaTacacsRequestCount"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.delta.tacacs.request.count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_DeltaTacacsRequestCount_to_cisco_ise_log_delta_tacacs_request_count_04b9aa0f")?;
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
                event.remove("cisco_ise.log.log_details.DeltaTacacsRequestCount");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.OperationCounters") {
                        event.rename(
                            "cisco_ise.log.log_details.OperationCounters",
                            "cisco_ise.log.operation_counters.original",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.SysStatsAcsProcessHealth") {
                        event.rename(
                            "cisco_ise.log.log_details.SysStatsAcsProcessHealth",
                            "cisco_ise.log.sysstats.acs.process.health",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.SysStatsCpuCount") {
                        if let Some(val) = event.get("cisco_ise.log.log_details.SysStatsCpuCount") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.SysStatsCpuCount".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.sysstats.cpu.count", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_SysStatsCpuCount_to_cisco_ise_log_sysstats_cpu_count_a7bb6737")?;
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
                event.remove("cisco_ise.log.log_details.SysStatsCpuCount");
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.SysStatsProcessMemoryMB") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.SysStatsProcessMemoryMB")
                        {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.SysStatsProcessMemoryMB"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.sysstats.process_memory_mb", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_SysStatsProcessMemoryMB_to_cisco_ise_log_sysstats_process_memory_mb_1d1675d5")?;
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
                event.remove("cisco_ise.log.log_details.SysStatsProcessMemoryMB");
                if event.has_value("cisco_ise.log.log_details.SysStatsUtilizationCpu") {
                    gsub_field(
                        event,
                        "cisco_ise.log.log_details.SysStatsUtilizationCpu",
                        "cisco_ise.log.log_details.SysStatsUtilizationCpu",
                        cached_regex!("%"),
                        "",
                    )?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.SysStatsUtilizationCpu") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.SysStatsUtilizationCpu")
                        {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.SysStatsUtilizationCpu".into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.sysstats.utilization.cpu", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_SysStatsUtilizationCpu_to_cisco_ise_log_sysstats_utilization_cpu_35d3d694")?;
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
                event.remove("cisco_ise.log.log_details.SysStatsUtilizationCpu");
                if event.has_value("cisco_ise.log.log_details.SysStatsUtilizationDiskIO") {
                    gsub_field(
                        event,
                        "cisco_ise.log.log_details.SysStatsUtilizationDiskIO",
                        "cisco_ise.log.log_details.SysStatsUtilizationDiskIO",
                        cached_regex!("%"),
                        "",
                    )?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.SysStatsUtilizationDiskIO") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.SysStatsUtilizationDiskIO")
                        {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.SysStatsUtilizationDiskIO"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.sysstats.utilization.disk.io", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_SysStatsUtilizationDiskIO_to_cisco_ise_log_sysstats_utilization_disk_io_e002d68e")?;
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
                event.remove("cisco_ise.log.log_details.SysStatsUtilizationDiskIO");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.SysStatsUtilizationDiskSpace") {
                        event.rename(
                            "cisco_ise.log.log_details.SysStatsUtilizationDiskSpace",
                            "cisco_ise.log.sysstats.utilization.disk.space",
                        )?;
                    }
                    Ok(())
                })();
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.SysStatsUtilizationLoadAvg") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.SysStatsUtilizationLoadAvg")
                        {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.SysStatsUtilizationLoadAvg"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.sysstats.utilization.load_avg", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_SysStatsUtilizationLoadAvg_to_cisco_ise_log_sysstats_utilization_load_avg_37bc713f")?;
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
                event.remove("cisco_ise.log.log_details.SysStatsUtilizationLoadAvg");
                if event.has_value("cisco_ise.log.log_details.SysStatsUtilizationMemory") {
                    gsub_field(
                        event,
                        "cisco_ise.log.log_details.SysStatsUtilizationMemory",
                        "cisco_ise.log.log_details.SysStatsUtilizationMemory",
                        cached_regex!("%"),
                        "",
                    )?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.SysStatsUtilizationMemory") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details.SysStatsUtilizationMemory")
                        {
                            let converted = convert_value(val, "double").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details.SysStatsUtilizationMemory"
                                        .into(),
                                    message,
                                }
                            })?;
                            event.set("cisco_ise.log.sysstats.utilization.memory", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_SysStatsUtilizationMemory_to_cisco_ise_log_sysstats_utilization_memory_16d9bb52")?;
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
                event.remove("cisco_ise.log.log_details.SysStatsUtilizationMemory");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.SysStatsUtilizationNetwork") {
                        event.rename(
                            "cisco_ise.log.log_details.SysStatsUtilizationNetwork",
                            "cisco_ise.log.sysstats.utilization.network",
                        )?;
                    }
                    Ok(())
                })();
                // End nested pipeline: "pipeline_system_statistics"
            }

            let _cond =
                { event.get_str("cisco_ise.log.category.name") == Some("CISE_TACACS_Accounting") };
            if _cond {
                // Begin nested pipeline: "pipeline_tacacs_accounting"
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
                        let mut kv_gap = false;
                        for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                                        event
                                            .set("_ingest._key", Value::String(key.to_string()))?;
                                    }
                                    event.set("_ingest._value", item)?;
                                    // on_failure: 1 handler(s)
                                    if let Err(err) = (|| -> Result<()> {
                                        if let Some(kv_str) = event.get_string("_ingest._value") {
                                            let mut kv_gap = false;
                                            for pair in kv_str.split(", ") {
                                                if pair.is_empty() {
                                                    kv_gap = true;
                                                    continue;
                                                }
                                                let Some((key, value)) =
                                                    pair.split_once("=").filter(|_| !kv_gap)
                                                else {
                                                    return Err(TransformError::KvValueSplit {
                                                        field: "_ingest._value".into(),
                                                        split: "=".into(),
                                                    });
                                                };
                                                {
                                                    if !key.is_empty() {
                                                        kv_put(
                                                            event,
                                                            &format!(
                                                                "cisco_ise.log.avpair.{}",
                                                                key
                                                            ),
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
                                        if event.get_object("_ingest").is_some_and(|m| m.is_empty())
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
                                let mut kv_gap = false;
                                for pair in kv_str.split(", ") {
                                    if pair.is_empty() {
                                        kv_gap = true;
                                        continue;
                                    }
                                    let Some((key, value)) =
                                        pair.split_once("=").filter(|_| !kv_gap)
                                    else {
                                        return Err(TransformError::KvValueSplit {
                                            field: "cisco_ise.log.log_details.cisco-av-pair".into(),
                                            split: "=".into(),
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
                        if let Some(val) = event.get("cisco_ise.log.log_details.Device IP Address")
                        {
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
                        if let Some(date_str) =
                            event.get_as_string("cisco_ise.log.avpair.start_time")
                        {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => {
                                    event.set("cisco_ise.log.avpair.start_time", parsed)?
                                }
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
                        if let Some(date_str) =
                            event.get_as_string("cisco_ise.log.avpair.stop_time")
                        {
                            match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                                Some(parsed) => {
                                    event.set("cisco_ise.log.avpair.stop_time", parsed)?
                                }
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
                        let mut kv_gap = false;
                        for pair in kv_str.split("; ") {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "_tmp.response".into(),
                                    split: "=".into(),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("cisco_ise.log.response.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                // End nested pipeline: "pipeline_tacacs_accounting"
            }

            let _cond = {
                event.get_str("cisco_ise.log.category.name")
                    == Some("CISE_Identity_Stores_Diagnostics")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_identity_stores_diagnostics"
                event.set("event.kind", json!("event"))?;
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
                let _cond = {
                    event.has_value("cisco_ise.log.message.description")
                        && event.get_str("cisco_ise.log.message.description") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                            // Grok pattern: ^%{DATA:event.action}:
                            if !cached_grok!("^%{DATA:event.action}:")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "24209", "24210", "24212", "24216", "24217", "24313", "24322", "24325",
                            "24352", "24366", "24412", "24430", "24631", "24633", "24715",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("iam"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "24313", "24322", "24325", "24352", "24412", "24430", "24633", "24715",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("24217")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("host"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && event.get_str("cisco_ise.log.message.code") == Some("24209")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("malware"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && [
                            "24209", "24210", "24212", "24216", "24217", "24313", "24322", "24325",
                            "24352", "24366", "24412", "24430", "24631", "24633", "24715",
                        ]
                        .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("info"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["24352", "24412", "24633"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("end"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("cisco_ise.log.message.code")
                        && ["24210", "24212", "24216", "24631"]
                            .contains(&event.get_str("cisco_ise.log.message.code").unwrap_or(""))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("user"))?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(
                        event,
                        "cisco_ise.log.log_details_raw",
                        "cisco_ise.log.log_details_raw",
                        |s| s.trim().to_string(),
                    )?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("cisco_ise.log.log_details_raw") {
                        let mut kv_gap = false;
                        for pair in cached_regex!(", (?=[^,=]+=)").split(&kv_str).into_iter() {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "cisco_ise.log.log_details_raw".into(),
                                    split: "=".into(),
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
                event.remove("cisco_ise.log.log_details.Response");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(kv_str) = event.get_string("_tmp.response") {
                        let mut kv_gap = false;
                        for pair in kv_str.split("; ") {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                            else {
                                return Err(TransformError::KvValueSplit {
                                    field: "_tmp.response".into(),
                                    split: "=".into(),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(
                                        event,
                                        &format!("cisco_ise.log.response.{}", key),
                                        value,
                                    )?;
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.UserName") {
                        event.rename("cisco_ise.log.log_details.UserName", "user.name")?;
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
                    if event.has_value("cisco_ise.log.log_details.AuthenticationMethod") {
                        event.rename(
                            "cisco_ise.log.log_details.AuthenticationMethod",
                            "cisco_ise.log.authentication.method",
                        )?;
                    }
                    Ok(())
                })();
                if event.has_value("cisco_ise.log.log_details.CurrentIDStoreName") {
                    event.rename(
                        "cisco_ise.log.log_details.CurrentIDStoreName",
                        "cisco_ise.log.currentid.store_name",
                    )?;
                }
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
                    if event.has_value("cisco_ise.log.log_details.EnableFlag") {
                        event.rename(
                            "cisco_ise.log.log_details.EnableFlag",
                            "cisco_ise.log.enable.flag",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.AD-Log-Id") {
                        event.rename(
                            "cisco_ise.log.log_details.AD-Log-Id",
                            "cisco_ise.log.ad.log_id",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("cisco_ise.log.log_details.Firstname") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.full_name",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.Firstname")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_ise.log.log_details.Firstname") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.Firstname")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.remove("cisco_ise.log.log_details.Firstname");
                let _cond = { event.has_value("cisco_ise.log.log_details.Lastname") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.full_name",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.Lastname")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_ise.log.log_details.Lastname") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("cisco_ise.log.log_details.Lastname")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.remove("cisco_ise.log.log_details.Lastname");
                let _cond = { event.has_value("user.full_name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        let joined = event
                            .get("user.full_name")
                            .and_then(|v| join_values(v, " "));
                        if let Some(joined) = joined {
                            event.set("user.full_name", json!(joined))?;
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("user.full_name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("user.full_name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                event.remove("cisco_ise.log.log_details.Lastname");
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details.OriginalUserName") {
                        event.rename(
                            "cisco_ise.log.log_details.OriginalUserName",
                            "cisco_ise.log.original.user.name",
                        )?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("cisco_ise.log.original.user.name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "user.name",
                            json!(
                                event
                                    .get("cisco_ise.log.original.user.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
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
                    if event.has_value("cisco_ise.log.log_details.Protocol") {
                        event.rename("cisco_ise.log.log_details.Protocol", "network.protocol")?;
                    }
                    Ok(())
                })();
                if event.has_value("network.protocol") {
                    map_strings(
                        event,
                        "network.protocol",
                        "network.protocol",
                        str::to_lowercase,
                    )?;
                }
                // End nested pipeline: "pipeline_identity_stores_diagnostics"
            }

            let _cond = { event.get_str("cisco_ise.log.category.name") == Some("CISE_Alarm") };
            if _cond {
                // Begin nested pipeline: "pipeline_alarm"
                event.set("event.kind", json!("event"))?;
                let _cond = { event.has_value("message") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(kv_str) = event.get_string("message") {
                            let mut kv_gap = false;
                            for pair in kv_str.split("; ") {
                                if pair.is_empty() {
                                    kv_gap = true;
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=").filter(|_| !kv_gap)
                                else {
                                    return Err(TransformError::KvValueSplit {
                                        field: "message".into(),
                                        split: "=".into(),
                                    });
                                };
                                {
                                    let key = key.trim_matches(|c: char| matches!(c, ' '));
                                    let value = value.trim_matches(|c: char| matches!(c, ' '));
                                    if !key.is_empty() {
                                        kv_put(
                                            event,
                                            &format!("cisco_ise.log.log_details_raw.{}", key),
                                            value,
                                        )?;
                                    }
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("cisco_ise.log.log_details_raw") };
                if _cond {
                    // Painless script, resolved to its runners at generation time
                    // Source: def c = [:];\nctx.cisco_ise.log.log_details_raw.forEach((k, v) -> c[k.replace(' ', '_').toLowerCase()] = v);\nctx.cisco_ise.log.log_details_raw = c;
                    rewrite_keys(
                        event,
                        &RewriteKeys::new(
                            "cisco_ise.log.log_details_raw".into(),
                            "cisco_ise.log.log_details_raw".into(),
                            vec![
                                KeyRewriteStep::ReplaceChars(" ".into(), Some('_')),
                                KeyRewriteStep::Lowercase,
                            ],
                        ),
                    );
                }
                let _cond = {
                    event.has_value("cisco_ise.log.log_details_raw.message")
                        && event.get_str("event.action") == Some("Queue Link Error")
                };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(input) =
                            event.get_string("cisco_ise.log.log_details_raw.message")
                        {
                            // Grok pattern: ^(%{DATA} )?From %{DATA:source.address} To %{DATA:destination.address}$
                            if !cached_grok!("^(%{DATA} )?From %{DATA:source.address} To %{DATA:destination.address}$").extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "grok")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "grok_alarm_queue_link_error_host_details",
                        )?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "{}: {} with \"{}\"",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("_ingest.on_failure_message")
                                    .map_or_else(String::new, template_to_string),
                                event
                                    .get("message")
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename("cisco_ise.log.log_details_raw.cause", "cisco_ise.log.cause")?;
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details_raw.server") {
                        event.rename("cisco_ise.log.log_details_raw.server", "server.address")?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details_raw.nad_address") {
                        if let Some(val) = event.get("cisco_ise.log.log_details_raw.nad_address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details_raw.nad_address".into(),
                                    message,
                                }
                            })?;
                            event.set("_tmp.nad_ip", converted)?;
                        }
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("_tmp.nad_ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_tmp.nad_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.rename(
                        "cisco_ise.log.log_details_raw.error_message",
                        "cisco_ise.log.error_message",
                    )?;
                    Ok(())
                })();
                let _cond = {
                    event.has_value("source.address") && event.get_str("source.address") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("source.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("destination.address")
                        && event.get_str("destination.address") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("destination.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = {
                    event.has_value("server.address") && event.get_str("server.address") != Some("")
                };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("server.address")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details_raw.nas_ip_address") {
                        event.rename(
                            "cisco_ise.log.log_details_raw.nas_ip_address",
                            "cisco_ise.log.nas_ip_address",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.nas_ip_address") {
                        if let Some(val) = event.get("cisco_ise.log.nas_ip_address") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.nas_ip_address".into(),
                                    message,
                                }
                            })?;
                            event.set("_tmp.nas_ip", converted)?;
                        }
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("_tmp.nas_ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_tmp.nas_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details_raw.nas_identifier") {
                        event.rename(
                            "cisco_ise.log.log_details_raw.nas_identifier",
                            "cisco_ise.log.nas_identifier",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details_raw.failure_reason") {
                        event.rename(
                            "cisco_ise.log.log_details_raw.failure_reason",
                            "cisco_ise.log.failure_reason",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details_raw.network_device_name") {
                        event.rename(
                            "cisco_ise.log.log_details_raw.network_device_name",
                            "cisco_ise.log.network_device_name",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details_raw.calling_station_id") {
                        event.rename(
                            "cisco_ise.log.log_details_raw.calling_station_id",
                            "cisco_ise.log.calling_station_id",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.calling_station_id") {
                        gsub_field(
                            event,
                            "cisco_ise.log.calling_station_id",
                            "cisco_ise.log.calling_station_id",
                            cached_regex!("[-:.]"),
                            "-",
                        )?;
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.calling_station_id") {
                        map_strings(
                            event,
                            "cisco_ise.log.calling_station_id",
                            "cisco_ise.log.calling_station_id",
                            str::to_uppercase,
                        )?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("cisco_ise.log.calling_station_id") };
                if _cond {
                    event.append_unique(
                        "server.mac",
                        json!(
                            event
                                .get("cisco_ise.log.calling_station_id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("cisco_ise.log.log_details_raw.network_device_ip") {
                        if let Some(val) =
                            event.get("cisco_ise.log.log_details_raw.network_device_ip")
                        {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "cisco_ise.log.log_details_raw.network_device_ip".into(),
                                    message,
                                }
                            })?;
                            event.set("_tmp.dev_ip", converted)?;
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_tmp.dev_ip") {
                        event.rename("_tmp.dev_ip", "cisco_ise.log.network_device_ip")?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("cisco_ise.log.network_device_ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("cisco_ise.log.network_device_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "pipeline_alarm"
            }

            let _cond = {
                event
                    .get_str("cisco_ise.log.category.name")
                    .is_some_and(|s| s.to_uppercase() == "CISE_MONITORING_DATA_PURGE_AUDIT")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_monitoring_data_purge_audit"
                event.set("event.kind", json!("event"))?;
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: (?P<_tmp_timestamp>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[ ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?)) %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},
                    if !cached_grok_mapped!("(?P<_tmp_timestamp>(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[ ]%{HOUR}:?%{MINUTE}(?::?%{SECOND})?)) %{ISO8601_TIMEZONE:event.timezone} %{DATA:event.sequence:long} %{DATA:log.syslog.severity.name} %{DATA:cisco_ise.log.message.description}, %{GREEDYDATA:cisco_ise.log.log_details_raw},", [("_tmp_timestamp", "_tmp.timestamp")]).extract_into(&input, event)? {
                return Err(TransformError::GrokNoMatch { value: input });
                }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(
                        event,
                        "cisco_ise.log.log_details_raw",
                        "cisco_ise.log.log_details_raw",
                        |s| s.trim().to_string(),
                    )?;
                    Ok(())
                })();
                let _cond = {
                    event.has_value("cisco_ise.log.message.description")
                        && event.get_str("cisco_ise.log.message.description") != Some("")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("cisco_ise.log.message.description") {
                            // Grok pattern: ^%{DATA:event.action}:
                            if !cached_grok!("^%{DATA:event.action}:")
                                .extract_into(&input, event)?
                            {
                                return Err(TransformError::GrokNoMatch { value: input });
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("event.action") {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                }
                gsub_field(
                    event,
                    "cisco_ise.log.log_details_raw",
                    "cisco_ise.log.log_details_raw",
                    cached_regex!("\\\\,"),
                    "",
                )?;
                // Painless script
                // Source: def m = ctx.cisco_ise.log[\"log_details\"];\nif (!(m instanceof Map)) {\n  m = new HashMap();\n}\nint pos = ctx.cisco_ise.log.log_details_raw.indexOf(\"=\");\nif (pos == -1) {\n    m[ctx.cisco_ise.log.log_details_raw] = null;\n} else {\n    m[ctx.cisco_ise.log.log_details_raw.substring(0,pos)] = ctx.cisco_ise.log.log_details_raw.substring(pos+1);\n}\nctx.cisco_ise.log[\"log_details\"] = m;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def m = ctx.cisco_ise.log[\"log_details\"];\nif (!(m instanceof Map)) {\n  m = new HashMap();\n}\nint pos = ctx.cisco_ise.log.log_details_raw.indexOf(\"=\");\nif (pos == -1) {\n    m[ctx.cisco_ise.log.log_details_raw] = null;\n} else {\n    m[ctx.cisco_ise.log.log_details_raw.substring(0,pos)] = ctx.cisco_ise.log.log_details_raw.substring(pos+1);\n}\nctx.cisco_ise.log[\"log_details\"] = m;"#
                    ),
                )?;
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
                // End nested pipeline: "pipeline_monitoring_data_purge_audit"
            }

            let _cond = { event.get("host.ip").is_some_and(|v| v.is_string()) };
            if _cond {
                event.set(
                    "host.ip",
                    Value::Array(vec![json!(
                        event
                            .get("host.ip")
                            .map_or_else(String::new, template_to_string)
                    )]),
                )?;
            }

            let _cond = {
                event.has_value("client.user.name")
                    && event
                        .get_str("client.user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("client.user.name", "client.user.email")?;
            }

            let _cond = { !event.has_value("client.user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("client.user.email") {
                        if let Some(input) = event.get_string("client.user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("client.user.name", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix("@") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("client.user.domain", remaining));
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

            let _cond = {
                event.get_str("cisco_ise.log.category.name")
                    == Some("CISE_Administrative_and_Operational_Audit")
                    && !event.has_value("user.name")
                    && event.has_value("client.user.name")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("client.user.name").cloned() {
                        event.set("user.name", v)?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("client.user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("client.user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("client.user.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("client.user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("cisco_ise.log.log_details.ConfigVersionId") {
                    if let Some(val) = event.get("cisco_ise.log.log_details.ConfigVersionId") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cisco_ise.log.log_details.ConfigVersionId".into(),
                                message,
                            }
                        })?;
                        event.set("cisco_ise.log.config_version.id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_cisco_ise_log_log_details_ConfigVersionId_to_cisco_ise_log_config_version_id_ca41f4e8")?;
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

            event.remove("cisco_ise.log.log_details.ConfigVersionId");

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("log.syslog.severity.name") {
                    map_strings(
                        event,
                        "log.syslog.severity.name",
                        "log.syslog.severity.name",
                        str::to_lowercase,
                    )?;
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("log.syslog.severity.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("log.level", v)?;
            }

            event.remove("_tmp");
            event.remove("cisco_ise.log.log_details_raw");

            let _cond = { event.has_value("cisco_ise.log.message.code") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(v) = event.get("cisco_ise.log.message.code").cloned() {
                        event.set("event.code", v)?;
                    }
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

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            let _cond = { !event.has_value("@timestamp") };
            if _cond {
                event.append_unique(
                    "tags",
                    json!("cisco_ise.timestamp_defaulted_to_ingest_time"),
                )?;
            }

            let _cond = { !event.has_value("@timestamp") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.set(
                        "@timestamp",
                        json!(
                            event
                                .get("_ingest.timestamp")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
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
