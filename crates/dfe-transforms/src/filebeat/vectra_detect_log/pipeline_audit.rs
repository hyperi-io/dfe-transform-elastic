// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_audit` pipeline.
pub struct PipelineAudit;

impl Transform for PipelineAudit {
    fn name(&self) -> &str {
        "pipeline_audit"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("event"))?;

            let _cond = { event.get("json.message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("session")), serde_json::Value::String(s) => s.contains("session"), _ => false }) };
            if _cond {
                event.append_unique("event.category", json!("session"))?;
            }

            let _cond = { event.get("json.message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("authentication")), serde_json::Value::String(s) => s.contains("authentication"), _ => false }) || event.get("json.message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("log in")), serde_json::Value::String(s) => s.contains("log in"), _ => false }) };
            if _cond {
                event.append_unique("event.category", json!("authentication"))?;
            }

            let _cond = { event.get("json.message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("configuration")), serde_json::Value::String(s) => s.contains("configuration"), _ => false }) };
            if _cond {
                event.append_unique("event.category", json!("configuration"))?;
            }

            let _cond = { event.get("json.message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("timeout")), serde_json::Value::String(s) => s.contains("timeout"), _ => false }) || event.get("json.message").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("log in")), serde_json::Value::String(s) => s.contains("log in"), _ => false }) };
            if _cond {
                event.append_unique("event.type", json!("end"))?;
            }

                if event.has_value("json.result") {
                    event.rename("json.result", "vectra_detect.log.result")?;
                }

            let _cond = { event.get_str("vectra_detect.log.result").is_some_and(|s| s.eq_ignore_ascii_case("success")) || event.get_str("vectra_detect.log.result").is_some_and(|s| s.eq_ignore_ascii_case("true")) };
            if _cond {
            event.set("event.outcome", json!("success"))?;
            }

            let _cond = { event.get_str("vectra_detect.log.result").is_some_and(|s| s.eq_ignore_ascii_case("failure")) || event.get_str("vectra_detect.log.result").is_some_and(|s| s.eq_ignore_ascii_case("false")) };
            if _cond {
            event.set("event.outcome", json!("failure"))?;
            }

            let _cond = { event.get_str("vectra_detect.log.result").is_some_and(|s| s.eq_ignore_ascii_case("pending")) };
            if _cond {
            event.set("event.outcome", json!("unknown"))?;
            }

                if event.has_value("json.dvchost") {
                    event.rename("json.dvchost", "vectra_detect.log.dvchost")?;
                }

            if let Some(v) = event.get("vectra_detect.log.dvchost").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.hostname", v)?;
            }

            let _cond = { event.get_str("json.source_ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.source_ip") {
                if let Some(val) = event.get("json.source_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.source_ip".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.source.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_source_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("vectra_detect.log.source.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("vectra_detect.log.source.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("vectra_detect.log.source.ip").map_or_else(String::new, template_to_string)))?;
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

                if event.has_value("json.user") {
                    event.rename("json.user", "vectra_detect.log.user.name")?;
                }

            if let Some(v) = event.get("vectra_detect.log.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.name", v)?;
            }

                if event.has_value("json.role") {
                    event.rename("json.role", "vectra_detect.log.role")?;
                }

            let _cond = { event.has_value("vectra_detect.log.role") };
            if _cond {
                event.append_unique("user.roles", json!(event.get("vectra_detect.log.role").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.message") {
                    event.rename("json.message", "vectra_detect.log.message")?;
                }

            let _cond = { event.has_value("observer.hostname") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("observer.hostname").map_or_else(String::new, template_to_string)))?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                    event.append("error.message", json!(event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string)))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
