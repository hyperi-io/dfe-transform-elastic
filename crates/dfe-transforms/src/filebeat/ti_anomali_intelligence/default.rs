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

            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            event.set("event.kind", json!("enrichment"))?;

            event.set("event.category", Value::Array(vec![json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("indicator")]))?;

            event.set(
                "@timestamp",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.created_ts") {
                event.rename("json.created_ts", "event.created")?;
            }

            let _cond = {
                event.has_value("json.modified_ts") && event.get_str("json.modified_ts") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.modified_ts") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("threat.indicator.modified_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.modified_ts".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_json_modified_ts")?;
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
                // Painless script
                // Source: def calculate(String now, String duration, String expiration_ts) {\n  def result = [:];\n  def start = ZonedDateTime.parse(now);\n  ZonedDateTime end;\n  char time_unit = duration.charAt(duration.length() - 1);\n  def time_value = Long.parseLong(duration.substring(0, duration.length() - 1));\n  if (time_unit == (char)'d') {\n    end = start.plusDays(time_value);\n  } else if (time_unit == (char)'h') {\n    end = start.plusHours(time_value);\n  } else if (time_unit == (char)'m') {\n    end = start.plusMinutes(time_value);\n  } else {\n    result[\"message\"] = \"Invalid ioc_duration_before_deletion: using default of 90 days\";\n    end = start.plusDays(90L);\n  }\n  ZonedDateTime upstream_expiration = null;\n  if (expiration_ts != null && expiration_ts != \"\") {\n    upstream_expiration = ZonedDateTime.parse(expiration_ts);\n  }\n  if (upstream_expiration != null && upstream_expiration.isBefore(end)) {\n    result[\"time\"] = upstream_expiration;\n  } else {\n    result[\"time\"] = end;\n  }\n  return result;\n}\ndef result = calculate(ctx[\"@timestamp\"], ctx._conf.ioc_duration_before_deletion, ctx.json.expiration_ts);\nif (result.message != null) {\n  if (ctx.error == null) {\n    ctx.error = [:];\n  }\n  if (ctx.error.message == null) {\n    ctx.error.message = [];\n  }\n  ctx.error.message.add(result.message);\n}\nctx.json[\"deletion_scheduled_at\"] = result.time;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def calculate(String now, String duration, String expiration_ts) {\n  def result = [:];\n  def start = ZonedDateTime.parse(now);\n  ZonedDateTime end;\n  char time_unit = duration.charAt(duration.length() - 1);\n  def time_value = Long.parseLong(duration.substring(0, duration.length() - 1));\n  if (time_unit == (char)'d') {\n    end = start.plusDays(time_value);\n  } else if (time_unit == (char)'h') {\n    end = start.plusHours(time_value);\n  } else if (time_unit == (char)'m') {\n    end = start.plusMinutes(time_value);\n  } else {\n    result[\"message\"] = \"Invalid ioc_duration_before_deletion: using default of 90 days\";\n    end = start.plusDays(90L);\n  }\n  ZonedDateTime upstream_expiration = null;\n  if (expiration_ts != null && expiration_ts != \"\") {\n    upstream_expiration = ZonedDateTime.parse(expiration_ts);\n  }\n  if (upstream_expiration != null && upstream_expiration.isBefore(end)) {\n    result[\"time\"] = upstream_expiration;\n  } else {\n    result[\"time\"] = end;\n  }\n  return result;\n}\ndef result = calculate(ctx[\"@timestamp\"], ctx._conf.ioc_duration_before_deletion, ctx.json.expiration_ts);\nif (result.message != null) {\n  if (ctx.error == null) {\n    ctx.error = [:];\n  }\n  if (ctx.error.message == null) {\n    ctx.error.message = [];\n  }\n  ctx.error.message.add(result.message);\n}\nctx.json[\"deletion_scheduled_at\"] = result.time;\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_calculate_deletion_scheduled_at",
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
                // Painless script
                // Source: if (params.containsKey(ctx.json.itype) && params[ctx.json.itype] != null) {\n  if (ctx.threat == null) {\n    ctx.threat = [:];\n  }\n  if (ctx.threat.indicator == null) {\n    ctx.threat.indicator = [:];\n  }\n  ctx.threat.indicator.type = params[ctx.json.itype];\n} else {\n  throw new IllegalArgumentException(\"indicator type not found from itype\");\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"if (params.containsKey(ctx.json.itype) && params[ctx.json.itype] != null) {\n  if (ctx.threat == null) {\n    ctx.threat = [:];\n  }\n  if (ctx.threat.indicator == null) {\n    ctx.threat.indicator = [:];\n  }\n  ctx.threat.indicator.type = params[ctx.json.itype];\n} else {\n  throw new IllegalArgumentException(\"indicator type not found from itype\");\n}\n"#
                    ),
                    cached_params!(
                        "{\"actor_ip\":\"ipv4-addr\",\"actor_ipv6\":\"ipv6-addr\",\"actor_subject\":null,\"adware_domain\":\"domain-name\",\"adware_registry_key\":\"windows-registry-key\",\"anon_proxy\":\"ipv4-addr\",\"anon_proxy_ipv6\":\"ipv6-addr\",\"anon_vpn\":\"ipv4-addr\",\"anon_vpn_ipv6\":\"ipv6-addr\",\"apt_domain\":\"domain-name\",\"apt_email\":\"email-addr\",\"apt_email_subject_line\":\"email\",\"apt_file_name\":\"file\",\"apt_file_path\":\"directory\",\"apt_ip\":\"ipv4-addr\",\"apt_ipv6\":\"ipv6-addr\",\"apt_md5\":\"file\",\"apt_mta\":null,\"apt_mutex\":\"mutex\",\"apt_registry_key\":\"windows-registry-key\",\"apt_service_description\":null,\"apt_service_displayname\":null,\"apt_service_name\":null,\"apt_ssdeep\":\"file\",\"apt_subject\":\"email\",\"apt_ua\":\"url\",\"apt_url\":\"url\",\"bot_domain\":\"domain-name\",\"bot_ip\":\"ipv4-addr\",\"bot_ipv6\":\"ipv6-addr\",\"bot_md5\":\"file\",\"bot_url\":\"url\",\"browser_extension_id\":null,\"brute_ip\":\"ipv4-addr\",\"brute_ipv6\":\"ipv6-addr\",\"c2_domain\":\"domain-name\",\"c2_ip\":\"ipv4-addr\",\"c2_ipv6\":\"ipv6-addr\",\"c2_url\":\"url\",\"comm_proxy_domain\":\"domain-name\",\"comm_proxy_ip\":\"ipv4-addr\",\"compromised_domain\":\"domain-name\",\"compromised_email\":\"email-addr\",\"compromised_email_subject\":\"email\",\"compromised_ip\":\"ipv4-addr\",\"compromised_ipv6\":\"ipv6-addr\",\"compromised_serv_account\":\"user-account\",\"compromised_url\":\"url\",\"compromised_username\":\"user-account\",\"crypto_hash\":\"file\",\"crypto_ip\":\"ipv4-addr\",\"crypto_pool\":\"domain\",\"crypto_url\":\"url\",\"crypto_wallet\":\"file\",\"ddos_ip\":\"ipv4-addr\",\"ddos_ipv6\":\"ipv6-addr\",\"disposable_email_domain\":\"domain-name\",\"downloader_domain\":\"domain-name\",\"downloader_hash\":\"file\",\"downloader_ip\":\"ipv4-addr\",\"downloader_ipv6\":\"ipv6-addr\",\"downloader_url\":\"url\",\"dyn_dns\":\"domain-name\",\"email_attachment_subject\":\"email\",\"exfil_domain\":\"domain-name\",\"exfil_ip\":\"ipv4-addr\",\"exfil_ipv6\":\"ipv6-addr\",\"exfil_url\":\"url\",\"exploit_domain\":\"domain-name\",\"exploit_ip\":\"ipv4-addr\",\"exploit_ipv6\":\"ipv6-addr\",\"exploit_md5\":\"file\",\"exploit_url\":\"url\",\"fraud_domain\":\"domain-name\",\"fraud_email\":\"email-addr\",\"fraud_email_subject\":\"email\",\"fraud_ip\":\"ipv4-addr\",\"fraud_ipv6\":\"ipv6-addr\",\"fraud_md5\":\"file\",\"fraud_url\":\"url\",\"free_email_domain\":\"domain-name\",\"geolocation_url\":\"url\",\"hack_tool\":\"file\",\"hack_tool_md5\":\"file\",\"i2p_ip\":\"ipv4-addr\",\"i2p_ipv6\":\"ipv6-addr\",\"image_hash\":\"file\",\"infostealer_domain\":\"domain-name\",\"infostealer_hash\":\"file\",\"infostealer_ip\":\"ipv4-addr\",\"infostealer_ipv6\":\"ipv6-addr\",\"infostealer_url\":\"url\",\"iot_domain\":\"domain-name\",\"iot_hash\":\"file\",\"iot_ip\":\"ipv4-addr\",\"iot_ipv6\":\"ipv6-addr\",\"iot_url\":\"url\",\"ipcheck_url\":\"url\",\"mal_domain\":\"domain-name\",\"mal_email\":\"email-addr\",\"mal_email_subject\":\"email\",\"mal_file_name\":\"file\",\"mal_file_path\":\"file\",\"mal_ip\":\"ipv4-addr\",\"mal_ipv6\":\"ipv6-addr\",\"mal_md5\":\"file\",\"mal_mutex\":\"mutex\",\"mal_registry_key\":\"windows-registry-key\",\"mal_service_description\":null,\"mal_service_displayname\":null,\"mal_service_name\":null,\"mal_ssdeep\":\"file\",\"mal_sslcert_sh1\":\"x509-certificate\",\"mal_sslcert_sha1\":\"x509-certificate\",\"mal_ua\":\"url\",\"mal_url\":\"url\",\"p2pcnc\":\"ipv4-addr\",\"p2pcnc_ipv6\":\"ipv6-addr\",\"parked_domain\":\"domain-name\",\"parked_ip\":\"ipv4-addr\",\"parked_ipv6\":\"ipv6-addr\",\"parked_url\":\"url\",\"pastesite_url\":\"url\",\"phish_domain\":\"domain-name\",\"phish_email\":\"email-addr\",\"phish_email_subject\":\"email\",\"phish_ip\":\"ipv4-addr\",\"phish_ipv6\":\"ipv6-addr\",\"phish_md5\":\"file\",\"phish_url\":\"url\",\"pos_domain\":\"domain-name\",\"pos_hash\":\"file\",\"pos_ip\":\"ipv4-addr\",\"pos_ipv6\":\"ipv6-addr\",\"pos_url\":\"url\",\"proxy_ip\":\"ipv4-addr\",\"proxy_ipv6\":\"ipv6-addr\",\"ransomware_domain\":\"domain-name\",\"ransomware_hash\":\"file\",\"ransomware_ip\":\"ipv4-addr\",\"ransomware_ipv6\":\"ipv6-addr\",\"ransomware_url\":\"url\",\"rootkit_hash\":\"file\",\"scan_ip\":\"ipv4-addr\",\"scan_ipv6\":\"ipv6-addr\",\"sinkhole_domain\":\"domain-name\",\"sinkhole_ip\":\"ipv4-addr\",\"sinkhole_ipv6\":\"ipv6-addr\",\"social_media_url\":\"url\",\"spam_domain\":\"domain-name\",\"spam_email\":\"email-addr\",\"spam_email_subject\":\"email\",\"spam_ip\":\"ipv4-addr\",\"spam_ipv6\":\"ipv6-addr\",\"spam_mta\":null,\"spam_url\":\"url\",\"speedtest_url\":\"url\",\"ssh_ip\":\"ipv4-addr\",\"ssh_ipv6\":\"ipv6-addr\",\"ssl_cert_serial_number\":null,\"suppress\":\"suppress\",\"suspicious_domain\":\"domain-name\",\"suspicious_email\":\"email-addr\",\"suspicious_email_subject\":\"email\",\"suspicious_ip\":\"ipv4-addr\",\"suspicious_reg_email\":\"email-addr\",\"suspicious_url\":\"url\",\"tor_ip\":\"ipv4-addr\",\"tor_ipv6\":\"ipv6-addr\",\"torrent_tracker_url\":\"url\",\"trojan_domain\":\"domain-name\",\"trojan_hash\":\"file\",\"trojan_ip\":\"ipv4-addr\",\"trojan_ipv6\":\"ipv6-addr\",\"trojan_url\":\"url\",\"vpn_domain\":\"domain-name\",\"vps_ip\":\"ipv4-addr\",\"vps_ipv6\":\"ipv6-addr\",\"whois_bulk_reg_email\":\"email-addr\",\"whois_privacy_domain\":\"domain-name\",\"whois_privacy_email\":\"email-addr\"}"
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_map_itype_field_to_stix_types",
                )?;
                event.append("error.message", json!(format!("Unable to determine STIX 2.0 indicator type from itype \"{}\": Processor {} with tag {} in pipeline {} failed with message: {}", event.get("json.itype").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            let _cond = {
                event.get_str("threat.indicator.type") == Some("ipv4-addr")
                    && event.has_value("json.ip")
                    && event.get("json.ip").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some(":")),
                        serde_json::Value::String(s) => s.contains(":"),
                        _ => false,
                    })
            };
            if _cond {
                event.set("threat.indicator.type", json!("ipv6-addr"))?;
            }

            let _cond = { event.has_value("json.latitude") && event.has_value("json.longitude") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("json.latitude") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.latitude".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.geo.location.lat", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_latitude")?;
                    event.append("error.message", json!(format!("Cannot convert latitude field \"{}\" to double: Processor {} with tag {} in pipeline {} failed with message: {}", event.get("json.latitude").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            let _cond = { event.has_value("json.latitude") && event.has_value("json.longitude") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(val) = event.get("json.longitude") {
                        let converted = convert_value(val, "double").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.longitude".into(),
                                message,
                            }
                        })?;
                        event.set("threat.indicator.geo.location.lon", converted)?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_json_longitude")?;
                    event.append("error.message", json!(format!("Cannot convert longitude field \"{}\" to double: Processor {} with tag {} in pipeline {} failed with message: {}", event.get("json.longitude").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            event.remove("json.latitude");
            event.remove("json.longitude");

            if event.has_value("json.tlp") {
                event.rename("json.tlp", "threat.indicator.marking.tlp")?;
            }

            if event.has_value("threat.indicator.marking.tlp") {
                map_strings(
                    event,
                    "threat.indicator.marking.tlp",
                    "threat.indicator.marking.tlp",
                    str::to_uppercase,
                )?;
            }

            let _cond = {
                !event.has_value("threat.indicator.marking.tlp")
                    && !(event.get_bool("json.is_public") == Some(true))
            };
            if _cond {
                event.set("threat.indicator.marking.tlp", json!("AMBER"))?;
            }

            let _cond = {
                !event.has_value("threat.indicator.marking.tlp")
                    && event.get_bool("json.is_public") == Some(true)
            };
            if _cond {
                event.set("threat.indicator.marking.tlp", json!("WHITE"))?;
            }

            let _cond = { event.has_value("json.confidence") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.threat == null) {\n  ctx.threat = [:];\n} if (ctx.threat.indicator == null) {\n  ctx.threat.indicator = [:];\n} def value = ctx.json.confidence; if (value == null) {\n  ctx.threat.indicator.confidence = \"Not Specified\";\n} else if (value <= 0.0 || 100.0 < value) {\n  ctx.threat.indicator.confidence = \"None\";\n} else if (value < 30.0) {\n  ctx.threat.indicator.confidence = \"Low\";\n} else if (value < 70.0) {\n  ctx.threat.indicator.confidence = \"Medium\";\n} else {\n  ctx.threat.indicator.confidence = \"High\";\n}\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"if (ctx.threat == null) {\n  ctx.threat = [:];\n} if (ctx.threat.indicator == null) {\n  ctx.threat.indicator = [:];\n} def value = ctx.json.confidence; if (value == null) {\n  ctx.threat.indicator.confidence = \"Not Specified\";\n} else if (value <= 0.0 || 100.0 < value) {\n  ctx.threat.indicator.confidence = \"None\";\n} else if (value < 30.0) {\n  ctx.threat.indicator.confidence = \"Low\";\n} else if (value < 70.0) {\n  ctx.threat.indicator.confidence = \"Medium\";\n} else {\n  ctx.threat.indicator.confidence = \"High\";\n}\n"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_normalize_confidence_to_stix_values",
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

            let _cond = { event.get_str("json.asn") != Some("") };
            if _cond {
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
                    event.set("_ingest.on_failure_processor_tag", "convert_json_asn")?;
                    event.append("error.message", json!(format!("Cannot convert asn field `{}` to long: Processor {} with tag {} in pipeline {} failed with message: {}", event.get("json.asn").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            event.remove("json.asn");

            let _cond = { event.has_value("json.org") && event.get_str("json.org") != Some("") };
            if _cond {
                event.rename("json.org", "threat.indicator.as.organization.name")?;
            }

            if event.has_value("json.country") {
                event.rename("json.country", "threat.indicator.geo.country_iso_code")?;
            }

            let _cond = { event.get_str("json.ip") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.ip") {
                        if let Some(val) = event.get("json.ip") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.ip".into(),
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
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_json_ip_to_threat_indicator_ip_da6daf63",
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

            let _cond = { event.get_str("json.type") == Some("email") };
            if _cond {
                if let Some(v) = event
                    .get("json.value")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.email.address", v)?;
                }
            }

            let _cond = {
                event.get_str("json.type") == Some("email")
                    && event.get_str("json.value") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("domain") };
            if _cond {
                if let Some(v) = event
                    .get("json.value")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.url.domain", v)?;
                }
            }

            let _cond =
                { event.get_str("json.type") == Some("domain") && event.has_value("json.value") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("json.type") == Some("url") };
            if _cond {
                uri_parts(event, "json.value", "threat.indicator.url", true, false)?;
            }

            if let Some(v) = event
                .get("threat.indicator.url.original")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.url.full", v)?;
            }

            let _cond = {
                event.get_str("json.type") == Some("md5")
                    && event.get_str("json.subtype") == Some("MD5")
            };
            if _cond {
                if let Some(v) = event.get("json.value").cloned() {
                    event.set("threat.indicator.file.hash.md5", v)?;
                }
            }

            let _cond = {
                event.get_str("json.type") == Some("md5")
                    && event.get_str("json.subtype") == Some("SHA1")
            };
            if _cond {
                if let Some(v) = event.get("json.value").cloned() {
                    event.set("threat.indicator.file.hash.sha1", v)?;
                }
            }

            let _cond = {
                event.get_str("json.type") == Some("md5")
                    && event.get_str("json.subtype") == Some("SHA256")
            };
            if _cond {
                if let Some(v) = event.get("json.value").cloned() {
                    event.set("threat.indicator.file.hash.sha256", v)?;
                }
            }

            let _cond = {
                event.get_str("json.type") == Some("md5")
                    && event.get_str("json.subtype") == Some("SHA512")
            };
            if _cond {
                if let Some(v) = event.get("json.value").cloned() {
                    event.set("threat.indicator.file.hash.sha512", v)?;
                }
            }

            let _cond = { event.get_str("json.type") == Some("md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("json.value")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.value") };
            if _cond {
                if let Some(v) = event
                    .get("json.value")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("threat.indicator.name", v)?;
                }
            }

            event.remove("json.subtype");

            let _cond = {
                event
                    .get_str("json.itype")
                    .is_some_and(|s| s.ends_with("_ssdeep"))
            };
            if _cond {
                if let Some(v) = event.get("json.value").cloned() {
                    event.set("threat.indicator.file.hash.ssdeep", v)?;
                }
            }

            let _cond = {
                event
                    .get_str("json.itype")
                    .is_some_and(|s| s.ends_with("_file_name"))
            };
            if _cond {
                if let Some(v) = event.get("json.value").cloned() {
                    event.set("threat.indicator.file.path", v)?;
                }
            }

            if event.has_value("json.source") {
                event.rename("json.source", "threat.indicator.provider")?;
            }

            if event.has_value("json.import_session_id") {
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

            if event.has_value("json.trusted_circle_ids") {
                if let Some(val) = event.get("json.trusted_circle_ids") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.trusted_circle_ids".into(),
                            message,
                        }
                    })?;
                    event.set("json.trusted_circle_ids", converted)?;
                }
            }

            let _cond = { event.has_value("json.meta.severity") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: ctx.event = ctx.event ?: [:];\nString severity_value = ctx.json.meta.severity;\nif (severity_value.equalsIgnoreCase(\"low\")) {\n  ctx.event.severity = 21;\n} else if (severity_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (severity_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (severity_value.equalsIgnoreCase(\"very-high\")) {\n  ctx.event.severity = 99;\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"ctx.event = ctx.event ?: [:];\nString severity_value = ctx.json.meta.severity;\nif (severity_value.equalsIgnoreCase(\"low\")) {\n  ctx.event.severity = 21;\n} else if (severity_value.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (severity_value.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (severity_value.equalsIgnoreCase(\"very-high\")) {\n  ctx.event.severity = 99;\n}"#
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

            if event.has_value("json.tags") {
                foreach_array(event, "json.tags", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        event.append(
                            "tags",
                            json!(
                                event
                                    .get("_ingest._value.name")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "append")?;
                        event.append("error.message", json!(format!("Found a tag without a name `{}`: Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest._value").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                        event.remove("_ingest.on_failure_message");
                        event.remove("_ingest.on_failure_processor_type");
                        event.remove("_ingest.on_failure_processor_tag");
                        if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                            event.remove("_ingest");
                        }
                    }
                    Ok(())
                })?;
            }

            event.remove("json.tags");

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.json?.id != null && !(ctx.json.id instanceof String)) {\n  ctx.json.id = Long.toString((long) ctx.json.id);\n}\nif (ctx.json?.update_id != null && !(ctx.json.update_id instanceof String)) {\n  ctx.json.update_id = Long.toString((long) ctx.json.update_id);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.json?.id != null && !(ctx.json.id instanceof String)) {\n  ctx.json.id = Long.toString((long) ctx.json.id);\n}\nif (ctx.json?.update_id != null && !(ctx.json.update_id instanceof String)) {\n  ctx.json.update_id = Long.toString((long) ctx.json.update_id);\n}\n"#
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_convert_a_large_integer",
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

            if event.has_value("json.meta.registrant_address") {
                event.rename(
                    "json.meta.registrant_address",
                    "json.meta.registrant.address",
                )?;
            }

            if event.has_value("json.meta.registrant_email") {
                event.rename("json.meta.registrant_email", "json.meta.registrant.email")?;
            }

            let _cond = { event.get_str("json.meta.registrant.email") != Some("") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("json.meta.registrant.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.meta.registrant_name") {
                event.rename("json.meta.registrant_name", "json.meta.registrant.name")?;
            }

            if event.has_value("json.meta.registrant_org") {
                event.rename("json.meta.registrant_org", "json.meta.registrant.org")?;
            }

            if event.has_value("json.meta.registrant_phone") {
                event.rename("json.meta.registrant_phone", "json.meta.registrant.phone")?;
            }

            event.remove("_conf");
            event.remove("json.created_by");
            event.remove("json.description");
            event.remove("json.locations");
            event.remove("json.meta.detail");
            event.remove("json.meta.detail2");
            event.remove("json.modified_ts");
            event.remove("json.resource_uri");
            event.remove("json.sort");
            event.remove("json.source_locations");
            event.remove("json.target_industry");
            event.remove("json.threatscore");
            event.remove("json.workgroups");
            event.remove("json.ip");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);"#
                ),
            )?;

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
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"\n", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
