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

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan_params(
                event,
                compiled_painless!(
                    "0151577661d8f8cc0446d3e73027acb30cf0391240efb9f78b3ece903a25f837",
                    r#"{"params":{"LookupMerge":{"statements":[{"Literal":{"Set":{"path":"event.kind","value":{"Literal":"event"}}}},{"Literal":{"Set":{"path":"event.type","value":{"Literal":["info"]}}}},{"If":{"test":[[{"Compare":{"path":"event.action","wanted":"Null","negated":false,"lowered":false}}]],"then":["Return"],"alt":[]}},{"If":{"test":[["Never"]],"then":["Return"],"alt":[]}}],"whole":false}},"known":[]}"#,
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "bd1582d50786335ec6a753d8108fd35f8047b86f8f57ad466dd6f08d9428986d",
                        r#"{"params":null,"known":[{"ItemWrites":{"body":[{"If":{"test":{"IsNull":{"Field":"event.action"}},"body":["Return"]}},{"If":{"test":{"IsNull":{"Field":"group"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"group","value":{"Local":"map"}}}]}},{"If":{"test":{"IsNull":{"Field":"user"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"user","value":{"Local":"map"}}}]}},{"If":{"test":{"IsNull":{"Field":"user.target"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"user.target","value":{"Local":"map"}}}]}},{"If":{"test":{"IsNull":{"Field":"user.changes"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"user.changes","value":{"Local":"map"}}}]}},{"If":{"test":{"IsNull":{"Field":"user.target.group"}},"body":[{"Declare":{"name":"map","value":"EmptyMap"}},{"Put":{"path":"user.target.group","value":{"Local":"map"}}}]}},{"If":{"test":{"NotNull":{"Field":"bitbucket.audit.affected_objects"}},"body":[{"Walk":{"index":"j","list":"bitbucket.audit.affected_objects","body":[{"If":{"test":{"Equals":[{"Member":{"list":"bitbucket.audit.affected_objects","index":"j","member":"type"}},"GROUP"]},"body":[{"If":{"test":{"OneOf":[["bitbucket.service.user.audit.action.groupcreated","bitbucket.service.user.audit.action.groupdeleted"],{"Field":"event.action"}]},"body":[{"Put":{"path":"group.name","value":{"Member":{"list":"bitbucket.audit.affected_objects","index":"j","member":"name"}}}},{"Put":{"path":"group.id","value":{"Member":{"list":"bitbucket.audit.affected_objects","index":"j","member":"id"}}}}]}},{"If":{"test":{"OneOf":[["bitbucket.service.user.audit.action.groupmembershipscreated.user","bitbucket.service.user.audit.action.groupmembershipdeleted"],{"Field":"event.action"}]},"body":[{"Put":{"path":"user.target.group.name","value":{"Member":{"list":"bitbucket.audit.affected_objects","index":"j","member":"name"}}}},{"Put":{"path":"user.target.group.id","value":{"Member":{"list":"bitbucket.audit.affected_objects","index":"j","member":"id"}}}}]}}]}},{"If":{"test":{"Equals":[{"Member":{"list":"bitbucket.audit.affected_objects","index":"j","member":"type"}},"USER"]},"body":[{"If":{"test":{"OneOf":[["bitbucket.service.user.audit.action.usercreated","bitbucket.service.user.audit.action.userdeleted","bitbucket.service.user.audit.action.userrenamed","bitbucket.service.user.audit.action.usercredentialupdated","bitbucket.service.user.audit.action.groupmembershipscreated.user","bitbucket.service.user.audit.action.groupmembershipdeleted"],{"Field":"event.action"}]},"body":[{"Put":{"path":"user.target.name","value":{"Member":{"list":"bitbucket.audit.affected_objects","index":"j","member":"name"}}}},{"Put":{"path":"user.target.id","value":{"Member":{"list":"bitbucket.audit.affected_objects","index":"j","member":"id"}}}}]}}]}}]}}]}},{"If":{"test":{"NotNull":{"Field":"bitbucket.audit.changed_values"}},"body":[{"Walk":{"index":"j","list":"bitbucket.audit.changed_values","body":[{"If":{"test":{"OneOf":[["bitbucket.service.user.audit.action.userrenamed"],{"Field":"event.action"}]},"body":[{"If":{"test":{"Equals":[{"Member":{"list":"bitbucket.audit.changed_values","index":"j","member":"i18nKey"}},"bitbucket.service.user.audit.attribute.user.name"]},"body":[{"Put":{"path":"user.changes.name","value":{"Member":{"list":"bitbucket.audit.changed_values","index":"j","member":"to"}}}},{"Put":{"path":"user.target.name","value":{"Member":{"list":"bitbucket.audit.changed_values","index":"j","member":"from"}}}}]}}]}}]}}]}}],"regexes":[]}}]}"#
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
