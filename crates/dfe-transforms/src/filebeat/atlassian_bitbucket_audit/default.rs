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
                event.get("json.entities").is_some_and(|v| v.is_array()) && event.get("json.entities").is_some_and(|v| match v { serde_json::Value::Array(a) => a.len(), serde_json::Value::Object(o) => o.len(), serde_json::Value::String(s) => s.chars().count(), _ => 0 } == 0)
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond = { event.get("json").is_some_and(|v| v.is_object()) };
            if _cond {
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
            }

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

            if event.has_value("json.author.id") {
                event.rename("json.author.id", "user.id")?;
            }

            if event.has_value("json.author.name") {
                event.rename("json.author.name", "user.name")?;
            }

            if event.has_value("json.auditType") {
                event.rename("json.auditType", "bitbucket.audit.type")?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "bitbucket.audit.type")?;
            }

            if event.has_value("json.method") {
                event.rename("json.method", "bitbucket.audit.method")?;
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
                event.rename("json.extraAttributes", "bitbucket.audit.extra_attributes")?;
            }

            if event.has_value("json.changedValues") {
                event.rename("json.changedValues", "bitbucket.audit.changed_values")?;
            }

            if event.has_value("json.affectedObjects") {
                event.rename("json.affectedObjects", "bitbucket.audit.affected_objects")?;
            }

            if let Some(v) = event
                .get("bitbucket.audit.type.actionI18nKey")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
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
                    "{\"bitbucket.service.user.audit.action.usercreated\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"]},\"bitbucket.service.user.audit.action.userrenamed\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"bitbucket.service.user.audit.action.usercredentialupdated\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\"},\"bitbucket.service.user.audit.action.userdeleted\":{\"category\":[\"iam\"],\"type\":[\"user\",\"deletion\"]},\"bitbucket.service.user.audit.action.groupmembershipscreated.user\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"bitbucket.service.user.audit.action.groupmembershipdeleted\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"bitbucket.service.user.audit.action.authenticationsuccess\":{\"category\":[\"authentication\"],\"type\":[\"start\"],\"outcome\":\"success\"},\"bitbucket.web.audit.action.logoutsuccess\":{\"category\":[\"authentication\"],\"type\":[\"end\"]},\"bitbucket.service.user.audit.action.authenticationfailure\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"bitbucket.service.user.audit.action.groupcreated\":{\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"bitbucket.service.user.audit.action.groupdeleted\":{\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"bitbucket.service.user.audit.action.projectpermissiongranted\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"creation\"]},\"bitbucket.access.tokens.audit.action.accesstokencreated.personal\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"bitbucket.access.tokens.audit.action.accesstokendeleted.personal\":{\"category\":[\"iam\"],\"type\":[\"deletion\"]},\"bitbucket.access.tokens.audit.action.accesstokenmodified.personal\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"bitbucket.access.tokens.audit.action.accesstokencreated.repository\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"bitbucket.access.tokens.audit.action.accesstokendeleted.repository\":{\"category\":[\"iam\"],\"type\":[\"deletion\"]},\"bitbucket.access.tokens.audit.action.accesstokenmodified.repository\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"bitbucket.ssh.audit.action.sshkeycreated\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"bitbucket.ssh.audit.action.sshkeydeleted\":{\"category\":[\"iam\"],\"type\":[\"deletion\"]},\"bitbucket.ssh.audit.action.sshaccesskeygranted.repository\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"bitbucket.ssh.audit.action.sshaccesskeyrevoked.repository\":{\"category\":[\"iam\"],\"type\":[\"deletion\"]},\"bitbucket.plugins.gpg.audit.action.gpgevent.created\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"bitbucket.plugins.gpg.audit.action.gpgevent.deleted\":{\"category\":[\"iam\"],\"type\":[\"deletion\"]},\"bitbucket.service.user.audit.action.repositorypermissiongranted\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"creation\"]},\"bitbucket.service.user.audit.action.repositorypermissiongrantrequested\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"creation\"]},\"bitbucket.service.user.audit.action.repositorypermissionrevoked\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"deletion\"]},\"bitbucket.service.user.audit.action.repositorypermissionrevocationrequested\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"deletion\"]},\"atlassian.audit.event.action.audit.config.updated\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"bitbucket.service.repository.audit.action.repositoryaccessed\":{\"category\":[\"web\"],\"type\":[\"access\"]},\"bitbucket.service.repository.audit.action.repositorymodified\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"bitbucket.service.repository.audit.action.repositorymodificationrequested\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"bitbucket.service.repository.audit.action.repositorydeleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"bitbucket.service.project.audit.action.projectcreated\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"bitbucket.service.project.audit.action.projectdeleted\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"bitbucket.service.project.audit.action.projectdeletionrequested\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"bitbucket.service.project.audit.action.projectmodified\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"bitbucket.service.project.audit.action.projectmodificationrequested\":{\"category\":[\"configuration\"],\"type\":[\"change\"]}}"
                ),
            )?;

            let _cond = {
                ["bitbucket.service.audit.category.usersandgroups"].contains(
                    &event
                        .get_str("bitbucket.audit.type.categoryI18nKey")
                        .unwrap_or(""),
                )
            };
            if _cond {
                // Painless script
                // Source: if (ctx?.event?.action == null) {\n    return;\n} if (ctx.group == null) {\n  Map map = new HashMap();\n  ctx.put(\"group\", map);\n} if (ctx.user == null) {\n  Map map = new HashMap();\n  ctx.put(\"user\", map);\n} if (ctx.user?.target == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"target\", map);\n} if (ctx.user?.changes == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"changes\", map);\n} if (ctx.user?.target?.group == null) {\n  Map map = new HashMap();\n  ctx.user.target.put(\"group\", map);\n} if(ctx.bitbucket?.audit?.affected_objects != null) {\n  for (def j = 0; j < ctx.bitbucket?.audit?.affected_objects.length; j++) {\n    if(ctx.bitbucket?.audit?.affected_objects[j]?.type == 'GROUP') {\n      if(['bitbucket.service.user.audit.action.groupcreated', 'bitbucket.service.user.audit.action.groupdeleted'].contains(ctx.event.action)) {\n        ctx.group.put(\"name\", ctx.bitbucket?.audit?.affected_objects[j]?.name);\n        ctx.group.put(\"id\", ctx.bitbucket?.audit?.affected_objects[j]?.id);\n      }\n      if(['bitbucket.service.user.audit.action.groupmembershipscreated.user', 'bitbucket.service.user.audit.action.groupmembershipdeleted'].contains(ctx.event.action)) {\n        ctx.user.target.group.put(\"name\", ctx.bitbucket?.audit?.affected_objects[j]?.name);\n        ctx.user.target.group.put(\"id\", ctx.bitbucket?.audit?.affected_objects[j]?.id);\n      }\n    }\n    if(ctx.bitbucket?.audit?.affected_objects[j]?.type == 'USER') {\n      if(['bitbucket.service.user.audit.action.usercreated', 'bitbucket.service.user.audit.action.userdeleted', 'bitbucket.service.user.audit.action.userrenamed', 'bitbucket.service.user.audit.action.usercredentialupdated','bitbucket.service.user.audit.action.groupmembershipscreated.user', 'bitbucket.service.user.audit.action.groupmembershipdeleted'].contains(ctx.event.action)) {\n        ctx.user.target.put(\"name\", ctx.bitbucket?.audit?.affected_objects[j]?.name);\n        ctx.user.target.put(\"id\", ctx.bitbucket?.audit?.affected_objects[j]?.id);\n      }\n    }\n  }\n} if(ctx.bitbucket?.audit?.changed_values != null) {\n  for (def j = 0; j < ctx.bitbucket?.audit?.changed_values.length; j++) {\n    if(['bitbucket.service.user.audit.action.userrenamed'].contains(ctx.event.action)) {\n      if(ctx.bitbucket?.audit?.changed_values[j]?.i18nKey == 'bitbucket.service.user.audit.attribute.user.name') {\n        ctx.user.changes.put(\"name\", ctx.bitbucket?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"name\", ctx.bitbucket?.audit?.changed_values[j]?.from);\n      }\n    }\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx?.event?.action == null) {\n    return;\n} if (ctx.group == null) {\n  Map map = new HashMap();\n  ctx.put(\"group\", map);\n} if (ctx.user == null) {\n  Map map = new HashMap();\n  ctx.put(\"user\", map);\n} if (ctx.user?.target == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"target\", map);\n} if (ctx.user?.changes == null) {\n  Map map = new HashMap();\n  ctx.user.put(\"changes\", map);\n} if (ctx.user?.target?.group == null) {\n  Map map = new HashMap();\n  ctx.user.target.put(\"group\", map);\n} if(ctx.bitbucket?.audit?.affected_objects != null) {\n  for (def j = 0; j < ctx.bitbucket?.audit?.affected_objects.length; j++) {\n    if(ctx.bitbucket?.audit?.affected_objects[j]?.type == 'GROUP') {\n      if(['bitbucket.service.user.audit.action.groupcreated', 'bitbucket.service.user.audit.action.groupdeleted'].contains(ctx.event.action)) {\n        ctx.group.put(\"name\", ctx.bitbucket?.audit?.affected_objects[j]?.name);\n        ctx.group.put(\"id\", ctx.bitbucket?.audit?.affected_objects[j]?.id);\n      }\n      if(['bitbucket.service.user.audit.action.groupmembershipscreated.user', 'bitbucket.service.user.audit.action.groupmembershipdeleted'].contains(ctx.event.action)) {\n        ctx.user.target.group.put(\"name\", ctx.bitbucket?.audit?.affected_objects[j]?.name);\n        ctx.user.target.group.put(\"id\", ctx.bitbucket?.audit?.affected_objects[j]?.id);\n      }\n    }\n    if(ctx.bitbucket?.audit?.affected_objects[j]?.type == 'USER') {\n      if(['bitbucket.service.user.audit.action.usercreated', 'bitbucket.service.user.audit.action.userdeleted', 'bitbucket.service.user.audit.action.userrenamed', 'bitbucket.service.user.audit.action.usercredentialupdated','bitbucket.service.user.audit.action.groupmembershipscreated.user', 'bitbucket.service.user.audit.action.groupmembershipdeleted'].contains(ctx.event.action)) {\n        ctx.user.target.put(\"name\", ctx.bitbucket?.audit?.affected_objects[j]?.name);\n        ctx.user.target.put(\"id\", ctx.bitbucket?.audit?.affected_objects[j]?.id);\n      }\n    }\n  }\n} if(ctx.bitbucket?.audit?.changed_values != null) {\n  for (def j = 0; j < ctx.bitbucket?.audit?.changed_values.length; j++) {\n    if(['bitbucket.service.user.audit.action.userrenamed'].contains(ctx.event.action)) {\n      if(ctx.bitbucket?.audit?.changed_values[j]?.i18nKey == 'bitbucket.service.user.audit.attribute.user.name') {\n        ctx.user.changes.put(\"name\", ctx.bitbucket?.audit?.changed_values[j]?.to);\n        ctx.user.target.put(\"name\", ctx.bitbucket?.audit?.changed_values[j]?.from);\n      }\n    }\n  }\n}"#
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
