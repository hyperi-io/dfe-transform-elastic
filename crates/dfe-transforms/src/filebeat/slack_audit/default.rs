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

            parse_json_field(event, "event.original", "slack")?;

            if let Some(date_str) = event.get_as_string("slack.date_create") {
                match parse_date_out(&date_str, &["UNIX"], None, None) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "slack.date_create".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            event.remove("slack.date_create");

            if event.has_value("slack.action") {
                event.rename("slack.action", "event.action")?;
            }

            if event.has_value("slack.id") {
                event.rename("slack.id", "event.id")?;
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("event.id") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("slack.actor.user.id") {
                event.rename("slack.actor.user.id", "user.id")?;
            }

            if event.has_value("slack.actor.user.name") {
                event.rename("slack.actor.user.name", "user.full_name")?;
            }

            if event.has_value("slack.actor.user.email") {
                event.rename("slack.actor.user.email", "user.email")?;
            }

            if event.has_value("slack.actor") {
                event.rename("slack.actor", "slack.audit.actor")?;
            }

            let _cond = { !event.has_value("slack.audit.entity") };
            if _cond {
                if event.has_value("slack.entity.workspace") {
                    event.rename("slack.entity.workspace", "slack.audit.entity")?;
                }
            }

            let _cond = { !event.has_value("slack.audit.entity") };
            if _cond {
                if event.has_value("slack.entity.enterprise") {
                    event.rename("slack.entity.enterprise", "slack.audit.entity")?;
                }
            }

            let _cond = { !event.has_value("slack.audit.entity") };
            if _cond {
                if event.has_value("slack.entity.user") {
                    event.rename("slack.entity.user", "slack.audit.entity")?;
                }
            }

            let _cond = { !event.has_value("slack.audit.entity") };
            if _cond {
                if event.has_value("slack.entity.file") {
                    event.rename("slack.entity.file", "slack.audit.entity")?;
                }
            }

            let _cond = { !event.has_value("slack.audit.entity") };
            if _cond {
                if event.has_value("slack.entity.channel") {
                    event.rename("slack.entity.channel", "slack.audit.entity")?;
                }
            }

            let _cond = { !event.has_value("slack.audit.entity") };
            if _cond {
                if event.has_value("slack.entity.app") {
                    event.rename("slack.entity.app", "slack.audit.entity")?;
                }
            }

            let _cond = { !event.has_value("slack.audit.entity") };
            if _cond {
                if event.has_value("slack.entity.workflow") {
                    event.rename("slack.entity.workflow", "slack.audit.entity")?;
                }
            }

            let _cond = { !event.has_value("slack.audit.entity") };
            if _cond {
                if event.has_value("slack.entity.usergroup") {
                    event.rename("slack.entity.usergroup", "slack.audit.entity")?;
                }
            }

            let _cond = { !event.has_value("slack.audit.entity") };
            if _cond {
                if event.has_value("slack.entity.barrier") {
                    event.rename("slack.entity.barrier", "slack.audit.entity")?;
                }
            }

            let _cond = { !event.has_value("slack.audit.entity") };
            if _cond {
                if event.has_value("slack.entity.message") {
                    event.rename("slack.entity.message", "slack.audit.entity")?;
                }
            }

            let _cond = { !event.has_value("slack.audit.entity") };
            if _cond {
                if event.has_value("slack.entity.role") {
                    event.rename("slack.entity.role", "slack.audit.entity")?;
                }
            }

            let _cond = { !event.has_value("slack.audit.entity") };
            if _cond {
                if event.has_value("slack.entity.account_type_role") {
                    event.rename("slack.entity.account_type_role", "slack.audit.entity")?;
                }
            }

            if event.has_value("slack.entity.type") {
                event.rename("slack.entity.type", "slack.audit.entity.entity_type")?;
            }

            if event.has_value("slack.context.ua") {
                event.rename("slack.context.ua", "user_agent.original")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(ua_str) = event.get_string("user_agent.original") {
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
                Ok(())
            })();

            let _cond = { event.get_str("slack.context.ip_address") == Some("") };
            if _cond {
                event.remove("slack.context.ip_address");
            }

            if event.has_value("slack.context.ip_address") {
                event.rename("slack.context.ip_address", "source.address")?;
            }

            if event.has_value("slack.details") {
                event.rename("slack.details", "slack.audit.details")?;
            }

            // on_failure: 3 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("slack.audit.details.previous_ip_address") {
                    if let Some(val) = event.get("slack.audit.details.previous_ip_address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "slack.audit.details.previous_ip_address".into(),
                                message,
                            }
                        })?;
                        event.set("slack.audit.details.previous_ip_address", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event
                        .remove("slack.audit.details.previous_ip_address")
                        .is_none()
                    {
                        return Err(TransformError::FieldNotFound {
                            path: "slack.audit.details.previous_ip_address".into(),
                        });
                    }
                    Ok(())
                })();
                event.set("event.kind", json!("pipeline_error"))?;
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
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("slack.audit.details.previous_ua") {
                event.rename(
                    "slack.audit.details.previous_ua",
                    "slack.audit.details.previous_user_agent",
                )?;
            }

            if event.has_value("slack.audit.details.md5_hash") {
                event.rename("slack.audit.details.md5_hash", "slack.audit.details.md5")?;
            }

            let _cond = { event.has_value("slack.audit.details.md5") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("slack.audit.details.md5")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get_str("slack.audit.entity.entity_type") == Some("file") };
            if _cond {
                if let Some(v) = event
                    .get("slack.audit.details.md5")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("file.hash.md5", v)?;
                }
            }

            let _cond = {
                event.has_value("slack.audit.details.action_timestamp")
                    && event
                        .get_i64("slack.audit.details.action_timestamp")
                        .is_some_and(|n| n > 10000000000000)
            };
            if _cond {
                // Painless script
                // Source: def secs = (long)(ctx.slack.audit.details.action_timestamp/1e6);\ndef nanos = (long)(ctx.slack.audit.details.action_timestamp % 1e6) * 1000;\nctx[\"@timestamp\"] = Instant.ofEpochSecond(secs, nanos).atZone(ZoneId.of(\"UTC\"));\nctx.slack.audit.details.remove(\"action_timestamp\");\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def secs = (long)(ctx.slack.audit.details.action_timestamp/1e6);\ndef nanos = (long)(ctx.slack.audit.details.action_timestamp % 1e6) * 1000;\nctx[\"@timestamp\"] = Instant.ofEpochSecond(secs, nanos).atZone(ZoneId.of(\"UTC\"));\nctx.slack.audit.details.remove(\"action_timestamp\");\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("slack.audit.details.action_timestamp") };
            if _cond {
                // on_failure: 3 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("slack.audit.details.action_timestamp")
                    {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "slack.audit.details.action_timestamp".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event
                            .remove("slack.audit.details.action_timestamp")
                            .is_none()
                        {
                            return Err(TransformError::FieldNotFound {
                                path: "slack.audit.details.action_timestamp".into(),
                            });
                        }
                        Ok(())
                    })();
                    event.set("event.kind", json!("pipeline_error"))?;
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
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            event.remove("slack.audit.details.action_timestamp");

            if event.has_value("slack.audit.details.url_private") {
                uri_parts(event, "slack.audit.details.url_private", "url", true, false)?;
            }

            if event.has_value("source.address") {
                if let Some(val) = event.get("source.address") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "source.address".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if event.has_value("slack.context.location") {
                event.rename("slack.context.location", "slack.audit.context")?;
            }

            if event.has_value("slack.context.session_id") {
                event.rename("slack.context.session_id", "slack.audit.context.session_id")?;
            }

            if event.has_value("slack.audit.context.session_id") {
                if let Some(val) = event.get("slack.audit.context.session_id") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "slack.audit.context.session_id".into(),
                            message,
                        }
                    })?;
                    event.set("slack.audit.context.session_id", converted)?;
                }
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
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

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append(
                    "related.ip",
                    json!(
                        event
                            .get("source.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // Painless script
            // Source: ctx.event.kind = 'event';\nctx.event.type = ['info'];\nif (ctx.event?.action == null) {\n    return;\n}\nif (params.get(ctx.event.action) == null) {\n    return;\n}\ndef hm = new HashMap(params.get(ctx.event.action));\nhm.forEach((k, v) -> ctx.event[k] = v);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"ctx.event.kind = 'event';\nctx.event.type = ['info'];\nif (ctx.event?.action == null) {\n    return;\n}\nif (params.get(ctx.event.action) == null) {\n    return;\n}\ndef hm = new HashMap(params.get(ctx.event.action));\nhm.forEach((k, v) -> ctx.event[k] = v);\n"#
                ),
                cached_params!(
                    "{\"user_login\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"info\",\"start\"],\"outcome\":\"success\"},\"user_login_failed\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"user_logout\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"info\",\"end\"],\"outcome\":\"success\"},\"user_session_invalidated\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"info\",\"end\"],\"outcome\":\"success\"},\"user_session_reset_by_admin\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"info\",\"end\"],\"outcome\":\"success\"},\"user_created\":{\"category\":[\"iam\"],\"type\":[\"creation\",\"user\"]},\"user_deactivated\":{\"category\":[\"iam\"],\"type\":[\"deletion\",\"user\"]},\"user_reactivated\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\"]},\"role_change_to_admin\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\",\"admin\"]},\"role_change_to_guest\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\"]},\"role_change_to_owner\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\",\"admin\"]},\"role_change_to_user\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\"]},\"user_email_updated\":{\"category\":[\"iam\"],\"type\":[\"change\",\"user\"]},\"user_added_to_usergroup\":{\"category\":[\"iam\"],\"type\":[\"change\",\"group\",\"user\"]},\"user_removed_from_usergroup\":{\"category\":[\"iam\"],\"type\":[\"change\",\"group\",\"user\"]},\"default_channel_added_to_usergroup\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\",\"group\"]},\"default_channel_removed_from_usergroup\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\",\"group\"]},\"role_added_to_usergroup\":{\"category\":[\"iam\"],\"type\":[\"change\",\"group\"]},\"role_removed_from_usergroup\":{\"category\":[\"iam\"],\"type\":[\"change\",\"group\"]},\"role_modified_on_usergroup\":{\"category\":[\"iam\"],\"type\":[\"change\",\"group\"]},\"file_downloaded\":{\"category\":[\"file\"],\"type\":[\"access\"]},\"file_downloaded_blocked\":{\"category\":[\"file\"],\"type\":[\"denied\"]},\"file_uploaded\":{\"category\":[\"file\"],\"type\":[\"creation\"]},\"file_public_link_created\":{\"category\":[\"file\"],\"type\":[\"info\"]},\"file_public_link_revoked\":{\"category\":[\"file\"],\"type\":[\"info\"]},\"file_shared\":{\"category\":[\"file\"],\"type\":[\"info\"]},\"file_malicious_content_detected\":{\"category\":[\"file\",\"malware\"],\"type\":[\"info\"]}}"
                ),
            )?;

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx);\n
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
