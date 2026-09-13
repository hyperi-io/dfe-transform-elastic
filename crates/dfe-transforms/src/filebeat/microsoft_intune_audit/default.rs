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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_event_original")?;
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

            event.set("event.kind", json!("event"))?;

            event.append("event.type", json!("change"))?;

            event.append("event.category", json!("configuration"))?;

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.properties.AuditEventId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.tenantId") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.time") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            if event.has_value("json.category") {
                event.rename("json.category", "microsoft_intune.audit.category")?;
            }

            if event.has_value("json.correlationId") {
                event.rename(
                    "json.correlationId",
                    "microsoft_intune.audit.correlation_id",
                )?;
            }

            if event.has_value("json.identity") {
                event.rename("json.identity", "microsoft_intune.audit.identity")?;
            }

            if let Some(v) = event
                .get("microsoft_intune.audit.identity")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.email", v)?;
            }

            let _cond = { event.has_value("microsoft_intune.audit.identity") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("microsoft_intune.audit.identity")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.operationName") {
                event.rename(
                    "json.operationName",
                    "microsoft_intune.audit.operation_name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.audit.operation_name")
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

            let _cond = { event.get_str("event.action") != Some("") };
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

            let _cond = {
                event.has_value("json.properties.ActivityDate")
                    && event.get_str("json.properties.ActivityDate") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.properties.ActivityDate") {
                        match parse_date_out(&date_str, &["M/d/yyyy h:mm:ss a"], None, None) {
                            Some(parsed) => event
                                .set("microsoft_intune.audit.properties.activity_date", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.properties.ActivityDate".into(),
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
                        "date_properties_ActivityDate",
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

            if event.has_value("json.properties.ActivityResultStatus") {
                if let Some(val) = event.get("json.properties.ActivityResultStatus") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.properties.ActivityResultStatus".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "microsoft_intune.audit.properties.activity_result_status",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.properties.ActivityType") {
                if let Some(val) = event.get("json.properties.ActivityType") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.properties.ActivityType".into(),
                            message,
                        }
                    })?;
                    event.set("microsoft_intune.audit.properties.activity_type", converted)?;
                }
            }

            if event.has_value("json.properties.Actor.ActorType") {
                if let Some(val) = event.get("json.properties.Actor.ActorType") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.properties.Actor.ActorType".into(),
                            message,
                        }
                    })?;
                    event.set(
                        "microsoft_intune.audit.properties.actor.actor_type",
                        converted,
                    )?;
                }
            }

            if event.has_value("json.properties.Actor.Application") {
                event.rename(
                    "json.properties.Actor.Application",
                    "microsoft_intune.audit.properties.actor.application",
                )?;
            }

            if event.has_value("json.properties.Actor.ApplicationName") {
                event.rename(
                    "json.properties.Actor.ApplicationName",
                    "microsoft_intune.audit.properties.actor.application_name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.audit.properties.actor.application_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("service.name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.properties.Actor.IsDelegatedAdmin") {
                    if let Some(val) = event.get("json.properties.Actor.IsDelegatedAdmin") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.properties.Actor.IsDelegatedAdmin".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "microsoft_intune.audit.properties.actor.is_delegated_admin",
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
                    "convert_properties_Actor_IsDelegatedAdmin_to_boolean",
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

            if event.has_value("json.properties.Actor.Name") {
                event.rename(
                    "json.properties.Actor.Name",
                    "microsoft_intune.audit.properties.actor.name",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.audit.properties.actor.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if event.has_value("json.properties.Actor.ObjectId") {
                event.rename(
                    "json.properties.Actor.ObjectId",
                    "microsoft_intune.audit.properties.actor.object_id",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.audit.properties.actor.object_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if event.has_value("json.properties.Actor.PartnerTenantId") {
                event.rename(
                    "json.properties.Actor.PartnerTenantId",
                    "microsoft_intune.audit.properties.actor.partner_tenant_id",
                )?;
            }

            if event.has_value("json.properties.Actor.UPN") {
                event.rename(
                    "json.properties.Actor.UPN",
                    "microsoft_intune.audit.properties.actor.upn",
                )?;
            }

            let _cond = { event.has_value("microsoft_intune.audit.properties.actor.upn") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("microsoft_intune.audit.properties.actor.upn")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.properties.Actor.UserPermissions") {
                event.rename(
                    "json.properties.Actor.UserPermissions",
                    "microsoft_intune.audit.properties.actor.user_permissions",
                )?;
            }

            if event.has_value("json.properties.AdditionalDetails") {
                event.rename(
                    "json.properties.AdditionalDetails",
                    "microsoft_intune.audit.properties.additional_detail",
                )?;
            }

            let _cond = { event.has_value("microsoft_intune.audit.properties.additional_detail") };
            if _cond {
                // Painless script
                // Source: def content = ctx.microsoft_intune.audit.properties.additional_detail;\ndef parsed = new HashMap();\ndef lines = /\\r\\n/.split(content);\nfor (int i = 0; i < lines.length; i++) {\n  def line = lines[i];\n  if (line.contains('Key = ') && line.contains('Value = ')) {\n    def parts = /Value = /.split(line, 2);\n    if (parts.length == 2) {\n      def key = parts[0].replace('Key = ', '').trim();\n      def value = parts[1].trim();\n      parsed.put(key, value);\n    }\n  }\n}\nif (ctx.microsoft_intune.audit.properties.additional_detail instanceof String) {\n  def temp = new HashMap();\n  temp.put('original', ctx.microsoft_intune.audit.properties.additional_detail);\n  temp.put('parsed', parsed);\n  ctx.microsoft_intune.audit.properties.additional_detail = temp;\n} else {\n  ctx.microsoft_intune.audit.properties.additional_detail.parsed = parsed;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def content = ctx.microsoft_intune.audit.properties.additional_detail;\ndef parsed = new HashMap();\ndef lines = /\\r\\n/.split(content);\nfor (int i = 0; i < lines.length; i++) {\n  def line = lines[i];\n  if (line.contains('Key = ') && line.contains('Value = ')) {\n    def parts = /Value = /.split(line, 2);\n    if (parts.length == 2) {\n      def key = parts[0].replace('Key = ', '').trim();\n      def value = parts[1].trim();\n      parsed.put(key, value);\n    }\n  }\n}\nif (ctx.microsoft_intune.audit.properties.additional_detail instanceof String) {\n  def temp = new HashMap();\n  temp.put('original', ctx.microsoft_intune.audit.properties.additional_detail);\n  temp.put('parsed', parsed);\n  ctx.microsoft_intune.audit.properties.additional_detail = temp;\n} else {\n  ctx.microsoft_intune.audit.properties.additional_detail.parsed = parsed;\n}\n"#
                    ),
                )?;
            }

            if event.has_value("json.properties.AuditEventId") {
                event.rename(
                    "json.properties.AuditEventId",
                    "microsoft_intune.audit.properties.audit_event_id",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.audit.properties.audit_event_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.properties.Category") {
                if let Some(val) = event.get("json.properties.Category") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "json.properties.Category".into(),
                            message,
                        }
                    })?;
                    event.set("microsoft_intune.audit.properties.category", converted)?;
                }
            }

            if event.has_value("json.properties.RelationId") {
                event.rename(
                    "json.properties.RelationId",
                    "microsoft_intune.audit.properties.relation_id",
                )?;
            }

            if event.has_value("json.properties.TargetDisplayNames") {
                event.rename(
                    "json.properties.TargetDisplayNames",
                    "microsoft_intune.audit.properties.target_display_names",
                )?;
            }

            if event.has_value("json.properties.TargetObjectIds") {
                event.rename(
                    "json.properties.TargetObjectIds",
                    "microsoft_intune.audit.properties.target_object_ids",
                )?;
            }

            let _cond = {
                event
                    .get("json.properties.Targets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.properties.Targets", |event| {
                    event.append_unique(
                        "destination.domain",
                        json!(
                            event
                                .get("_ingest._value.Name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.properties.Targets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.properties.Targets", |event| {
                    if event.has_value("_ingest._value.Name") {
                        event.rename("_ingest._value.Name", "_ingest._value.name")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.properties.Targets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.properties.Targets", |event| {
                    if event.has_value("_ingest._value.ModifiedProperties") {
                        foreach_array(event, "_ingest._value.ModifiedProperties", |event| {
                            if event.has_value("_ingest._value.Name") {
                                event.rename("_ingest._value.Name", "_ingest._value.name")?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.properties.Targets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.properties.Targets", |event| {
                    if event.has_value("_ingest._value.ModifiedProperties") {
                        foreach_array(event, "_ingest._value.ModifiedProperties", |event| {
                            if event.has_value("_ingest._value.Old") {
                                event.rename("_ingest._value.Old", "_ingest._value.old")?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.properties.Targets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.properties.Targets", |event| {
                    if event.has_value("_ingest._value.ModifiedProperties") {
                        foreach_array(event, "_ingest._value.ModifiedProperties", |event| {
                            if event.has_value("_ingest._value.New") {
                                event.rename("_ingest._value.New", "_ingest._value.new")?;
                            }
                            Ok(())
                        })?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.properties.Targets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.properties.Targets", |event| {
                    if event.has_value("_ingest._value.ModifiedProperties") {
                        event.rename(
                            "_ingest._value.ModifiedProperties",
                            "_ingest._value.modified_properties",
                        )?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.properties.Targets") {
                event.rename(
                    "json.properties.Targets",
                    "microsoft_intune.audit.properties.targets",
                )?;
            }

            if event.has_value("json.properties.additionalDetails") {
                event.rename(
                    "json.properties.additionalDetails",
                    "microsoft_intune.audit.properties.additional_details",
                )?;
            }

            if event.has_value("json.properties.loggedByService") {
                event.rename(
                    "json.properties.loggedByService",
                    "microsoft_intune.audit.properties.logged_by_service",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.audit.properties.logged_by_service")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.provider", v)?;
            }

            if event.has_value("json.resultDescription") {
                event.rename(
                    "json.resultDescription",
                    "microsoft_intune.audit.result_description",
                )?;
            }

            if let Some(v) = event
                .get("microsoft_intune.audit.result_description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.reason", v)?;
            }

            if event.has_value("json.resultType") {
                event.rename("json.resultType", "microsoft_intune.audit.result_type")?;
            }

            if let Some(v) = event
                .get("microsoft_intune.audit.result_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.outcome", v)?;
            }

            if event.has_value("event.outcome") {
                map_strings(event, "event.outcome", "event.outcome", str::to_lowercase)?;
            }

            if event.has_value("json.tenantId") {
                event.rename("json.tenantId", "microsoft_intune.audit.tenant_id")?;
            }

            if let Some(v) = event
                .get("microsoft_intune.audit.tenant_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            let _cond = { event.has_value("json.time") && event.get_str("json.time") != Some("") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.time") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("microsoft_intune.audit.time", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.time".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_time")?;
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
                .get("microsoft_intune.audit.time")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("@timestamp", v)?;
            }

            let _cond = {
                event
                    .get("microsoft_intune.audit.properties.targets")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "microsoft_intune.audit.properties.targets",
                    |event| {
                        let _cond = {
                            !event.has_value("tags")
                                || !(event.get("tags").is_some_and(|v| match v {
                                    serde_json::Value::Array(a) => a.iter().any(|x| {
                                        x.as_str() == Some("preserve_duplicate_custom_fields")
                                    }),
                                    serde_json::Value::String(s) => {
                                        s.contains("preserve_duplicate_custom_fields")
                                    }
                                    _ => false,
                                }))
                        };
                        if _cond {
                            event.remove("_ingest._value.name");
                        }
                        Ok(())
                    },
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
                event.remove("microsoft_intune.audit.identity");
                event.remove("microsoft_intune.audit.operation_name");
                event.remove("microsoft_intune.audit.properties.actor.application_name");
                event.remove("microsoft_intune.audit.properties.actor.name");
                event.remove("microsoft_intune.audit.properties.actor.object_id");
                event.remove("microsoft_intune.audit.properties.audit_event_id");
                event.remove("microsoft_intune.audit.properties.logged_by_service");
                event.remove("microsoft_intune.audit.result_description");
                event.remove("microsoft_intune.audit.result_type");
                event.remove("microsoft_intune.audit.tenant_id");
                event.remove("microsoft_intune.audit.time");
            }

            event.remove("json");

            // Painless script, resolved to its runners at generation time
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n      handleMap(v);\n    } else if (v instanceof List) {\n      handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);
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
                        "Processor '{}'\n{}failed with message '{}'",
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
                                "with tag '{}'\n",
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
