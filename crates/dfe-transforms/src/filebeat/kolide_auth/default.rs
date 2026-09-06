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
            event.set("ecs.version", json!("9.4.0"))?;

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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

            let _cond = { event.get("event").is_some_and(|v| v.is_string()) };
            if _cond {
                if event.has_value("event") {
                    event.rename("event", "json.event")?;
                }
            }

            let _cond = { event.has_value("json.event") && event.has_value("data") };
            if _cond {
                // Painless script
                // Source: if (ctx.containsKey('id')) { ctx.json.id = ctx.remove('id'); }\nif (ctx.containsKey('timestamp')) { ctx.json.timestamp = ctx.remove('timestamp'); }\nif (ctx.containsKey('data')) { ctx.json.data = ctx.remove('data'); }
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.containsKey('id')) { ctx.json.id = ctx.remove('id'); }\nif (ctx.containsKey('timestamp')) { ctx.json.timestamp = ctx.remove('timestamp'); }\nif (ctx.containsKey('data')) { ctx.json.data = ctx.remove('data'); }"#
                    ),
                )?;
            }

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

            let _cond = { !event.has_value("json") && event.has_value("event.original") };
            if _cond {
                parse_json_field(event, "event.original", "json")?;
            }

            let _cond = { event.get_str("json.type") == Some("auth_log") };
            if _cond {
                // Begin nested pipeline: "s3"
                let _cond = { event.get("json.data").is_some_and(|v| v.is_object()) };
                if _cond {
                    // Painless script
                    // Source: Map data = (Map) ctx.json.remove('data');\nfor (def entry : data.entrySet()) {\n  ctx.json[entry.getKey()] = entry.getValue();\n}\nctx.json.remove('type');
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"Map data = (Map) ctx.json.remove('data');\nfor (def entry : data.entrySet()) {\n  ctx.json[entry.getKey()] = entry.getValue();\n}\nctx.json.remove('type');"#
                        ),
                    )?;
                }
                let _cond = { !event.has_value("json.sub_event_type") };
                if _cond {
                    if event.has_value("json.auth_event_type") {
                        event.rename("json.auth_event_type", "json.sub_event_type")?;
                    }
                }
                let _cond = { !event.has_value("json.id") };
                if _cond {
                    if let Some(v) = event
                        .get("json.request_id")
                        .filter(|v| !painless_is_empty_value(v))
                        .cloned()
                    {
                        event.set("json.id", v)?;
                    }
                }
                event.remove("json.request_id");
                let _cond = { event.get_bool("json.succeeded") == Some(true) };
                if _cond {
                    event.set("json.result", json!("Success"))?;
                }
                let _cond = { event.get_bool("json.succeeded") == Some(false) };
                if _cond {
                    event.set("json.result", json!("Fail"))?;
                }
                event.remove("json.succeeded");
                event.remove("json.authentication_events");
                // End nested pipeline: "s3"
            }

            let _cond = { event.has_value("json.timestamp") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("json.session_timestamp")
                    && event.get_str("json.session_timestamp") != Some("")
            };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.session_timestamp") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("event.start", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.session_timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("json.sub_event_type")
                    && event.get_str("json.sub_event_type") != Some("")
            };
            if _cond {
                event.set(
                    "event.id",
                    json!(format!(
                        "{}:{}:{}",
                        event
                            .get("json.id")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("json.sub_event_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("json.timestamp")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            let _cond = {
                !event.has_value("json.event")
                    && !event.has_value("kolide.auth.session_id")
                    && event.has_value("json.id")
                    && event.get_str("json.id") != Some("")
            };
            if _cond {
                if let Some(v) = event
                    .get("json.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("kolide.auth.session_id", v)?;
                }
            }

            let _cond = { !event.has_value("event.id") };
            if _cond {
                if event.has_value("json.id") {
                    event.rename("json.id", "event.id")?;
                }
            }

            let _cond = { event.has_value("json.event") };
            if _cond {
                // Begin nested pipeline: "webhook"
                let _cond = { !event.has_value("event.action") };
                if _cond {
                    if event.has_value("json.event") {
                        event.rename("json.event", "event.action")?;
                    }
                }
                let _cond = { event.get_str("event.action") == Some("auth_logs.success") };
                if _cond {
                    event.set("event.outcome", json!("success"))?;
                }
                let _cond = { event.get_str("event.action") == Some("auth_logs.failure") };
                if _cond {
                    event.set("event.outcome", json!("failure"))?;
                }
                if event.has_value("json.data.person_id") {
                    if let Some(val) = event.get("json.data.person_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.person_id".into(),
                                message,
                            }
                        })?;
                        event.set("user.id", converted)?;
                    }
                }
                let _cond = { !event.has_value("host.name") };
                if _cond {
                    if event.has_value("json.data.device_name") {
                        event.rename("json.data.device_name", "host.name")?;
                    }
                }
                if event.has_value("json.data.device_id") {
                    if let Some(val) = event.get("json.data.device_id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.device_id".into(),
                                message,
                            }
                        })?;
                        event.set("host.id", converted)?;
                    }
                }
                let _cond = { !event.has_value("user_agent.name") };
                if _cond {
                    if event.has_value("json.data.browser_name") {
                        event.rename("json.data.browser_name", "user_agent.name")?;
                    }
                }
                let _cond = { !event.has_value("user_agent.version") };
                if _cond {
                    if event.has_value("json.data.browser_version") {
                        event.rename("json.data.browser_version", "user_agent.version")?;
                    }
                }
                let _cond = { !event.has_value("kolide.auth.agent_version") };
                if _cond {
                    if event.has_value("json.data.launcher_version") {
                        event.rename("json.data.launcher_version", "kolide.auth.agent_version")?;
                    }
                }
                let _cond = {
                    event.has_value("json.data.auth_log_url")
                        && event.get_str("json.data.auth_log_url") != Some("")
                };
                if _cond {
                    if let Some(input) = event.get_string("json.data.auth_log_url") {
                        // Grok pattern: ^%{NOTSPACE}/auth_logs/%{DATA:kolide.auth.session_id}$
                        if !cached_grok!("^%{NOTSPACE}/auth_logs/%{DATA:kolide.auth.session_id}$")
                            .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                }
                let _cond = { !event.has_value("kolide.auth.url") };
                if _cond {
                    if event.has_value("json.data.auth_log_url") {
                        event.rename("json.data.auth_log_url", "kolide.auth.url")?;
                    }
                }
                // End nested pipeline: "webhook"
            }

            let _cond = {
                !event.has_value("json.event")
                    && event.has_value("json.sub_event_type")
                    && event.get_str("json.sub_event_type") != Some("")
            };
            if _cond {
                if let Some(v) = event
                    .get("json.sub_event_type")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
            }

            let _cond = { !event.has_value("json.event") && !event.has_value("event.action") };
            if _cond {
                event.set("event.action", json!("auth_log"))?;
            }

            let _cond = {
                !event.has_value("json.event")
                    && event.has_value("json.sub_event_description")
                    && event.get_str("json.sub_event_description") != Some("")
            };
            if _cond {
                if let Some(v) = event
                    .get("json.sub_event_description")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("message", v)?;
                }
            }

            let _cond = {
                !event.has_value("json.event") && event.get_str("json.result") == Some("Success")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond =
                { !event.has_value("json.event") && event.get_str("json.result") == Some("Fail") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                if event.has_value("json.person_name") {
                    event.rename("json.person_name", "user.name")?;
                }
            }

            let _cond = { !event.has_value("user.email") };
            if _cond {
                if event.has_value("json.person_email") {
                    event.rename("json.person_email", "user.email")?;
                }
            }

            let _cond = { !event.has_value("user.id") };
            if _cond {
                if event.has_value("json.person_info.identifier") {
                    event.rename("json.person_info.identifier", "user.id")?;
                }
            }

            let _cond = { !event.has_value("host.id") };
            if _cond {
                if event.has_value("json.device_info.identifier") {
                    event.rename("json.device_info.identifier", "host.id")?;
                }
            }

            let _cond = { !event.has_value("source.ip") };
            if _cond {
                if event.has_value("json.ip_address") {
                    event.rename("json.ip_address", "source.ip")?;
                }
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
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

            let _cond = { !event.has_value("source.as.number") };
            if _cond {
                if event.has_value("source.as.asn") {
                    event.rename("source.as.asn", "source.as.number")?;
                }
            }

            let _cond = { !event.has_value("source.as.organization.name") };
            if _cond {
                if event.has_value("source.as.organization_name") {
                    event.rename("source.as.organization_name", "source.as.organization.name")?;
                }
            }

            let _cond = { !event.has_value("source.geo.country_name") };
            if _cond {
                if event.has_value("json.country") {
                    event.rename("json.country", "source.geo.country_name")?;
                }
            }

            event.remove("json.country");

            let _cond = { !event.has_value("source.geo.city_name") };
            if _cond {
                if event.has_value("json.city") {
                    event.rename("json.city", "source.geo.city_name")?;
                }
            }

            event.remove("json.city");

            let _cond = { !event.has_value("user_agent.name") };
            if _cond {
                if event.has_value("json.browser_name") {
                    event.rename("json.browser_name", "user_agent.name")?;
                }
            }

            let _cond = { !event.has_value("user_agent.original") };
            if _cond {
                if event.has_value("json.browser_user_agent") {
                    event.rename("json.browser_user_agent", "user_agent.original")?;
                }
            }

            let _cond = { event.has_value("user_agent.original") };
            if _cond {
                if event.has_value("user_agent.original") {
                    if let Some(ua_str) = event.get_string("user_agent.original") {
                        let ua_str = ua_str.to_string();
                        // User agent parsing
                        if let Ok(ua) = parse_user_agent(&ua_str) {
                            event.set("user_agent.original", json!(ua_str))?;
                            if let Some(name) = ua.name {
                                event.set("user_agent.name", json!(name))?;
                            }
                            if let Some(version) = ua.version {
                                event.set("user_agent.version", json!(version))?;
                            }
                            if let Some(os_name) = ua.os_name {
                                event.set("user_agent.os.name", json!(os_name))?;
                                if let Some(os_version) = ua.os_version {
                                    event.set("user_agent.os.version", json!(os_version))?;
                                    event.set(
                                        "user_agent.os.full",
                                        json!(format!("{} {}", os_name, os_version)),
                                    )?;
                                }
                            }
                            if let Some(device) = ua.device {
                                event.set("user_agent.device.name", json!(device))?;
                            }
                        }
                    }
                }
            }

            let _cond = {
                event.has_value("json.agent_version")
                    && !event.has_value("kolide.auth.agent_version")
            };
            if _cond {
                if event.has_value("json.agent_version") {
                    event.rename("json.agent_version", "kolide.auth.agent_version")?;
                }
            }

            let _cond = { !event.has_value("kolide.auth.result") };
            if _cond {
                if event.has_value("json.result") {
                    event.rename("json.result", "kolide.auth.result")?;
                }
            }

            let _cond = { !event.has_value("kolide.auth.initial_status") };
            if _cond {
                if event.has_value("json.initial_status") {
                    event.rename("json.initial_status", "kolide.auth.initial_status")?;
                }
            }

            let _cond = {
                event.has_value("json.okta_app_name")
                    && !event.has_value("kolide.auth.okta.app_name")
            };
            if _cond {
                if event.has_value("json.okta_app_name") {
                    event.rename("json.okta_app_name", "kolide.auth.okta.app_name")?;
                }
            }

            let _cond = {
                event.has_value("json.okta_app_instance_id")
                    && !event.has_value("kolide.auth.okta.app_instance_id")
            };
            if _cond {
                if event.has_value("json.okta_app_instance_id") {
                    event.rename(
                        "json.okta_app_instance_id",
                        "kolide.auth.okta.app_instance_id",
                    )?;
                }
            }

            let _cond = { !event.has_value("kolide.auth.issues_displayed") };
            if _cond {
                if event.has_value("json.issues_displayed") {
                    event.rename("json.issues_displayed", "kolide.auth.issues_displayed")?;
                }
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                if let Some(v) = event.get("host.name").cloned() {
                    event.set("host.hostname", v)?;
                }
            }

            let _cond = { event.has_value("source.ip") };
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

            let _cond = { event.has_value("user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("host.name") };
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

            // Begin nested pipeline: "extended-mappings"
            let _cond = { event.get("json.events").is_some_and(|v| v.is_array()) };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    // Painless script
                    // Source: def prefix = 'Downloaded package: ';\ndef packages = new ArrayList();\nfor (def ev : ctx.json.events) {\n  def desc = ev?.event_description;\n  if (desc instanceof String && desc.startsWith(prefix)) {\n    packages.add(desc.substring(prefix.length()));\n  }\n}\nif (!packages.isEmpty()) {\n  if (ctx.kolide.auth.downloaded_packages == null) {\n    ctx.kolide.auth.downloaded_packages = packages;\n  } else {\n    ctx.kolide.auth.downloaded_packages.addAll(packages);\n  }\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"def prefix = 'Downloaded package: ';\ndef packages = new ArrayList();\nfor (def ev : ctx.json.events) {\n  def desc = ev?.event_description;\n  if (desc instanceof String && desc.startsWith(prefix)) {\n    packages.add(desc.substring(prefix.length()));\n  }\n}\nif (!packages.isEmpty()) {\n  if (ctx.kolide.auth.downloaded_packages == null) {\n    ctx.kolide.auth.downloaded_packages = packages;\n  } else {\n    ctx.kolide.auth.downloaded_packages.addAll(packages);\n  }\n}"#
                        ),
                    )?;
                    Ok(())
                })();
            }
            // End nested pipeline: "extended-mappings"

            // Begin nested pipeline: "categorize"
            let _cond = { event.has_value("event.action") };
            if _cond {
                // Painless script
                // Source: def action = ctx.event.action;\nctx.event.kind = 'event';\n\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n  if (m.containsKey('outcome') && ctx.event.outcome == null) {\n    ctx.event.outcome = m.outcome;\n  }\n} else {\n  ctx.event.category = ['authentication'];\n  ctx.event.type = ['info'];\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def action = ctx.event.action;\nctx.event.kind = 'event';\n\ndef m = params.exact.get(action);\nif (m != null) {\n  ctx.event.category = new ArrayList(m.category);\n  ctx.event.type = new ArrayList(m.type);\n  if (m.containsKey('outcome') && ctx.event.outcome == null) {\n    ctx.event.outcome = m.outcome;\n  }\n} else {\n  ctx.event.category = ['authentication'];\n  ctx.event.type = ['info'];\n}"#
                    ),
                    cached_params!(
                        "{\"exact\":{\"sign_in_attempt\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"start\"]},\"sign_in_success\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"start\"],\"outcome\":\"success\"},\"auth_logs.success\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"start\"],\"outcome\":\"success\"},\"auth_logs.failure\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"start\"],\"outcome\":\"failure\"},\"auth_session_summary\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"agent_detection_success\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"agent_detection_failure\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"mobile_agent_detection_success\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"agent_download_request\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"device_registration_request\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"device_registration_successful\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"device_registration_blocked\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"device_blocked\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"push_notification_sent\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"auth_log\":{\"category\":[\"authentication\"],\"type\":[\"info\"]}}}"
                    ),
                )?;
            }
            let _cond = {
                event.get("json.events").is_some_and(|v| v.is_array()) && event.get("json.events").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } > 0) && event.has_value("json.timestamp")
            };
            if _cond {
                // Painless script
                // Source: def evs = ctx.json.events;\ndef ts = ctx.json.timestamp;\nint idx = -1;\nfor (int i = 0; i < evs.size(); i++) {\n  if (evs[i] instanceof Map && evs[i].timestamp == ts) { idx = i; break; }\n}\nif (idx != -1) {\n  if (ctx.event == null) { ctx.event = [:]; }\n  if (idx == evs.size() - 1) {\n    ctx.event.type = ['end'];\n  } else if (idx == 0) {\n    ctx.event.type = ['start'];\n  } else {\n    ctx.event.type = ['info'];\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def evs = ctx.json.events;\ndef ts = ctx.json.timestamp;\nint idx = -1;\nfor (int i = 0; i < evs.size(); i++) {\n  if (evs[i] instanceof Map && evs[i].timestamp == ts) { idx = i; break; }\n}\nif (idx != -1) {\n  if (ctx.event == null) { ctx.event = [:]; }\n  if (idx == evs.size() - 1) {\n    ctx.event.type = ['end'];\n  } else if (idx == 0) {\n    ctx.event.type = ['start'];\n  } else {\n    ctx.event.type = ['info'];\n  }\n}"#
                    ),
                )?;
            }
            // End nested pipeline: "categorize"

            let _cond = { event.has_value("event.id") && event.get_str("event.id") != Some("") };
            if _cond {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("event.id") {
                        values.push(v.clone());
                    } else {
                        return Err(TransformError::FieldNotFound {
                            path: "event.id".into(),
                        });
                    }
                    if !values.is_empty() {
                        event.set(
                            "_id",
                            json!(fingerprint_with(&values, "SHA-256", "").map_err(|message| {
                                TransformError::ParseError {
                                    path: "_id".into(),
                                    message,
                                }
                            })?),
                        )?;
                    }
                }
            }

            event.remove("json");
            event.remove("data");

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

            // Painless script, resolved to its runners at generation time
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
