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
            let _cond = {
                event.get("organization").is_some_and(|v| v.is_string())
                    && event.get("division").is_some_and(|v| v.is_string())
                    && event.get("team").is_some_and(|v| v.is_string())
            };
            if _cond {
                event.remove("organization");
                event.remove("division");
                event.remove("team");
            }

            event.set("ecs.version", json!("8.16.0"))?;

            event.append("event.category", json!("iam"))?;

            event.append("event.type", json!("group"))?;

            event.set("event.kind", json!("event"))?;

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

            let _cond = { !event.has_value("json.events") };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            let _cond =
                { event.has_value("json.id.time") && event.get_str("json.id.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.id.time") {
                        match parse_date_out(
                            &date_str,
                            &[
                                "ISO8601",
                                "yyyy-MM-dd'T'HH:mm:ss",
                                "yyyy-MM-dd'T'HH:mm:ssZ",
                                "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
                                "yyyy/MM/dd HH:mm:ss z",
                            ],
                            Some("UTC"),
                            None,
                        ) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.id.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.append(
                        "error.message",
                        json!(
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                {
                    let mut values = Vec::new();
                    if let Some(v) = event.get("json.events") {
                        values.push(v.clone());
                    }
                    if let Some(v) = event.get("json.id") {
                        values.push(v.clone());
                    }
                    if !values.is_empty() {
                        event.set("_id", json!(fingerprint_default(&values)))?;
                    }
                }
                Ok(())
            })();

            if event.has_value("json.events.name") {
                event.rename("json.events.name", "event.action")?;
            }

            if event.has_value("json.id.applicationName") {
                event.rename("json.id.applicationName", "event.provider")?;
            }

            if event.has_value("json.id.uniqueQualifier") {
                if let Some(val) = event.get("json.id.uniqueQualifier") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.id.uniqueQualifier".into(),
                            message,
                        }
                    })?;
                    event.set("event.id", converted)?;
                }
            }

            if event.has_value("json.actor.email") {
                event.rename("json.actor.email", "source.user.email")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(v) = event.get("source.user.email").cloned() {
                    event.set("user.email", v)?;
                }
                Ok(())
            })();

            if event.has_value("json.actor.profileId") {
                if let Some(val) = event.get("json.actor.profileId") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.actor.profileId".into(),
                            message,
                        }
                    })?;
                    event.set("source.user.id", converted)?;
                }
            }

            if event.has_value("json.ipAddress") {
                if let Some(val) = event.get("json.ipAddress") {
                    let converted =
                        convert_value(val, "ip").map_err(|message| TransformError::ParseError {
                            path: "json.ipAddress".into(),
                            message,
                        })?;
                    event.set("source.ip", converted)?;
                }
            }

            if event.has_value("json.kind") {
                event.rename("json.kind", "google_workspace.kind")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.id.customerId") {
                    if let Some(val) = event.get("json.id.customerId") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.id.customerId".into(),
                                message,
                            }
                        })?;
                        event.set("organization.id", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_customer_id_to_string",
                )?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor {} with tag {} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
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

            if event.has_value("json.actor.callerType") {
                event.rename("json.actor.callerType", "google_workspace.actor.type")?;
            }

            if event.has_value("json.actor.key") {
                event.rename("json.actor.key", "google_workspace.actor.key")?;
            }

            if event.has_value("json.ownerDomain") {
                event.rename("json.ownerDomain", "google_workspace.organization.domain")?;
            }

            if event.has_value("json.events.type") {
                event.rename("json.events.type", "google_workspace.event.type")?;
            }

            let _cond = { event.has_value("source.user.id") };
            if _cond {
                if let Some(v) = event.get("source.user.id").cloned() {
                    event.set("user.id", v)?;
                }
            }

            let _cond = {
                event.has_value("source.user.email")
                    && event.get("source.user.email").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                // Painless script
                // Source: String[] splitmail = ctx.source.user.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.user == null) {\n  ctx.user = new HashMap();\n} ctx.user.name = splitmail[0]; ctx.source.user.name = splitmail[0]; ctx.user.domain = splitmail[1]; ctx.source.user.domain = splitmail[1];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String[] splitmail = ctx.source.user.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.user == null) {\n  ctx.user = new HashMap();\n} ctx.user.name = splitmail[0]; ctx.source.user.name = splitmail[0]; ctx.user.domain = splitmail[1]; ctx.source.user.domain = splitmail[1];\n"#
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

            let _cond = { event.has_value("source.user.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("source.user.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                [
                    "change_basic_setting",
                    "change_identity_setting",
                    "change_info_setting",
                    "change_new_members_restrictions_setting",
                    "change_post_replies_setting",
                    "change_spam_moderation_setting",
                    "change_topic_setting",
                    "change_acl_permission",
                    "approve_join_request",
                    "join",
                ]
                .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("change"))?;
            }

            let _cond = {
                [
                    "change_basic_setting",
                    "change_identity_setting",
                    "change_info_setting",
                    "change_new_members_restrictions_setting",
                    "change_post_replies_setting",
                    "change_spam_moderation_setting",
                    "change_topic_setting",
                    "add_info_setting",
                    "remove_info_setting",
                ]
                .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.category", json!("configuration"))?;
            }

            let _cond = {
                [
                    "accept_invitation",
                    "request_to_join",
                    "ban_user_with_moderation",
                    "revoke_invitation",
                    "invite_user",
                    "reject_join_request",
                    "reinvite_user",
                    "moderate_message",
                    "always_post_from_user",
                ]
                .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("info"))?;
            }

            let _cond = {
                [
                    "accept_invitation",
                    "approve_join_request",
                    "join",
                    "request_to_join",
                    "ban_user_with_moderation",
                    "revoke_invitation",
                    "invite_user",
                    "reject_join_request",
                    "reinvite_user",
                    "add_user",
                    "remove_user",
                ]
                .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("user"))?;
            }

            let _cond = {
                ["create_group", "add_info_setting", "add_user"]
                    .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("creation"))?;
            }

            let _cond = {
                ["delete_group", "remove_info_setting", "remove_user"]
                    .contains(&event.get_str("event.action").unwrap_or(""))
            };
            if _cond {
                event.append("event.type", json!("deletion"))?;
            }

            let _cond = {
                event.has_value("json.events.parameters")
                    && event
                        .get("json.events.parameters")
                        .is_some_and(|v| v.is_array())
            };
            if _cond {
                // Painless script
                // Source: if (ctx.google_workspace.groups == null) {\n  ctx.google_workspace.groups = new HashMap();\n} for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"value\"] != null) {\n    ctx.google_workspace.groups[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"value\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"intValue\"] != null) {\n    ctx.google_workspace.groups[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"intValue\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiValue\"] != null) {\n    ctx.google_workspace.groups[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiValue\"];\n  }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.google_workspace.groups == null) {\n  ctx.google_workspace.groups = new HashMap();\n} for (int i = 0; i < ctx.json.events.parameters.length; ++i) {\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"value\"] != null) {\n    ctx.google_workspace.groups[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"value\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"intValue\"] != null) {\n    ctx.google_workspace.groups[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"intValue\"];\n  }\n  if (ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiValue\"] != null) {\n    ctx.google_workspace.groups[ctx[\"json\"][\"events\"][\"parameters\"][i][\"name\"]] = ctx[\"json\"][\"events\"][\"parameters\"][i][\"multiValue\"];\n  }\n}\n"#
                    ),
                )?;
            }

            if event.has_value("google_workspace.groups.group_email") {
                event.rename(
                    "google_workspace.groups.group_email",
                    "google_workspace.groups.email",
                )?;
            }

            if event.has_value("google_workspace.groups.new_value_repeated") {
                event.rename(
                    "google_workspace.groups.new_value_repeated",
                    "google_workspace.groups.new_value",
                )?;
            }

            if event.has_value("google_workspace.groups.old_value_repeated") {
                event.rename(
                    "google_workspace.groups.old_value_repeated",
                    "google_workspace.groups.old_value",
                )?;
            }

            if event.has_value("google_workspace.groups.user_email") {
                event.rename(
                    "google_workspace.groups.user_email",
                    "google_workspace.groups.member.email",
                )?;
            }

            if event.has_value("google_workspace.groups.basic_setting") {
                event.rename(
                    "google_workspace.groups.basic_setting",
                    "google_workspace.groups.setting",
                )?;
            }

            if event.has_value("google_workspace.groups.identity_setting") {
                event.rename(
                    "google_workspace.groups.identity_setting",
                    "google_workspace.groups.setting",
                )?;
            }

            if event.has_value("google_workspace.groups.info_setting") {
                event.rename(
                    "google_workspace.groups.info_setting",
                    "google_workspace.groups.setting",
                )?;
            }

            if event.has_value("google_workspace.groups.new_members_restrictions_setting") {
                event.rename(
                    "google_workspace.groups.new_members_restrictions_setting",
                    "google_workspace.groups.setting",
                )?;
            }

            if event.has_value("google_workspace.groups.post_replies_setting") {
                event.rename(
                    "google_workspace.groups.post_replies_setting",
                    "google_workspace.groups.setting",
                )?;
            }

            if event.has_value("google_workspace.groups.spam_moderation_setting") {
                event.rename(
                    "google_workspace.groups.spam_moderation_setting",
                    "google_workspace.groups.setting",
                )?;
            }

            if event.has_value("google_workspace.groups.topic_setting") {
                event.rename(
                    "google_workspace.groups.topic_setting",
                    "google_workspace.groups.setting",
                )?;
            }

            if event.has_value("google_workspace.groups.message_id") {
                event.rename(
                    "google_workspace.groups.message_id",
                    "google_workspace.groups.message.id",
                )?;
            }

            if event.has_value("google_workspace.groups.message_moderation_action") {
                event.rename(
                    "google_workspace.groups.message_moderation_action",
                    "google_workspace.groups.message.moderation_action",
                )?;
            }

            if event.has_value("google_workspace.groups.member_role") {
                event.rename(
                    "google_workspace.groups.member_role",
                    "google_workspace.groups.member.role",
                )?;
            }

            let _cond = { event.get_str("google_workspace.groups.status") == Some("failed") };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("google_workspace.groups.status") == Some("success") };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("google_workspace.groups.email")
                    && event
                        .get("google_workspace.groups.email")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("@"))
                            }
                            serde_json::Value::String(s) => s.contains("@"),
                            _ => false,
                        })
            };
            if _cond {
                // Painless script
                // Source: String[] splitmail = ctx.google_workspace.groups.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.group == null) {\n  ctx.group = new HashMap();\n} ctx.group.name = splitmail[0]; ctx.group.domain = splitmail[1];\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String[] splitmail = ctx.google_workspace.groups.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.group == null) {\n  ctx.group = new HashMap();\n} ctx.group.name = splitmail[0]; ctx.group.domain = splitmail[1];\n"#
                    ),
                )?;
            }

            let _cond = {
                event.has_value("google_workspace.groups.member.email")
                    && event
                        .get("google_workspace.groups.member.email")
                        .is_some_and(|v| match v {
                            serde_json::Value::Array(a) => {
                                a.iter().any(|x| x.as_str() == Some("@"))
                            }
                            serde_json::Value::String(s) => s.contains("@"),
                            _ => false,
                        })
            };
            if _cond {
                // Painless script
                // Source: String[] splitmail = ctx.google_workspace.groups.member.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.user == null) {\n  ctx.user = new HashMap();\n} if (ctx.user.target == null) {\n  ctx.user.target = new HashMap();\n} if (ctx.related == null) {\n  ctx.related = new HashMap();\n} if (ctx.related.user == null) {\n  ctx.related.user = new ArrayList();\n} ctx.related.user.add(splitmail[0]); ctx.user.target.name = splitmail[0]; ctx.user.target.domain = splitmail[1]; ctx.user.target.email = ctx.google_workspace.groups.member.email;\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"String[] splitmail = ctx.google_workspace.groups.member.email.splitOnToken('@'); if (splitmail.length != 2) {\n  return;\n} if (ctx.user == null) {\n  ctx.user = new HashMap();\n} if (ctx.user.target == null) {\n  ctx.user.target = new HashMap();\n} if (ctx.related == null) {\n  ctx.related = new HashMap();\n} if (ctx.related.user == null) {\n  ctx.related.user = new ArrayList();\n} ctx.related.user.add(splitmail[0]); ctx.user.target.name = splitmail[0]; ctx.user.target.domain = splitmail[1]; ctx.user.target.email = ctx.google_workspace.groups.member.email;\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("group.name") };
            if _cond {
                if let Some(v) = event.get("group.name").cloned() {
                    event.set("user.target.group.name", v)?;
                }
            }

            let _cond = { event.has_value("group.domain") };
            if _cond {
                if let Some(v) = event.get("group.domain").cloned() {
                    event.set("user.target.group.domain", v)?;
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

            event.remove("json");

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
