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
                    // The pipeline's script, emitted as the runtime's equivalent
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

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan_params(
                event,
                compiled_painless!(
                    "0151577661d8f8cc0446d3e73027acb30cf0391240efb9f78b3ece903a25f837",
                    r#"{"params":{"LookupMerge":{"statements":[{"Literal":{"Set":{"path":"event.kind","value":{"Literal":"event"}}}},{"Literal":{"Set":{"path":"event.type","value":{"Literal":["info"]}}}},{"If":{"test":[[{"Compare":{"path":"event.action","wanted":"Null","negated":false,"lowered":false}}]],"then":["Return"],"alt":[]}},{"If":{"test":[["Never"]],"then":["Return"],"alt":[]}}],"whole":false}},"known":[]}"#,
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "80e2673cae3034e0c1bf48284bc31dc887aadba7b04487a1aaffc3570cd9020c",
                        r#"{"params":null,"known":[{"ItemWrites":{"body":[{"If":{"test":{"IsNull":{"Field":"event.action"}},"body":["Return"]}},{"If":{"test":{"IsNull":{"Field":"group"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"group","value":{"Local":"map"}}}]}},{"If":{"test":{"IsNull":{"Field":"user"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"user","value":{"Local":"map"}}}]}},{"If":{"test":{"IsNull":{"Field":"user.target"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"user.target","value":{"Local":"map"}}}]}},{"If":{"test":{"IsNull":{"Field":"user.changes"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"user.changes","value":{"Local":"map"}}}]}},{"If":{"test":{"IsNull":{"Field":"user.target.group"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"user.target.group","value":{"Local":"map"}}}]}},{"If":{"test":{"NotNull":{"Field":"confluence.audit.affected_objects"}},"body":[{"Walk":{"index":"j","list":"confluence.audit.affected_objects","body":[{"If":{"test":{"Equals":[{"Member":{"list":"confluence.audit.affected_objects","index":"j","member":"type"}},"Group"]},"body":[{"Declare":{"name":"group_name","value":{"Member":{"list":"confluence.audit.affected_objects","index":"j","member":"name"}}}},{"Declare":{"name":"group_id","value":{"Member":{"list":"confluence.audit.affected_objects","index":"j","member":"id"}}}},{"If":{"test":{"NotNull":{"Field":"_config.atlassian_cloud"}},"body":[{"Declare":{"name":"m","value":{"Matcher":{"regex":0,"subject":{"Local":"group_name"}}}}},{"If":{"test":{"Finds":"m"},"body":[{"Assign":{"name":"group_name","value":{"Group":{"matcher":"m","group":1}}}},{"Assign":{"name":"group_id","value":{"Group":{"matcher":"m","group":2}}}}]}}]}},{"If":{"test":{"OneOf":[["audit.logging.summary.group.created","audit.logging.summary.group.deleted","Group created","Group deleted"],{"Field":"event.action"}]},"body":[{"Put":{"path":"group.name","value":{"Local":"group_name"}}},{"Put":{"path":"group.id","value":{"Local":"group_id"}}}]}},{"If":{"test":{"OneOf":[["audit.logging.summary.group.membership.added","audit.logging.summary.group.membership.removed","User added to group","User removed from group"],{"Field":"event.action"}]},"body":[{"Put":{"path":"user.target.group.name","value":{"Local":"group_name"}}},{"Put":{"path":"user.target.group.id","value":{"Local":"group_id"}}}]}}]}},{"If":{"test":{"Equals":[{"Member":{"list":"confluence.audit.affected_objects","index":"j","member":"type"}},"User"]},"body":[{"If":{"test":{"OneOf":[["audit.logging.summary.user.created","audit.logging.summary.user.deleted","audit.logging.summary.user.password.changed","audit.logging.summary.user.updated","User created","User deleted","User details updated"],{"Field":"event.action"}]},"body":[{"Put":{"path":"user.target.full_name","value":{"Member":{"list":"confluence.audit.affected_objects","index":"j","member":"name"}}}},{"Put":{"path":"user.target.id","value":{"Member":{"list":"confluence.audit.affected_objects","index":"j","member":"id"}}}},{"If":{"test":{"NotNull":{"Member":{"list":"confluence.audit.affected_objects","index":"j","member":"uri"}}},"body":[{"Declare":{"name":"m","value":{"Matcher":{"regex":1,"subject":{"Member":{"list":"confluence.audit.affected_objects","index":"j","member":"uri"}}}}}},{"If":{"test":{"Finds":"m"},"body":[{"Put":{"path":"user.target.name","value":{"Group":{"matcher":"m","group":1}}}}]}}]}}]}},{"If":{"test":{"OneOf":[["audit.logging.summary.group.membership.added","audit.logging.summary.group.membership.removed","User added to group","User removed from group"],{"Field":"event.action"}]},"body":[{"Put":{"path":"user.target.name","value":{"Member":{"list":"confluence.audit.affected_objects","index":"j","member":"name"}}}},{"Put":{"path":"user.target.id","value":{"Member":{"list":"confluence.audit.affected_objects","index":"j","member":"id"}}}}]}},{"If":{"test":{"OneOf":[["audit.logging.summary.login.success","audit.logging.summary.login.failed"],{"Field":"event.action"}]},"body":[{"Put":{"path":"user.full_name","value":{"Member":{"list":"confluence.audit.affected_objects","index":"j","member":"name"}}}},{"Put":{"path":"user.id","value":{"Member":{"list":"confluence.audit.affected_objects","index":"j","member":"id"}}}},{"If":{"test":{"NotNull":{"Member":{"list":"confluence.audit.affected_objects","index":"j","member":"uri"}}},"body":[{"Declare":{"name":"m","value":{"Matcher":{"regex":2,"subject":{"Member":{"list":"confluence.audit.affected_objects","index":"j","member":"uri"}}}}}},{"If":{"test":{"Finds":"m"},"body":[{"Put":{"path":"user.name","value":{"Group":{"matcher":"m","group":1}}}}]}}]}}]}}]}}]}}]}},{"If":{"test":{"NotNull":{"Field":"confluence.audit.changed_values"}},"body":[{"Walk":{"index":"j","list":"confluence.audit.changed_values","body":[{"If":{"test":{"OneOf":[["audit.logging.summary.user.renamed","User renamed"],{"Field":"event.action"}]},"body":[{"If":{"test":{"Equals":[{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"i18nKey"}},"audit.logging.changed.value.username"]},"body":[{"Put":{"path":"user.changes.name","value":{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"to"}}}},{"Put":{"path":"user.target.name","value":{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"from"}}}}]}}]}},{"If":{"test":{"OneOf":[["audit.logging.summary.user.created","audit.logging.summary.user.updated","User created","User details updated"],{"Field":"event.action"}]},"body":[{"If":{"test":{"Equals":[{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"i18nKey"}},"Username"]},"body":[{"Put":{"path":"user.changes.name","value":{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"to"}}}},{"Put":{"path":"user.target.name","value":{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"to"}}}},{"If":{"test":{"NotNull":{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"from"}}},"body":[{"Put":{"path":"user.target.name","value":{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"from"}}}}]}}]}},{"If":{"test":{"Equals":[{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"i18nKey"}},"Email"]},"body":[{"Put":{"path":"user.changes.email","value":{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"to"}}}},{"Put":{"path":"user.target.email","value":{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"to"}}}},{"If":{"test":{"NotNull":{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"from"}}},"body":[{"Put":{"path":"user.target.email","value":{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"from"}}}}]}}]}},{"If":{"test":{"Equals":[{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"i18nKey"}},"Display name"]},"body":[{"Put":{"path":"user.changes.full_name","value":{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"to"}}}},{"Put":{"path":"user.target.full_name","value":{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"to"}}}},{"If":{"test":{"NotNull":{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"from"}}},"body":[{"Put":{"path":"user.target.full_name","value":{"Member":{"list":"confluence.audit.changed_values","index":"j","member":"from"}}}}]}}]}}]}}]}}]}}],"regexes":[{"source":"(.+):(\\b[0-9a-f]{8}\\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\\b[0-9a-f]{12}\\b)$","compiled":"(.+):(\\b[0-9a-f]{8}\\b-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-\\b[0-9a-f]{12}\\b)$"},{"source":"\\?username=([a-zA-Z0-9._-]+)$","compiled":"\\?username=([a-zA-Z0-9._-]+)$"},{"source":"\\?username=([a-zA-Z0-9._-]+)$","compiled":"\\?username=([a-zA-Z0-9._-]+)$"}]}}]}"#
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
