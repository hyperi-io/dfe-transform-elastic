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
            if let Some(v) = event.get("message").cloned() {
                event.set("event.original", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "message", "json")?;
                Ok(())
            })();

            let _cond =
                { !event.has_value("json") || !(event.get("json").is_some_and(|v| v.is_object())) };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            event.remove("message");
            event.remove("input");
            event.remove("log");

            if event.has_value("json") {
                event.rename("json", "swimlane.audit_log")?;
            }

            let _cond = {
                event.has_value("swimlane.audit_log.description")
                    && (event
                        .get_str("swimlane.audit_log.description")
                        .is_some_and(|s| s.to_lowercase().contains("read"))
                        || event
                            .get_str("swimlane.audit_log.description")
                            .is_some_and(|s| s.to_lowercase().contains("created a new record")))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = { event.has_value("swimlane.audit_log.newValue") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "swimlane.audit_log.newValue",
                        "swimlane.audit_log.newValue",
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("swimlane.audit_log.user") {
                event.rename("swimlane.audit_log.user", "user.name")?;
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

            if event.has_value("swimlane.audit_log.accountId") {
                event.rename("swimlane.audit_log.accountId", "cloud.origin.account.id")?;
            }

            if event.has_value("swimlane.audit_log.tenantId") {
                event.rename("swimlane.audit_log.tenantId", "cloud.origin.project.id")?;
            }

            if event.has_value("swimlane.audit_log.logLevel") {
                event.rename("swimlane.audit_log.logLevel", "log.level")?;
            }

            if event.has_value("swimlane.audit_log.path") {
                event.rename("swimlane.audit_log.path", "url.path")?;
            }

            // Painless script
            // Source: def src = ctx.swimlane?.audit_log?.sourceIp; if (src != null) {\n    if (src instanceof List && src.size() > 0) {\n        ctx.swimlane.audit_log.sourceIp = src[0];\n    } else if (src instanceof String) {\n        ctx.swimlane.audit_log.sourceIp = src;\n    }\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def src = ctx.swimlane?.audit_log?.sourceIp; if (src != null) {\n    if (src instanceof List && src.size() > 0) {\n        ctx.swimlane.audit_log.sourceIp = src[0];\n    } else if (src instanceof String) {\n        ctx.swimlane.audit_log.sourceIp = src;\n    }\n}"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("swimlane.audit_log.sourceIp") {
                    // Grok pattern: ::ffff:%{IPV4:source.ip}
                    // Grok pattern: %{IPV4:source.ip}
                    // Grok pattern: %{IPV6:source.ip}
                    let _ = extract_first_match(
                        &[
                            cached_grok!("::ffff:%{IPV4:source.ip}"),
                            cached_grok!("%{IPV4:source.ip}"),
                            cached_grok!("%{IPV6:source.ip}"),
                        ],
                        &input,
                        event,
                    )?;
                }
                Ok(())
            })();

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

            if event.has_value("swimlane.audit_log.tenant") {
                event.rename("swimlane.audit_log.tenant", "cloud.origin.tenant.id")?;
            }

            if event.has_value("swimlane.audit_log.category") {
                event.rename("swimlane.audit_log.category", "log.category")?;
            }

            if event.has_value("swimlane.audit_log.logFeatureCategory") {
                event.rename(
                    "swimlane.audit_log.logFeatureCategory",
                    "log.feature_category",
                )?;
            }

            if event.has_value("swimlane.audit_log.logSource") {
                event.rename("swimlane.audit_log.logSource", "log.source.type")?;
            }

            if event.has_value("swimlane.audit_log.logType") {
                event.rename("swimlane.audit_log.logType", "log.type")?;
            }

            if event.has_value("swimlane.audit_log.description") {
                event.rename("swimlane.audit_log.description", "message")?;
            }

            let _cond = { event.has_value("message") && event.get_str("message") != Some("") };
            if _cond {
                event.set(
                    "message",
                    json!(
                        event
                            .get("message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("swimlane.audit_log.userId") {
                event.rename("swimlane.audit_log.userId", "user.id")?;
            }

            // Painless script
            // Source: ctx.event.kind = 'event'; ctx.event.type = ['info']; if (ctx.swimlane?.audit_log?.ActionType == null) {\n    return;\n} if (params.get(ctx.swimlane?.audit_log?.ActionType) == null) {\n    return;\n} def hm = new HashMap(params.get(ctx.swimlane?.audit_log?.ActionType)); hm.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"ctx.event.kind = 'event'; ctx.event.type = ['info']; if (ctx.swimlane?.audit_log?.ActionType == null) {\n    return;\n} if (params.get(ctx.swimlane?.audit_log?.ActionType) == null) {\n    return;\n} def hm = new HashMap(params.get(ctx.swimlane?.audit_log?.ActionType)); hm.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"Create\":{\"type\":[\"creation\"]},\"Read\":{\"type\":[\"access\"]},\"Update\":{\"type\":[\"change\"]},\"Delete\":{\"type\":[\"deletion\"]},\"Login\":{\"type\":[\"start\"]},\"Logout\":{\"type\":[\"end\"]}}"
                ),
            )?;

            if event.has_value("swimlane.audit_log.endpoint") {
                if let Some(input) = event.get_string("swimlane.audit_log.endpoint") {
                    // Grok pattern: %{GREEDYDATA:url.path}
                    let _ = cached_grok!("%{GREEDYDATA:url.path}").extract_into(&input, event)?;
                }
            }

            let _cond = {
                event.get("url.path").is_some_and(|v| v.is_string())
                    && event
                        .get_str("url.path")
                        .is_some_and(|s| s.to_lowercase().contains("hubs/record"))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = {
                event.get("url.path").is_some_and(|v| v.is_string())
                    && event
                        .get_str("url.path")
                        .is_some_and(|s| s.to_lowercase().ends_with("/values"))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            if event.has_value("swimlane.audit_log.eventOutcome") {
                event.rename("swimlane.audit_log.eventOutcome", "event.outcome")?;
            }

            if event.has_value("event.outcome") {
                map_strings(event, "event.outcome", "event.outcome", str::to_lowercase)?;
            }

            let _cond = { event.get_str("swimlane.audit_log.isAdmin") == Some("True") };
            if _cond {
                event.set("user.roles", Value::Array(vec![json!("administrator")]))?;
            }

            if event.has_value("swimlane.audit_log.authenticationType") {
                event.rename(
                    "swimlane.audit_log.authenticationType",
                    "user.authentication.type",
                )?;
            }

            if event.has_value("user.authentication.type") {
                map_strings(
                    event,
                    "user.authentication.type",
                    "user.authentication.type",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("swimlane.audit_log.userAgent") {
                if let Some(ua_str) = event.get_string("swimlane.audit_log.userAgent") {
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

            if let Some(date_str) = event.get_as_string("swimlane.audit_log.eventTime") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "swimlane.audit_log.eventTime".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.remove("swimlane").is_none() {
                return Err(TransformError::FieldNotFound {
                    path: "swimlane".into(),
                });
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
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
