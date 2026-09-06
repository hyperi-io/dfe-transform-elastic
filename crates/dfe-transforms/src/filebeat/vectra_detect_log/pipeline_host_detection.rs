// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_host_detection` pipeline.
pub struct PipelineHostDetection;

impl Transform for PipelineHostDetection {
    fn name(&self) -> &str {
        "pipeline_host_detection"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            event.set("event.kind", json!("alert"))?;

            event.set("event.category", Value::Array(vec![json!("host"), json!("threat")]))?;

            event.set("event.type", Value::Array(vec![json!("info")]))?;

            let _cond = { event.get_str("json.dd_dst_port") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.dd_dst_port") {
                if let Some(val) = event.get("json.dd_dst_port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.dd_dst_port".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.dd.dst.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dd_dst_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("vectra_detect.log.dd.dst.port").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.port", v)?;
            }

            let _cond = { event.get_str("json.dd_dst_ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.dd_dst_ip") {
                if let Some(val) = event.get("json.dd_dst_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.dd_dst_ip".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.dd.dst.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dd_dst_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("vectra_detect.log.dd.dst.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.ip", v)?;
            }

            let _cond = { event.has_value("destination.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("destination.ip").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("destination.ip") {
                if let Some(ip_str) = event.get_string("destination.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("destination.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("destination.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("destination.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("destination.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("destination.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("destination.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("destination.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("destination.geo.location", v.clone())?;
                        }
                    }
                }
            }

                if event.has_value("json.dd_dst_dns") {
                    event.rename("json.dd_dst_dns", "vectra_detect.log.dd.dst.dns")?;
                }

            if let Some(v) = event.get("vectra_detect.log.dd.dst.dns").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("destination.domain", v)?;
            }

            let _cond = { event.get_str("json.detection_id") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.detection_id") {
                if let Some(val) = event.get("json.detection_id") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.detection_id".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.detection.id", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_detection_id_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("vectra_detect.log.detection.id").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.id", v)?;
            }

                if event.has_value("json.reason") {
                    event.rename("json.reason", "vectra_detect.log.reason")?;
                }

            if let Some(v) = event.get("vectra_detect.log.reason").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reason", v)?;
            }

                if event.has_value("json.href") {
                    event.rename("json.href", "vectra_detect.log.href")?;
                }

            if let Some(v) = event.get("vectra_detect.log.href").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("event.reference", v)?;
            }

            let _cond = { event.has_value("event.reference") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if !uri_parts(event, "event.reference", "url", true, false)?
                    && event.get_str("event.reference").is_some_and(|value| !value.is_empty())
                {
                    return Err(TransformError::ParseError {
                        path: "event.reference".into(),
                        message: "uri_parts: not a parseable URI".into(),
                    });
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "uri_parts")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.host_name") {
                    event.rename("json.host_name", "vectra_detect.log.host.name")?;
                }

            if let Some(v) = event.get("vectra_detect.log.host.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("host.name") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("host.name").map_or_else(String::new, template_to_string)))?;
            }

                if event.has_value("json.http_method") {
                    event.rename("json.http_method", "vectra_detect.log.http.method")?;
                }

            if let Some(v) = event.get("vectra_detect.log.http.method").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("http.request.method", v)?;
            }

            let _cond = { event.get_str("json.response_code") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.response_code") {
                if let Some(val) = event.get("json.response_code") {
                    let converted = convert_value(val, "string")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.response_code".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.http.response_code", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_response_code_to_string")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.dd_proto") {
                    event.rename("json.dd_proto", "vectra_detect.log.dd.proto")?;
                }

            if let Some(v) = event.get("vectra_detect.log.dd.proto").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.transport", v)?;
            }

            let _cond = { event.get_str("json.bytes_sent") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.bytes_sent") {
                if let Some(val) = event.get("json.bytes_sent") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.bytes_sent".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.bytes.sent", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_bytes_sent_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("vectra_detect.log.bytes.sent").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.network.egress.bytes", v)?;
            }

            let _cond = { event.has_value("host.network.egress.bytes") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("host.network.egress.bytes").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("json.bytes_received") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.bytes_received") {
                if let Some(val) = event.get("json.bytes_received") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.bytes_received".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.bytes.received", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_bytes_received_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("vectra_detect.log.bytes.received").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("host.network.ingress.bytes", v)?;
            }

            let _cond = { event.has_value("host.network.ingress.bytes") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("host.network.ingress.bytes").map_or_else(String::new, template_to_string)))?;
            }

                // Painless script
                // Source: if (ctx.vectra_detect?.log?.bytes?.sent != null && ctx.vectra_detect?.log?.bytes?.received != null) {\n  ctx.temp = ctx.vectra_detect.log.bytes.sent + ctx.vectra_detect.log.bytes.received;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(event, cached_painless!(r#"if (ctx.vectra_detect?.log?.bytes?.sent != null && ctx.vectra_detect?.log?.bytes?.received != null) {\n  ctx.temp = ctx.vectra_detect.log.bytes.sent + ctx.vectra_detect.log.bytes.received;\n}"#))?;

            if let Some(v) = event.get("temp").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.bytes", v)?;
            }

            let _cond = { event.get_str("json.dd_bytes_sent") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.dd_bytes_sent") {
                if let Some(val) = event.get("json.dd_bytes_sent") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.dd_bytes_sent".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.dd.bytes.sent", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dd_bytes_sent_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.protocol") {
                    event.rename("json.protocol", "vectra_detect.log.protocol")?;
                }

            if let Some(v) = event.get("vectra_detect.log.protocol").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("network.protocol", v)?;
            }

                if event.has_value("json.dvchost") {
                    event.rename("json.dvchost", "vectra_detect.log.dvchost")?;
                }

            if let Some(v) = event.get("vectra_detect.log.dvchost").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("observer.hostname", v)?;
            }

            let _cond = { event.has_value("observer.hostname") };
            if _cond {
                event.append_unique("related.hosts", json!(event.get("observer.hostname").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("json.host_ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.host_ip") {
                if let Some(val) = event.get("json.host_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.host_ip".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.host.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_host_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            if let Some(v) = event.get("vectra_detect.log.host.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("source.ip", v)?;
            }

            let _cond = { event.has_value("source.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
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

                if event.has_value("json.extensions") {
                    event.rename("json.extensions", "vectra_detect.log.extensions")?;
                }

            if let Some(v) = event.get("vectra_detect.log.extensions").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.indicator.file.extension", v)?;
            }

                if event.has_value("json.threat_feeds") {
                    event.rename("json.threat_feeds", "vectra_detect.log.threat.feeds")?;
                }

            if let Some(v) = event.get("vectra_detect.log.threat.feeds").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.feed.name", v)?;
            }

                if event.has_value("json.url") {
                    event.rename("json.url", "vectra_detect.log.url")?;
                }

            if let Some(v) = event.get("vectra_detect.log.url").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("threat.indicator.url", v)?;
            }

            let _cond = { event.get_str("json.threat") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.threat") {
                if let Some(val) = event.get("json.threat") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.threat".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.threat.score", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_threat_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.account_name") {
                    event.rename("json.account_name", "vectra_detect.log.account.name")?;
                }

            if let Some(v) = event.get("vectra_detect.log.account.name").filter(|v| !painless_is_empty_value(v)).cloned() {
                event.set("user.target.name", v)?;
            }

                if event.has_value("json.role") {
                    event.rename("json.role", "vectra_detect.log.role")?;
                }

            let _cond = { event.has_value("vectra_detect.log.role") };
            if _cond {
                event.append_unique("user.roles", json!(event.get("vectra_detect.log.role").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("vectra_detect.log.role") };
            if _cond {
                event.append_unique("related.user", json!(event.get("vectra_detect.log.role").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.get_str("json.count") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.count") {
                if let Some(val) = event.get("json.count") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.count".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.count", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_count_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.normal_servers") {
                    event.rename("json.normal_servers", "vectra_detect.log.normal.servers")?;
                }

                if event.has_value("json.category") {
                    event.rename("json.category", "vectra_detect.log.category")?;
                }

            let _cond = { event.get_str("json.certainty") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.certainty") {
                if let Some(val) = event.get("json.certainty") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.certainty".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.certainty", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_certainty_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.d_type_vname") {
                    event.rename("json.d_type_vname", "vectra_detect.log.d_type.vname")?;
                }

                if event.has_value("json.d_type") {
                    event.rename("json.d_type", "vectra_detect.log.d_type.name")?;
                }

            let _cond = { event.get_str("json.triaged") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.triaged") {
                if let Some(val) = event.get("json.triaged") {
                    let converted = convert_value(val, "boolean")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.triaged".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.triaged", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_triaged_to_boolean")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.get_str("json.dd_bytes_rcvd") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.dd_bytes_rcvd") {
                if let Some(val) = event.get("json.dd_bytes_rcvd") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.dd_bytes_rcvd".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.dd.bytes.rcvd", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_dd_bytes_rcvd_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.dos_type") {
                    event.rename("json.dos_type", "vectra_detect.log.dos_type")?;
                }

            let _cond = { event.get_str("json.num_attempts") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.num_attempts") {
                if let Some(val) = event.get("json.num_attempts") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.num_attempts".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.num_attempts", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_num_attempts_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.networks") {
                    event.rename("json.networks", "vectra_detect.log.networks")?;
                }

                if event.has_value("json.shares") {
                    event.rename("json.shares", "vectra_detect.log.shares")?;
                }

                if event.has_value("json.user_agent") {
                    event.rename("json.user_agent", "vectra_detect.log.user.agent")?;
                }

                if let Some(ua_str) = event.get_string("vectra_detect.log.user.agent") {
                    let ua_str = ua_str.to_string();
                    // User agent parsing
                    if let Ok(ua) = parse_user_agent(&ua_str) {
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

                if event.has_value("json.ransom_notes") {
                    event.rename("json.ransom_notes", "vectra_detect.log.ransom_notes")?;
                }

                if event.has_value("json.sent_pattern") {
                    event.rename("json.sent_pattern", "vectra_detect.log.sent.pattern")?;
                }

                if event.has_value("json.sent_normal_pattern") {
                    event.rename("json.sent_normal_pattern", "vectra_detect.log.sent.normal_pattern")?;
                }

                if event.has_value("json.received_pattern") {
                    event.rename("json.received_pattern", "vectra_detect.log.received.pattern")?;
                }

                if event.has_value("json.received_normal_pattern") {
                    event.rename("json.received_normal_pattern", "vectra_detect.log.received.normal_pattern")?;
                }

            if event.has_value("json.accounts") {
                if let Some(s) = event.get_string("json.accounts") {
                    let mut parts: Vec<Value> = s.split(",").map(|p| json!(p)).collect();
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
                    }
                    event.set("vectra_detect.log.accounts", Value::Array(parts))?;
                }
            }

                if event.has_value("json.sql_fragment") {
                    event.rename("json.sql_fragment", "vectra_detect.log.sql_fragment")?;
                }

                if event.has_value("json.http_segment") {
                    event.rename("json.http_segment", "vectra_detect.log.http_segment")?;
                }

                if event.has_value("json.normal_admins") {
                    event.rename("json.normal_admins", "vectra_detect.log.normal.admins")?;
                }

                if event.has_value("json.client_token") {
                    event.rename("json.client_token", "vectra_detect.log.client.token")?;
                }

                if event.has_value("json.client_name") {
                    event.rename("json.client_name", "vectra_detect.log.client.name")?;
                }

                if event.has_value("json.keyboard_id") {
                    event.rename("json.keyboard_id", "vectra_detect.log.keyboard.id")?;
                }

                if event.has_value("json.keyboard_name") {
                    event.rename("json.keyboard_name", "vectra_detect.log.keyboard.name")?;
                }

                if event.has_value("json.product_id") {
                    event.rename("json.product_id", "vectra_detect.log.product_id")?;
                }

                if event.has_value("json.function") {
                    event.rename("json.function", "vectra_detect.log.function")?;
                }

                if event.has_value("json.uuid") {
                    event.rename("json.uuid", "vectra_detect.log.uuid")?;
                }

                if event.has_value("json.namedpipe") {
                    event.rename("json.namedpipe", "vectra_detect.log.named_pipe")?;
                }

                if event.has_value("json.request") {
                    event.rename("json.request", "vectra_detect.log.request")?;
                }

                if event.has_value("json.base_object") {
                    event.rename("json.base_object", "vectra_detect.log.base_object")?;
                }

                if event.has_value("json.cookie") {
                    event.rename("json.cookie", "vectra_detect.log.cookie")?;
                }

                if event.has_value("json.dst_ips") {
                    event.rename("json.dst_ips", "vectra_detect.log.dst.ips")?;
                }

                if event.has_value("json.ports") {
                    event.rename("json.ports", "vectra_detect.log.ports")?;
                }

                if event.has_value("json.successes") {
                    event.rename("json.successes", "vectra_detect.log.successes")?;
                }

                if event.has_value("json.tunnel_type") {
                    event.rename("json.tunnel_type", "vectra_detect.log.tunnel_type")?;
                }

                if event.has_value("json.matched.domain") {
                    event.rename("json.matched.domain", "vectra_detect.log.matched.domain")?;
                }

            let _cond = { event.get_str("json.matched_ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.matched_ip") {
                if let Some(val) = event.get("json.matched_ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.matched_ip".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.matched.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_matched_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.matched_user_agent") {
                    event.rename("json.matched_user_agent", "vectra_detect.log.matched.user_agent")?;
                }

                if event.has_value("json.referer") {
                    event.rename("json.referer", "vectra_detect.log.referer")?;
                }

            let _cond = { event.get_str("json.host") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.host") {
                if let Some(val) = event.get("json.host") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.host".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.host.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_host_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.reply_cache_control") {
                    event.rename("json.reply_cache_control", "vectra_detect.log.reply_cache_control")?;
                }

            let _cond = { event.get_str("json.ip") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.ip") {
                if let Some(val) = event.get("json.ip") {
                    let converted = convert_value(val, "ip")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.ip".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.ip", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_ip_to_ip")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

            let _cond = { event.has_value("vectra_detect.log.ip") };
            if _cond {
                event.append_unique("host.ip", json!(event.get("vectra_detect.log.ip").map_or_else(String::new, template_to_string)))?;
            }

            let _cond = { event.has_value("vectra_detect.log.ip") };
            if _cond {
                event.append_unique("related.ip", json!(event.get("vectra_detect.log.ip").map_or_else(String::new, template_to_string)))?;
            }

            if event.has_value("host.ip") {
                if let Some(ip_str) = event.get_string("host.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("host.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("host.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("host.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("host.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("host.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("host.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("host.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("host.geo.location", v.clone())?;
                        }
                    }
                }
            }

            let _cond = { event.get_str("json.port") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.port") {
                if let Some(val) = event.get("json.port") {
                    let converted = convert_value(val, "long")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.port".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.port", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_port_to_long")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.proxied_dst") {
                    event.rename("json.proxied_dst", "vectra_detect.log.proxied_dst")?;
                }

            let _cond = { event.get("json.service_info").is_some_and(|v| v.is_string()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.service_info", "json.service_info")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_to_split_service_info")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.service_info") {
                    event.rename("json.service_info", "vectra_detect.log.service.info")?;
                }

            let _cond = { event.get("vectra_detect.log.service.info").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.service.info", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.count") {
                    if let Some(val) = event.get("_ingest._value.count") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.count".into(),
                    message,
                    })?;
                    event.set("_ingest._value.counts", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_service_info_count_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("vectra_detect.log.service.info").is_some_and(|v| v.is_array()) };
            if _cond {
            if event.has_value("vectra_detect.log.service.info") {
                foreach_array(event, "vectra_detect.log.service.info", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value.account_uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }
            }

            let _cond = { event.get("vectra_detect.log.service.info").is_some_and(|v| v.is_array()) };
            if _cond {
            if event.has_value("vectra_detect.log.service.info") {
                foreach_array(event, "vectra_detect.log.service.info", |event| {
                    event.remove("_ingest._value.count");
                    Ok(())
                })?;
            }
            }

            let _cond = { event.get("json.account_info").is_some_and(|v| v.is_string()) };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                parse_json_field(event, "json.account_info", "json.account_info")?;
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "json")?;
                event.set("_ingest.on_failure_processor_tag", "json_to_split_account_info")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.account_info") {
                    event.rename("json.account_info", "vectra_detect.log.account.info")?;
                }

            let _cond = { event.get("vectra_detect.log.account.info").is_some_and(|v| v.is_array()) };
            if _cond {
                foreach_array(event, "vectra_detect.log.account.info", |event| {
                    // on_failure: 1 handler(s)
                    if let Err(err) = (|| -> Result<()> {
                    if event.has_value("_ingest._value.count") {
                    if let Some(val) = event.get("_ingest._value.count") {
                    let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                    path: "_ingest._value.count".into(),
                    message,
                    })?;
                    event.set("_ingest._value.counts", converted)?;
                    }
                    }
                    Ok(())
                    })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "convert")?;
                    event.set("_ingest.on_failure_processor_tag", "convert_account_info_count_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
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

            let _cond = { event.get("vectra_detect.log.account.info").is_some_and(|v| v.is_array()) };
            if _cond {
            if event.has_value("vectra_detect.log.account.info") {
                foreach_array(event, "vectra_detect.log.account.info", |event| {
                    event.append_unique("related.user", json!(event.get("_ingest._value.account_uid").map_or_else(String::new, template_to_string)))?;
                    Ok(())
                })?;
            }
            }

            let _cond = { event.get("vectra_detect.log.account.info").is_some_and(|v| v.is_array()) };
            if _cond {
            if event.has_value("vectra_detect.log.account.info") {
                foreach_array(event, "vectra_detect.log.account.info", |event| {
                    event.remove("_ingest._value.count");
                    Ok(())
                })?;
            }
            }

                if event.has_value("json.service_name") {
                    event.rename("json.service_name", "vectra_detect.log.service.name")?;
                }

            let _cond = { event.get_str("json.severity") != Some("") };
            if _cond {
            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
            if event.has_value("json.severity") {
                if let Some(val) = event.get("json.severity") {
                    let converted = convert_value(val, "double")
                        .map_err(|message| TransformError::ParseError {
                            path: "json.severity".into(),
                            message,
                        })?;
                    event.set("vectra_detect.log.severity", converted)?;
                }
            }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "convert")?;
                event.set("_ingest.on_failure_processor_tag", "convert_severity_to_double")?;
                        event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }
            }

                if event.has_value("json.account") {
                    event.rename("json.account", "vectra_detect.log.account.id")?;
                }

                if event.has_value("json.dst_ports") {
                    event.rename("json.dst_ports", "vectra_detect.log.dst.ports")?;
                }

                event.remove("temp");

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
