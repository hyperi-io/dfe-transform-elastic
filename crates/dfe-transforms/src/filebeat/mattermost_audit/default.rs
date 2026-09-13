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

            parse_json_field(event, "event.original", "json")?;

            let _cond = { !(event.get("json").is_some_and(|v| v.is_object())) };
            if _cond {
                return Err(TransformError::ParseError {
                    path: "_fail".into(),
                    message: ("Missing JSON object").to_string(),
                });
            }

            if let Some(date_str) = event.get_as_string("json.timestamp") {
                match parse_date_out(
                    &date_str,
                    &["yyyy-MM-dd HH:mm:ss.SSS 'Z'"],
                    Some("UTC"),
                    None,
                ) {
                    Some(parsed) => event.set("@timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }

            if event.has_value("json.event") {
                event.rename("json.event", "event.action")?;
            }

            if event.has_value("json.err") {
                event.rename("json.err", "error.code")?;
            }

            let _cond = { event.get_str("json.errors") != Some("[]") };
            if _cond {
                if event.has_value("json.errors") {
                    event.rename("json.errors", "mattermost.audit.error.message")?;
                }
            }

            if event.has_value("mattermost.audit.error.message") {
                gsub_field(
                    event,
                    "mattermost.audit.error.message",
                    "mattermost.audit.error.message",
                    cached_regex!("(\\[|\\])"),
                    "",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("mattermost.audit.error.message") {
                    if let Some(s) = event.get_string("mattermost.audit.error.message") {
                        let mut parts: Vec<Value> = cached_regex!(",\\s+")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set("mattermost.audit.error.message", Value::Array(parts))?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.get_str("json.status") == Some("success") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.get_str("json.status") == Some("fail")
                    || event.has_value("mattermost.audit.error.message")
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { !event.has_value("event.outcome") };
            if _cond {
                event.set("event.outcome", json!("unknown"))?;
            }

            if event.has_value("json.user_id") {
                event.rename("json.user_id", "user.id")?;
            }

            if event.has_value("json.user_id") {
                event.rename("json.user_id", "user.id")?;
            }

            let _cond = { !event.has_value("user.id") };
            if _cond {
                if event.has_value("json.login_id") {
                    event.rename("json.login_id", "user.id")?;
                }
            }

            if event.has_value("json.ip_address") {
                event.rename("json.ip_address", "source.address")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("source.address") {
                    if let Some(val) = event.get("source.address") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "source.address".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
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

            if event.has_value("json.client") {
                if let Some(ua_str) = event.get_string("json.client") {
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

            if event.has_value("json.api_path") {
                event.rename("json.api_path", "mattermost.audit.api_path")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                uri_parts(event, "mattermost.audit.api_path", "url", true, false)?;
                Ok(())
            })();

            if event.has_value("json.session_id") {
                event.rename("json.session_id", "mattermost.audit.session.id")?;
            }

            if event.has_value("json.device_id") {
                event.rename("json.device_id", "mattermost.audit.device.id")?;
            }

            if event.has_value("json.cluster_id") {
                event.rename("json.cluster_id", "mattermost.audit.cluster.id")?;
            }

            if event.has_value("json.user.id") {
                event.rename("json.user.id", "user.target.id")?;
            }

            if event.has_value("json.user.name") {
                event.rename("json.user.name", "user.target.name")?;
            }

            if event.has_value("json.user.roles") {
                event.rename("json.user.roles", "user.target.roles")?;
            }

            if event.has_value("user.target.roles") {
                if let Some(s) = event.get_string("user.target.roles") {
                    let mut parts: Vec<Value> = cached_regex!("\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("user.target.roles", Value::Array(parts))?;
                }
            }

            if event.has_value("json.remove_user_id") {
                event.rename("json.remove_user_id", "user.target.id")?;
            }

            if event.has_value("json.user_ids") {
                gsub_field(
                    event,
                    "json.user_ids",
                    "json.user_ids",
                    cached_regex!("(\\[|\\])"),
                    "",
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.user_ids") {
                    if let Some(s) = event.get_string("json.user_ids") {
                        let mut parts: Vec<Value> = cached_regex!("\\s+")
                            .split(&s)
                            .into_iter()
                            .map(|p| json!(p))
                            .collect();
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                        event.set("json.user_ids", Value::Array(parts))?;
                    }
                }
                Ok(())
            })();

            if event.has_value("json.user_ids") {
                event.rename("json.user_ids", "user.target.id")?;
            }

            if event.has_value("json.team") {
                event.rename("json.team", "mattermost.audit.team")?;
            }

            if event.has_value("json.code") {
                event.rename("json.code", "http.response.status_code")?;
            }

            if event.has_value("json.post") {
                event.rename("json.post", "mattermost.audit.post")?;
            }

            if event.has_value("mattermost.audit.post.channel_id") {
                event.rename(
                    "mattermost.audit.post.channel_id",
                    "mattermost.audit.post.channel.id",
                )?;
            }

            if event.has_value("json.patch") {
                event.rename("json.patch", "mattermost.audit.patch")?;
            }

            if event.has_value("json.patched") {
                event.rename("json.patched", "mattermost.audit.patch")?;
            }

            if event.has_value("json.channel") {
                event.rename("json.channel", "mattermost.audit.channel")?;
            }

            if event.has_value("json.channeld") {
                event.rename("json.channeld", "mattermost.audit.channel")?;
            }

            // Painless script
            // Source: ctx.event.kind = 'event'; ctx.event.category = ['configuration']; ctx.event.type = ['info']; if (ctx.event.action == null) {\n    return;\n} if (params.get(ctx.event.action) == null) {\n    return;\n} def hm = new HashMap(params.get(ctx.event.action)); hm.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"ctx.event.kind = 'event'; ctx.event.category = ['configuration']; ctx.event.type = ['info']; if (ctx.event.action == null) {\n    return;\n} if (params.get(ctx.event.action) == null) {\n    return;\n} def hm = new HashMap(params.get(ctx.event.action)); hm.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"login\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"start\"]},\"Logout\":{\"category\":[\"authentication\",\"session\"],\"type\":[\"end\"]},\"revokeAllSessionsForUser\":{\"category\":[\"session\"],\"type\":[\"end\"]},\"getConfig\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"updateConfig\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"updatePassword\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"updatePreferences\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"updateUserActive\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"patchUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"createPost\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"createChannel\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"patchChannel\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"deleteChannel\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"convertChannelToPrivate\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"restoreChannel\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"removeChannelMember\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"createTeam\":{\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"patchTeam\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"deleteTeam\":{\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"addTeamMembers\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"removeTeamMember\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]}}"
                ),
            )?;

            let _cond = {
                event.get("event.category").is_some_and(|v| match v {
                    serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("iam")),
                    serde_json::Value::String(s) => s.contains("iam"),
                    _ => false,
                })
            };
            if _cond {
                // Painless script
                // Source: if (ctx.event.action == null) {\n    return;\n} if (ctx.group == null) {\n  Map map = new HashMap();\n  ctx.put(\"group\", map);\n} if (ctx.user == null) {\n  Map map = new HashMap();\n  ctx.put(\"user\", map);\n} if (ctx.user.target == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"target\", map);\n} if (ctx.user.changes == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"changes\", map);\n} if (ctx.user.target.group == null) {\n  Map map = new HashMap();\n  ctx.user.target.put(\"group\", map);\n} if (['patchUser'].contains(ctx.event.action)) {\n  if(ctx.user.target.name != ctx.mattermost?.audit?.patch?.name) {\n    ctx.user.changes.put(\"name\", ctx.mattermost?.audit?.patch?.name);\n  }\n} else if (['createTeam','patchTeam','deleteTeam'].contains(ctx.event.action)) {\n  ctx.group.put(\"name\", ctx.mattermost?.audit?.team?.name);\n  ctx.group.put(\"id\", ctx.mattermost?.audit?.team?.id);\n} else if (['addTeamMembers','removeTeamMember'].contains(ctx.event.action)) {\n  ctx.user.target.group.put(\"name\", ctx.mattermost?.audit?.team?.name);\n  ctx.user.target.group.put(\"id\", ctx.mattermost?.audit?.team?.id);\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.event.action == null) {\n    return;\n} if (ctx.group == null) {\n  Map map = new HashMap();\n  ctx.put(\"group\", map);\n} if (ctx.user == null) {\n  Map map = new HashMap();\n  ctx.put(\"user\", map);\n} if (ctx.user.target == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"target\", map);\n} if (ctx.user.changes == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"changes\", map);\n} if (ctx.user.target.group == null) {\n  Map map = new HashMap();\n  ctx.user.target.put(\"group\", map);\n} if (['patchUser'].contains(ctx.event.action)) {\n  if(ctx.user.target.name != ctx.mattermost?.audit?.patch?.name) {\n    ctx.user.changes.put(\"name\", ctx.mattermost?.audit?.patch?.name);\n  }\n} else if (['createTeam','patchTeam','deleteTeam'].contains(ctx.event.action)) {\n  ctx.group.put(\"name\", ctx.mattermost?.audit?.team?.name);\n  ctx.group.put(\"id\", ctx.mattermost?.audit?.team?.id);\n} else if (['addTeamMembers','removeTeamMember'].contains(ctx.event.action)) {\n  ctx.user.target.group.put(\"name\", ctx.mattermost?.audit?.team?.name);\n  ctx.user.target.group.put(\"id\", ctx.mattermost?.audit?.team?.id);\n}"#
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

            let _cond = { event.has_value("user.changes.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.changes.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
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

            let _cond = { event.get("user.target.id").is_some_and(|v| v.is_string()) };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.get("user.target.id").is_some_and(|v| v.is_array()) };
            if _cond {
                if event.has_value("user.target.id") {
                    foreach_array(event, "user.target.id", |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    })?;
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

            let _cond = { event.has_value("mattermost.audit.post.channel.id") };
            if _cond {
                event.append_unique(
                    "mattermost.audit.related.channel",
                    json!(
                        event
                            .get("mattermost.audit.post.channel.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("mattermost.audit.channel.id") };
            if _cond {
                event.append_unique(
                    "mattermost.audit.related.channel",
                    json!(
                        event
                            .get("mattermost.audit.channel.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("mattermost.audit.team.id") };
            if _cond {
                event.append_unique(
                    "mattermost.audit.related.team",
                    json!(
                        event
                            .get("mattermost.audit.team.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("json");

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
