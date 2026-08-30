// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `audit` pipeline.
pub struct Audit;

impl Transform for Audit {
    fn name(&self) -> &str {
        "audit"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("ecs.version", json!("8.11.0"))?;

            let _cond = { !event.has_value("event.original") };
            if _cond {
                if event.has_value("message") {
                    event.rename("message", "event.original")?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(input) = event.get_string("event.original") {
                    // Grok pattern: ^<%{NONNEGINT:log.syslog.priority:long}>%{NONNEGINT} %{TIMESTAMP_ISO8601:_tmp.syslog_ts} %{SYSLOGHOST:_tmp.hostname} (?P<_tmp_payload>(?:{\"format\":\"elastic\",\"version\":\"1.0\",.*}))
                    // Grok pattern: ^%{SYSLOGTIMESTAMP:_tmp.syslog_ts} %{SYSLOGHOST:_tmp.hostname} (?P<_tmp_payload>(?:{\"format\":\"elastic\",\"version\":\"1.0\",.*}))
                    // Grok pattern: (?P<_tmp_payload>(?:{\"format\":\"elastic\",\"version\":\"1.0\",.*}))
                    let _ = extract_first_match(
                        &[
                            cached_grok_mapped!("^<%{NONNEGINT:log.syslog.priority:long}>%{NONNEGINT} %{TIMESTAMP_ISO8601:_tmp.syslog_ts} %{SYSLOGHOST:_tmp.hostname} (?P<_tmp_payload>(?:{\"format\":\"elastic\",\"version\":\"1.0\",.*}))", [("_tmp_payload", "_tmp.payload")]),
                            cached_grok_mapped!("^%{SYSLOGTIMESTAMP:_tmp.syslog_ts} %{SYSLOGHOST:_tmp.hostname} (?P<_tmp_payload>(?:{\"format\":\"elastic\",\"version\":\"1.0\",.*}))", [("_tmp_payload", "_tmp.payload")]),
                            cached_grok_mapped!("(?P<_tmp_payload>(?:{\"format\":\"elastic\",\"version\":\"1.0\",.*}))", [("_tmp_payload", "_tmp.payload")]),
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
                        return Err(TransformError::ParseError {
                            path: "_fail".into(),
                            message: (format!("unexpected event format: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))).to_string(),
                        });
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "_tmp.payload", "_tmp.json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_tmp_payload")?;
                        return Err(TransformError::ParseError {
                            path: "_fail".into(),
                            message: (format!("malformed JSON event: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))).to_string(),
                        });
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                event.rename("_tmp.json.syslog.audit_record", "cyberarkpas.audit")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "rename")?;
                event.set("_ingest.on_failure_processor_tag", "rename_tmp_json_syslog_audit_record")?;
                        return Err(TransformError::ParseError {
                            path: "_fail".into(),
                            message: (format!("unexpected event structure: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))).to_string(),
                        });
            }

                // Painless script
                // Source: ctx.cyberarkpas.audit.entrySet().removeIf(entry -> entry.getValue() == \"\");
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"ctx.cyberarkpas.audit.entrySet().removeIf(entry -> entry.getValue() == \"\");"#))?;

                if event.has_value("_tmp.json.raw") {
                    event.rename("_tmp.json.raw", "cyberarkpas.audit.raw")?;
                }

            let _cond = { event.has_value("cyberarkpas.audit.IsoTimestamp") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("cyberarkpas.audit.IsoTimestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("_tmp.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cyberarkpas.audit.IsoTimestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_cyberarkpas_audit_isotimestamp")?;
                        event.append("error.message", json!(format!("failed to parse ISO timestamp field: {}: {}", event.get("cyberarkpas.audit.IsoTimestamp").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { !event.has_value("_tmp.timestamp") && event.has_value("cyberarkpas.audit.Timestamp") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("cyberarkpas.audit.Timestamp") {
                    match parse_date_out(&date_str, &["MMM dd HH:mm:ss", "ISO8601", "MMM  d HH:mm:ss", "EEE MMM dd HH:mm:ss", "EEE MMM  d HH:mm:ss", "MMM  d HH:mm:ss z", "MMM dd HH:mm:ss z", "EEE MMM  d HH:mm:ss z", "EEE MMM dd HH:mm:ss z", "MMM  d yyyy HH:mm:ss", "MMM dd yyyy HH:mm:ss", "EEE MMM  d yyyy HH:mm:ss", "EEE MMM dd yyyy HH:mm:ss", "MMM  d yyyy HH:mm:ss z", "MMM dd yyyy HH:mm:ss z", "EEE MMM  d yyyy HH:mm:ss z", "EEE MMM dd yyyy HH:mm:ss z"], None, None) {
                        Some(parsed) => event.set("_tmp.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cyberarkpas.audit.Timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_cyberarkpas_audit_timestamp")?;
                        event.append("error.message", json!(format!("failed to parse timestamp field: {}: {}", event.get("cyberarkpas.audit.Timestamp").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { !event.has_value("_tmp.timestamp") && event.has_value("_tmp.syslog_ts") && !event.has_value("event.timezone") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.syslog_ts") {
                    match parse_date_out(&date_str, &["MMM dd HH:mm:ss", "ISO8601", "MMM  d HH:mm:ss", "EEE MMM dd HH:mm:ss", "EEE MMM  d HH:mm:ss", "MMM  d HH:mm:ss z", "MMM dd HH:mm:ss z", "EEE MMM  d HH:mm:ss z", "EEE MMM dd HH:mm:ss z", "MMM  d yyyy HH:mm:ss", "MMM dd yyyy HH:mm:ss", "EEE MMM  d yyyy HH:mm:ss", "EEE MMM dd yyyy HH:mm:ss", "MMM  d yyyy HH:mm:ss z", "MMM dd yyyy HH:mm:ss z", "EEE MMM  d yyyy HH:mm:ss z", "EEE MMM dd yyyy HH:mm:ss z"], None, None) {
                        Some(parsed) => event.set("_tmp.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.syslog_ts".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_tmp_syslog_ts")?;
                        event.append("error.message", json!(format!("failed to parse legacy syslog timestamp: {}: {}", event.get("_tmp.syslog_ts").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { !event.has_value("_tmp.timestamp") && event.has_value("_tmp.syslog_ts") && event.has_value("event.timezone") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("_tmp.syslog_ts") {
                    match parse_date_out(&date_str, &["MMM dd HH:mm:ss", "ISO8601", "MMM  d HH:mm:ss", "EEE MMM dd HH:mm:ss", "EEE MMM  d HH:mm:ss", "MMM  d HH:mm:ss z", "MMM dd HH:mm:ss z", "EEE MMM  d HH:mm:ss z", "EEE MMM dd HH:mm:ss z", "MMM  d yyyy HH:mm:ss", "MMM dd yyyy HH:mm:ss", "EEE MMM  d yyyy HH:mm:ss", "EEE MMM dd yyyy HH:mm:ss", "MMM  d yyyy HH:mm:ss z", "MMM dd yyyy HH:mm:ss z", "EEE MMM  d yyyy HH:mm:ss z", "EEE MMM dd yyyy HH:mm:ss z"], event.get_str("event.timezone") , None) {
                        Some(parsed) => event.set("_tmp.timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.syslog_ts".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_tmp_syslog_ts_2")?;
                        event.append("error.message", json!(format!("failed to parse legacy syslog timestamp: {}: {}", event.get("_tmp.syslog_ts").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let v = json!(event.get("_tmp.timestamp").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("@timestamp", v)?;
            }

                // Painless script
                // Source: def props = ctx.cyberarkpas?.audit?.CAProperties?.CAProperty; if (props instanceof Map) {\n  ctx.cyberarkpas.audit.CAProperties.CAProperty = [ props ];\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def props = ctx.cyberarkpas?.audit?.CAProperties?.CAProperty; if (props instanceof Map) {\n  ctx.cyberarkpas.audit.CAProperties.CAProperty = [ props ];\n}\n"#))?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cyberarkpas.audit.CAProperties.CAProperty") {
                foreach_array(event, "cyberarkpas.audit.CAProperties.CAProperty", |event| {
                    event.set("cyberarkpas.audit.CAProperties.{{{_ingest._value.Name}}}", json!(event.get("_ingest._value.Value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "foreach")?;
                event.set("_ingest.on_failure_processor_tag", "foreach_cyberarkpas_audit_caproperties_caproperty")?;
                        event.append("error.message", json!(format!("failed to process CAProperties array: {}", event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cyberarkpas.audit.CAProperties.CAProperty");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("cyberarkpas.audit.ExtraDetails") {
                if let Some(kv_str) = event.get_string("cyberarkpas.audit.ExtraDetails") {
                    for pair in cached_regex!("(?<!\\\\);").split(&kv_str).into_iter() {
                        if pair.trim().is_empty() {
                            continue;
                        }
                        let Some((key, value)) = ({ let parts = cached_regex!("(?<!\\\\)=").splitn(&pair, 2); match (parts.first(), parts.get(1)) { (Some(k), Some(v)) => Some((k.clone(), v.clone())), _ => None } }) else {
                            return Err(TransformError::ParseError {
                                path: "cyberarkpas.audit.ExtraDetails".into(),
                                message: format!("does not contain value_split: {pair}"),
                            });
                        };
                        {
                            let key = &key[..];
                            if !key.is_empty() {
                                kv_put(event, &format!("_tmp.kv.{}", key), value)?;
                            }
                        }
                    }
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "kv")?;
                event.set("_ingest.on_failure_processor_tag", "kv_cyberarkpas_audit_extradetails")?;
                        event.append("error.message", json!(format!("failed to process ExtraDetails expression '{}': {}", event.get("cyberarkpas.audit.ExtraDetails").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

                event.remove("cyberarkpas.audit.ExtraDetails");

                if event.has_value("_tmp.kv") {
                    event.rename("_tmp.kv", "cyberarkpas.audit.ExtraDetails")?;
                }

                // Painless script
                // Source: String to_snake_case(String s) {\n  /* faster code path for strings that won't need an underscore */\n  if (s.chars().skip(1).noneMatch(Character::isUpperCase)) {\n    return s.toLowerCase();\n  }\n  int run = 0;\n  boolean first = true;\n  StringBuilder result = new StringBuilder();\n  for (char c : s.toCharArray()) {\n    char o = Character.toLowerCase(c);\n    if (c != o) {\n      if (run == 0 && !first) {\n        result.append('_');\n      }\n      run ++;\n    } else {\n      if (run > 1) {\n        char prev = result.charAt(result.length()-1);\n        result.setCharAt(result.length()-1, (char)'_');\n        result.append(prev);\n      }\n      run = 0;\n      first = false;\n    }\n    result.append(o);\n  }\n  return result.toString();\n} def keys_to_snake_case_recursive(Map object) {\n  return object.entrySet().stream().collect(\n    Collectors.toMap(\n      e -> to_snake_case(e.getKey()),\n      e -> e.getValue() instanceof Map ? keys_to_snake_case_recursive(e.getValue()) : e.getValue()\n    )\n  );\n} ctx.cyberarkpas.audit = keys_to_snake_case_recursive(ctx.cyberarkpas.audit);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"String to_snake_case(String s) {\n  /* faster code path for strings that won't need an underscore */\n  if (s.chars().skip(1).noneMatch(Character::isUpperCase)) {\n    return s.toLowerCase();\n  }\n  int run = 0;\n  boolean first = true;\n  StringBuilder result = new StringBuilder();\n  for (char c : s.toCharArray()) {\n    char o = Character.toLowerCase(c);\n    if (c != o) {\n      if (run == 0 && !first) {\n        result.append('_');\n      }\n      run ++;\n    } else {\n      if (run > 1) {\n        char prev = result.charAt(result.length()-1);\n        result.setCharAt(result.length()-1, (char)'_');\n        result.append(prev);\n      }\n      run = 0;\n      first = false;\n    }\n    result.append(o);\n  }\n  return result.toString();\n} def keys_to_snake_case_recursive(Map object) {\n  return object.entrySet().stream().collect(\n    Collectors.toMap(\n      e -> to_snake_case(e.getKey()),\n      e -> e.getValue() instanceof Map ? keys_to_snake_case_recursive(e.getValue()) : e.getValue()\n    )\n  );\n} ctx.cyberarkpas.audit = keys_to_snake_case_recursive(ctx.cyberarkpas.audit);\n"#))?;

                // Painless script
                // Source: def value = ctx.cyberarkpas.audit.rfc5424; ctx.cyberarkpas.audit[\"rfc5424\"] = value == 'yes';\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"def value = ctx.cyberarkpas.audit.rfc5424; ctx.cyberarkpas.audit[\"rfc5424\"] = value == 'yes';\n"#))?;

            event.set("event.kind", json!("event"))?;

            if event.has_value("cyberarkpas.audit.action") {
                map_strings(event, "cyberarkpas.audit.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_str("cyberarkpas.audit.severity") == Some("Info") };
            if _cond {
            event.set("event.severity", json!(2))?;
            }

            let _cond = { event.get_str("cyberarkpas.audit.severity") == Some("Error") };
            if _cond {
            event.set("event.severity", json!(7))?;
            }

            let _cond = { event.get_str("cyberarkpas.audit.severity") == Some("Critical") };
            if _cond {
            event.set("event.severity", json!(10))?;
            }

            let _cond = { event.has_value("event.severity") && event.get_i64("event.severity").is_some_and(|n| n > 6) };
            if _cond {
            event.set("event.type", Value::Array(vec![json!("error")]))?;
            }

                if event.has_value("cyberarkpas.audit.message_id") {
                    event.rename("cyberarkpas.audit.message_id", "event.code")?;
                }

            let v = json!(event.get("cyberarkpas.audit.station").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("source.address", v)?;
            }

            let v = json!(event.get("cyberarkpas.audit.gateway_station").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("destination.address", v)?;
            }

            let _cond = { event.has_value("cyberarkpas.audit.file") };
            if _cond {
            event.set("file.path", json!(event.get("cyberarkpas.audit.file").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("cyberarkpas.audit.vendor") {
                    event.rename("cyberarkpas.audit.vendor", "observer.vendor")?;
                }

                if event.has_value("cyberarkpas.audit.product") {
                    event.rename("cyberarkpas.audit.product", "observer.product")?;
                }

                if event.has_value("cyberarkpas.audit.version") {
                    event.rename("cyberarkpas.audit.version", "observer.version")?;
                }

                if event.has_value("cyberarkpas.audit.hostname") {
                    event.rename("cyberarkpas.audit.hostname", "observer.hostname")?;
                }

            let _cond = { !event.has_value("observer.hostname") };
            if _cond {
                if event.has_value("_tmp.hostname") {
                    event.rename("_tmp.hostname", "observer.hostname")?;
                }
            }

                // Painless script
                // Source: def clone(def val) {\n    return val instanceof List ? new ArrayList(val) : val;\n} def read_field(def map, String name) {\n  if (map == null || !(map instanceof Map)) return null;\n  int pos = name.indexOf(\".\");\n  return pos == -1 ? map[name]\n                   : read_field(map[name.substring(0, pos)], name.substring(pos+1));\n} String msgID = ctx.event?.code; def actions = params.get(msgID); if (actions == null) return; List values = new ArrayList(); for (def item : actions) {\n  def val = item.value;\n  if (val == null && (val = read_field(ctx, item.from)) == null || val == \"\") continue;\n  values.add([\n    \"to\": item.set,\n    \"value\": clone(val)\n  ]);\n} if (!values.isEmpty()) ctx._tmp[\"values\"] = values;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"def clone(def val) {\n    return val instanceof List ? new ArrayList(val) : val;\n} def read_field(def map, String name) {\n  if (map == null || !(map instanceof Map)) return null;\n  int pos = name.indexOf(\".\");\n  return pos == -1 ? map[name]\n                   : read_field(map[name.substring(0, pos)], name.substring(pos+1));\n} String msgID = ctx.event?.code; def actions = params.get(msgID); if (actions == null) return; List values = new ArrayList(); for (def item : actions) {\n  def val = item.value;\n  if (val == null && (val = read_field(ctx, item.from)) == null || val == \"\") continue;\n  values.add([\n    \"to\": item.set,\n    \"value\": clone(val)\n  ]);\n} if (!values.isEmpty()) ctx._tmp[\"values\"] = values;\n"#), cached_params!("{\"4\":[{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"authentication\"]},{\"set\":\"event.type\",\"value\":[\"start\"]},{\"set\":\"event.action\",\"value\":\"authentication_failure\"},{\"set\":\"event.outcome\",\"value\":\"failure\"}],\"7\":[{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"authentication\",\"session\"]},{\"set\":\"event.type\",\"value\":[\"start\"]},{\"set\":\"event.action\",\"value\":\"authentication_success\"},{\"set\":\"event.outcome\",\"value\":\"success\"}],\"8\":[{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"authentication\",\"session\"]},{\"set\":\"event.type\",\"value\":[\"end\"]},{\"set\":\"event.outcome\",\"value\":\"success\"}],\"19\":[{\"set\":\"source.user.name\",\"from\":\"cyberarkpas.audit.source_user\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.source_user\"},{\"set\":\"destination.user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"network\"]},{\"set\":\"event.type\",\"value\":[\"start\"]},{\"set\":\"event.outcome\",\"value\":\"success\"}],\"22\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.ca_properties.address\"},{\"set\":\"event.outcome\",\"from\":\"cyberarkpas.audit.ca_properties.cpm_status\"},{\"set\":\"destination.user.name\",\"from\":\"cyberarkpas.audit.ca_properties.user_name\"},{\"set\":\"source.user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.type\",\"value\":[\"admin\",\"info\"]}],\"24\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.ca_properties.address\"},{\"set\":\"event.outcome\",\"from\":\"cyberarkpas.audit.ca_properties.cpm_status\"},{\"set\":\"user.target.name\",\"from\":\"cyberarkpas.audit.ca_properties.user_name\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.type\",\"value\":[\"user\",\"change\"]}],\"31\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.ca_properties.address\"},{\"set\":\"event.outcome\",\"from\":\"cyberarkpas.audit.ca_properties.cpm_status\"},{\"set\":\"user.target.name\",\"from\":\"cyberarkpas.audit.ca_properties.user_name\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.type\",\"value\":[\"user\",\"change\"]}],\"32\":[{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"user.target.name\",\"from\":\"cyberarkpas.audit.source_user\"},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.type\",\"value\":[\"admin\",\"change\"]},{\"set\":\"event.outcome\",\"value\":\"success\"}],\"33\":[{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"user.target.name\",\"from\":\"cyberarkpas.audit.source_user\"},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.type\",\"value\":[\"admin\",\"change\"]},{\"set\":\"event.outcome\",\"value\":\"success\"}],\"38\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.ca_properties.address\"},{\"set\":\"event.outcome\",\"value\":\"failure\"},{\"set\":\"event.reason\",\"from\":\"cyberarkpas.audit.ca_properties.cpm_error_details\"},{\"set\":\"destination.user.name\",\"from\":\"cyberarkpas.audit.ca_properties.user_name\"},{\"set\":\"source.user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.type\",\"value\":[\"admin\",\"info\"]}],\"57\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.ca_properties.address\"},{\"set\":\"event.outcome\",\"value\":\"failure\"},{\"set\":\"user.target.name\",\"from\":\"cyberarkpas.audit.ca_properties.user_name\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.type\",\"value\":[\"user\",\"change\"]},{\"set\":\"event.reason\",\"from\":\"cyberarkpas.audit.ca_properties.cpm_error_details\"}],\"60\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.ca_properties.address\"},{\"set\":\"event.outcome\",\"value\":\"failure\"},{\"set\":\"user.target.name\",\"from\":\"cyberarkpas.audit.ca_properties.user_name\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.type\",\"value\":[\"user\",\"change\"]},{\"set\":\"event.reason\",\"from\":\"cyberarkpas.audit.ca_properties.cpm_error_details\"}],\"130\":[{\"set\":\"event.outcome\",\"value\":\"failure\"},{\"set\":\"user.target.name\",\"from\":\"cyberarkpas.audit.ca_properties.user_name\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.type\",\"value\":[\"user\",\"change\"]},{\"set\":\"event.reason\",\"from\":\"cyberarkpas.audit.ca_properties.cpm_error_details\"},{\"set\":\"event.outcome\",\"from\":\"cyberarkpas.audit.ca_properties.cpm_status\"}],\"174\":[{\"set\":\"user.target.name\",\"from\":\"cyberarkpas.audit.source_user\"},{\"set\":\"event.type\",\"value\":[\"user\",\"change\"]},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.outcome\",\"value\":\"success\"}],\"175\":[{\"set\":\"user.target.name\",\"from\":\"cyberarkpas.audit.source_user\"},{\"set\":\"event.type\",\"value\":[\"user\",\"change\"]},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.outcome\",\"value\":\"success\"}],\"176\":[{\"set\":\"user.target.name\",\"from\":\"cyberarkpas.audit.source_user\"},{\"set\":\"event.type\",\"value\":[\"user\",\"deletion\"]},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.outcome\",\"value\":\"success\"}],\"177\":[{\"set\":\"user.target.name\",\"from\":\"cyberarkpas.audit.source_user\"},{\"set\":\"event.type\",\"value\":[\"user\",\"deletion\"]},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.outcome\",\"value\":\"success\"}],\"173\":[{\"set\":\"user.target.name\",\"from\":\"cyberarkpas.audit.source_user\"},{\"set\":\"event.type\",\"value\":[\"user\",\"creation\"]},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.outcome\",\"value\":\"success\"}],\"180\":[{\"set\":\"user.target.name\",\"from\":\"cyberarkpas.audit.source_user\"},{\"set\":\"event.type\",\"value\":[\"user\",\"creation\"]},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.outcome\",\"value\":\"success\"}],\"295\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.ca_properties.address\"},{\"set\":\"destination.user.name\",\"from\":\"cyberarkpas.audit.ca_properties.user_name\"},{\"set\":\"source.user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"iam\",\"authentication\"]},{\"set\":\"event.type\",\"value\":[\"admin\",\"start\"]},{\"set\":\"event.outcome\",\"value\":\"success\"},{\"set\":\"event.reason\",\"from\":\"cyberarkpas.audit.reason\"}],\"300\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.extra_details.dst_host\"},{\"set\":\"destination.user.name\",\"from\":\"cyberarkpas.audit.extra_details.user\"},{\"set\":\"source.address\",\"from\":\"cyberarkpas.audit.extra_details.src_host\"},{\"set\":\"source.user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"network.application\",\"from\":\"cyberarkpas.audit.extra_details.protocol\"},{\"set\":\"event.category\",\"value\":[\"session\"]},{\"set\":\"event.type\",\"value\":[\"start\"]},{\"set\":\"event.outcome\",\"value\":\"success\"}],\"302\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.extra_details.dst_host\"},{\"set\":\"destination.user.name\",\"from\":\"cyberarkpas.audit.extra_details.user\"},{\"set\":\"source.address\",\"from\":\"cyberarkpas.audit.extra_details.src_host\"},{\"set\":\"source.user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"network.application\",\"from\":\"cyberarkpas.audit.extra_details.protocol\"},{\"set\":\"_tmp.duration_hms\",\"from\":\"cyberarkpas.audit.extra_details.session_duration\"},{\"set\":\"event.category\",\"value\":[\"session\"]},{\"set\":\"event.type\",\"value\":[\"end\"]},{\"set\":\"event.outcome\",\"value\":\"success\"}],\"308\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.ca_properties.address\"},{\"set\":\"destination.user.name\",\"from\":\"cyberarkpas.audit.ca_properties.user_name\"},{\"set\":\"source.user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"iam\",\"authentication\"]},{\"set\":\"event.type\",\"value\":[\"admin\",\"start\"]},{\"set\":\"event.outcome\",\"from\":\"cyberarkpas.audit.ca_properties.cpm_status\"},{\"set\":\"event.reason\",\"from\":\"cyberarkpas.audit.reason\"}],\"309\":[{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"authentication\"]},{\"set\":\"event.type\",\"value\":[\"start\"]},{\"set\":\"event.action\",\"value\":\"authentication_failure\"},{\"set\":\"event.outcome\",\"value\":\"failure\"}],\"361\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.extra_details.dst_host\"},{\"set\":\"destination.user.name\",\"from\":\"cyberarkpas.audit.extra_details.user\"},{\"set\":\"source.address\",\"from\":\"cyberarkpas.audit.extra_details.src_host\"},{\"set\":\"source.user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"network.application\",\"from\":\"cyberarkpas.audit.extra_details.protocol\"},{\"set\":\"event.category\",\"value\":[\"session\"]},{\"set\":\"event.type\",\"value\":[\"info\"]}],\"412\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.extra_details.dst_host\"},{\"set\":\"destination.user.name\",\"from\":\"cyberarkpas.audit.extra_details.user\"},{\"set\":\"source.address\",\"from\":\"cyberarkpas.audit.extra_details.src_host\"},{\"set\":\"source.user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"network.application\",\"from\":\"cyberarkpas.audit.extra_details.protocol\"},{\"set\":\"event.category\",\"value\":[\"session\"]},{\"set\":\"event.type\",\"value\":[\"info\"]}],\"359\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.extra_details.dst_host\"},{\"set\":\"destination.user.name\",\"from\":\"cyberarkpas.audit.extra_details.user\"},{\"set\":\"source.address\",\"from\":\"cyberarkpas.audit.extra_details.src_host\"},{\"set\":\"source.user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"network.application\",\"from\":\"cyberarkpas.audit.extra_details.protocol\"},{\"set\":\"event.category\",\"value\":[\"database\"]},{\"set\":\"event.type\",\"value\":[\"access\"]},{\"set\":\"event.outcome\",\"from\":\"cyberarkpas.audit.ca_properties.cpm_status\"}],\"411\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.extra_details.dst_host\"},{\"set\":\"destination.user.name\",\"from\":\"cyberarkpas.audit.extra_details.user\"},{\"set\":\"source.address\",\"from\":\"cyberarkpas.audit.extra_details.src_host\"},{\"set\":\"source.user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"network.application\",\"from\":\"cyberarkpas.audit.extra_details.protocol\"},{\"set\":\"process.pid\",\"from\":\"cyberarkpas.audit.extra_details.process_id\"},{\"set\":\"process.name\",\"from\":\"cyberarkpas.audit.extra_details.process_name\"},{\"set\":\"event.category\",\"value\":[\"process\"]},{\"set\":\"event.type\",\"value\":[\"access\",\"info\"]}],\"414\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.ca_properties.address\"},{\"set\":\"event.outcome\",\"from\":\"cyberarkpas.audit.ca_properties.cpm_status\"},{\"set\":\"destination.user.name\",\"from\":\"cyberarkpas.audit.ca_properties.user_name\"},{\"set\":\"source.user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"iam\"]},{\"set\":\"event.type\",\"value\":[\"admin\",\"info\"]}],\"428\":[{\"set\":\"destination.address\",\"from\":\"cyberarkpas.audit.ca_properties.address\"},{\"set\":\"destination.user.name\",\"from\":\"cyberarkpas.audit.ca_properties.user_name\"},{\"set\":\"source.user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"user.name\",\"from\":\"cyberarkpas.audit.issuer\"},{\"set\":\"event.category\",\"value\":[\"iam\",\"authentication\"]},{\"set\":\"event.type\",\"value\":[\"admin\",\"start\"]},{\"set\":\"event.outcome\",\"value\":\"success\"},{\"set\":\"event.reason\",\"from\":\"cyberarkpas.audit.reason\"}]}"))?;

            if event.has_value("_tmp.values") {
                foreach_array(event, "_tmp.values", |event| {
                    if let Some(v) = event.get("_ingest._value.value").filter(|v| !painless_is_empty_value(v)).cloned() {
                    event.set("{{{_ingest._value.to}}}", v)?;
                    }
                    Ok(())
                })?;
            }

            let _cond = { event.has_value("event.outcome") && !(["success", "failure"].contains(&event.get_str("event.outcome").unwrap_or(""))) };
            if _cond {
            event.set("event.outcome", json!("unknown"))?;
            }

            let _cond = { event.has_value("_tmp.duration_hms") };
            if _cond {
                // Painless script
                // Source: long parse_hms(String s) {\n    long cur = 0, total = 0;\n    for (int i = 0, n = s.length(); i < n; i++) {\n        char c = s.charAt(i);\n        if (c >= (char)'0' && c <= (char)'9') {\n            cur = (cur*10) + (long)(c - (char)'0');\n        } else if (c == (char)':') {\n            total = (total + cur) * 60;\n            cur = 0;\n        } else {\n            return 0;\n        }\n    }\n    return total + cur;\n} long nanos = parse_hms(ctx._tmp.duration_hms) * 1000000000L; ctx.event['duration'] = nanos;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"long parse_hms(String s) {\n    long cur = 0, total = 0;\n    for (int i = 0, n = s.length(); i < n; i++) {\n        char c = s.charAt(i);\n        if (c >= (char)'0' && c <= (char)'9') {\n            cur = (cur*10) + (long)(c - (char)'0');\n        } else if (c == (char)':') {\n            total = (total + cur) * 60;\n            cur = 0;\n        } else {\n            return 0;\n        }\n    }\n    return total + cur;\n} long nanos = parse_hms(ctx._tmp.duration_hms) * 1000000000L; ctx.event['duration'] = nanos;\n"#))?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("source.address") {
                if let Some(val) = event.get("source.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_address")?;
                    if let Some(v) = event.get("source.address").cloned() {
                        event.set("source.domain", v)?;
                    }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("destination.address") {
                if let Some(val) = event.get("destination.address") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "destination.address".into(),
                            message,
                        })?;
                    event.set("destination.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_destination_address")?;
                    if let Some(v) = event.get("destination.address").cloned() {
                        event.set("destination.domain", v)?;
                    }
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("destination.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("cyberarkpas.audit.station") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("cyberarkpas.audit.station").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("cyberarkpas.audit.gateway_station") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("cyberarkpas.audit.gateway_station").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("source.user.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("destination.user.name").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("user.target.name") };
            if _cond {
                event.append_unique("related.user", json!(event.get("user.target.name").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("network.application") {
                map_strings(event, "network.application", "network.application", str::to_lowercase)?;
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

            let _cond = { !event.has_value("host.name") };
            if _cond {
            let v = json!(event.get("observer.hostname").map_or_else(String::new, template_to_string));
            if !painless_is_empty_value(&v) {
                    event.set("host.name", v)?;
            }
            }

            if event.has_value("source.ip") {
                // Classify network direction against the internal network ranges
                if let (Some(src), Some(dst)) = (event.get_string("source.ip"), event.get_string("destination.ip")) {
                    let networks: Vec<&str> = vec!["loopback", "private", "unspecified"];
                    let direction = match (ip_in_networks(&src, &networks), ip_in_networks(&dst, &networks)) {
                        (true, false) => "outbound",
                        (false, true) => "inbound",
                        (true, true) => "internal",
                        (false, false) => "external",
                    };
                    event.set("network.direction", json!(direction))?;
                }
            }

            if event.has_value("process.pid") {
                if let Some(val) = event.get("process.pid") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "process.pid".into(),
                            message,
                        })?;
                    event.set("process.pid", converted)?;
                }
            }

                // Painless script
                // Source: Map audit = ctx.cyberarkpas.audit; params.entrySet().stream().filter(e -> audit.containsKey(e.getKey())).forEach(lst -> {\n  Map base = audit[lst.getKey()],\n      selected = new HashMap();\n  lst.getValue().stream().filter(fld -> base.containsKey(fld)).forEach(fld -> {\n    selected[fld] = base.remove(fld);\n  });\n  selected['other'] = base;\n  audit[lst.getKey()] = selected;\n});\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(event, cached_painless!(r#"Map audit = ctx.cyberarkpas.audit; params.entrySet().stream().filter(e -> audit.containsKey(e.getKey())).forEach(lst -> {\n  Map base = audit[lst.getKey()],\n      selected = new HashMap();\n  lst.getValue().stream().filter(fld -> base.containsKey(fld)).forEach(fld -> {\n    selected[fld] = base.remove(fld);\n  });\n  selected['other'] = base;\n  audit[lst.getKey()] = selected;\n});\n"#), cached_params!("{\"ca_properties\":[\"address\",\"cpm_disabled\",\"cpm_error_details\",\"cpm_status\",\"creation_method\",\"customer\",\"database\",\"device_type\",\"dual_account_status\",\"group_name\",\"in_process\",\"index\",\"last_fail_date\",\"last_success_change\",\"last_success_reconciliation\",\"last_success_verification\",\"last_task\",\"logon_domain\",\"policy_id\",\"port\",\"privcloud\",\"reset_immediately\",\"retries_count\",\"sequence_id\",\"tags\",\"user_dn\",\"user_name\",\"virtual_username\"],\"extra_details\":[\"ad_process_id\",\"ad_process_name\",\"application_type\",\"command\",\"connection_component_id\",\"dst_host\",\"logon_account\",\"managed_account\",\"process_id\",\"process_name\",\"protocol\",\"psmid\",\"session_duration\",\"session_id\",\"src_host\",\"username\"]}"))?;

                event.remove("_tmp");

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                    event.append("error.message", json!(format!("Processor '{}' {}failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), if event.get("_ingest.on_failure_processor_tag").is_some_and(|v| !v.is_null() && v.as_str() != Some("") && !matches!(v, Value::Bool(false)) && !v.as_array().is_some_and(Vec::is_empty)) { format!("with tag '{}' ", event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string)) } else { String::new() }, event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_tmp");
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
