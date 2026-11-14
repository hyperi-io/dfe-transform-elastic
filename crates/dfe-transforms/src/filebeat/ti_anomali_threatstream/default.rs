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
            event.set("ecs.version", json!("9.3.0"))?;

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

            let _cond = { event.has_value("json.itype") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan_params(
                        event,
                        compiled_painless!(
                            "c8df5cdff3eb2ec78bf098002b834baba27a16f9fdd5cc57f214aebab8928faf",
                            r#"{"params":{"NormalisedLookup":{"source":"json.itype","target":"threatintel_indicator_type","fold":"None"}},"known":[]}"#
                        ),
                        cached_params!(
                            "{\"actor_ip\":\"ipv4-addr\",\"adware_domain\":\"domain-name\",\"anon_proxy\":\"ipv4-addr\",\"anon_vpn\":\"ipv4-addr\",\"apt_domain\":\"domain-name\",\"apt_email\":\"email-addr\",\"apt_ip\":\"ipv4-addr\",\"apt_md5\":\"file\",\"apt_subject\":\"email\",\"apt_ua\":\"url\",\"apt_url\":\"url\",\"bot_ip\":\"ipv4-addr\",\"brute_ip\":\"ipv4-addr\",\"c2_domain\":\"domain-name\",\"c2_ip\":\"ipv4-addr\",\"c2_url\":\"url\",\"comm_proxy_domain\":\"domain-name\",\"comm_proxy_ip\":\"ipv4-addr\",\"compromised_domain\":\"domain-name\",\"compromised_ip\":\"ipv4-addr\",\"compromised_url\":\"url\",\"crypto_hash\":\"file\",\"crypto_ip\":\"ipv4-addr\",\"crypto_pool\":\"domain\",\"crypto_url\":\"url\",\"crypto_wallet\":\"file\",\"ddos_ip\":\"ipv4-addr\",\"disposable_email_domain\":\"domain-name\",\"dyn_dns\":\"domain-name\",\"exfil_domain\":\"domain-name\",\"exfil_ip\":\"ipv4-addr\",\"exfil_url\":\"url\",\"exploit_domain\":\"domain-name\",\"exploit_ip\":\"ipv4-addr\",\"exploit_url\":\"url\",\"free_email_domain\":\"domain-name\",\"geolocation_url\":\"url\",\"hack_tool\":\"file\",\"i2p_ip\":\"ipv4-addr\",\"ipcheck_url\":\"url\",\"mal_domain\":\"domain-name\",\"mal_email\":\"email-addr\",\"mal_ip\":\"ipv4-addr\",\"mal_md5\":\"file\",\"mal_sslcert_sh1\":\"x509-certificate\",\"mal_sslcert_sha1\":\"x509-certificate\",\"mal_ua\":\"url\",\"mal_url\":\"url\",\"p2pcnc\":\"ipv4-addr\",\"parked_domain\":\"domain-name\",\"parked_ip\":\"ipv4-addr\",\"parked_url\":\"url\",\"pastesite_url\":\"url\",\"phish_domain\":\"domain-name\",\"phish_email\":\"email-addr\",\"phish_ip\":\"ipv4-addr\",\"phish_url\":\"url\",\"proxy_ip\":\"ipv4-addr\",\"scan_ip\":\"ipv4-addr\",\"sinkhole_domain\":\"domain-name\",\"sinkhole_ip\":\"ipv4-addr\",\"spam_domain\":\"domain-name\",\"spam_email\":\"email-addr\",\"spam_ip\":\"ipv4-addr\",\"spam_url\":\"url\",\"speedtest_url\":\"url\",\"ssh_ip\":\"ipv4-addr\",\"suppress\":\"suppress\",\"suspicious_domain\":\"domain-name\",\"suspicious_email\":\"email-addr\",\"suspicious_ip\":\"ipv4-addr\",\"suspicious_reg_email\":\"email-addr\",\"suspicious_url\":\"url\",\"tor_ip\":\"ipv4-addr\",\"torrent_tracker_url\":\"url\",\"vpn_domain\":\"domain-name\",\"vps_ip\":\"ipv4-addr\",\"whois_bulk_reg_email\":\"email-addr\",\"whois_privacy_domain\":\"domain-name\",\"whois_privacy_email\":\"email-addr\"}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_0bfa0bbb")?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Unable to determine indicator type from \"{}\": {}",
                            event
                                .get("json.itype")
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

            if event.has_value("threatintel_indicator_type") {
                event.rename("threatintel_indicator_type", "threat.indicator.type")?;
            }

            let _cond = { event.has_value("threat.indicator.type") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "3470538df8ed6f2d72743d0e962c0354109b54e011cc27542afe1f16020664d1",
                            r#"{"params":null,"known":[{"CopyByLabel":{"subject":"threat.indicator.type","arms":[[["ipv4-addr","ipv6-addr"],"json.srcip"],[["domain-name"],"json.domain"],[["url"],"json.url"],[["email-addr","email"],"json.email"],[["file"],"json.md5"]],"target":"threat.indicator.name"}}]}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_threat_indicator_name",
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

            let _cond = {
                event.get_str("threat.indicator.type") == Some("ipv4-addr")
                    && event.has_value("json.srcip")
                    && event.get("json.srcip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("threat.indicator.type", json!("ipv6-addr"))?;
            }

            let _cond = {
                !event.has_value("json.deleted_at")
                    && event.has_value("json.added_at")
                    && event.has_value("_conf.ioc_expiration_duration")
                    && event.get_str("_conf.ioc_expiration_duration") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "e5db9eb3633b4753703de6805af81b4f5781cf6585f09ffffa5741e5b1c74eb3",
                            r#"{"params":null,"known":["JoinOptional"]}"#,
                            r#"def dur = ctx._conf.ioc_expiration_duration; String added_at = ctx.json.added_at; ZonedDateTime _tmp_deleted_at; if (dur instanceof String){\n  char time_unit;\n  String time_value;\n  if (dur.length() != 0){\n    time_unit = dur.charAt(dur.length() - 1);\n    time_value = dur.substring(0, dur.length() - 1);\n  }\n  if (time_unit == (char)'d') {\n    _tmp_deleted_at = ZonedDateTime.parse(added_at, DateTimeFormatter.ISO_LOCAL_DATE_TIME.withZone(ZoneId.of('Z'))).plusDays(Long.parseLong(time_value));\n  } else if (time_unit == (char)'h') {\n    _tmp_deleted_at = ZonedDateTime.parse(added_at, DateTimeFormatter.ISO_LOCAL_DATE_TIME.withZone(ZoneId.of('Z'))).plusHours(Long.parseLong(time_value));\n  } else if (time_unit == (char)'m') {\n    _tmp_deleted_at = ZonedDateTime.parse(added_at, DateTimeFormatter.ISO_LOCAL_DATE_TIME.withZone(ZoneId.of('Z'))).plusMinutes(Long.parseLong(time_value));\n  } else {\n    if (ctx.error == null) {\n      ctx.error = new HashMap();\n    }\n    if (ctx.error.message == null) {\n      ctx.error.message = new ArrayList();\n    }\n    ctx.error.message.add('invalid ioc_expiration_duration: using default 90 days');\n    _tmp_deleted_at = ZonedDateTime.parse(added_at, DateTimeFormatter.ISO_LOCAL_DATE_TIME.withZone(ZoneId.of('Z'))).plusDays(90L);\n  }\n  ctx.json.deleted_at = _tmp_deleted_at;\n}\n"#
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

            let _cond = { event.has_value("json.added_at") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.added_at") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "strict_date_optional_time_nanos"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("json.added_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.added_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date-added_at")?;
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

            let _cond = { event.has_value("json.deleted_at") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.deleted_at") {
                        match parse_date_out(
                            &date_str,
                            &["ISO8601", "strict_date_optional_time_nanos"],
                            None,
                            None,
                        ) {
                            Some(parsed) => event.set("json.deleted_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.deleted_at".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date-deleted_at")?;
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

            let _cond = { event.has_value("json.date_first") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.date_first") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("threat.indicator.first_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.date_first".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date-date_first")?;
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

            let _cond = { event.has_value("json.date_last") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.date_last") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("threat.indicator.last_seen", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.date_last".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date-last_seen")?;
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

            let _cond = { event.has_value("json.added_at") };
            if _cond {
                if let Some(v) = event.get("json.added_at").cloned() {
                    event.set("_temp_.timestamp", v)?;
                }
            }

            let _cond =
                { event.has_value("json.deleted_at") && !event.has_value("_temp_.timestamp") };
            if _cond {
                if let Some(v) = event.get("json.deleted_at").cloned() {
                    event.set("_temp_.timestamp", v)?;
                }
            }

            let _cond = { event.has_value("_temp_.timestamp") };
            if _cond {
                if let Some(v) = event.get("_temp_.timestamp").cloned() {
                    event.set("@timestamp", v)?;
                }
            }

            let _cond = { event.has_value("json.lat") && event.has_value("json.lon") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("json.lat") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.lat".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.geo.location.lat", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_lat_to_threat_indicator_geo_location_lat_561d51e8",
                    )?;
                    event.append("error.message", json!(format!("Cannot convert lat field \"{}\" to double: Processor {} with tag {} in pipeline {} failed with message: {}", event.get("json.lat").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("json.lat") && event.has_value("json.lon") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("json.lon") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.lon".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.geo.location.lon", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_lon_to_threat_indicator_geo_location_lon_9e85d988",
                    )?;
                    event.append("error.message", json!(format!("Cannot convert lon field \"{}\" to double: Processor {} with tag {} in pipeline {} failed with message: {}", event.get("json.lon").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.get_str("json.classification") == Some("private") };
            if _cond {
                event.append("threat.indicator.marking.tlp", json!("AMBER"))?;
            }

            let _cond = { event.get_str("json.classification") == Some("public") };
            if _cond {
                event.append("threat.indicator.marking.tlp", json!("WHITE"))?;
            }

            let _cond = { event.has_value("json.confidence") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "a581da9c0ec8d047e25f6b8164c25018b812428a1d65b737cd3566d516f91a6d",
                            r#"{"params":null,"known":[{"BandLadder":{"subject":"json.confidence","arms":[[{"bounds":[{"op":"Le","scaled":0},{"op":"Gt","scaled":100000}],"all":false},{"Text":"None"}],[{"bounds":[{"op":"Ge","scaled":1000},{"op":"Le","scaled":29000}],"all":true},{"Text":"Low"}],[{"bounds":[{"op":"Ge","scaled":30000},{"op":"Le","scaled":69000}],"all":true},{"Text":"Medium"}],[{"bounds":[{"op":"Ge","scaled":70000},{"op":"Le","scaled":100000}],"all":true},{"Text":"High"}]],"default":null,"absent":null,"target":"threatintel_indicator_confidence"}}]}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_04169c2c")?;
                    event.append("error.message", json!(format!("failed to normalize confidence value `{}`: Processor {} with tag {} in pipeline {} failed with message: {}", event.get("json.confidence").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            if event.has_value("threatintel_indicator_confidence") {
                event.rename(
                    "threatintel_indicator_confidence",
                    "threat.indicator.confidence",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.asn") {
                    if let Some(val) = event.get("json.asn") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.asn".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.as.number", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_json_asn_to_threat_indicator_as_number_8cd5606d",
                )?;
                event.append("error.message", json!(format!("Cannot convert asn field `{}` to long: Processor {} with tag {} in pipeline {} failed with message: {}", event.get("json.asn").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("json.org") {
                event.rename("json.org", "threat.indicator.as.organization.name")?;
            }

            if event.has_value("json.email") {
                event.rename("json.email", "threat.indicator.email.address")?;
            }

            let _cond = { event.has_value("json.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("json.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.srcip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.srcip") {
                        if let Some(val) = event.get("json.srcip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.srcip".into(),
                                    message,
                                }
                            })?;
                            event.set("threat.indicator.ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_srcip_to_ip")?;
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

            let _cond = { event.has_value("threat.indicator.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("threat.indicator.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("threat.indicator.ip") };
            if _cond {
                if event.has_value("threat.indicator.ip") {
                    if let Some(ip_str) = event.get_string("threat.indicator.ip") {
                        let ip_str = ip_str.to_string();
                        // GeoIP enrichment (GeoLite2-City.mmdb)
                        if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                            if let Some(v) = geo.get("country_iso_code") {
                                event.set("threat.indicator.geo.country_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("country_name") {
                                event.set("threat.indicator.geo.country_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("continent_name") {
                                event.set("threat.indicator.geo.continent_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_iso_code") {
                                event.set("threat.indicator.geo.region_iso_code", v.clone())?;
                            }
                            if let Some(v) = geo.get("region_name") {
                                event.set("threat.indicator.geo.region_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("city_name") {
                                event.set("threat.indicator.geo.city_name", v.clone())?;
                            }
                            if let Some(v) = geo.get("timezone") {
                                event.set("threat.indicator.geo.timezone", v.clone())?;
                            }
                            if let Some(v) = geo.get("location") {
                                event.set("threat.indicator.geo.location", v.clone())?;
                            }
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.url") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if !uri_parts(event, "json.url", "threat.indicator.url", true, true)?
                        && event
                            .get_str("json.url")
                            .is_some_and(|value| !value.is_empty())
                    {
                        return Err(TransformError::ParseError {
                            path: "json.url".into(),
                            message: "uri_parts: not a parseable URI".into(),
                        });
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "uri_parts")?;
                    event.append("error.message", json!(format!("Cannot parse url field `{}`: Processor {} with tag {} in pipeline {} failed with message: {}", event.get("json.url").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let v = json!(
                event
                    .get("threat.indicator.url.original")
                    .map_or_else(String::new, template_to_string)
            );
            if !painless_is_empty_value(&v) {
                event.set("threat.indicator.url.full", v)?;
            }

            let _cond = { !event.has_value("threat.indicator.url.domain") };
            if _cond {
                if event.has_value("json.domain") {
                    event.rename("json.domain", "threat.indicator.url.domain")?;
                }
            }

            let _cond = { event.has_value("json.domain") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("json.domain")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { !event.has_value("threat.indicator.geo.country_iso_code") };
            if _cond {
                if event.has_value("json.country") {
                    event.rename("json.country", "threat.indicator.geo.country_iso_code")?;
                }
            }

            let _cond = {
                event.has_value("json.md5")
                    && event
                        .get_as_string("json.md5")
                        .is_some_and(|s| s.len() == 32)
            };
            if _cond {
                event.rename("json.md5", "threat.indicator.file.hash.md5")?;
            }

            let _cond = {
                event.has_value("json.md5")
                    && event
                        .get_as_string("json.md5")
                        .is_some_and(|s| s.len() == 40)
            };
            if _cond {
                event.rename("json.md5", "threat.indicator.file.hash.sha1")?;
            }

            let _cond = {
                event.has_value("json.md5")
                    && event
                        .get_as_string("json.md5")
                        .is_some_and(|s| s.len() == 64)
            };
            if _cond {
                event.rename("json.md5", "threat.indicator.file.hash.sha256")?;
            }

            let _cond = {
                event.has_value("json.md5")
                    && event
                        .get_as_string("json.md5")
                        .is_some_and(|s| s.len() == 128)
            };
            if _cond {
                event.rename("json.md5", "threat.indicator.file.hash.sha512")?;
            }

            let _cond = { event.has_value("json.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("json.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.source") {
                event.rename("json.source", "threat.indicator.provider")?;
            }

            let _cond = { event.has_value("json.severity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "46dd881b651e872d3425f948452ca351740f8b9e18b89e65525c1258012ba17d",
                            r#"{"params":null,"known":["CaseInsensitiveLadder"]}"#,
                            r#"ctx.event = ctx.event ?: [:];\nString severity_value = ctx.json.severity;\nif (severity_value.equalsIgnoreCase(\"low\")) {\n  ctx.event.severity = 21;\n} else if (severity_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (severity_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (severity_value.equalsIgnoreCase(\"very-high\")) {\n  ctx.event.severity = 99;\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_event_severity")?;
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
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = {
                event.has_value("json.trusted_circle_ids")
                    && event
                        .get("json.trusted_circle_ids")
                        .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan(
                        event,
                        compiled_painless!(
                            "63c523157a1363b576fb4657c2fb2eda32c116db133f29d75a1fceee2d22077f",
                            r#"{"params":null,"known":[{"SplitTokenField":{"source":"json.trusted_circle_ids","separator":",","parse_int":false,"target":"json.trusted_circle_ids","head":null,"drop_empty":true,"remove_if_empty":true}}]}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "script_51613d11")?;
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
                if event.has_value("json.detail") {
                    if let Some(s) = event.get_string("json.detail") {
                        let mut parts: Vec<Value> = cached_regex!("(?<!\\\\),")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        if parts.len() > 1 {
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                        }
                        event.set("json.detail", Value::Array(parts))?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "split")?;
                event.append(
                    "error.message",
                    json!(format!(
                        "failed to split detail field '{}': {}",
                        event
                            .get("json.detail")
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

            if event.has_value("json.detail") {
                foreach_array(event, "json.detail", |event| {
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

            let _cond = { event.has_value("json.id") };
            if _cond {
                if let Some(val) = event.get("json.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.id".into(),
                            message,
                        }
                    })?;
                    event.set("json.id", converted)?;
                }
            }

            let _cond = { event.has_value("json.source_feed_id") };
            if _cond {
                if let Some(val) = event.get("json.source_feed_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.source_feed_id".into(),
                            message,
                        }
                    })?;
                    event.set("json.source_feed_id", converted)?;
                }
            }

            let _cond = { event.has_value("json.update_id") };
            if _cond {
                if let Some(val) = event.get("json.update_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.update_id".into(),
                            message,
                        }
                    })?;
                    event.set("json.update_id", converted)?;
                }
            }

            let _cond = { event.has_value("json.import_session_id") };
            if _cond {
                if let Some(val) = event.get("json.import_session_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.import_session_id".into(),
                            message,
                        }
                    })?;
                    event.set("json.import_session_id", converted)?;
                }
            }

            event.remove("_temp_");
            event.remove("_conf");
            event.remove("json.asn");
            event.remove("json.date_first");
            event.remove("json.date_last");
            event.remove("json.detail");
            event.remove("json.lat");
            event.remove("json.lon");
            event.remove("json.srcip");
            event.remove("json.country");

            // The pipeline's script, emitted as the runtime's equivalent
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

            event.rename("json", "anomali.threatstream")?;

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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
