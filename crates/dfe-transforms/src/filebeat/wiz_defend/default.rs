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
            event.set("ecs.version", json!("8.17.0"))?;

            event.set("observer.product", json!("Defend"))?;

            event.set("observer.vendor", json!("Wiz"))?;

            event.set("event.kind", json!("alert"))?;

            event.append("event.type", json!("indicator"))?;

            event.append("event.category", json!("threat"))?;

            let _cond = { event.get("json").is_some_and(|v| v.is_object()) };
            if _cond {
                // Painless script
                // Source: def stringified_orig = Json.dump(ctx.json);\nif (stringified_orig != null) {\n  ctx.event = ctx.event ?: [:];\n  ctx.event.original = stringified_orig;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def stringified_orig = Json.dump(ctx.json);\nif (stringified_orig != null) {\n  ctx.event = ctx.event ?: [:];\n  ctx.event.original = stringified_orig;\n}\n"#
                    ),
                )?;
            }

            if event.has_value("json.cloudOrganizations") {
                event.rename("json.cloudOrganizations", "wiz.defend.cloudOrganizations")?;
            }

            let _cond = {
                event.has_value("json.createdAt") && event.get_str("json.createdAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.createdAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("wiz.defend.created_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.createdAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_createdAt")?;
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
            }

            if event.has_value("json.description") {
                event.rename("json.description", "wiz.defend.description")?;
            }

            if event.has_value("json.detectionUrl") {
                event.rename("json.detectionUrl", "wiz.defend.detection_url")?;
            }

            if let Some(v) = event
                .get("wiz.defend.detection_url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reference", v)?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "wiz.defend.id")?;
            }

            let _cond = { event.get("json.mitreTactics").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "json.mitreTactics", |event| {
                    event.append_unique(
                        "threat.tactic.id",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.mitreTactics") {
                event.rename("json.mitreTactics", "wiz.defend.mitreTactics")?;
            }

            let _cond = {
                event
                    .get("json.mitreTechniques")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.mitreTechniques", |event| {
                    event.append_unique(
                        "threat.technique.id",
                        json!(
                            event
                                .get("_ingest._value")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            if event.has_value("json.mitreTechniques") {
                event.rename("json.mitreTechniques", "wiz.defend.mitreTechniques")?;
            }

            if event.has_value("json.primaryActor.email") {
                event.rename("json.primaryActor.email", "wiz.defend.primary_actor.email")?;
            }

            let _cond = { event.has_value("wiz.defend.primary_actor.email") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("wiz.defend.primary_actor.email")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.primaryActor.externalId") {
                event.rename(
                    "json.primaryActor.externalId",
                    "wiz.defend.primary_actor.external_id",
                )?;
            }

            if event.has_value("json.primaryActor.id") {
                event.rename("json.primaryActor.id", "wiz.defend.primary_actor.id")?;
            }

            let _cond = { event.has_value("wiz.defend.primary_actor.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("wiz.defend.primary_actor.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.primaryActor.name") {
                event.rename("json.primaryActor.name", "wiz.defend.primary_actor.name")?;
            }

            let _cond = { event.has_value("wiz.defend.primary_actor.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("wiz.defend.primary_actor.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.primaryActor.nativeType") {
                event.rename(
                    "json.primaryActor.nativeType",
                    "wiz.defend.primary_actor.native_type",
                )?;
            }

            if event.has_value("json.primaryActor.type") {
                event.rename("json.primaryActor.type", "wiz.defend.primary_actor.type")?;
            }

            if event.has_value("json.primaryResource.cloudAccount.cloudPlatform") {
                event.rename(
                    "json.primaryResource.cloudAccount.cloudPlatform",
                    "wiz.defend.primary_resource.cloud_account.cloud_platform",
                )?;
            }

            if event.has_value("json.primaryResource.cloudAccount.externalId") {
                event.rename(
                    "json.primaryResource.cloudAccount.externalId",
                    "wiz.defend.primary_resource.cloud_account.external_id",
                )?;
            }

            if event.has_value("json.primaryResource.cloudAccount.id") {
                event.rename(
                    "json.primaryResource.cloudAccount.id",
                    "wiz.defend.primary_resource.cloud_account.id",
                )?;
            }

            if event.has_value("json.primaryResource.cloudProviderUrl") {
                event.rename(
                    "json.primaryResource.cloudProviderUrl",
                    "wiz.defend.primary_resource.cloud_provider_url",
                )?;
            }

            if event.has_value("json.primaryResource.externalId") {
                event.rename(
                    "json.primaryResource.externalId",
                    "wiz.defend.primary_resource.external_id",
                )?;
            }

            if event.has_value("json.primaryResource.id") {
                event.rename("json.primaryResource.id", "wiz.defend.primary_resource.id")?;
            }

            if event.has_value("json.primaryResource.kubernetesClusterId") {
                event.rename(
                    "json.primaryResource.kubernetesClusterId",
                    "wiz.defend.primary_resource.kubernetes_cluster_id",
                )?;
            }

            if event.has_value("json.primaryResource.kubernetesClusterName") {
                event.rename(
                    "json.primaryResource.kubernetesClusterName",
                    "wiz.defend.primary_resource.kubernetes_cluster_name",
                )?;
            }

            if event.has_value("json.primaryResource.kubernetesNamespaceId") {
                event.rename(
                    "json.primaryResource.kubernetesNamespaceId",
                    "wiz.defend.primary_resource.kubernetes_namespace_id",
                )?;
            }

            if event.has_value("json.primaryResource.kubernetesNamespaceName") {
                event.rename(
                    "json.primaryResource.kubernetesNamespaceName",
                    "wiz.defend.primary_resource.kubernetes_namespace_name",
                )?;
            }

            if event.has_value("json.primaryResource.kubernetesNodeId") {
                event.rename(
                    "json.primaryResource.kubernetesNodeId",
                    "wiz.defend.primary_resource.kubernetes_node_id",
                )?;
            }

            if event.has_value("json.primaryResource.kubernetesNodeName") {
                event.rename(
                    "json.primaryResource.kubernetesNodeName",
                    "wiz.defend.primary_resource.kubernetes_node_name",
                )?;
            }

            if event.has_value("json.primaryResource.name") {
                event.rename(
                    "json.primaryResource.name",
                    "wiz.defend.primary_resource.name",
                )?;
            }

            if event.has_value("json.primaryResource.nativeType") {
                event.rename(
                    "json.primaryResource.nativeType",
                    "wiz.defend.primary_resource.native_type",
                )?;
            }

            if event.has_value("json.primaryResource.providerUniqueId") {
                event.rename(
                    "json.primaryResource.providerUniqueId",
                    "wiz.defend.primary_resource.provider_unique_id",
                )?;
            }

            if event.has_value("json.primaryResource.region") {
                event.rename(
                    "json.primaryResource.region",
                    "wiz.defend.primary_resource.region",
                )?;
            }

            if event.has_value("json.primaryResource.status") {
                event.rename(
                    "json.primaryResource.status",
                    "wiz.defend.primary_resource.status",
                )?;
            }

            if event.has_value("json.primaryResource.type") {
                event.rename(
                    "json.primaryResource.type",
                    "wiz.defend.primary_resource.type",
                )?;
            }

            if event.has_value("json.severity") {
                event.rename("json.severity", "wiz.defend.severity")?;
            }

            let _cond = {
                event
                    .get("wiz.defend.severity")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // Painless script
                // Source: ctx.event = ctx.event ?: [:];\nString severity = ctx.wiz.defend.severity;\nif (severity.equalsIgnoreCase(\"low\")) {\n  ctx.event.severity = 21;\n} else if (severity.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (severity.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (severity.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (severity.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.event = ctx.event ?: [:];\nString severity = ctx.wiz.defend.severity;\nif (severity.equalsIgnoreCase(\"low\")) {\n  ctx.event.severity = 21;\n} else if (severity.equalsIgnoreCase(\"informational\")) {\n  ctx.event.severity = 21;\n} else if (severity.equalsIgnoreCase(\"medium\")) {\n  ctx.event.severity = 47;\n} else if (severity.equalsIgnoreCase(\"high\")) {\n  ctx.event.severity = 73;\n} else if (severity.equalsIgnoreCase(\"critical\")) {\n  ctx.event.severity = 99;\n}"#
                    ),
                )?;
            }

            if event.has_value("json.source") {
                event.rename("json.source", "wiz.defend.source")?;
            }

            if event.has_value("json.tdrId") {
                event.rename("json.tdrId", "wiz.defend.tdr_id")?;
            }

            if event.has_value("json.tdrSource") {
                event.rename("json.tdrSource", "wiz.defend.tdr_source")?;
            }

            if event.has_value("json.threatId") {
                event.rename("json.threatId", "wiz.defend.threat_id")?;
            }

            let _cond = { event.has_value("wiz.defend.threat_id") };
            if _cond {
                event.append_unique(
                    "threat.indicator.id",
                    json!(
                        event
                            .get("wiz.defend.threat_id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.threatURL") {
                event.rename("json.threatURL", "wiz.defend.threat_url")?;
            }

            if let Some(v) = event
                .get("wiz.defend.threat_url")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.indicator.reference", v)?;
            }

            let _cond = {
                event.has_value("json.timeframe.end")
                    && event.get_str("json.timeframe.end") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timeframe.end") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("wiz.defend.timeframe.end", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timeframe.end".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_timeframe_end")?;
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
            }

            let _cond = {
                event.has_value("json.timeframe.start")
                    && event.get_str("json.timeframe.start") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.timeframe.start") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("wiz.defend.timeframe.start", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.timeframe.start".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_timeframe_start")?;
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
            }

            if event.has_value("json.title") {
                event.rename("json.title", "wiz.defend.title")?;
            }

            if event.has_value("json.trigger.ruleId") {
                event.rename("json.trigger.ruleId", "wiz.defend.trigger.rule_id")?;
            }

            if let Some(v) = event
                .get("wiz.defend.trigger.rule_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if event.has_value("json.trigger.ruleName") {
                event.rename("json.trigger.ruleName", "wiz.defend.trigger.rule_name")?;
            }

            if let Some(v) = event
                .get("wiz.defend.trigger.rule_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if let Some(v) = event
                .get("wiz.defend.trigger.rule_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("wiz.defend.friendly_name", v)?;
            }

            if event.has_value("json.trigger.source") {
                event.rename("json.trigger.source", "wiz.defend.trigger.source")?;
            }

            if event.has_value("json.trigger.type") {
                event.rename("json.trigger.type", "wiz.defend.trigger.type")?;
            }

            if let Some(v) = event
                .get("wiz.defend.trigger.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("event.action") {
                map_strings(event, "event.action", "event.action", str::to_lowercase)?;
            }

            let _cond = { event.get_str("event.action") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("event.action") {
                        if let Some(s) = event.get_string("event.action") {
                            let mut parts: Vec<Value> = cached_regex!("\\s+")
                                .split(&s)
                                .into_iter()
                                .map(|p| json!(p))
                                .collect();
                            while parts.last().and_then(Value::as_str) == Some("") {
                                parts.pop();
                            }
                            event.set("event.action", Value::Array(parts))?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "split")?;
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
            }

            let _cond = { event.get("event.action").is_some_and(|v| v.is_array()) };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    let joined = event.get("event.action").and_then(|v| join_values(v, "-"));
                    if let Some(joined) = joined {
                        event.set("event.action", json!(joined))?;
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "join")?;
                    event.set("_ingest.on_failure_processor_tag", "join_event_action")?;
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
            }

            if event.has_value("json.triggeringEvent.actor.actingAs.id") {
                event.rename(
                    "json.triggeringEvent.actor.actingAs.id",
                    "wiz.defend.triggering_event.actor.acting_as.id",
                )?;
            }

            if event.has_value("json.triggeringEvent.actor.actingAs.name") {
                event.rename(
                    "json.triggeringEvent.actor.actingAs.name",
                    "wiz.defend.triggering_event.actor.acting_as.name",
                )?;
            }

            if event.has_value("json.triggeringEvent.actor.actingAs.nativeType") {
                event.rename(
                    "json.triggeringEvent.actor.actingAs.nativeType",
                    "wiz.defend.triggering_event.actor.acting_as.native_type",
                )?;
            }

            if event.has_value("json.triggeringEvent.actor.actingAs.type") {
                event.rename(
                    "json.triggeringEvent.actor.actingAs.type",
                    "wiz.defend.triggering_event.actor.acting_as.type",
                )?;
            }

            if event.has_value("json.triggeringEvent.actor.externalId") {
                event.rename(
                    "json.triggeringEvent.actor.externalId",
                    "wiz.defend.triggering_event.actor.external_id",
                )?;
            }

            if event.has_value("json.triggeringEvent.actor.id") {
                event.rename(
                    "json.triggeringEvent.actor.id",
                    "wiz.defend.triggering_event.actor.id",
                )?;
            }

            let _cond = { event.has_value("wiz.defend.triggering_event.actor.id") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("wiz.defend.triggering_event.actor.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("wiz.defend.triggering_event.actor.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if event.has_value("json.triggeringEvent.actor.name") {
                event.rename(
                    "json.triggeringEvent.actor.name",
                    "wiz.defend.triggering_event.actor.name",
                )?;
            }

            let _cond = { event.has_value("wiz.defend.triggering_event.actor.name") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("wiz.defend.triggering_event.actor.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("wiz.defend.triggering_event.actor.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if event.has_value("json.triggeringEvent.actor.nativeType") {
                event.rename(
                    "json.triggeringEvent.actor.nativeType",
                    "wiz.defend.triggering_event.actor.native_type",
                )?;
            }

            if event.has_value("json.triggeringEvent.actor.providerUniqueId") {
                event.rename(
                    "json.triggeringEvent.actor.providerUniqueId",
                    "wiz.defend.triggering_event.actor.provider_unique_id",
                )?;
            }

            if event.has_value("json.triggeringEvent.actor.type") {
                event.rename(
                    "json.triggeringEvent.actor.type",
                    "wiz.defend.triggering_event.actor.type",
                )?;
            }

            if event.has_value("json.triggeringEvent.actorIP") {
                if let Some(ip_str) = event.get_string("json.triggeringEvent.actorIP") {
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

            let _cond = { event.has_value("json.triggeringEvent.actorIP") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("json.triggeringEvent.actorIP")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.get_str("json.triggeringEvent.actorIP") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.triggeringEvent.actorIP") {
                        if let Some(val) = event.get("json.triggeringEvent.actorIP") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.triggeringEvent.actorIP".into(),
                                    message,
                                }
                            })?;
                            event.set("wiz.defend.triggering_event.actor_ip", converted)?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_triggeringEvent_actorIP_to_ip",
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
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.triggeringEvent.actorIPMeta.autonomousSystemNumber") {
                    if let Some(val) =
                        event.get("json.triggeringEvent.actorIPMeta.autonomousSystemNumber")
                    {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.triggeringEvent.actorIPMeta.autonomousSystemNumber"
                                    .into(),
                                message,
                            }
                        })?;
                        event.set(
                            "wiz.defend.triggering_event.actor_ip_meta.autonomous_system_number",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_triggeringEvent_actorIPMeta_autonomousSystemNumber_to_long",
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

            if let Some(v) = event
                .get("wiz.defend.triggering_event.actor_ip_meta.autonomous_system_number")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.as.number", v)?;
            }

            if event.has_value("json.triggeringEvent.actorIPMeta.autonomousSystemOrganization") {
                event.rename(
                    "json.triggeringEvent.actorIPMeta.autonomousSystemOrganization",
                    "wiz.defend.triggering_event.actor_ip_meta.autonomous_system_organization",
                )?;
            }

            if let Some(v) = event
                .get("wiz.defend.triggering_event.actor_ip_meta.autonomous_system_organization")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.as.organization.name", v)?;
            }

            if event.has_value("json.triggeringEvent.actorIPMeta.country") {
                event.rename(
                    "json.triggeringEvent.actorIPMeta.country",
                    "wiz.defend.triggering_event.actor_ip_meta.country",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.triggeringEvent.actorIPMeta.isForeign") {
                    if let Some(val) = event.get("json.triggeringEvent.actorIPMeta.isForeign") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.triggeringEvent.actorIPMeta.isForeign".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "wiz.defend.triggering_event.actor_ip_meta.is_foreign",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_triggeringEvent_actorIPMeta_isForeign_to_boolean",
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

            if event.has_value("json.triggeringEvent.actorIPMeta.relatedAttackGroupNames") {
                event.rename(
                    "json.triggeringEvent.actorIPMeta.relatedAttackGroupNames",
                    "wiz.defend.triggering_event.actor_ip_meta.related_attack_group_names",
                )?;
            }

            if event.has_value("json.triggeringEvent.actorIPMeta.reputation") {
                event.rename(
                    "json.triggeringEvent.actorIPMeta.reputation",
                    "wiz.defend.triggering_event.actor_ip_meta.reputation",
                )?;
            }

            if event.has_value("json.triggeringEvent.actorIPMeta.reputationDescription") {
                event.rename(
                    "json.triggeringEvent.actorIPMeta.reputationDescription",
                    "wiz.defend.triggering_event.actor_ip_meta.reputation_description",
                )?;
            }

            if event.has_value("json.triggeringEvent.actorIPMeta.reputationSource") {
                event.rename(
                    "json.triggeringEvent.actorIPMeta.reputationSource",
                    "wiz.defend.triggering_event.actor_ip_meta.reputation_source",
                )?;
            }

            if event.has_value("json.triggeringEvent.category") {
                event.rename(
                    "json.triggeringEvent.category",
                    "wiz.defend.triggering_event.category",
                )?;
            }

            if event.has_value("json.triggeringEvent.cloudPlatform") {
                event.rename(
                    "json.triggeringEvent.cloudPlatform",
                    "wiz.defend.triggering_event.cloud_platform",
                )?;
            }

            if let Some(v) = event
                .get("wiz.defend.triggering_event.cloud_platform")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.provider", v)?;
            }

            if event.has_value("json.triggeringEvent.cloudProviderUrl") {
                event.rename(
                    "json.triggeringEvent.cloudProviderUrl",
                    "wiz.defend.triggering_event.cloud_provider_url",
                )?;
            }

            if event.has_value("json.triggeringEvent.description") {
                event.rename(
                    "json.triggeringEvent.description",
                    "wiz.defend.triggering_event.description",
                )?;
            }

            if let Some(v) = event
                .get("wiz.defend.triggering_event.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = {
                event.has_value("json.triggeringEvent.eventTime")
                    && event.get_str("json.triggeringEvent.eventTime") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.triggeringEvent.eventTime") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => {
                                event.set("wiz.defend.triggering_event.event_time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.triggeringEvent.eventTime".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "date_triggeringEvent_eventTime",
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
            }

            if let Some(v) = event
                .get("wiz.defend.triggering_event.event_time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            if event.has_value("json.triggeringEvent.externalId") {
                event.rename(
                    "json.triggeringEvent.externalId",
                    "wiz.defend.triggering_event.external_id",
                )?;
            }

            if event.has_value("json.triggeringEvent.id") {
                event.rename("json.triggeringEvent.id", "wiz.defend.triggering_event.id")?;
            }

            if let Some(v) = event
                .get("wiz.defend.triggering_event.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.triggeringEvent.name") {
                event.rename(
                    "json.triggeringEvent.name",
                    "wiz.defend.triggering_event.name",
                )?;
            }

            if event.has_value("json.triggeringEvent.origin") {
                event.rename(
                    "json.triggeringEvent.origin",
                    "wiz.defend.triggering_event.origin",
                )?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.resources")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.triggeringEvent.resources", |event| {
                    if event.has_value("_ingest._value.cloudAccount.cloudPlatform") {
                        event.rename(
                            "_ingest._value.cloudAccount.cloudPlatform",
                            "_ingest._value.cloud_account.cloud_platform",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.resources")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.triggeringEvent.resources", |event| {
                    if event.has_value("_ingest._value.cloudAccount.externalId") {
                        event.rename(
                            "_ingest._value.cloudAccount.externalId",
                            "_ingest._value.cloud_account.external_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.resources")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.triggeringEvent.resources", |event| {
                    if event.has_value("_ingest._value.cloudAccount.id") {
                        event.rename(
                            "_ingest._value.cloudAccount.id",
                            "_ingest._value.cloud_account.id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.resources")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.triggeringEvent.resources", |event| {
                    if event.has_value("_ingest._value.cloudProviderUrl") {
                        event.rename(
                            "_ingest._value.cloudProviderUrl",
                            "_ingest._value.cloud_provider_url",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.resources")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.triggeringEvent.resources", |event| {
                    if event.has_value("_ingest._value.externalId") {
                        event.rename("_ingest._value.externalId", "_ingest._value.external_id")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.resources")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.triggeringEvent.resources", |event| {
                    if event.has_value("_ingest._value.kubernetesClusterId") {
                        event.rename(
                            "_ingest._value.kubernetesClusterId",
                            "_ingest._value.kubernetes_cluster_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.resources")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.triggeringEvent.resources", |event| {
                    if event.has_value("_ingest._value.kubernetesClusterName") {
                        event.rename(
                            "_ingest._value.kubernetesClusterName",
                            "_ingest._value.kubernetes_cluster_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.resources")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.triggeringEvent.resources", |event| {
                    if event.has_value("_ingest._value.kubernetesNamespaceId") {
                        event.rename(
                            "_ingest._value.kubernetesNamespaceId",
                            "_ingest._value.kubernetes_namespace_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.resources")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.triggeringEvent.resources", |event| {
                    if event.has_value("_ingest._value.kubernetesNamespaceName") {
                        event.rename(
                            "_ingest._value.kubernetesNamespaceName",
                            "_ingest._value.kubernetes_namespace_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.resources")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.triggeringEvent.resources", |event| {
                    if event.has_value("_ingest._value.kubernetesNodeId") {
                        event.rename(
                            "_ingest._value.kubernetesNodeId",
                            "_ingest._value.kubernetes_node_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.resources")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.triggeringEvent.resources", |event| {
                    if event.has_value("_ingest._value.kubernetesNodeName") {
                        event.rename(
                            "_ingest._value.kubernetesNodeName",
                            "_ingest._value.kubernetes_node_name",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.resources")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.triggeringEvent.resources", |event| {
                    if event.has_value("_ingest._value.nativeType") {
                        event.rename("_ingest._value.nativeType", "_ingest._value.native_type")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.resources")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.triggeringEvent.resources", |event| {
                    if event.has_value("_ingest._value.providerUniqueId") {
                        event.rename(
                            "_ingest._value.providerUniqueId",
                            "_ingest._value.provider_unique_id",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.triggeringEvent.resources") {
                event.rename(
                    "json.triggeringEvent.resources",
                    "wiz.defend.triggering_event.resources",
                )?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.runtimeDetails.processTree")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.triggeringEvent.runtimeDetails.processTree",
                    |event| {
                        if event.has_value("_ingest._value.container.externalId") {
                            event.rename(
                                "_ingest._value.container.externalId",
                                "_ingest._value.container.external_id",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.runtimeDetails.processTree")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.triggeringEvent.runtimeDetails.processTree",
                    |event| {
                        if event.has_value("_ingest._value.container.imageExternalId") {
                            event.rename(
                                "_ingest._value.container.imageExternalId",
                                "_ingest._value.container.image_external_id",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.runtimeDetails.processTree")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.triggeringEvent.runtimeDetails.processTree",
                    |event| {
                        if event.has_value("_ingest._value.container.imageId") {
                            event.rename(
                                "_ingest._value.container.imageId",
                                "_ingest._value.container.image_id",
                            )?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.runtimeDetails.processTree")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                {
                    // A foreach walks a LIST or an OBJECT: over an object Elastic
                    // binds `_ingest._key` per entry, which is what a target of
                    // `<field>.{{{_ingest._key}}}` reads.
                    let subject = event
                        .get("json.triggeringEvent.runtimeDetails.processTree")
                        .cloned();
                    let keyed = matches!(subject, Some(Value::Object(_)));
                    let entries: Vec<(Option<String>, Value)> = match subject {
                        Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                        Some(Value::Object(fields)) => {
                            fields.into_iter().map(|(k, v)| (Some(k), v)).collect()
                        }
                        _ => Vec::new(),
                    };
                    if !entries.is_empty() {
                        // A NESTED loop borrows the same slots, so the enclosing
                        // entry is saved and put back afterwards.
                        let enclosing = event.get("_ingest._value").cloned();
                        let enclosing_key = event.get("_ingest._key").cloned();
                        let mut list = Vec::with_capacity(entries.len());
                        let mut fields = Map::new();
                        for (key, item) in entries {
                            if let Some(key) = key.as_deref() {
                                event.set("_ingest._key", Value::String(key.to_string()))?;
                            }
                            event.set("_ingest._value", item)?;
                            // on_failure: 1 handler(s)
                            if let Err(err) = (|| -> Result<()> {
                                if let Some(date_str) =
                                    event.get_as_string("_ingest._value.executionTime")
                                {
                                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                                        Some(parsed) => {
                                            event.set("_ingest._value.execution_time", parsed)?
                                        }
                                        None => {
                                            return Err(TransformError::ParseError {
                                                path: "_ingest._value.executionTime".into(),
                                                message: format!(
                                                    "unable to parse date [{date_str}]"
                                                ),
                                            });
                                        }
                                    }
                                }
                                Ok(())
                            })() {
                                event.set("_ingest.on_failure_message", err.to_string())?;
                                event.set("_ingest.on_failure_processor_type", "date")?;
                                event.set(
                                    "_ingest.on_failure_processor_tag",
                                    "date_triggeringEvent_runtimeDetails_processTree_executionTime",
                                )?;
                                event.remove("_ingest._value.executionTime");
                                event.remove("_ingest.on_failure_message");
                                event.remove("_ingest.on_failure_processor_type");
                                event.remove("_ingest.on_failure_processor_tag");
                                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                    event.remove("_ingest");
                                }
                            }
                            let left = event.remove("_ingest._value");
                            match key {
                                // An entry the body renamed AWAY is gone from the
                                // object, which is how a foreach lifts fields up.
                                Some(key) => {
                                    if let Some(value) = left {
                                        fields.insert(key, value);
                                    }
                                }
                                None => list.push(left.unwrap_or(Value::Null)),
                            }
                        }
                        match enclosing {
                            Some(previous) => {
                                event.set("_ingest._value", previous)?;
                            }
                            None => {
                                event.remove("_ingest");
                            }
                        }
                        if let Some(previous) = enclosing_key {
                            event.set("_ingest._key", previous)?;
                        }
                        event.set(
                            "json.triggeringEvent.runtimeDetails.processTree",
                            if keyed {
                                Value::Object(fields)
                            } else {
                                Value::Array(list)
                            },
                        )?;
                    }
                }
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.runtimeDetails.processTree")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.triggeringEvent.runtimeDetails.processTree",
                    |event| {
                        if event.has_value("_ingest._value.userId") {
                            event.rename("_ingest._value.userId", "_ingest._value.user_id")?;
                        }
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.runtimeDetails.processTree")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.triggeringEvent.runtimeDetails.processTree",
                    |event| {
                        event.remove("_ingest._value.executionTime");
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.runtimeDetails.processTree")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.triggeringEvent.runtimeDetails.processTree",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.user_id")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.runtimeDetails.processTree")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.triggeringEvent.runtimeDetails.processTree",
                    |event| {
                        event.append_unique(
                            "related.user",
                            json!(
                                event
                                    .get("_ingest._value.username")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            let _cond = {
                event
                    .get("json.triggeringEvent.runtimeDetails.processTree")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "json.triggeringEvent.runtimeDetails.processTree",
                    |event| {
                        event.append_unique(
                            "related.hash",
                            json!(
                                event
                                    .get("_ingest._value.hash")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            if event.has_value("json.triggeringEvent.runtimeDetails.processTree") {
                event.rename(
                    "json.triggeringEvent.runtimeDetails.processTree",
                    "wiz.defend.triggering_event.runtime_details.process_tree",
                )?;
            }

            if event.has_value("json.triggeringEvent.source") {
                event.rename(
                    "json.triggeringEvent.source",
                    "wiz.defend.triggering_event.source",
                )?;
            }

            if let Some(v) = event
                .get("wiz.defend.triggering_event.source")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.provider", v)?;
            }

            if event.has_value("json.triggeringEvent.status") {
                event.rename(
                    "json.triggeringEvent.status",
                    "wiz.defend.triggering_event.status",
                )?;
            }

            let _cond = {
                event
                    .get("wiz.defend.triggering_event.status")
                    .is_some_and(|v| v.is_string())
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: String status = ctx.wiz.defend.triggering_event.status.toLowerCase();\nif (status.contains('success')) {\n  ctx.event.outcome = \"success\";\n} else if (status.contains('fail')) {\n  ctx.event.outcome = \"failure\";\n} else {\n  ctx.event.outcome = \"unknown\";\n}
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan(
                        event,
                        cached_painless!(
                            r#"String status = ctx.wiz.defend.triggering_event.status.toLowerCase();\nif (status.contains('success')) {\n  ctx.event.outcome = \"success\";\n} else if (status.contains('fail')) {\n  ctx.event.outcome = \"failure\";\n} else {\n  ctx.event.outcome = \"unknown\";\n}"#
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set("_ingest.on_failure_processor_tag", "set_event_outcome")?;
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
            }

            if event.has_value("json.triggeringEvent.subjectResourceId") {
                event.rename(
                    "json.triggeringEvent.subjectResourceId",
                    "wiz.defend.triggering_event.subject_resource_id",
                )?;
            }

            let _cond = { event.get_str("json.triggeringEvent.subjectResourceIp") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if event.has_value("json.triggeringEvent.subjectResourceIp") {
                        if let Some(val) = event.get("json.triggeringEvent.subjectResourceIp") {
                            let converted = convert_value(val, "ip").map_err(|message| {
                                TransformError::ParseError {
                                    path: "json.triggeringEvent.subjectResourceIp".into(),
                                    message,
                                }
                            })?;
                            event.set(
                                "wiz.defend.triggering_event.subject_resource_ip",
                                converted,
                            )?;
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "convert_triggeringEvent_subjectResourceIp_to_ip",
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
            }

            let _cond = { event.has_value("wiz.defend.triggering_event.subject_resource_ip") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.ip",
                        json!(
                            event
                                .get("wiz.defend.triggering_event.subject_resource_ip")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            if let Some(v) = event
                .get("wiz.defend.triggering_event.subject_resource_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            if event.has_value("json.triggeringEventsCount") {
                event.rename(
                    "json.triggeringEventsCount",
                    "wiz.defend.triggering_events_count",
                )?;
            }

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")),
                        serde_json::Value::String(s) => {
                            s.contains("preserve_duplicate_custom_fields")
                        }
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("wiz.defend.detection_url");
                event.remove("wiz.defend.threat_id");
                event.remove("wiz.defend.threat_url");
                event.remove("wiz.defend.trigger.rule_id");
                event.remove("wiz.defend.trigger.rule_name");
                event.remove("wiz.defend.trigger.type");
                event.remove("wiz.defend.triggering_event.actor.id");
                event.remove("wiz.defend.triggering_event.actor.name");
                event.remove("wiz.defend.triggering_event.actor_ip_meta.autonomous_system_number");
                event.remove(
                    "wiz.defend.triggering_event.actor_ip_meta.autonomous_system_organization",
                );
                event.remove("wiz.defend.triggering_event.cloud_platform");
                event.remove("wiz.defend.triggering_event.description");
                event.remove("wiz.defend.triggering_event.event_time");
                event.remove("wiz.defend.triggering_event.id");
                event.remove("wiz.defend.triggering_event.source");
                event.remove("wiz.defend.triggering_event.subject_resource_ip");
                event.remove("wiz.defend.mitreTactics");
                event.remove("wiz.defend.mitreTactics");
            }

            event.remove("json");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            let _cond = { event.has_value("error.message") };
            if _cond {
                event.set("event.kind", json!("pipeline_error"))?;
            }

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
