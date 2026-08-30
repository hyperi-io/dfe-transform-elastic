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
            event.set("ecs.version", json!("9.3.0"))?;

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

            let _cond = {
                event.has_value("error.message")
                    && !event.has_value("message")
                    && !event.has_value("event.original")
            };
            if _cond {
                return Ok(TransformResult::Continue);
            }

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

            let _cond = { event.has_value("event.original") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    parse_json_field(event, "event.original", "json")?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "json")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "parse_event_original_json",
                    )?;
                    event.append(
                        "error.message",
                        json!(format!(
                            "Processor {} with tag {} failed with message: {}",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
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
            }

            let _cond = { event.has_value("json.attributes.location") };
            if _cond {
                // Painless script
                // Source: def loc = ctx.json.attributes.location;\nfor (def key : new ArrayList(loc.keySet())) {\n  def val = loc.get(key);\n  if (val instanceof String && val.isEmpty()) {\n    loc.remove(key);\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def loc = ctx.json.attributes.location;\nfor (def key : new ArrayList(loc.keySet())) {\n  def val = loc.get(key);\n  if (val instanceof String && val.isEmpty()) {\n    loc.remove(key);\n  }\n}"#
                    ),
                )?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "event.id")?;
            }

            if event.has_value("json.attributes.action") {
                event.rename("json.attributes.action", "event.action")?;
            }

            let _cond = { event.has_value("json.attributes.time") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.attributes.time") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.attributes.time".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            let _cond = { event.has_value("json.attributes.processedAt") };
            if _cond {
                if let Some(date_str) = event.get_as_string("json.attributes.processedAt") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("event.created", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.attributes.processedAt".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
            }

            if event.has_value("json.attributes.context") {
                event.rename("json.attributes.context", "atlassian_cloud.audit.context")?;
            }

            if event.has_value("json.attributes.container") {
                event.rename(
                    "json.attributes.container",
                    "atlassian_cloud.audit.container",
                )?;
            }

            if event.has_value("json.attributes.location.ip") {
                event.rename("json.attributes.location.ip", "source.ip")?;
            }

            if event.has_value("json.attributes.location.countryName") {
                event.rename(
                    "json.attributes.location.countryName",
                    "source.geo.country_name",
                )?;
            }

            if event.has_value("json.attributes.location.regionName") {
                event.rename(
                    "json.attributes.location.regionName",
                    "source.geo.region_name",
                )?;
            }

            if event.has_value("json.attributes.location.city") {
                event.rename("json.attributes.location.city", "source.geo.city_name")?;
            }

            if event.has_value("json.attributes.location.geo") {
                event.rename(
                    "json.attributes.location.geo",
                    "atlassian_cloud.audit.location.geo",
                )?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
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
                        if let Some(v) = geo.get("network") {
                            event.set("source.as.network", v.clone())?;
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

            event.remove("source.as.ip");
            event.remove("source.as.network");

            if event.has_value("json.message.content") {
                event.rename("json.message.content", "message")?;
            }

            if event.has_value("json.message.format") {
                event.rename(
                    "json.message.format",
                    "atlassian_cloud.audit.message.format",
                )?;
            }

            if event.has_value("json.attributes.actor.auth.authType") {
                event.rename(
                    "json.attributes.actor.auth.authType",
                    "atlassian_cloud.audit.actor.auth.auth_type",
                )?;
            }

            if event.has_value("json.attributes.actor.auth.tokenId") {
                event.rename(
                    "json.attributes.actor.auth.tokenId",
                    "atlassian_cloud.audit.actor.auth.token_id",
                )?;
            }

            if event.has_value("json.attributes.actor.auth.tokenLabel") {
                event.rename(
                    "json.attributes.actor.auth.tokenLabel",
                    "atlassian_cloud.audit.actor.auth.token_label",
                )?;
            }

            if event.has_value("json.attributes.actor.onBehalfOf.id") {
                event.rename(
                    "json.attributes.actor.onBehalfOf.id",
                    "atlassian_cloud.audit.actor.on_behalf_of.id",
                )?;
            }

            if event.has_value("json.attributes.actor.onBehalfOf.name") {
                event.rename(
                    "json.attributes.actor.onBehalfOf.name",
                    "atlassian_cloud.audit.actor.on_behalf_of.name",
                )?;
            }

            if event.has_value("json.attributes.actor.onBehalfOf.email") {
                event.rename(
                    "json.attributes.actor.onBehalfOf.email",
                    "atlassian_cloud.audit.actor.on_behalf_of.email",
                )?;
            }

            if event.has_value("json.attributes.actor.app.id") {
                event.rename(
                    "json.attributes.actor.app.id",
                    "atlassian_cloud.audit.actor.app.id",
                )?;
            }

            if event.has_value("json.attributes.actor.app.type") {
                event.rename(
                    "json.attributes.actor.app.type",
                    "atlassian_cloud.audit.actor.app.type",
                )?;
            }

            if event.has_value("json.attributes.actor.app.attributes") {
                event.rename(
                    "json.attributes.actor.app.attributes",
                    "atlassian_cloud.audit.actor.app.attributes",
                )?;
            }

            let _cond = { event.has_value("json.attributes.actor") };
            if _cond {
                // Painless script
                // Source: def actor = ctx.json.attributes.actor;\nboolean isUser = false;\nif (actor.email instanceof String && actor.email.indexOf('@') >= 0) {\n  isUser = true;\n} else if (actor.id instanceof String) {\n  String id = actor.id;\n  if (id.contains(':') || id ==~ /^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/) {\n    isUser = true;\n  }\n}\nif (isUser) {\n  ctx.user = ctx.user != null ? ctx.user : new HashMap();\n  if (actor.id != null) {\n    ctx.user.id = actor.id;\n  }\n  if (actor.name != null) {\n    ctx.user.name = actor.name;\n  }\n  if (actor.email instanceof String && actor.email.indexOf('@') >= 0) {\n    ctx.user.email = actor.email;\n  }\n} else {\n  ctx.atlassian_cloud = ctx.atlassian_cloud != null ? ctx.atlassian_cloud : new HashMap();\n  ctx.atlassian_cloud.audit = ctx.atlassian_cloud.audit != null ? ctx.atlassian_cloud.audit : new HashMap();\n  ctx.atlassian_cloud.audit.actor = ctx.atlassian_cloud.audit.actor != null ? ctx.atlassian_cloud.audit.actor : new HashMap();\n  def app = ctx.atlassian_cloud.audit.actor.app != null ? ctx.atlassian_cloud.audit.actor.app : new HashMap();\n  if (actor.id != null) {\n    app.id = actor.id;\n  }\n  if (actor.name != null) {\n    app.name = actor.name;\n  }\n  ctx.atlassian_cloud.audit.actor.app = app;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def actor = ctx.json.attributes.actor;\nboolean isUser = false;\nif (actor.email instanceof String && actor.email.indexOf('@') >= 0) {\n  isUser = true;\n} else if (actor.id instanceof String) {\n  String id = actor.id;\n  if (id.contains(':') || id ==~ /^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$/) {\n    isUser = true;\n  }\n}\nif (isUser) {\n  ctx.user = ctx.user != null ? ctx.user : new HashMap();\n  if (actor.id != null) {\n    ctx.user.id = actor.id;\n  }\n  if (actor.name != null) {\n    ctx.user.name = actor.name;\n  }\n  if (actor.email instanceof String && actor.email.indexOf('@') >= 0) {\n    ctx.user.email = actor.email;\n  }\n} else {\n  ctx.atlassian_cloud = ctx.atlassian_cloud != null ? ctx.atlassian_cloud : new HashMap();\n  ctx.atlassian_cloud.audit = ctx.atlassian_cloud.audit != null ? ctx.atlassian_cloud.audit : new HashMap();\n  ctx.atlassian_cloud.audit.actor = ctx.atlassian_cloud.audit.actor != null ? ctx.atlassian_cloud.audit.actor : new HashMap();\n  def app = ctx.atlassian_cloud.audit.actor.app != null ? ctx.atlassian_cloud.audit.actor.app : new HashMap();\n  if (actor.id != null) {\n    app.id = actor.id;\n  }\n  if (actor.name != null) {\n    app.name = actor.name;\n  }\n  ctx.atlassian_cloud.audit.actor.app = app;\n}"#
                    ),
                )?;
            }

            event.set("observer.vendor", json!("Atlassian"))?;

            event.set("observer.product", json!("Atlassian Cloud"))?;

            event.set("observer.type", json!("saas"))?;

            let _cond = { event.has_value("event.action") };
            if _cond {
                // Painless script
                // Source: def action = ctx.event.action;\nctx.event.kind = 'event';\ndef mapping = null;\nif (params.exact.containsKey(action)) {\n  mapping = params.exact[action];\n}\nif (mapping == null && params.prefixes instanceof List) {\n  for (def entry : params.prefixes) {\n    if (action.startsWith(entry.prefix)) {\n      mapping = entry;\n      break;\n    }\n  }\n}\nif (mapping == null && action.endsWith(params.policy_suffix)) {\n  mapping = params.policy;\n}\nif (mapping == null) {\n  mapping = params.default;\n}\nctx.event.category = mapping.category;\nctx.event.type = mapping.type;\nif (mapping.containsKey('outcome')) {\n  ctx.event.outcome = mapping.outcome;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def action = ctx.event.action;\nctx.event.kind = 'event';\ndef mapping = null;\nif (params.exact.containsKey(action)) {\n  mapping = params.exact[action];\n}\nif (mapping == null && params.prefixes instanceof List) {\n  for (def entry : params.prefixes) {\n    if (action.startsWith(entry.prefix)) {\n      mapping = entry;\n      break;\n    }\n  }\n}\nif (mapping == null && action.endsWith(params.policy_suffix)) {\n  mapping = params.policy;\n}\nif (mapping == null) {\n  mapping = params.default;\n}\nctx.event.category = mapping.category;\nctx.event.type = mapping.type;\nif (mapping.containsKey('outcome')) {\n  ctx.event.outcome = mapping.outcome;\n}"#
                    ),
                    cached_params!(
                        "{\"exact\":{\"user_login\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"user_logout\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"user_login_failed\":{\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"failure\"},\"org_settings_update\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"}},\"prefixes\":[{\"prefix\":\"user_login_failed\",\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"failure\"},{\"prefix\":\"user_login\",\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"success\"},{\"prefix\":\"user_logout\",\"category\":[\"authentication\"],\"type\":[\"info\"],\"outcome\":\"success\"},{\"prefix\":\"organization_users_\",\"category\":[\"iam\"],\"type\":[\"info\"],\"outcome\":\"success\"},{\"prefix\":\"user_profile_\",\"category\":[\"iam\"],\"type\":[\"user\"],\"outcome\":\"success\"},{\"prefix\":\"user_invitation_\",\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"],\"outcome\":\"success\"},{\"prefix\":\"user_invit\",\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"],\"outcome\":\"success\"},{\"prefix\":\"user_deactiv\",\"category\":[\"iam\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\"},{\"prefix\":\"user_reactiv\",\"category\":[\"iam\"],\"type\":[\"user\",\"change\"],\"outcome\":\"success\"},{\"prefix\":\"user_remov\",\"category\":[\"iam\"],\"type\":[\"user\",\"deletion\"],\"outcome\":\"success\"},{\"prefix\":\"marketplace_app_install\",\"category\":[\"configuration\",\"package\"],\"type\":[\"installation\"],\"outcome\":\"success\"},{\"prefix\":\"marketplace_app_uninstall\",\"category\":[\"configuration\",\"package\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},{\"prefix\":\"api_token_creat\",\"category\":[\"iam\"],\"type\":[\"creation\"],\"outcome\":\"success\"},{\"prefix\":\"api_token_revok\",\"category\":[\"iam\"],\"type\":[\"deletion\"],\"outcome\":\"success\"},{\"prefix\":\"group_member\",\"category\":[\"iam\"],\"type\":[\"group\",\"change\"],\"outcome\":\"success\"},{\"prefix\":\"confluence_\",\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"success\"},{\"prefix\":\"jira_\",\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"success\"},{\"prefix\":\"bitbucket_\",\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"success\"}],\"default\":{\"category\":[\"configuration\"],\"type\":[\"info\"],\"outcome\":\"success\"},\"policy_suffix\":\"_policy_update\",\"policy\":{\"category\":[\"iam\",\"configuration\"],\"type\":[\"change\"],\"outcome\":\"success\"}}"
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

            let _cond = { event.has_value("atlassian_cloud.audit.actor.on_behalf_of.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("atlassian_cloud.audit.actor.on_behalf_of.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("atlassian_cloud.audit.actor.on_behalf_of.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("atlassian_cloud.audit.actor.on_behalf_of.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            event.remove("json");

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.append_unique("tags", json!("preserve_original_event"))?;
            }

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
