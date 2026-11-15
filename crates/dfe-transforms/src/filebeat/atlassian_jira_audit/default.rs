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

            // The pipeline's script, emitted as the runtime's equivalent
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
                // The pipeline's script, emitted as the runtime's equivalent
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
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("_tmp.source_ip", Value::Array(parts))?;
                }
            }

            let _cond = { event.get("_tmp.source_ip").is_some_and(|v| v.is_array()) };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "089b844ec3456e1c46149a09e9cb539f0c54ff18a4f91708f66cedc6e02825d3",
                        r#"{"params":null,"known":[{"FirstElement":{"takes":[{"array":"_tmp.source_ip","target":"source.address","only_if_unset":false,"only_if_single":false,"consumes_array":false}]}}]}"#
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

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan_params(
                event,
                compiled_painless!(
                    "0151577661d8f8cc0446d3e73027acb30cf0391240efb9f78b3ece903a25f837",
                    r#"{"params":{"LookupMerge":{"statements":[{"Literal":{"Set":{"path":"event.kind","value":{"Literal":"event"}}}},{"Literal":{"Set":{"path":"event.type","value":{"Literal":["info"]}}}},{"If":{"test":[[{"Compare":{"path":"event.action","wanted":"Null","negated":false,"lowered":false}}]],"then":["Return"],"alt":[]}},{"If":{"test":[["Never"]],"then":["Return"],"alt":[]}}],"whole":false}},"known":[]}"#,
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "366860bec17fc5ac07e8d98271144168e9cc0e8b9c4f33842bb5968a894f4a86",
                        r#"{"params":null,"known":[{"ItemWrites":{"body":[{"If":{"test":{"IsNull":{"Field":"event.action"}},"body":["Return"]}},{"If":{"test":{"IsNull":{"Field":"group"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"group","value":{"Local":"map"}}}]}},{"If":{"test":{"IsNull":{"Field":"user"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"user","value":{"Local":"map"}}}]}},{"If":{"test":{"IsNull":{"Field":"user.target"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"user.target","value":{"Local":"map"}}}]}},{"If":{"test":{"IsNull":{"Field":"user.changes"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"user.changes","value":{"Local":"map"}}}]}},{"If":{"test":{"IsNull":{"Field":"user.target.group"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"user.target.group","value":{"Local":"map"}}}]}},{"If":{"test":{"NotNull":{"Field":"jira.audit.affected_objects"}},"body":[{"Walk":{"index":"j","list":"jira.audit.affected_objects","body":[{"If":{"test":{"Equals":[{"Member":{"list":"jira.audit.affected_objects","index":"j","member":"type"}},"GROUP"]},"body":[{"If":{"test":{"OneOf":[["jira.auditing.group.created","jira.auditing.group.deleted","Group created","Group deleted"],{"Field":"event.action"}]},"body":[{"Put":{"path":"group.name","value":{"Member":{"list":"jira.audit.affected_objects","index":"j","member":"name"}}}}]}},{"If":{"test":{"OneOf":[["jira.auditing.user.added.to.group","jira.auditing.user.removed.from.group","User added to group","User removed from group"],{"Field":"event.action"}]},"body":[{"Put":{"path":"user.target.group.name","value":{"Member":{"list":"jira.audit.affected_objects","index":"j","member":"name"}}}},{"Put":{"path":"user.target.group.id","value":{"Member":{"list":"jira.audit.affected_objects","index":"j","member":"id"}}}}]}}]}},{"If":{"test":{"Equals":[{"Member":{"list":"jira.audit.affected_objects","index":"j","member":"type"}},"USER"]},"body":[{"If":{"test":{"OneOf":[["jira.auditing.user.created","jira.auditing.user.deleted","jira.auditing.user.password.changed","jira.auditing.user.updated","jira.auditing.user.added.to.group","jira.auditing.user.removed.from.group","User created","User deleted","User added to group","User removed from group"],{"Field":"event.action"}]},"body":[{"Put":{"path":"user.target.name","value":{"Member":{"list":"jira.audit.affected_objects","index":"j","member":"name"}}}},{"Put":{"path":"user.target.id","value":{"Member":{"list":"jira.audit.affected_objects","index":"j","member":"id"}}}}]}}]}}]}}]}},{"If":{"test":{"NotNull":{"Field":"jira.audit.changed_values"}},"body":[{"Walk":{"index":"j","list":"jira.audit.changed_values","body":[{"If":{"test":{"OneOf":[["jira.auditing.user.renamed","User renamed"],{"Field":"event.action"}]},"body":[{"If":{"test":{"Equals":[{"Member":{"list":"jira.audit.changed_values","index":"j","member":"i18nKey"}},"common.words.username"]},"body":[{"Put":{"path":"user.changes.name","value":{"Member":{"list":"jira.audit.changed_values","index":"j","member":"to"}}}},{"Put":{"path":"user.target.name","value":{"Member":{"list":"jira.audit.changed_values","index":"j","member":"from"}}}}]}}]}},{"If":{"test":{"OneOf":[["jira.auditing.user.created","jira.auditing.user.updated","User created","User updated"],{"Field":"event.action"}]},"body":[{"If":{"test":{"Equals":[{"Member":{"list":"jira.audit.changed_values","index":"j","member":"i18nKey"}},"common.words.username"]},"body":[{"Put":{"path":"user.changes.name","value":{"Member":{"list":"jira.audit.changed_values","index":"j","member":"to"}}}},{"If":{"test":{"NotNull":{"Member":{"list":"jira.audit.changed_values","index":"j","member":"from"}}},"body":[{"Put":{"path":"user.target.name","value":{"Member":{"list":"jira.audit.changed_values","index":"j","member":"from"}}}}]}}]}},{"If":{"test":{"Equals":[{"Member":{"list":"jira.audit.changed_values","index":"j","member":"i18nKey"}},"common.words.email"]},"body":[{"Put":{"path":"user.changes.email","value":{"Member":{"list":"jira.audit.changed_values","index":"j","member":"to"}}}},{"If":{"test":{"NotNull":{"Member":{"list":"jira.audit.changed_values","index":"j","member":"from"}}},"body":[{"Put":{"path":"user.target.email","value":{"Member":{"list":"jira.audit.changed_values","index":"j","member":"from"}}}}]}}]}},{"If":{"test":{"Equals":[{"Member":{"list":"jira.audit.changed_values","index":"j","member":"i18nKey"}},"common.words.fullname"]},"body":[{"Put":{"path":"user.changes.full_name","value":{"Member":{"list":"jira.audit.changed_values","index":"j","member":"to"}}}},{"If":{"test":{"NotNull":{"Member":{"list":"jira.audit.changed_values","index":"j","member":"from"}}},"body":[{"Put":{"path":"user.target.full_name","value":{"Member":{"list":"jira.audit.changed_values","index":"j","member":"from"}}}}]}}]}}]}}]}}]}}],"regexes":[]}}]}"#
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

            // The pipeline's script, emitted as the runtime's equivalent
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
