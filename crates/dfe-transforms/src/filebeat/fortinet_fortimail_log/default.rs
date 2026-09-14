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

            event.set("observer.product", json!("FortiMail"))?;

            event.set("observer.type", json!("firewall"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            event.set("event.kind", json!("event"))?;

            let _cond = {
                event.has_value("event.original") && event.get_str("event.original") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: ^<%{NUMBER:fortinet_fortimail.log.priority_number:long}>%{GREEDYDATA:temp.message},msg=\\\"%{DATA:fortinet_fortimail.log.message}\\\"$
                        // Grok pattern: ^<%{NUMBER:fortinet_fortimail.log.priority_number:long}>%{GREEDYDATA:temp.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^<%{NUMBER:fortinet_fortimail.log.priority_number:long}>%{GREEDYDATA:temp.message},msg=\\\"%{DATA:fortinet_fortimail.log.message}\\\"$"
                                ),
                                cached_grok!(
                                    "^<%{NUMBER:fortinet_fortimail.log.priority_number:long}>%{GREEDYDATA:temp.message}$"
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
                    event.set("_ingest.on_failure_processor_tag", "grok_syslog_line")?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("temp.message") {
                    gsub_field(
                        event,
                        "temp.message",
                        "temp.message",
                        cached_regex!(",,"),
                        ",",
                    )?;
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "gsub")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "gsub_multiple_redundant_commas",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("temp.message") {
                    if let Some(kv_str) = event.get_string("temp.message") {
                        let mut kv_gap = false;
                        for pair in cached_regex!("(,(?=(?:[^'\"]|'[^']*'|\"[^\"]*\")*$))")
                            .split(&kv_str)
                            .into_iter()
                        {
                            if pair.is_empty() {
                                kv_gap = true;
                                continue;
                            }
                            let Some((key, value)) = ({
                                let parts = cached_regex!("(?<!\\\\)=").splitn(&pair, 2);
                                match (parts.first(), parts.get(1)) {
                                    (Some(k), Some(v)) => Some((k.clone(), v.clone())),
                                    _ => None,
                                }
                            })
                            .filter(|_| !kv_gap) else {
                                return Err(TransformError::KvValueSplit {
                                    field: "temp.message".into(),
                                    split: "(?<!\\\\)=".into(),
                                });
                            };
                            {
                                let key = &key[..];
                                let key = key.trim_matches(|c: char| matches!(c, ' '));
                                let value = value.trim_matches(|c: char| matches!(c, '\"' | '\''));
                                if !key.is_empty() {
                                    kv_put(event, &format!("temp.{}", key), value)?;
                                }
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "kv")?;
                event.set("_ingest.on_failure_processor_tag", "split_key_value_pair")?;
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

            let _cond = {
                event.has_value("_conf.tz_offset")
                    && event.get_str("_conf.tz_offset") != Some("local")
            };
            if _cond {
                if let Some(v) = event
                    .get("_conf.tz_offset")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.timezone", v)?;
                }
            }

            if event.has_value("temp.date") {
                event.rename("temp.date", "fortinet_fortimail.log.date")?;
            }

            if event.has_value("temp.time") {
                event.rename("temp.time", "fortinet_fortimail.log.time")?;
            }

            let _cond = {
                event.has_value("fortinet_fortimail.log.date")
                    && event.get_str("fortinet_fortimail.log.date") != Some("")
                    && event.has_value("fortinet_fortimail.log.time")
                    && event.get_str("fortinet_fortimail.log.time") != Some("")
            };
            if _cond {
                event.set(
                    "temp.timestamp",
                    json!(format!(
                        "{}T{}",
                        event
                            .get("fortinet_fortimail.log.date")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("fortinet_fortimail.log.time")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = {
                event.has_value("temp.timestamp")
                    && event.get_str("temp.timestamp") != Some("")
                    && event.has_value("event.timezone")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("temp.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy-MM-dd'T'HH:mm:ss.SSS", "yyyy-MM-dd'T'HH:mm:ss"],
                            event.get_str("event.timezone"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "temp.timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
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

            let _cond = {
                event.has_value("temp.timestamp")
                    && event.get_str("temp.timestamp") != Some("")
                    && !event.has_value("event.timezone")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("temp.timestamp") {
                        match parse_date_out(
                            &date_str,
                            &["yyyy-MM-dd'T'HH:mm:ss.SSS", "yyyy-MM-dd'T'HH:mm:ss"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "temp.timestamp".into(),
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
                        "date_temp_timestamp_0ab6aaa4",
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

            if event.has_value("temp.device_id") {
                event.rename("temp.device_id", "fortinet_fortimail.log.device_id")?;
            }

            if let Some(v) = event
                .get("fortinet_fortimail.log.device_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.serial_number", v)?;
            }

            let _cond = { event.get_str("fortinet_fortimail.log.priority_number") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("fortinet_fortimail.log.priority_number") {
                        if let Some(val) = event.get("fortinet_fortimail.log.priority_number") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "fortinet_fortimail.log.priority_number".into(),
                                    message,
                                }
                            })?;
                            event.set("fortinet_fortimail.log.priority_number", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_priority_number_to_long",
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

            if let Some(v) = event
                .get("fortinet_fortimail.log.priority_number")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("log.syslog.priority", v)?;
            }

            if event.has_value("temp.log_id") {
                event.rename("temp.log_id", "fortinet_fortimail.log.id")?;
            }

            if let Some(v) = event
                .get("fortinet_fortimail.log.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.code", v)?;
            }

            if event.has_value("temp.type") {
                event.rename("temp.type", "fortinet_fortimail.log.type")?;
            }

            if event.has_value("temp.subtype") {
                event.rename("temp.subtype", "fortinet_fortimail.log.sub_type")?;
            }

            if event.has_value("temp.pri") {
                event.rename("temp.pri", "fortinet_fortimail.log.priority")?;
            }

            if let Some(v) = event
                .get("fortinet_fortimail.log.priority")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("log.level", v)?;
            }

            if event.has_value("temp.subject") {
                event.rename("temp.subject", "fortinet_fortimail.log.subject")?;
            }

            if let Some(v) = event
                .get("fortinet_fortimail.log.subject")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("email.subject", v)?;
            }

            if event.has_value("temp.url") {
                event.rename("temp.url", "fortinet_fortimail.log.url")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("fortinet_fortimail.log.url") {
                    if !uri_parts(event, "fortinet_fortimail.log.url", "url", true, false)?
                        && event
                            .get_str("fortinet_fortimail.log.url")
                            .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "fortinet_fortimail.log.url".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uri_parts")?;
                if let Some(v) = event.get("fortinet_fortimail.log.url").cloned() {
                    event.set("url.original", v)?;
                }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.has_value("log.level")
                    && event.get_str("log.level") != Some("")
                    && event.has_value("log.syslog.priority")
                    && event.get_str("log.syslog.priority") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script, resolved to its runners at generation time
                    // Source: ArrayList severities = new ArrayList(['emergency','alert','critical','error','warning','notice','information','debug']);\nHashMap sevrityMap = new HashMap();\nHashMap facilityMap = new HashMap();\nString severity = ctx.log.level.toLowerCase();\nLong priority = ctx.log.syslog.priority;\nfor (def i = 0; i < severities.length; i++) {\n  if (severities[i] == severity){\n    sevrityMap.put('code',i);\n    ctx.log.syslog.severity = sevrityMap;\n    facilityMap.put('code', (priority-i)/8);\n    ctx.log.syslog.facility = facilityMap;\n    break;\n  }\n}
                    syslog_priority(event, &SyslogPriorityScript::new(None, true, true, false));
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_for_set_log.syslog.severity.code_log.syslog.facility.code",
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

            if event.has_value("temp.msg") {
                event.rename("temp.msg", "fortinet_fortimail.log.message")?;
            }

            if let Some(v) = event
                .get("fortinet_fortimail.log.message")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = {
                event.has_value("fortinet_fortimail.log.type")
                    && event
                        .get_str("fortinet_fortimail.log.type")
                        .is_some_and(|s| s.to_lowercase() == "statistics")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_history"
                let _cond = { event.get_str("temp.client_ip") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("temp.client_ip") {
                            if let Some(val) = event.get("temp.client_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "temp.client_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("fortinet_fortimail.log.client.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_ip_to_ip",
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
                let _cond = { event.has_value("fortinet_fortimail.log.client.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("fortinet_fortimail.log.client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.client.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.ip", v)?;
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
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
                if event.has_value("temp.client_name") {
                    event.rename("temp.client_name", "fortinet_fortimail.log.client.name")?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.client.name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("fortinet_fortimail.log.client.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.client.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.user.name", v)?;
                }
                let _cond = { event.get_str("temp.dst_ip") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("temp.dst_ip") {
                            if let Some(val) = event.get("temp.dst_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "temp.dst_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("fortinet_fortimail.log.destination_ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_dst_ip_to_ip")?;
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
                let _cond = { event.has_value("fortinet_fortimail.log.destination_ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("fortinet_fortimail.log.destination_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.destination_ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.ip", v)?;
                }
                if event.has_value("temp.direction") {
                    event.rename("temp.direction", "fortinet_fortimail.log.direction")?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.direction")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("email.direction", v)?;
                }
                if event.has_value("temp.from") {
                    event.rename("temp.from", "fortinet_fortimail.log.from")?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.from") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("fortinet_fortimail.log.from")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.from") };
                if _cond {
                    event.append_unique(
                        "email.from.address",
                        json!(
                            event
                                .get("fortinet_fortimail.log.from")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("temp.subject") {
                    event.rename("temp.subject", "fortinet_fortimail.log.subject")?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.subject")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("email.subject", v)?;
                }
                if event.has_value("temp.to") {
                    event.rename("temp.to", "fortinet_fortimail.log.to")?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.to") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("fortinet_fortimail.log.to")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.to") };
                if _cond {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("fortinet_fortimail.log.to")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("temp.mailer") {
                    event.rename("temp.mailer", "fortinet_fortimail.log.mailer")?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.mailer")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("email.x_mailer", v)?;
                }
                if event.has_value("temp.session_id") {
                    event.rename("temp.session_id", "fortinet_fortimail.log.session_id")?;
                }
                if event.has_value("temp.endpoint") {
                    event.rename("temp.endpoint", "fortinet_fortimail.log.endpoint")?;
                }
                if event.has_value("temp.polid") {
                    event.rename("temp.polid", "fortinet_fortimail.log.policy_id")?;
                }
                if event.has_value("temp.domain") {
                    event.rename("temp.domain", "fortinet_fortimail.log.domain")?;
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("fortinet_fortimail.log.domain") {
                        if let Some(domain) = event.get_string("fortinet_fortimail.log.domain") {
                            // Public suffix list lookup for registered domain extraction.
                            // A failed lookup writes NO target field, which is what
                            // Elasticsearch does.
                            if let Some(rd) = registered_domain_lookup(&domain) {
                                event.set("server.domain", json!(domain))?;
                                if let Some(registered) = rd.registered_domain {
                                    event.set("server.registered_domain", json!(registered))?;
                                }
                                event.set("server.top_level_domain", json!(rd.top_level_domain))?;
                                if let Some(sub) = rd.subdomain {
                                    event.set("server.subdomain", json!(sub))?;
                                }
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "registered_domain")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "registered_domain_for_domain",
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
                if event.has_value("temp.resolved") {
                    event.rename("temp.resolved", "fortinet_fortimail.log.resolved")?;
                }
                let _cond = {
                    event
                        .get_str("fortinet_fortimail.log.resolved")
                        .is_some_and(|s| s.to_lowercase() == "ok")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event
                        .get_str("fortinet_fortimail.log.resolved")
                        .is_some_and(|s| s.to_lowercase() == "fail")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = { !event.has_value("event.outcome") };
                if _cond {
                    event.set("event.outcome", json!("unknown"))?;
                }
                if event.has_value("temp.virus") {
                    event.rename("temp.virus", "fortinet_fortimail.log.virus")?;
                }
                if event.has_value("temp.disposition") {
                    event.rename("temp.disposition", "fortinet_fortimail.log.disposition")?;
                }
                if event.has_value("temp.classifier") {
                    event.rename("temp.classifier", "fortinet_fortimail.log.classifier")?;
                }
                if event.has_value("temp.hfrom") {
                    event.rename("temp.hfrom", "fortinet_fortimail.log.hfrom")?;
                }
                if event.has_value("temp.src_type") {
                    event.rename("temp.src_type", "fortinet_fortimail.log.source.type")?;
                }
                let _cond = { event.get_str("temp.message_length") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("temp.message_length") {
                            if let Some(val) = event.get("temp.message_length") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "temp.message_length".into(),
                                        message,
                                    }
                                })?;
                                event.set("fortinet_fortimail.log.message_length", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_message_length_to_long",
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
                if event.has_value("temp.client_cc") {
                    event.rename("temp.client_cc", "fortinet_fortimail.log.client.cc")?;
                }
                if event.has_value("temp.detail") {
                    event.rename("temp.detail", "fortinet_fortimail.log.detail")?;
                }
                if event.has_value("temp.message_id") {
                    event.rename("temp.message_id", "fortinet_fortimail.log.message_id")?;
                }
                if event.has_value("temp.recv_time") {
                    event.rename("temp.recv_time", "fortinet_fortimail.log.recv_time")?;
                }
                if event.has_value("temp.notif_delay") {
                    event.rename("temp.notif_delay", "fortinet_fortimail.log.notif_delay")?;
                }
                let _cond = { event.get_str("temp.scan_time") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("temp.scan_time") {
                            if let Some(val) = event.get("temp.scan_time") {
                                let converted =
                                    convert_value(val, "double").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "temp.scan_time".into(),
                                            message,
                                        }
                                    })?;
                                event.set("fortinet_fortimail.log.scan_time", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_scan_time_to_double",
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
                let _cond = { event.get_str("temp.xfer_time") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("temp.xfer_time") {
                            if let Some(val) = event.get("temp.xfer_time") {
                                let converted =
                                    convert_value(val, "double").map_err(|message| {
                                        TransformError::ParseError {
                                            path: "temp.xfer_time".into(),
                                            message,
                                        }
                                    })?;
                                event.set("fortinet_fortimail.log.xfer_time", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_xfer_time_to_double",
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
                if event.has_value("temp.srcfolder") {
                    event.rename("temp.srcfolder", "fortinet_fortimail.log.source.folder")?;
                }
                if event.has_value("temp.read_status") {
                    event.rename("temp.read_status", "fortinet_fortimail.log.read_status")?;
                }
                // End nested pipeline: "pipeline_history"
            }

            let _cond = {
                event.has_value("fortinet_fortimail.log.type")
                    && event
                        .get_str("fortinet_fortimail.log.type")
                        .is_some_and(|s| s.to_lowercase() == "kevent")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_system"
                event.set("event.outcome", json!("unknown"))?;
                let _cond = {
                    event.get_str("fortinet_fortimail.log.sub_type") == Some("admin")
                        && event
                            .get_str("temp.status")
                            .is_some_and(|s| s.to_lowercase() == "success")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("fortinet_fortimail.log.sub_type") == Some("admin")
                        && event
                            .get_str("temp.status")
                            .is_some_and(|s| s.to_lowercase() == "failure")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = {
                    event.get_str("fortinet_fortimail.log.sub_type") == Some("admin")
                        && (event
                            .get_str("fortinet_fortimail.log.message")
                            .is_some_and(|s| s.to_lowercase().contains("login"))
                            || event
                                .get_str("fortinet_fortimail.log.message")
                                .is_some_and(|s| s.to_lowercase().contains("logged")))
                };
                if _cond {
                    event.set(
                        "event.category",
                        Value::Array(vec![json!("authentication")]),
                    )?;
                }
                let _cond = {
                    event.get_str("fortinet_fortimail.log.sub_type") == Some("admin")
                        && (event
                            .get_str("fortinet_fortimail.log.message")
                            .is_some_and(|s| s.to_lowercase().contains("login successfully"))
                            || event
                                .get_str("fortinet_fortimail.log.message")
                                .is_some_and(|s| s.to_lowercase().contains("logged in"))
                            || event
                                .get_str("fortinet_fortimail.log.message")
                                .is_some_and(|s| s.to_lowercase().contains("login")))
                };
                if _cond {
                    event.append_unique("event.type", json!("end"))?;
                }
                let _cond = {
                    event.get_str("fortinet_fortimail.log.sub_type") == Some("admin")
                        && event.has_value("event.category")
                };
                if _cond {
                    event.append_unique("event.type", json!("info"))?;
                }
                let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("config") };
                if _cond {
                    event.append_unique("event.category", json!("configuration"))?;
                }
                let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("config") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = {
                    event.get_str("fortinet_fortimail.log.sub_type") == Some("config")
                        && event
                            .get_str("fortinet_fortimail.log.message")
                            .is_some_and(|s| s.to_lowercase().contains("delet"))
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("deletion")]))?;
                }
                let _cond = {
                    event.get_str("fortinet_fortimail.log.sub_type") == Some("system")
                        && event
                            .get_str("fortinet_fortimail.log.message")
                            .is_some_and(|s| s.to_lowercase().contains("system"))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("host")]))?;
                }
                let _cond = {
                    (event.get_str("fortinet_fortimail.log.sub_type") == Some("system")
                        && event
                            .get_str("fortinet_fortimail.log.message")
                            .is_some_and(|s| s.to_lowercase().contains("change")))
                        || (event.get_str("fortinet_fortimail.log.sub_type") == Some("update")
                            && event
                                .get_str("fortinet_fortimail.log.message")
                                .is_some_and(|s| s.to_lowercase().contains("update")))
                };
                if _cond {
                    event.set("event.category", Value::Array(vec![json!("configuration")]))?;
                }
                let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("system") };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("info")]))?;
                }
                let _cond = {
                    event.get_str("fortinet_fortimail.log.sub_type") == Some("system")
                        && event
                            .get_str("fortinet_fortimail.log.message")
                            .is_some_and(|s| s.to_lowercase().contains("shutdown"))
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("end")]))?;
                }
                let _cond = {
                    (event.get_str("fortinet_fortimail.log.sub_type") == Some("system")
                        && event
                            .get_str("fortinet_fortimail.log.message")
                            .is_some_and(|s| s.to_lowercase().contains("change")))
                        || (event.get_str("fortinet_fortimail.log.sub_type") == Some("update")
                            && event
                                .get_str("fortinet_fortimail.log.message")
                                .is_some_and(|s| s.to_lowercase().contains("update")))
                };
                if _cond {
                    event.set("event.type", Value::Array(vec![json!("change")]))?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "fortinet_fortimail.log.user",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "user.name",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("message") {
                        if let Some(input) = event.get_string("message") {
                            // Grok pattern: ^(?:%{DATA}(?i)interface %{NUMBER:fortinet_fortimail.log.port:long}%{DATA} (?:(?i)user %{NOTSPACE:temp.user})) %{DATA}%{IP:fortinet_fortimail.log.ip}%{GREEDYDATA:temp.msg}$
                            // Grok pattern: ^(?:%{DATA}(?i)interface %{NUMBER:fortinet_fortimail.log.port:long}%{DATA} (?:(?i)user %{NOTSPACE:temp.user}))%{GREEDYDATA:temp.msg}$
                            // Grok pattern: ^%{DATA}(?:(?i)user %{NOTSPACE:temp.user}) %{DATA}%{IP:fortinet_fortimail.log.ip}%{GREEDYDATA:temp.msg}$
                            // Grok pattern: ^authserver: (?P<temp_block_action>(?:(?:added|removed))) block rule for %{IP:temp.block_ip}$
                            // Grok pattern: ^authserver: (?P<temp_block_action>(?:(?:added|removed))) block rule for %{IP:temp.block_ip} until %{GREEDYDATA}$
                            // Grok pattern: ^%{DATA}(?:(?i)user %{NOTSPACE:temp.user}) %{GREEDYDATA:temp.msg}$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^(?:%{DATA}(?i)interface %{NUMBER:fortinet_fortimail.log.port:long}%{DATA} (?:(?i)user %{NOTSPACE:temp.user})) %{DATA}%{IP:fortinet_fortimail.log.ip}%{GREEDYDATA:temp.msg}$"
                                    ),
                                    cached_grok!(
                                        "^(?:%{DATA}(?i)interface %{NUMBER:fortinet_fortimail.log.port:long}%{DATA} (?:(?i)user %{NOTSPACE:temp.user}))%{GREEDYDATA:temp.msg}$"
                                    ),
                                    cached_grok!(
                                        "^%{DATA}(?:(?i)user %{NOTSPACE:temp.user}) %{DATA}%{IP:fortinet_fortimail.log.ip}%{GREEDYDATA:temp.msg}$"
                                    ),
                                    cached_grok_mapped!(
                                        "^authserver: (?P<temp_block_action>(?:(?:added|removed))) block rule for %{IP:temp.block_ip}$",
                                        [("temp_block_action", "temp.block_action")]
                                    ),
                                    cached_grok_mapped!(
                                        "^authserver: (?P<temp_block_action>(?:(?:added|removed))) block rule for %{IP:temp.block_ip} until %{GREEDYDATA}$",
                                        [("temp_block_action", "temp.block_action")]
                                    ),
                                    cached_grok!(
                                        "^%{DATA}(?:(?i)user %{NOTSPACE:temp.user}) %{GREEDYDATA:temp.msg}$"
                                    ),
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
                if let Some(v) = event
                    .get("fortinet_fortimail.log.port")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.port", v)?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("fortinet_fortimail.log.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.block_ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("temp.block_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.block_ip") };
                if _cond {
                    event.append_unique(
                        "client.ip",
                        json!(
                            event
                                .get("temp.block_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.block_ip") };
                if _cond {
                    event.set(
                        "event.action",
                        json!(format!(
                            "{}_block_rule",
                            event
                                .get("temp.block_action")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "fortinet_fortimail.log.user",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "user.name",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("temp.action") {
                    event.rename("temp.action", "fortinet_fortimail.log.action")?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.action")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    if !event.has("event.action") {
                        event.set("event.action", v)?;
                    }
                }
                if event.has_value("temp.module") {
                    event.rename("temp.module", "fortinet_fortimail.log.module")?;
                }
                if event.has_value("temp.submodule") {
                    event.rename("temp.submodule", "fortinet_fortimail.log.sub_module")?;
                }
                if event.has_value("temp.ui") {
                    event.rename("temp.ui", "fortinet_fortimail.log.ui")?;
                }
                if event.has_value("temp.reason") {
                    event.rename("temp.reason", "fortinet_fortimail.log.reason")?;
                }
                if event.has_value("temp.status") {
                    event.rename("temp.status", "fortinet_fortimail.log.status")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("fortinet_fortimail.log.ui") {
                        if let Some(input) = event.get_string("fortinet_fortimail.log.ui") {
                            // Grok pattern: ^(?P<fortinet_fortimail_log_network>(?:(?i)(?:telnet|ssh|http)))%{SPACE}\\(%{SPACE}%{IP:fortinet_fortimail.log.ui_ip}%{SPACE}\\)$
                            // Grok pattern: ^%{WORD}%{SPACE}\\(%{SPACE}%{IP:fortinet_fortimail.log.ui_ip}%{SPACE}\\)$
                            // Grok pattern: ^%{DATA}%{IP:fortinet_fortimail.log.ui_ip}%{GREEDYDATA:temp.msg}$
                            if !extract_first_match(
                                &[
                                    cached_grok_mapped!(
                                        "^(?P<fortinet_fortimail_log_network>(?:(?i)(?:telnet|ssh|http)))%{SPACE}\\(%{SPACE}%{IP:fortinet_fortimail.log.ui_ip}%{SPACE}\\)$",
                                        [(
                                            "fortinet_fortimail_log_network",
                                            "fortinet_fortimail.log.network"
                                        )]
                                    ),
                                    cached_grok!(
                                        "^%{WORD}%{SPACE}\\(%{SPACE}%{IP:fortinet_fortimail.log.ui_ip}%{SPACE}\\)$"
                                    ),
                                    cached_grok!(
                                        "^%{DATA}%{IP:fortinet_fortimail.log.ui_ip}%{GREEDYDATA:temp.msg}$"
                                    ),
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
                if let Some(v) = event
                    .get("fortinet_fortimail.log.ui_ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.ip", v)?;
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
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
                let _cond = { event.has_value("fortinet_fortimail.log.ui_ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("fortinet_fortimail.log.ui_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.network")
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
                // End nested pipeline: "pipeline_system"
            }

            let _cond = {
                event.has_value("fortinet_fortimail.log.type")
                    && event
                        .get_str("fortinet_fortimail.log.type")
                        .is_some_and(|s| s.to_lowercase() == "event")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_mail"
                event.set("event.category", Value::Array(vec![json!("email")]))?;
                let _cond = {
                    event.get_str("fortinet_fortimail.log.sub_type") == Some("webmail")
                        && event
                            .get_str("temp.status")
                            .is_some_and(|s| s.to_lowercase() == "success")
                };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = {
                    event.get_str("fortinet_fortimail.log.sub_type") == Some("webmail")
                        && event
                            .get_str("temp.status")
                            .is_some_and(|s| s.to_lowercase() == "failure")
                };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("webmail") };
                if _cond {
                    event.append_unique("event.category", json!("authentication"))?;
                }
                event.set("event.type", Value::Array(vec![json!("info")]))?;
                let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("webmail") };
                if _cond {
                    event.append("event.type", json!("end"))?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "fortinet_fortimail.log.user",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "user.name",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.get_str("fortinet_fortimail.log.sub_type") == Some("webmail") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("message") {
                            if let Some(input) = event.get_string("message") {
                                // Grok pattern: ^(?:(?:%{DATA}(?i)user %{NOTSPACE:temp.user}|%{DATA}(?i)login for \\'%{NOTSPACE:temp.user}\\')) %{DATA}%{IP:fortinet_fortimail.log.ip}%{GREEDYDATA:temp.msg}$
                                // Grok pattern: ^%{DATA}(?i)user %{NOTSPACE:temp.user} %{GREEDYDATA:temp.msg}$
                                if !extract_first_match(
                                    &[
                                        cached_grok!(
                                            "^(?:(?:%{DATA}(?i)user %{NOTSPACE:temp.user}|%{DATA}(?i)login for \\'%{NOTSPACE:temp.user}\\')) %{DATA}%{IP:fortinet_fortimail.log.ip}%{GREEDYDATA:temp.msg}$"
                                        ),
                                        cached_grok!(
                                            "^%{DATA}(?i)user %{NOTSPACE:temp.user} %{GREEDYDATA:temp.msg}$"
                                        ),
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
                let _cond = { event.has_value("fortinet_fortimail.log.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("fortinet_fortimail.log.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "fortinet_fortimail.log.user",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "user.name",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("temp.action") {
                    event.rename("temp.action", "fortinet_fortimail.log.action")?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.action")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
                if event.has_value("temp.module") {
                    event.rename("temp.module", "fortinet_fortimail.log.module")?;
                }
                if event.has_value("temp.reason") {
                    event.rename("temp.reason", "fortinet_fortimail.log.reason")?;
                }
                if event.has_value("temp.session_id") {
                    event.rename("temp.session_id", "fortinet_fortimail.log.session_id")?;
                }
                if event.has_value("temp.status") {
                    event.rename("temp.status", "fortinet_fortimail.log.status")?;
                }
                if event.has_value("temp.submodule") {
                    event.rename("temp.submodule", "fortinet_fortimail.log.sub_module")?;
                }
                if event.has_value("temp.ui") {
                    event.rename("temp.ui", "fortinet_fortimail.log.ui")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("fortinet_fortimail.log.ui") {
                        if let Some(input) = event.get_string("fortinet_fortimail.log.ui") {
                            // Grok pattern: ^(?P<fortinet_fortimail_log_network>(?:SSH|telnet|ssh|http|HTTP))\\\\s*\\\\(\\\\s*%{IP:fortinet_fortimail.log.ui_ip}\\\\s*\\\\)$
                            // Grok pattern: ^%{WORD}\\\\s*\\\\(\\\\s*%{IP:fortinet_fortimail.log.ui_ip}\\\\s*\\\\)$
                            // Grok pattern: ^%{DATA}%{IP:fortinet_fortimail.log.ui_ip}%{GREEDYDATA:temp.msg}$
                            if !extract_first_match(
                                &[
                                    cached_grok_mapped!(
                                        "^(?P<fortinet_fortimail_log_network>(?:SSH|telnet|ssh|http|HTTP))\\\\s*\\\\(\\\\s*%{IP:fortinet_fortimail.log.ui_ip}\\\\s*\\\\)$",
                                        [(
                                            "fortinet_fortimail_log_network",
                                            "fortinet_fortimail.log.network"
                                        )]
                                    ),
                                    cached_grok!(
                                        "^%{WORD}\\\\s*\\\\(\\\\s*%{IP:fortinet_fortimail.log.ui_ip}\\\\s*\\\\)$"
                                    ),
                                    cached_grok!(
                                        "^%{DATA}%{IP:fortinet_fortimail.log.ui_ip}%{GREEDYDATA:temp.msg}$"
                                    ),
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
                if let Some(v) = event
                    .get("fortinet_fortimail.log.ui_ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.ip", v)?;
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
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
                let _cond = { event.has_value("fortinet_fortimail.log.ui_ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("fortinet_fortimail.log.ui_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.network")
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
                // End nested pipeline: "pipeline_mail"
            }

            let _cond = {
                event.has_value("fortinet_fortimail.log.type")
                    && event
                        .get_str("fortinet_fortimail.log.type")
                        .is_some_and(|s| s.to_lowercase() == "virus")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_antivirus"
                if event.has_value("temp.from") {
                    event.rename("temp.from", "fortinet_fortimail.log.from")?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.from") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("fortinet_fortimail.log.from")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.from") };
                if _cond {
                    event.append_unique(
                        "email.from.address",
                        json!(
                            event
                                .get("fortinet_fortimail.log.from")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("temp.to") {
                    event.rename("temp.to", "fortinet_fortimail.log.to")?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.to") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("fortinet_fortimail.log.to")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.to") };
                if _cond {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("fortinet_fortimail.log.to")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.get_str("temp.src") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("temp.src") {
                            if let Some(val) = event.get("temp.src") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "temp.src".into(),
                                        message,
                                    }
                                })?;
                                event.set("fortinet_fortimail.log.source.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_src_to_ip")?;
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
                let _cond = { event.has_value("fortinet_fortimail.log.source.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("fortinet_fortimail.log.source.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.source.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.ip", v)?;
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
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
                if event.has_value("temp.session_id") {
                    event.rename("temp.session_id", "fortinet_fortimail.log.session_id")?;
                }
                // End nested pipeline: "pipeline_antivirus"
            }

            let _cond = {
                event.has_value("fortinet_fortimail.log.type")
                    && event
                        .get_str("fortinet_fortimail.log.type")
                        .is_some_and(|s| s.to_lowercase() == "spam")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_antispam"
                let _cond = { event.get_str("temp.client_ip") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("temp.client_ip") {
                            if let Some(val) = event.get("temp.client_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "temp.client_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("fortinet_fortimail.log.client.ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set(
                            "_ingest.on_failure_processor_tag",
                            "convert_client_ip_to_ip",
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
                let _cond = { event.has_value("fortinet_fortimail.log.client.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("fortinet_fortimail.log.client.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.client.ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.ip", v)?;
                }
                let _cond = { event.has_value("source.ip") };
                if _cond {
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
                if event.has_value("temp.client_name") {
                    event.rename("temp.client_name", "fortinet_fortimail.log.client.name")?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.client.name") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("fortinet_fortimail.log.client.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.client.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("source.user.name", v)?;
                }
                let _cond = { event.get_str("temp.dst_ip") != Some("") };
                if _cond {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("temp.dst_ip") {
                            if let Some(val) = event.get("temp.dst_ip") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "temp.dst_ip".into(),
                                        message,
                                    }
                                })?;
                                event.set("fortinet_fortimail.log.destination_ip", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_dst_ip_to_ip")?;
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
                let _cond = { event.has_value("fortinet_fortimail.log.destination_ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("fortinet_fortimail.log.destination_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.destination_ip")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("destination.ip", v)?;
                }
                if event.has_value("temp.from") {
                    event.rename("temp.from", "fortinet_fortimail.log.from")?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.from") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("fortinet_fortimail.log.from")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.from") };
                if _cond {
                    event.append_unique(
                        "email.from.address",
                        json!(
                            event
                                .get("fortinet_fortimail.log.from")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("temp.subject") {
                    event.rename("temp.subject", "fortinet_fortimail.log.subject")?;
                }
                if let Some(v) = event
                    .get("fortinet_fortimail.log.subject")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("email.subject", v)?;
                }
                if event.has_value("temp.to") {
                    event.rename("temp.to", "fortinet_fortimail.log.to")?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.to") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("fortinet_fortimail.log.to")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.to") };
                if _cond {
                    event.append_unique(
                        "email.to.address",
                        json!(
                            event
                                .get("fortinet_fortimail.log.to")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                if event.has_value("temp.session_id") {
                    event.rename("temp.session_id", "fortinet_fortimail.log.session_id")?;
                }
                if event.has_value("temp.endpoint") {
                    event.rename("temp.endpoint", "fortinet_fortimail.log.endpoint")?;
                }
                // End nested pipeline: "pipeline_antispam"
            }

            let _cond = {
                event.has_value("fortinet_fortimail.log.type")
                    && event
                        .get_str("fortinet_fortimail.log.type")
                        .is_some_and(|s| s.to_lowercase() == "encrypt")
            };
            if _cond {
                // Begin nested pipeline: "pipeline_encryption"
                if event.has_value("temp.session_id") {
                    event.rename("temp.session_id", "fortinet_fortimail.log.session_id")?;
                }
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("message") {
                        if let Some(input) = event.get_string("message") {
                            // Grok pattern: ^%{DATA}(?i)user %{NOTSPACE:temp.user} %{DATA}%{IP:fortinet_fortimail.log.ip}%{GREEDYDATA:temp.msg}$
                            // Grok pattern: ^%{DATA}(?i)user %{NOTSPACE:temp.user} %{GREEDYDATA:temp.msg},%{SPACE}sent from:%{SPACE}\\'%{DATA:fortinet_fortimail.log.sent_from}\\',%{SPACE}subject:%{SPACE}\\'%{GREEDYDATA:temp.subject}\\'(?:%{SPACE}%{GREEDYDATA:temp.msg2})$
                            // Grok pattern: ^%{DATA}(?i)user %{NOTSPACE:temp.user} %{GREEDYDATA:temp.msg}$
                            if !extract_first_match(
                                &[
                                    cached_grok!(
                                        "^%{DATA}(?i)user %{NOTSPACE:temp.user} %{DATA}%{IP:fortinet_fortimail.log.ip}%{GREEDYDATA:temp.msg}$"
                                    ),
                                    cached_grok!(
                                        "^%{DATA}(?i)user %{NOTSPACE:temp.user} %{GREEDYDATA:temp.msg},%{SPACE}sent from:%{SPACE}\\'%{DATA:fortinet_fortimail.log.sent_from}\\',%{SPACE}subject:%{SPACE}\\'%{GREEDYDATA:temp.subject}\\'(?:%{SPACE}%{GREEDYDATA:temp.msg2})$"
                                    ),
                                    cached_grok!(
                                        "^%{DATA}(?i)user %{NOTSPACE:temp.user} %{GREEDYDATA:temp.msg}$"
                                    ),
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
                let _cond = { event.has_value("fortinet_fortimail.log.ip") };
                if _cond {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("fortinet_fortimail.log.ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "fortinet_fortimail.log.user",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "user.name",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.user") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("temp.user")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.sent_from") };
                if _cond {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("fortinet_fortimail.log.sent_from")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("fortinet_fortimail.log.sent_from") };
                if _cond {
                    event.append_unique(
                        "email.from.address",
                        json!(
                            event
                                .get("fortinet_fortimail.log.sent_from")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.subject") };
                if _cond {
                    event.append_unique(
                        "fortinet_fortimail.log.subject",
                        json!(
                            event
                                .get("temp.subject")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                let _cond = { event.has_value("temp.subject") };
                if _cond {
                    event.append_unique(
                        "email.subject",
                        json!(
                            event
                                .get("temp.subject")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "pipeline_encryption"
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
                event.remove("fortinet_fortimail.log.action");
                event.remove("fortinet_fortimail.log.client.ip");
                event.remove("fortinet_fortimail.log.client.name");
                event.remove("fortinet_fortimail.log.destination_ip");
                event.remove("fortinet_fortimail.log.device_id");
                event.remove("fortinet_fortimail.log.direction");
                event.remove("fortinet_fortimail.log.from");
                event.remove("fortinet_fortimail.log.ip");
                event.remove("fortinet_fortimail.log.id");
                event.remove("fortinet_fortimail.log.mailer");
                event.remove("fortinet_fortimail.log.message");
                event.remove("fortinet_fortimail.log.network");
                event.remove("fortinet_fortimail.log.port");
                event.remove("fortinet_fortimail.log.priority");
                event.remove("fortinet_fortimail.log.priority_number");
                event.remove("fortinet_fortimail.log.server");
                event.remove("fortinet_fortimail.log.source.ip");
                event.remove("fortinet_fortimail.log.subject");
                event.remove("fortinet_fortimail.log.to");
                event.remove("fortinet_fortimail.log.url");
                event.remove("fortinet_fortimail.log.user");
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_original_event")),
                        serde_json::Value::String(s) => s.contains("preserve_original_event"),
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("event.original");
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object o) {\n  if (o == null || o == '') {\n    return true;\n  } else if (o instanceof Map) {\n    ((Map) o).values().removeIf(v -> drop(v));\n    return (((Map) o).size() == 0);\n  } else if (o instanceof List) {\n    ((List) o).removeIf(v -> drop(v));\n    return (((List) o).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
