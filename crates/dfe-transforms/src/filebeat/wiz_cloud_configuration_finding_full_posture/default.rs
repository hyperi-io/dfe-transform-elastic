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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_decoding")?;
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

            event.set("observer.vendor", json!("Wiz"))?;

            if event.has_value("json.resource.subscription.cloudProvider") {
                event.rename("json.resource.subscription.cloudProvider", "wiz.cloud_configuration_finding_full_posture.resource.subscription.cloud_provider")?;
            }

            if event.has_value(
                "wiz.cloud_configuration_finding_full_posture.resource.subscription.cloud_provider",
            ) {
                map_strings(
                    event,
                    "wiz.cloud_configuration_finding_full_posture.resource.subscription.cloud_provider",
                    "cloud.provider",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("json.resource.subscription.externalId") {
                event.rename("json.resource.subscription.externalId", "wiz.cloud_configuration_finding_full_posture.resource.subscription.external_id")?;
            }

            if let Some(v) = event.get("wiz.cloud_configuration_finding_full_posture.resource.subscription.external_id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("cloud.account.id", v)?;
            }

            if event.has_value("json.resource.subscription.name") {
                event.rename(
                    "json.resource.subscription.name",
                    "wiz.cloud_configuration_finding_full_posture.resource.subscription.name",
                )?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.resource.subscription.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.name", v)?;
            }

            if event.has_value("json.resource.region") {
                event.rename(
                    "json.resource.region",
                    "wiz.cloud_configuration_finding_full_posture.resource.region",
                )?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.resource.region")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.region", v)?;
            }

            if event.has_value("json.resource.cloudPlatform") {
                event.rename(
                    "json.resource.cloudPlatform",
                    "wiz.cloud_configuration_finding_full_posture.resource.cloud_platform",
                )?;
            }

            if event
                .has_value("wiz.cloud_configuration_finding_full_posture.resource.cloud_platform")
            {
                map_strings(
                    event,
                    "wiz.cloud_configuration_finding_full_posture.resource.cloud_platform",
                    "cloud.service.name",
                    str::to_lowercase,
                )?;
            }

            event.append("event.category", json!("configuration"))?;

            event.append("event.type", json!("info"))?;

            let _cond = {
                event.has_value("json.analyzedAt") && event.get_str("json.analyzedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.analyzedAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "wiz.cloud_configuration_finding_full_posture.analyzed_at",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.analyzedAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_set_analyzedat")?;
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
                .get("wiz.cloud_configuration_finding_full_posture.analyzed_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            let _cond = {
                event.has_value("json.updatedAt") && event.get_str("json.updatedAt") != Some("")
            };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.updatedAt") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set(
                                "wiz.cloud_configuration_finding_full_posture.updated_at",
                                parsed,
                            )?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.updatedAt".into(),
                                    message: format!("unable to parse date [{date_str}]"),
                                });
                            }
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "date")?;
                    event.set("_ingest.on_failure_processor_tag", "date_set_updatedat")?;
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

            if event.has_value("json.id") {
                event.rename("json.id", "wiz.cloud_configuration_finding_full_posture.id")?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            event.set("event.kind", json!("state"))?;

            if event.has_value("json.name") {
                event.rename(
                    "json.name",
                    "wiz.cloud_configuration_finding_full_posture.name",
                )?;
            }

            if event.has_value("json.rule.description") {
                event.rename(
                    "json.rule.description",
                    "wiz.cloud_configuration_finding_full_posture.rule.description",
                )?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.rule.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.rule.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.description", v)?;
            }

            event.set(
                "@timestamp",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            if event.has_value("json.rule.id") {
                event.rename(
                    "json.rule.id",
                    "wiz.cloud_configuration_finding_full_posture.rule.id",
                )?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.rule.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.uuid", v)?;
            }

            if event.has_value("json.rule.shortId") {
                event.rename(
                    "json.rule.shortId",
                    "wiz.cloud_configuration_finding_full_posture.rule.short_id",
                )?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.rule.short_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if event.has_value("json.rule.name") {
                event.rename(
                    "json.rule.name",
                    "wiz.cloud_configuration_finding_full_posture.rule.name",
                )?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.rule.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if event.has_value("json.rule.remediationInstructions") {
                event.rename(
                    "json.rule.remediationInstructions",
                    "wiz.cloud_configuration_finding_full_posture.rule.remediation_instructions",
                )?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.rule.remediation_instructions")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.remediation", v)?;
            }

            if event.has_value("json.resource.id") {
                event.rename(
                    "json.resource.id",
                    "wiz.cloud_configuration_finding_full_posture.resource.id",
                )?;
            }

            if event.has_value("json.resource.providerId") {
                event.rename(
                    "json.resource.providerId",
                    "wiz.cloud_configuration_finding_full_posture.resource.provider_id",
                )?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.resource.provider_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("resource.id", v)?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.resource.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("resource.id") {
                    event.set("resource.id", v)?;
                }
            }

            if event.has_value("json.resource.name") {
                event.rename(
                    "json.resource.name",
                    "wiz.cloud_configuration_finding_full_posture.resource.name",
                )?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.resource.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("resource.name", v)?;
            }

            if event.has_value("json.resource.type") {
                event.rename(
                    "json.resource.type",
                    "wiz.cloud_configuration_finding_full_posture.resource.type",
                )?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.resource.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("resource.type", v)?;
            }

            if event.has_value("json.resource.nativeType") {
                event.rename(
                    "json.resource.nativeType",
                    "wiz.cloud_configuration_finding_full_posture.resource.native_type",
                )?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.resource.native_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("resource.sub_type", v)?;
            }

            let _cond = { event.get_str("resource.type") == Some("USER_ACCOUNT") };
            if _cond {
                if let Some(v) = event
                    .get("wiz.cloud_configuration_finding_full_posture.resource.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            let _cond = { event.get_str("resource.type") == Some("USER_ACCOUNT") };
            if _cond {
                if let Some(v) = event
                    .get("wiz.cloud_configuration_finding_full_posture.resource.provider_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
            }

            let _cond = { event.get_str("resource.type") == Some("VIRTUAL_MACHINE") };
            if _cond {
                if event.has_value("wiz.cloud_configuration_finding_full_posture.resource.name") {
                    map_strings(
                        event,
                        "wiz.cloud_configuration_finding_full_posture.resource.name",
                        "host.name",
                        str::to_lowercase,
                    )?;
                }
            }

            if event.has_value("json.result") {
                event.rename(
                    "json.result",
                    "wiz.cloud_configuration_finding_full_posture.result",
                )?;
            }

            let _cond = {
                event.get_str("wiz.cloud_configuration_finding_full_posture.result") == Some("PASS")
            };
            if _cond {
                let v = json!("passed");
                if !painless_is_empty_value(&v) {
                    event.set("result.evaluation", v)?;
                }
            }

            let _cond = {
                event.get_str("wiz.cloud_configuration_finding_full_posture.result") == Some("FAIL")
            };
            if _cond {
                let v = json!("failed");
                if !painless_is_empty_value(&v) {
                    event.set("result.evaluation", v)?;
                }
            }

            let _cond = {
                event.get_str("wiz.cloud_configuration_finding_full_posture.result") != Some("PASS")
                    && event.get_str("wiz.cloud_configuration_finding_full_posture.result")
                        != Some("FAIL")
            };
            if _cond {
                let v = json!("unknown");
                if !painless_is_empty_value(&v) {
                    event.set("result.evaluation", v)?;
                }
            }

            let _cond = {
                event.get_str("wiz.cloud_configuration_finding_full_posture.result") == Some("PASS")
            };
            if _cond {
                let v = json!("success");
                if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
                }
            }

            let _cond = {
                event.get_str("wiz.cloud_configuration_finding_full_posture.result") == Some("FAIL")
            };
            if _cond {
                let v = json!("failure");
                if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
                }
            }

            let _cond = {
                event.get_str("wiz.cloud_configuration_finding_full_posture.result") != Some("PASS")
                    && event.get_str("wiz.cloud_configuration_finding_full_posture.result")
                        != Some("FAIL")
            };
            if _cond {
                let v = json!("unknown");
                if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
                }
            }

            if event.has_value("json.evidence.currentValue") {
                event.rename(
                    "json.evidence.currentValue",
                    "wiz.cloud_configuration_finding_full_posture.evidence.current_value",
                )?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.evidence.current_value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("result.evidence.current_value", v)?;
            }

            if event.has_value("json.evidence.expectedValue") {
                event.rename(
                    "json.evidence.expectedValue",
                    "wiz.cloud_configuration_finding_full_posture.evidence.expected_value",
                )?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.evidence.expected_value")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("result.evidence.expected_value", v)?;
            }

            if event.has_value("json.evidence.configurationPath") {
                event.rename(
                    "json.evidence.configurationPath",
                    "wiz.cloud_configuration_finding_full_posture.evidence.configuration_path",
                )?;
            }

            if let Some(v) = event
                .get("wiz.cloud_configuration_finding_full_posture.evidence.configuration_path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("result.evidence.configuration_path", v)?;
            }

            if event.has_value("json.evidence.cloudConfigurationLink") {
                event.rename("json.evidence.cloudConfigurationLink", "wiz.cloud_configuration_finding_full_posture.evidence.cloud_configuration_link")?;
            }

            if let Some(v) = event.get("wiz.cloud_configuration_finding_full_posture.evidence.cloud_configuration_link").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("result.evidence.cloud_configuration_link", v)?;
            }

            if let Some(v) = event.get("wiz.cloud_configuration_finding_full_posture.evidence.cloud_configuration_link").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("rule.reference", v)?;
            }

            if event.has_value("json.status") {
                event.rename(
                    "json.status",
                    "wiz.cloud_configuration_finding_full_posture.status",
                )?;
            }

            event.remove("json");

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
                event.remove("wiz.cloud_configuration_finding_full_posture.analyzed_at");
                event.remove("wiz.cloud_configuration_finding_full_posture.resource.subscription.cloud_provider");
                event.remove("wiz.cloud_configuration_finding_full_posture.resource.subscription.external_id");
                event.remove(
                    "wiz.cloud_configuration_finding_full_posture.resource.subscription.name",
                );
                event.remove("wiz.cloud_configuration_finding_full_posture.resource.region");
                event.remove("wiz.cloud_configuration_finding_full_posture.resource.name");
                event.remove("wiz.cloud_configuration_finding_full_posture.resource.type");
                event.remove("wiz.cloud_configuration_finding_full_posture.resource.sub_type");
                event.remove("wiz.cloud_configuration_finding_full_posture.resource.provider_id");
                event.remove("wiz.cloud_configuration_finding_full_posture.id");
                event.remove("wiz.cloud_configuration_finding_full_posture.name");
                event.remove("wiz.cloud_configuration_finding_full_posture.rule.description");
                event.remove("wiz.cloud_configuration_finding_full_posture.rule.name");
                event.remove("wiz.cloud_configuration_finding_full_posture.rule.id");
                event.remove("wiz.cloud_configuration_finding_full_posture.rule.short_id");
                event.remove(
                    "wiz.cloud_configuration_finding_full_posture.rule.remediation_instructions",
                );
                event
                    .remove("wiz.cloud_configuration_finding_full_posture.evidence.expected_value");
                event.remove("wiz.cloud_configuration_finding_full_posture.evidence.current_value");
                event.remove(
                    "wiz.cloud_configuration_finding_full_posture.evidence.configuration_path",
                );
                event.remove("wiz.cloud_configuration_finding_full_posture.evidence.cloud_configuration_link");
                event.remove("wiz.cloud_configuration_finding_full_posture.status");
            }

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);\n"#
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
                event.set(
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
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
