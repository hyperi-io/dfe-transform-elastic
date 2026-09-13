// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_object_http_request` pipeline.
pub struct PipelineObjectHttpRequest;

impl Transform for PipelineObjectHttpRequest {
    fn name(&self) -> &str {
        "pipeline_object_http_request"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            if let Some(v) = event.get("ocsf.http_request.uid").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.request.id", v)?;
            }

            if let Some(v) = event.get("ocsf.http_request.http_method").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.request.method", v)?;
            }

            if let Some(v) = event.get("ocsf.http_request.referrer").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.request.referrer", v)?;
            }

            if let Some(v) = event.get("ocsf.http_request.version").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.version", v)?;
            }

            if let Some(v) = event.get("ocsf.http_request.url.hostname").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.domain", v)?;
            }

            let _cond = { event.has_value("url.domain") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("url.domain").map_or_else(String::new, template_to_string)))?;
            }

            if let Some(v) = event.get("ocsf.http_request.url.url_string").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.original", v)?;
            }

            if let Some(v) = event.get("ocsf.http_request.url.path").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.path", v)?;
            }

            let _cond = { event.get_str("ocsf.http_request.url.port") != Some("") };
            if _cond {
            // on_failure: 2 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("ocsf.http_request.url.port") {
                if let Some(val) = event.get("ocsf.http_request.url.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "ocsf.http_request.url.port".into(),
                            message,
                        })?;
                    event.set("ocsf.http_request.url.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_http_request_url_port_to_long")?;
                        event.remove("ocsf.http_request.url.port");
                        event.append_unique("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("ocsf.http_request.url.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.port", v)?;
            }

            if let Some(v) = event.get("ocsf.http_request.url.query_string").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.query", v)?;
            }

            if let Some(v) = event.get("ocsf.http_request.url.scheme").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.scheme", v)?;
            }

            if let Some(v) = event.get("ocsf.http_request.url.subdomain").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("url.subdomain", v)?;
            }

            if event.has_value("ocsf.http_request.user_agent") {
                if let Some(ua_str) = event.get_string("ocsf.http_request.user_agent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
                        event.remove("user_agent");
                        event.set("user_agent.original", json!(ua_str))?;
                        if let Some(name) = ua.name { event.set("user_agent.name", json!(name))?; }
                        if let Some(version) = ua.version { event.set("user_agent.version", json!(version))?; }
                        if let Some(os_name) = ua.os_name {
                            event.set("user_agent.os.name", json!(os_name))?;
                            if let Some(os_version) = ua.os_version {
                                event.set("user_agent.os.version", json!(os_version))?;
                                event.set("user_agent.os.full", json!(format!("{} {}", os_name, os_version)))?;
                            }
                        }
                        if let Some(device) = ua.device { event.set("user_agent.device.name", json!(device))?; }
                    }
                }
            }

            if let Some(v) = event.get("ocsf.http_request.user_agent").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user_agent.original", v)?;
            }

            let _cond = { event.get("ocsf.http_request.x_forwarded_for").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.http_request.x_forwarded_for", |event| {
                    // on_failure: 2 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_http_request_x_forwarded_for_to_ip")?;
                    event.remove("_ingest._value");
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
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.http_request.url.category_ids").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.http_request.url.category_ids", |event| {
                    if event.has_value("_ingest._value") {
                    if let Some(val) = event.get("_ingest._value") {
                    let converted = convert_value(val, "string")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value".into(),
                    message,
                    })?;
                    event.set("_ingest._value", converted)?;
                    }
                    }
                    Ok(())
                })?;
                Ok(())
            })();
            }

            let _cond = { event.get("ocsf.http_request.x_forwarded_for").is_some_and(|v| v.is_array()) };
            if _cond {
            // ignore_failure: true
            let _ = (|| -> Result<()> {
                foreach_array(event, "ocsf.http_request.x_forwarded_for", |event| {
                    event.append_unique("related.ip", json!(event.get("_ingest._value").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
                Ok(())
            })();
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
