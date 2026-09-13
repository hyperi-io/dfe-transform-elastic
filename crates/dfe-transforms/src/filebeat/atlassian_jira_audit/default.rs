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
                event.get("json.records").is_some_and(|v| v.is_array()) && event.get("json.records").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

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

            let _cond = { event.has_value("_config.atlassian_cloud") };
            if _cond {
                // Begin nested pipeline: "cloud"
                let _cond = { event.has_value("json.created") };
                if _cond {
                    if let Some(v) = event.get("json.created").cloned() {
                        event.set("_tmp.timestamp", v)?;
                    }
                }
                if event.has_value("json.id") {
                    if let Some(val) = event.get("json.id") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.id".into(),
                                message,
                            }
                        })?;
                        event.set("event.id", converted)?;
                    }
                }
                if event.has_value("json.remoteAddress") {
                    event.rename("json.remoteAddress", "source.address")?;
                }
                if event.has_value("json.authorAccountId") {
                    event.rename("json.authorAccountId", "user.id")?;
                }
                if event.has_value("json.category") {
                    event.rename("json.category", "jira.audit.type.category")?;
                }
                if event.has_value("json.summary") {
                    event.rename("json.summary", "jira.audit.type.action")?;
                }
                if let Some(v) = event
                    .get("jira.audit.type.action")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("event.action", v)?;
                }
                if event.has_value("json.associatedItems") {
                    event.rename("json.associatedItems", "jira.audit.affected_objects")?;
                }
                if event.has_value("json.changedValues") {
                    event.rename("json.changedValues", "jira.audit.changed_values")?;
                }
                // Painless script, resolved to its runners at generation time
                // Source: if(ctx.jira?.audit?.affected_objects == null) {\n    ArrayList items = new ArrayList();\n    ctx.jira?.audit.put(\"affected_objects\", items);\n} if(ctx.json?.objectItem != null && !ctx.jira?.audit?.affected_objects.contains(ctx.json?.objectItem)) {\n    ctx.jira?.audit?.affected_objects.add(ctx.json?.objectItem);\n}\n        \nif(ctx.jira?.audit?.affected_objects != null) {\n    for (def j = 0; j < ctx.jira?.audit?.affected_objects.length; j++) {\n        if(ctx.jira.audit.affected_objects[j]?.typeName != null) {\n            ctx.jira.audit.affected_objects[j].put('type', ctx.jira.audit.affected_objects[j].typeName);\n            ctx.jira.audit.affected_objects[j].remove('typeName');\n        }\n    }\n} if(ctx.jira?.audit?.changed_values != null) {\n    for (def j = 0; j < ctx.jira?.audit?.changed_values.length; j++) {\n        if(ctx.jira.audit.changed_values[j]?.fieldName != null) {\n            ctx.jira.audit.changed_values[j].put('i18nKey', ctx.jira.audit.changed_values[j].fieldName);\n            ctx.jira.audit.changed_values[j].put('key', ctx.jira.audit.changed_values[j].fieldName);\n            ctx.jira.audit.changed_values[j].remove('fieldName');\n        }\n        if(ctx.jira.audit.changed_values[j]?.changedTo != null) {\n            ctx.jira.audit.changed_values[j].put('to', ctx.jira.audit.changed_values[j].changedTo);\n            ctx.jira.audit.changed_values[j].remove('changedTo');\n        }\n        if(ctx.jira.audit.changed_values[j]?.changedFrom != null) {\n            ctx.jira.audit.changed_values[j].put('from', ctx.jira.audit.changed_values[j].changedFrom);\n            ctx.jira.audit.changed_values[j].remove('changedFrom');\n        }\n    }\n}
                list_item_renames(
                    event,
                    &ListItemRenames::new(
                        Some(EnsureItem::new(
                            "jira.audit.affected_objects",
                            "json.objectItem",
                        )),
                        vec![
                            ListWalk::new(
                                "jira.audit.affected_objects",
                                vec![ItemRename::new("typeName", vec!["type".into()])],
                            ),
                            ListWalk::new(
                                "jira.audit.changed_values",
                                vec![
                                    ItemRename::new(
                                        "fieldName",
                                        vec!["i18nKey".into(), "key".into()],
                                    ),
                                    ItemRename::new("changedTo", vec!["to".into()]),
                                    ItemRename::new("changedFrom", vec!["from".into()]),
                                ],
                            ),
                        ],
                    ),
                );
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
                if event.has_value("json.author.id") {
                    event.rename("json.author.id", "user.id")?;
                }
                if event.has_value("json.author.name") {
                    event.rename("json.author.name", "user.name")?;
                }
                if event.has_value("json.auditType") {
                    event.rename("json.auditType", "jira.audit.type")?;
                }
                if event.has_value("json.type") {
                    event.rename("json.type", "jira.audit.type")?;
                }
                if event.has_value("json.method") {
                    event.rename("json.method", "jira.audit.method")?;
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
                    event.rename("json.extraAttributes", "jira.audit.extra_attributes")?;
                }
                if event.has_value("json.changedValues") {
                    event.rename("json.changedValues", "jira.audit.changed_values")?;
                }
                if event.has_value("json.affectedObjects") {
                    event.rename("json.affectedObjects", "jira.audit.affected_objects")?;
                }
                if let Some(v) = event
                    .get("jira.audit.type.actionI18nKey")
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
                if let Some(s) = event.get_string("json.source") {
                    let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("_tmp.source_ip", Value::Array(parts))?;
                }
            }

            let _cond = { event.get("_tmp.source_ip").is_some_and(|v| v.is_array()) };
            if _cond {
                // Painless script
                // Source: int ip_list_size = ctx._tmp.source_ip.size(); ctx.source = new HashMap(); if (ip_list_size > 0) {\n  ctx.source.address = ctx._tmp.source_ip[0];\n\n  if (ip_list_size > 1) {\n    ctx.jira.audit.additional_source_ips = ctx._tmp.source_ip.subList(1, ip_list_size);\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"int ip_list_size = ctx._tmp.source_ip.size(); ctx.source = new HashMap(); if (ip_list_size > 0) {\n  ctx.source.address = ctx._tmp.source_ip[0];\n\n  if (ip_list_size > 1) {\n    ctx.jira.audit.additional_source_ips = ctx._tmp.source_ip.subList(1, ip_list_size);\n  }\n}\n"#
                    ),
                )?;
            }

            let _cond = {
                event
                    .get("jira.audit.additional_source_ips")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "jira.audit.additional_source_ips", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                        if event.has_value("{{{_ingest._value}}}") {
                            if let Some(val) = event.get("{{{_ingest._value}}}") {
                                let converted = convert_value(val, "ip").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "{{{_ingest._value}}}".into(),
                                        message,
                                    }
                                })?;
                                event.set("{{{_ingest._value}}}", converted)?;
                            }
                        }
                        Ok(())
                    })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "convert")?;
                        event.remove("_ingest._value");
                        event.append(
                            "error.message",
                            json!(format!(
                                "Processor {} in pipeline {} failed with message: {}",
                                event
                                    .get("_ingest.on_failure_processor_type")
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
                    Ok(())
                })?;
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
            // Source: ctx.event.kind = 'event'; ctx.event.type = ['info'];\nif (ctx?.event?.action == null) {\n    return;\n}\nif (params.get(ctx.event.action) == null) {\n    return;\n}\ndef hm = new HashMap(params.get(ctx.event.action)); hm.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"ctx.event.kind = 'event'; ctx.event.type = ['info'];\nif (ctx?.event?.action == null) {\n    return;\n}\nif (params.get(ctx.event.action) == null) {\n    return;\n}\ndef hm = new HashMap(params.get(ctx.event.action)); hm.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"User created\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"]},\"jira.auditing.user.created\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"]},\"User updated\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"jira.auditing.user.updated\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"User deleted\":{\"category\":[\"iam\"],\"type\":[\"user\",\"deletion\"]},\"jira.auditing.user.deleted\":{\"category\":[\"iam\"],\"type\":[\"user\",\"deletion\"]},\"User added to group\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"jira.auditing.user.added.to.group\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"User removed from group\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"jira.auditing.user.removed.from.group\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"jira.auditing.user.logged.in\":{\"category\":[\"authentication\"],\"type\":[\"start\"],\"outcome\":\"success\"},\"jira.auditing.user.logged.out\":{\"category\":[\"authentication\"],\"type\":[\"end\"]},\"jira.auditing.user.password.changed\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\"},\"jira.auditing.user.login.failed\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"jira.auditing.websudo.session.started\":{\"category\":[\"authentication\"],\"type\":[\"start\"]},\"jira.auditing.websudo.session.invalidated\":{\"category\":[\"authentication\"],\"type\":[\"end\"]},\"Group created\":{\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"jira.auditing.group.created\":{\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"Group deleted\":{\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"jira.auditing.group.deleted\":{\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"Global permission added\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"creation\"]},\"jira.auditing.global.permission.added\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"creation\"]},\"Global permission removed\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"admin\",\"deletion\"]},\"personal.access.tokens.audit.log.summary.token.created\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"creation\"]},\"personal.access.tokens.audit.log.summary.token.deleted\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"deletion\"]},\"jira.auditing.issue.type.created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"atlassian.audit.event.action.audit.config.updated\":{\"category\":[\"configuration\"],\"type\":[\"admin\",\"change\"]},\"Project created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"jira.auditing.project.created\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"jira.auditing.project.lead.updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"jira.auditing.project.default.assignee.updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"jira.auditing.project.updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"jira.auditing.permission.scheme.removed.from.project\":{\"category\":[\"configuration\",\"iam\"],\"type\":[\"deletion\"]},\"jira.auditing.issue.type.screen.scheme.removed\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"jira.auditing.project.deleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]}}"
                ),
            )?;

            let _cond = {
                [
                    "jira.auditing.category.usermanagement",
                    "jira.auditing.category.groupmanagement",
                ]
                .contains(
                    &event
                        .get_str("jira.audit.type.categoryI18nKey")
                        .unwrap_or(""),
                ) || ["user management", "group management"]
                    .contains(&event.get_str("jira.audit.type.category").unwrap_or(""))
            };
            if _cond {
                // Painless script
                // Source: if (ctx?.event?.action == null) {\n    return;\n} if (ctx.group == null) {\n  Map map = new HashMap();\n  ctx.put(\"group\", map);\n} if (ctx.user == null) {\n  Map map = new HashMap();\n  ctx.put(\"user\", map);\n} if (ctx.user?.target == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"target\", map);\n} if (ctx.user?.changes == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"changes\", map);\n} if (ctx.user?.target?.group == null) {\n  Map map = new HashMap();\n  ctx.user.target.put(\"group\", map);\n} if(ctx.jira?.audit?.affected_objects != null) {\n  for (def j = 0; j < ctx.jira?.audit?.affected_objects.length; j++) {\n    if(ctx.jira?.audit?.affected_objects[j]?.type == 'GROUP') {\n      if(['jira.auditing.group.created', 'jira.auditing.group.deleted', 'Group created', 'Group deleted'].contains(ctx.event.action)) {\n        ctx.group.put(\"name\", ctx.jira?.audit?.affected_objects[j]?.name);\n      }\n      if(['jira.auditing.user.added.to.group', 'jira.auditing.user.removed.from.group', 'User added to group', 'User removed from group'].contains(ctx.event.action)) {\n        ctx.user.target.group.put(\"name\", ctx.jira?.audit?.affected_objects[j]?.name);\n        ctx.user.target.group.put(\"id\", ctx.jira?.audit?.affected_objects[j]?.id);\n      }\n    }\n    if(ctx.jira?.audit?.affected_objects[j]?.type == 'USER') {\n      if(['jira.auditing.user.created', 'jira.auditing.user.deleted','jira.auditing.user.password.changed','jira.auditing.user.updated','jira.auditing.user.added.to.group', 'jira.auditing.user.removed.from.group', 'User created', 'User deleted', 'User added to group', 'User removed from group'].contains(ctx.event.action)) {\n        ctx.user.target.put(\"name\", ctx.jira?.audit?.affected_objects[j]?.name);\n        ctx.user.target.put(\"id\", ctx.jira?.audit?.affected_objects[j]?.id);\n      }\n    }\n  }\n} if(ctx.jira?.audit?.changed_values != null) {\n  for (def j = 0; j < ctx.jira?.audit?.changed_values.length; j++) {\n    if(['jira.auditing.user.renamed', 'User renamed'].contains(ctx.event.action)) {\n      if(ctx.jira?.audit?.changed_values[j]?.i18nKey == 'common.words.username') {\n        ctx.user.changes.put(\"name\", ctx.jira?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"name\", ctx.jira?.audit?.changed_values[j]?.from);\n      }\n    }\n    if(['jira.auditing.user.created','jira.auditing.user.updated', 'User created', 'User updated'].contains(ctx.event.action)) {\n      if(ctx.jira?.audit?.changed_values[j]?.i18nKey == 'common.words.username') {\n        ctx.user.changes.put(\"name\", ctx.jira?.audit?.changed_values[j]?.to);\n        if(ctx.jira?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"name\", ctx.jira?.audit?.changed_values[j]?.from);\n        }\n      }\n      if(ctx.jira?.audit?.changed_values[j]?.i18nKey == 'common.words.email') {\n        ctx.user.changes.put(\"email\", ctx.jira?.audit?.changed_values[j]?.to);\n        if(ctx.jira?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"email\", ctx.jira?.audit?.changed_values[j]?.from);\n        }\n      }\n      if(ctx.jira?.audit?.changed_values[j]?.i18nKey == 'common.words.fullname') {\n        ctx.user.changes.put(\"full_name\", ctx.jira?.audit?.changed_values[j]?.to);\n        if(ctx.jira?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"full_name\", ctx.jira?.audit?.changed_values[j]?.from);\n        }\n      }\n    }\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx?.event?.action == null) {\n    return;\n} if (ctx.group == null) {\n  Map map = new HashMap();\n  ctx.put(\"group\", map);\n} if (ctx.user == null) {\n  Map map = new HashMap();\n  ctx.put(\"user\", map);\n} if (ctx.user?.target == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"target\", map);\n} if (ctx.user?.changes == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"changes\", map);\n} if (ctx.user?.target?.group == null) {\n  Map map = new HashMap();\n  ctx.user.target.put(\"group\", map);\n} if(ctx.jira?.audit?.affected_objects != null) {\n  for (def j = 0; j < ctx.jira?.audit?.affected_objects.length; j++) {\n    if(ctx.jira?.audit?.affected_objects[j]?.type == 'GROUP') {\n      if(['jira.auditing.group.created', 'jira.auditing.group.deleted', 'Group created', 'Group deleted'].contains(ctx.event.action)) {\n        ctx.group.put(\"name\", ctx.jira?.audit?.affected_objects[j]?.name);\n      }\n      if(['jira.auditing.user.added.to.group', 'jira.auditing.user.removed.from.group', 'User added to group', 'User removed from group'].contains(ctx.event.action)) {\n        ctx.user.target.group.put(\"name\", ctx.jira?.audit?.affected_objects[j]?.name);\n        ctx.user.target.group.put(\"id\", ctx.jira?.audit?.affected_objects[j]?.id);\n      }\n    }\n    if(ctx.jira?.audit?.affected_objects[j]?.type == 'USER') {\n      if(['jira.auditing.user.created', 'jira.auditing.user.deleted','jira.auditing.user.password.changed','jira.auditing.user.updated','jira.auditing.user.added.to.group', 'jira.auditing.user.removed.from.group', 'User created', 'User deleted', 'User added to group', 'User removed from group'].contains(ctx.event.action)) {\n        ctx.user.target.put(\"name\", ctx.jira?.audit?.affected_objects[j]?.name);\n        ctx.user.target.put(\"id\", ctx.jira?.audit?.affected_objects[j]?.id);\n      }\n    }\n  }\n} if(ctx.jira?.audit?.changed_values != null) {\n  for (def j = 0; j < ctx.jira?.audit?.changed_values.length; j++) {\n    if(['jira.auditing.user.renamed', 'User renamed'].contains(ctx.event.action)) {\n      if(ctx.jira?.audit?.changed_values[j]?.i18nKey == 'common.words.username') {\n        ctx.user.changes.put(\"name\", ctx.jira?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"name\", ctx.jira?.audit?.changed_values[j]?.from);\n      }\n    }\n    if(['jira.auditing.user.created','jira.auditing.user.updated', 'User created', 'User updated'].contains(ctx.event.action)) {\n      if(ctx.jira?.audit?.changed_values[j]?.i18nKey == 'common.words.username') {\n        ctx.user.changes.put(\"name\", ctx.jira?.audit?.changed_values[j]?.to);\n        if(ctx.jira?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"name\", ctx.jira?.audit?.changed_values[j]?.from);\n        }\n      }\n      if(ctx.jira?.audit?.changed_values[j]?.i18nKey == 'common.words.email') {\n        ctx.user.changes.put(\"email\", ctx.jira?.audit?.changed_values[j]?.to);\n        if(ctx.jira?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"email\", ctx.jira?.audit?.changed_values[j]?.from);\n        }\n      }\n      if(ctx.jira?.audit?.changed_values[j]?.i18nKey == 'common.words.fullname') {\n        ctx.user.changes.put(\"full_name\", ctx.jira?.audit?.changed_values[j]?.to);\n        if(ctx.jira?.audit?.changed_values[j]?.from != null) {\n          ctx.user.target.put(\"full_name\", ctx.jira?.audit?.changed_values[j]?.from);\n        }\n      }\n    }\n  }\n}"#
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

            let _cond = {
                event
                    .get("jira.audit.additional_source_ips")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "jira.audit.additional_source_ips", |event| {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"\n", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
