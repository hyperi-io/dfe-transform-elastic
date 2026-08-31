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

            if event.has_value("event.id") {
                if let Some(val) = event.get("event.id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "event.id".into(),
                            message,
                        }
                    })?;
                    event.set("event.id", converted)?;
                }
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename(
                    "destination.as.organization_name",
                    "destination.as.organization.name",
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.fileHash")
                    && event.get_str("cef.extensions.fileHash") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("cef.extensions.fileHash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("cef.extensions.oldFileHash")
                    && event.get_str("cef.extensions.oldFileHash") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("cef.extensions.oldFileHash")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("destination.ip") && event.get_str("destination.ip") != Some("")
            };
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

            let _cond = {
                event.has_value("destination.nat.ip")
                    && event.get_str("destination.nat.ip") != Some("")
            };
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

            let _cond = { event.has_value("source.ip") && event.get_str("source.ip") != Some("") };
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

            let _cond =
                { event.has_value("source.nat.ip") && event.get_str("source.nat.ip") != Some("") };
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

            let _cond = { event.has_value("destination.user.name") };
            if _cond {
                event.append(
                    "related.user",
                    json!(
                        event
                            .get("destination.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("source.user.name") && event.get_str("source.user.name") != Some("")
            };
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

            let _cond = {
                event.has_value("observer.hostname")
                    && event.get_str("observer.hostname") != Some("")
            };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("observer.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("cef.device.vendor") == Some("FORCEPOINT") };
            if _cond {
                // Begin nested pipeline: "fp-pipeline"
                let v = json!(
                    event
                        .get("cef.extensions.deviceCustomString1")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("rule.id", v)?;
                }
                let v = json!(
                    event
                        .get("cef.extensions.deviceCustomString2")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("rule.id", v)?;
                }
                let v = json!(
                    event
                        .get("cef.extensions.deviceCustomString3")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("vulnerability.reference", v)?;
                }
                let v = json!(
                    event
                        .get("cef.extensions.deviceCustomString4")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("cef.forcepoint.virus_id", v)?;
                }
                // End nested pipeline: "fp-pipeline"
            }

            let _cond = { event.get_str("cef.device.vendor") == Some("Check Point") };
            if _cond {
                // Begin nested pipeline: "cp-pipeline"
                // Painless script
                // Source: def actions = new ArrayList();\ndef exts = ctx.cef?.extensions;\nif (exts == null) return;\nfor (entry in params.extensions) {\n  def value = exts[entry.name];\n  if (value == null ||\n    (entry.convert != null &&\n      (value=entry.convert[value.toLowerCase()]) == null))\n    continue;\n  if (entry.to != null) {\n    actions.add([\n      \"value\": value,\n      \"to\": entry.to\n    ]);\n    continue;\n  }\n  def label = exts[entry.name + \"Label\"];\n  if (label == null) continue;\n  def dest = entry.labels[label.toLowerCase()];\n  if (dest == null) continue;\n  actions.add([\n    \"value\": value,\n    \"to\": dest\n  ]);\n}\nctx[\"_tmp_copy\"] = actions;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def actions = new ArrayList();\ndef exts = ctx.cef?.extensions;\nif (exts == null) return;\nfor (entry in params.extensions) {\n  def value = exts[entry.name];\n  if (value == null ||\n    (entry.convert != null &&\n      (value=entry.convert[value.toLowerCase()]) == null))\n    continue;\n  if (entry.to != null) {\n    actions.add([\n      \"value\": value,\n      \"to\": entry.to\n    ]);\n    continue;\n  }\n  def label = exts[entry.name + \"Label\"];\n  if (label == null) continue;\n  def dest = entry.labels[label.toLowerCase()];\n  if (dest == null) continue;\n  actions.add([\n    \"value\": value,\n    \"to\": dest\n  ]);\n}\nctx[\"_tmp_copy\"] = actions;\n"#
                    ),
                    cached_params!(
                        "{\"extensions\":[{\"name\":\"cp_app_risk\",\"to\":\"checkpoint.app_risk\"},{\"name\":\"cp_app_risk\",\"to\":\"event.risk_score\",\"convert\":{\"unknown\":0,\"informational\":0,\"very-low\":1,\"low\":2,\"medium\":3,\"high\":4,\"very-high\":5,\"critical\":5}},{\"name\":\"cp_severity\",\"to\":\"checkpoint.severity\"},{\"name\":\"cp_severity\",\"to\":\"event.severity\",\"convert\":{\"unknown\":0,\"informational\":0,\"very-low\":1,\"low\":1,\"medium\":2,\"high\":3,\"very-high\":4,\"critical\":4}},{\"name\":\"baseEventCount\",\"to\":\"checkpoint.event_count\"},{\"name\":\"deviceExternalId\",\"to\":\"observer.type\"},{\"name\":\"deviceFacility\",\"to\":\"observer.type\",\"convert\":{\"0\":\"Network\",\"1\":\"Endpoint\",\"2\":\"Access\",\"3\":\"Threat\",\"4\":\"Mobile\"}},{\"name\":\"deviceInboundInterface\",\"to\":\"observer.ingress.interface.name\"},{\"name\":\"deviceOutboundInterface\",\"to\":\"observer.egress.interface.name\"},{\"name\":\"externalId\",\"to\":\"checkpoint.uuid\"},{\"name\":\"fileHash\",\"to\":\"checkpoint.file_hash\"},{\"name\":\"reason\",\"to\":\"checkpoint.termination_reason\"},{\"name\":\"requestCookies\",\"to\":\"checkpoint.cookie\"},{\"name\":\"checkrequestCookies\",\"to\":\"checkpoint.cookie\"},{\"name\":\"sourceNtDomain\",\"to\":\"dns.question.name\"},{\"name\":\"Signature\",\"to\":\"vulnerability.id\"},{\"name\":\"Recipient\",\"to\":\"destination.user.email\"},{\"name\":\"Sender\",\"to\":\"source.user.email\"},{\"name\":\"deviceCustomFloatingPoint1\",\"labels\":{\"update version\":\"observer.version\"}},{\"name\":\"deviceCustomIPv6Address2\",\"labels\":{\"source ipv6 address\":\"source.ip\"}},{\"name\":\"deviceCustomIPv6Address3\",\"labels\":{\"destination ipv6 address\":\"destination.ip\"}},{\"name\":\"deviceCustomNumber1\",\"labels\":{\"payload\":\"network.bytes\",\"elapsed time in seconds\":\"event.duration\",\"email recipients number\":\"checkpoint.email_recipients_num\"}},{\"name\":\"deviceCustomNumber2\",\"labels\":{\"duration in seconds\":\"event.duration\",\"icmp type\":\"checkpoint.icmp_type\"}},{\"name\":\"deviceCustomNumber3\",\"labels\":{\"icmp code\":\"checkpoint.icmp_code\"}},{\"name\":\"deviceCustomString1\",\"labels\":{\"application rule name\":\"rule.name\",\"dlp rule name\":\"rule.name\",\"threat prevention rule name\":\"rule.name\",\"connectivity state\":\"checkpoint.connectivity_state\",\"email id\":\"checkpoint.email_id\",\"voip log type\":\"checkpoint.voip_log_type\"}},{\"name\":\"deviceCustomString2\",\"labels\":{\"protection id\":\"checkpoint.protection_id\",\"update status\":\"checkpoint.update_status\",\"email subject\":\"checkpoint.email_subject\",\"sensor mode\":\"checkpoint.sensor_mode\",\"scan invoke type\":\"checkpoint.integrity_av_invoke_type\",\"category\":\"checkpoint.category\",\"categories\":\"rule.category\",\"peer gateway\":\"checkpoint.peer_gateway\"}},{\"name\":\"deviceCustomString6\",\"labels\":{\"application name\":\"network.application\",\"virus name\":\"checkpoint.virus_name\",\"malware name\":\"checkpoint.spyware_name\",\"malware family\":\"checkpoint.malware_family\"}},{\"name\":\"deviceCustomString3\",\"labels\":{\"user group\":\"group.name\",\"incident extension\":\"checkpoint.incident_extension\",\"identity type\":\"checkpoint.identity_type\",\"email spool id\":\"checkpoint.email_spool_id\",\"protection type\":\"checkpoint.protection_type\"}},{\"name\":\"deviceCustomString4\",\"labels\":{\"malware status\":\"checkpoint.spyware_status\",\"destination os\":\"os.name\",\"scan result\":\"checkpoint.scan_result\",\"frequency\":\"checkpoint.frequency\",\"protection name\":\"checkpoint.protection_name\",\"user response\":\"checkpoint.user_status\",\"email control\":\"checkpoint.email_control\",\"tcp flags\":\"checkpoint.tcp_flags\",\"threat prevention rule id\":\"rule.id\"}},{\"name\":\"deviceCustomString5\",\"labels\":{\"matched category\":\"rule.category\",\"authentication method\":\"checkpoint.auth_method\",\"email session id\":\"checkpoint.email_session_id\",\"vlan id\":\"network.vlan.id\"}},{\"name\":\"deviceCustomDate2\",\"labels\":{\"subscription expiration\":\"checkpoint.subs_exp\"}},{\"name\":\"deviceFlexNumber1\",\"labels\":{\"confidence\":\"checkpoint.confidence_level\"}},{\"name\":\"deviceFlexNumber2\",\"labels\":{\"destination phone number\":\"checkpoint.dst_phone_number\",\"performance impact\":\"checkpoint.performance_impact\"}},{\"name\":\"flexString1\",\"labels\":{\"application signature id\":\"checkpoint.app_sig_id\"}},{\"name\":\"flexString2\",\"labels\":{\"malware action\":\"rule.description\",\"attack information\":\"event.action\"}},{\"name\":\"rule_uid\",\"to\":\"rule.uuid\"},{\"name\":\"ifname\",\"to\":\"observer.ingress.interface.name\"},{\"name\":\"inzone\",\"to\":\"observer.ingress.zone\"},{\"name\":\"outzone\",\"to\":\"observer.egress.zone\"},{\"name\":\"product\",\"to\":\"observer.product\"}]}"
                    ),
                )?;
                foreach_array(event, "_tmp_copy", |event| {
                    event.set(
                        "{{{_ingest._value.to}}}",
                        json!(
                            event
                                .get("_ingest._value.value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
                if event.remove("_tmp_copy").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "_tmp_copy".into(),
                    });
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
                let _cond = { event.has_value("source.user.email") };
                if _cond {
                    event.set(
                        "email.from.address",
                        Value::Array(vec![json!(
                            event
                                .get("source.user.email")
                                .map_or_else(String::new, template_to_string)
                        )]),
                    )?;
                }
                let _cond = { event.has_value("checkpoint.email_subject") };
                if _cond {
                    if let Some(v) = event.get("checkpoint.email_subject").cloned() {
                        event.set("email.subject", v)?;
                    }
                }
                let _cond = { event.has_value("checkpoint.email_session_id") };
                if _cond {
                    if let Some(v) = event.get("checkpoint.email_session_id").cloned() {
                        event.set("email.message_id", v)?;
                    }
                }
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("event.risk_score") {
                        if let Some(val) = event.get("event.risk_score") {
                            let converted = convert_value(val, "float").map_err(|message| {
                                TransformError::ParseError {
                                    path: "event.risk_score".into(),
                                    message,
                                }
                            })?;
                            event.set("event.risk_score", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert risk score")?;
                    if event.remove("event.risk_score").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "event.risk_score".into(),
                        });
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
                    if event.has_value("event.severity") {
                        if let Some(val) = event.get("event.severity") {
                            let converted = convert_value(val, "long").map_err(|message| {
                                TransformError::ParseError {
                                    path: "event.severity".into(),
                                    message,
                                }
                            })?;
                            event.set("event.severity", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert event.severity")?;
                    if event.remove("event.severity").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "event.severity".into(),
                        });
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
                    // Painless script
                    // Source: def duration = ctx.event?.duration;\nif (duration == null) return;\nctx.event.duration = Long.parseLong(duration) * params.second_to_nanos;\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"def duration = ctx.event?.duration;\nif (duration == null) return;\nctx.event.duration = Long.parseLong(duration) * params.second_to_nanos;\n"#
                        ),
                        cached_params!("{\"second_to_nanos\":1000000000}"),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "calculate duration")?;
                    event.remove("event.duration");
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
                let _cond = {
                    event.has_value("checkpoint.file_hash")
                        && event
                            .get_as_string("checkpoint.file_hash")
                            .is_some_and(|s| s.len() == 32)
                };
                if _cond {
                    event.rename("checkpoint.file_hash", "file.hash.md5")?;
                }
                let _cond = {
                    event.has_value("checkpoint.file_hash")
                        && event
                            .get_as_string("checkpoint.file_hash")
                            .is_some_and(|s| s.len() == 40)
                };
                if _cond {
                    event.rename("checkpoint.file_hash", "file.hash.sha1")?;
                }
                let _cond = {
                    event.has_value("checkpoint.file_hash")
                        && event
                            .get_as_string("checkpoint.file_hash")
                            .is_some_and(|s| s.len() == 64)
                };
                if _cond {
                    event.rename("checkpoint.file_hash", "file.hash.sha256")?;
                }
                event.set("event.kind", json!("event"))?;
                let _cond =
                    { event.has_value("cef.extensions.cp_app_risk") && event.has_value("rule") };
                if _cond {
                    event.set("event.kind", json!("alert"))?;
                }
                let _cond = { event.has_value("source.ip") && event.has_value("destination.ip") };
                if _cond {
                    event.append("event.category", json!("network"))?;
                }
                let _cond = {
                    event.has_value("checkpoint.protection_id")
                        || event.has_value("checkpoint.spyware_name")
                        || event.has_value("checkpoint.malware_family")
                        || event.has_value("checkpoint.spyware_status")
                };
                if _cond {
                    event.append("event.category", json!("malware"))?;
                }
                let _cond = {
                    event.has_value("event.category")
                        && !(event.get("event.action").is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("malware"))
                            }
                            serde_json::Value::String(s) => s.contains("malware"),
                            _ => false,
                        }))
                        && (event.has_value("checkpoint.protection_type")
                            || event.get_str("cef.extensions.flexString2Label")
                                == Some("Attack Information"))
                };
                if _cond {
                    event.append("event.category", json!("intrusion_detection"))?;
                }
                if event.has_value("checkpoint.event_count") {
                    if let Some(val) = event.get("checkpoint.event_count") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "checkpoint.event_count".into(),
                                message,
                            }
                        })?;
                        event.set("checkpoint.event_count", converted)?;
                    }
                }
                if event.has_value("cef.extensions.baseEventCount") {
                    if let Some(val) = event.get("cef.extensions.baseEventCount") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "cef.extensions.baseEventCount".into(),
                                message,
                            }
                        })?;
                        event.set("cef.extensions.baseEventCount", converted)?;
                    }
                }
                // End nested pipeline: "cp-pipeline"
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.ip") {
                    // Community ID v1 hash
                    if let (Some(src_ip), Some(dst_ip), Some(protocol)) = (
                        event.get_string("source.ip"),
                        event.get_string("destination.ip"),
                        event
                            .get_as_string("network.iana_number")
                            .or_else(|| event.get_as_string("network.transport")),
                    ) {
                        let icmp = matches!(
                            protocol.to_ascii_lowercase().as_str(),
                            "icmp" | "1" | "icmpv6" | "ipv6-icmp" | "58",
                        );
                        let (src_field, dst_field) = if icmp {
                            ("icmp.type", "icmp.code")
                        } else {
                            ("source.port", "destination.port")
                        };
                        let src_port =
                            u16::try_from(event.get_as_i64(src_field).unwrap_or(0)).unwrap_or(0);
                        let dst_port =
                            u16::try_from(event.get_as_i64(dst_field).unwrap_or(0)).unwrap_or(0);
                        match community_id_v1(&src_ip, &dst_ip, src_port, dst_port, &protocol) {
                            Ok(cid) => event.set("network.community_id", cid)?,
                            Err(message) => {
                                return Err(TransformError::ParseError {
                                    path: "network.community_id".into(),
                                    message,
                                });
                            }
                        }
                    }
                }
                Ok(())
            })();

            if event.has_value("destination.mac") {
                gsub_field(
                    event,
                    "destination.mac",
                    "destination.mac",
                    cached_regex!("[:.]"),
                    "-",
                )?;
            }

            if event.has_value("source.mac") {
                gsub_field(
                    event,
                    "source.mac",
                    "source.mac",
                    cached_regex!("[:.]"),
                    "-",
                )?;
            }

            if event.has_value("destination.mac") {
                map_strings(
                    event,
                    "destination.mac",
                    "destination.mac",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("source.mac") {
                map_strings(event, "source.mac", "source.mac", str::to_uppercase)?;
            }

            let _cond = { !event.has_value("cef.extensions.deviceReceiptTime") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if let Some(input) = event.get_string("event.original") {
                        // Grok pattern: ^(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601}))
                        // Grok pattern: ^(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601}))
                        // Grok pattern: ^(?:<%{NONNEGINT:log.syslog.priority:long}>)%{NONNEGINT} (?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601}))
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601})) "
                                ),
                                cached_grok!(
                                    "^(?:<%{NONNEGINT:log.syslog.priority:long}>)(?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601})) "
                                ),
                                cached_grok!(
                                    "^(?:<%{NONNEGINT:log.syslog.priority:long}>)%{NONNEGINT} (?:(?:%{SYSLOGTIMESTAMP:_tmp.timestamp}|%{TIMESTAMP_ISO8601:_tmp.timestamp8601})) "
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

            let _cond = { event.has_value("_tmp.timestamp8601") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp8601") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp8601".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("_tmp.timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(
                        &date_str,
                        &["MMM  d HH:mm:ss", "MMM dd HH:mm:ss"],
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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.remove("event.original");
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("observer.ip")
                    && !(event.get("observer.ip").is_some_and(|v| v.is_array()))
            };
            if _cond {
                if event.has_value("observer.ip") {
                    event.rename("observer.ip", "_tmp.observer")?;
                }
            }

            let _cond = { event.has_value("_tmp.observer") && !event.has_value("observer.ip") };
            if _cond {
                event.append(
                    "observer.ip",
                    json!(
                        event
                            .get("_tmp.observer")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("cef.extensions.categoryOutcome") == Some("/Success") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("cef.extensions.categoryOutcome") == Some("/Failure") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            event.remove("cef.extensions._cefVer");
            event.remove("_tmp");

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
                event.remove("_tmp");
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
