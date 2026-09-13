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

            let _cond = { event.get_str("input.type") == Some("http_endpoint") };
            if _cond {
                event.remove("json");
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                parse_json_field(
                    event,
                    "event.original",
                    "zscaler_zia.saas_security_activity",
                )?;
            }

            let _cond = {
                event.has_value("zscaler_zia.saas_security_activity")
                    && event.get_bool("_conf.strict_fields") == Some(true)
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: if (ctx.zscaler_zia.saas_security_activity.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message ?: [];\n  ctx.error.message.add('template version mismatch: ' + (ctx.zscaler_zia.saas_security_activity.version == null ? 'null' : ctx.zscaler_zia.saas_security_activity.version.toString()) + ' is not expected version (see ' + params.data_stream + ' https://epr.elastic.co/package/zscaler_zia/' + params.pkg_version + '/docs/README.md)');\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"if (ctx.zscaler_zia.saas_security_activity.version != params.expect.version) {\n  ctx.error = ctx.error ?: [:];\n  ctx.error.message = ctx.error.message ?: [];\n  ctx.error.message.add('template version mismatch: ' + (ctx.zscaler_zia.saas_security_activity.version == null ? 'null' : ctx.zscaler_zia.saas_security_activity.version.toString()) + ' is not expected version (see ' + params.data_stream + ' https://epr.elastic.co/package/zscaler_zia/' + params.pkg_version + '/docs/README.md)');\n}"#
                        ),
                        cached_params!(
                            "{\"pkg_version\":\"3.21.0\",\"data_stream\":\"saas_security_activity\",\"expect\":{\"version\":\"v1\"}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "check_template_version")?;
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("zscaler_zia.saas_security_activity.activity.count") {
                    if let Some(val) =
                        event.get("zscaler_zia.saas_security_activity.activity.count")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "zscaler_zia.saas_security_activity.activity.count".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "zscaler_zia.saas_security_activity.activity.count",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_activity_count_to_long",
                )?;
                event.remove("zscaler_zia.saas_security_activity.activity.count");
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

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("zscaler_zia.saas_security_activity.src_ip") {
                    if let Some(val) = event.get("zscaler_zia.saas_security_activity.src_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "zscaler_zia.saas_security_activity.src_ip".into(),
                                message,
                            }
                        })?;
                        event.set("zscaler_zia.saas_security_activity.src_ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_src_ip_to_ip")?;
                event.remove("zscaler_zia.saas_security_activity.src_ip");
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

            let _cond = {
                event
                    .get("zscaler_zia.saas_security_activity.is_admin")
                    .filter(|v| !v.is_null())
                    .map(painless_to_string)
                    .is_some_and(|s| ["1", "true", "yes"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                event.set("zscaler_zia.saas_security_activity.is_admin", json!(true))?;
            }

            let _cond = {
                event
                    .get("zscaler_zia.saas_security_activity.is_admin")
                    .filter(|v| !v.is_null())
                    .map(painless_to_string)
                    .is_some_and(|s| ["0", "false", "no"].contains(&s.to_lowercase().as_str()))
            };
            if _cond {
                event.set("zscaler_zia.saas_security_activity.is_admin", json!(false))?;
            }

            let _cond = {
                event.has_value("zscaler_zia.saas_security_activity.is_admin")
                    && !(event
                        .get("zscaler_zia.saas_security_activity.is_admin")
                        .is_some_and(|v| v.is_boolean()))
            };
            if _cond {
                event.remove("zscaler_zia.saas_security_activity.is_admin");
            }

            let _cond = {
                event.has_value("zscaler_zia.saas_security_activity.time")
                    && event.get_str("zscaler_zia.saas_security_activity.time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("zscaler_zia.saas_security_activity.time")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-MM-dd HH:mm:ss",
                            ],
                            event.get_str("zscaler_zia.saas_security_activity.tz"),
                            None,
                        ) {
                            Some(parsed) => {
                                event.set("zscaler_zia.saas_security_activity.time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "zscaler_zia.saas_security_activity.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_time")?;
                    event.remove("zscaler_zia.saas_security_activity.time");
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
                event.has_value("zscaler_zia.saas_security_activity.event_time")
                    && event.get_str("zscaler_zia.saas_security_activity.event_time") != Some("")
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("zscaler_zia.saas_security_activity.event_time")
                    {
                        match parse_date_out(
                            &date_str,
                            &[
                                "E MMM dd HH:mm:ss yyyy",
                                "E MMM  d HH:mm:ss yyyy",
                                "E MMM d HH:mm:ss yyyy",
                                "yyyy-MM-dd HH:mm:ss",
                            ],
                            event.get_str("zscaler_zia.saas_security_activity.tz"),
                            None,
                        ) {
                            Some(parsed) => event
                                .set("zscaler_zia.saas_security_activity.event_time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "zscaler_zia.saas_security_activity.event_time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_event_time")?;
                    event.remove("zscaler_zia.saas_security_activity.event_time");
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

            if let Some(v) = event
                .get("zscaler_zia.saas_security_activity.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            event.set("event.kind", json!("event"))?;

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                // Painless script
                // Source: def at = ctx.zscaler_zia?.saas_security_activity?.activity?.type;\ndef m = at != null ? params.mapping.get(at) : null;\nctx.event = ctx.event ?: [:];\nif (m != null) {\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n  if (m.outcome != null) {\n    ctx.event.outcome = m.outcome;\n  }\n} else {\n  ctx.event.category = ['iam'];\n  ctx.event.type = ['info'];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def at = ctx.zscaler_zia?.saas_security_activity?.activity?.type;\ndef m = at != null ? params.mapping.get(at) : null;\nctx.event = ctx.event ?: [:];\nif (m != null) {\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n  if (m.outcome != null) {\n    ctx.event.outcome = m.outcome;\n  }\n} else {\n  ctx.event.category = ['iam'];\n  ctx.event.type = ['info'];\n}"#
                    ),
                    cached_params!(
                        "{\"mapping\":{\"Accept\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Add\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"Approve\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Assigned To User\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"BC HW is not registered\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"Blocked\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Change\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Collaboration Accepted\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Collaboration Change\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Collaboration Revoked\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Create\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"DLP Sensitive Email Sent\":{\"category\":[\"email\"],\"type\":[\"info\"]},\"Delete\":{\"category\":[\"iam\"],\"type\":[\"deletion\"]},\"Deny Access\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Detected\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"Disable\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Download\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"Download Sync\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"Drop\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Edit\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Email With Malware Received\":{\"category\":[\"email\",\"malware\"],\"type\":[\"info\"]},\"Enable\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Fork\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"Join\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"Login Fail\":{\"category\":[\"authentication\",\"iam\"],\"type\":[\"start\",\"info\"],\"outcome\":\"failure\"},\"Login Pass\":{\"category\":[\"authentication\",\"iam\"],\"type\":[\"start\",\"info\"],\"outcome\":\"success\"},\"Logout Compromised\":{\"category\":[\"authentication\",\"iam\"],\"type\":[\"end\",\"info\"],\"outcome\":\"failure\"},\"Move\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Other\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"Pull request\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"Reactivate\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"Reject\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Release\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Remove\":{\"category\":[\"iam\"],\"type\":[\"deletion\"]},\"Reset\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Resource Granted\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Restore\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Restrict\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Revoked\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Risky Download\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"Risky Share\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"Risky Upload\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Share\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"Suspend\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"Transfer\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Unassign\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"Unsuspend\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"Unverify\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Update\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Upload\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Upload Sync\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"User/App Deactivate\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"Verify\":{\"category\":[\"iam\"],\"type\":[\"change\"]}}}"
                    ),
                )?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "script")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "script_set_event_category_type_from_activity_type",
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

            if let Some(v) = event
                .get("zscaler_zia.saas_security_activity.tz")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.timezone", v)?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security_activity.activity.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            event.set("event.provider", json!("Zscaler"))?;

            event.set("observer.vendor", json!("Zscaler"))?;

            event.set("observer.product", json!("Zscaler ZIA"))?;

            if let Some(v) = event
                .get("zscaler_zia.saas_security_activity.src_ip")
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

            let _cond = { event.has_value("source.ip") };
            if _cond {
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
            }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if let Some(v) = event
                .get("zscaler_zia.saas_security_activity.user_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            let _cond = {
                event.has_value("zscaler_zia.saas_security_activity.user_name")
                    && event
                        .get("zscaler_zia.saas_security_activity.user_name")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("@"))
                            }
                            serde_json::Value::String(s) => s.contains("@"),
                            _ => false,
                        })
            };
            if _cond {
                if let Some(v) = event
                    .get("zscaler_zia.saas_security_activity.user_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event.has_value("zscaler_zia.saas_security_activity.user_name")
                    && event
                        .get("zscaler_zia.saas_security_activity.user_name")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("@"))
                            }
                            serde_json::Value::String(s) => s.contains("@"),
                            _ => false,
                        })
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) =
                        event.get_string("zscaler_zia.saas_security_activity.user_name")
                    {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("@") else {
                                break 'dissect false;
                            };
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("@") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("user.domain", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        } else {
                            return Err(TransformError::ParseError {
                                path: "zscaler_zia.saas_security_activity.user_name".into(),
                                message: "dissect pattern did not match".into(),
                            });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "dissect")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "dissect_user_name_to_user_domain",
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

            let _cond = { event.has_value("zscaler_zia.saas_security_activity.user_name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("zscaler_zia.saas_security_activity.user_name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.saas_security_activity.external_owner") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("zscaler_zia.saas_security_activity.external_owner")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("zscaler_zia.saas_security_activity.src_ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("zscaler_zia.saas_security_activity.src_ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("zscaler_zia.saas_security_activity.time");
            event.remove("zscaler_zia.saas_security_activity.user_name");
            event.remove("zscaler_zia.saas_security_activity.src_ip");
            event.remove("zscaler_zia.saas_security_activity.tz");
            event.remove("_conf");

            // Painless script, resolved to its runners at generation time
            // Source: boolean dropScalar(Object v) {\n  return v == null || v == '' || v == '0' || v == 'N/A'\n    || v == 'None' || v == 'Unknown' || v == 'Unknown Host' || v == 'Unknown URL';\n}\nvoid handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap((Map) v);\n    } else if (v instanceof List) {\n      handleList((List) v);\n    }\n    return dropScalar(v)\n      || (v instanceof Map && ((Map) v).size() == 0)\n      || (v instanceof List && ((List) v).size() == 0);\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap((Map) v);\n    } else if (v instanceof List) {\n      handleList((List) v);\n    }\n    return dropScalar(v)\n      || (v instanceof Map && ((Map) v).size() == 0)\n      || (v instanceof List && ((List) v).size() == 0);\n  });\n}\nhandleMap(ctx);
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec![
                        "0".into(),
                        "N/A".into(),
                        "None".into(),
                        "Unknown".into(),
                        "Unknown Host".into(),
                        "Unknown URL".into(),
                    ],
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
