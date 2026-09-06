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
            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.set("ecs.version", json!("8.11.0"))?;

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: ^<%{NUMBER:log.syslog.priority:long}>(?:%{SYSLOGTIMESTAMP:event.created}|%{TIMESTAMP_ISO8601:event.created})\\s+(?:%{IP:_tmp.ip}\\s+)?%{NOTSPACE:host.domain}\\s+%{IP:_tmp.host.ip}\\s+%{DATA:infoblox_nios.log.service_name}\\[?%{NUMBER:process.pid:long}?\\]?:\\s+%{GREEDYDATA:message}$
                // Grok pattern: ^<%{NUMBER:log.syslog.priority:long}>(?:%{SYSLOGTIMESTAMP:event.created}|%{TIMESTAMP_ISO8601:event.created})\\s+(%{IP:_tmp.host.ip}|%{NOTSPACE:host.domain})\\s+%{DATA:infoblox_nios.log.service_name}\\[?%{NUMBER:process.pid:long}?\\]?:\\s+%{GREEDYDATA:message}$
                // Grok pattern: ^<%{NUMBER:log.syslog.priority:long}>(?:%{SYSLOGTIMESTAMP:event.created}|%{TIMESTAMP_ISO8601:event.created})\\s+%{IP:_tmp.host.ip}\\s+%{GREEDYDATA:message}$
                // Grok pattern: ^%{GREEDYDATA:message}$
                if !extract_first_match(
                    &[
                        cached_grok!(
                            "^<%{NUMBER:log.syslog.priority:long}>(?:%{SYSLOGTIMESTAMP:event.created}|%{TIMESTAMP_ISO8601:event.created})\\s+(?:%{IP:_tmp.ip}\\s+)?%{NOTSPACE:host.domain}\\s+%{IP:_tmp.host.ip}\\s+%{DATA:infoblox_nios.log.service_name}\\[?%{NUMBER:process.pid:long}?\\]?:\\s+%{GREEDYDATA:message}$"
                        ),
                        cached_grok!(
                            "^<%{NUMBER:log.syslog.priority:long}>(?:%{SYSLOGTIMESTAMP:event.created}|%{TIMESTAMP_ISO8601:event.created})\\s+(%{IP:_tmp.host.ip}|%{NOTSPACE:host.domain})\\s+%{DATA:infoblox_nios.log.service_name}\\[?%{NUMBER:process.pid:long}?\\]?:\\s+%{GREEDYDATA:message}$"
                        ),
                        cached_grok!(
                            "^<%{NUMBER:log.syslog.priority:long}>(?:%{SYSLOGTIMESTAMP:event.created}|%{TIMESTAMP_ISO8601:event.created})\\s+%{IP:_tmp.host.ip}\\s+%{GREEDYDATA:message}$"
                        ),
                        cached_grok!("^%{GREEDYDATA:message}$"),
                    ],
                    &input,
                    event,
                )? {
                    return Err(TransformError::GrokNoMatch { value: input });
                }
            }

            let _cond = {
                event.has_value("_conf.tz_offset")
                    && event.get_str("_conf.tz_offset") != Some("local")
            };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_conf.tz_offset") {
                        event.rename("_conf.tz_offset", "event.timezone")?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("event.timezone") && event.has_value("event.created") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("event.created") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "MMM  d HH:mm:ss",
                                "MMM dd HH:mm:ss",
                                "MMM d HH:mm:ss",
                                "dd-MMM-yyyy HH:mm:ss.SSS",
                                "ISO8601",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("event.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "event.created".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_event_created_tz")?;
                    event.remove("event.created");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor '{}' {}in pipeline {} failed with message '{}'",
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
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { !event.has_value("event.timezone") && event.has_value("event.created") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("event.created") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "MMM  d HH:mm:ss",
                                "MMM dd HH:mm:ss",
                                "MMM d HH:mm:ss",
                                "dd-MMM-yyyy HH:mm:ss.SSS",
                                "ISO8601",
                            ],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("event.created", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "event.created".into(),
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
                        "date_event_created_notz",
                    )?;
                    event.remove("event.created");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor '{}' {}in pipeline {} failed with message '{}'",
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
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.get_str("infoblox_nios.log.service_name") == Some("dhcpd")
                    || event.get_str("infoblox_nios.log.service_name") == Some("dhcpdv6")
            };
            if _cond {
                event.set("infoblox_nios.log.type", json!("DHCP"))?;
            }

            let _cond = { event.get_str("infoblox_nios.log.service_name") == Some("named") };
            if _cond {
                event.set("infoblox_nios.log.type", json!("DNS"))?;
            }

            let _cond = { event.get_str("infoblox_nios.log.service_name") == Some("httpd") };
            if _cond {
                event.set("infoblox_nios.log.type", json!("AUDIT"))?;
            }

            let _cond = { event.get_str("infoblox_nios.log.type") == Some("AUDIT") };
            if _cond {
                // Begin nested pipeline: "pipeline_audit"
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("Created"))
                        }
                        serde_json::Value::String(s) => s.contains("Created"),
                        _ => false,
                    }) || event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("Modified"))
                        }
                        serde_json::Value::String(s) => s.contains("Modified"),
                        _ => false,
                    }) || event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("Deleted"))
                        }
                        serde_json::Value::String(s) => s.contains("Deleted"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} %{DATA:infoblox_nios.log.audit.object.name} %{DATA:infoblox_nios.log.audit.object.value}:? %{GREEDYDATA:infoblox_nios.log.audit.message}$
                        // Grok pattern: ^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} %{GREEDYDATA:infoblox_nios.log.audit.message}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.audit.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} %{DATA:infoblox_nios.log.audit.object.name} %{DATA:infoblox_nios.log.audit.object.value}:? %{GREEDYDATA:infoblox_nios.log.audit.message}$"
                                ),
                                cached_grok!(
                                    "^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} %{GREEDYDATA:infoblox_nios.log.audit.message}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.audit.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("Called"))
                        }
                        serde_json::Value::String(s) => s.contains("Called"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} - %{WORD:infoblox_nios.log.audit.object.name}:? %{GREEDYDATA:infoblox_nios.log.audit.message}$
                        // Grok pattern: ^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} - %{GREEDYDATA:infoblox_nios.log.audit.message}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.audit.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} - %{WORD:infoblox_nios.log.audit.object.name}:? %{GREEDYDATA:infoblox_nios.log.audit.message}$"
                                ),
                                cached_grok!(
                                    "^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} - %{GREEDYDATA:infoblox_nios.log.audit.message}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.audit.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = { !event.has_value("event.action") };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} - - %{GREEDYDATA:details}$
                        // Grok pattern: ^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} %{GREEDYDATA:infoblox_nios.log.audit.message}$
                        // Grok pattern: ^%{IPORHOST:server.address}: AD authentication for user %{DATA:user.name} (?P<_tmp_ad_auth_failed>(?:failed))$
                        // Grok pattern: ^%{GREEDYDATA:_tmp.timestamp} %{GREEDYDATA:infoblox_nios.log.audit.message}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.audit.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} - - %{GREEDYDATA:details}$"
                                ),
                                cached_grok!(
                                    "^%{GREEDYDATA:_tmp.timestamp} \\[%{DATA:user.name}\\]: %{DATA:event.action} %{GREEDYDATA:infoblox_nios.log.audit.message}$"
                                ),
                                cached_grok_mapped!(
                                    "^%{IPORHOST:server.address}: AD authentication for user %{DATA:user.name} (?P<_tmp_ad_auth_failed>(?:failed))$",
                                    [("_tmp_ad_auth_failed", "_tmp.ad_auth_failed")]
                                ),
                                cached_grok!(
                                    "^%{GREEDYDATA:_tmp.timestamp} %{GREEDYDATA:infoblox_nios.log.audit.message}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.audit.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = { event.has_value("_tmp.timestamp") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &["dd-MMM-yyyy HH:mm:ss.SSS", "yyyy-MM-dd HH:mm:ss.SSS'Z'"],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("_tmp.timestamp", parsed)?,
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
                        event.set("_ingest.on_failure_processor_tag", "date_tmp_timestamp")?;
                        event.remove("_tmp.timestamp");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor '{}' {}in pipeline {} failed with message '{}'",
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
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                if event.has_value("details") {
                    if let Some(kv_str) = event.get_string("details") {
                        for pair in kv_str.split(" ") {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once("=") else {
                                return Err(TransformError::ParseError {
                                    path: "details".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                if !key.is_empty() {
                                    kv_put(event, &format!("audit.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                let _cond = { event.has_value("event.action") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("login_allowed") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("success"))?;
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("login_allowed") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("start"))?;
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("login_allowed") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("event.action") == Some("login_denied")
                        || event.has_value("_tmp.ad_auth_failed")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.set("event.outcome", json!("failure"))?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("event.action") == Some("login_denied")
                        || event.has_value("_tmp.ad_auth_failed")
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("logout") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.type", json!("end"))?;
                        Ok(())
                    })();
                }
                let _cond = { event.get_str("event.action") == Some("logout") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append("event.category", json!("authentication"))?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("audit") };
                if _cond {
                    // Painless script
                    // Source: if (ctx.infoblox_nios == null) {\n  ctx['infoblox_nios'] = new HashMap();\n}\nif (ctx.infoblox_nios.log == null) {\n  ctx.infoblox_nios['log'] = new HashMap();\n}\nif (ctx.infoblox_nios.log.audit == null) {\n  ctx.infoblox_nios.log['audit'] = new HashMap();\n}\nfor (Map.Entry m : ctx.audit.entrySet()) {\n  def value = m.getValue();\n  if (value instanceof String) {\n    value = value.replace('\\\\040', ' ')\n  }\n  ctx.infoblox_nios.log.audit[m.getKey()] = value;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.infoblox_nios == null) {\n  ctx['infoblox_nios'] = new HashMap();\n}\nif (ctx.infoblox_nios.log == null) {\n  ctx.infoblox_nios['log'] = new HashMap();\n}\nif (ctx.infoblox_nios.log.audit == null) {\n  ctx.infoblox_nios.log['audit'] = new HashMap();\n}\nfor (Map.Entry m : ctx.audit.entrySet()) {\n  def value = m.getValue();\n  if (value instanceof String) {\n    value = value.replace('\\\\040', ' ')\n  }\n  ctx.infoblox_nios.log.audit[m.getKey()] = value;\n}\n"#
                        ),
                    )?;
                }
                let _cond = { event.has_value("infoblox_nios.log.audit.ip") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        // Painless script
                        // Source: String s = ctx.infoblox_nios.log.audit.ip; StringBuilder sb = new StringBuilder(); for (int i = 0; i < s.length();) {\n    if (s.charAt(i) == (char)'\\\\') {\n        sb.append(':');\n        int b = Integer.parseInt(s.substring(i+1,i+4), 8);\n        if (b != (char)':') {\n            sb.append((char)b);\n        }\n        i+=4;\n        continue;\n    }\n    sb.append(s.charAt(i));\n    i++;\n} ctx.infoblox_nios.log.audit.ip = sb.toString();\n
                        // TODO: Transpile Painless to Rust (2.2.3)
                        painless_exec_plan(
                            event,
                            cached_painless!(
                                r#"String s = ctx.infoblox_nios.log.audit.ip; StringBuilder sb = new StringBuilder(); for (int i = 0; i < s.length();) {\n    if (s.charAt(i) == (char)'\\\\') {\n        sb.append(':');\n        int b = Integer.parseInt(s.substring(i+1,i+4), 8);\n        if (b != (char)':') {\n            sb.append((char)b);\n        }\n        i+=4;\n        continue;\n    }\n    sb.append(s.charAt(i));\n    i++;\n} ctx.infoblox_nios.log.audit.ip = sb.toString();\n"#
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "script")?;
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor '{}' {}in pipeline {} failed with message '{}'",
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
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = {
                    event.has_value("infoblox_nios.log.audit.ip")
                        && event.get_str("infoblox_nios.log.audit.ip") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("infoblox_nios.log.audit.ip") {
                            if let Some(val) = event.get("infoblox_nios.log.audit.ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "infoblox_nios.log.audit.ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("infoblox_nios.log.audit.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("infoblox_nios.log.audit.ip");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor '{}' {}in pipeline {} failed with message '{}'",
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
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("infoblox_nios.log.audit.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("infoblox_nios.log.audit.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(val) = event.get("server.adress") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "server.adress".into(),
                                message,
                            }
                        })?;
                        event.set("server.ip", converted)?;
                    }
                    Ok(())
                })();
                let _cond = { event.has_value("server.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("server.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                if event.has_value("user.name") {
                    gsub_field(
                        event,
                        "user.name",
                        "user.name",
                        cached_regex!("\\\\040"),
                        " ",
                    )?;
                }
                event.remove("details");
                event.remove("audit");
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
                // End nested pipeline: "pipeline_audit"
            }

            let _cond = { event.get_str("infoblox_nios.log.type") == Some("DHCP") };
            if _cond {
                // Begin nested pipeline: "pipeline_dhcp"
                event.set("network.protocol", json!("dhcp"))?;
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("DHCPDISCOVER"))
                        }
                        serde_json::Value::String(s) => s.contains("DHCPDISCOVER"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{WORD:event.action} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                        // Grok pattern: ^%{WORD:event.action} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: network %{DATA:infoblox_nios.log.dhcp.network}: %{GREEDYDATA:infoblox_nios.log.dhcp.discover.message}$
                        // Grok pattern: ^%{WORD:event.action} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$
                        // Grok pattern: ^%{WORD:event.action} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                        // Grok pattern: ^%{WORD:event.action} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: network %{DATA:infoblox_nios.log.dhcp.network}: %{GREEDYDATA:infoblox_nios.log.dhcp.discover.message}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("DHCPOFFER"))
                        }
                        serde_json::Value::String(s) => s.contains("DHCPOFFER"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{GREEDYDATA:infoblox_nios.log.dhcp.offered.duration:long}$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{GREEDYDATA:infoblox_nios.log.dhcp.offered.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{GREEDYDATA:infoblox_nios.log.dhcp.offered.duration:long}$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{GREEDYDATA:infoblox_nios.log.dhcp.lease.duration:long}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{GREEDYDATA:infoblox_nios.log.dhcp.offered.duration:long}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{GREEDYDATA:infoblox_nios.log.dhcp.offered.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{GREEDYDATA:infoblox_nios.log.dhcp.offered.duration:long}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{GREEDYDATA:infoblox_nios.log.dhcp.lease.duration:long}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("DHCPREQUEST"))
                        }
                        serde_json::Value::String(s) => s.contains("DHCPREQUEST"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$
                        // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$
                        // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid}: %{GREEDYDATA:infoblox_nios.log.dhcp.request.message}$
                        // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                        // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$
                        // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$
                        // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.request.message}$
                        // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.request.message}$
                        // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$
                        // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                        // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$
                        // Grok pattern: ^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name})$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid}: %{GREEDYDATA:infoblox_nios.log.dhcp.request.message}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{DATA:infoblox_nios.log.dhcp.uid} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.request.message}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.request.message}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} for %{IP:client.ip} \\(%{IP:infoblox_nios.log.dhcp.router.ip}\\) from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} for %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name})$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("DHCPACK"))
                        }
                        serde_json::Value::String(s) => s.contains("DHCPACK"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.lease.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} (?:\\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) )?via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.message}\\)$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.lease.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.lease.message}\\)$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{GREEDYDATA:infoblox_nios.log.dhcp.lease.duration:long}$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{GREEDYDATA:infoblox_nios.log.dhcp.lease.duration:long}$
                        // Grok pattern: ^%{WORD:event.action} to %{IP:client.ip} \\(%{MAC:client.mac}\\) via %{WORD:observer.ingress.interface.name}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.lease.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} (?:\\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) )?via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} offered-duration %{NUMBER:infoblox_nios.log.dhcp.offered.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.message}\\)$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.lease.message}\\) uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{DATA:infoblox_nios.log.dhcp.lease.message}\\)$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} \\(%{GREEDYDATA:infoblox_nios.log.dhcp.lease.message}\\)$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{GREEDYDATA:infoblox_nios.log.dhcp.lease.duration:long}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{NUMBER:infoblox_nios.log.dhcp.lease.duration:long} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) relay (%{IP:infoblox_nios.log.dhcp.relay.interface.ip}|%{WORD:infoblox_nios.log.dhcp.relay.interface.name}) lease-duration %{GREEDYDATA:infoblox_nios.log.dhcp.lease.duration:long}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} to %{IP:client.ip} \\(%{MAC:client.mac}\\) via %{WORD:observer.ingress.interface.name}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("RELEASE"))
                        }
                        serde_json::Value::String(s) => s.contains("RELEASE"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) \\(%{DATA:infoblox_nios.log.dhcp.release.info}\\) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$
                        // Grok pattern: ^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) \\(%{DATA:infoblox_nios.log.dhcp.release.info}\\) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} \\(%{DATA:infoblox_nios.log.dhcp.client_hostname}\\) via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) \\(%{DATA:infoblox_nios.log.dhcp.release.info}\\) TransID %{DATA:infoblox_nios.log.dhcp.trans_id} uid %{GREEDYDATA:infoblox_nios.log.dhcp.uid}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) \\(%{DATA:infoblox_nios.log.dhcp.release.info}\\) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("DHCPEXPIRE"))
                        }
                        serde_json::Value::String(s) => s.contains("DHCPEXPIRE"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{GREEDYDATA:client.mac}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{GREEDYDATA:client.mac}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("DHCPINFORM"))
                        }
                        serde_json::Value::String(s) => s.contains("DHCPINFORM"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{WORD:event.action} from %{IP:client.ip} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.inform.message}$
                        // Grok pattern: ^%{WORD:event.action} from %{IP:client.ip} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} from %{IP:client.ip} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.inform.message}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} from %{IP:client.ip} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("DHCPDECLINE"))
                        }
                        serde_json::Value::String(s) => s.contains("DHCPDECLINE"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.decline.message}$
                        // Grok pattern: ^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}): %{GREEDYDATA:infoblox_nios.log.dhcp.decline.message}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}) TransID %{DATA:infoblox_nios.log.dhcp.trans_id}: %{GREEDYDATA:infoblox_nios.log.dhcp.decline.message}$"
                                ),
                                cached_grok!(
                                    "^%{WORD:event.action} of %{IP:client.ip} from %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name}): %{GREEDYDATA:infoblox_nios.log.dhcp.decline.message}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("DHCPNAK"))
                        }
                        serde_json::Value::String(s) => s.contains("DHCPNAK"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name})$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} on %{IP:client.ip} to %{MAC:client.mac} via (%{IP:infoblox_nios.log.dhcp.interface.ip}|%{WORD:observer.ingress.interface.name})$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("DHCPLEASEQUERY"))
                        }
                        serde_json::Value::String(s) => s.contains("DHCPLEASEQUERY"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{WORD:event.action} from %{IP:client.ip}: %{GREEDYDATA:infoblox_nios.log.dhcp.lease_query.message}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{WORD:event.action} from %{IP:client.ip}: %{GREEDYDATA:infoblox_nios.log.dhcp.lease_query.message}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("REFUSED"))
                        }
                        serde_json::Value::String(s) => s.contains("REFUSED"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^(?P<event_action>(?:(?i:reverse map update))) for %{IP:client.ip} abandoned because of non-retryable failure: %{DATA:event.outcome}$
                        // Grok pattern: ^Unable to (?P<event_action>(?:(?i:add forward map))) from %{DATA:infoblox_nios.log.dhcp.forward_name} to %{IP:infoblox_nios.log.dhcp.ip} by server %{IP:server.ip}#%{NUMBER:server.port:long}: %{DATA:event.outcome}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok_mapped!(
                                    "^(?P<event_action>(?:(?i:reverse map update))) for %{IP:client.ip} abandoned because of non-retryable failure: %{DATA:event.outcome}$",
                                    [("event_action", "event.action")]
                                ),
                                cached_grok_mapped!(
                                    "^Unable to (?P<event_action>(?:(?i:add forward map))) from %{DATA:infoblox_nios.log.dhcp.forward_name} to %{IP:infoblox_nios.log.dhcp.ip} by server %{IP:server.ip}#%{NUMBER:server.port:long}: %{DATA:event.outcome}$",
                                    [("event_action", "event.action")]
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event
                        .get_str("event.outcome")
                        .is_some_and(|s| s.eq_ignore_ascii_case("refused"))
                };
                if _cond {
                    gsub_field(
                        event,
                        "event.action",
                        "event.action",
                        cached_regex!(" "),
                        "_",
                    )?;
                }
                let _cond = {
                    event
                        .get_str("event.outcome")
                        .is_some_and(|s| s.eq_ignore_ascii_case("refused"))
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("Encapsulated Solicit"))
                        }
                        serde_json::Value::String(s) => s.contains("Encapsulated Solicit"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{DATA:event.action} message from %{IP:client.ip} port %{NUMBER:client.port:long} from client DUID %{GREEDYDATA:infoblox_nios.log.dhcp.duid}, transaction ID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{DATA:event.action} message from %{IP:client.ip} port %{NUMBER:client.port:long} from client DUID %{GREEDYDATA:infoblox_nios.log.dhcp.duid}, transaction ID %{GREEDYDATA:infoblox_nios.log.dhcp.trans_id}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("Advertise NA"))
                        }
                        serde_json::Value::String(s) => s.contains("Advertise NA"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{DATA:event.action}: address %{IP:client.ip} to client with duid %{GREEDYDATA:infoblox_nios.log.dhcp.duid} iaid = -%{GREEDYDATA:infoblox_nios.log.dhcp.iaid} valid for %{NUMBER:infoblox_nios.log.dhcp.validation_second:long} seconds$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{DATA:event.action}: address %{IP:client.ip} to client with duid %{GREEDYDATA:infoblox_nios.log.dhcp.duid} iaid = -%{GREEDYDATA:infoblox_nios.log.dhcp.iaid} valid for %{NUMBER:infoblox_nios.log.dhcp.validation_second:long} seconds$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("Relay-forward"))
                        }
                        serde_json::Value::String(s) => s.contains("Relay-forward"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{DATA:event.action} message from %{IP:client.ip} port %{NUMBER:client.port:long}, link address %{IP:infoblox_nios.log.dhcp.link_address}, peer address %{IP:infoblox_nios.log.dhcp.peer_address}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{DATA:event.action} message from %{IP:client.ip} port %{NUMBER:client.port:long}, link address %{IP:infoblox_nios.log.dhcp.link_address}, peer address %{IP:infoblox_nios.log.dhcp.peer_address}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("Encapsulating Advertise")),
                        serde_json::Value::String(s) => s.contains("Encapsulating Advertise"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{DATA:event.action} message to send to %{IP:client.ip} port %{NUMBER:client.port:long}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{DATA:event.action} message to send to %{IP:client.ip} port %{NUMBER:client.port:long}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = {
                    event.get("message").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("Sending Relay-reply"))
                        }
                        serde_json::Value::String(s) => s.contains("Sending Relay-reply"),
                        _ => false,
                    })
                };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{DATA:event.action} message to %{IP:client.ip} port %{NUMBER:client.port:long}$
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^%{DATA:event.action} message to %{IP:client.ip} port %{NUMBER:client.port:long}$"
                                ),
                                cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$"),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = { !event.has_value("event.action") };
                if _cond {
                    if let Some(input) = event.get_string("message") {
                        // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$
                        if !cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dhcp.message}$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                    }
                    Ok(())
                })();
                if event.has_value("client.mac") {
                    gsub_field(
                        event,
                        "client.mac",
                        "client.mac",
                        cached_regex!("[-:.]"),
                        "-",
                    )?;
                }
                if event.has_value("client.mac") {
                    map_strings(event, "client.mac", "client.mac", str::to_uppercase)?;
                }
                let _cond =
                    { event.has_value("client.ip") && event.get_str("client.ip") != Some("") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("client.ip") {
                            if let Some(val) = event.get("client.ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.ip".into(),
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
                        event.set("_ingest.on_failure_processor_tag", "convert_client_ip")?;
                        event.remove("client.ip");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor '{}' {}in pipeline {} failed with message '{}'",
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
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
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
                let _cond = {
                    event.has_value("infoblox_nios.log.dhcp.link_address")
                        && event.get_str("infoblox_nios.log.dhcp.link_address") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("infoblox_nios.log.dhcp.link_address") {
                            if let Some(val) = event.get("infoblox_nios.log.dhcp.link_address") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "infoblox_nios.log.dhcp.link_address".into(),
                                        message,
                                    }
                                })?;
                                event.set("infoblox_nios.log.dhcp.link_address", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_dhcp_link_address",
                        )?;
                        event.remove("infoblox_nios.log.dhcp.link_address");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor '{}' {}in pipeline {} failed with message '{}'",
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
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("infoblox_nios.log.dhcp.link_address") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("infoblox_nios.log.dhcp.link_address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("infoblox_nios.log.dhcp.peer_address")
                        && event.get_str("infoblox_nios.log.dhcp.peer_address") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("infoblox_nios.log.dhcp.peer_address") {
                            if let Some(val) = event.get("infoblox_nios.log.dhcp.peer_address") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "infoblox_nios.log.dhcp.peer_address".into(),
                                        message,
                                    }
                                })?;
                                event.set("infoblox_nios.log.dhcp.peer_address", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_dhcp_peer_address",
                        )?;
                        event.remove("infoblox_nios.log.dhcp.peer_address");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor '{}' {}in pipeline {} failed with message '{}'",
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
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("infoblox_nios.log.dhcp.peer_address") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("infoblox_nios.log.dhcp.peer_address")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("infoblox_nios.log.dhcp.router.ip")
                        && event.get_str("infoblox_nios.log.dhcp.router.ip") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("infoblox_nios.log.dhcp.router.ip") {
                            if let Some(val) = event.get("infoblox_nios.log.dhcp.router.ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "infoblox_nios.log.dhcp.router.ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("infoblox_nios.log.dhcp.router.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_dhcp_router_ip")?;
                        event.remove("infoblox_nios.log.dhcp.router.ip");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor '{}' {}in pipeline {} failed with message '{}'",
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
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("infoblox_nios.log.dhcp.router.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("infoblox_nios.log.dhcp.router.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("infoblox_nios.log.dhcp.interface.ip")
                        && event.get_str("infoblox_nios.log.dhcp.interface.ip") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("infoblox_nios.log.dhcp.interface.ip") {
                            if let Some(val) = event.get("infoblox_nios.log.dhcp.interface.ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "infoblox_nios.log.dhcp.interface.ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("infoblox_nios.log.dhcp.interface.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_dhcp_interface_ip",
                        )?;
                        event.remove("infoblox_nios.log.dhcp.interface.ip");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor '{}' {}in pipeline {} failed with message '{}'",
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
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("infoblox_nios.log.dhcp.interface.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("infoblox_nios.log.dhcp.interface.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("infoblox_nios.log.dhcp.relay.interface.ip")
                        && event.get_str("infoblox_nios.log.dhcp.relay.interface.ip") != Some("")
                };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("infoblox_nios.log.dhcp.relay.interface.ip") {
                            if let Some(val) =
                                event.get("infoblox_nios.log.dhcp.relay.interface.ip")
                            {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "infoblox_nios.log.dhcp.relay.interface.ip".into(),
                                        message,
                                    }
                                })?;
                                event
                                    .set("infoblox_nios.log.dhcp.relay.interface.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_dhcp_relay_interface_ip",
                        )?;
                        event.remove("infoblox_nios.log.dhcp.relay.interface.ip");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor '{}' {}in pipeline {} failed with message '{}'",
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
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("infoblox_nios.log.dhcp.relay.interface.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("infoblox_nios.log.dhcp.relay.interface.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("infoblox_nios.log.dhcp.client_hostname") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("infoblox_nios.log.dhcp.client_hostname")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                // End nested pipeline: "pipeline_dhcp"
            }

            let _cond = { event.get_str("infoblox_nios.log.type") == Some("DNS") };
            if _cond {
                // Begin nested pipeline: "pipeline_dns"
                event.set("network.protocol", json!("dns"))?;
                if let Some(input) = event.get_string("message") {
                    // Grok pattern: ^zone %{DATA:dns.question.name}/%{DATA:dns.question.class}: notify from %{IP:client.ip}#%{NUMBER:client.port:long}:? %{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^transfer of '%{DATA:dns.question.name}/%{DATA:dns.question.class}' from %{IP:client.ip}#%{NUMBER:client.port:long}:? %{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^validating %{DATA:dns.question.name}/%{WORD:dns.question.type}: %{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) updating zone '%{DATA:dns.question.name}/%{DATA:dns.question.class}': %{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA}\\): (?:view %{DATA:infoblox_nios.log.view}: )?query failed %{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA:infoblox_nios.log.dns.before_query}\\): rewriting query name %{DATA} to '%{DATA:infoblox_nios.log.dns.after_query}', type %{DATA:dns.question.type}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA}\\): (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} %{DATA:infoblox_nios.log.dns.header_flags} \\(%{IP:server.ip}\\)$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{DATA:network.transport}: (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} response: %{DATA:dns.response_code} %{DATA:infoblox_nios.log.dns.header_flags}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA}\\): transfer of '%{DATA:dns.question.name}/%{DATA:dns.question.class}': %{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*CEF:0\\|Infoblox\\|NIOS\\|%{GREEDYDATA:infoblox_nios.log.dns.version}\\|RPZ-%{DATA:dns.answers.type}\\|%{DATA:infoblox_nios.log.dns.answers_policy}\\|\\d+\\|app=DNS dst=%{IP:server.ip} src=%{IP:client.ip} spt=%{NUMBER:client.port:long} view=%{DATA:infoblox_nios.log.dns.view_name} qtype=%{WORD:dns.question.type} msg=%{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*%{GREEDYDATA:_tmp.timestamp} (?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{DATA:network.transport}: (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} response: %{DATA:dns.response_code} %{DATA:infoblox_nios.log.dns.header_flags} %{GREEDYDATA:repeat_message}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*%{GREEDYDATA:_tmp.timestamp} (?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{DATA:network.transport}: (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} response: %{DATA:dns.response_code} %{DATA:infoblox_nios.log.dns.header_flags}$
                    // Grok pattern: ^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{GREEDYDATA:infoblox_nios.log.dns.message}$
                    // Grok pattern: ^%{GREEDYDATA:infoblox_nios.log.dns.message}$
                    if !extract_first_match(
                        &[
                            cached_grok!(
                                "^zone %{DATA:dns.question.name}/%{DATA:dns.question.class}: notify from %{IP:client.ip}#%{NUMBER:client.port:long}:? %{GREEDYDATA:infoblox_nios.log.dns.message}$"
                            ),
                            cached_grok!(
                                "^transfer of '%{DATA:dns.question.name}/%{DATA:dns.question.class}' from %{IP:client.ip}#%{NUMBER:client.port:long}:? %{GREEDYDATA:infoblox_nios.log.dns.message}$"
                            ),
                            cached_grok!(
                                "^validating %{DATA:dns.question.name}/%{WORD:dns.question.type}: %{GREEDYDATA:infoblox_nios.log.dns.message}$"
                            ),
                            cached_grok!(
                                "^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) updating zone '%{DATA:dns.question.name}/%{DATA:dns.question.class}': %{GREEDYDATA:infoblox_nios.log.dns.message}$"
                            ),
                            cached_grok!(
                                "^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA}\\): (?:view %{DATA:infoblox_nios.log.view}: )?query failed %{GREEDYDATA:infoblox_nios.log.dns.message}$"
                            ),
                            cached_grok!(
                                "^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA:infoblox_nios.log.dns.before_query}\\): rewriting query name %{DATA} to '%{DATA:infoblox_nios.log.dns.after_query}', type %{DATA:dns.question.type}$"
                            ),
                            cached_grok!(
                                "^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA}\\): (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} %{DATA:infoblox_nios.log.dns.header_flags} \\(%{IP:server.ip}\\)$"
                            ),
                            cached_grok!(
                                "^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{DATA:network.transport}: (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} response: %{DATA:dns.response_code} %{DATA:infoblox_nios.log.dns.header_flags}$"
                            ),
                            cached_grok!(
                                "^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) \\(%{DATA}\\): transfer of '%{DATA:dns.question.name}/%{DATA:dns.question.class}': %{GREEDYDATA:infoblox_nios.log.dns.message}$"
                            ),
                            cached_grok!(
                                "^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*CEF:0\\|Infoblox\\|NIOS\\|%{GREEDYDATA:infoblox_nios.log.dns.version}\\|RPZ-%{DATA:dns.answers.type}\\|%{DATA:infoblox_nios.log.dns.answers_policy}\\|\\d+\\|app=DNS dst=%{IP:server.ip} src=%{IP:client.ip} spt=%{NUMBER:client.port:long} view=%{DATA:infoblox_nios.log.dns.view_name} qtype=%{WORD:dns.question.type} msg=%{GREEDYDATA:infoblox_nios.log.dns.message}$"
                            ),
                            cached_grok!(
                                "^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*%{GREEDYDATA:_tmp.timestamp} (?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{DATA:network.transport}: (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} response: %{DATA:dns.response_code} %{DATA:infoblox_nios.log.dns.header_flags} %{GREEDYDATA:repeat_message}$"
                            ),
                            cached_grok!(
                                "^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*%{GREEDYDATA:_tmp.timestamp} (?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{DATA:network.transport}: (?:view %{DATA:infoblox_nios.log.view}: )?query: %{DATA:dns.question.name} %{DATA:dns.question.class} %{WORD:dns.question.type} response: %{DATA:dns.response_code} %{DATA:infoblox_nios.log.dns.header_flags}$"
                            ),
                            cached_grok!(
                                "^(%{NOTSPACE:infoblox_nios.log.dns.category}:)?\\s*(?:client (?:%{DATA} )?%{IP:client.ip}#%{NUMBER:client.port:long}:?) %{GREEDYDATA:infoblox_nios.log.dns.message}$"
                            ),
                            cached_grok!("^%{GREEDYDATA:infoblox_nios.log.dns.message}$"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                }
                let _cond =
                    { event.has_value("_tmp.timestamp") && event.has_value("event.timezone") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &["dd-MMM-yyyy HH:mm:ss.SSS", "yyyy-MM-dd HH:mm:ss.SSS'Z'"],
                                event.get_str("event.timezone"),
                                None,
                            ) {
                                Some(parsed) => event.set("_tmp.timestamp", parsed)?,
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
                        event.set("_ingest.on_failure_processor_tag", "date_tmp_timestamp_tz")?;
                        event.remove("_tmp.timestamp");
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
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond =
                    { event.has_value("_tmp.timestamp") && !event.has_value("event.timezone") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                            match parse_date_out(
                                &date_str,
                                &["dd-MMM-yyyy HH:mm:ss.SSS", "yyyy-MM-dd HH:mm:ss.SSS'Z'"],
                                None,
                                None,
                            ) {
                                Some(parsed) => event.set("_tmp.timestamp", parsed)?,
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
                            "date_tmp_timestamp_notz",
                        )?;
                        event.remove("_tmp.timestamp");
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
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("repeat_message") };
                if _cond {
                    // Painless script
                    // Source: def splitUnquoted(String input, String sep) {\n  def tokens = [];\n  def startPosition = 0;\n  def isInQuotes = false;\n  char quote = (char)\"\\\"\";\n  for (def currentPosition = 0; currentPosition < input.length(); currentPosition++) {\n      if (input.charAt(currentPosition) == quote) {\n          isInQuotes = !isInQuotes;\n      }\n      else if (input.charAt(currentPosition) == (char)sep && !isInQuotes) {\n          def token = input.substring(startPosition, currentPosition).trim();\n          if (!token.equals(\"\")) {\n            tokens.add(token);\n          }\n          startPosition = currentPosition + 1;\n      }\n  }\n\n  def lastToken = input.substring(startPosition);\n  if (!lastToken.equals(sep) && !lastToken.equals(\"\")) {\n      tokens.add(lastToken.trim());\n  }\n  return tokens;\n}\n\ndef arr = splitUnquoted(ctx.repeat_message, \";\");\nctx.repeat_message = arr;\nMap map = new HashMap();\nmap.put('name', new ArrayList());\nmap.put('ttl', new ArrayList());\nmap.put('class', new ArrayList());\nmap.put('type', new ArrayList());\nmap.put('data', new ArrayList());\n\nfor (def i = 0; i < arr.length; i++) {\n  def response = splitUnquoted(arr[i], \" \");\n  if (response.size() >= 4) {\n    map['name'].add(response[0]);\n    map['ttl'].add(response[1]);\n    map['class'].add(response[2]);\n    map['type'].add(response[3]);\n    map['data'].addAll(response.subList(4, response.length));\n  }\n}\nctx.dns.answers = map;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def splitUnquoted(String input, String sep) {\n  def tokens = [];\n  def startPosition = 0;\n  def isInQuotes = false;\n  char quote = (char)\"\\\"\";\n  for (def currentPosition = 0; currentPosition < input.length(); currentPosition++) {\n      if (input.charAt(currentPosition) == quote) {\n          isInQuotes = !isInQuotes;\n      }\n      else if (input.charAt(currentPosition) == (char)sep && !isInQuotes) {\n          def token = input.substring(startPosition, currentPosition).trim();\n          if (!token.equals(\"\")) {\n            tokens.add(token);\n          }\n          startPosition = currentPosition + 1;\n      }\n  }\n\n  def lastToken = input.substring(startPosition);\n  if (!lastToken.equals(sep) && !lastToken.equals(\"\")) {\n      tokens.add(lastToken.trim());\n  }\n  return tokens;\n}\n\ndef arr = splitUnquoted(ctx.repeat_message, \";\");\nctx.repeat_message = arr;\nMap map = new HashMap();\nmap.put('name', new ArrayList());\nmap.put('ttl', new ArrayList());\nmap.put('class', new ArrayList());\nmap.put('type', new ArrayList());\nmap.put('data', new ArrayList());\n\nfor (def i = 0; i < arr.length; i++) {\n  def response = splitUnquoted(arr[i], \" \");\n  if (response.size() >= 4) {\n    map['name'].add(response[0]);\n    map['ttl'].add(response[1]);\n    map['class'].add(response[2]);\n    map['type'].add(response[3]);\n    map['data'].addAll(response.subList(4, response.length));\n  }\n}\nctx.dns.answers = map;\n"#
                        ),
                    )?;
                }
                let _cond = { event.has_value("infoblox_nios.log.dns.message") };
                if _cond {
                    gsub_field(
                        event,
                        "infoblox_nios.log.dns.message",
                        "infoblox_nios.log.dns.message",
                        cached_regex!("\""),
                        "",
                    )?;
                }
                let _cond = { event.has_value("infoblox_nios.log.dns.message") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("infoblox_nios.log.dns.message") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(rest) = remaining.strip_prefix("rpz ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "infoblox_nios.log.dns.rpz.rule_type",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "infoblox_nios.log.dns.rpz.query_class",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured
                                    .push(("infoblox_nios.log.dns.rpz.action", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" ") else {
                                    break 'dissect false;
                                };
                                captured
                                    .push(("infoblox_nios.log.dns.rpz.domain", &remaining[..pos]));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" via ") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "infoblox_nios.log.dns.rpz.query_class_rewrite",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" via ") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                let Some(pos) = remaining.find(" CAT=") else {
                                    break 'dissect false;
                                };
                                captured.push((
                                    "infoblox_nios.log.dns.rpz.domain_rewrite",
                                    &remaining[..pos],
                                ));
                                remaining = &remaining[pos..];
                                let Some(rest) = remaining.strip_prefix(" CAT=") else {
                                    break 'dissect false;
                                };
                                remaining = rest;
                                captured.push(("infoblox_nios.log.dns.rpz.type", remaining));
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
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("dns.answers.ttl") {
                        if let Some(val) = event.get("dns.answers.ttl") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "dns.answers.ttl".into(),
                                    message,
                                }
                            })?;
                            event.set("dns.answers.ttl", converted)?;
                        }
                    }
                    Ok(())
                })();
                let _cond = { event.get("dns.answers.data").is_some_and(|v| v.is_array()) };
                if _cond {
                    // Painless script
                    // Source: def hash = new ArrayList();\nfor(data in ctx.dns.answers.data){\n  def n = data.length();\n  if(data.charAt(n-1).toString() == '.'){\n    def data_substring = data.substring(0,n-1) + data.substring(n);\n    hash.add(data_substring);\n  }\n  else{\n    hash.add(data);\n  }\n}\nctx.dns.answers.data = hash;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def hash = new ArrayList();\nfor(data in ctx.dns.answers.data){\n  def n = data.length();\n  if(data.charAt(n-1).toString() == '.'){\n    def data_substring = data.substring(0,n-1) + data.substring(n);\n    hash.add(data_substring);\n  }\n  else{\n    hash.add(data);\n  }\n}\nctx.dns.answers.data = hash;\n"#
                        ),
                    )?;
                }
                let _cond = { event.get("dns.answers.name").is_some_and(|v| v.is_array()) };
                if _cond {
                    // Painless script
                    // Source: def hash = new ArrayList();\nfor(name in ctx.dns.answers.name){\n  def n = name.length();\n  if(name.charAt(n-1).toString() == '.'){\n    def name_substring = name.substring(0,n-1) + name.substring(n);\n    hash.add(name_substring);\n  }\n  else{\n    hash.add(name);\n  }\n}\nctx.dns.answers.name = hash;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def hash = new ArrayList();\nfor(name in ctx.dns.answers.name){\n  def n = name.length();\n  if(name.charAt(n-1).toString() == '.'){\n    def name_substring = name.substring(0,n-1) + name.substring(n);\n    hash.add(name_substring);\n  }\n  else{\n    hash.add(name);\n  }\n}\nctx.dns.answers.name = hash;\n"#
                        ),
                    )?;
                }
                let _cond = { event.has_value("dns.answers.data") };
                if _cond {
                    {
                        // A foreach walks a LIST or an OBJECT: over an object Elastic
                        // binds `_ingest._key` per entry, which is what a target of
                        // `<field>.{{{_ingest._key}}}` reads.
                        let subject = event.get("dns.answers.data").cloned();
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
                                    if let Some(input) = event.get_string("_ingest._value") {
                                        // Grok pattern: ^%{IP:related.ip}$
                                        // Grok pattern: ^%{HOSTNAME:related.hosts}$
                                        if !extract_first_match(
                                            &[
                                                cached_grok!("^%{IP:related.ip}$"),
                                                cached_grok!("^%{HOSTNAME:related.hosts}$"),
                                            ],
                                            &input,
                                            event,
                                        )? {
                                            return Err(TransformError::GrokNoMatch {
                                                value: input,
                                            });
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
                                "dns.answers.data",
                                if keyed {
                                    Value::Object(fields)
                                } else {
                                    Value::Array(list)
                                },
                            )?;
                        }
                    }
                }
                let _cond =
                    { event.has_value("client.ip") && event.get_str("client.ip") != Some("") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("client.ip") {
                            if let Some(val) = event.get("client.ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "client.ip".into(),
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
                        event.remove("client.ip");
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
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
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
                let _cond =
                    { event.has_value("server.ip") && event.get_str("server.ip") != Some("") };
                if _cond {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("server.ip") {
                            if let Some(val) = event.get("server.ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "server.ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("server.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("server.ip");
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
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                }
                let _cond = { event.has_value("server.ip") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.ip",
                            json!(
                                event
                                    .get("server.ip")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = { event.has_value("dns.answers.name") };
                if _cond {
                    foreach_array(event, "dns.answers.name", |event| {
                        // ignore_failure: true
                        let _ = (|| -> Result<()> {
                            event.append_unique(
                                "related.hosts",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })();
                        Ok(())
                    })?;
                }
                let _cond = { event.has_value("dns.question.name") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        event.append_unique(
                            "related.hosts",
                            json!(
                                event
                                    .get("dns.question.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })();
                }
                let _cond = {
                    event.has_value("infoblox_nios.log.dns.header_flags")
                        && event.get_str("infoblox_nios.log.dns.header_flags") != Some("")
                };
                if _cond {
                    // Painless script
                    // Source: ArrayList hf = new ArrayList();\nfor (entry in params.entrySet()) {\n  if (ctx.infoblox_nios.log.dns.header_flags.contains(entry.getKey())) {\n    hf.add(entry.getValue());\n  }\n}\nif (ctx.dns?.response_code != null && ctx.dns.response_code != '') {\n  if (ctx.infoblox_nios.log.dns.header_flags.contains('+')) {\n    hf.add('RA')\n  }\n} else {\n  if (ctx.infoblox_nios.log.dns.header_flags.contains('+')) {\n    hf.add('RD')\n  }\n}\nif (hf.length == 0) {\n  return;\n}\nif (ctx.dns == null) {\n  HashMap hm = new HashMap();\n  ctx.put('dns', hm);\n}\nctx.dns.put('header_flags', hf);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"ArrayList hf = new ArrayList();\nfor (entry in params.entrySet()) {\n  if (ctx.infoblox_nios.log.dns.header_flags.contains(entry.getKey())) {\n    hf.add(entry.getValue());\n  }\n}\nif (ctx.dns?.response_code != null && ctx.dns.response_code != '') {\n  if (ctx.infoblox_nios.log.dns.header_flags.contains('+')) {\n    hf.add('RA')\n  }\n} else {\n  if (ctx.infoblox_nios.log.dns.header_flags.contains('+')) {\n    hf.add('RD')\n  }\n}\nif (hf.length == 0) {\n  return;\n}\nif (ctx.dns == null) {\n  HashMap hm = new HashMap();\n  ctx.put('dns', hm);\n}\nctx.dns.put('header_flags', hf);\n"#
                        ),
                        cached_params!("{\"A\":\"AA\",\"t\":\"TC\",\"C\":\"CD\",\"D\":\"DO\"}"),
                    )?;
                }
                let _cond = { event.has_value("dns.question") };
                if _cond {
                    if let Some(domain_str) = event.get_string("dns.question.name") {
                        let domain = domain_str.to_string();
                        event.set("dns.question.domain", json!(domain.clone()))?;
                        // Public suffix list lookup for registered domain extraction
                        if let Some(rd) = registered_domain_lookup(&domain) {
                            if let Some(registered) = rd.registered_domain {
                                event.set("dns.question.registered_domain", json!(registered))?;
                            }
                            event
                                .set("dns.question.top_level_domain", json!(rd.top_level_domain))?;
                            if let Some(sub) = rd.subdomain {
                                event.set("dns.question.subdomain", json!(sub))?;
                            }
                        }
                    }
                }
                event.remove("repeat_message");
                event.remove("dns.question.domain");
                // End nested pipeline: "pipeline_dns"
            }

            let _cond = { event.has_value("event.created") };
            if _cond {
                if let Some(v) = event.get("event.created").cloned() {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = { event.has_value("_tmp.timestamp") };
            if _cond {
                if let Some(v) = event.get("_tmp.timestamp").cloned() {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond =
                { event.has_value("_tmp.host.ip") && event.get_str("_tmp.host.ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_tmp.host.ip") {
                        if let Some(val) = event.get("_tmp.host.ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_tmp.host.ip".into(),
                                    message,
                                }
                            })?;
                            event.set("_tmp.host.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.remove("_tmp.host.ip");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor '{}' {}in pipeline {} failed with message '{}'",
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
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("_tmp.host.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_tmp.host.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_tmp.ip") && event.get_str("_tmp.ip") != Some("") };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_tmp.ip") {
                        if let Some(val) = event.get("_tmp.ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_tmp.ip".into(),
                                    message,
                                }
                            })?;
                            event.set("_tmp.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.remove("_tmp.ip");
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor '{}' {}in pipeline {} failed with message '{}'",
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
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("_tmp.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_tmp.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("host.domain") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("host.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_tmp.host.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append(
                        "host.ip",
                        json!(
                            event
                                .get("_tmp.host.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("_tmp.ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append(
                        "host.ip",
                        json!(
                            event
                                .get("_tmp.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("event.action") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    map_strings(event, "event.action", "event.action", str::to_lowercase)?;
                    Ok(())
                })();
            }

            let _cond = { !event.has_value("client.geo") && event.has_value("client.ip") };
            if _cond {
                if event.has_value("client.ip") {
                    if let Some(ip_str) = event.get_string("client.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("client.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("client.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("client.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("client.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("client.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("client.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("client.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("client.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("client.ip") };
            if _cond {
                if event.has_value("client.ip") {
                    if let Some(ip_str) = event.get_string("client.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-ASN.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                            if let Some(v) = geo.get("asn") {
                                event.set("client.as.asn", v.clone())?;
                            }
                            if let Some(v) = geo.get("organization_name") {
                                event.set("client.as.organization_name", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("client.as.asn") };
            if _cond {
                if event.has_value("client.as.asn") {
                    event.rename("client.as.asn", "client.as.number")?;
                }
            }

            let _cond = { event.has_value("client.as.organization_name") };
            if _cond {
                if event.has_value("client.as.organization_name") {
                    event.rename("client.as.organization_name", "client.as.organization.name")?;
                }
            }

            let _cond = {
                event
                    .get("network.transport")
                    .is_some_and(|v| v.is_string())
                    && event.get("network.transport").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("view")),
                        serde_json::Value::String(s) => s.contains("view"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(input) = event.get_string("network.transport") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(rest) = remaining.strip_prefix("view ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        let Some(pos) = remaining.find(": ") else {
                            break 'dissect false;
                        };
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix(": ") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        captured.push(("network.transport", remaining));
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "network.transport".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            if event.has_value("network.transport") {
                map_strings(
                    event,
                    "network.transport",
                    "network.transport",
                    str::to_lowercase,
                )?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.remove("_conf");
                event.remove("_tmp");
                Ok(())
            })();

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
                        "Processor '{}' {}in pipeline {} failed with message '{}'",
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
