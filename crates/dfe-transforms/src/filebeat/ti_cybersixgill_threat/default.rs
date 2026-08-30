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

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

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

            parse_json_field(event, "event.original", "cybersixgill")?;

            event.remove("cybersixgill.extensions");

            let _cond = { event.get_str("cybersixgill.type") != Some("indicator") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = { event.has_value("cybersixgill.created") };
            if _cond {
                if let Some(date_str) = event.get_as_string("cybersixgill.created") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:mm:ssz",
                            "yyyy-MM-dd'T'HH:mm:ssZ",
                            "yyyy-MM-dd'T'HH:mm:ss.Sz",
                            "yyyy-MM-dd'T'HH:mm:ss.SZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSz",
                            "yyyy-MM-dd'T'HH:mm:ss.SSZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSz",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cybersixgill.created".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("cybersixgill.modified") };
            if _cond {
                if let Some(date_str) = event.get_as_string("cybersixgill.modified") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:mm:ssz",
                            "yyyy-MM-dd'T'HH:mm:ssZ",
                            "yyyy-MM-dd'T'HH:mm:ss.Sz",
                            "yyyy-MM-dd'T'HH:mm:ss.SZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSz",
                            "yyyy-MM-dd'T'HH:mm:ss.SSZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSz",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("threat.indicator.last_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cybersixgill.modified".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("cybersixgill.valid_from") };
            if _cond {
                if let Some(date_str) = event.get_as_string("cybersixgill.valid_from") {
                    match parse_date_out(
                        &date_str,
                        &[
                            "yyyy-MM-dd'T'HH:mm:ssz",
                            "yyyy-MM-dd'T'HH:mm:ssZ",
                            "yyyy-MM-dd'T'HH:mm:ss.Sz",
                            "yyyy-MM-dd'T'HH:mm:ss.SZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSz",
                            "yyyy-MM-dd'T'HH:mm:ss.SSZ",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSz",
                            "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                        ],
                        None,
                        None,
                    ) {
                        Some(parsed) => event.set("threat.indicator.first_seen", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cybersixgill.valid_from".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("cybersixgill.created") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("cybersixgill.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("cybersixgill.modified") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("cybersixgill.pattern") };
            if _cond {
                if let Some(input) = event.get_string("cybersixgill.pattern") {
                    // Grok pattern: ^\\[(?:%{DATA:_temp_.type}:hashes.MD5%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.md5}')\\]
                    // Grok pattern: ^\\[(?:%{DATA:_temp_.type}:hashes.MD5%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.md5}') OR (?:%{DATA:_temp_.type}:hashes.'SHA-1'%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.sha1}')\\]
                    // Grok pattern: ^\\[(?:%{DATA:_temp_.type}:hashes.MD5%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.md5}') OR (?:%{DATA:_temp_.type}:hashes.'SHA-1'%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.sha1}') OR (?:%{DATA:_temp_.type}:hashes.'SHA-256'%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.sha256}')\\]
                    // Grok pattern: ^\\[(?:%{DATA:_temp_.type}:hashes.'SHA-1'%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.sha1}')\\]
                    // Grok pattern: ^\\[(?:%{DATA:_temp_.type}:hashes.'SHA-1'%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.sha1}') OR (?:%{DATA:_temp_.type}:hashes.'SHA-256'%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.sha256}')\\]
                    // Grok pattern: ^\\[(?:%{DATA:_temp_.type}:hashes.'SHA-256'%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.sha256}')\\]
                    // Grok pattern: ^\\[%{DATA:_temp_.type}:value%{SPACE}=%{SPACE}'%{DATA:_temp_.threatvalue}'\\]
                    let _ = extract_first_match(
                        &[
                            cached_grok!(
                                "^\\[(?:%{DATA:_temp_.type}:hashes.MD5%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.md5}')\\]"
                            ),
                            cached_grok!(
                                "^\\[(?:%{DATA:_temp_.type}:hashes.MD5%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.md5}') OR (?:%{DATA:_temp_.type}:hashes.'SHA-1'%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.sha1}')\\]"
                            ),
                            cached_grok!(
                                "^\\[(?:%{DATA:_temp_.type}:hashes.MD5%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.md5}') OR (?:%{DATA:_temp_.type}:hashes.'SHA-1'%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.sha1}') OR (?:%{DATA:_temp_.type}:hashes.'SHA-256'%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.sha256}')\\]"
                            ),
                            cached_grok!(
                                "^\\[(?:%{DATA:_temp_.type}:hashes.'SHA-1'%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.sha1}')\\]"
                            ),
                            cached_grok!(
                                "^\\[(?:%{DATA:_temp_.type}:hashes.'SHA-1'%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.sha1}') OR (?:%{DATA:_temp_.type}:hashes.'SHA-256'%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.sha256}')\\]"
                            ),
                            cached_grok!(
                                "^\\[(?:%{DATA:_temp_.type}:hashes.'SHA-256'%{SPACE}=%{SPACE}'%{WORD:threat.indicator.file.hash.sha256}')\\]"
                            ),
                            cached_grok!(
                                "^\\[%{DATA:_temp_.type}:value%{SPACE}=%{SPACE}'%{DATA:_temp_.threatvalue}'\\]"
                            ),
                        ],
                        &input,
                        event,
                    )?;
                }
            }

            let _cond = {
                event.has_value("_temp_.type")
                    && event.get("_temp_.type").is_some_and(|v| v.is_string())
            };
            if _cond {
                if let Some(v) = event.get("_temp_.type").cloned() {
                    event.set("threat.indicator.type", v)?;
                }
            }

            let _cond = {
                event.has_value("_temp_.type")
                    && event.get("_temp_.type").is_some_and(|v| v.is_array())
            };
            if _cond {
                if event.has_value("_temp_.type") {
                    foreach_array(event, "_temp_.type", |event| {
                        if !event.has("threat.indicator.type") {
                            event.set(
                                "threat.indicator.type",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                        }
                        Ok(())
                    })?;
                }
            }

            let v = json!(
                event
                    .get("_temp_.threatvalue")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("threat.indicator.name", v)?;
            }

            if let Some(v) = event
                .get("threat.indicator.file.hash.md5")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            if let Some(v) = event
                .get("threat.indicator.file.hash.sha1")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            if let Some(v) = event
                .get("threat.indicator.file.hash.sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.name", v)?;
            }

            let _cond = {
                ["ipv4-addr", "ipv6-addr"]
                    .contains(&event.get_str("threat.indicator.type").unwrap_or(""))
            };
            if _cond {
                if event.has_value("_temp_.threatvalue") {
                    event.rename("_temp_.threatvalue", "threat.indicator.ip")?;
                }
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("url") };
            if _cond {
                uri_parts(
                    event,
                    "_temp_.threatvalue",
                    "threat.indicator.url",
                    true,
                    true,
                )?;
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("email-addr") };
            if _cond {
                if event.has_value("_temp_.threatvalue") {
                    event.rename("_temp_.threatvalue", "threat.indicator.email.address")?;
                }
            }

            let _cond = { event.get_str("threat.indicator.type") == Some("domain-name") };
            if _cond {
                if event.has_value("_temp_.threatvalue") {
                    event.rename("_temp_.threatvalue", "threat.indicator.url.domain")?;
                }
            }

            let _cond = { !event.has_value("threat.indicator.type") };
            if _cond {
                event.set("threat.indicator.type", json!("unknown"))?;
            }

            if event.has_value("cybersixgill.labels") {
                event.rename("cybersixgill.labels", "_temp_.tags")?;
            }

            if event.has_value("cybersixgill.sixgill_severity") {
                event.rename("cybersixgill.sixgill_severity", "event.severity")?;
            }

            if event.has_value("cybersixgill.description") {
                event.rename("cybersixgill.description", "threat.indicator.description")?;
            }

            if event.has_value("cybersixgill.sixgill_feedname") {
                event.rename("cybersixgill.sixgill_feedname", "cybersixgill.feedname")?;
            }

            if event.has_value("cybersixgill.sixgill_source") {
                event.rename("cybersixgill.sixgill_source", "threat.indicator.provider")?;
            }

            if event.has_value("cybersixgill.sixgill_posttitle") {
                event.rename("cybersixgill.sixgill_posttitle", "cybersixgill.title")?;
            }

            if event.has_value("cybersixgill.sixgill_actor") {
                event.rename("cybersixgill.sixgill_actor", "cybersixgill.actor")?;
            }

            let _cond = { event.has_value("cybersixgill.sixgill_postid") };
            if _cond {
                event.set(
                    "threat.indicator.reference",
                    json!(format!(
                        "https://portal.cybersixgill.com/#/search?q=_id:{}",
                        event
                            .get("cybersixgill.sixgill_postid")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            if event.has_value("cybersixgill.sixgill_confidence") {
                if let Some(val) = event.get("cybersixgill.sixgill_confidence") {
                    let converted = convert_value(val, "integer").map_err(|message| {
                        TransformError::ParseError {
                            path: "cybersixgill.sixgill_confidence".into(),
                            message,
                        }
                    })?;
                    event.set("threat.indicator.confidence", converted)?;
                }
            }

            let _cond = {
                !event.has_value("cybersixgill.deleted_at")
                    && event.has_value("_conf.ioc_expiration_duration")
                    && event.get_str("_conf.ioc_expiration_duration") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: def dur = ctx._conf.ioc_expiration_duration; ZonedDateTime _tmp_deleted_at; ZonedDateTime _tmp_created_at = ZonedDateTime.parse(ctx.cybersixgill.created); if (dur instanceof String){\n  String time_unit = dur.substring(dur.length() -  1, dur.length());\n  String time_value = dur.substring(0, dur.length() - 1);\n  if (time_unit == 'd') {\n    _tmp_deleted_at = _tmp_created_at.plusDays(Long.parseLong(time_value));\n  } else if (time_unit == 'h') {\n    _tmp_deleted_at = _tmp_created_at.plusHours(Long.parseLong(time_value));\n  } else if (time_unit == 'm') {\n    _tmp_deleted_at = _tmp_created_at.plusMinutes(Long.parseLong(time_value));\n  } else {\n    _tmp_deleted_at = _tmp_created_at.plusDays(90L);\n    if (ctx.error == null) {\n      ctx.error = new HashMap();\n    }\n    if (ctx.error.message == null) {\n      ctx.error.message = new ArrayList();\n    }\n    ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n  }\n  ctx.cybersixgill.deleted_at = _tmp_deleted_at;\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def dur = ctx._conf.ioc_expiration_duration; ZonedDateTime _tmp_deleted_at; ZonedDateTime _tmp_created_at = ZonedDateTime.parse(ctx.cybersixgill.created); if (dur instanceof String){\n  String time_unit = dur.substring(dur.length() -  1, dur.length());\n  String time_value = dur.substring(0, dur.length() - 1);\n  if (time_unit == 'd') {\n    _tmp_deleted_at = _tmp_created_at.plusDays(Long.parseLong(time_value));\n  } else if (time_unit == 'h') {\n    _tmp_deleted_at = _tmp_created_at.plusHours(Long.parseLong(time_value));\n  } else if (time_unit == 'm') {\n    _tmp_deleted_at = _tmp_created_at.plusMinutes(Long.parseLong(time_value));\n  } else {\n    _tmp_deleted_at = _tmp_created_at.plusDays(90L);\n    if (ctx.error == null) {\n      ctx.error = new HashMap();\n    }\n    if (ctx.error.message == null) {\n      ctx.error.message = new ArrayList();\n    }\n    ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n  }\n  ctx.cybersixgill.deleted_at = _tmp_deleted_at;\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script-default-deleted_at",
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("cybersixgill.deleted_at") {
                    match parse_date_out(&date_str, &["ISO8601", "UNIX"], None, None) {
                        Some(parsed) => event.set("cybersixgill.deleted_at", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "cybersixgill.deleted_at".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "date")?;
                event.set("_ingest.on_failure_processor_tag", "date_deleted_at")?;
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
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: (format!(
                        "Processor {} with tag {} in pipeline {} failed",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_pipeline")
                            .map_or_else(String::new, template_to_string)
                    ))
                    .to_string(),
                });
            }

            if event.has_value("_conf.ioc_expiration_duration") {
                event.rename(
                    "_conf.ioc_expiration_duration",
                    "cybersixgill.expiration_duration",
                )?;
            }

            let _cond = { event.has_value("cybersixgill.external_references") };
            if _cond {
                // Painless script
                // Source: def refs = ctx.cybersixgill.external_references; ctx.cybersixgill.mitre = new HashMap(); ctx.cybersixgill.virustotal = new HashMap(); ctx.threat.tactic = new HashMap(); for (def ref : refs) {\n  if (ref?.description != null) {\n    ctx.cybersixgill.mitre.description = ref.description;\n  }\n  if (ref?.mitre_attack_tactic != null) {\n    ctx.threat.tactic.name = [ref.mitre_attack_tactic];\n  }\n  if (ref?.mitre_attack_tactic_id != null) {\n    ctx.threat.tactic.id = [ref.mitre_attack_tactic_id];\n  }\n  if (ref?.mitre_attack_tactic_url != null) {\n    ctx.threat.tactic.reference = [ref.mitre_attack_tactic_url];\n  }\n  if (ref?.positive_rate != null) {\n    ctx.cybersixgill.virustotal.pr = ref.positive_rate;\n  }\n  if (ref?.url != null) {\n    ctx.cybersixgill.virustotal.url = ref.url;\n  }    \n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def refs = ctx.cybersixgill.external_references; ctx.cybersixgill.mitre = new HashMap(); ctx.cybersixgill.virustotal = new HashMap(); ctx.threat.tactic = new HashMap(); for (def ref : refs) {\n  if (ref?.description != null) {\n    ctx.cybersixgill.mitre.description = ref.description;\n  }\n  if (ref?.mitre_attack_tactic != null) {\n    ctx.threat.tactic.name = [ref.mitre_attack_tactic];\n  }\n  if (ref?.mitre_attack_tactic_id != null) {\n    ctx.threat.tactic.id = [ref.mitre_attack_tactic_id];\n  }\n  if (ref?.mitre_attack_tactic_url != null) {\n    ctx.threat.tactic.reference = [ref.mitre_attack_tactic_url];\n  }\n  if (ref?.positive_rate != null) {\n    ctx.cybersixgill.virustotal.pr = ref.positive_rate;\n  }\n  if (ref?.url != null) {\n    ctx.cybersixgill.virustotal.url = ref.url;\n  }    \n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("_temp_.tags") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("_temp_.tags") {
                        foreach_array(event, "_temp_.tags", |event| {
                            event.append(
                                "tags",
                                json!(
                                    event
                                        .get("_ingest._value")
                                        .map_or_else(String::new, template_to_string)
                                ),
                            )?;
                            Ok(())
                        })?;
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("threat.indicator.confidence") };
            if _cond {
                // Painless script
                // Source: def value = ctx.threat.indicator.confidence; if (value <= 0.0 || value > 100.0) {\n  ctx.threat.indicator.confidence = \"None\";\n  return;\n} if (value >= 1.0 && value <= 29.0) {\n  ctx.threat.indicator.confidence = \"Low\";\n  return;\n} if (value >= 30.0 && value <= 69.0) {\n  ctx.threat.indicator.confidence = \"Med\";\n  return;\n} if (value >= 70 && value <= 100) {\n  ctx.threat.indicator.confidence = \"High\";\n  return;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def value = ctx.threat.indicator.confidence; if (value <= 0.0 || value > 100.0) {\n  ctx.threat.indicator.confidence = \"None\";\n  return;\n} if (value >= 1.0 && value <= 29.0) {\n  ctx.threat.indicator.confidence = \"Low\";\n  return;\n} if (value >= 30.0 && value <= 69.0) {\n  ctx.threat.indicator.confidence = \"Med\";\n  return;\n} if (value >= 70 && value <= 100) {\n  ctx.threat.indicator.confidence = \"High\";\n  return;\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("threatintel") };
            if _cond {
                // Painless script
                // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\nmap.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\nmap.values().removeIf(v -> v == null);\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n}\nhandleMap(ctx);\n"#
                    ),
                )?;
            }

            event.remove("_temp_");
            event.remove("cybersixgill.sixgill_postid");
            event.remove("cybersixgill.extensions");
            event.remove("cybersixgill.spec_version");
            event.remove("cybersixgill.valid_from");
            event.remove("cybersixgill.created");
            event.remove("cybersixgill.modified");
            event.remove("cybersixgill.lang");
            event.remove("cybersixgill.name");
            event.remove("cybersixgill.pattern_type");
            event.remove("cybersixgill.external_references");
            event.remove("cybersixgill.confidence");
            event.remove("cybersixgill.sixgill_confidence");
            event.remove("cybersixgill.id");
            event.remove("cybersixgill.indicator_types");
            event.remove("cybersixgill.pattern");
            event.remove("cybersixgill.sixgill_feedid");
            event.remove("cybersixgill.sixgill_post_virustotallink");
            event.remove("cybersixgill.type");
            event.remove("_conf");

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
