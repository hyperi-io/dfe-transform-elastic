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

            event.set("observer.vendor", json!("Cisco"))?;

            event.set("observer.product", json!("Nexus"))?;

            event.set("observer.type", json!("switches"))?;

            event.set("event.kind", json!("event"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has("message") {
                    event.rename("message", "event.original")?;
                }
            }

            let _cond = {
                event.has_value("event.original")
                    && event
                        .get_str("event.original")
                        .is_some_and(|s| !s.is_empty())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: ^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{NUMBER:cisco_nexus.log.sequence_number:long}:%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name}):%{SPACE}%{SYSLOGTIMESTAMP:temp.timestamp}:%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$
                        if !cached_grok!("^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{NUMBER:cisco_nexus.log.sequence_number:long}:%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name}):%{SPACE}%{SYSLOGTIMESTAMP:temp.timestamp}:%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$").extract_into(&input, event)? {
                        // Grok pattern: ^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:cisco_nexus.log.syslog_time}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}%{NUMBER:cisco_nexus.log.sequence_number:long}:%{SPACE}%{SYSLOGTIMESTAMP:temp.timestamp}%{SPACE}%{WORD:temp.timezone}:%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$
                        if !cached_grok!("^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:cisco_nexus.log.syslog_time}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}%{NUMBER:cisco_nexus.log.sequence_number:long}:%{SPACE}%{SYSLOGTIMESTAMP:temp.timestamp}%{SPACE}%{WORD:temp.timezone}:%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$").extract_into(&input, event)? {
                            // Grok pattern: ^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:cisco_nexus.log.syslog_time}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}(?::)?%{SPACE}(?:(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}))%{SPACE}%{WORD:temp.timezone}):%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$
                            if !cached_grok_mapped!("^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:cisco_nexus.log.syslog_time}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}(?::)?%{SPACE}(?:(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}))%{SPACE}%{WORD:temp.timezone}):%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$", [("temp_timestamp", "temp.timestamp")]).extract_into(&input, event)? {
                                // Grok pattern: ^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:cisco_nexus.log.syslog_time}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}(?::)?%{SPACE}%{WORD:temp.timezone}:%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$
                                if !cached_grok!("^<%{NUMBER:cisco_nexus.log.priority_number:long}>%{SYSLOGTIMESTAMP:cisco_nexus.log.syslog_time}%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}(?::)?%{SPACE}%{WORD:temp.timezone}:%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$").extract_into(&input, event)? {
                                    // Grok pattern: ^<%{NUMBER:cisco_nexus.log.priority_number:long}>(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name}):%{SPACE}(?:(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}))%{SPACE}%{WORD:temp.timezone}):%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$
                                    if !cached_grok_mapped!("^<%{NUMBER:cisco_nexus.log.priority_number:long}>(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name}):%{SPACE}(?:(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}))%{SPACE}%{WORD:temp.timezone}):%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$", [("temp_timestamp", "temp.timestamp")]).extract_into(&input, event)? {
                                        // Grok pattern: ^<%{NUMBER:cisco_nexus.log.priority_number:long}>:%{SPACE}(?:(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}))%{SPACE}%{WORD:temp.timezone})%{SPACE}:%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$
                                        if !cached_grok_mapped!("^<%{NUMBER:cisco_nexus.log.priority_number:long}>:%{SPACE}(?:(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}))%{SPACE}%{WORD:temp.timezone})%{SPACE}:%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$", [("temp_timestamp", "temp.timestamp")]).extract_into(&input, event)? {
                                            // Grok pattern: ^(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}))%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$
                                            if !cached_grok_mapped!("^(?P<temp_timestamp>(?:%{YEAR}%{SPACE}%{MONTH}%{SPACE}%{MONTHDAY}%{SPACE}%{TIME}))%{SPACE}(%{IP:cisco_nexus.log.ip_address}|%{NOTSPACE:cisco_nexus.log.switch_name})%{SPACE}(?:(?:%%{WORD:cisco_nexus.log.facility}-(?:(%{INT:cisco_nexus.log.slot_number:long}|%{WORD:cisco_nexus.log.standby})-)?%{INT:cisco_nexus.log.severity:long}-%{WORD:cisco_nexus.log.type}:)?%{DATA:cisco_nexus.log.description})$", [("temp_timestamp", "temp.timestamp")]).extract_into(&input, event)? {
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
                    event.set("_ingest.on_failure_processor_tag", "grok_syslog_line")?;
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
                event.has_value("temp.timezone")
                    && event
                        .get_str("temp.timezone")
                        .is_some_and(|s| !s.is_empty())
                    && event.has_value("_conf.tz_map")
                    && !event.has_value("event.timezone")
            };
            if _cond {
                // Painless script
                // Source: for (def item : ctx._conf.tz_map) {\n  if (item.tz_short == ctx.temp.timezone) {\n    ctx.temp.timezone = item.tz_long;\n    break;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec(
                    event,
                    cached_script!(
                        r#"for (def item : ctx._conf.tz_map) {\n  if (item.tz_short == ctx.temp.timezone) {\n    ctx.temp.timezone = item.tz_long;\n    break;\n  }\n}"#
                    ),
                )?;
            }

            let _cond = { !event.has_value("temp.timezone") && !event.has_value("event.timezone") };
            if _cond {
                if event.has("_conf.tz_offset") {
                    event.rename("_conf.tz_offset", "temp.timezone")?;
                }
            }

            let _cond = { !event.has_value("event.timezone") };
            if _cond {
                if event.has("temp.timezone") {
                    event.rename("temp.timezone", "event.timezone")?;
                }
            }

            let _cond = {
                event.has_value("cisco_nexus.log.syslog_time") && !event.has_value("temp.timestamp")
            };
            if _cond {
                let v = json!(true);
                if !painless_is_empty_value(&v) {
                    event.set("temp.syslog_timestamp_used", v)?;
                }
            }

            let _cond = {
                event.has_value("cisco_nexus.log.syslog_time") && !event.has_value("temp.timestamp")
            };
            if _cond {
                if let Some(v) = event
                    .get("cisco_nexus.log.syslog_time")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("temp.timestamp", v)?;
                }
            }

            let _cond = {
                event.has_value("temp.timestamp")
                    && event
                        .get_str("temp.timestamp")
                        .is_some_and(|s| !s.is_empty())
                    && !event.has_value("event.timezone")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("temp.timestamp") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "yyyy MMM d HH:mm:ss zzz",
                                "yyyy MMM dd HH:mm:ss zzz",
                                "yyyy MMM  d HH:mm:ss zzz",
                                "yyyy MMM d HH:mm:ss.SSS zzz",
                                "yyyy MMM dd HH:mm:ss.SSS zzz",
                                "yyyy MMM  d HH:mm:ss.SSS zzz",
                                "yyyy MMM d HH:mm:ss",
                                "yyyy MMM dd HH:mm:ss",
                                "yyyy MMM  d HH:mm:ss",
                                "yyyy MMM d HH:mm:ss.SSS",
                                "yyyy MMM dd HH:mm:ss.SSS",
                                "yyyy MMM  d HH:mm:ss.SSS",
                                "MMM  d HH:mm:ss",
                                "MMM dd HH:mm:ss",
                                "MMM d HH:mm:ss",
                                "MMM  d HH:mm:ss.SSS",
                                "MMM dd HH:mm:ss.SSS",
                                "MMM d HH:mm:ss.SSS",
                            ],
                            None,
                            None,
                        ) {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_set_timestamp")?;
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
                event.has_value("temp.timestamp")
                    && event
                        .get_str("temp.timestamp")
                        .is_some_and(|s| !s.is_empty())
                    && event.has_value("event.timezone")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("temp.timestamp") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "yyyy MMM d HH:mm:ss zzz",
                                "yyyy MMM dd HH:mm:ss zzz",
                                "yyyy MMM  d HH:mm:ss zzz",
                                "yyyy MMM d HH:mm:ss.SSS zzz",
                                "yyyy MMM dd HH:mm:ss.SSS zzz",
                                "yyyy MMM  d HH:mm:ss.SSS zzz",
                                "yyyy MMM d HH:mm:ss",
                                "yyyy MMM dd HH:mm:ss",
                                "yyyy MMM  d HH:mm:ss",
                                "yyyy MMM d HH:mm:ss.SSS",
                                "yyyy MMM dd HH:mm:ss.SSS",
                                "yyyy MMM  d HH:mm:ss.SSS",
                                "MMM  d HH:mm:ss",
                                "MMM dd HH:mm:ss",
                                "MMM d HH:mm:ss",
                                "MMM  d HH:mm:ss.SSS",
                                "MMM dd HH:mm:ss.SSS",
                                "MMM d HH:mm:ss.SSS",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            event.set("@timestamp", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_set_timestamp_with_timezone",
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
                event.has_value("cisco_nexus.log.syslog_time") && event.has_value("event.timezone")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cisco_nexus.log.syslog_time") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "MMM  d HH:mm:ss",
                                "MMM dd HH:mm:ss",
                                "MMM d HH:mm:ss",
                                "MMM  d HH:mm:ss.SSS",
                                "MMM dd HH:mm:ss.SSS",
                                "MMM d HH:mm:ss.SSS",
                            ],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            event.set("cisco_nexus.log.syslog_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_set_syslog_time_output_with_timezone",
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
                event.has_value("cisco_nexus.log.syslog_time") && !event.has_value("event.timezone")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("cisco_nexus.log.syslog_time") {
                        if let Some(parsed) = parse_date_out(
                            &date_str,
                            &[
                                "MMM  d HH:mm:ss",
                                "MMM dd HH:mm:ss",
                                "MMM d HH:mm:ss",
                                "MMM  d HH:mm:ss.SSS",
                                "MMM dd HH:mm:ss.SSS",
                                "MMM d HH:mm:ss.SSS",
                            ],
                            None,
                            None,
                        ) {
                            event.set("cisco_nexus.log.syslog_time", parsed)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_set_syslog_time_output_no_timezone",
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
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    })
                    && event.get_bool("temp.syslog_timestamp_used") == Some(true)
            };
            if _cond {
                if let Some(v) = event
                    .get("cisco_nexus.log.syslog_time")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cisco_nexus.log.time", v)?;
                }
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    })
                    && !event.has_value("cisco_nexus.log.time")
            };
            if _cond {
                if let Some(v) = event
                    .get("@timestamp")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cisco_nexus.log.time", v)?;
                }
            }

            let _cond = {
                event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    })
            };
            if _cond {
                if let Some(v) = event
                    .get("event.timezone")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cisco_nexus.log.timezone", v)?;
                }
            }

            if let Some(v) = event
                .get("cisco_nexus.log.priority_number")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("log.syslog.priority", v)?;
            }

            if let Some(v) = event
                .get("cisco_nexus.log.switch_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.name", v)?;
            }

            if let Some(v) = event
                .get("cisco_nexus.log.switch_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.hostname", v)?;
            }

            let _cond = { event.has_value("host.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.hostname")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cisco_nexus.log.ip_address") };
            if _cond {
                event.append_unique(
                    "observer.ip",
                    json!(
                        event
                            .get("cisco_nexus.log.ip_address")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("cisco_nexus.log.ip_address") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("cisco_nexus.log.ip_address")
                            .map_or_else(String::new, painless_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("cisco_nexus.log.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.code", v)?;
            }

            if let Some(v) = event
                .get("cisco_nexus.log.severity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.severity", v)?;
            }

            if let Some(v) = event
                .get("cisco_nexus.log.severity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("log.syslog.severity.code", v)?;
            }

            if let Some(v) = event
                .get("cisco_nexus.log.sequence_number")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.sequence", v)?;
            }

            let _cond = { event.has_value("event.severity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def LogLevelValue = (int) ctx.event.severity;\nif (LogLevelValue >= 0 && LogLevelValue < params.LogLevel.length) {\n  ctx.log.put('level', params['LogLevel'][LogLevelValue]);\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_params(
                        event,
                        cached_script!(
                            r#"def LogLevelValue = (int) ctx.event.severity;\nif (LogLevelValue >= 0 && LogLevelValue < params.LogLevel.length) {\n  ctx.log.put('level', params['LogLevel'][LogLevelValue]);\n}"#
                        ),
                        cached_params!(
                            "{\"LogLevel\":[\"emergency\",\"alert\",\"critical\",\"error\",\"warning\",\"notification\",\"informational\",\"debugging\"]}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_set_log_level",
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
                event.has_value("cisco_nexus.log.priority_number")
                    && event.has_value("event.severity")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.log.syslog.facility = new HashMap();\nctx.log.syslog.facility.code = (ctx.cisco_nexus.log.priority_number - ctx.event.severity)/8;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec(
                        event,
                        cached_script!(
                            r#"ctx.log.syslog.facility = new HashMap();\nctx.log.syslog.facility.code = (ctx.cisco_nexus.log.priority_number - ctx.event.severity)/8;\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_to_set_log_syslog_facility_code",
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

            if event.has("cisco_nexus.log.description") {
                if let Some(s) = event.get_string("cisco_nexus.log.description") {
                    let trimmed = s.trim().to_string();
                    event.set("cisco_nexus.log.description", trimmed)?;
                }
            }

            if let Some(v) = event
                .get("cisco_nexus.log.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = {
                event.has_value("event.code")
                    && event.get_str("event.code").is_some_and(|s| {
                        [
                            "IF_DOWN_ADMIN_DOWN",
                            "IF_ADMIN_UP",
                            "SPEED",
                            "IF_DUPLEX",
                            "IF_RX_FLOW_CONTROL",
                            "IF_TX_FLOW_CONTROL",
                            "IF_UP",
                            "IF_XCVR_WARNING",
                            "VSHD_SYSLOG_CONFIG_I",
                            "DETECT_MULTIPLE_PEERS",
                            "SYSTEM_MSG",
                            "UPDOWN",
                            "CFGWRITE_STARTED",
                            "CFGWRITE_DONE",
                            "INVAL_IP",
                            "L2FM_MAC_MOVE2",
                            "DUPLEX_MISMATCH",
                            "NATIVE_VLAN_MISMATCH",
                            "LOGIN_SUCCESS",
                            "LOGOUT",
                            "LOGOUT_C6K",
                            "L3_VPC_UNEQUAL_WEIGHT",
                            "AAA_ACCOUNTING_MESSAGE",
                            "TACACS_WARNING",
                            "DUP_HOSTS",
                            "NF_PARITY_ERROR",
                            "EXCESSIVE_PARITY_ERROR",
                            "LINEPROTO",
                            "THRESHOLD_VIOLATION",
                            "SYSLOG_SL_MSG_WARNING",
                        ]
                        .contains(&s.to_uppercase().as_str())
                    })
            };
            if _cond {
                // Begin nested pipeline: "pipeline_extract_message"
                let _cond = {
                    event.get_str("event.code").is_some_and(|s| {
                        [
                            "IF_DOWN_ADMIN_DOWN",
                            "IF_ADMIN_UP",
                            "SPEED",
                            "IF_DUPLEX",
                            "IF_RX_FLOW_CONTROL",
                            "IF_TX_FLOW_CONTROL",
                            "IF_UP",
                            "IF_XCVR_WARNING",
                        ]
                        .contains(&s.to_uppercase().as_str())
                    })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("message") {
                            // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is up in mode %{DATA:cisco_nexus.log.interface.mode}$
                            if !cached_grok!("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is up in mode %{DATA:cisco_nexus.log.interface.mode}$").extract_into(&input, event)? {
                // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is %{GREEDYDATA}$
                if !cached_grok!("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name} is %{GREEDYDATA}$").extract_into(&input, event)? {
                // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational speed changed to %{DATA:cisco_nexus.log.operational.speed}$
                if !cached_grok!("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational speed changed to %{DATA:cisco_nexus.log.operational.speed}$").extract_into(&input, event)? {
                // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational duplex mode changed to %{DATA:cisco_nexus.log.operational.duplex_mode}$
                if !cached_grok!("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational duplex mode changed to %{DATA:cisco_nexus.log.operational.duplex_mode}$").extract_into(&input, event)? {
                // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational Receive Flow Control state changed to %{DATA:cisco_nexus.log.operational.receive_flow_control_state}$
                if !cached_grok!("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational Receive Flow Control state changed to %{DATA:cisco_nexus.log.operational.receive_flow_control_state}$").extract_into(&input, event)? {
                // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational Transmit Flow Control state changed to %{DATA:cisco_nexus.log.operational.transmit_flow_control_state}$
                if !cached_grok!("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, operational Transmit Flow Control state changed to %{DATA:cisco_nexus.log.operational.transmit_flow_control_state}$").extract_into(&input, event)? {
                // Grok pattern: ^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, %{GREEDYDATA}$
                if !cached_grok!("^(?:%{GREEDYDATA}%{SPACE}(?i)interface)%{SPACE}%{DATA:cisco_nexus.log.interface.name}, %{GREEDYDATA}$").extract_into(&input, event)? {
                }
                }
                }
                }
                }
                }
                }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("event.code").is_some_and(|s| {
                        [
                            "VSHD_SYSLOG_CONFIG_I",
                            "DETECT_MULTIPLE_PEERS",
                            "UPDOWN",
                            "CFGWRITE_STARTED",
                            "LINEPROTO",
                        ]
                        .contains(&s.to_uppercase().as_str())
                    })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("message") {
                            // Grok pattern: ^Configured from vty by %{USERNAME:user.name} on %{IP:source.ip}@%{DATA:cisco_nexus.log.terminal}$
                            if !cached_grok!("^Configured from vty by %{USERNAME:user.name} on %{IP:source.ip}@%{DATA:cisco_nexus.log.terminal}$").extract_into(&input, event)? {
                // Grok pattern: ^Multiple peers detected on %{DATA:cisco_nexus.log.interface.name}$
                if !cached_grok!("^Multiple peers detected on %{DATA:cisco_nexus.log.interface.name}$").extract_into(&input, event)? {
                // Grok pattern: ^Line (?i)protocol on Interface %{DATA:cisco_nexus.log.interface.name}, changed state to %{DATA:cisco_nexus.log.line_protocol_state}$
                if !cached_grok!("^Line (?i)protocol on Interface %{DATA:cisco_nexus.log.interface.name}, changed state to %{DATA:cisco_nexus.log.line_protocol_state}$").extract_into(&input, event)? {
                // Grok pattern: ^Interface %{DATA:cisco_nexus.log.interface.name}, changed state to %{DATA:cisco_nexus.log.state}$
                if !cached_grok!("^Interface %{DATA:cisco_nexus.log.interface.name}, changed state to %{DATA:cisco_nexus.log.state}$").extract_into(&input, event)? {
                // Grok pattern: ^%{DATA}(PID %{NUMBER:process.pid:long})%{GREEDYDATA}$
                if !cached_grok!("^%{DATA}(PID %{NUMBER:process.pid:long})%{GREEDYDATA}$").extract_into(&input, event)? {
                }
                }
                }
                }
                }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event
                        .get_str("event.code")
                        .is_some_and(|s| ["SYSTEM_MSG"].contains(&s.to_uppercase().as_str()))
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("message") {
                            // Grok pattern: ^%{DATA}authentication failure; %{GREEDYDATA:temp.message} - %{GREEDYDATA}$
                            if !cached_grok!("^%{DATA}authentication failure; %{GREEDYDATA:temp.message} - %{GREEDYDATA}$").extract_into(&input, event)? {
                // Grok pattern: ^%{DATA}Authentication failure for %{USERNAME:user.name} from %{IP:source.ip} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$
                if !cached_grok!("^%{DATA}Authentication failure for %{USERNAME:user.name} from %{IP:source.ip} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$").extract_into(&input, event)? {
                // Grok pattern: ^%{DATA}Authentication failed for user %{USERNAME:user.name} from %{IP:source.ip} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$
                if !cached_grok!("^%{DATA}Authentication failed for user %{USERNAME:user.name} from %{IP:source.ip} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$").extract_into(&input, event)? {
                // Grok pattern: ^Login failed for user %{USERNAME:user.name} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$
                if !cached_grok!("^Login failed for user %{USERNAME:user.name} - %{WORD:network.protocol}\\[%{NUMBER:process.pid:long}\\]%{GREEDYDATA}$").extract_into(&input, event)? {
                // Grok pattern: ^%{DATA} : %{GREEDYDATA:temp.message2}$
                if !cached_grok!("^%{DATA} : %{GREEDYDATA:temp.message2}$").extract_into(&input, event)? {
                }
                }
                }
                }
                }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("event.code").is_some_and(|s| {
                        [
                            "INVAL_IP",
                            "L2FM_MAC_MOVE2",
                            "DUPLEX_MISMATCH",
                            "NATIVE_VLAN_MISMATCH",
                            "THRESHOLD_VIOLATION",
                        ]
                        .contains(&s.to_uppercase().as_str())
                    })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("message") {
                            // Grok pattern: ^%{DATA:network.protocol} %{DATA}%{SPACE}Received packet with invalid destination IP address (%{DATA}) from %{CISCOMAC:source.mac} on %{DATA:cisco_nexus.log.interface.name}$
                            if !cached_grok!("^%{DATA:network.protocol} %{DATA}%{SPACE}Received packet with invalid destination IP address (%{DATA}) from %{CISCOMAC:source.mac} on %{DATA:cisco_nexus.log.interface.name}$").extract_into(&input, event)? {
                // Grok pattern: ^Mac %{CISCOMAC:source.mac} in %{DATA:cisco_nexus.log.interface.name} has moved from %{GREEDYDATA}$
                if !cached_grok!("^Mac %{CISCOMAC:source.mac} in %{DATA:cisco_nexus.log.interface.name} has moved from %{GREEDYDATA}$").extract_into(&input, event)? {
                // Grok pattern: ^%{DATA} mismatch discovered on %{DATA:cisco_nexus.log.network.ingress_interface}(?:\\(%{DATA}\\))?, with %{DATA:cisco_nexus.log.network.egress_interface}(?:\\(%{DATA}\\))?$
                if !cached_grok!("^%{DATA} mismatch discovered on %{DATA:cisco_nexus.log.network.ingress_interface}(?:\\(%{DATA}\\))?, with %{DATA:cisco_nexus.log.network.egress_interface}(?:\\(%{DATA}\\))?$").extract_into(&input, event)? {
                // Grok pattern: ^%{DATA:cisco_nexus.log.interface.name}: Rx power high warning; Operating value: %{DATA:cisco_nexus.log.operating_value}, Threshold value: %{DATA:cisco_nexus.log.threshold_value}.$
                if !cached_grok!("^%{DATA:cisco_nexus.log.interface.name}: Rx power high warning; Operating value: %{DATA:cisco_nexus.log.operating_value}, Threshold value: %{DATA:cisco_nexus.log.threshold_value}.$").extract_into(&input, event)? {
                }
                }
                }
                }
                        }
                        Ok(())
                    })();
                }
                let _cond = {
                    event.get_str("event.code").is_some_and(|s| {
                        ["LOGIN_SUCCESS", "LOGOUT", "LOGOUT_C6K"]
                            .contains(&s.to_uppercase().as_str())
                    })
                };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if let Some(input) = event.get_string("message") {
                            // Grok pattern: ^Login Success \\[user: %{USERNAME:user.name}\\] \\[Source: %{IP:source.ip}\\] \\[localport: %{NUMBER:source.port:long}\\] at %{GREEDYDATA}$
                            if !cached_grok!("^Login Success \\[user: %{USERNAME:user.name}\\] \\[Source: %{IP:source.ip}\\] \\[localport: %{NUMBER:source.port:long}\\] at %{GREEDYDATA}$").extract_into(&input, event)? {
                // Grok pattern: ^User %{USERNAME:user.name} %{GREEDYDATA}\\(%{IP:source.ip}\\)$
                if !cached_grok!("^User %{USERNAME:user.name} %{GREEDYDATA}\\(%{IP:source.ip}\\)$").extract_into(&input, event)? {
                }
                }
                        }
                        Ok(())
                    })();
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has("source.mac") {
                        if let Some(s) = event.get_string("source.mac") {
                            let re = cached_regex!("[.]");
                            let replaced = re.replace_all(&s, "").into_owned();
                            event.set("source.mac", replaced)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "gsub")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "gsub_sourcemac_remove_dot",
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
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // SKIPPED: pattern unsupported by the regex engine: (..)(?!$)
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "gsub")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "gsub_sourcemac_add_hyphen",
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
                if event.has("source.mac") {
                    if let Some(s) = event.get_string("source.mac") {
                        let uppered = s.to_uppercase();
                        event.set("source.mac", uppered)?;
                    }
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("temp.message") {
                        if let Some(kv_str) = event.get_string("temp.message") {
                            for pair in cached_regex!("\\s+").split(&kv_str).into_iter() {
                                if pair.trim().is_empty() {
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=") else {
                                    return Err(TransformError::ParseError {
                                        path: "temp.message".into(),
                                        message: format!("does not contain value_split: {pair}"),
                                    });
                                };
                                {
                                    if !key.is_empty() {
                                        event.set(&format!("temp.{}", key), value)?;
                                    }
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has("temp.message2") {
                        if let Some(kv_str) = event.get_string("temp.message2") {
                            for pair in kv_str.split(" ; ") {
                                if pair.trim().is_empty() {
                                    continue;
                                }
                                let Some((key, value)) = pair.split_once("=") else {
                                    return Err(TransformError::ParseError {
                                        path: "temp.message2".into(),
                                        message: format!("does not contain value_split: {pair}"),
                                    });
                                };
                                {
                                    if !key.is_empty() {
                                        event.set(&format!("temp.{}", key), value)?;
                                    }
                                }
                            }
                        }
                    }
                    Ok(())
                })();
                if event.has("temp.logname") {
                    event.rename("temp.logname", "cisco_nexus.log.logname")?;
                }
                if event.has("temp.uid") {
                    event.rename("temp.uid", "cisco_nexus.log.uid")?;
                }
                if event.has("temp.euid") {
                    event.rename("temp.euid", "cisco_nexus.log.euid")?;
                }
                if event.has("temp.tty") {
                    event.rename("temp.tty", "cisco_nexus.log.tty")?;
                }
                if event.has("temp.ruser") {
                    event.rename("temp.ruser", "cisco_nexus.log.ruser")?;
                }
                if event.has("temp.rhost") {
                    event.rename("temp.rhost", "cisco_nexus.log.rhost")?;
                }
                if event.has("temp.user") {
                    event.rename("temp.user", "user.name")?;
                }
                if event.has("temp.COMMAND") {
                    event.rename("temp.COMMAND", "cisco_nexus.log.command")?;
                }
                if event.has("temp.PWD") {
                    event.rename("temp.PWD", "cisco_nexus.log.pwd")?;
                }
                if event.has("temp.TTY") {
                    event.rename("temp.TTY", "cisco_nexus.log.tty")?;
                }
                if event.has("temp.USER") {
                    event.rename("temp.USER", "user.name")?;
                }
                if event.has("network.protocol") {
                    if let Some(s) = event.get_string("network.protocol") {
                        let lowered = s.to_lowercase();
                        event.set("network.protocol", lowered)?;
                    }
                }
                let _cond = {
                    event.has_value("cisco_nexus.log.interface.name")
                        || event.has_value("cisco_nexus.log.network.ingress_interface")
                        || event.has_value("cisco_nexus.log.network.egress_interface")
                        || event.get_str("event.code").is_some_and(|s| {
                            [
                                "L2FM_MAC_MOVE2",
                                "L3_VPC_UNEQUAL_WEIGHT",
                                "AAA_ACCOUNTING_MESSAGE",
                                "DUP_HOSTS",
                                "NF_PARITY_ERROR",
                                "EXCESSIVE_PARITY_ERROR",
                                "DETECT_MULTIPLE_PEERS",
                                "TACACS_WARNING",
                                "SYSLOG_SL_MSG_WARNING",
                            ]
                            .contains(&s.to_uppercase().as_str())
                        })
                        || event.get_str("message").is_some_and(|s| {
                            s.to_lowercase().contains("kex_exchange_identification")
                        })
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("network")]))?;
                }
                let _cond = {
                    event.has_value("cisco_nexus.log.interface.name")
                        || event.has_value("cisco_nexus.log.network.ingress_interface")
                        || event.has_value("cisco_nexus.log.network.egress_interface")
                        || event.get_str("event.code").is_some_and(|s| {
                            [
                                "VSHD_SYSLOG_CONFIG_I",
                                "L2FM_MAC_MOVE2",
                                "L3_VPC_UNEQUAL_WEIGHT",
                                "AAA_ACCOUNTING_MESSAGE",
                                "DUP_HOSTS",
                                "NF_PARITY_ERROR",
                                "EXCESSIVE_PARITY_ERROR",
                                "DETECT_MULTIPLE_PEERS",
                                "TACACS_WARNING",
                                "SYSLOG_SL_MSG_WARNING",
                            ]
                            .contains(&s.to_uppercase().as_str())
                        })
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = {
                    ["VSHD_SYSLOG_CONFIG_I", "CFGWRITE_STARTED", "CFGWRITE_DONE"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("configuration")]))?;
                }
                let _cond = {
                    ["CFGWRITE_STARTED", "CFGWRITE_DONE"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("LOGIN_SUCCESS")
                        || (event.get_str("event.code") == Some("SYSTEM_MSG")
                            && (event
                                .get_str("message")
                                .is_some_and(|s| s.to_lowercase().contains("authentication"))
                                || event.get_str("message").is_some_and(|s| {
                                    s.to_lowercase().contains("authentication failure")
                                })
                                || event
                                    .get_str("message")
                                    .is_some_and(|s| s.to_lowercase().contains("login"))))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("authentication")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("LOGIN_SUCCESS")
                        || (event.get_str("event.code") == Some("SYSTEM_MSG")
                            && (event.get_str("message").is_some_and(|s| {
                                s.to_lowercase().contains("authentication failed")
                            }) || event.get_str("message").is_some_and(|s| {
                                s.to_lowercase().contains("authentication failure")
                            }) || event
                                .get_str("message")
                                .is_some_and(|s| s.to_lowercase().contains("login failed"))))
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("end")]))?;
                }
                let _cond = {
                    ["LOGOUT", "LOGOUT_C6K"].contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("authentication")]),
                    )?;
                }
                let _cond = {
                    ["LOGOUT", "LOGOUT_C6K"].contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("end")]))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("SYSTEM_MSG")
                        && event.has_value("cisco_nexus.log.command")
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("iam"), json!("process")]),
                    )?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("SYSTEM_MSG")
                        && event.has_value("cisco_nexus.log.command")
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("start")]))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("SYSTEM_MSG")
                        && !event.has_value("event.category")
                        && (event.get_str("cisco_nexus.log.facility") == Some("USER")
                            || event.get_str("cisco_nexus.log.facility") == Some("KERN"))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("host")]))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("SYSTEM_MSG")
                        && !event.has_value("event.type")
                        && (event.get_str("cisco_nexus.log.facility") == Some("USER")
                            || event.get_str("cisco_nexus.log.facility") == Some("KERN"))
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = {
                    event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("kex_exchange_identification"))
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("connection")]))?;
                }
                let _cond = {
                    event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("failed"))
                        || event
                            .get_str("message")
                            .is_some_and(|s| s.to_lowercase().contains("failure"))
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event
                        .get_str("message")
                        .is_some_and(|s| s.to_lowercase().contains("successful"))
                        || event
                            .get_str("message")
                            .is_some_and(|s| s.to_lowercase().contains("success"))
                        || event.get_str("event.code") == Some("IF_ADMIN_UP")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("IF_DOWN_ADMIN_DOWN")
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    ["EXCESSIVE_PARITY_ERROR", "NF_PARITY_ERROR"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("INVAL_IP")
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    ["DUPLEX_MISMATCH", "NATIVE_VLAN_MISMATCH"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("L3_VPC_UNEQUAL_WEIGHT")
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("DUP_HOSTS")
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("TACACS_WARNING")
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("SYSTEM_MSG")
                        && event.get_str("cisco_nexus.log.facility") == Some("KERN")
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("SYSTEM_MSG")
                        && !event.has_value("event.outcome")
                        && event.has_value("message")
                        && event.get_str("message").is_some_and(|s| {
                            s.to_lowercase().contains("kex_exchange_identification")
                        })
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("IF_UP")
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    ["LOGOUT", "LOGOUT_C6K"].contains(&event.get_str("event.code").unwrap_or(""))
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("VSHD_SYSLOG_CONFIG_I")
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("AAA_ACCOUNTING_MESSAGE")
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("SYSTEM_MSG")
                        && event.has_value("cisco_nexus.log.command")
                        && event
                            .get_str("message")
                            .is_some_and(|s| s.to_lowercase().contains("command not allowed"))
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("SYSTEM_MSG")
                        && event.has_value("cisco_nexus.log.command")
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("CFGWRITE_STARTED")
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("unknown"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("CFGWRITE_DONE")
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("DETECT_MULTIPLE_PEERS")
                        && !event.has_value("event.outcome")
                };
                if _cond {
                    event.set("event.outcome", json!("unknown"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("UPDOWN")
                        && !event.has_value("event.outcome")
                        && (event.get_str("cisco_nexus.log.line_protocol_state") == Some("up")
                            || event.get_str("cisco_nexus.log.state") == Some("up"))
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("UPDOWN")
                        && !event.has_value("event.outcome")
                        && (event.get_str("cisco_nexus.log.line_protocol_state") == Some("down")
                            || event.get_str("cisco_nexus.log.state") == Some("down"))
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = { event.get_str("event.code") == Some("IF_DOWN_ADMIN_DOWN") };
                if _cond {
                    event.set("event.action", json!("interface-down"))?;
                }
                let _cond = {
                    ["IF_ADMIN_UP", "IF_UP"].contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("interface-up"))?;
                }
                let _cond = { event.get_str("event.code") == Some("SPEED") };
                if _cond {
                    event.set("event.action", json!("interface-speed-changed"))?;
                }
                let _cond = { event.get_str("event.code") == Some("IF_DUPLEX") };
                if _cond {
                    event.set("event.action", json!("interface-duplex-changed"))?;
                }
                let _cond = {
                    ["IF_RX_FLOW_CONTROL", "IF_TX_FLOW_CONTROL"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("interface-flow-control-changed"))?;
                }
                let _cond = { event.get_str("event.code") == Some("IF_XCVR_WARNING") };
                if _cond {
                    event.set("event.action", json!("transceiver-warning"))?;
                }
                let _cond = { event.get_str("event.code") == Some("UPDOWN") };
                if _cond {
                    event.set("event.action", json!("interface-state-changed"))?;
                }
                let _cond = { event.get_str("event.code") == Some("LINEPROTO") };
                if _cond {
                    event.set("event.action", json!("interface-state-changed"))?;
                }
                let _cond = { event.get_str("event.code") == Some("VSHD_SYSLOG_CONFIG_I") };
                if _cond {
                    event.set("event.action", json!("configuration-changed"))?;
                }
                let _cond = { event.get_str("event.code") == Some("CFGWRITE_STARTED") };
                if _cond {
                    event.set("event.action", json!("config-write-started"))?;
                }
                let _cond = { event.get_str("event.code") == Some("CFGWRITE_DONE") };
                if _cond {
                    event.set("event.action", json!("config-write-completed"))?;
                }
                let _cond = { event.get_str("event.code") == Some("LOGIN_SUCCESS") };
                if _cond {
                    event.set("event.action", json!("logged-in"))?;
                }
                let _cond = {
                    ["LOGOUT", "LOGOUT_C6K"].contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("logged-out"))?;
                }
                let _cond = { event.get_str("event.code") == Some("DETECT_MULTIPLE_PEERS") };
                if _cond {
                    event.set("event.action", json!("multiple-peers-detected"))?;
                }
                let _cond = { event.get_str("event.code") == Some("INVAL_IP") };
                if _cond {
                    event.set("event.action", json!("invalid-packet-received"))?;
                }
                let _cond = { event.get_str("event.code") == Some("SYSLOG_SL_MSG_WARNING") };
                if _cond {
                    event.set("event.action", json!("arp-warning"))?;
                }
                let _cond = { event.get_str("event.code") == Some("L2FM_MAC_MOVE2") };
                if _cond {
                    event.set("event.action", json!("mac-address-moved"))?;
                }
                let _cond = {
                    ["EXCESSIVE_PARITY_ERROR", "NF_PARITY_ERROR"]
                        .contains(&event.get_str("event.code").unwrap_or(""))
                };
                if _cond {
                    event.set("event.action", json!("hardware-error"))?;
                }
                let _cond = { event.get_str("event.code") == Some("DUPLEX_MISMATCH") };
                if _cond {
                    event.set("event.action", json!("duplex-mismatch-detected"))?;
                }
                let _cond = { event.get_str("event.code") == Some("NATIVE_VLAN_MISMATCH") };
                if _cond {
                    event.set("event.action", json!("vlan-mismatch-detected"))?;
                }
                let _cond = { event.get_str("event.code") == Some("L3_VPC_UNEQUAL_WEIGHT") };
                if _cond {
                    event.set("event.action", json!("vpc-config-mismatch"))?;
                }
                let _cond = { event.get_str("event.code") == Some("AAA_ACCOUNTING_MESSAGE") };
                if _cond {
                    event.set("event.action", json!("session-recorded"))?;
                }
                let _cond = { event.get_str("event.code") == Some("TACACS_WARNING") };
                if _cond {
                    event.set("event.action", json!("tacacs-lookup-failed"))?;
                }
                let _cond = { event.get_str("event.code") == Some("DUP_HOSTS") };
                if _cond {
                    event.set("event.action", json!("duplicate-host-detected"))?;
                }
                let _cond = { event.get_str("event.code") == Some("THRESHOLD_VIOLATION") };
                if _cond {
                    event.set("event.action", json!("transceiver-threshold-violated"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("SYSTEM_MSG")
                        && event.has_value("cisco_nexus.log.command")
                        && event
                            .get_str("message")
                            .is_some_and(|s| s.to_lowercase().contains("command not allowed"))
                };
                if _cond {
                    event.set("event.action", json!("command-denied"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("SYSTEM_MSG")
                        && event.has_value("cisco_nexus.log.command")
                        && !event.has_value("event.action")
                };
                if _cond {
                    event.set("event.action", json!("command-executed"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("SYSTEM_MSG")
                        && !event.has_value("event.action")
                        && event.has_value("message")
                        && (event
                            .get_str("message")
                            .is_some_and(|s| s.to_lowercase().contains("authentication"))
                            || event
                                .get_str("message")
                                .is_some_and(|s| s.to_lowercase().contains("login failed")))
                };
                if _cond {
                    event.set("event.action", json!("authentication-failure"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("SYSTEM_MSG")
                        && !event.has_value("event.action")
                        && event.has_value("message")
                        && event.get_str("message").is_some_and(|s| {
                            s.to_lowercase().contains("kex_exchange_identification")
                        })
                };
                if _cond {
                    event.set("event.action", json!("connection-failed"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("SYSTEM_MSG")
                        && !event.has_value("event.action")
                        && event.get_str("cisco_nexus.log.facility") == Some("KERN")
                };
                if _cond {
                    event.set("event.action", json!("hardware-error"))?;
                }
                let _cond = {
                    event.get_str("event.code") == Some("SYSTEM_MSG")
                        && !event.has_value("event.action")
                };
                if _cond {
                    event.set("event.action", json!("system-message"))?;
                }
                let _cond = { event.has_value("source.ip") };
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
                let _cond = { event.has_value("user.name") };
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
                // End nested pipeline: "pipeline_extract_message"
            }

            let _cond = {
                event.has_value("cisco_nexus.log.facility")
                    && event
                        .get_str("cisco_nexus.log.facility")
                        .is_some_and(|s| s.to_lowercase().contains("arp"))
            };
            if _cond {
                event.set("network.protocol", json!("arp"))?;
            }

            event.remove("_conf");
            event.remove("temp");

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
                event.remove("cisco_nexus.log.time");
                event.remove("cisco_nexus.log.description");
                event.remove("cisco_nexus.log.type");
                event.remove("cisco_nexus.log.severity");
                event.remove("cisco_nexus.log.priority_number");
                event.remove("cisco_nexus.log.ip_address");
                event.remove("cisco_nexus.log.switch_name");
                event.remove("cisco_nexus.log.sequence_number");
            }

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec(
                event,
                cached_script!(
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
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, painless_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, painless_to_string), event.get("_ingest.pipeline").map_or_else(String::new, painless_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, painless_to_string))))?;
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
