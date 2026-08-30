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
                event.set("_ingest.on_failure_processor_tag", "json_message")?;
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
                            .get("_ingest.pipeline")
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

            event.append("event.category", json!("configuration"))?;

            event.append("event.kind", json!("state"))?;

            event.append("event.type", json!("info"))?;

            event.set("observer.vendor", json!("Palo Alto Networks"))?;

            event.set("observer.product", json!("Prisma Cloud"))?;

            event.remove("cloud");

            if event.has_value("json.accountId") {
                event.rename("json.accountId", "prisma_cloud.misconfiguration.account_id")?;
            }

            if let Some(v) = event
                .get("prisma_cloud.misconfiguration.account_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.id", v)?;
            }

            if event.has_value("json.accountName") {
                event.rename(
                    "json.accountName",
                    "prisma_cloud.misconfiguration.account_name",
                )?;
            }

            if let Some(v) = event
                .get("prisma_cloud.misconfiguration.account_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.account.name", v)?;
            }

            if event.has_value("json.alertStatus") {
                event.rename(
                    "json.alertStatus",
                    "prisma_cloud.misconfiguration.alert_status",
                )?;
            }

            if event.has_value("prisma_cloud.misconfiguration.alert_status") {
                foreach_array(
                    event,
                    "prisma_cloud.misconfiguration.alert_status",
                    |event| {
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                            if let Some(val) = event.get("_ingest._value") {
                                let converted = convert_value(val, "long").map_err(|message| {
                                    TransformError::ParseError {
                                        path: "_ingest._value".into(),
                                        message,
                                    }
                                })?;
                                event.set("_ingest._value", converted)?;
                            }
                            Ok(())
                        })() {
                            event.set("_ingest.on_failure_message", err.to_string())?;
                            event.set("_ingest.on_failure_processor_type", "convert")?;
                            event.set(
                                "_ingest.on_failure_processor_tag",
                                "convert_alert_status_value_to_long",
                            )?;
                            event.remove("_ingest._key");
                            event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                            event.remove("_ingest.on_failure_message");
                            event.remove("_ingest.on_failure_processor_type");
                            event.remove("_ingest.on_failure_processor_tag");
                            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                                event.remove("_ingest");
                            }
                        }
                        Ok(())
                    },
                )?;
            }

            if event.has_value("json.appNames") {
                event.rename("json.appNames", "prisma_cloud.misconfiguration.app_names")?;
            }

            if event.has_value("json.assetType") {
                event.rename("json.assetType", "prisma_cloud.misconfiguration.asset_type")?;
            }

            if event.has_value("json.cloudType") {
                event.rename("json.cloudType", "prisma_cloud.misconfiguration.cloud_type")?;
            }

            if let Some(v) = event
                .get("prisma_cloud.misconfiguration.cloud_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.provider", v)?;
            }

            if event.has_value("cloud.provider") {
                map_strings(event, "cloud.provider", "cloud.provider", str::to_lowercase)?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "prisma_cloud.misconfiguration.id")?;
            }

            if event.has_value("json.name") {
                event.rename("json.name", "prisma_cloud.misconfiguration.name")?;
            }

            let _cond = {
                event.get_str("prisma_cloud.misconfiguration.asset_type")
                    == Some("AWS IAM Credential Report")
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_cloud.misconfiguration.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.name", v)?;
                }
            }

            let _cond = { event.has_value("user.name") && event.get_str("user.name") != Some("") };
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

            let _cond = {
                event.get_str("prisma_cloud.misconfiguration.asset_type") == Some("EC2 Instance")
                    || event.get_str("prisma_cloud.misconfiguration.asset_type")
                        == Some("Azure Virtual Machine")
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_cloud.misconfiguration.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.name", v)?;
                }
            }

            let _cond = { event.has_value("host.name") && event.get_str("host.name") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("prisma_cloud.misconfiguration.asset_type") == Some("EC2 Instance")
                    || event.get_str("prisma_cloud.misconfiguration.asset_type")
                        == Some("Azure Virtual Machine")
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_cloud.misconfiguration.id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.id", v)?;
                }
            }

            let _cond = {
                event.get_str("prisma_cloud.misconfiguration.asset_type") == Some("EC2 Instance")
                    || event.get_str("prisma_cloud.misconfiguration.asset_type")
                        == Some("Azure Virtual Machine")
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_cloud.misconfiguration.name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("cloud.instance.name", v)?;
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.overallPassed") {
                    if let Some(val) = event.get("json.overallPassed") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.overallPassed".into(),
                                message,
                            }
                        })?;
                        event.set("prisma_cloud.misconfiguration.overall_passed", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "convert_prisma_cloud_misconfiguration_overall_passed_to_boolean",
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
                            .get("_ingest.pipeline")
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

            if event.has_value("json.regionId") {
                event.rename("json.regionId", "prisma_cloud.misconfiguration.region_id")?;
            }

            if let Some(v) = event
                .get("prisma_cloud.misconfiguration.region_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.region", v)?;
            }

            if event.has_value("json.regionName") {
                event.rename(
                    "json.regionName",
                    "prisma_cloud.misconfiguration.region_name",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.resourceConfigJsonAvailable") {
                    if let Some(val) = event.get("json.resourceConfigJsonAvailable") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.resourceConfigJsonAvailable".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_cloud.misconfiguration.resource_config_json_available",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_prisma_cloud_misconfiguration_resource_config_json_available_to_boolean")?;
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
                            .get("_ingest.pipeline")
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.resourceDetailsAvailable") {
                    if let Some(val) = event.get("json.resourceDetailsAvailable") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.resourceDetailsAvailable".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_cloud.misconfiguration.resource_details_available",
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
                    "convert_prisma_cloud_misconfiguration_resource_details_available_to_boolean",
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
                            .get("_ingest.pipeline")
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

            if event.has_value("json.rrn") {
                event.rename("json.rrn", "prisma_cloud.misconfiguration.rrn")?;
            }

            if event.has_value("json.scannedPolicy") {
                event.rename(
                    "json.scannedPolicy",
                    "prisma_cloud.misconfiguration.scanned_policy",
                )?;
            }

            let _cond = {
                event
                    .get("prisma_cloud.misconfiguration.scanned_policy.labels")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(
                    event,
                    "prisma_cloud.misconfiguration.scanned_policy.labels",
                    |event| {
                        event.append_unique(
                            "rule.tags",
                            json!(
                                event
                                    .get("_ingest._value")
                                    .map_or_else(String::new, template_to_string)
                            ),
                        )?;
                        Ok(())
                    },
                )?;
            }

            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("prisma_cloud.misconfiguration.scanned_policy.passed") {
                    if let Some(val) =
                        event.get("prisma_cloud.misconfiguration.scanned_policy.passed")
                    {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "prisma_cloud.misconfiguration.scanned_policy.passed".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "prisma_cloud.misconfiguration.scanned_policy.passed",
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
                    "convert_prisma_cloud_misconfiguration_scanned_policy_to_boolean",
                )?;
                event.remove("prisma_cloud.misconfiguration.scanned_policy.passed");
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
                            .get("_ingest.pipeline")
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

            let _cond = {
                event.get_bool("prisma_cloud.misconfiguration.scanned_policy.passed") == Some(true)
            };
            if _cond {
                let v = json!("passed");
                if !painless_is_empty_value(&v) {
                    event.set("result.evaluation", v)?;
                }
            }

            let _cond = {
                event.get_bool("prisma_cloud.misconfiguration.scanned_policy.passed") == Some(false)
            };
            if _cond {
                let v = json!("failed");
                if !painless_is_empty_value(&v) {
                    event.set("result.evaluation", v)?;
                }
            }

            let _cond = { !event.has_value("result.evaluation") };
            if _cond {
                let v = json!("unknown");
                if !painless_is_empty_value(&v) {
                    event.set("result.evaluation", v)?;
                }
            }

            let _cond = {
                event
                    .get("prisma_cloud.misconfiguration.scanned_policy.severity")
                    .is_some_and(|v| v.is_string())
                    && event.get_str("prisma_cloud.misconfiguration.scanned_policy.severity")
                        != Some("")
            };
            if _cond {
                // Painless script
                // Source: ctx.event = ctx.event ?: [:];\ndef severityScores = [\n  'informational': 21,\n  'low': 21,\n  'medium': 47,\n  'high': 73,\n  'critical': 99\n];\nInteger score = severityScores[ctx.prisma_cloud.misconfiguration.scanned_policy.severity.toLowerCase()];\nif (score != null) {\n  ctx.event.severity = score;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"ctx.event = ctx.event ?: [:];\ndef severityScores = [\n  'informational': 21,\n  'low': 21,\n  'medium': 47,\n  'high': 73,\n  'critical': 99\n];\nInteger score = severityScores[ctx.prisma_cloud.misconfiguration.scanned_policy.severity.toLowerCase()];\nif (score != null) {\n  ctx.event.severity = score;\n}"#
                    ),
                )?;
            }

            if let Some(v) = event
                .get("prisma_cloud.misconfiguration.scanned_policy.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if let Some(v) = event
                .get("prisma_cloud.misconfiguration.scanned_policy.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.uuid", v)?;
            }

            if let Some(v) = event
                .get("rule.uuid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.id", v)?;
            }

            if event.has_value("json.unifiedAssetId") {
                event.rename(
                    "json.unifiedAssetId",
                    "prisma_cloud.misconfiguration.unified_asset_id",
                )?;
            }

            let _cond = {
                event.get_str("prisma_cloud.misconfiguration.asset_type")
                    == Some("AWS IAM Credential Report")
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_cloud.misconfiguration.unified_asset_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.id", v)?;
                }
            }

            let _cond = { event.has_value("user.id") && event.get_str("user.id") != Some("") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("user.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.get_str("prisma_cloud.misconfiguration.asset_type") == Some("EC2 Instance")
                    || event.get_str("prisma_cloud.misconfiguration.asset_type")
                        == Some("Azure Virtual Machine")
            };
            if _cond {
                if let Some(v) = event
                    .get("prisma_cloud.misconfiguration.unified_asset_id")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("host.id", v)?;
                }
            }

            let _cond = { event.has_value("host.id") && event.get_str("host.id") != Some("") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("host.id")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("prisma_cloud.misconfiguration.unified_asset_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("resource.id", v)?;
            }

            if let Some(v) = event
                .get("prisma_cloud.misconfiguration.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("resource.name", v)?;
            }

            if let Some(v) = event
                .get("prisma_cloud.misconfiguration.asset_type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("resource.type", v)?;
            }

            let v = json!(format!(
                "{}|{}",
                event
                    .get("resource.id")
                    .map_or_else(String::new, template_to_string),
                event
                    .get("rule.uuid")
                    .map_or_else(String::new, template_to_string)
            ));
            if !painless_is_empty_value(&v) {
                event.set("event.id", v)?;
            }

            let _cond = { event.get_str("result.evaluation") == Some("passed") };
            if _cond {
                let v = json!("success");
                if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
                }
            }

            let _cond = { event.get_str("result.evaluation") == Some("failed") };
            if _cond {
                let v = json!("failure");
                if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
                }
            }

            let _cond = { event.get_str("result.evaluation") == Some("unknown") };
            if _cond {
                let v = json!("unknown");
                if !painless_is_empty_value(&v) {
                    event.set("event.outcome", v)?;
                }
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
                event.remove("prisma_cloud.misconfiguration.account_id");
                event.remove("prisma_cloud.misconfiguration.account_name");
                event.remove("prisma_cloud.misconfiguration.cloud_type");
                event.remove("prisma_cloud.misconfiguration.region_id");
                event.remove("prisma_cloud.misconfiguration.asset_type");
                event.remove("prisma_cloud.misconfiguration.scanned_policy.name");
                event.remove("prisma_cloud.misconfiguration.scanned_policy.id");
                event.remove("prisma_cloud.misconfiguration.unified_asset_id");
            }

            // Painless script
            // Source: boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean drop(Object object) {\n  if (object == null || object == '') {\n    return true;\n  } else if (object instanceof Map) {\n    ((Map) object).values().removeIf(v -> drop(v));\n    return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n    ((List) object).removeIf(v -> drop(v));\n    return (((List) object).length == 0);\n  }\n  return false;\n}\ndrop(ctx);"#
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
                        "Processor {} with tag fail-{} in pipeline {} failed with message: {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.pipeline")
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
