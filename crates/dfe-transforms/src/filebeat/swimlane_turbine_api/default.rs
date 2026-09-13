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
            event.remove("stream");
            event.remove("log");

            if event.has_value("json") {
                event.rename("json", "swimlane.audit_log")?;
            }

            let _cond = { event.get_str("swimlane.audit_log.LogType") != Some("Audit") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = { event.has_value("swimlane.audit_log.NewValue") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    parse_json_field(
                        event,
                        "swimlane.audit_log.NewValue",
                        "swimlane.audit_log.NewValue",
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("swimlane.audit_log.User") {
                event.rename("swimlane.audit_log.User", "user.name")?;
            }

            let _cond = {
                event.has_value("user.name")
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                event.rename("user.name", "user.email")?;
            }

            let _cond = { !event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.has_value("user.email") {
                        if let Some(input) = event.get_string("user.email") {
                            let mut remaining: &str = &input;
                            let mut captured: Vec<(&str, &str)> = Vec::new();
                            let matched = 'dissect: {
                                let Some(pos) = remaining.find("@") else {
                                    break 'dissect false;
                                };
                                captured.push(("user.name", &remaining[..pos]));
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
                            }
                        }
                    }
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.email") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.email")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if event.has_value("swimlane.audit_log.AccountId") {
                event.rename("swimlane.audit_log.AccountId", "cloud.origin.account.id")?;
            }

            if event.has_value("swimlane.audit_log.TenantId") {
                event.rename("swimlane.audit_log.TenantId", "cloud.origin.project.id")?;
            }

            if event.has_value("swimlane.audit_log.LogLevel") {
                event.rename("swimlane.audit_log.LogLevel", "log.level")?;
            }

            if event.has_value("swimlane.audit_log.Message") {
                event.rename("swimlane.audit_log.Message", "message")?;
            }

            // Painless script
            // Source: def src = ctx.swimlane?.audit_log?.SourceIp; if (src != null) {\n    if (src instanceof List && src.size() > 0) {\n        ctx.swimlane.audit_log.SourceIp = src[0];\n    } else if (src instanceof String) {\n        ctx.swimlane.audit_log.SourceIp = src;\n    }\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def src = ctx.swimlane?.audit_log?.SourceIp; if (src != null) {\n    if (src instanceof List && src.size() > 0) {\n        ctx.swimlane.audit_log.SourceIp = src[0];\n    } else if (src instanceof String) {\n        ctx.swimlane.audit_log.SourceIp = src;\n    }\n}"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("swimlane.audit_log.SourceIp") {
                    // Grok pattern: ::ffff:%{IPV4:source.ip}
                    // Grok pattern: %{IPV4:source.ip}
                    // Grok pattern: %{IPV6:source.ip}
                    if !extract_first_match(
                        &[
                            cached_grok!("::ffff:%{IPV4:source.ip}"),
                            cached_grok!("%{IPV4:source.ip}"),
                            cached_grok!("%{IPV6:source.ip}"),
                        ],
                        &input,
                        event,
                    )? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
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

            if event.has_value("swimlane.audit_log.Tenant") {
                event.rename("swimlane.audit_log.Tenant", "cloud.origin.tenant.id")?;
            }

            if event.has_value("swimlane.audit_log.LogCategory") {
                event.rename("swimlane.audit_log.LogCategory", "log.category")?;
            }

            if event.has_value("swimlane.audit_log.LogFeatureCategory") {
                event.rename(
                    "swimlane.audit_log.LogFeatureCategory",
                    "log.feature_category",
                )?;
            }

            if event.has_value("swimlane.audit_log.LogSource") {
                event.rename("swimlane.audit_log.LogSource", "log.source.type")?;
            }

            if event.has_value("swimlane.audit_log.LogType") {
                event.rename("swimlane.audit_log.LogType", "log.type")?;
            }

            if event.has_value("swimlane.audit_log.Description") {
                event.rename("swimlane.audit_log.Description", "message")?;
            }

            let _cond = {
                event.has_value("message")
                    && event.get_str("message") != Some("")
                    && event.has_value("swimlane.audit_log.NewValue.name")
                    && event.has_value("swimlane.audit_log.NewValue.id")
            };
            if _cond {
                event.set(
                    "message",
                    json!(format!(
                        "{}: {} with Id: {}",
                        event
                            .get("message")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("swimlane.audit_log.NewValue.name")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("swimlane.audit_log.NewValue.id")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
            }

            if event.has_value("swimlane.audit_log.UserId") {
                event.rename("swimlane.audit_log.UserId", "user.id")?;
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

            if event.has_value("swimlane.audit_log.Category") {
                event.rename("swimlane.audit_log.Category", "log.category")?;
            }

            if event.has_value("swimlane.audit_log.Endpoint") {
                event.rename("swimlane.audit_log.Endpoint", "url.path")?;
            }

            if event.has_value("swimlane.audit_log.EventOutcome") {
                event.rename("swimlane.audit_log.EventOutcome", "event.outcome")?;
            }

            if event.has_value("event.outcome") {
                map_strings(event, "event.outcome", "event.outcome", str::to_lowercase)?;
            }

            let _cond = { event.get_str("swimlane.audit_log.isAdmin") == Some("True") };
            if _cond {
                event.set("user.roles", Value::Array(vec![json!("administrator")]))?;
            }

            if event.has_value("swimlane.audit_log.AuthenticationType") {
                event.rename(
                    "swimlane.audit_log.AuthenticationType",
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

            if event.has_value("swimlane.audit_log.NewValue.Id") {
                event.rename(
                    "swimlane.audit_log.NewValue.Id",
                    "destination.user.changes.id",
                )?;
            }

            if event.has_value("swimlane.audit_log.NewValue.Name") {
                event.rename(
                    "swimlane.audit_log.NewValue.Name",
                    "destination.user.changes.name",
                )?;
            }

            if event.has_value("swimlane.audit_log.NewValue.ModifiedByUser") {
                event.rename(
                    "swimlane.audit_log.NewValue.ModifiedByUser",
                    "source.user.changes.id",
                )?;
            }

            if event.has_value("swimlane.audit_log.UserAgent") {
                if let Some(ua_str) = event.get_string("swimlane.audit_log.UserAgent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
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

            if let Some(date_str) = event.get_as_string("swimlane.audit_log.EventTime") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "swimlane.audit_log.EventTime".into(),
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
