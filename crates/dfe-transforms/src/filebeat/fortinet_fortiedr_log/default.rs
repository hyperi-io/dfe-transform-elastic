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

            event.set("observer.vendor", json!("Fortinet"))?;

            event.set("observer.product", json!("FortiEDR"))?;

            event.set("observer.type", json!("edr"))?;

            event.set("event.category", json!("malware"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            if let Some(input) = event.get_string("event.original") {
                // Grok pattern: (?:(?:(?:(?:<%{NONNEGINT:log.syslog.priority:int}>)%{NONNEGINT:log.syslog.version}?\\s*)?(?:(?P<_temp__raw_date>(?:(?:(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND}(?:\\.%{NUMBER})?)?%{ISO8601_TIMEZONE}?)))):?\\s+)?(?:(?:-|%{SYSLOGHOST:log.syslog.hostname}) (?:-|(?P<log_syslog_appname>(?:(?:[^%\\s:\\[]+)))) (?:-|%{POSINT:log.syslog.procid}) (?:-|%{NOTSPACE:log.syslog.msgid})) - (?:{DATA})?(?:(?:(:|\\s)\\s+))?))?\\s*%{GREEDYDATA:_temp_.full_message}
                let _ = cached_grok_mapped!("(?:(?:(?:(?:<%{NONNEGINT:log.syslog.priority:int}>)%{NONNEGINT:log.syslog.version}?\\s*)?(?:(?P<_temp__raw_date>(?:(?:(?:%{YEAR}-%{MONTHNUM}-%{MONTHDAY}[T ]%{HOUR}:?%{MINUTE}(?::?%{SECOND}(?:\\.%{NUMBER})?)?%{ISO8601_TIMEZONE}?)))):?\\s+)?(?:(?:-|%{SYSLOGHOST:log.syslog.hostname}) (?:-|(?P<log_syslog_appname>(?:(?:[^%\\s:\\[]+)))) (?:-|%{POSINT:log.syslog.procid}) (?:-|%{NOTSPACE:log.syslog.msgid})) - (?:{DATA})?(?:(?:(:|\\s)\\s+))?))?\\s*%{GREEDYDATA:_temp_.full_message}", [("_temp__raw_date", "_temp_.raw_date"), ("log_syslog_appname", "log.syslog.appname")]).extract_into(&input, event)?;
            }

            let _cond = { event.has_value("_temp_.raw_date") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_temp_.raw_date") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_temp_.raw_date".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            // Painless script
            // Source: if (ctx.log?.syslog?.priority != null) {\n  def severity = new HashMap();\n  severity['code'] = ctx.log.syslog.priority&0x7;\n  ctx.log.syslog['severity'] = severity;\n  def facility = new HashMap();\n  facility['code'] = ctx.log.syslog.priority>>3;\n  ctx.log.syslog['facility'] = facility;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.log?.syslog?.priority != null) {\n  def severity = new HashMap();\n  severity['code'] = ctx.log.syslog.priority&0x7;\n  ctx.log.syslog['severity'] = severity;\n  def facility = new HashMap();\n  facility['code'] = ctx.log.syslog.priority>>3;\n  ctx.log.syslog['facility'] = facility;\n}\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("_temp_.full_message") {
                    if let Some(kv_str) = event.get_string("_temp_.full_message") {
                        for pair in kv_str.split(";") {
                            if pair.trim().is_empty() {
                                continue;
                            }
                            let Some((key, value)) = pair.split_once(": ") else {
                                return Err(TransformError::ParseError {
                                    path: "_temp_.full_message".into(),
                                    message: format!("does not contain value_split: {pair}"),
                                });
                            };
                            {
                                let key = key.trim_matches(|c| " ".contains(c));
                                let value = value.trim_matches(|c| " ".contains(c));
                                if !key.is_empty() {
                                    kv_put(event, &format!("fortinet.edr.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                Ok(())
            })();

            event.rename("fortinet.edr.Action", "fortinet.edr.action")?;

            event.rename(
                "fortinet.edr.Autonomous System",
                "fortinet.edr.autonomous_system",
            )?;

            event.rename("fortinet.edr.Certificate", "fortinet.edr.certificate")?;

            event.rename("fortinet.edr.Classification", "fortinet.edr.classification")?;

            event.rename("fortinet.edr.Count", "fortinet.edr.count")?;

            event.rename("fortinet.edr.Country", "fortinet.edr.country")?;

            event.rename("fortinet.edr.Destination", "fortinet.edr.destination")?;

            event.rename("fortinet.edr.Device Name", "fortinet.edr.device_name")?;

            event.rename("fortinet.edr.Event ID", "fortinet.edr.event_id")?;

            event.rename("fortinet.edr.First Seen", "fortinet.edr.first_seen")?;

            event.rename("fortinet.edr.Last Seen", "fortinet.edr.last_seen")?;

            event.rename("fortinet.edr.MAC Address", "fortinet.edr.mac_address")?;

            event.rename(
                "fortinet.edr.Operating System",
                "fortinet.edr.operating_system",
            )?;

            event.rename("fortinet.edr.Organization", "fortinet.edr.organization")?;

            event.rename(
                "fortinet.edr.Organization ID",
                "fortinet.edr.organization_id",
            )?;

            event.rename("fortinet.edr.Process Name", "fortinet.edr.process_name")?;

            event.rename("fortinet.edr.Process Path", "fortinet.edr.process_path")?;

            event.rename("fortinet.edr.Process Type", "fortinet.edr.process_type")?;

            event.rename("fortinet.edr.Raw Data ID", "fortinet.edr.raw_data_id")?;

            event.rename("fortinet.edr.Rules List", "fortinet.edr.rules_list")?;

            event.rename("fortinet.edr.Script", "fortinet.edr.script")?;

            event.rename("fortinet.edr.Script Path", "fortinet.edr.script_path")?;

            event.rename("fortinet.edr.Severity", "fortinet.edr.severity")?;

            event.rename("fortinet.edr.Users", "fortinet.edr.users")?;

            let _cond = { event.has_value("fortinet.edr.event_id") };
            if _cond {
                if let Some(v) = event.get("fortinet.edr.event_id").cloned() {
                    event.set("event.id", v)?;
                }
            }

            let _cond = { event.has_value("fortinet.edr.action") };
            if _cond {
                if let Some(v) = event.get("fortinet.edr.action").cloned() {
                    event.set("event.action", v)?;
                }
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = {
                event.has_value("fortinet.edr.device_name")
                    && event.get_str("fortinet.edr.device_name") != Some("N/A")
            };
            if _cond {
                if let Some(v) = event.get("fortinet.edr.device_name").cloned() {
                    event.set("host.hostname", v)?;
                }
            }

            let _cond = {
                event.has_value("fortinet.edr.operating_system")
                    && event.get_str("fortinet.edr.operating_system") != Some("N/A")
            };
            if _cond {
                if let Some(v) = event.get("fortinet.edr.operating_system").cloned() {
                    event.set("host.os.full", v)?;
                }
            }

            let _cond = { event.has_value("host.hostname") };
            if _cond {
                event.append(
                    "related.hosts",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("log.syslog.hostname") };
            if _cond {
                event.append(
                    "related.hosts",
                    json!(
                        event
                            .get("log.syslog.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("fortinet.edr.mac_address")
                    && event.get_str("fortinet.edr.mac_address") != Some("N/A")
            };
            if _cond {
                event.append(
                    "host.mac",
                    json!(
                        event
                            .get("fortinet.edr.mac_address")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("fortinet.edr.users")
                    && event.get_str("fortinet.edr.users") != Some("N/A")
            };
            if _cond {
                if let Some(v) = event.get("fortinet.edr.users").cloned() {
                    event.set("user.id", v)?;
                }
            }

            let _cond = {
                event.has_value("fortinet.edr.process_name")
                    && event.get_str("fortinet.edr.process_name") != Some("N/A")
            };
            if _cond {
                if let Some(v) = event.get("fortinet.edr.process_name").cloned() {
                    event.set("process.name", v)?;
                }
            }

            let _cond = {
                event.has_value("fortinet.edr.process_path")
                    && event.get_str("fortinet.edr.process_path") != Some("N/A")
            };
            if _cond {
                if let Some(v) = event.get("fortinet.edr.process_path").cloned() {
                    event.set("process.executable", v)?;
                }
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
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
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("fortinet.edr.first_seen") };
            if _cond {
                if let Some(date_str) = event.get_as_string("fortinet.edr.first_seen") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "d-MMM-yyyy, HH:mm:ss",
                            "MMM d yyyy HH:mm:ss.SSS z",
                            "MMM d yyyy HH:mm:ss.SSS",
                            "MMM d yyyy HH:mm:ss z",
                            "MMM d yyyy HH:mm:ss",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("fortinet.edr.first_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "fortinet.edr.first_seen".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("fortinet.edr.first_seen") };
            if _cond {
                if let Some(v) = event.get("fortinet.edr.first_seen").cloned() {
                    event.set("event.start", v)?;
                }
            }

            let _cond = { event.has_value("fortinet.edr.last_seen") };
            if _cond {
                if let Some(date_str) = event.get_as_string("fortinet.edr.last_seen") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "ISO8601",
                            "d-MMM-yyyy, HH:mm:ss",
                            "MMM d yyyy HH:mm:ss.SSS z",
                            "MMM d yyyy HH:mm:ss.SSS",
                            "MMM d yyyy HH:mm:ss z",
                            "MMM d yyyy HH:mm:ss",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("fortinet.edr.last_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "fortinet.edr.last_seen".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("fortinet.edr.last_seen") };
            if _cond {
                if let Some(v) = event.get("fortinet.edr.last_seen").cloned() {
                    event.set("event.end", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.remove("_temp_").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "_temp_".into(),
                    });
                }
                Ok(())
            })();

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
