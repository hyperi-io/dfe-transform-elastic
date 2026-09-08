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

            parse_json_field(event, "event.original", "json")?;

            let _cond = {
                (event.get("json.entities").is_some_and(|v| v.is_array()) && event.get("json.entities").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)) || (event.get("json.results").is_some_and(|v| v.is_array()) && event.get("json.results").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0))
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  for (def x : map.values()) {\n    if (x instanceof Map) {\n        handleMap(x);\n    } else if (x instanceof List) {\n        handleList(x);\n    }\n  }\n  map.values().removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nvoid handleList(List list) {\n  for (def x : list) {\n      if (x instanceof Map) {\n          handleMap(x);\n      } else if (x instanceof List) {\n          handleList(x);\n      }\n  }\n  list.removeIf(v -> v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0));\n}\nhandleMap(ctx.json);\n
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                Some("json"),
            );

            let _cond = { event.has_value("_config.atlassian_cloud") };
            if _cond {
                // Begin nested pipeline: "cloud"
                let _cond = { event.has_value("json.creationDate") };
                if _cond {
                    if let Some(date_str) = event.get_as_string("json.creationDate") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.creationDate".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                }
                if event.has_value("json.remoteAddress") {
                    event.rename("json.remoteAddress", "source.address")?;
                }
                if event.has_value("json.author.accountId") {
                    event.rename("json.author.accountId", "user.id")?;
                }
                if event.has_value("json.author.displayName") {
                    event.rename("json.author.displayName", "user.full_name")?;
                }
                if event.has_value("json.author.externalCollaborator") {
                    event.rename(
                        "json.author.externalCollaborator",
                        "confluence.audit.external_collaborator",
                    )?;
                }
                if event.has_value("json.category") {
                    event.rename("json.category", "confluence.audit.type.category")?;
                }
                if event.has_value("json.summary") {
                    event.rename("json.summary", "confluence.audit.type.action")?;
                }
                if let Some(v) = event
                    .get("confluence.audit.type.action")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
                if event.has_value("json.associatedObjects") {
                    event.rename(
                        "json.associatedObjects",
                        "confluence.audit.affected_objects",
                    )?;
                }
                if event.has_value("json.changedValues") {
                    event.rename("json.changedValues", "confluence.audit.changed_values")?;
                }
                let _cond = { event.has_value("confluence.audit") };
                if _cond {
                    // Painless script, resolved to its runners at generation time
                    // Source: if(ctx.confluence.audit.affected_objects == null) {\n    ArrayList items = new ArrayList();\n    ctx.confluence.audit.put(\"affected_objects\", items);\n} if(ctx.json?.affectedObject != null && !ctx.confluence?.audit?.affected_objects.contains(ctx.json?.affectedObject)) {\n    ctx.confluence.audit.affected_objects.add(ctx.json?.affectedObject);\n}\n        \nif(ctx.confluence.audit.affected_objects != null) {\n    for (def j = 0; j < ctx.confluence?.audit?.affected_objects.length; j++) {\n        if(ctx.confluence.audit.affected_objects[j]?.objectType != null) {\n            ctx.confluence.audit.affected_objects[j].put('type', ctx.confluence.audit.affected_objects[j].objectType);\n            ctx.confluence.audit.affected_objects[j].remove('objectType');\n        }\n    }\n} if(ctx.confluence.audit.changed_values != null) {\n    for (def j = 0; j < ctx.confluence.audit.changed_values.length; j++) {\n        if(ctx.confluence.audit.changed_values[j]?.name != null) {\n            ctx.confluence.audit.changed_values[j].put('i18nKey', ctx.confluence.audit.changed_values[j].name);\n            ctx.confluence.audit.changed_values[j].put('key', ctx.confluence.audit.changed_values[j].name);\n            ctx.confluence.audit.changed_values[j].remove('name');\n        }\n        if(ctx.confluence.audit.changed_values[j]?.newValue != null) {\n            ctx.confluence.audit.changed_values[j].put('to', ctx.confluence.audit.changed_values[j].newValue);\n            ctx.confluence.audit.changed_values[j].remove('newValue');\n        }\n        if(ctx.confluence.audit.changed_values[j]?.oldValue != null) {\n            ctx.confluence.audit.changed_values[j].put('from', ctx.confluence.audit.changed_values[j].oldValue);\n            ctx.confluence.audit.changed_values[j].remove('oldValue');\n        }\n    }\n}
                    list_item_renames(
                        event,
                        &ListItemRenames::new(
                            Some(EnsureItem::new(
                                "confluence.audit.affected_objects",
                                "json.affectedObject",
                            )),
                            vec![
                                ListWalk::new(
                                    "confluence.audit.affected_objects",
                                    vec![ItemRename::new("objectType", vec!["type".into()])],
                                ),
                                ListWalk::new(
                                    "confluence.audit.changed_values",
                                    vec![
                                        ItemRename::new(
                                            "name",
                                            vec!["i18nKey".into(), "key".into()],
                                        ),
                                        ItemRename::new("newValue", vec!["to".into()]),
                                        ItemRename::new("oldValue", vec!["from".into()]),
                                    ],
                                ),
                            ],
                        ),
                    );
                }
                // End nested pipeline: "cloud"
            }

            let _cond = { !event.has_value("_config.atlassian_cloud") };
            if _cond {
                // Begin nested pipeline: "self-hosted"
                let _cond = {
                    event.has_value("json.timestamp")
                        && event.get("json.timestamp").is_some_and(|v| v.is_string())
                };
                if _cond {
                    if let Some(v) = event.get("json.timestamp").cloned() {
                        event.set("_tmp.timestamp", v)?;
                    }
                }
                let _cond = {
                    event.has_value("json.timestamp")
                        && event.get("json.timestamp").is_some_and(|v| v.is_object())
                        && event.has_value("json.timestamp.epochSecond")
                        && event.has_value("json.timestamp.nano")
                };
                if _cond {
                    event.set(
                        "_tmp.timestamp",
                        json!(format!(
                            "{}.{}",
                            event
                                .get("json.timestamp.epochSecond")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("json.timestamp.nano")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                }
                if let Some(date_str) = event.get_as_string("_tmp.timestamp") {
                    match parse_date_out(&date_str, &["UNIX", "ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "_tmp.timestamp".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                if event.has_value("json.source") {
                    event.rename("json.source", "source.address")?;
                }
                if event.has_value("json.author.id") {
                    event.rename("json.author.id", "user.id")?;
                }
                if event.has_value("json.author.name") {
                    event.rename("json.author.name", "user.full_name")?;
                }
                let _cond = { event.get_str("json.author.uri") != Some("") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        if event.has_value("json.author.uri") {
                            if let Some(input) = event.get_string("json.author.uri") {
                                // Grok pattern: \\?username=%{USER:user.name}$
                                if !cached_grok!("\\?username=%{USER:user.name}$")
                                    .extract_into(&input, event)?
                                {
                                    return Err(TransformError::GrokNoMatch { value: input });
                                }
                            }
                        }
                        Ok(())
                    })();
                }
                if event.has_value("json.auditType") {
                    event.rename("json.auditType", "confluence.audit.type")?;
                }
                if event.has_value("json.type") {
                    event.rename("json.type", "confluence.audit.type")?;
                }
                if event.has_value("json.method") {
                    event.rename("json.method", "confluence.audit.method")?;
                }
                if event.has_value("json.system") {
                    event.rename("json.system", "service.address")?;
                }
                let _cond = { event.has_value("service.address") };
                if _cond {
                    // ignore_failure: true
                    let _ = (|| -> Result<()> {
                        uri_parts(event, "service.address", "_tmp.service", true, false)?;
                        Ok(())
                    })();
                }
                if event.has_value("json.extraAttributes") {
                    event.rename("json.extraAttributes", "confluence.audit.extra_attributes")?;
                }
                if event.has_value("json.changedValues") {
                    event.rename("json.changedValues", "confluence.audit.changed_values")?;
                }
                if event.has_value("json.affectedObjects") {
                    event.rename("json.affectedObjects", "confluence.audit.affected_objects")?;
                }
                if let Some(v) = event
                    .get("confluence.audit.type.actionI18nKey")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
                let _cond = { event.has_value("_tmp.service.domain") };
                if _cond {
                    event.append_unique(
                        "related.hosts",
                        json!(
                            event
                                .get("_tmp.service.domain")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                }
                // End nested pipeline: "self-hosted"
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

            // Painless script
            // Source: ctx.event.kind = 'event'; ctx.event.type = ['info']; if (ctx?.event?.action == null) {\n    return;\n} if (params.get(ctx.event.action) == null) {\n    return;\n} def hm = new HashMap(params.get(ctx.event.action)); hm.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"ctx.event.kind = 'event'; ctx.event.type = ['info']; if (ctx?.event?.action == null) {\n    return;\n} if (params.get(ctx.event.action) == null) {\n    return;\n} def hm = new HashMap(params.get(ctx.event.action)); hm.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"Global permission added\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"creation\"]},\"audit.logging.summary.global.permission.added\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"creation\"]},\"Global permission removed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"deletion\"]},\"audit.logging.summary.space.permission.added\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"creation\"]},\"User created\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"]},\"audit.logging.summary.user.created\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"]},\"User renamed\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"audit.logging.summary.user.renamed\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"User details updated\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"audit.logging.summary.user.updated\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"User deleted\":{\"category\":[\"iam\"],\"type\":[\"user\",\"deletion\"]},\"audit.logging.summary.user.deleted\":{\"category\":[\"iam\"],\"type\":[\"user\",\"deletion\"]},\"User added to group\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"audit.logging.summary.group.membership.added\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"User removed from group\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"audit.logging.summary.group.membership.removed\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"Group created\":{\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"audit.logging.summary.group.created\":{\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"Group deleted\":{\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"audit.logging.summary.group.deleted\":{\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"Audit Log configuration updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"atlassian.audit.event.action.audit.config.updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"audit.logging.summary.global.settings.edited\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"personal.access.tokens.audit.log.summary.token.created\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"creation\"]},\"personal.access.tokens.audit.log.summary.token.deleted\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"deletion\"]},\"audit.logging.summary.login.success\":{\"category\":[\"authentication\"],\"type\":[\"start\"],\"outcome\":\"success\"},\"audit.logging.summary.user.logout\":{\"category\":[\"authentication\"],\"type\":[\"end\"]},\"audit.logging.summary.login.failed\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"audit.logging.summary.user.password.changed\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\"},\"audit.logging.summary.sudo.auth.successful\":{\"category\":[\"authentication\"],\"type\":[\"start\"]},\"audit.logging.summary.sudo.logout\":{\"category\":[\"authentication\"],\"type\":[\"end\"]},\"audit.logging.summary.space.created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"audit.logging.summary.page.created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"audit.logging.summary.page.deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"audit.logging.summary.space.removed\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"audit.logging.summary.space.config.updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"]}}"
                ),
            )?;

            let _cond = {
                [
                    "audit.logging.category.user.management",
                    "audit.logging.category.auth",
                ]
                .contains(
                    &event
                        .get_str("confluence.audit.type.categoryI18nKey")
                        .unwrap_or(""),
                ) || ["Users and groups"].contains(
                    &event
                        .get_str("confluence.audit.type.category")
                        .unwrap_or(""),
                )
            };
            if _cond {
                // Painless script
                // Source: if (ctx?.event?.action == null) {\n    return;\n} if (ctx.group == null) {\n  Map map = new HashMap();\n  ctx.put(\"group\", map);\n} if (ctx.user == null) {\n  Map map = new HashMap();\n  ctx.put(\"user\", map);\n} if (ctx.user?.target == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"target\", map);\n} if (ctx.user?.changes == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"changes\", map);\n} if (ctx.user?.target?.group == null) {\n  Map map = new HashMap();\n  ctx.user.target.put(\"group\", map);\n} if(ctx.confluence?.audit?.affected_objects != null) {\n  for (def j = 0; j < ctx.confluence?.audit?.affected_objects.length; j++) {\n    if(ctx.confluence?.audit?.affected_objects[j]?.type == 'Group') {\n      String group_name = ctx.confluence?.audit?.affected_objects[j]?.name;\n      String group_id = ctx.confluence?.audit?.affected_objects[j]?.id;\n      if(ctx._config?.atlassian_cloud != null) {\n          def m = /(.+):(\\b[0-9a-f]{8}\\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\\b[0-9a-f]{12}\\b)$/.matcher(group_name);\n          if (m.find()) {\n            group_name = m.group(1);\n            group_id = m.group(2);\n          }\n      }\n      if(['audit.logging.summary.group.created', 'audit.logging.summary.group.deleted', 'Group created', 'Group deleted'].contains(ctx.event.action)) {\n        ctx.group.put(\"name\", group_name);\n        ctx.group.put(\"id\", group_id);\n      }\n      if(['audit.logging.summary.group.membership.added', 'audit.logging.summary.group.membership.removed', 'User added to group','User removed from group'].contains(ctx.event.action)) {\n        ctx.user.target.group.put(\"name\", group_name);\n        ctx.user.target.group.put(\"id\", group_id);\n      }\n    }\n    if(ctx.confluence?.audit?.affected_objects[j]?.type == 'User') {\n      if(['audit.logging.summary.user.created', 'audit.logging.summary.user.deleted', 'audit.logging.summary.user.password.changed','audit.logging.summary.user.updated', 'User created', 'User deleted', 'User details updated'].contains(ctx.event.action)) {\n        ctx.user.target.put(\"full_name\", ctx.confluence?.audit?.affected_objects[j]?.name);\n        ctx.user.target.put(\"id\", ctx.confluence?.audit?.affected_objects[j]?.id);\n        if(ctx.confluence?.audit?.affected_objects[j]?.uri != null) {\n          def m = /\\?username=([a-zA-Z0-9._-]+)$/.matcher(ctx.confluence?.audit?.affected_objects[j]?.uri);\n          if (m.find()) {\n            ctx.user.target.put(\"name\", m.group(1));\n          }\n        }\n      }\n      if(['audit.logging.summary.group.membership.added', 'audit.logging.summary.group.membership.removed', 'User added to group', 'User removed from group'].contains(ctx.event.action)) {\n        ctx.user.target.put(\"name\", ctx.confluence?.audit?.affected_objects[j]?.name);\n        ctx.user.target.put(\"id\", ctx.confluence?.audit?.affected_objects[j]?.id);\n      }\n      if(['audit.logging.summary.login.success', 'audit.logging.summary.login.failed'].contains(ctx.event.action)) {\n        ctx.user.put(\"full_name\", ctx.confluence?.audit?.affected_objects[j]?.name);\n        ctx.user.put(\"id\", ctx.confluence?.audit?.affected_objects[j]?.id);\n        if(ctx.confluence?.audit?.affected_objects[j]?.uri != null) {\n          def m = /\\?username=([a-zA-Z0-9._-]+)$/.matcher(ctx.confluence?.audit?.affected_objects[j]?.uri);\n          if (m.find()) {\n            ctx.user.put(\"name\", m.group(1));\n          }\n        }\n      }\n    }\n  }\n} if(ctx.confluence?.audit?.changed_values != null) {\n  for (def j = 0; j < ctx.confluence?.audit?.changed_values.length; j++) {\n    if(['audit.logging.summary.user.renamed', 'User renamed'].contains(ctx.event.action)) {\n      if(ctx.confluence?.audit?.changed_values[j]?.i18nKey == 'audit.logging.changed.value.username') {\n        ctx.user.changes.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.from);\n      }\n    }\n    if(['audit.logging.summary.user.created','audit.logging.summary.user.updated', 'User created', 'User details updated'].contains(ctx.event.action)) {\n      if(ctx.confluence?.audit?.changed_values[j]?.i18nKey == 'Username') {\n        ctx.user.changes.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.to);\n          ctx.user.target.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.to);\n        if(ctx.confluence?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.from);\n        }\n      }\n      if(ctx.confluence?.audit?.changed_values[j]?.i18nKey == 'Email') {\n        ctx.user.changes.put(\"email\", ctx.confluence?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"email\", ctx.confluence?.audit?.changed_values[j]?.to);\n        if(ctx.confluence?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"email\", ctx.confluence?.audit?.changed_values[j]?.from);\n        }\n      }\n      if(ctx.confluence?.audit?.changed_values[j]?.i18nKey == 'Display name') {\n        ctx.user.changes.put(\"full_name\", ctx.confluence?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"full_name\", ctx.confluence?.audit?.changed_values[j]?.to);\n        if(ctx.confluence?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"full_name\", ctx.confluence?.audit?.changed_values[j]?.from);\n        }\n      }\n    }\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx?.event?.action == null) {\n    return;\n} if (ctx.group == null) {\n  Map map = new HashMap();\n  ctx.put(\"group\", map);\n} if (ctx.user == null) {\n  Map map = new HashMap();\n  ctx.put(\"user\", map);\n} if (ctx.user?.target == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"target\", map);\n} if (ctx.user?.changes == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"changes\", map);\n} if (ctx.user?.target?.group == null) {\n  Map map = new HashMap();\n  ctx.user.target.put(\"group\", map);\n} if(ctx.confluence?.audit?.affected_objects != null) {\n  for (def j = 0; j < ctx.confluence?.audit?.affected_objects.length; j++) {\n    if(ctx.confluence?.audit?.affected_objects[j]?.type == 'Group') {\n      String group_name = ctx.confluence?.audit?.affected_objects[j]?.name;\n      String group_id = ctx.confluence?.audit?.affected_objects[j]?.id;\n      if(ctx._config?.atlassian_cloud != null) {\n          def m = /(.+):(\\b[0-9a-f]{8}\\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\\b[0-9a-f]{12}\\b)$/.matcher(group_name);\n          if (m.find()) {\n            group_name = m.group(1);\n            group_id = m.group(2);\n          }\n      }\n      if(['audit.logging.summary.group.created', 'audit.logging.summary.group.deleted', 'Group created', 'Group deleted'].contains(ctx.event.action)) {\n        ctx.group.put(\"name\", group_name);\n        ctx.group.put(\"id\", group_id);\n      }\n      if(['audit.logging.summary.group.membership.added', 'audit.logging.summary.group.membership.removed', 'User added to group','User removed from group'].contains(ctx.event.action)) {\n        ctx.user.target.group.put(\"name\", group_name);\n        ctx.user.target.group.put(\"id\", group_id);\n      }\n    }\n    if(ctx.confluence?.audit?.affected_objects[j]?.type == 'User') {\n      if(['audit.logging.summary.user.created', 'audit.logging.summary.user.deleted', 'audit.logging.summary.user.password.changed','audit.logging.summary.user.updated', 'User created', 'User deleted', 'User details updated'].contains(ctx.event.action)) {\n        ctx.user.target.put(\"full_name\", ctx.confluence?.audit?.affected_objects[j]?.name);\n        ctx.user.target.put(\"id\", ctx.confluence?.audit?.affected_objects[j]?.id);\n        if(ctx.confluence?.audit?.affected_objects[j]?.uri != null) {\n          def m = /\\?username=([a-zA-Z0-9._-]+)$/.matcher(ctx.confluence?.audit?.affected_objects[j]?.uri);\n          if (m.find()) {\n            ctx.user.target.put(\"name\", m.group(1));\n          }\n        }\n      }\n      if(['audit.logging.summary.group.membership.added', 'audit.logging.summary.group.membership.removed', 'User added to group', 'User removed from group'].contains(ctx.event.action)) {\n        ctx.user.target.put(\"name\", ctx.confluence?.audit?.affected_objects[j]?.name);\n        ctx.user.target.put(\"id\", ctx.confluence?.audit?.affected_objects[j]?.id);\n      }\n      if(['audit.logging.summary.login.success', 'audit.logging.summary.login.failed'].contains(ctx.event.action)) {\n        ctx.user.put(\"full_name\", ctx.confluence?.audit?.affected_objects[j]?.name);\n        ctx.user.put(\"id\", ctx.confluence?.audit?.affected_objects[j]?.id);\n        if(ctx.confluence?.audit?.affected_objects[j]?.uri != null) {\n          def m = /\\?username=([a-zA-Z0-9._-]+)$/.matcher(ctx.confluence?.audit?.affected_objects[j]?.uri);\n          if (m.find()) {\n            ctx.user.put(\"name\", m.group(1));\n          }\n        }\n      }\n    }\n  }\n} if(ctx.confluence?.audit?.changed_values != null) {\n  for (def j = 0; j < ctx.confluence?.audit?.changed_values.length; j++) {\n    if(['audit.logging.summary.user.renamed', 'User renamed'].contains(ctx.event.action)) {\n      if(ctx.confluence?.audit?.changed_values[j]?.i18nKey == 'audit.logging.changed.value.username') {\n        ctx.user.changes.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.from);\n      }\n    }\n    if(['audit.logging.summary.user.created','audit.logging.summary.user.updated', 'User created', 'User details updated'].contains(ctx.event.action)) {\n      if(ctx.confluence?.audit?.changed_values[j]?.i18nKey == 'Username') {\n        ctx.user.changes.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.to);\n          ctx.user.target.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.to);\n        if(ctx.confluence?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"name\", ctx.confluence?.audit?.changed_values[j]?.from);\n        }\n      }\n      if(ctx.confluence?.audit?.changed_values[j]?.i18nKey == 'Email') {\n        ctx.user.changes.put(\"email\", ctx.confluence?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"email\", ctx.confluence?.audit?.changed_values[j]?.to);\n        if(ctx.confluence?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"email\", ctx.confluence?.audit?.changed_values[j]?.from);\n        }\n      }\n      if(ctx.confluence?.audit?.changed_values[j]?.i18nKey == 'Display name') {\n        ctx.user.changes.put(\"full_name\", ctx.confluence?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"full_name\", ctx.confluence?.audit?.changed_values[j]?.to);\n        if(ctx.confluence?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"full_name\", ctx.confluence?.audit?.changed_values[j]?.from);\n        }\n      }\n    }\n  }\n}"#
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

            let _cond = { event.has_value("user.target.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.target.name")
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

            event.remove("json");
            event.remove("_tmp");
            event.remove("_config");

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
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    if event.remove("_config").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "_config".into(),
                        });
                    }
                    if event.remove("_tmp").is_none() {
                        return Err(TransformError::FieldNotFound {
                            path: "_tmp".into(),
                        });
                    }
                    Ok(())
                })();
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"\n", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
