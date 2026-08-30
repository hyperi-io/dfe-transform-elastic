// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_bigipsystem` pipeline.
pub struct PipelineBigipsystem;

impl Transform for PipelineBigipsystem {
    fn name(&self) -> &str {
        "pipeline_bigipsystem"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        event.set("observer.product", json!("System Information"))?;

            event.append("event.category", json!("host"))?;

            event.append("event.type", json!("info"))?;

            if event.has_value("json.system.afmState") {
                event.rename("json.system.afmState", "f5_bigip.log.afm_state")?;
            }

            if event.has_value("json.system.apmState") {
                event.rename("json.system.apmState", "f5_bigip.log.apm_state")?;
            }

            if event.has_value("json.system.asmAttackSignatures") {
                event.rename("json.system.asmAttackSignatures", "f5_bigip.log.asm_attack_signatures")?;
            }

        let _cond = { event.has_value("f5_bigip.log.asm_attack_signatures") };
        if _cond {
            {
                // A foreach walks a LIST or an OBJECT: over an object Elastic
                // binds `_ingest._key` per entry, which is what a target of
                // `<field>.{{{_ingest._key}}}` reads.
                let subject = event.get("f5_bigip.log.asm_attack_signatures").cloned();
                let keyed = matches!(subject, Some(Value::Object(_)));
                let entries: Vec<(Option<String>, Value)> = match subject {
                    Some(Value::Array(items)) => items.into_iter().map(|v| (None, v)).collect(),
                    Some(Value::Object(fields)) => fields.into_iter().map(|(k, v)| (Some(k), v)).collect(),
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
                        // on_failure: 2 handler(s)
                        if let Err(err) = (|| -> Result<()> {
                        if let Some(date_str) = event.get_as_string("_ingest._value.createDateTime") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                        Some(parsed) => event.set("_ingest._value.create_date_time", parsed)?,
                        None => {
                        return Err(TransformError::ParseError {
                        path: "_ingest._value.createDateTime".into(),
                        message: format!("unable to parse date [{date_str}]"),
                        });
                        }
                        }
                        }
                        Ok(())
                        })() {
                        event.set("_ingest.on_failure_message", err.to_string())?;
                        event.set("_ingest.on_failure_processor_type", "date")?;
                        event.set("_ingest.on_failure_processor_tag", "convert_createDateTime")?;
                        event.remove("_ingest._value.createDateTime");
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
                                if let Some(value) = left { fields.insert(key, value); }
                            }
                            None => list.push(left.unwrap_or(Value::Null)),
                        }
                    }
                    match enclosing {
                        Some(previous) => { event.set("_ingest._value", previous)?; }
                        None => { event.remove("_ingest"); }
                    }
                    if let Some(previous) = enclosing_key {
                        event.set("_ingest._key", previous)?;
                    }
                    event.set("f5_bigip.log.asm_attack_signatures", if keyed { Value::Object(fields) } else { Value::Array(list) })?;
                }
            }
        }

        let _cond = { event.has_value("f5_bigip.log.asm_attack_signatures") };
        if _cond {
            foreach_array(event, "f5_bigip.log.asm_attack_signatures", |event| {
                event.remove("_ingest._value.createDateTime");
                event.remove("_ingest._value.name");
                Ok(())
            })?;
        }

        let _cond = { event.has_value("f5_bigip.log.asm_attack_signatures") };
        if _cond {
            foreach_array(event, "f5_bigip.log.asm_attack_signatures", |event| {
                event.append_unique("file.name", json!(event.get("_ingest._value.filename").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

        let _cond = { event.has_value("f5_bigip.log.asm_attack_signatures") };
        if _cond {
            foreach_array(event, "f5_bigip.log.asm_attack_signatures", |event| {
                let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
                if _cond {
                event.remove("_ingest._value.filename");
                }
                Ok(())
            })?;
        }

            if event.has_value("json.system.asmState") {
                event.rename("json.system.asmState", "f5_bigip.log.asm_state")?;
            }

            if event.has_value("json.system.callBackUrl") {
                event.rename("json.system.callBackUrl", "f5_bigip.log.callback_url")?;
            }

            if event.has_value("json.system.chassisId") {
                event.rename("json.system.chassisId", "f5_bigip.log.chassis_id")?;
            }

        if let Some(v) = event.get("f5_bigip.log.chassis_id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("observer.serial_number", v)?;
        }

            if event.has_value("json.system.configReady") {
                event.rename("json.system.configReady", "f5_bigip.log.config_ready")?;
            }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.system.configSyncSucceeded") {
            if let Some(val) = event.get("json.system.configSyncSucceeded") {
                let converted = convert_value(val, "boolean")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.system.configSyncSucceeded".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.config_sync_succeeded", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_system_configSyncSucceeded_to_boolean")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

            if event.has_value("json.system.connectionsPerformance") {
                event.rename("json.system.connectionsPerformance", "f5_bigip.log.connections_performance")?;
            }

        let _cond = { event.has_value("f5_bigip.log.connections_performance") };
        if _cond {
            foreach_array(event, "f5_bigip.log.connections_performance", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.average") {
                if let Some(val) = event.get("_ingest._value.average") {
                let converted = convert_value(val, "double")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.average".into(),
                message,
                })?;
                event.set("_ingest._value.average", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_average")?;
                event.remove("_ingest._value.average");
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

        let _cond = { event.has_value("f5_bigip.log.connections_performance") };
        if _cond {
            foreach_array(event, "f5_bigip.log.connections_performance", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.current") {
                if let Some(val) = event.get("_ingest._value.current") {
                let converted = convert_value(val, "double")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.current".into(),
                message,
                })?;
                event.set("_ingest._value.current", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_current")?;
                event.remove("_ingest._value.current");
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

        let _cond = { event.has_value("f5_bigip.log.connections_performance") };
        if _cond {
            foreach_array(event, "f5_bigip.log.connections_performance", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.max") {
                if let Some(val) = event.get("_ingest._value.max") {
                let converted = convert_value(val, "double")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.max".into(),
                message,
                })?;
                event.set("_ingest._value.max", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_max")?;
                event.remove("_ingest._value.max");
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

        let _cond = { event.has_value("f5_bigip.log.connections_performance") };
        if _cond {
            foreach_array(event, "f5_bigip.log.connections_performance", |event| {
                event.remove("_ingest._value.name");
                Ok(())
            })?;
        }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.system.cpu") {
            if let Some(val) = event.get("json.system.cpu") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.system.cpu".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.cpu_value", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_system_cpu_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

            if event.has_value("json.system.description") {
                event.rename("json.system.description", "f5_bigip.log.description")?;
            }

        if let Some(v) = event.get("f5_bigip.log.description").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.name", v)?;
        }

        let _cond = { event.has_value("f5_bigip.log.description") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("f5_bigip.log.description").map_or_else(String::new, template_to_string)))?;
        }

            if event.has_value("json.system.diskStorage") {
                event.rename("json.system.diskStorage", "f5_bigip.log.disk_storage")?;
            }

        let _cond = { event.has_value("f5_bigip.log.disk_storage") };
        if _cond {
            foreach_array(event, "f5_bigip.log.disk_storage", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.Capacity_Float") {
                if let Some(val) = event.get("_ingest._value.Capacity_Float") {
                let converted = convert_value(val, "double")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.Capacity_Float".into(),
                message,
                })?;
                event.set("_ingest._value.capacity_float", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_capacity_float")?;
                event.remove("_ingest._value.Capacity_Float");
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

        let _cond = { event.has_value("f5_bigip.log.disk_storage") };
        if _cond {
            foreach_array(event, "f5_bigip.log.disk_storage", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.1024-blocks") {
                if let Some(val) = event.get("_ingest._value.1024-blocks") {
                let converted = convert_value(val, "long")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.1024-blocks".into(),
                message,
                })?;
                event.set("_ingest._value.1024_blocks", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_1024-blocks")?;
                event.remove("_ingest._value.1024-blocks");
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

        let _cond = { event.has_value("f5_bigip.log.disk_storage") };
        if _cond {
            foreach_array(event, "f5_bigip.log.disk_storage", |event| {
                event.remove("_ingest._value.1024-blocks");
                event.remove("_ingest._value.Capacity_Float");
                event.remove("_ingest._value.Capacity");
                event.remove("_ingest._value.name");
                Ok(())
            })?;
        }

            if event.has_value("json.system.diskLatency") {
                event.rename("json.system.diskLatency", "f5_bigip.log.disk_latency")?;
            }

        let _cond = { event.has_value("f5_bigip.log.disk_latency") };
        if _cond {
            foreach_array(event, "f5_bigip.log.disk_latency", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.%util") {
                if let Some(val) = event.get("_ingest._value.%util") {
                let converted = convert_value(val, "double")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.%util".into(),
                message,
                })?;
                event.set("_ingest._value.per_util", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_%util")?;
                event.remove("_ingest._value.%util");
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

        let _cond = { event.has_value("f5_bigip.log.disk_latency") };
        if _cond {
            foreach_array(event, "f5_bigip.log.disk_latency", |event| {
                event.remove("_ingest._value.%util");
                event.remove("_ingest._value.name");
                Ok(())
            })?;
        }

        let _cond = { event.has_value("f5_bigip.log.disk_latency") };
        if _cond {
            foreach_array(event, "f5_bigip.log.disk_latency", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.r/s") {
                if let Some(val) = event.get("_ingest._value.r/s") {
                let converted = convert_value(val, "double")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.r/s".into(),
                message,
                })?;
                event.set("_ingest._value.r/s", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_r/s")?;
                event.remove("_ingest._value.r/s");
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

        let _cond = { event.has_value("f5_bigip.log.disk_latency") };
        if _cond {
            foreach_array(event, "f5_bigip.log.disk_latency", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.w/s") {
                if let Some(val) = event.get("_ingest._value.w/s") {
                let converted = convert_value(val, "double")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.w/s".into(),
                message,
                })?;
                event.set("_ingest._value.w/s", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_w/s")?;
                event.remove("_ingest._value.w/s");
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            if event.has_value("json.system.failoverColor") {
                event.rename("json.system.failoverColor", "f5_bigip.log.failover_color")?;
            }

            if event.has_value("json.system.failoverStatus") {
                event.rename("json.system.failoverStatus", "f5_bigip.log.failover_status")?;
            }

        let _cond = { event.has_value("json.system.gtmConfigTime") && event.get_str("json.system.gtmConfigTime") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.system.gtmConfigTime") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.gtm_config_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.system.gtmConfigTime".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_system_gtmConfigTime")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.system.hostname") {
                event.rename("json.system.hostname", "f5_bigip.log.hostname")?;
            }

        if let Some(v) = event.get("f5_bigip.log.hostname").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.hostname", v)?;
        }

        let _cond = { event.has_value("host.hostname") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("host.hostname").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("json.system.lastAfmDeploy") && event.get_str("json.system.lastAfmDeploy") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.system.lastAfmDeploy") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.last_afm_deploy", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.system.lastAfmDeploy".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_system_lastAfmDeploy")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("json.system.lastAsmChange") && event.get_str("json.system.lastAsmChange") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.system.lastAsmChange") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.last_asm_change", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.system.lastAsmChange".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_system_lastAsmChange")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("json.telemetryServiceInfo.cycleEnd") && event.get_str("json.telemetryServiceInfo.cycleEnd") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.telemetryServiceInfo.cycleEnd") {
                match parse_date_out(&date_str, &["EEE, dd MMM yyyy HH:mm:ss z", "ISO8601"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.telemetry_service_info.cycle_end", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.telemetryServiceInfo.cycleEnd".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_cycle_end_conversion")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("json.telemetryServiceInfo.cycleStart") && event.get_str("json.telemetryServiceInfo.cycleStart") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.telemetryServiceInfo.cycleStart") {
                match parse_date_out(&date_str, &["EEE, dd MMM yyyy HH:mm:ss z", "ISO8601"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.telemetry_service_info.cycle_start", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.telemetryServiceInfo.cycleStart".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_cycle_start_conversion")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
        if event.has_value("json.telemetryServiceInfo.pollingInterval") {
            if let Some(val) = event.get("json.telemetryServiceInfo.pollingInterval") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.telemetryServiceInfo.pollingInterval".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.telemetry_service_info.polling_interval", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_telemetryservice_polling_interval")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

            if event.has_value("json.system.licenseReady") {
                event.rename("json.system.licenseReady", "f5_bigip.log.license_ready")?;
            }

            if event.has_value("json.system.location") {
                event.rename("json.system.location", "f5_bigip.log.location")?;
            }

        if let Some(v) = event.get("f5_bigip.log.location").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.geo.name", v)?;
        }

        let _cond = { event.has_value("json.system.ltmConfigTime") && event.get_str("json.system.ltmConfigTime") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.system.ltmConfigTime") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.ltm_config_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.system.ltmConfigTime".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_system_ltmConfigTime")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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
        if event.has_value("json.system.baseMac") {
            gsub_field(event, "json.system.baseMac", "f5_bigip.log.base_mac", cached_regex!("[:.]"), "-")?;
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "gsub")?;
            event.set("_ingest.on_failure_processor_tag", "gsub_system_baseMac")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        if event.has_value("f5_bigip.log.base_mac") {
            map_strings(event, "f5_bigip.log.base_mac", "f5_bigip.log.base_mac", str::to_uppercase)?;
        }

        let _cond = { event.has_value("f5_bigip.log.base_mac") };
        if _cond {
            event.append_unique("host.mac", json!(event.get("f5_bigip.log.base_mac").map_or_else(String::new, template_to_string)))?;
        }

        let _cond = { event.has_value("host.mac") };
        if _cond {
            foreach_array(event, "host.mac", |event| {
                event.append_unique("related.hosts", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                Ok(())
            })?;
        }

            if event.has_value("json.system.machineId") {
                event.rename("json.system.machineId", "f5_bigip.log.machine_id")?;
            }

        if let Some(v) = event.get("f5_bigip.log.machine_id").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.id", v)?;
        }

        let _cond = { event.has_value("host.id") };
        if _cond {
            event.append_unique("related.hosts", json!(event.get("host.id").map_or_else(String::new, template_to_string)))?;
        }

            if event.has_value("json.system.marketingName") {
                event.rename("json.system.marketingName", "f5_bigip.log.marketing_name")?;
            }

        if let Some(v) = event.get("f5_bigip.log.marketing_name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("observer.vendor", v)?;
        }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.system.memory") {
            if let Some(val) = event.get("json.system.memory") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.system.memory".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.memory_value", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_system_memory_value_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

            if event.has_value("json.system.networkInterfaces") {
                event.rename("json.system.networkInterfaces", "f5_bigip.log.network_interfaces")?;
            }

        let _cond = { event.has_value("f5_bigip.log.network_interfaces") };
        if _cond {
            foreach_array(event, "f5_bigip.log.network_interfaces", |event| {
                event.remove("_ingest._value.name");
                Ok(())
            })?;
        }

            if event.has_value("json.system.platformId") {
                event.rename("json.system.platformId", "f5_bigip.log.platform_id")?;
            }

            if event.has_value("json.system.provisionReady") {
                event.rename("json.system.provisionReady", "f5_bigip.log.provision_ready")?;
            }

            if event.has_value("json.system.provisioning") {
                event.rename("json.system.provisioning", "f5_bigip.log.provisioning")?;
            }

        let _cond = { event.has_value("f5_bigip.log.provisioning") };
        if _cond {
            foreach_array(event, "f5_bigip.log.provisioning", |event| {
                event.remove("_ingest._value.name");
                Ok(())
            })?;
        }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.system.swap") {
            if let Some(val) = event.get("json.system.swap") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.system.swap".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.swap", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_system_swap_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

            if event.has_value("json.system.syncColor") {
                event.rename("json.system.syncColor", "f5_bigip.log.sync_color")?;
            }

            if event.has_value("json.system.syncMode") {
                event.rename("json.system.syncMode", "f5_bigip.log.sync_mode")?;
            }

            if event.has_value("json.system.syncStatus") {
                event.rename("json.system.syncStatus", "f5_bigip.log.sync_status")?;
            }

            if event.has_value("json.system.syncSummary") {
                event.rename("json.system.syncSummary", "f5_bigip.log.sync_summary")?;
            }

        let _cond = { event.has_value("json.system.systemTimestamp") && event.get_str("json.system.systemTimestamp") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.system.systemTimestamp") {
                match parse_date_out(&date_str, &["ISO8601"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.system_timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.system.systemTimestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_system_systemTimestamp")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.system_timestamp").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("@timestamp", v)?;
        }

            if event.has_value("json.telemetryEventCategory") {
                event.rename("json.telemetryEventCategory", "f5_bigip.log.telemetry.event.category")?;
            }

            if event.has_value("json.system.throughputPerformance") {
                event.rename("json.system.throughputPerformance", "f5_bigip.log.throughput_performance")?;
            }

        let _cond = { event.has_value("f5_bigip.log.throughput_performance") };
        if _cond {
            foreach_array(event, "f5_bigip.log.throughput_performance", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.average") {
                if let Some(val) = event.get("_ingest._value.average") {
                let converted = convert_value(val, "double")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.average".into(),
                message,
                })?;
                event.set("_ingest._value.average", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_average")?;
                event.remove("_ingest._value.average");
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

        let _cond = { event.has_value("f5_bigip.log.throughput_performance") };
        if _cond {
            foreach_array(event, "f5_bigip.log.throughput_performance", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.current") {
                if let Some(val) = event.get("_ingest._value.current") {
                let converted = convert_value(val, "double")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.current".into(),
                message,
                })?;
                event.set("_ingest._value.current", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_current")?;
                event.remove("_ingest._value.current");
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

        let _cond = { event.has_value("f5_bigip.log.throughput_performance") };
        if _cond {
            foreach_array(event, "f5_bigip.log.throughput_performance", |event| {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                if event.has_value("_ingest._value.max") {
                if let Some(val) = event.get("_ingest._value.max") {
                let converted = convert_value(val, "double")
                .map_err(|message| TransformError::ParseError {
                path: "_ingest._value.max".into(),
                message,
                })?;
                event.set("_ingest._value.max", converted)?;
                }
                }
                Ok(())
                })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_max")?;
                event.remove("_ingest._value.max");
                event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

        let _cond = { event.has_value("f5_bigip.log.throughput_performance") };
        if _cond {
            foreach_array(event, "f5_bigip.log.throughput_performance", |event| {
                event.remove("_ingest._value.name");
                Ok(())
            })?;
        }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.system.tmmCpu") {
            if let Some(val) = event.get("json.system.tmmCpu") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.system.tmmCpu".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.tmm_cpu", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_system_tmmCpu_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.system.tmmMemory") {
            if let Some(val) = event.get("json.system.tmmMemory") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.system.tmmMemory".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.tmm_memory", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_system_tmmMemory_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // SKIPPED: condition not transpiled: ctx.json?.system?.tmmTraffic != null && ctx.json?.system?.tmmTraffic['clientSideTraffic.bitsIn'] != null
        #[allow(unreachable_code, unused_variables)]
        if false {
            // Painless script
            // Source: def client_side_traffic = new HashMap(); def obj = ctx.json.system.tmmTraffic.remove('clientSideTraffic.bitsIn'); client_side_traffic.put('bits_in', obj); if (ctx.f5_bigip?.log?.tmm_traffic == null) {\n  ctx.f5_bigip.log.tmm_traffic = new HashMap();\n  ctx.f5_bigip.log.tmm_traffic.put('client_side_traffic', client_side_traffic);\n} else{\n  ctx.f5_bigip.log.tmm_traffic.client_side_traffic.put('bits_in', obj);\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"def client_side_traffic = new HashMap(); def obj = ctx.json.system.tmmTraffic.remove('clientSideTraffic.bitsIn'); client_side_traffic.put('bits_in', obj); if (ctx.f5_bigip?.log?.tmm_traffic == null) {\n  ctx.f5_bigip.log.tmm_traffic = new HashMap();\n  ctx.f5_bigip.log.tmm_traffic.put('client_side_traffic', client_side_traffic);\n} else{\n  ctx.f5_bigip.log.tmm_traffic.client_side_traffic.put('bits_in', obj);\n}"#))?;
        }

        // SKIPPED: condition not transpiled: ctx.json?.system?.tmmTraffic != null && ctx.json?.system?.tmmTraffic['clientSideTraffic.bitsOut'] != null
        #[allow(unreachable_code, unused_variables)]
        if false {
            // Painless script
            // Source: def client_side_traffic = new HashMap(); def obj = ctx.json.system.tmmTraffic.remove('clientSideTraffic.bitsOut'); client_side_traffic.put('bits_out', obj); if (ctx.f5_bigip?.log?.tmm_traffic == null) {\n  ctx.f5_bigip.log.tmm_traffic = new HashMap();\n  ctx.f5_bigip.log.tmm_traffic.put('client_side_traffic', client_side_traffic);\n} else{\n  ctx.f5_bigip.log.tmm_traffic.client_side_traffic.put('bits_out', obj);\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"def client_side_traffic = new HashMap(); def obj = ctx.json.system.tmmTraffic.remove('clientSideTraffic.bitsOut'); client_side_traffic.put('bits_out', obj); if (ctx.f5_bigip?.log?.tmm_traffic == null) {\n  ctx.f5_bigip.log.tmm_traffic = new HashMap();\n  ctx.f5_bigip.log.tmm_traffic.put('client_side_traffic', client_side_traffic);\n} else{\n  ctx.f5_bigip.log.tmm_traffic.client_side_traffic.put('bits_out', obj);\n}"#))?;
        }

        // SKIPPED: condition not transpiled: ctx.json?.system?.tmmTraffic != null && ctx.json?.system?.tmmTraffic['serverSideTraffic.bitsIn'] != null
        #[allow(unreachable_code, unused_variables)]
        if false {
            // Painless script
            // Source: def server_side_traffic = new HashMap(); def obj = ctx.json.system.tmmTraffic.remove('serverSideTraffic.bitsIn'); server_side_traffic.put('bits_in', obj); if (ctx.f5_bigip?.log?.tmm_traffic == null) {\n  ctx.f5_bigip.log.tmm_traffic = new HashMap();\n  ctx.f5_bigip.log.tmm_traffic.put('server_side_traffic', server_side_traffic);\n} else{\n    if (ctx.f5_bigip?.log?.tmm_traffic?.server_side_traffic == null) {\n        ctx.f5_bigip.log.tmm_traffic.server_side_traffic = new HashMap();\n    }\n    ctx.f5_bigip.log.tmm_traffic.server_side_traffic.put('bits_in', obj);\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"def server_side_traffic = new HashMap(); def obj = ctx.json.system.tmmTraffic.remove('serverSideTraffic.bitsIn'); server_side_traffic.put('bits_in', obj); if (ctx.f5_bigip?.log?.tmm_traffic == null) {\n  ctx.f5_bigip.log.tmm_traffic = new HashMap();\n  ctx.f5_bigip.log.tmm_traffic.put('server_side_traffic', server_side_traffic);\n} else{\n    if (ctx.f5_bigip?.log?.tmm_traffic?.server_side_traffic == null) {\n        ctx.f5_bigip.log.tmm_traffic.server_side_traffic = new HashMap();\n    }\n    ctx.f5_bigip.log.tmm_traffic.server_side_traffic.put('bits_in', obj);\n}"#))?;
        }

        // SKIPPED: condition not transpiled: ctx.json?.system?.tmmTraffic != null && ctx.json?.system?.tmmTraffic['serverSideTraffic.bitsOut'] != null
        #[allow(unreachable_code, unused_variables)]
        if false {
            // Painless script
            // Source: def server_side_traffic = new HashMap(); def obj = ctx.json.system.tmmTraffic.remove('serverSideTraffic.bitsOut'); server_side_traffic.put('bits_out', obj); if (ctx.f5_bigip?.log?.tmm_traffic == null) {\n  ctx.f5_bigip.log.tmm_traffic = new HashMap();\n  ctx.f5_bigip.log.tmm_traffic.put('server_side_traffic', server_side_traffic);\n} else{\n    if (ctx.f5_bigip?.log?.tmm_traffic?.server_side_traffic == null) {\n        ctx.f5_bigip.log.tmm_traffic.server_side_traffic = new HashMap();\n    }\n    ctx.f5_bigip.log.tmm_traffic.server_side_traffic.put('bits_out', obj);\n}
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"def server_side_traffic = new HashMap(); def obj = ctx.json.system.tmmTraffic.remove('serverSideTraffic.bitsOut'); server_side_traffic.put('bits_out', obj); if (ctx.f5_bigip?.log?.tmm_traffic == null) {\n  ctx.f5_bigip.log.tmm_traffic = new HashMap();\n  ctx.f5_bigip.log.tmm_traffic.put('server_side_traffic', server_side_traffic);\n} else{\n    if (ctx.f5_bigip?.log?.tmm_traffic?.server_side_traffic == null) {\n        ctx.f5_bigip.log.tmm_traffic.server_side_traffic = new HashMap();\n    }\n    ctx.f5_bigip.log.tmm_traffic.server_side_traffic.put('bits_out', obj);\n}"#))?;
        }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("f5_bigip.log.tmm_traffic.client_side_traffic.bits_in") {
            if let Some(val) = event.get("f5_bigip.log.tmm_traffic.client_side_traffic.bits_in") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "f5_bigip.log.tmm_traffic.client_side_traffic.bits_in".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.tmm_traffic.client_side_traffic.bits_in", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_system_tmm_traffic_client_side_traffic_bits_in_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("f5_bigip.log.tmm_traffic.client_side_traffic.bits_out") {
            if let Some(val) = event.get("f5_bigip.log.tmm_traffic.client_side_traffic.bits_out") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "f5_bigip.log.tmm_traffic.client_side_traffic.bits_out".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.tmm_traffic.client_side_traffic.bits_out", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_system_tmm_traffic_client_side_traffic_bits_out_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("f5_bigip.log.tmm_traffic.server_side_traffic.bits_in") {
            if let Some(val) = event.get("f5_bigip.log.tmm_traffic.server_side_traffic.bits_in") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "f5_bigip.log.tmm_traffic.server_side_traffic.bits_in".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.tmm_traffic.server_side_traffic.bits_in", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_system_tmm_traffic_server_side_traffic_bits_in_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("f5_bigip.log.tmm_traffic.server_side_traffic.bits_out") {
            if let Some(val) = event.get("f5_bigip.log.tmm_traffic.server_side_traffic.bits_out") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "f5_bigip.log.tmm_traffic.server_side_traffic.bits_out".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.tmm_traffic.server_side_traffic.bits_out", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_system_tmm_traffic_server_side_traffic_bits_out_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }

            if event.has_value("json.system.version") {
                event.rename("json.system.version", "f5_bigip.log.version")?;
            }

        if let Some(v) = event.get("f5_bigip.log.version").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("host.os.version", v)?;
        }

            if event.has_value("json.system.versionBuild") {
                event.rename("json.system.versionBuild", "f5_bigip.log.version_build")?;
            }

        let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
        if _cond {
            event.remove("f5_bigip.log.chassis_id");
            event.remove("f5_bigip.log.description");
            event.remove("f5_bigip.log.hostname");
            event.remove("f5_bigip.log.location");
            event.remove("f5_bigip.log.base_mac");
            event.remove("f5_bigip.log.machine_id");
            event.remove("f5_bigip.log.marketing_name");
            event.remove("f5_bigip.log.system_timestamp");
            event.remove("f5_bigip.log.version");
        }

            event.remove("json");

        let _cond = { event.has_value("error.message") };
        if _cond {
            event.append_unique("event.kind", json!("pipeline_error"))?;
        }

        Ok(TransformResult::Continue)
    }
}
