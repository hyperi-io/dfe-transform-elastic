// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `pipeline_bigipltm` pipeline.
pub struct PipelineBigipltm;

impl Transform for PipelineBigipltm {
    fn name(&self) -> &str {
        "pipeline_bigipltm"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
            event.append("event.category", json!("network"))?;

            event.append("event.type", json!("info"))?;

        event.set("observer.product", json!("Local Traffic Manager"))?;

        let _cond = { event.has_value("json.event_timestamp") && event.get_str("json.event_timestamp") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.event_timestamp") {
                match parse_date_out(&date_str, &["yyyy-MM-dd:HH:mm.SSSz", "yyyy-MM-dd:HH:mm:ss.SSSz", "yyyy-MM-dd'T'HH:mm:ss.SSS'Z'"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.event.timestamp", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.event_timestamp".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_event_timestamp")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.event.timestamp").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("@timestamp", v)?;
        }

        let _cond = { event.get_str("json.client_ip") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.client_ip") {
            if let Some(val) = event.get("json.client_ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.client_ip".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.client.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_client_ip_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.client.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.ip", v)?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.ip", json!(event.get("client.ip").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.http_method") {
                event.rename("json.http_method", "f5_bigip.log.http.method")?;
            }

        if let Some(v) = event.get("f5_bigip.log.http.method").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.request.method", v)?;
        }

            if event.has_value("json.http_referrer") {
                event.rename("json.http_referrer", "f5_bigip.log.http.referrer")?;
            }

        if let Some(v) = event.get("f5_bigip.log.http.referrer").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.request.referrer", v)?;
        }

            if event.has_value("json.http_status") {
                event.rename("json.http_status", "f5_bigip.log.http.status")?;
            }

            if event.has_value("json.http_version") {
                event.rename("json.http_version", "f5_bigip.log.http.version")?;
            }

        if let Some(v) = event.get("f5_bigip.log.http.version").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("http.version", v)?;
        }

        let _cond = { event.get_str("json.server_ip") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.server_ip") {
            if let Some(val) = event.get("json.server_ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.server_ip".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.server.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_server_ip_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.server.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.ip", v)?;
        }

        if let Some(v) = event.get("server.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("destination.ip", v)?;
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

        if event.has_value("destination.ip") {
            if let Some(ip_str) = event.get_string("destination.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-ASN.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                    if let Some(v) = geo.get("asn") {
                        event.set("destination.as.asn", v.clone())?;
                    }
                    if let Some(v) = geo.get("organization_name") {
                        event.set("destination.as.organization_name", v.clone())?;
                    }
                }
            }
        }

            if event.has_value("destination.as.asn") {
                event.rename("destination.as.asn", "destination.as.number")?;
            }

            if event.has_value("destination.as.organization_name") {
                event.rename("destination.as.organization_name", "destination.as.organization.name")?;
            }

        if let Some(v) = event.get("destination.geo").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.geo", v)?;
        }

        if let Some(v) = event.get("destination.as").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("server.as", v)?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.ip", json!(event.get("server.ip").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.get_str("json.src_ip") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.src_ip") {
            if let Some(val) = event.get("json.src_ip") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.src_ip".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.src.ip", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_src_ip_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        if let Some(v) = event.get("f5_bigip.log.src.ip").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("source.ip", v)?;
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

        if event.has_value("source.ip") {
            if let Some(ip_str) = event.get_string("source.ip") {
                let ip_str = ip_str.to_string();
                // GeoIP enrichment (GeoLite2-ASN.mmdb)
                if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                    if let Some(v) = geo.get("asn") {
                        event.set("source.as.asn", v.clone())?;
                    }
                    if let Some(v) = geo.get("organization_name") {
                        event.set("source.as.organization_name", v.clone())?;
                    }
                }
            }
        }

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

        if let Some(v) = event.get("source.geo").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.geo", v)?;
        }

        if let Some(v) = event.get("source.as").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("client.as", v)?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.ip", json!(event.get("source.ip").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.http_user_agent") {
                event.rename("json.http_user_agent", "f5_bigip.log.http.user_agent")?;
            }

        if event.has_value("f5_bigip.log.http.user_agent") {
            gsub_field(event, "f5_bigip.log.http.user_agent", "f5_bigip.log.http.user_agent", cached_regex!("(\\([^)]*)\\+(https?://)"), "$1%2b$2")?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
        if event.has_value("f5_bigip.log.http.user_agent") {
            if let Some(s) = event.get_string("f5_bigip.log.http.user_agent") {
                match url_decode(&s) {
                    Some(decoded) => event.set("f5_bigip.log.http.user_agent", json!(decoded))?,
                    None => return Err(TransformError::ParseError {
                        path: "f5_bigip.log.http.user_agent".into(),
                        message: format!("cannot url-decode '{s}'"),
                    }),
                }
            }
        }
            Ok(())
        })();

            if let Some(ua_str) = event.get_string("f5_bigip.log.http.user_agent") {
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

            if event.has_value("json.event_source") {
                event.rename("json.event_source", "f5_bigip.log.event.source")?;
            }

            if event.has_value("json.http_uri") {
                event.rename("json.http_uri", "f5_bigip.log.http.uri")?;
            }

            if event.has_value("json.telemetryEventCategory") {
                event.rename("json.telemetryEventCategory", "f5_bigip.log.telemetry.event.category")?;
            }

            if event.has_value("json.tenant") {
                event.rename("json.tenant", "f5_bigip.log.tenant")?;
            }

            if event.has_value("json.virtual_name") {
                event.rename("json.virtual_name", "f5_bigip.log.virtual.name")?;
            }

        let _cond = { event.get_str("json.bytes_in") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.bytes_in") {
            if let Some(val) = event.get("json.bytes_in") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.bytes_in".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.bytes.in", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_bytes_in_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.bytes_out") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.bytes_out") {
            if let Some(val) = event.get("json.bytes_out") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.bytes_out".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.bytes.out", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_bytes_out_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("f5_bigip.log.bytes.in") && event.has_value("f5_bigip.log.bytes.out") };
        if _cond {
            // Painless script
            // Source: ctx.network = new HashMap();\nctx.network.bytes = ctx.f5_bigip.log.bytes.in + ctx.f5_bigip.log.bytes.out;\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(event, cached_painless!(r#"ctx.network = new HashMap();\nctx.network.bytes = ctx.f5_bigip.log.bytes.in + ctx.f5_bigip.log.bytes.out;\n"#))?;
        }

        let _cond = { event.has_value("f5_bigip.log.bytes.in") && event.get_i64("f5_bigip.log.bytes.in") != Some(0) && event.get_i64("f5_bigip.log.bytes.out") == Some(0) };
        if _cond {
        let v = json!("ingress");
        if !painless_is_empty_value(&v) {
                event.set("network.direction", v)?;
        }
        }

        let _cond = { event.has_value("f5_bigip.log.bytes.out") && event.get_i64("f5_bigip.log.bytes.out") != Some(0) && event.get_i64("f5_bigip.log.bytes.in") == Some(0) };
        if _cond {
        let v = json!("egress");
        if !painless_is_empty_value(&v) {
                event.set("network.direction", v)?;
        }
        }

        let _cond = { event.has_value("f5_bigip.log.http.method") && event.get_str("f5_bigip.log.http.method") != Some("") };
        if _cond {
        let v = json!("http");
        if !painless_is_empty_value(&v) {
                event.set("network.protocol", v)?;
        }
        }

            if event.has_value("json.cookie") {
                event.rename("json.cookie", "f5_bigip.log.cookie")?;
            }

            if event.has_value("json.http_content_type") {
                event.rename("json.http_content_type", "f5_bigip.log.http.content_type")?;
            }

            if event.has_value("json.http_host") {
                event.rename("json.http_host", "f5_bigip.log.http.host")?;
            }

            if event.has_value("json.http_url") {
                event.rename("json.http_url", "f5_bigip.log.http.url")?;
            }

        let _cond = { event.has_value("f5_bigip.log.http.url") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if !uri_parts(event, "f5_bigip.log.http.url", "url", true, false)?
                && event.get_str("f5_bigip.log.http.url").is_some_and(|value| !value.is_empty())
            {
                return Err(TransformError::ParseError {
                    path: "f5_bigip.log.http.url".into(),
                    message: "uri_parts: not a parseable URI".into(),
                });
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "uri_parts")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.node") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.node") {
            if let Some(val) = event.get("json.node") {
                let converted = convert_value(val, "ip")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.node".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.node", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_node_to_ip")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.ip", json!(event.get("f5_bigip.log.node").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

        let _cond = { event.get_str("json.node_port") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.node_port") {
            if let Some(val) = event.get("json.node_port") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.node_port".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.node_port", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_node_port_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.get_str("json.req_elapsed_time") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
        if event.has_value("json.req_elapsed_time") {
            if let Some(val) = event.get("json.req_elapsed_time") {
                let converted = convert_value(val, "long")
                    .map_err(|message| TransformError::ParseError {
                        path: "json.req_elapsed_time".into(),
                        message,
                    })?;
                event.set("f5_bigip.log.req.elapsed_time", converted)?;
            }
        }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "convert")?;
            event.set("_ingest.on_failure_processor_tag", "convert_req_elapsed_time_to_long")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("json.req_start_time") && event.get_str("json.req_start_time") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.req_start_time") {
                match parse_date_out(&date_str, &["yyyy/MM/dd HH:mm:ss"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.req.start_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.req_start_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_req_start_time")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

        let _cond = { event.has_value("json.res_start_time") && event.get_str("json.res_start_time") != Some("") };
        if _cond {
        // on_failure: 1 handler(s)
        if let Err(err) = (|| -> Result<()> {
            if let Some(date_str) = event.get_as_string("json.res_start_time") {
                match parse_date_out(&date_str, &["yyyy/MM/dd HH:mm:ss"], None, None) {
                    Some(parsed) => event.set("f5_bigip.log.res.start_time", parsed)?,
                    None => {
                        return Err(TransformError::ParseError {
                            path: "json.res_start_time".into(),
                            message: format!("unable to parse date [{date_str}]"),
                        });
                    }
                }
            }
            Ok(())
        })() {
            event.set("_ingest.on_failure_message", err.to_string())?;
            event.set("_ingest.on_failure_processor_type", "date")?;
            event.set("_ingest.on_failure_processor_tag", "date_res_start_time")?;
                    event.append("error.message", json!(format!("Processor {} with tag {} in pipeline {} failed with message: {}", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
            event.remove("_ingest.on_failure_message");
            event.remove("_ingest.on_failure_processor_type");
            event.remove("_ingest.on_failure_processor_tag");
            if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                event.remove("_ingest");
            }
        }
        }

            if event.has_value("json.user") {
                event.rename("json.user", "f5_bigip.log.user.name")?;
            }

        if let Some(v) = event.get("f5_bigip.log.user.name").filter(|v| !painless_is_empty_value(v)).cloned() {
            event.set("user.name", v)?;
        }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.append_unique("related.user", json!(event.get("user.name").map_or_else(String::new, template_to_string)))?;
            Ok(())
        })();

            if event.has_value("json.vip") {
                event.rename("json.vip", "f5_bigip.log.vip")?;
            }

            if event.has_value("json.virtual_server") {
                event.rename("json.virtual_server", "f5_bigip.log.virtual.server")?;
            }

        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.remove("json");
            Ok(())
        })();

        let _cond = { !event.has_value("tags") || !(event.get("tags").is_some_and(|v| match v { serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("preserve_duplicate_custom_fields")), serde_json::Value::String(s) => s.contains("preserve_duplicate_custom_fields"), _ => false })) };
        if _cond {
        // ignore_failure: true
        let _ = (|| -> Result<()> {
            event.remove("f5_bigip.log.event.timestamp");
            event.remove("f5_bigip.log.client.ip");
            event.remove("f5_bigip.log.hostname");
            event.remove("f5_bigip.log.http.method");
            event.remove("f5_bigip.log.http.referrer");
            event.remove("f5_bigip.log.http.version");
            event.remove("f5_bigip.log.server.ip");
            event.remove("f5_bigip.log.src.ip");
            event.remove("f5_bigip.log.user.name");
            event.remove("f5_bigip.log.application.name");
            event.remove("f5_bigip.log.http.user_agent");
            Ok(())
        })();
        }

        Ok(TransformResult::Continue)
    }
}
