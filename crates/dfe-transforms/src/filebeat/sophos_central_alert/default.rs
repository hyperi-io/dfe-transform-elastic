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

            event.append("event.kind", json!("alert"))?;

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

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                parse_json_field(event, "event.original", "json")?;
                Ok(())
            })();

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

            if event.has_value("json.description") {
                event.rename("json.description", "sophos_central.alert.description")?;
            }

            if let Some(v) = event
                .get("sophos_central.alert.description")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            let _cond = { event.has_value("json.when") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.when") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("sophos_central.alert.when", parsed)?,
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

            let _cond = { event.has_value("sophos_central.alert.when") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("sophos_central.alert.when") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("@timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "sophos_central.alert.when".into(),
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

            let _cond = { event.has_value("json.created_at") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.created_at") {
                        match parse_date_out(&date_str, &["ISO8601"], None, None) {
                            Some(parsed) => event.set("sophos_central.alert.created_at", parsed)?,
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
                .get("sophos_central.alert.created_at")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.created", v)?;
            }

            if event.has_value("json.type") {
                event.rename("json.type", "sophos_central.alert.type")?;
            }

            if let Some(v) = event
                .get("sophos_central.alert.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.code", v)?;
            }

            if event.has_value("json.id") {
                event.rename("json.id", "sophos_central.alert.id")?;
            }

            if let Some(v) = event
                .get("sophos_central.alert.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.id", v)?;
            }

            if event.has_value("json.data.endpoint_id") {
                event.rename(
                    "json.data.endpoint_id",
                    "sophos_central.alert.data.endpoint.id",
                )?;
            }

            if event.has_value("json.data.endpoint_type") {
                event.rename(
                    "json.data.endpoint_type",
                    "sophos_central.alert.data.endpoint.type",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.source_info.ip") {
                    if let Some(val) = event.get("json.data.source_info.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.source_info.ip".into(),
                                message,
                            }
                        })?;
                        event.set("sophos_central.alert.data.source_info_ip", converted)?;
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
                .get("sophos_central.alert.data.source_info_ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.ip", v)?;
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

            if event.has_value("json.data.hmpa_exploit.process_name") {
                event.rename(
                    "json.data.hmpa_exploit.process_name",
                    "sophos_central.alert.data.hmpa_exploit.process_name",
                )?;
            }

            if let Some(v) = event
                .get("sophos_central.alert.data.hmpa_exploit.process_name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.hmpa_exploit.process_pid") {
                    if let Some(val) = event.get("json.data.hmpa_exploit.process_pid") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.hmpa_exploit.process_pid".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sophos_central.alert.data.hmpa_exploit.process_pid",
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
                .get("sophos_central.alert.data.hmpa_exploit.process_pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pid", v)?;
            }

            if event.has_value("json.data.ips_threat.executable_name") {
                event.rename(
                    "json.data.ips_threat.executable_name",
                    "sophos_central.alert.data.ips_threat.executable.name",
                )?;
            }

            if let Some(v) = event
                .get("sophos_central.alert.data.ips_threat.executable.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            if event.has_value("json.customer_id") {
                event.rename("json.customer_id", "sophos_central.alert.customer_id")?;
            }

            if let Some(v) = event
                .get("sophos_central.alert.customer_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("organization.id", v)?;
            }

            if event.has_value("json.data.certificates") {
                event.rename(
                    "json.data.certificates",
                    "sophos_central.alert.data.certificates",
                )?;
            }

            let _cond = { event.has_value("json.source") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("json.source") {
                        // Grok pattern: ^(?:%{DATA:sophos_central.alert.source.domain.name}\\\\)?%{GREEDYDATA:sophos_central.alert.source.user.name}$
                        if !cached_grok!("^(?:%{DATA:sophos_central.alert.source.domain.name}\\\\)?%{GREEDYDATA:sophos_central.alert.source.user.name}$").extract_into(&input, event)? {
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
                event.rename("json.source", "sophos_central.alert.source.original")?;
            }

            if let Some(v) = event
                .get("sophos_central.alert.source.domain.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.domain", v)?;
            }

            if let Some(v) = event
                .get("sophos_central.alert.source.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if event.has_value("json.severity") {
                event.rename("json.severity", "sophos_central.alert.severity")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.core_remedy_items.totalItems") {
                    if let Some(val) = event.get("json.data.core_remedy_items.totalItems") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.core_remedy_items.totalItems".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sophos_central.alert.data.core_remedy.total_items",
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

            let _cond = {
                event
                    .get("json.data.core_remedy_items.items")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.data.core_remedy_items.items", |event| {
                    if event.has_value("_ingest._value.sophosPid") {
                        event.rename("_ingest._value.sophosPid", "_ingest._value.sophos_pid")?;
                    }
                    Ok(())
                })?;
            }

            let _cond = {
                event
                    .get("json.data.core_remedy_items.items")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.data.core_remedy_items.items", |event| {
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
                    .get("json.data.core_remedy_items.items")
                    .is_some_and(|v| v.is_array())
            };
            if _cond {
                foreach_array(event, "json.data.core_remedy_items.items", |event| {
                    if event.has_value("_ingest._value.processPath") {
                        event
                            .rename("_ingest._value.processPath", "_ingest._value.process_path")?;
                    }
                    Ok(())
                })?;
            }

            if event.has_value("json.data.core_remedy_items.items") {
                event.rename(
                    "json.data.core_remedy_items.items",
                    "sophos_central.alert.data.core_remedy.items",
                )?;
            }

            if event.has_value("json.data.endpoint_java_id") {
                event.rename(
                    "json.data.endpoint_java_id",
                    "sophos_central.alert.data.endpoint.java_id",
                )?;
            }

            if event.has_value("json.data.endpoint_platform") {
                event.rename(
                    "json.data.endpoint_platform",
                    "sophos_central.alert.data.endpoint.platform",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.hmpa_exploit.uid") {
                    if let Some(val) = event.get("json.data.hmpa_exploit.uid") {
                        let converted = convert_value(val, "string").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.hmpa_exploit.uid".into(),
                                message,
                            }
                        })?;
                        event.set("sophos_central.alert.data.hmpa_exploit.uid", converted)?;
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

            if event.has_value("json.data.hmpa_exploit.family_id") {
                event.rename(
                    "json.data.hmpa_exploit.family_id",
                    "sophos_central.alert.data.hmpa_exploit.family_id",
                )?;
            }

            if event.has_value("json.data.hmpa_exploit.process_version") {
                event.rename(
                    "json.data.hmpa_exploit.process_version",
                    "sophos_central.alert.data.hmpa_exploit.process_version",
                )?;
            }

            if event.has_value("json.data.hmpa_exploit.thumbprint") {
                event.rename(
                    "json.data.hmpa_exploit.thumbprint",
                    "sophos_central.alert.data.hmpa_exploit.thumbprint",
                )?;
            }

            if event.has_value("json.data.hmpa_exploit.process_path") {
                event.rename(
                    "json.data.hmpa_exploit.process_path",
                    "sophos_central.alert.data.hmpa_exploit.process_path",
                )?;
            }

            if event.has_value("json.data.hmpa_exploit.type") {
                event.rename(
                    "json.data.hmpa_exploit.type",
                    "sophos_central.alert.data.hmpa_exploit.type",
                )?;
            }

            if event.has_value("json.data.hmpa_exploit.version") {
                event.rename(
                    "json.data.hmpa_exploit.version",
                    "sophos_central.alert.data.hmpa_exploit.version",
                )?;
            }

            let _cond = { event.has_value("json.data.inserted_at") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.inserted_at") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("sophos_central.alert.data.inserted_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.inserted_at".into(),
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.ips_threat.remote_ip") {
                    if let Some(val) = event.get("json.data.ips_threat.remote_ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.ips_threat.remote_ip".into(),
                                message,
                            }
                        })?;
                        event.set("sophos_central.alert.data.ips_threat.remote.ip", converted)?;
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
                .get("sophos_central.alert.data.ips_threat.remote.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.ip", v)?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique(
                    "related.ip",
                    json!(
                        event
                            .get("destination.ip")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.data.ips_threat.executable_pid") {
                event.rename(
                    "json.data.ips_threat.executable_pid",
                    "sophos_central.alert.data.ips_threat.executable.pid",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.ips_threat.local_port") {
                    if let Some(val) = event.get("json.data.ips_threat.local_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.ips_threat.local_port".into(),
                                message,
                            }
                        })?;
                        event.set("sophos_central.alert.data.ips_threat.local_port", converted)?;
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.ips_threat.detection_type") {
                    if let Some(val) = event.get("json.data.ips_threat.detection_type") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.ips_threat.detection_type".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sophos_central.alert.data.ips_threat.detection_type",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.ips_threat.remote_port") {
                    if let Some(val) = event.get("json.data.ips_threat.remote_port") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.ips_threat.remote_port".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sophos_central.alert.data.ips_threat.remote.port",
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
                .get("sophos_central.alert.data.ips_threat.remote.port")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if event.has_value("json.data.ips_threat.executable_path") {
                event.rename(
                    "json.data.ips_threat.executable_path",
                    "sophos_central.alert.data.ips_threat.executable.path",
                )?;
            }

            let _cond = { event.has_value("json.data.ips_threat.raw_data") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(input) = event.get_string("json.data.ips_threat.raw_data") {
                        // Grok pattern: ^Message\\s*%{GREEDYDATA:sophos_central.alert.data.ips_threat.raw_data.message}\\nReference\\s*%{DATA:sophos_central.alert.data.ips_threat.raw_data.reference}\\nPacket type\\s*%{DATA:sophos_central.alert.data.ips_threat.raw_data.packet_type}\\nLocal IP:\\s*%{IP:sophos_central.alert.data.ips_threat.raw_data.local.ip}(\\nLocal Port:\\s*%{NUMBER:sophos_central.alert.data.ips_threat.raw_data.local.port:long})?(\\nLocal MAC:\\s*%{MAC:temp.local_mac})?(\\n)?(Remote IP:\\s*%{IP:sophos_central.alert.data.ips_threat.raw_data.remote.ip})?(\\n)?(Remote Port:\\s*%{NUMBER:sophos_central.alert.data.ips_threat.raw_data.remote.port:long})?(\\n)?(Remote MAC:\\s*%{MAC:temp.remote_mac})?(\\n)?(PID:\\s*%{NUMBER:sophos_central.alert.data.ips_threat.raw_data.pid})?(\\n)?(Executable:\\s*%{PATH:sophos_central.alert.data.ips_threat.raw_data.executable})?\\n(Version:\\s*%{DATA:sophos_central.alert.data.ips_threat.raw_data.version})?\\n+(Signer:\\s*%{DATA:sophos_central.alert.data.ips_threat.raw_data.signer})?\\n(SHA-256:\\s*%{WORD:sophos_central.alert.data.ips_threat.raw_data.sha_256})?$
                        // Grok pattern: ^Message\\s*%{GREEDYDATA:sophos_central.alert.data.ips_threat.raw_data.message}\\nReference\\s*%{DATA:sophos_central.alert.data.ips_threat.raw_data.reference}\\nPacket type\\s*%{DATA:sophos_central.alert.data.ips_threat.raw_data.packet_type}\\nLocal IP:\\s*%{IP:sophos_central.alert.data.ips_threat.raw_data.local.ip}\\nLocal Port:\\s*%{NUMBER:sophos_central.alert.data.ips_threat.raw_data.local.port:long}\\nLocal MAC:\\s*%{MAC:temp.local_mac}\\nRemote IP:\\s*%{IP:sophos_central.alert.data.ips_threat.raw_data.remote.ip}\\nRemote Port:\\s*%{NUMBER:sophos_central.alert.data.ips_threat.raw_data.remote.port:long}\\nRemote MAC:\\s*%{MAC:temp.remote_mac}$
                        // Grok pattern: ^Message\\s*%{GREEDYDATA:sophos_central.alert.data.ips_threat.raw_data.message}\\nPacket type\\s*%{DATA:sophos_central.alert.data.ips_threat.raw_data.packet_type}\\nLocal IP:\\s*%{IP:sophos_central.alert.data.ips_threat.raw_data.local.ip}(\\nLocal Port:\\s*%{NUMBER:sophos_central.alert.data.ips_threat.raw_data.local.port:long})?(\\nLocal MAC:\\s*%{MAC:temp.local_mac})?(\\n)?(Remote IP:\\s*%{IP:sophos_central.alert.data.ips_threat.raw_data.remote.ip})?(\\n)?(Remote Port:\\s*%{NUMBER:sophos_central.alert.data.ips_threat.raw_data.remote.port:long})?(\\n)?(Remote MAC:\\s*%{MAC:temp.remote_mac})?$
                        // Grok pattern: ^%{GREEDYDATA:sophos_central.alert.data.ips_threat.raw_data.message}$
                        if !extract_first_match(
                            &[
                                cached_grok!(
                                    "^Message\\s*%{GREEDYDATA:sophos_central.alert.data.ips_threat.raw_data.message}\\nReference\\s*%{DATA:sophos_central.alert.data.ips_threat.raw_data.reference}\\nPacket type\\s*%{DATA:sophos_central.alert.data.ips_threat.raw_data.packet_type}\\nLocal IP:\\s*%{IP:sophos_central.alert.data.ips_threat.raw_data.local.ip}(\\nLocal Port:\\s*%{NUMBER:sophos_central.alert.data.ips_threat.raw_data.local.port:long})?(\\nLocal MAC:\\s*%{MAC:temp.local_mac})?(\\n)?(Remote IP:\\s*%{IP:sophos_central.alert.data.ips_threat.raw_data.remote.ip})?(\\n)?(Remote Port:\\s*%{NUMBER:sophos_central.alert.data.ips_threat.raw_data.remote.port:long})?(\\n)?(Remote MAC:\\s*%{MAC:temp.remote_mac})?(\\n)?(PID:\\s*%{NUMBER:sophos_central.alert.data.ips_threat.raw_data.pid})?(\\n)?(Executable:\\s*%{PATH:sophos_central.alert.data.ips_threat.raw_data.executable})?\\n(Version:\\s*%{DATA:sophos_central.alert.data.ips_threat.raw_data.version})?\\n+(Signer:\\s*%{DATA:sophos_central.alert.data.ips_threat.raw_data.signer})?\\n(SHA-256:\\s*%{WORD:sophos_central.alert.data.ips_threat.raw_data.sha_256})?$"
                                ),
                                cached_grok!(
                                    "^Message\\s*%{GREEDYDATA:sophos_central.alert.data.ips_threat.raw_data.message}\\nReference\\s*%{DATA:sophos_central.alert.data.ips_threat.raw_data.reference}\\nPacket type\\s*%{DATA:sophos_central.alert.data.ips_threat.raw_data.packet_type}\\nLocal IP:\\s*%{IP:sophos_central.alert.data.ips_threat.raw_data.local.ip}\\nLocal Port:\\s*%{NUMBER:sophos_central.alert.data.ips_threat.raw_data.local.port:long}\\nLocal MAC:\\s*%{MAC:temp.local_mac}\\nRemote IP:\\s*%{IP:sophos_central.alert.data.ips_threat.raw_data.remote.ip}\\nRemote Port:\\s*%{NUMBER:sophos_central.alert.data.ips_threat.raw_data.remote.port:long}\\nRemote MAC:\\s*%{MAC:temp.remote_mac}$"
                                ),
                                cached_grok!(
                                    "^Message\\s*%{GREEDYDATA:sophos_central.alert.data.ips_threat.raw_data.message}\\nPacket type\\s*%{DATA:sophos_central.alert.data.ips_threat.raw_data.packet_type}\\nLocal IP:\\s*%{IP:sophos_central.alert.data.ips_threat.raw_data.local.ip}(\\nLocal Port:\\s*%{NUMBER:sophos_central.alert.data.ips_threat.raw_data.local.port:long})?(\\nLocal MAC:\\s*%{MAC:temp.local_mac})?(\\n)?(Remote IP:\\s*%{IP:sophos_central.alert.data.ips_threat.raw_data.remote.ip})?(\\n)?(Remote Port:\\s*%{NUMBER:sophos_central.alert.data.ips_threat.raw_data.remote.port:long})?(\\n)?(Remote MAC:\\s*%{MAC:temp.remote_mac})?$"
                                ),
                                cached_grok!(
                                    "^%{GREEDYDATA:sophos_central.alert.data.ips_threat.raw_data.message}$"
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

            if event.has_value("json.data.ips_threat.raw_data") {
                event.rename(
                    "json.data.ips_threat.raw_data",
                    "sophos_central.alert.data.ips_threat.raw_data.original",
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
                    "sophos_central.alert.data.ips_threat.raw_data.local.mac",
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
                    "sophos_central.alert.data.ips_threat.raw_data.remote.mac",
                )?;
            }

            let _cond =
                { event.has_value("sophos_central.alert.data.ips_threat.raw_data.sha_256") };
            if _cond {
                event.append_unique(
                    "related.hash",
                    json!(
                        event
                            .get("sophos_central.alert.data.ips_threat.raw_data.sha_256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if event.has_value("json.data.ips_threat.executable_version") {
                event.rename(
                    "json.data.ips_threat.executable_version",
                    "sophos_central.alert.data.ips_threat.executable.version",
                )?;
            }

            if event.has_value("json.data.ips_threat.tech_support_id") {
                event.rename(
                    "json.data.ips_threat.tech_support_id",
                    "sophos_central.alert.data.ips_threat.tech_support_id",
                )?;
            }

            if event.has_value("json.data.source_app_id") {
                event.rename(
                    "json.data.source_app_id",
                    "sophos_central.alert.data.source_app_id",
                )?;
            }

            let _cond = { event.has_value("json.data.threat_id.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.threat_id.timestamp") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event
                                .set("sophos_central.alert.data.threat_id.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.threat_id.timestamp".into(),
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.threat_id.machineIdentifier") {
                    if let Some(val) = event.get("json.data.threat_id.machineIdentifier") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.threat_id.machineIdentifier".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sophos_central.alert.data.threat_id.machine_identifier",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.threat_id.processIdentifier") {
                    if let Some(val) = event.get("json.data.threat_id.processIdentifier") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.threat_id.processIdentifier".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sophos_central.alert.data.threat_id.process_identifier",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.threat_id.counter") {
                    if let Some(val) = event.get("json.data.threat_id.counter") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.threat_id.counter".into(),
                                message,
                            }
                        })?;
                        event.set("sophos_central.alert.data.threat_id.counter", converted)?;
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

            let _cond = { event.has_value("json.data.threat_id.time") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.threat_id.time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("sophos_central.alert.data.threat_id.time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.threat_id.time".into(),
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

            let _cond = { event.has_value("json.data.threat_id.date") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.threat_id.date") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("sophos_central.alert.data.threat_id.date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.threat_id.date".into(),
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

            let _cond = { event.has_value("json.data.threat_id.timeSecond") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.threat_id.timeSecond") {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => {
                                event.set("sophos_central.alert.data.threat_id.time_sec", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.threat_id.timeSecond".into(),
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

            if event.has_value("json.data.threat_status") {
                event.rename(
                    "json.data.threat_status",
                    "sophos_central.alert.data.threat_status",
                )?;
            }

            let _cond = { event.has_value("json.data.user_match_id.timestamp") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.user_match_id.timestamp")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event
                                .set("sophos_central.alert.data.user_match_id.timestamp", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.user_match_id.timestamp".into(),
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.user_match_id.machineIdentifier") {
                    if let Some(val) = event.get("json.data.user_match_id.machineIdentifier") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.user_match_id.machineIdentifier".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sophos_central.alert.data.user_match_id.machine_identifier",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.user_match_id.processIdentifier") {
                    if let Some(val) = event.get("json.data.user_match_id.processIdentifier") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.user_match_id.processIdentifier".into(),
                                message,
                            }
                        })?;
                        event.set(
                            "sophos_central.alert.data.user_match_id.process_identifier",
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.user_match_id.counter") {
                    if let Some(val) = event.get("json.data.user_match_id.counter") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.user_match_id.counter".into(),
                                message,
                            }
                        })?;
                        event.set("sophos_central.alert.data.user_match_id.counter", converted)?;
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

            let _cond = { event.has_value("json.data.user_match_id.time") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.user_match_id.time") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("sophos_central.alert.data.user_match_id.time", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.user_match_id.time".into(),
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

            let _cond = { event.has_value("json.data.threat_id.date") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.user_match_id.date") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("sophos_central.alert.data.user_match_id.date", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.user_match_id.date".into(),
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

            let _cond = { event.has_value("json.data.user_match_id.timeSecond") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) =
                        event.get_as_string("json.data.user_match_id.timeSecond")
                    {
                        match parse_date_out(&date_str, &["UNIX"], None, None) {
                            Some(parsed) => event
                                .set("sophos_central.alert.data.user_match_id.time_sec", parsed)?,
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.user_match_id.timeSecond".into(),
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

            let _cond = { event.has_value("json.data.created_at") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.created_at") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("sophos_central.alert.data.created_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.created_at".into(),
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

            if event.has_value("json.threat") {
                event.rename("json.threat", "sophos_central.alert.threat.value")?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.threat_cleanable") {
                    if let Some(val) = event.get("json.threat_cleanable") {
                        let converted = convert_value(val, "boolean").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.threat_cleanable".into(),
                                message,
                            }
                        })?;
                        event.set("sophos_central.alert.threat.cleanable", converted)?;
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

            if event.has_value("json.event_service_event_id") {
                event.rename(
                    "json.event_service_event_id",
                    "sophos_central.alert.event_service_event_id",
                )?;
            }

            if event.has_value("json.location") {
                event.rename("json.location", "sophos_central.alert.location")?;
            }

            if event.has_value("json.data.event_service_id.data") {
                event.rename(
                    "json.data.event_service_id.data",
                    "sophos_central.alert.data.event_service_id.data",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.event_service_id.type") {
                    if let Some(val) = event.get("json.data.event_service_id.type") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.event_service_id.type".into(),
                                message,
                            }
                        })?;
                        event.set("sophos_central.alert.data.event_service_id.type", converted)?;
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

            if event.has_value("json.data.user_match_uuid.data") {
                event.rename(
                    "json.data.user_match_uuid.data",
                    "sophos_central.alert.data.user_match_uuid.data",
                )?;
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.user_match_uuid.type") {
                    if let Some(val) = event.get("json.data.user_match_uuid.type") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.user_match_uuid.type".into(),
                                message,
                            }
                        })?;
                        event.set("sophos_central.alert.data.user_match_uuid.type", converted)?;
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

            let _cond = { event.has_value("json.data.make_actionable_at") };
            if _cond {
                // on_failure: 1 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    if let Some(date_str) = event.get_as_string("json.data.make_actionable_at") {
                        match parse_date_out(&date_str, &["UNIX_MS"], None, None) {
                            Some(parsed) => {
                                event.set("sophos_central.alert.data.make_actionable_at", parsed)?
                            }
                            None => {
                                return Err(TransformError::ParseError {
                                    path: "json.data.make_actionable_at".into(),
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

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.data.policy_type") {
                    if let Some(val) = event.get("json.data.policy_type") {
                        let converted = convert_value(val, "long").map_err(|message| {
                            TransformError::ParseError {
                                path: "json.data.policy_type".into(),
                                message,
                            }
                        })?;
                        event.set("sophos_central.alert.data.policy_type", converted)?;
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

            if event.has_value("json.data.app_id") {
                event.rename("json.data.app_id", "sophos_central.alert.data.app_id")?;
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
                event.remove("sophos_central.alert.when");
                event.remove("sophos_central.alert.description");
                event.remove("sophos_central.alert.created_at");
                event.remove("sophos_central.alert.type");
                event.remove("sophos_central.alert.id");
                event.remove("sophos_central.alert.data.source_info_ip");
                event.remove("sophos_central.alert.data.hmpa_exploit.process_name");
                event.remove("sophos_central.alert.data.hmpa_exploit.process_pid");
                event.remove("sophos_central.alert.data.ips_threat.executable.name");
                event.remove("sophos_central.alert.customer_id");
                event.remove("sophos_central.alert.source.domain.name");
                event.remove("sophos_central.alert.source.user.name");
                event.remove("sophos_central.alert.data.ips_threat.remote.ip");
                event.remove("sophos_central.alert.data.ips_threat.remote.port");
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
