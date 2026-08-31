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

            event.append("event.kind", json!("event"))?;

            event.set("event.category", Value::Array(vec![json!("host")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

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

            let _cond = {
                event.has_value("json.items")
                    && event.get("json.items").is_some_and(|v| match v {
                        serde_json::Value::String(s) => s.is_empty(),
                        serde_json::Value::Array(a) => a.is_empty(),
                        serde_json::Value::Object(o) => o.is_empty(),
                        serde_json::Value::Null => true,
                        _ => false,
                    })
            };
            if _cond {
                return Ok(TransformResult::Drop);
            }

            {
                let mut values = Vec::new();
                if let Some(v) = event.get("json.created_at") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.id") {
                    values.push(v.clone());
                }
                if let Some(v) = event.get("json.when") {
                    values.push(v.clone());
                }
                if !values.is_empty() {
                    event.set("_id", json!(fingerprint_default(&values)))?;
                }
            }

            let _cond = { event.has_value("json.created_at") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.created_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("sophos_central.event.created_at", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.created_at".into(),
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

            if let Some(v) = event
                .get("sophos_central.event.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "sophos_central.event.id")?;
            }

            if let Some(v) = event
                .get("sophos_central.event.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.name") {
                event.rename("json.name", "sophos_central.event.name")?;
            }

            if let Some(v) = event
                .get("sophos_central.event.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.action", v)?;
            }

            if event.has_value("json.appSha256") {
                event.rename("json.appSha256", "sophos_central.event.app_sha256")?;
            }

            if let Some(v) = event
                .get("sophos_central.event.app_sha256")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.hash.sha256", v)?;
            }

            if event.has_value("json.amsi_threat_data.parentProcessPath") {
                event.rename(
                    "json.amsi_threat_data.parentProcessPath",
                    "sophos_central.event.amsi_threat_data.parent_process.path",
                )?;
            }

            if let Some(v) = event
                .get("sophos_central.event.amsi_threat_data.parent_process.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.executable", v)?;
            }

            if event.has_value("json.amsi_threat_data.processName") {
                event.rename(
                    "json.amsi_threat_data.processName",
                    "sophos_central.event.amsi_threat_data.process.name",
                )?;
            }

            if let Some(v) = event
                .get("sophos_central.event.amsi_threat_data.process.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            if event.has_value("json.amsi_threat_data.processPath") {
                event.rename(
                    "json.amsi_threat_data.processPath",
                    "sophos_central.event.amsi_threat_data.process.path",
                )?;
            }

            if let Some(v) = event
                .get("sophos_central.event.amsi_threat_data.process.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.working_directory", v)?;
            }

            if event.has_value("json.amsi_threat_data.parentProcessId") {
                event.rename(
                    "json.amsi_threat_data.parentProcessId",
                    "sophos_central.event.amsi_threat_data.parent_process.id",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ips_threat_data.executablePid") {
                    if let Some(val) = event.get("json.ips_threat_data.executablePid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ips_threat_data.executablePid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sophos_central.event.ips_threat_data.executable.pid",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            if let Some(v) = event
                .get("sophos_central.event.ips_threat_data.executable.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pid", v)?;
            }

            if event.has_value("json.ips_threat_data.executableName") {
                event.rename(
                    "json.ips_threat_data.executableName",
                    "sophos_central.event.ips_threat_data.executable.name",
                )?;
            }

            if event.has_value("json.ips_threat_data.executablePath") {
                event.rename(
                    "json.ips_threat_data.executablePath",
                    "sophos_central.event.ips_threat_data.executable.path",
                )?;
            }

            if let Some(v) = event
                .get("sophos_central.event.ips_threat_data.executable.path")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.source_info.ip") {
                    if let Some(val) = event.get("json.source_info.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.source_info.ip".into(),
                                message,
                            }
                        })?;
                        event.set("sophos_central.event.source_info.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            if let Some(v) = event
                .get("sophos_central.event.source_info.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("sophos_central.event.source_info.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("sophos_central.event.source_info.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ips_threat_data.localPort") {
                    if let Some(val) = event.get("json.ips_threat_data.localPort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ips_threat_data.localPort".into(),
                                message,
                            }
                        })?;
                        event.set("sophos_central.event.ips_threat_data.local_port", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            if let Some(v) = event
                .get("sophos_central.event.ips_threat_data.local_port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if event.has_value("json.threat") {
                event.rename("json.threat", "sophos_central.event.threat")?;
            }

            if let Some(v) = event
                .get("sophos_central.event.threat")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("threat.feed.name", v)?;
            }

            if event.has_value("json.user_id") {
                event.rename("json.user_id", "sophos_central.event.user_id")?;
            }

            if let Some(v) = event
                .get("sophos_central.event.user_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.id", v)?;
            }

            if event.has_value("json.customer_id") {
                event.rename("json.customer_id", "sophos_central.event.customer_id")?;
            }

            if let Some(v) = event
                .get("sophos_central.event.customer_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ips_threat_data.remoteIp") {
                    if let Some(val) = event.get("json.ips_threat_data.remoteIp") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ips_threat_data.remoteIp".into(),
                                message,
                            }
                        })?;
                        event.set("sophos_central.event.ips_threat_data.remote.ip", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            if let Some(v) = event
                .get("sophos_central.event.ips_threat_data.remote.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            let _cond = { event.has_value("sophos_central.event.ips_threat_data.remote.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("sophos_central.event.ips_threat_data.remote.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ips_threat_data.remotePort") {
                    if let Some(val) = event.get("json.ips_threat_data.remotePort") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ips_threat_data.remotePort".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sophos_central.event.ips_threat_data.remote.port",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            if let Some(v) = event
                .get("sophos_central.event.ips_threat_data.remote.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            let _cond = { event.has_value("json.when") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.when") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("sophos_central.event.when", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.when".into(),
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

            let _cond = { event.has_value("sophos_central.event.when") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("sophos_central.event.when") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "sophos_central.event.when".into(),
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

            if event.has_value("json.endpoint_id") {
                event.rename("json.endpoint_id", "sophos_central.event.endpoint.id")?;
            }

            if event.has_value("json.endpoint_type") {
                event.rename("json.endpoint_type", "sophos_central.event.endpoint.type")?;
            }

            if event.has_value("json.location") {
                event.rename("json.location", "sophos_central.event.location")?;
            }

            if event.has_value("json.group") {
                event.rename("json.group", "sophos_central.event.group")?;
            }

            if event.has_value("json.origin") {
                event.rename("json.origin", "sophos_central.event.origin")?;
            }

            if event.has_value("json.appCerts") {
                event.rename("json.appCerts", "sophos_central.event.app_certs")?;
            }

            if event.has_value("json.ips_threat_data.techSupportId") {
                event.rename(
                    "json.ips_threat_data.techSupportId",
                    "sophos_central.event.ips_threat_data.tech_support_id",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.ips_threat_data.detectionType") {
                    if let Some(val) = event.get("json.ips_threat_data.detectionType") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.ips_threat_data.detectionType".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sophos_central.event.ips_threat_data.detection_type",
                            converted,
                        )?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            if event.has_value("json.ips_threat_data.executableVersion") {
                event.rename(
                    "json.ips_threat_data.executableVersion",
                    "sophos_central.event.ips_threat_data.executable.version",
                )?;
            }

            let _cond = { event.has_value("json.source") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("json.source") {
                        // Grok pattern: ^(?:%{DATA:sophos_central.event.source.domain.name}\\\\)?%{GREEDYDATA:sophos_central.event.source.user.name}$
                        if !cached_grok!("^(?:%{DATA:sophos_central.event.source.domain.name}\\\\)?%{GREEDYDATA:sophos_central.event.source.user.name}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
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

            if event.has_value("json.source") {
                event.rename("json.source", "sophos_central.event.source.original")?;
            }

            if let Some(v) = event
                .get("sophos_central.event.source.domain.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.domain", v)?;
            }

            if let Some(v) = event
                .get("sophos_central.event.source.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if event.has_value("json.amsi_threat_data.processId") {
                event.rename(
                    "json.amsi_threat_data.processId",
                    "sophos_central.event.amsi_threat_data.process.id",
                )?;
            }

            let _cond = {
                event
                    .get("json.core_remedy_items.items")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.core_remedy_items.items", |event| {
                    if event.has_value("_ingest._value.sophosPid") {
                        event.rename("_ingest._value.sophosPid", "_ingest._value.sophos_pid")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.core_remedy_items.items")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.core_remedy_items.items", |event| {
                    if event.has_value("_ingest._value.suspendResult") {
                        event.rename(
                            "_ingest._value.suspendResult",
                            "_ingest._value.suspend_result",
                        )?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.core_remedy_items.items")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.core_remedy_items.items", |event| {
                    if event.has_value("_ingest._value.processPath") {
                        event
                            .rename("_ingest._value.processPath", "_ingest._value.process_path")?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.core_remedy_items.items") {
                event.rename(
                    "json.core_remedy_items.items",
                    "sophos_central.event.core_remedy.items",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.core_remedy_items.totalItems") {
                    if let Some(val) = event.get("json.core_remedy_items.totalItems") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.core_remedy_items.totalItems".into(),
                                message,
                            }
                        })?;
                        event.set("sophos_central.event.core_remedy.total_items", converted)?;
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
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

            if event.has_value("json.severity") {
                event.rename("json.severity", "sophos_central.event.severity")?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "sophos_central.event.type")?;
            }

            let _cond = { event.has_value("json.ips_threat_data.rawData") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("json.ips_threat_data.rawData") {
                        // Grok pattern: ^Message\\s*%{GREEDYDATA:sophos_central.event.ips_threat_data.raw_data.message}\\nReference\\s*%{DATA:sophos_central.event.ips_threat_data.raw_data.reference}\\nPacket type\\s*%{DATA:sophos_central.event.ips_threat_data.raw_data.packet_type}\\nLocal IP:\\s*%{IP:sophos_central.event.ips_threat_data.raw_data.local.ip}(\\nLocal Port:\\s*%{NUMBER:sophos_central.event.ips_threat_data.raw_data.local.port:long})?(\\nLocal MAC:\\s*%{MAC:temp.local_mac})?(\\n)?(Remote IP:\\s*%{IP:sophos_central.event.ips_threat_data.raw_data.remote.ip})?(\\n)?(Remote Port:\\s*%{NUMBER:sophos_central.event.ips_threat_data.raw_data.remote.port:long})?(\\n)?(Remote MAC:\\s*%{MAC:temp.remote_mac})?(\\n)?(PID:\\s*%{NUMBER:sophos_central.event.ips_threat_data.raw_data.pid})?(\\n)?(Executable:\\s*%{PATH:sophos_central.event.ips_threat_data.raw_data.executable})?\\n(Version:\\s*%{DATA:sophos_central.event.ips_threat_data.raw_data.version})?\\n+(Signer:\\s*%{DATA:sophos_central.event.ips_threat_data.raw_data.signer})?\\n(SHA-256:\\s*%{WORD:sophos_central.event.ips_threat_data.raw_data.sha_256})?$
                        // Grok pattern: ^Message\\s*%{GREEDYDATA:sophos_central.event.ips_threat_data.raw_data.message}\\nReference\\s*%{DATA:sophos_central.event.ips_threat_data.raw_data.reference}\\nPacket type\\s*%{DATA:sophos_central.event.ips_threat_data.raw_data.packet_type}\\nLocal IP:\\s*%{IP:sophos_central.event.ips_threat_data.raw_data.local.ip}\\nLocal Port:\\s*%{NUMBER:sophos_central.event.ips_threat_data.raw_data.local.port:long}\\nLocal MAC:\\s*%{MAC:temp.local_mac}\\nRemote IP:\\s*%{IP:sophos_central.event.ips_threat_data.raw_data.remote.ip}\\nRemote Port:\\s*%{NUMBER:sophos_central.event.ips_threat_data.raw_data.remote.port:long}\\nRemote MAC:\\s*%{MAC:temp.remote_mac}$
                        // Grok pattern: ^Message\\s*%{GREEDYDATA:sophos_central.event.ips_threat_data.raw_data.message}\\nPacket type\\s*%{DATA:sophos_central.event.ips_threat_data.raw_data.packet_type}\\nLocal IP:\\s*%{IP:sophos_central.event.ips_threat_data.raw_data.local.ip}(\\nLocal Port:\\s*%{NUMBER:sophos_central.event.ips_threat_data.raw_data.local.port:long})?(\\nLocal MAC:\\s*%{MAC:temp.local_mac})?(\\n)?(Remote IP:\\s*%{IP:sophos_central.event.ips_threat_data.raw_data.remote.ip})?(\\n)?(Remote Port:\\s*%{NUMBER:sophos_central.event.ips_threat_data.raw_data.remote.port:long})?(\\n)?(Remote MAC:\\s*%{MAC:temp.remote_mac})?$
                        // Grok pattern: ^%{GREEDYDATA:sophos_central.event.ips_threat_data.raw_data.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^Message\\s*%{GREEDYDATA:sophos_central.event.ips_threat_data.raw_data.message}\\nReference\\s*%{DATA:sophos_central.event.ips_threat_data.raw_data.reference}\\nPacket type\\s*%{DATA:sophos_central.event.ips_threat_data.raw_data.packet_type}\\nLocal IP:\\s*%{IP:sophos_central.event.ips_threat_data.raw_data.local.ip}(\\nLocal Port:\\s*%{NUMBER:sophos_central.event.ips_threat_data.raw_data.local.port:long})?(\\nLocal MAC:\\s*%{MAC:temp.local_mac})?(\\n)?(Remote IP:\\s*%{IP:sophos_central.event.ips_threat_data.raw_data.remote.ip})?(\\n)?(Remote Port:\\s*%{NUMBER:sophos_central.event.ips_threat_data.raw_data.remote.port:long})?(\\n)?(Remote MAC:\\s*%{MAC:temp.remote_mac})?(\\n)?(PID:\\s*%{NUMBER:sophos_central.event.ips_threat_data.raw_data.pid})?(\\n)?(Executable:\\s*%{PATH:sophos_central.event.ips_threat_data.raw_data.executable})?\\n(Version:\\s*%{DATA:sophos_central.event.ips_threat_data.raw_data.version})?\\n+(Signer:\\s*%{DATA:sophos_central.event.ips_threat_data.raw_data.signer})?\\n(SHA-256:\\s*%{WORD:sophos_central.event.ips_threat_data.raw_data.sha_256})?$"
                                ),
                                cached_grok!(
                                    "^Message\\s*%{GREEDYDATA:sophos_central.event.ips_threat_data.raw_data.message}\\nReference\\s*%{DATA:sophos_central.event.ips_threat_data.raw_data.reference}\\nPacket type\\s*%{DATA:sophos_central.event.ips_threat_data.raw_data.packet_type}\\nLocal IP:\\s*%{IP:sophos_central.event.ips_threat_data.raw_data.local.ip}\\nLocal Port:\\s*%{NUMBER:sophos_central.event.ips_threat_data.raw_data.local.port:long}\\nLocal MAC:\\s*%{MAC:temp.local_mac}\\nRemote IP:\\s*%{IP:sophos_central.event.ips_threat_data.raw_data.remote.ip}\\nRemote Port:\\s*%{NUMBER:sophos_central.event.ips_threat_data.raw_data.remote.port:long}\\nRemote MAC:\\s*%{MAC:temp.remote_mac}$"
                                ),
                                cached_grok!(
                                    "^Message\\s*%{GREEDYDATA:sophos_central.event.ips_threat_data.raw_data.message}\\nPacket type\\s*%{DATA:sophos_central.event.ips_threat_data.raw_data.packet_type}\\nLocal IP:\\s*%{IP:sophos_central.event.ips_threat_data.raw_data.local.ip}(\\nLocal Port:\\s*%{NUMBER:sophos_central.event.ips_threat_data.raw_data.local.port:long})?(\\nLocal MAC:\\s*%{MAC:temp.local_mac})?(\\n)?(Remote IP:\\s*%{IP:sophos_central.event.ips_threat_data.raw_data.remote.ip})?(\\n)?(Remote Port:\\s*%{NUMBER:sophos_central.event.ips_threat_data.raw_data.remote.port:long})?(\\n)?(Remote MAC:\\s*%{MAC:temp.remote_mac})?$"
                                ),
                                cached_grok!(
                                    "^%{GREEDYDATA:sophos_central.event.ips_threat_data.raw_data.message}$"
                                ),
                            ],
                            &input,
                            event,
                        )? {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
                    }
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "grok")?;
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

            if event.has_value("json.ips_threat_data.rawData") {
                event.rename(
                    "json.ips_threat_data.rawData",
                    "sophos_central.event.ips_threat_data.raw_data.original",
                )?;
            }

            if event.has_value("temp.local_mac") {
                gsub_field(
                    event,
                    "temp.local_mac",
                    "temp.local_mac",
                    cached_regex!("[:.]"),
                    "-",
                )?;
            }

            if event.has_value("temp.local_mac") {
                map_strings(event, "temp.local_mac", "temp.local_mac", str::to_uppercase)?;
            }

            if event.has_value("temp.local_mac") {
                event.rename(
                    "temp.local_mac",
                    "sophos_central.event.ips_threat_data.raw_data.local.mac",
                )?;
            }

            if event.has_value("temp.remote_mac") {
                gsub_field(
                    event,
                    "temp.remote_mac",
                    "temp.remote_mac",
                    cached_regex!("[:.]"),
                    "-",
                )?;
            }

            if event.has_value("temp.remote_mac") {
                map_strings(
                    event,
                    "temp.remote_mac",
                    "temp.remote_mac",
                    str::to_uppercase,
                )?;
            }

            if event.has_value("temp.remote_mac") {
                event.rename(
                    "temp.remote_mac",
                    "sophos_central.event.ips_threat_data.raw_data.remote.mac",
                )?;
            }

            let _cond =
                { event.has_value("sophos_central.event.ips_threat_data.raw_data.sha_256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sophos_central.event.ips_threat_data.raw_data.sha_256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond =
                { event.has_value("error.message") && event.get_str("error.message") != Some("") };
            if _cond {
                event.append_unique("event.kind", json!("pipeline_error"))?;
            }

            event.remove("json");
            event.remove("temp");

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
                event.remove("sophos_central.event.created_at");
                event.remove("sophos_central.event.id");
                event.remove("sophos_central.event.name");
                event.remove("sophos_central.event.app_sha256");
                event.remove("sophos_central.event.amsi_threat_data.parent_process.path");
                event.remove("sophos_central.event.amsi_threat_data.process.name");
                event.remove("sophos_central.event.amsi_threat_data.process.path");
                event.remove("sophos_central.event.amsi_threat_data.parent_process.id");
                event.remove("sophos_central.event.ips_threat_data.executable.pid");
                event.remove("sophos_central.event.ips_threat_data.executable.path");
                event.remove("sophos_central.event.source_info.ip");
                event.remove("sophos_central.event.ips_threat_data.local_port");
                event.remove("sophos_central.event.threat");
                event.remove("sophos_central.event.user_id");
                event.remove("sophos_central.event.customer_id");
                event.remove("sophos_central.event.source.domain.name");
                event.remove("sophos_central.event.source.user.name");
                event.remove("sophos_central.event.ips_threat_data.remote.ip");
                event.remove("sophos_central.event.ips_threat_data.remote.port");
                event.remove("sophos_central.event.when");
            }

            // Painless script
            // Source: boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n  return true;\n  } else if (object instanceof Map) {\n  ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n  return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n  ((List) object).removeIf(value -> dropEmptyFields(value));\n  return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"boolean dropEmptyFields(Object object) {\n  if (object == null || object == '') {\n  return true;\n  } else if (object instanceof Map) {\n  ((Map) object).values().removeIf(value -> dropEmptyFields(value));\n  return (((Map) object).size() == 0);\n  } else if (object instanceof List) {\n  ((List) object).removeIf(value -> dropEmptyFields(value));\n  return (((List) object).length == 0);\n  }\n  return false;\n}\ndropEmptyFields(ctx);\n"#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append(
                    "error.message",
                    json!(
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
